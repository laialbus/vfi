#!/usr/bin/env bash
#
# VFI agent run wrapper.
#
# This script owns the guardrails. The agent runs inside it, not around it.
# It lives OUTSIDE the repository on purpose: an agent that can edit its own
# wrapper has no wrapper. Keep it short and stable.
#
# Roles:
#   worker  — fleet mode: claims one task by branch, works it, opens a PR.
#   decider — reviews and merges. The only role the merge-guard lets through.
#   lead    — team mode: spawns teammates, each in its own worktree, each
#             claiming its own task by pushing the branch. Never merges.
#   planner — refills the queue as a PR when the decider requests it. Runs
#             as a phase of the decider run (the request ref) or directly.

set -euo pipefail

# launchd and bash -lc don't see Homebrew's PATH (timeout, shuf live there).
export PATH="$HOME/.local/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

# HTTPS, not SSH: the sandbox proxy cannot relay SSH.
REPO_URL="${VFI_REPO_URL:-https://github.com/laialbus/vfi.git}"
WORK_ROOT="${VFI_WORK_ROOT:-$(cd "$(dirname "$0")" && pwd)/vfi-work}"
WORKER_ID="${VFI_WORKER_ID:-1}"           # distinct per parallel worker
ROLE="${VFI_ROLE:-worker}"                # worker | decider | lead | planner
TIMEOUT_SECONDS="${VFI_TIMEOUT:-3600}"    # hard wall clock
HEADLESS="${VFI_HEADLESS:-0}"             # lead only: 1 = claude -p
MODEL="${VFI_MODEL:-claude-opus-5}"       # model for agent runs

export VFI_ROLE="$ROLE"                   # the merge-guard hook reads this

WORKTREE="$WORK_ROOT/worker-$WORKER_ID"
LOCKDIR="$WORK_ROOT/.lock-$WORKER_ID"
PROMPTS="$(cd "$(dirname "$0")" && pwd)/prompts"

mkdir -p "$WORK_ROOT"

# ---------------------------------------------------------------------------
# Logging: one file per run, pruned to the last 30.
# ---------------------------------------------------------------------------

LOG_DIR="$WORK_ROOT/logs"
mkdir -p "$LOG_DIR"
RUN_LOG="$LOG_DIR/$ROLE-$WORKER_ID-$(date +%Y%m%d-%H%M%S).log"
exec >>"$RUN_LOG" 2>&1
echo "=== run start $(date) role=$ROLE worker=$WORKER_ID ==="
ls -1t "$LOG_DIR/$ROLE-$WORKER_ID"-*.log 2>/dev/null | tail -n +31 | xargs rm -f --

# ---------------------------------------------------------------------------
# A run fired into a dark wake cannot finish: lid closed on battery, the
# machine re-sleeps in seconds and the run advances only in slivers. Decline
# before claiming anything. On AC, caffeinate holds the machine awake for
# the run instead; on battery it still blocks idle sleep while the lid is
# open. Nothing can hold off a lid-close on battery.
# ---------------------------------------------------------------------------

if ioreg -r -k AppleClamshellState -d 4 | grep -q '"AppleClamshellState" = Yes' &&
   pmset -g batt | grep -q 'Battery Power'; then
  echo "lid closed on battery: declining to run"
  exit 0
fi

caffeinate -is -w $$ &

# ---------------------------------------------------------------------------
# One run per worker at a time. mkdir is atomic; a stale lock (dead PID) is
# reclaimed, because unlike flock, mkdir does not release on process death.
# ---------------------------------------------------------------------------

if ! mkdir "$LOCKDIR" 2>/dev/null; then
  old_pid="$(cat "$LOCKDIR/pid" 2>/dev/null || true)"
  if [[ -n "$old_pid" ]] && kill -0 "$old_pid" 2>/dev/null; then
    echo "worker $WORKER_ID still running (pid $old_pid); exiting"
    exit 0
  fi
  echo "reclaiming stale lock (pid ${old_pid:-unknown} is gone)"
  rm -rf "$LOCKDIR"
  mkdir "$LOCKDIR"
fi
echo $$ >"$LOCKDIR/pid"

