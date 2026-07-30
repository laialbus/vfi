#!/usr/bin/env python3
"""Corpus test for the protect-paths guard. DRAFT — NOT INSTALLED.

Destination: `.claude/hooks/protect_paths_tests.py`, alongside the hook it
tests. Installed by a human together with `docs/proposals/protect_paths.py`,
after `docs/adr/protect-paths-hook-matching.md` is approved.

Run it before installing anything:

    python3 docs/proposals/protect_paths_tests.py

It builds a throwaway checkout, feeds each case to both the installed hook and
the draft as the harness does — the tool call as JSON on stdin — and compares
the exit codes. Exit 0 means every case matched what the draft is supposed to
do. The old column is informational: it shows which refusals change, and every
one of those changes is a case the proposal argues is a false positive.

Each case runs with TMPDIR pointed at a scratch directory inside the fixture,
so the guard's temp exemption covers that directory and not the fixture's own
"outside the workspace" paths. `{scratch}` in a command is replaced with it.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.realpath(__file__))
REPO = os.path.realpath(os.path.join(HERE, "..", ".."))
DRAFT = os.path.join(HERE, "protect_paths.py")
INSTALLED = os.path.join(REPO, ".claude", "hooks", "protect_paths.py")

ALLOW, DENY = "allow", "deny"

# (name, expected, tool, tool_input, extra env)
CASES = [
    # Text that merely names a protected path. Every one of these is a
    # refusal observed in the 2026-07-29 session.
    ("body names AGENTS.md with an arrow", ALLOW, "Bash", {
        "command": 'gh pr create --base main --head t --title "docs: layout" '
                   '--body "AGENTS.md says fetch -> normalize -> analyze"'}, {}),
    ("commit message names contracts/", ALLOW, "Bash", {
        "command": 'git commit -m "docs: record that contracts/ are versioned (v1 -> v2)"'}, {}),
    ("commit message names rm and AGENTS.md", ALLOW, "Bash", {
        "command": 'git commit -m "fix: stop refusing an rm of AGENTS.md named in a message"'}, {}),
    ("heredoc body names protected paths", ALLOW, "Bash", {
        "command": "gh pr create --base main --head t --body-file - <<'EOF'\n"
                   "Adds a task to WORKPLAN.md and explains why .claude/hooks/ "
                   "and scripts/gates.sh are left alone.\nEOF"}, {}),

    # Ordinary work must stay allowed.
    ("write inside the repo", ALLOW, "Bash", {"command": 'echo "note" > docs/notes.md'}, {}),
    ("read a protected file", ALLOW, "Bash", {"command": "cat ANCHORS.md"}, {}),
    ("copy a protected file out to temp", ALLOW, "Bash", {"command": "cp ANCHORS.md {scratch}/copy.md"}, {}),
    ("push a task branch", ALLOW, "Bash", {"command": "git push origin M1-04"}, {}),
    ("write to the run's scratch directory", ALLOW, "Bash", {
        "command": "echo x > {scratch}/scratch.txt"}, {}),
    ("in-place edit of an unprotected file", ALLOW, "Bash", {
        "command": "sed -i '' 's/a/b/' docs/notes.md"}, {}),
    ("chained mkdir and write", ALLOW, "Bash", {
        "command": "mkdir -p docs/adr && echo x > docs/adr/foo.md"}, {}),
    ("file tool write inside the repo", ALLOW, "Write", {"file_path": "docs/notes.md"}, {}),

    # Protected paths, by any route.
    ("rm a hook file", DENY, "Bash", {"command": "rm .claude/hooks/protect_paths.py"}, {}),
    ("rm -rf the hooks directory", DENY, "Bash", {"command": "rm -rf .claude/hooks"}, {}),
    ("append to the workplan", DENY, "Bash", {"command": 'echo "- [ ] M9-01" >> WORKPLAN.md'}, {}),
    ("in-place edit of an anchor", DENY, "Bash", {"command": "sed -i '' 's/x/y/' ANCHORS.md"}, {}),
    ("rm an anchor from a nested shell", DENY, "Bash", {"command": "bash -c 'rm ANCHORS.md'"}, {}),
    ("rename the ADR template", DENY, "Bash", {
        "command": "mv docs/adr/TEMPLATE.md docs/adr/T2.md"}, {}),
    ("touch a file in a protected directory", DENY, "Bash", {
        "command": "touch contracts/v1.rs"}, {}),
    ("tee over an anchor", DENY, "Bash", {"command": "tee ANCHORS.md < /dev/null"}, {}),
    ("interpreter writing an anchor", DENY, "Bash", {
        "command": "python3 -c \"open('ANCHORS.md','w').write('')\""}, {}),
    ("patch naming an anchor", DENY, "Bash", {"command": "patch AGENTS.md < fix.diff"}, {}),
    ("file tool write to an anchor", DENY, "Write", {"file_path": "ANCHORS.md"}, {}),
    ("file tool write to an anchor in a worktree", DENY, "Write", {
        "file_path": ".claude/worktrees/wt/ANCHORS.md"}, {}),

    # Outside the workspace.
    ("redirect into the prompts directory", DENY, "Bash", {
        "command": "cat docs/notes.md > ../prompts/lead.md"}, {}),
    ("file tool write outside the repo", DENY, "Write", {"file_path": "../prompts/lead.md"}, {}),

    # Authority guards.
    ("push to main", DENY, "Bash", {"command": "git push origin main"}, {}),
    ("merge without the role", DENY, "Bash", {"command": "gh pr merge 12 --squash"}, {}),
    ("merge via the api without the role", DENY, "Bash", {
        "command": "gh api -X PUT repos/o/r/pulls/12/merge"}, {}),
    ("merge as the decider", ALLOW, "Bash", {"command": "gh pr merge 12 --squash"},
     {"VFI_ROLE": "decider"}),
    ("apply the approval label", DENY, "Bash", {
        "command": "gh pr edit 3 --add-label human-approved"}, {}),
    ("apply the approval label as the decider", DENY, "Bash", {
        "command": "gh pr edit 3 --add-label human-approved"}, {"VFI_ROLE": "decider"}),
]


def build_fixture(base: str) -> str:
    """A throwaway workspace: a checkout, a worktree inside it, prompts outside."""
    repo = os.path.join(base, "vfi")
    for directory in (".git", "docs/adr", "contracts", ".claude/hooks",
                      ".claude/worktrees/wt/.git"):
        os.makedirs(os.path.join(repo, directory), exist_ok=True)
    os.makedirs(os.path.join(base, "prompts"), exist_ok=True)
    shutil.copy(
        os.path.join(REPO, ".claude", "hooks", "protected-paths.txt"),
        os.path.join(repo, ".claude", "hooks", "protected-paths.txt"),
    )
    for name in ("ANCHORS.md", "AGENTS.md", "WORKPLAN.md", "docs/notes.md",
                 "docs/adr/TEMPLATE.md", ".claude/worktrees/wt/ANCHORS.md"):
        path = os.path.join(repo, name)
        with open(path, "w", encoding="utf-8") as f:
            f.write("fixture\n")
    return repo


def run(hook: str, repo: str, tool: str, tool_input: dict, extra: dict) -> str:
    scratch = os.path.join(os.path.dirname(repo), "scratch")
    os.makedirs(scratch, exist_ok=True)
    env = dict(os.environ, CLAUDE_PROJECT_DIR=repo, TMPDIR=scratch)
    env.pop("VFI_ROLE", None)
    env.update(extra)
    filled = {k: v.replace("{scratch}", scratch) for k, v in tool_input.items()}
    payload = json.dumps({"tool_name": tool, "tool_input": filled, "cwd": repo})
    done = subprocess.run(
        [sys.executable, hook], input=payload, capture_output=True, text=True,
        env=env, cwd=repo,
    )
    return DENY if done.returncode == 2 else ALLOW


def main() -> int:
    with tempfile.TemporaryDirectory(prefix="protect-paths-") as base:
        repo = build_fixture(base)
        failures = 0
        changed = 0
        print(f"{'case':<46} {'old':>5} {'new':>5} {'want':>5}")
        print("-" * 65)
        for name, expected, tool, tool_input, extra in CASES:
            old = run(INSTALLED, repo, tool, tool_input, extra) if os.path.exists(INSTALLED) else "n/a"
            new = run(DRAFT, repo, tool, tool_input, extra)
            mark = ""
            if new != expected:
                mark, failures = "  FAIL", failures + 1
            elif old != new:
                mark, changed = "  changed", changed + 1
            print(f"{name:<46} {old:>5} {new:>5} {expected:>5}{mark}")
        print("-" * 65)
        print(f"{len(CASES)} cases, {failures} failing, {changed} changed from the installed hook")
        return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
