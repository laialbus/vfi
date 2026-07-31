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
do.

The draft is a refusal floor: it allows a Bash command only if the installed
hook allows that command's text with the prose arguments of `git commit -m` and
`gh pr create|comment|edit --body/--title` blanked out. So the `old` column
carries a claim this corpus checks case by case — **no row may read
`deny allow` unless the refusal is caused by prose in one of those
arguments**. Six rows do; they are the false positives the change exists for.
Every other loosening is a bug in the draft, whatever the `want` column says.

The `bypass:` block is the reason for that rule. It is the twelve commands an
adversarial review of the predecessor draft — which modelled write targets —
found that the installed hook refuses and that draft allowed, in all their
reported spellings. Each was confirmed under `/bin/bash` to really write. They
are ordinary rows here because the floor refuses them all for naming a
protected path, which is how the installed hook caught them in the first place.
Run them alone through the draft with:

    python3 docs/proposals/protect_paths_tests.py --bypasses

Cases marked `regression:` are refusals the predecessor draft gained by
modelling writes and this one gives back. They are allowed here because the
installed hook allows them; the sandbox is the layer that covers them.

Written cases check the draft where someone thought to look. The floor itself is
checked where nobody did:

    python3 docs/proposals/protect_paths_tests.py --floor

crosses every carrier shape in `CARRIERS` with every write in `writes()` and
asserts two things of each: the draft refuses it, and — the guarantee as stated
— whatever the draft allows, the installed hook allows on the same text with the
draft's redaction applied. A `--floor` breach is the failure this design exists
to make impossible.

Passing all three is necessary and not sufficient. Add a case for every refusal
or escape found later.