CLAIMED_BRANCH=""
HANDED_OFF=0

cleanup() {
  local code=$?
  if [[ -n "$CLAIMED_BRANCH" && "$HANDED_OFF" -eq 0 ]]; then
    echo "releasing claim on $CLAIMED_BRANCH"
    git -C "$WORKTREE" push origin --delete "$CLAIMED_BRANCH" || true
  fi
  rm -rf "$LOCKDIR"
  exit "$code"
}
trap cleanup EXIT

# ---------------------------------------------------------------------------
# Preflight. Never build on a broken base.
# ---------------------------------------------------------------------------

if [[ ! -d "$WORKTREE/.git" ]]; then
  git clone "$REPO_URL" "$WORKTREE"
fi

cd "$WORKTREE"
git fetch --prune origin
git checkout main
git reset --hard origin/main
git clean -fd
git for-each-ref refs/heads --format='%(refname:short)' | awk '$0 != "main"' | xargs -r git branch -D

if [[ -n "$(git status --porcelain)" ]]; then
  echo "working tree dirty after reset; escalating"
  exit 1
fi

if ! ./scripts/ci-status.sh main; then
  echo "CI is not green on main; refusing to start"
  exit 1
fi

# ---------------------------------------------------------------------------
# Decider: reviews and merges. Merge authority comes from VFI_ROLE=decider,
# which the hook checks; the wrapper is outside the repo, so no agent can
# grant itself the role. If the decider requested a refill (the
# planner/requested ref), the wrapper consumes the ref and runs the planner
# as its own phase, under its own role: the request is judgment, the launch
# is mechanism, and the planner never borrows merge authority. Consume
# before launch — a planner that dies leaves the queue drained, so the next
# decider run simply requests again.
# ---------------------------------------------------------------------------

if [[ "$ROLE" == "decider" ]]; then
  ./scripts/tasks.sh sweep || true
  timeout --signal=TERM --kill-after=60 "$TIMEOUT_SECONDS" \
    claude -p "$(cat "$PROMPTS/decider.md")"

  if git ls-remote --exit-code --heads origin planner/requested >/dev/null 2>&1; then
    git push origin --delete planner/requested || true
    echo "planner phase: refill requested"
    # The decider session left HEAD on its session branch and local main
    # behind the merges it made server-side; the planner needs the tree the
    # queue actually drained on.
    git fetch --prune origin
    git checkout -f main
    git reset --hard origin/main
    git clean -fd
    VFI_ROLE=planner timeout --signal=TERM --kill-after=60 "$TIMEOUT_SECONDS" \
      claude -p --model "$MODEL" "$(cat "$PROMPTS/planner.md")" ||
      echo "planner exited nonzero; the queue is still drained, so the next decider run re-requests"
  fi
  exit 0
fi

# ---------------------------------------------------------------------------
# Lead: spawns teammates, assigns tasks, never merges. Headless verified
# (TEAMS_HEADLESS_OK canary); interactive remains the default for supervised
# sessions.
# ---------------------------------------------------------------------------

if [[ "$ROLE" == "lead" ]]; then
  if [[ "$HEADLESS" == "1" ]]; then
    timeout --signal=TERM --kill-after=60 "$TIMEOUT_SECONDS" \
      claude -p "$(cat "$PROMPTS/lead.md")"
  else
    claude "$(cat "$PROMPTS/lead.md")"
  fi
  exit 0
fi

# ---------------------------------------------------------------------------
# Planner, run directly: same session the decider path launches, minus the
# request ref. For manual refills and shakedowns.
# ---------------------------------------------------------------------------

if [[ "$ROLE" == "planner" ]]; then
  timeout --signal=TERM --kill-after=60 "$TIMEOUT_SECONDS" \
    claude -p --model "$MODEL" "$(cat "$PROMPTS/planner.md")"
  exit 0
fi

# ---------------------------------------------------------------------------
# Worker (fleet mode): claim exactly one task. Branch creation is the lock.
# ---------------------------------------------------------------------------

