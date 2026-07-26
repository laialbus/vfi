#!/usr/bin/env python3
"""PreToolUse guard for VFI (GOALS.md, M0).

PROPOSED REVISION — adds the merge-guard (see "NEW in this revision" below).
Install: review, then cp docs/proposals/protect_paths.py .claude/hooks/
This file is protected, so only a human can install it; that is the point.

Refuses:
  1. Any file-tool write to a protected path (list: protected-paths.txt) or to
     any location outside the project directory.
  2. Any Bash command that names a protected path alongside a write-capable
     operation, and any git push to main.
  3. NEW in this revision:
     a. Merging a PR (gh pr merge, or the merge endpoint via gh api) unless
        the run carries VFI_ROLE=decider. The wrapper sets the role outside
        the repo, so no agent can grant itself merge authority.
     b. Applying the human-approved label from any agent, decider included.
        That label is the human sign-off for protected-path changes; only a
        human hand applies it.

This is friction, not a security boundary. The OS sandbox confines shell
commands; the server (ruleset, required checks) is the backstop.

Exit 0 allows the tool call. Exit 2 blocks it; stderr is shown to the agent.
"""

import json
import os
import re
import sys

WRITE_TOOLS = ("Edit", "Write", "NotebookEdit")
BASH_WRITE_HINTS = re.compile(
    r"(>>?|\btee\b|\brm\b|\bmv\b|\bcp\b|\bsed\s+(-\S+\s+)*-i|\btruncate\b"
    r"|\bln\b|\bchmod\b|\bpatch\b|\bdd\b|\btouch\b"
    r"|\bgit\s+(mv|rm|checkout|restore|apply|clean)\b)"
)
PUSH_TO_MAIN = re.compile(r"\bgit\b[^\n;|&]*\bpush\b[^\n;|&]*\b(main|master)\b")
MERGE_COMMAND = re.compile(
    r"\bgh\s+pr\s+merge\b|\bgh\s+api\b[^\n;|&]*/pulls/[^\n;|&]*/merge\b"
)
APPROVAL_LABEL = re.compile(
    r"(--add-label\s+\S*human-approved|labels\b[^\n;|&]*human-approved)"
)


def deny(message: str) -> None:
    print(message, file=sys.stderr)
    sys.exit(2)


def load_protected(root: str) -> list[str]:
    listing = os.path.join(root, ".claude", "hooks", "protected-paths.txt")
    try:
        with open(listing, encoding="utf-8") as f:
            return [
                line.strip()
                for line in f
                if line.strip() and not line.lstrip().startswith("#")
            ]
    except OSError:
        # Fail closed: a guard that cannot read its own list guards nothing.
        deny(
            "protect_paths hook: protected-paths.txt is missing or unreadable; "
            "refusing all writes until it is restored."
        )
    return []  # unreachable


def main() -> None:
    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError:
        deny("protect_paths hook: could not parse tool-call JSON; refusing.")

    root = os.environ.get("CLAUDE_PROJECT_DIR") or data.get("cwd") or os.getcwd()
    root = os.path.realpath(root)
    protected = load_protected(root)
    tool = data.get("tool_name", "")
    tool_input = data.get("tool_input") or {}

    if tool in WRITE_TOOLS:
        path = tool_input.get("file_path") or tool_input.get("notebook_path") or ""
        if not path:
            sys.exit(0)
        resolved = os.path.realpath(
            path if os.path.isabs(path) else os.path.join(root, path)
        )
        if resolved != root and not resolved.startswith(root + os.sep):
            deny(
                f"Write outside the project directory refused: {resolved}. "
                "Agents work only inside the workspace."
            )
        rel = os.path.relpath(resolved, root)
        for entry in protected:
            bare = entry.rstrip("/")
            if rel == bare or rel.startswith(bare + os.sep):
                deny(
                    f"'{rel}' is a protected path (matches '{entry}'). "
                    "Changing it requires human sign-off recorded in an ADR. "
                    "Stop and write an escalation instead."
                )
        sys.exit(0)

    if tool == "Bash":
        command = tool_input.get("command", "") or ""
        if PUSH_TO_MAIN.search(command):
            deny(
                "Pushing to main is forbidden. Work on a task branch and open "
                "a PR; the decider merges."
            )
        if MERGE_COMMAND.search(command) and os.environ.get("VFI_ROLE") != "decider":
            deny(
                "Merging is the decider's job. This run is not the decider "
                "(VFI_ROLE is not 'decider'), so it merges nothing. Open a PR "
                "and stop."
            )
        if APPROVAL_LABEL.search(command):
            deny(
                "The human-approved label is the human's signature. No agent "
                "applies it, ever. If a protected-path change needs approval, "
                "write an escalation and wait."
            )
        if BASH_WRITE_HINTS.search(command):
            for entry in protected:
                bare = entry.rstrip("/")
                base = os.path.basename(bare)
                named_full = bare in command
                named_base = base and re.search(
                    r"(?<![\w/.-])" + re.escape(base), command
                )
                if named_full or named_base:
                    deny(
                        f"This command names the protected path '{entry}' and "
                        "contains a write-capable operation; refused. Use a "
                        "read-only command if a read was intended. Changing "
                        "protected paths requires human sign-off."
                    )
        sys.exit(0)

    sys.exit(0)


if __name__ == "__main__":
    main()