Each case runs with TMPDIR pointed at a scratch directory inside the fixture,
so the guard's temp exemption covers that directory and not the fixture's own
"outside the workspace" paths. `{scratch}` in a command is replaced with it.
"""

import contextlib
import importlib.util
import io
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
    # The exemption: prose in a commit message or a PR body. Every one of these
    # is a refusal observed in a session, and every one of them is a `deny
    # allow` row that the rule above permits.
    ("body names AGENTS.md with an arrow", ALLOW, "Bash", {
        "command": 'gh pr create --base main --head t --title "docs: layout" '
                   '--body "AGENTS.md says fetch -> normalize -> analyze"'}, {}),
    ("commit message names contracts/", ALLOW, "Bash", {
        "command": 'git commit -m "docs: record that contracts/ are versioned (v1 -> v2)"'}, {}),
    ("commit message names rm and AGENTS.md", ALLOW, "Bash", {
        "command": 'git commit -m "fix: stop refusing an rm of AGENTS.md named in a message"'}, {}),
    ("commit message given with --message=", ALLOW, "Bash", {
        "command": 'git commit --message="chore: rm the stale ANCHORS.md copy"'}, {}),
    ("commit message under a git -C", ALLOW, "Bash", {
        "command": 'git -C {scratch}/other commit -m "fix: an rm of ANCHORS.md in prose"'}, {}),
    ("pr edit body names the hooks directory", ALLOW, "Bash", {
        "command": 'gh pr edit 19 --body "an rm of .claude/hooks/ is out of scope"'}, {}),

    # The exemption stops at the quotes of a whitelisted flag. Each of these
    # would be allowed by a redaction one step wider than the draft's.
    ("a message followed by an rm", DENY, "Bash", {
        "command": 'git commit -m "docs: note" ; rm ANCHORS.md'}, {}),
    ("a message flag that is not the command's", DENY, "Bash", {
        "command": 'echo git commit -m "rm ANCHORS.md"'}, {}),
    ("a message whose body is a command substitution", DENY, "Bash", {
        "command": 'git commit -m "$(rm ANCHORS.md)"'}, {}),
    ("a message whose body is a backtick substitution", DENY, "Bash", {
        "command": 'git commit -m "`rm ANCHORS.md`"'}, {}),
    ("an unquoted message operand", DENY, "Bash", {
        "command": "git commit -m fix-rm-of-ANCHORS.md"}, {}),
    ("a prose flag on a command not in the whitelist", DENY, "Bash", {
        "command": 'git tag -a v1 -m "rm of ANCHORS.md"'}, {}),
    ("quoted heredoc marker then an rm", DENY, "Bash", {
        "command": 'git commit -m "note about <<EOF markers"\nrm ANCHORS.md'}, {}),
    ("quoted heredoc marker then rm -rf of the hooks", DENY, "Bash", {
        "command": 'gh pr create --body "use <<TOKEN here"\nrm -rf .claude/hooks/'}, {}),
    ("heredoc marker inside single quotes then an rm", DENY, "Bash", {
        "command": "python3 -c 'print(\"<<EOF\")' ; rm ANCHORS.md"}, {}),

    # The authority guards read the text as written, and the redacted text too,
    # so redaction can neither hide a push to main nor uncover one.
    ("a commit message describing a push to main", DENY, "Bash", {
        "command": 'git commit -m "the fixture is set up with git push origin main"'}, {}),
    ("redaction may not uncover a push to main", DENY, "Bash", {
        "command": 'git commit -m "a ; b" push origin main'}, {}),

    # A quote the walk cannot close exempts nothing, so the floor decides. The
    # second case is why that is better than refusing: the walk reads the
    # apostrophe as an open quote, and bash runs the command without complaint.
    ("an unterminated quote hiding an rm", DENY, "Bash", {
        "command": 'echo "note\nrm ANCHORS.md'}, {}),
    ("an apostrophe in a heredoc body", ALLOW, "Bash", {
        "command": "cat > docs/notes.md <<'EOF'\nit's fine\nEOF"}, {}),

    # Ordinary work must stay allowed.
    ("write inside the repo", ALLOW, "Bash", {"command": 'echo "note" > docs/notes.md'}, {}),
    ("read a protected file", ALLOW, "Bash", {"command": "cat ANCHORS.md"}, {}),
    ("read a file in a protected directory", ALLOW, "Bash", {
        "command": "cat .github/workflows/ci.yml | head"}, {}),
    ("diff two protected files", ALLOW, "Bash", {"command": "git diff ANCHORS.md AGENTS.md"}, {}),
    ("list a protected directory", ALLOW, "Bash", {"command": "ls -la .claude/hooks/"}, {}),
    ("prose containing a bare arrow", ALLOW, "Bash", {
        "command": 'gh pr comment 19 --body "the old count -> the new count"'}, {}),
    ("here-string naming an anchor", ALLOW, "Bash", {
        "command": 'grep -q x <<< "ANCHORS.md"'}, {}),
    ("a body passed as a file", ALLOW, "Bash", {
        "command": "gh pr create --base main --head t --body-file {scratch}/body.md"}, {}),
    ("heredoc body names protected paths", ALLOW, "Bash", {
        "command": "gh pr create --base main --head t --body-file - <<'EOF'\n"
                   "Adds a task to WORKPLAN.md and explains why .claude/hooks/ "
                   "and scripts/gates.sh are left alone.\nEOF"}, {}),
    ("push a task branch", ALLOW, "Bash", {"command": "git push origin M1-04"}, {}),
    ("write to the run's scratch directory", ALLOW, "Bash", {
        "command": "echo x > {scratch}/scratch.txt"}, {}),
    ("in-place edit of an unprotected file", ALLOW, "Bash", {
        "command": "sed -i '' 's/a/b/' docs/notes.md"}, {}),
    ("chained mkdir and write", ALLOW, "Bash", {
        "command": "mkdir -p docs/adr && echo x > docs/adr/foo.md"}, {}),
    ("a comment containing a quote", ALLOW, "Bash", {
        "command": "ls docs  # don't refuse this"}, {}),
    ("file tool write inside the repo", ALLOW, "Write", {"file_path": "docs/notes.md"}, {}),

    # The one loosening outside Bash: the run's scratch directory is writable
    # by the file tools, not only through a shell.
    ("file tool write to the scratch directory", ALLOW, "Write", {
        "file_path": "{scratch}/draft.md"}, {}),
    ("file tool write to a scratch file named like an anchor", ALLOW, "Write", {
        "file_path": "{scratch}/ANCHORS.md"}, {}),

    # Protected paths, by any route.
    ("rm a hook file", DENY, "Bash", {"command": "rm .claude/hooks/protect_paths.py"}, {}),
    ("rm -rf the hooks directory", DENY, "Bash", {"command": "rm -rf .claude/hooks"}, {}),
    ("append to the workplan", DENY, "Bash", {"command": 'echo "- [ ] M9-01" >> WORKPLAN.md'}, {}),
    ("in-place edit of an anchor", DENY, "Bash", {"command": "sed -i '' 's/x/y/' ANCHORS.md"}, {}),
    ("sed -i with an attached backup suffix", DENY, "Bash", {
        "command": "sed -i.bak 's/x/y/' ANCHORS.md"}, {}),
    ("rm an anchor from a nested shell", DENY, "Bash", {"command": "bash -c 'rm ANCHORS.md'"}, {}),
    ("rename the ADR template", DENY, "Bash", {
        "command": "mv docs/adr/TEMPLATE.md docs/adr/T2.md"}, {}),
    ("touch a file in a protected directory", DENY, "Bash", {
        "command": "touch contracts/v1.rs"}, {}),
    ("tee over an anchor", DENY, "Bash", {"command": "tee ANCHORS.md < /dev/null"}, {}),
    ("patch naming an anchor", DENY, "Bash", {"command": "patch AGENTS.md < fix.diff"}, {}),
    ("copy a protected file out to temp", DENY, "Bash", {
        "command": "cp ANCHORS.md {scratch}/copy.md"}, {}),
    ("echo writes a test file that names an anchor", DENY, "Bash", {
        "command": 'echo "ANCHORS.md" > docs/notes.md'}, {}),
    ("heredoc body carries a push to main", DENY, "Bash", {
        "command": "cat > docs/notes.md <<'EOF'\n"
                   "The fixture repo is set up with: git push origin main\nEOF"}, {}),
    ("file tool write to an anchor", DENY, "Write", {"file_path": "ANCHORS.md"}, {}),
    ("file tool write to an anchor in a worktree", DENY, "Write", {
        "file_path": ".claude/worktrees/wt/ANCHORS.md"}, {}),
    ("file tool write outside the repo", DENY, "Write", {"file_path": "../prompts/lead.md"}, {}),

    # The twelve commands the review found the predecessor draft allowing and
    # the installed hook refusing. All five mechanisms, in every reported
    # spelling. The floor refuses each one by name.
    ("bypass: heredoc marker in a comment", DENY, "Bash", {
        "command": "# example: cat <<EOF\nrm ANCHORS.md\nEOF"}, {}),
    ("bypass: glued semicolon and subshell", DENY, "Bash", {
        "command": "echo hi;(rm ANCHORS.md)"}, {}),
    ("bypass: glued and-and and subshell", DENY, "Bash", {
        "command": "echo hi&&(rm ANCHORS.md)"}, {}),
    ("bypass: glued pipe and subshell", DENY, "Bash", {
        "command": "echo hi|(rm ANCHORS.md)"}, {}),
    ("bypass: doubled semicolon", DENY, "Bash", {
        "command": "echo hi;;rm ANCHORS.md"}, {}),
    ("bypass: glued semicolon and arithmetic", DENY, "Bash", {
        "command": "echo hi;((rm ANCHORS.md))"}, {}),
    ("bypass: glued close-paren and and-and", DENY, "Bash", {
        "command": "(echo hi)&&rm ANCHORS.md"}, {}),
    ("bypass: command substitution in a string", DENY, "Bash", {
        "command": 'echo "$(rm ANCHORS.md)"'}, {}),
    ("bypass: backtick substitution in a string", DENY, "Bash", {
        "command": 'echo "`rm ANCHORS.md`"'}, {}),
    ("bypass: bare backtick substitution", DENY, "Bash", {
        "command": "echo `rm ANCHORS.md`"}, {}),
    ("bypass: substitution into an assignment", DENY, "Bash", {
        "command": "OUT=`rm ANCHORS.md`"}, {}),
    ("bypass: nice with a flag", DENY, "Bash", {"command": "nice -n 5 rm ANCHORS.md"}, {}),
    ("bypass: stdbuf with a flag", DENY, "Bash", {"command": "stdbuf -o0 rm ANCHORS.md"}, {}),
    ("bypass: sudo with a flag", DENY, "Bash", {"command": "sudo -u me rm ANCHORS.md"}, {}),
    ("bypass: command with a flag", DENY, "Bash", {"command": "command -p rm ANCHORS.md"}, {}),
    ("bypass: redirect through a tilde", DENY, "Bash", {"command": "echo x > ~/ANCHORS.md"}, {}),
    ("bypass: xargs taking its target from stdin", DENY, "Bash", {
        "command": "echo ANCHORS.md | xargs rm"}, {}),

    # Refusals the predecessor draft gained by modelling write targets, given
    # back with that model. Each is a real write that neither the installed
    # hook nor this one sees; the sandbox is what covers them. Kept as cases so
    # the cost of the floor is counted rather than described.
    ("regression: interpreter writing an anchor", ALLOW, "Bash", {
        "command": "python3 -c \"open('ANCHORS.md','w').write('')\""}, {}),
    ("regression: sed --in-place with a suffix", ALLOW, "Bash", {
        "command": "sed --in-place=.bak 's/x/y/' ANCHORS.md"}, {}),
    ("regression: sed -i in a combined short flag", ALLOW, "Bash", {
        "command": "sed -ni.bak 's/x/y/p' ANCHORS.md"}, {}),
    ("regression: ed driven from a script file", ALLOW, "Bash", {
        "command": "ed ANCHORS.md < script.ed"}, {}),
    ("regression: ex writing in silent mode", ALLOW, "Bash", {
        "command": "ex -sc wq ANCHORS.md"}, {}),
    ("regression: vim in ex mode writing a file", ALLOW, "Bash", {
        "command": "vim -es -c wq ANCHORS.md"}, {}),
    ("regression: git config writing a named file", ALLOW, "Bash", {
        "command": "git config -f WORKPLAN.md core.bare false"}, {}),
    ("regression: git config with an equals", ALLOW, "Bash", {
        "command": "git config --file=WORKPLAN.md core.bare false"}, {}),
    ("regression: redirect into the prompts directory", ALLOW, "Bash", {
        "command": "cat docs/notes.md > ../prompts/lead.md"}, {}),
    ("regression: unterminated heredoc", ALLOW, "Bash", {
        "command": "cat <<EOF\nsome text"}, {}),

    # Known limitation of both hooks, kept visible: an interpreter writing
    # outside the checkout names no protected path, so nothing matches.
    ("interpreter writing outside the workspace", ALLOW, "Bash", {
        "command": "python3 -c \"open('../prompts/lead.md','w').write('')\""}, {}),

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

    # Known limitation, kept as a case so it stays visible: the push guard
    # reads the branch name, not which repository it belongs to. A push to
    # main in some other checkout is refused too. Widening it would mean
    # trusting `git -C <path>`, which is the argument an agent would forge.
    ("push to main in an unrelated checkout", DENY, "Bash", {
        "command": "git -C {scratch}/other push origin refs/heads/main"}, {}),
]

# Rows allowed to read `deny allow`: the refusal is caused by prose inside a
# whitelisted flag's quotes, which is the exemption this draft exists for.
EXEMPT = {
    "body names AGENTS.md with an arrow",
    "commit message names contracts/",
    "commit message names rm and AGENTS.md",
    "commit message given with --message=",
    "commit message under a git -C",
    "pr edit body names the hooks directory",
    # Not Bash: the file tools gain the run's scratch directory.
    "file tool write to the scratch directory",
    "file tool write to a scratch file named like an anchor",
}


def build_fixture(base: str) -> str:
    """A throwaway workspace: a checkout, a worktree inside it, prompts outside."""
    repo = os.path.join(base, "vfi")
    for directory in (".git", "docs/adr", "contracts", ".claude/hooks",
                      ".github/workflows", ".claude/worktrees/wt/.git"):
        os.makedirs(os.path.join(repo, directory), exist_ok=True)
    os.makedirs(os.path.join(base, "prompts"), exist_ok=True)
    shutil.copy(
        os.path.join(REPO, ".claude", "hooks", "protected-paths.txt"),
        os.path.join(repo, ".claude", "hooks", "protected-paths.txt"),
    )
    for name in ("ANCHORS.md", "AGENTS.md", "WORKPLAN.md", "docs/notes.md",
                 "docs/adr/TEMPLATE.md", ".github/workflows/ci.yml",
                 ".claude/worktrees/wt/ANCHORS.md"):
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


def bypasses() -> int:
    """The review's bypass commands, through the draft alone."""
    with tempfile.TemporaryDirectory(prefix="protect-paths-") as base:
        repo = build_fixture(base)
        allowed = 0
        for name, _, tool, tool_input, extra in CASES:
            if not name.startswith("bypass:"):
                continue
            verdict = run(DRAFT, repo, tool, tool_input, extra)
            allowed += verdict == ALLOW
            print(f"{verdict:>5}  {tool_input['command']!r}")
        print(f"\n{allowed} of the review's bypass commands are allowed by the draft")
        return 1 if allowed else 0