if ! AVAILABLE="$(./scripts/tasks.sh available)"; then
  echo "tasks.sh failed; queue may hold a malformed task"
  ./scripts/escalate.sh "queue" "tasks.sh available failed: $(./scripts/tasks.sh available 2>&1 >/dev/null | head -1)" || true
  git add -A && git commit -m "escalation: queue read failed" || true
  git push origin "HEAD:refs/heads/escalated/queue" || true
  exit 1
fi
TASK_ID="$(printf '%s\n' "$AVAILABLE" | shuf -n 1 || true)"

if [[ -z "$TASK_ID" ]]; then
  echo "no claimable work"
  exit 0
fi

git checkout -b "$TASK_ID"

if ! git push origin "$TASK_ID"; then
  echo "claim lost on $TASK_ID; another worker has it"
  exit 0
fi
CLAIMED_BRANCH="$TASK_ID"

ESCALATED_BEFORE="$(git ls-remote --heads origin "escalated/$TASK_ID-*" | awk '{print $2}' | sort)"

set +e
timeout --signal=TERM --kill-after=60 "$TIMEOUT_SECONDS" \
  claude -p --model "$MODEL" "Work task $TASK_ID. Read CLAUDE.md first. One task only."
AGENT_CODE=$?
set -e

if [[ "$AGENT_CODE" -ne 0 ]]; then
  echo "agent exited $AGENT_CODE (124 means it hit the timeout)"
  ./scripts/escalate.sh "$TASK_ID" "agent exited $AGENT_CODE" || true
  git add -A && git commit -m "escalation: agent exited $AGENT_CODE on $TASK_ID" || true
  git push origin "HEAD:refs/heads/escalated/$TASK_ID-$(date +%Y%m%d-%H%M%S)" || true
  exit 1
fi

ESCALATED_AFTER="$(git ls-remote --heads origin "escalated/$TASK_ID-*" | awk '{print $2}' | sort)"
NEW_ESCALATED="$(comm -13 <(printf '%s\n' "$ESCALATED_BEFORE") <(printf '%s\n' "$ESCALATED_AFTER"))"
if [[ -n "$NEW_ESCALATED" ]]; then
  PR_STATE="$(gh pr view "$TASK_ID" --json state --jq .state 2>/dev/null || true)"
  if [[ "$PR_STATE" != "OPEN" ]]; then
    echo "agent escalated on $TASK_ID ($NEW_ESCALATED); releasing the claim"
    exit 0
  fi
fi

if ! ./scripts/gates.sh; then
  echo "gates failed; preserving branch as escalated/$TASK_ID"
  ./scripts/escalate.sh "$TASK_ID" "gates failed" || true
  git add -A && git commit -m "escalation: gates failed on $TASK_ID" || true
  git push origin "HEAD:refs/heads/escalated/$TASK_ID-$(date +%Y%m%d-%H%M%S)" || true
  exit 1
fi

git push origin "$TASK_ID"
HANDED_OFF=1

PR_STATE="$(gh pr view "$TASK_ID" --json state --jq .state 2>/dev/null || true)"
if [[ "$PR_STATE" != "OPEN" ]]; then
  echo "agent exited 0 but no open PR for $TASK_ID (found: ${PR_STATE:-none}); recovering and escalating"

  BODY_FILE="$WORK_ROOT/pr-body-$WORKER_ID.md"
  {
    echo "> Opened by the run wrapper: the agent exited 0 without opening this"
    echo "> pull request. The escalation is on the escalated/ ref."
    echo
    cat sessions/*"$TASK_ID"* 2>/dev/null || git log -1 --format=%b
  } >"$BODY_FILE"
  gh pr create --head "$TASK_ID" --title "$(git log -1 --format=%s)" \
    --body-file "$BODY_FILE" || true

  ./scripts/escalate.sh "$TASK_ID" "agent exited 0 but no open PR (found: ${PR_STATE:-none}); wrapper opened one from the session entry" || true
  git add escalations && git commit -m "escalation: no open PR on $TASK_ID at handoff" || true
  git push origin "HEAD:refs/heads/escalated/$TASK_ID-$(date +%Y%m%d-%H%M%S)" || true
  git reset --hard "origin/$TASK_ID" || true
  exit 1
fi

echo "task $TASK_ID handed off"
