#!/usr/bin/env python3
r"""Stop gate: a worker run ends only in a terminal state.

Three pieces act together: this file, the Stop wiring in
.claude/settings.json, and the finish-step sentence in AGENTS.md.

## What this refuses

One thing: a worker session ending its turn while it still holds a claim and
has reached none of the terminal states. The block carries the missing step by
name, so the agent — which still has the whole run in context — finishes the
job in the next turn instead of a human repairing it in the morning.

Everything else is allowed. `VFI_ROLE` must be `worker`: the decider, the
lead, and supervised sessions are never gated.

## The terminal states (docs/adr/worker-terminal-state.md)

  - **Handoff** — the claimed branch is on origin with this checkout's HEAD in
    it, and a pull request for that branch is open. Both halves are required:
    a pull request that does not contain the session entry is a handoff of
    work nobody will read.
  - **Escalation** — an `escalated/<task>-*` ref on origin whose tip is HEAD or
    an ancestor of it. The ancestry is what makes it *this* run's escalation:
    a ref left behind by an earlier run of the same task sits on a commit this
    checkout does not contain, and closes nothing.
  - **No claim** — HEAD is on main, detached, or on a branch origin does not
    have. Nothing is held, so nothing is owed.

## Fail open, always

The stop is allowed, with a warning to the log, whenever the hook cannot
answer: unreadable event JSON, git unavailable, origin unreachable, `gh`
failing or returning something that will not parse, or a remote branch holding
commits this checkout does not have. It also yields the moment
`stop_hook_active` is set, so it blocks at most once per session and can never
wedge a run in a loop.

This is friction in front of an honest agent, not a boundary. The wrapper's
own postcondition check — it opens the missing pull request and escalates
after the fact — is unchanged and remains the backstop behind this.

## What this does not do

  - It does not judge the work. Gates, review, and merge are elsewhere; this
    asks only whether the run reached an end state.
  - It does not cover teammates in team mode. They inherit `VFI_ROLE=lead`
    from the lead that spawned them, and the failure class this closes is the
    fleet worker's.
  - It does not fetch. A checkout that has fallen behind origin is a state the
    hook reports and allows, rather than one it repairs; a Stop hook that
    writes to `.git` would be a side effect nobody asked for.
  - It does not read the transcript. What the agent *said* it would do is not
    evidence; the refs and the pull request are.

Exit is always 0. The verdict is the JSON on stdout: `decision: block` holds
the stop and hands `reason` to the agent, `systemMessage` warns the human and
allows, and no output at all allows silently.
"""
from __future__ import annotations
import json
import os
import subprocess
import sys

ROLE = "worker"
MAIN = "main"
ESCALATED = "escalated/"
GIT_TIMEOUT = 10
GH_TIMEOUT = 20


def allow() -> int:
    return 0


def warn(message: str) -> int:
    print(json.dumps({"systemMessage": f"terminal-state gate: {message}"}))
    return 0


def block(reason: str) -> int:
    print(json.dumps({"decision": "block", "reason": reason}))
    return 0


def run(args: tuple[str, ...], cwd: str, timeout: int) -> tuple[int, str] | None:
    """Exit code and stdout, or None when the command could not be run at all."""
    try:
        finished = subprocess.run(
            args, cwd=cwd, capture_output=True, text=True, timeout=timeout
        )
    except (OSError, subprocess.SubprocessError):
        return None
    return finished.returncode, finished.stdout.strip()


def git(root: str, *args: str) -> str | None:
    result = run(("git", "-C", root) + args, root, GIT_TIMEOUT)
    if result is None:
        return None
    code, out = result
    return out if code == 0 else None


def is_ancestor(root: str, older: str, newer: str) -> bool | None:
    """None when git cannot decide — a missing object is not an answer."""
    result = run(
        ("git", "-C", root, "merge-base", "--is-ancestor", older, newer),
        root,
        GIT_TIMEOUT,
    )
    if result is None:
        return None
    code, _ = result
    if code in (0, 1):
        return code == 0
    return None


def remote_heads(root: str) -> dict[str, str] | None:
    """origin's branches to their tips, or None when origin cannot be reached."""
    out = git(root, "ls-remote", "--heads", "origin")
    if out is None:
        return None
    heads = {}
    for line in out.splitlines():
        sha, _, ref = line.partition("\t")
        if ref.startswith("refs/heads/"):
            heads[ref[len("refs/heads/") :]] = sha.strip()
    return heads