# Shapes that carry a write somewhere in a command. `{write}` is substituted.
# Each is a way an earlier draft lost track of the command, or a way of hiding
# one that a later draft might.
CARRIERS = (
    "{write}",
    "echo hi; {write}",
    "echo hi;{write}",
    "echo hi;({write})",
    "echo hi&&({write})",
    "echo hi|({write})",
    "echo hi;;{write}",
    "echo hi;(({write}))",
    "(echo hi)&&{write}",
    'echo "$({write})"',
    'echo "`{write}`"',
    "echo `{write}`",
    "OUT=`{write}`",
    "bash -c '{write}'",
    "eval '{write}'",
    "# example: cat <<EOF\n{write}\nEOF",
    "cat <<EOF\nbody\nEOF\n{write}",
    "cat <<EOF\nbody\n{write}",
    "echo hi # a comment\n{write}",
    "{write} # a trailing comment",
    'git commit -m "a message" ; {write}',
    'git commit -m "a <<EOF marker"\n{write}',
    'git commit --message="a message" && {write}',
    'gh pr create --body "a body" ; {write}',
    'gh pr edit 1 --title "a title"\n{write}',
    'echo "a string" ; {write}',
    "echo 'a string' ; {write}",
    'git commit -m "$({write})"',
    "for f in a b; do {write}; done",
    "if true; then {write}; fi",
    "xargs -I{} {write} < list",
    "nice -n 5 {write}",
    "sudo -u me {write}",
    "command -p {write}",
    "stdbuf -o0 {write}",
    "timeout 5 {write}",
)


