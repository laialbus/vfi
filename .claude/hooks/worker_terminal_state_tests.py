#!/usr/bin/env python3
"""Corpus for the worker terminal-state gate. DRAFT — NOT INSTALLED.

Destination: `.claude/hooks/worker_terminal_state_tests.py`, beside the hook it
judges. It installs with `docs/proposals/worker_terminal_state.py` — the corpus
and the hook move together, because a gate whose corpus stayed behind is a gate
nobody can change safely. Run it before installing either:

    python3 docs/proposals/worker_terminal_state_tests.py
    python3 docs/proposals/worker_terminal_state_tests.py --show
    python3 docs/proposals/worker_terminal_state_tests.py --only escalat

There is no installed predecessor to measure the draft against — this hook is
new — so every case states the verdict it wants outright, and a block also
states what its feedback must name. Feedback that does not name the missing
step is the failure this gate exists to avoid: an agent told only that it may
not stop learns nothing it can act on.

Each case builds a throwaway repository with a real bare `origin`, drives the
checkout into one state through the ops below, and feeds the hook a Stop event
on stdin exactly as the harness does. `gh` is a stub on PATH whose answer the
case picks, so the pull-request states — open, closed, absent, and `gh` itself
failing — are all reachable without a network.

The properties `docs/adr/worker-terminal-state.md` decided, and the cases that
hold them:

  - a claim with no terminal state is blocked, with the missing step named
  - an open pull request over pushed commits passes
  - an escalation pushed to an `escalated/` ref passes
  - a run holding no claim passes — on main, detached, or unpushed branch
  - a second stop is never blocked (`stop_hook_active`)
  - a hook that cannot answer warns and allows

Passing is necessary, not sufficient. Add a case for every miss found later.
"""
from __future__ import annotations
import collections
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.realpath(__file__))
DRAFT = os.path.join(HERE, "worker_terminal_state.py")

TASK = "M9-99"
BLOCK, WARN, ALLOW, ERROR = "block", "warn", "allow", "error"

GH_STUB = """#!/bin/sh
# Stub gh for the corpus. The hook asks one question — `gh pr list --json` —
# and VFI_TEST_GH is the answer this case wants back.
case "${VFI_TEST_GH:-none}" in
  fail)    echo 'gh: could not resolve host: github.com' >&2; exit 1 ;;
  garbage) echo 'Welcome to gh! Run "gh auth login" to get started.' ;;
  none)    echo '[]' ;;
  open)    echo '[{"number":7,"state":"OPEN"}]' ;;
  closed)  echo '[{"number":7,"state":"CLOSED"}]' ;;
  *)       echo "stub gh: no answer for VFI_TEST_GH=$VFI_TEST_GH" >&2; exit 3 ;;
esac
"""

_serial = 0


def sh(args: list[str], cwd: str | None = None) -> None:
    subprocess.run(args, cwd=cwd, check=True, capture_output=True, text=True)