def pull_requests(root: str, branch: str) -> list[dict] | None:
    """Every pull request for `branch`, or None when gh cannot answer.

    `pr list` rather than `pr view`: it exits 0 with an empty array when there
    is no pull request, which separates that answer from a failure. `pr view`
    spells both as exit 1.
    """
    result = run(
        (
            "gh", "pr", "list",
            "--head", branch,
            "--state", "all",
            "--json", "number,state",
            "--limit", "10",
        ),
        root,
        GH_TIMEOUT,
    )
    if result is None:
        return None
    code, out = result
    if code != 0:
        return None
    try:
        listed = json.loads(out or "[]")
    except json.JSONDecodeError:
        return None
    if not isinstance(listed, list):
        return None
    return [pr for pr in listed if isinstance(pr, dict)]


def escalation_pushed(root: str, branch: str, head: str, heads: dict[str, str]) -> bool:
    """Does origin hold an escalated/ ref this run pushed?

    A candidate whose tip git cannot place is treated as another run's. This
    run's own escalation commit is in this checkout by construction, so the
    only refs that fall out here are ones that were never ours.
    """
    prefix = ESCALATED + branch
    for name, sha in heads.items():
        if name != prefix and not name.startswith(prefix + "-"):
            continue
        if is_ancestor(root, sha, head):
            return True
    return False


def deferral_note(event: dict) -> str:
    tasks = event.get("background_tasks")
    if not isinstance(tasks, list) or not tasks:
        return ""
    kinds = sorted({str(t.get("type", "task")) for t in tasks if isinstance(t, dict)})
    count = f"{len(tasks)} background task" + ("s are" if len(tasks) > 1 else " is")
    return (
        f"\n\n{count} still in flight ({', '.join(kinds) or 'unknown'}). "
        "Whatever you deferred there ends when this turn does. Do it here "
        "instead."
    )


def reason(
    branch: str, pushed: bool, prs: list[dict], event: dict
) -> str:
    steps = []
    if not pushed:
        steps.append(
            f"- Commits in this checkout are not on origin/{branch}: "
            f"`git push origin {branch}`"
        )
    if not any(pr.get("state") == "OPEN" for pr in prs):
        others = ", ".join(
            f"#{pr.get('number')} is {str(pr.get('state', '?')).lower()}" for pr in prs
        )
        tail = f" ({others}, which is not a handoff)" if others else ""
        steps.append(
            f"- No open pull request for {branch}{tail}: "
            f"`gh pr create --head {branch} --title ... --body-file ...`"
        )
    return (
        f"You still hold the claim on `{branch}` and this run has not reached a "
        "terminal state. AGENTS.md ends a run one of two ways: push the branch "
        "and open a pull request, or revert and push an escalation.\n\n"
        + "\n".join(steps)
        + "\n\nThe run ends when this turn ends. A headless session does not "
        "outlive its final message, so a pull request left to a background "
        "task, a monitor, or a next turn is a pull request that never opens. "
        "Do it now, in this turn."
        + deferral_note(event)
        + "\n\nIf the work should not ship, that is the other ending: revert, "
        "write the escalation with `scripts/escalate.sh`, commit it, and push "
        f"it to `refs/heads/escalated/{branch}-$(date +%Y%m%d-%H%M%S)`. This "
        "gate treats that as done, because it is."
    )


def main() -> int:
    try:
        event = json.load(sys.stdin)
    except (json.JSONDecodeError, UnicodeDecodeError, OSError, ValueError):
        return warn("the Stop event did not parse; allowing the stop.")
    if not isinstance(event, dict):
        return warn("the Stop event was not an object; allowing the stop.")
    if event.get("hook_event_name", "Stop") != "Stop":
        return allow()
    if event.get("stop_hook_active"):
        return allow()
    if os.environ.get("VFI_ROLE") != ROLE:
        return allow()

    root = os.environ.get("CLAUDE_PROJECT_DIR") or event.get("cwd") or os.getcwd()
    root = os.path.realpath(str(root))

    branch = git(root, "rev-parse", "--abbrev-ref", "HEAD")
    if branch is None:
        return warn("git could not name the branch; allowing the stop.")
    if branch in ("HEAD", MAIN) or branch.startswith(ESCALATED):
        return allow()

    heads = remote_heads(root)
    if heads is None:
        return warn("origin is unreachable, so the claim cannot be read; allowing.")
    claimed = heads.get(branch)
    if claimed is None:
        return allow()

    head = git(root, "rev-parse", "HEAD")
    if head is None:
        return warn("git could not resolve HEAD; allowing the stop.")

    if escalation_pushed(root, branch, head, heads):
        return allow()

    prs = pull_requests(root, branch)
    if prs is None:
        return warn(f"gh could not report pull requests for {branch}; allowing.")

    pushed = head == claimed
    if not pushed:
        behind = is_ancestor(root, head, claimed)
        if behind is None:
            return warn(
                f"origin/{branch} and HEAD cannot be compared here; allowing."
            )
        pushed = behind

    if pushed and any(pr.get("state") == "OPEN" for pr in prs):
        return allow()

    return block(reason(branch, pushed, prs, event))


if __name__ == "__main__":
    sys.exit(main())