def writes(names: list[str]) -> list[str]:
    """A write to each protected path, spelled several ways."""
    spellings = []
    for entry in names:
        bare = entry.rstrip("/")
        spellings += [
            f"rm {bare}",
            f"rm -rf {bare}",
            f"echo x > {bare}",
            f"touch {bare}",
            f"cp /dev/null {bare}",
            f"sed -i '' s/x/y/ {bare}",
            f"echo {bare} | xargs rm",
        ]
        if bare == os.path.basename(bare):
            # Only for an entry at the root: `~/settings.json` is not the
            # protected `.claude/settings.json`, so it would not belong here.
            spellings.append(f"echo x >> ~/{bare}")
    return spellings


def load_hook(path: str, name: str):
    sys.dont_write_bytecode = True  # a test that litters the checkout is a defect
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verdict(hook, command: str, repo: str) -> str:
    payload = json.dumps({
        "tool_name": "Bash", "tool_input": {"command": command}, "cwd": repo,
    })
    saved = sys.stdin
    sys.stdin = io.StringIO(payload)
    code = 0
    try:
        with contextlib.redirect_stderr(io.StringIO()):
            hook.main()
    except SystemExit as stop:
        code = stop.code
    finally:
        sys.stdin = saved
    return DENY if code == 2 else ALLOW