def commit(work: str, path: str, message: str) -> None:
    global _serial
    _serial += 1
    full = os.path.join(work, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as f:
        f.write(f"{message} ({_serial})\n")
    sh(["git", "add", "-A"], work)
    sh(["git", "commit", "-q", "-m", message], work)


def op_branch(work: str, base: str) -> None:
    sh(["git", "checkout", "-q", "-b", TASK], work)


def op_claim(work: str, base: str) -> None:
    op_branch(work, base)
    sh(["git", "push", "-q", "origin", TASK], work)


def op_commit(work: str, base: str) -> None:
    commit(work, f"sessions/2026-08-04-{TASK}.md", "feat: do the work")


def op_push(work: str, base: str) -> None:
    sh(["git", "push", "-q", "origin", TASK], work)


def op_escalate(work: str, base: str) -> None:
    commit(work, f"escalations/2026-08-04-{TASK}.md", "escalation: stopped")
    sh(
        ["git", "push", "-q", "origin",
         f"HEAD:refs/heads/escalated/{TASK}-20260804-000000"],
        work,
    )


def op_stale_escalation(work: str, base: str) -> None:
    """An escalated ref from an earlier run of this task: on main, not in HEAD."""
    here = subprocess.run(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        cwd=work, check=True, capture_output=True, text=True,
    ).stdout.strip()
    sh(["git", "checkout", "-q", "-b", "earlier-run", "main"], work)
    commit(work, f"escalations/2026-08-01-{TASK}.md", "escalation: earlier run")
    sh(
        ["git", "push", "-q", "origin",
         f"HEAD:refs/heads/escalated/{TASK}-20260801-000000"],
        work,
    )
    sh(["git", "checkout", "-q", here], work)
    sh(["git", "branch", "-q", "-D", "earlier-run"], work)


def op_detach(work: str, base: str) -> None:
    sh(["git", "checkout", "-q", "--detach"], work)


def op_unreachable(work: str, base: str) -> None:
    sh(["git", "remote", "set-url", "origin", os.path.join(base, "gone.git")], work)


OPS = {
    "branch": op_branch,
    "claim": op_claim,
    "commit": op_commit,
    "push": op_push,
    "escalate": op_escalate,
    "stale-escalation": op_stale_escalation,
    "detach": op_detach,
    "unreachable": op_unreachable,
}


def build(base: str, ops: tuple[str, ...]) -> str:
    origin = os.path.join(base, "origin.git")
    work = os.path.join(base, "work")
    binary = os.path.join(base, "bin")
    os.makedirs(binary)
    stub = os.path.join(binary, "gh")
    with open(stub, "w", encoding="utf-8") as f:
        f.write(GH_STUB)
    os.chmod(stub, os.stat(stub).st_mode | stat.S_IXUSR | stat.S_IXGRP)

    sh(["git", "init", "-q", "--bare", "-b", "main", origin])
    sh(["git", "init", "-q", "-b", "main", work])
    sh(["git", "config", "user.email", "corpus@vfi.test"], work)
    sh(["git", "config", "user.name", "corpus"], work)
    sh(["git", "config", "commit.gpgsign", "false"], work)
    commit(work, "README.md", "chore: base commit")
    sh(["git", "remote", "add", "origin", origin], work)
    sh(["git", "push", "-q", "origin", "main"], work)
    for op in ops:
        OPS[op](work, base)
    return work


def event(work: str, extra: dict) -> str:
    payload = {
        "session_id": "corpus",
        "transcript_path": "",
        "cwd": work,
        "permission_mode": "bypassPermissions",
        "hook_event_name": "Stop",
        "stop_hook_active": False,
        "last_assistant_message": (
            "I'll push the branch and open the PR the moment the monitor comes "
            "back green."
        ),
    }
    payload.update(extra)
    return json.dumps(payload)


def verdict(work: str, base: str, stdin: str, role: str | None, gh: str):
    env = dict(os.environ)
    env["PATH"] = os.path.join(base, "bin") + os.pathsep + env.get("PATH", "")
    env["CLAUDE_PROJECT_DIR"] = work
    env["VFI_TEST_GH"] = gh
    env.pop("VFI_ROLE", None)
    if role is not None:
        env["VFI_ROLE"] = role
    proc = subprocess.run(
        [sys.executable, DRAFT],
        input=stdin, cwd=work, env=env, capture_output=True, text=True,
    )
    if proc.returncode != 0:
        return ERROR, f"exit {proc.returncode}: {proc.stderr.strip()}"
    out = proc.stdout.strip()
    if not out:
        return ALLOW, ""
    try:
        answer = json.loads(out)
    except json.JSONDecodeError:
        return ERROR, f"stdout is not JSON: {out}"
    if answer.get("decision") == "block":
        return BLOCK, str(answer.get("reason", ""))
    if "systemMessage" in answer:
        return WARN, str(answer["systemMessage"])
    return ALLOW, out


Case = collections.namedtuple("Case", "name ops gh role extra raw want says")


def case(name, want, ops=(), gh="none", role="worker", extra=None, raw=None, says=()):
    return Case(name, tuple(ops), gh, role, extra or {}, raw, want, tuple(says))


CASES = [
    case("claim held, nothing pushed, no PR", BLOCK,
         ops=["claim", "commit"],
         says=[f"git push origin {TASK}", "gh pr create"]),
    case("branch pushed, no pull request", BLOCK,
         ops=["claim", "commit", "push"],
         says=["gh pr create"]),
    case("handoff: pushed, pull request open", ALLOW,
         ops=["claim", "commit", "push"], gh="open"),
    case("open pull request, commits left unpushed", BLOCK,
         ops=["claim", "commit", "push", "commit"], gh="open",
         says=[f"git push origin {TASK}"]),
    case("pull request closed is not a handoff", BLOCK,
         ops=["claim", "commit", "push"], gh="closed",
         says=["#7 is closed"]),
    case("escalation pushed, no pull request", ALLOW,
         ops=["claim", "commit", "escalate"]),
    case("escalation riding an open pull request", ALLOW,
         ops=["claim", "commit", "push", "escalate"], gh="open"),
    case("an earlier run's escalated ref closes nothing", BLOCK,
         ops=["claim", "commit", "stale-escalation"],
         says=[f"git push origin {TASK}"]),
    case("no claim: still on main", ALLOW),
    case("no claim: branch never pushed", ALLOW, ops=["branch", "commit"]),
    case("no claim: detached HEAD", ALLOW, ops=["claim", "commit", "detach"]),
    case("second stop after a block", ALLOW,
         ops=["claim", "commit"], extra={"stop_hook_active": True}),
    case("the decider is not gated", ALLOW,
         ops=["claim", "commit"], role="decider"),
    case("a session with no role is not gated", ALLOW,
         ops=["claim", "commit"], role=None),
    case("a subagent stopping is not the run ending", ALLOW,
         ops=["claim", "commit"], extra={"hook_event_name": "SubagentStop"}),
    case("cannot answer: gh fails", WARN, ops=["claim", "commit"], gh="fail"),
    case("cannot answer: gh answers in prose", WARN,
         ops=["claim", "commit"], gh="garbage"),
    case("cannot answer: origin unreachable", WARN,
         ops=["claim", "commit", "unreachable"]),
    case("cannot answer: the event is not JSON", WARN,
         ops=["claim", "commit"], raw="not json at all"),
    case("the deferred background task is named", BLOCK,
         ops=["claim", "commit"],
         extra={"background_tasks": [
             {"id": "t1", "type": "monitor", "status": "running",
              "description": "watch the CI run"}]},
         says=["background task", "monitor"]),
]


def require_draft() -> bool:
    if os.path.exists(DRAFT):
        return True
    print(f"no draft at {DRAFT}; nothing to judge.", file=sys.stderr)
    return False


def main() -> int:
    if not require_draft():
        return 1
    if not shutil.which("git"):
        print("git is not on PATH; cannot build the fixtures.", file=sys.stderr)
        return 1

    show = "--show" in sys.argv
    only = ""
    if "--only" in sys.argv:
        index = sys.argv.index("--only") + 1
        only = sys.argv[index] if index < len(sys.argv) else ""

    selected = [c for c in CASES if only in c.name]
    failures = 0
    print(f"{'case':<46} {'want':>5} {'got':>5}")
    print("-" * 60)
    for c in selected:
        with tempfile.TemporaryDirectory(prefix="terminal-state-") as base:
            work = build(base, c.ops)
            stdin = c.raw if c.raw is not None else event(work, c.extra)
            got, feedback = verdict(work, base, stdin, c.role, c.gh)
        mark = ""
        if got != c.want:
            mark, failures = "  FAIL", failures + 1
        else:
            missing = [s for s in c.says if s not in feedback]
            if missing:
                mark = f"  FAIL says {missing}"
                failures += 1
        print(f"{c.name:<46} {c.want:>5} {got:>5}{mark}")
        if show and feedback:
            print("\n".join(f"    | {line}" for line in feedback.splitlines()))
    print("-" * 60)
    print(f"{len(selected)} cases, {failures} failing")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