def floor() -> int:
    """The guarantee, over generated commands rather than written ones.

    Two properties. First, the one the hook's header claims: whatever the draft
    allows, the installed hook allows on the same text with the draft's own
    redaction applied — so no shape of command can be allowed here and refused
    there. Second, the operational one: every generated command writes a
    protected path outside any prose argument, so every one must be refused.
    """
    with tempfile.TemporaryDirectory(prefix="protect-paths-") as base:
        repo = build_fixture(base)
        scratch = os.path.join(base, "scratch")
        os.makedirs(scratch, exist_ok=True)
        os.environ.update(CLAUDE_PROJECT_DIR=repo, TMPDIR=scratch)
        os.environ.pop("VFI_ROLE", None)
        draft = load_hook(DRAFT, "protect_paths_draft")
        installed = load_hook(INSTALLED, "protect_paths_installed")
        names = draft.load_protected(repo)
        breaches: list[str] = []
        allowed: list[str] = []
        total = 0
        for carrier in CARRIERS:
            for write in writes(names):
                command = carrier.replace("{write}", write)
                total += 1
                if verdict(draft, command, repo) == DENY:
                    continue
                allowed.append(command)
                redacted = draft.redact(command, draft.redaction_spans(command))
                if verdict(installed, redacted, repo) == ALLOW:
                    continue
                breaches.append(command)
        for command in breaches:
            print(f"FLOOR BREACH  {command!r}")
        for command in allowed[:20]:
            print(f"allowed       {command!r}")
        print(f"\n{total} generated commands, {len(breaches)} below the floor, "
              f"{len(allowed)} allowed")
        return 1 if breaches or allowed else 0


def main() -> int:
    if "--bypasses" in sys.argv:
        return bypasses()
    if "--floor" in sys.argv:
        return floor()
    with tempfile.TemporaryDirectory(prefix="protect-paths-") as base:
        repo = build_fixture(base)
        failures = 0
        loosened = 0
        tightened = 0
        print(f"{'case':<50} {'old':>5} {'new':>5} {'want':>5}")
        print("-" * 69)
        for name, expected, tool, tool_input, extra in CASES:
            old = run(INSTALLED, repo, tool, tool_input, extra) if os.path.exists(INSTALLED) else "n/a"
            new = run(DRAFT, repo, tool, tool_input, extra)
            mark = ""
            if new != expected:
                mark, failures = "  FAIL", failures + 1
            elif old == DENY and new == ALLOW:
                loosened += 1
                if name in EXEMPT:
                    mark = "  exempt"
                else:
                    mark, failures = "  FLOOR BREACH", failures + 1
            elif old == ALLOW and new == DENY:
                mark, tightened = "  tighter", tightened + 1
            print(f"{name:<50} {old:>5} {new:>5} {expected:>5}{mark}")
        print("-" * 69)
        print(f"{len(CASES)} cases, {failures} failing; "
              f"{loosened} allowed that the installed hook refuses "
              f"({len(EXEMPT)} permitted by the exemption), {tightened} newly refused")
        return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
