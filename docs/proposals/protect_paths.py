#!/usr/bin/env python3
"""PreToolUse guard for VFI (GOALS.md, M0). DRAFT — NOT INSTALLED.

Destination: `.claude/hooks/protect_paths.py`, which is a protected path.
Installation is human-only, and only after `docs/adr/protect-paths-owns-grant.md`
is approved. Re-verify first with `docs/proposals/protect_paths_tests.py`.

This revision is the installed refusal-floor guard (accepted in
`docs/adr/protect-paths-hook-matching.md`, installed in PR #20, made
version-independent in PR #28) plus exactly one addition: the owns grant.
Everything above the grant section below is the installed hook's design,
unchanged.

Refuses:
  1. Any file-tool write to a protected path (list: protected-paths.txt) or to
     any location outside the project directory. Scratch is the exception:
     the run's temp directory is writable by every tool, not only by bash.
  2. Any Bash command the installed guard refuses, judged on its text with the
     prose arguments of a short whitelist of flags blanked out.
  3. Merging a PR unless the run carries VFI_ROLE=decider. The wrapper sets
     the role outside the repo, so no agent can grant itself merge authority.
  4. Applying the human-approved label, from any agent, decider included.
     That label is the human sign-off for protected-path changes; only a
     human hand applies it.

## The guarantee

For a Bash command, this hook allows only what the installed hook allows on the
same text with prose redacted:

    allows(this)  ⊆  allows(installed applied to prose-redacted text)

That is the whole design. The installed hook's substring matching is the floor;
this hook never reasons about what a command *writes*, so there is no parser
whose gaps could let a write through. The single exemption is redaction: the
span between the quotes of an operand of `git commit -m/--message` and of
`gh pr create|comment|edit --body/--title` is blanked before the protected-name
scan runs — and before nothing else. A refusal is waived only when it is
positively attributable to prose in one of those arguments.

Redaction is skipped, leaving the floor untouched, whenever it cannot be done
soundly: a command containing a command substitution (`$(` or a backtick) is
never redacted, since a quoted argument can execute; an operand that is not
exactly one quoted string is never redacted; a flag is attributed only when the
segment's first word is the bare, unpathed, unquoted command that gives it its
meaning — `./git` and `tools/git` earn nothing; and a command whose quotes the
walk cannot close exempts nothing at all.

## The owns grant (docs/adr/protect-paths-owns-grant.md)

The one loosening this revision adds. A write to a protected path is allowed
when the planner granted that path to the running task — all of these, or the
floor applies unchanged:

  - The checkout is on a branch named for a task, and `tasks/<branch>.md` AS
    COMMITTED ON origin/main lists the path under `owns:`. The working-tree
    copy is never read: a worker editing a task file locally grants nothing,
    because the grant is read with `git cat-file` against
    `refs/remotes/origin/main`.
  - The path is on GRANTABLE below — the gate runner, CI, contracts. The
    constitution, `docs/adr/TEMPLATE.md`, `.claude/settings.json`, and
    everything under `.claude/hooks/` (this file and the list it reads) are
    never grantable, whatever a task file says. A guard that can be taught to
    permit edits to itself is no guard one step later.
  - Every failure is the empty grant: no branch (detached HEAD), branch is
    main, no task file on origin/main, more than one `owns:` key, a key
    spelling the parser does not vouch for, git missing or timing out. The
    grant can only shrink on error, never widen.

Granularity differs by tool, in the fail-closed direction. The file tools
grant an exact target: the resolved path must fall inside an `owns` entry and
inside a GRANTABLE entry. Bash — which the floor deliberately never parses for
targets — grants only by erasing a protected entry from the name scan, so it
requires the `owns` entry to equal the protected entry: a task owning
`scripts/gates.sh` may run `sed -i` on it, but a task owning only
`.github/workflows/ci.yml` gets file-tool writes and no bash exemption for
`.github/`, because erasing `.github/` from the scan would exempt the whole
directory.

The authority guards are outside the grant entirely: push-to-main, merging
without the decider role, and the human-approved label refuse exactly as
before, on every branch, granted or not. The grant moves *delivery*; the
server-side label check is still what decides whether a protected change
*lands*.

Documented limitation, same class as the name-rebinding one below: the grant
trusts the local `refs/remotes/origin/main`. An agent that moves that ref at a
doctored commit has forged its own permission slip — deliberate circumvention,
which this hook does not model. The sandbox and the server-side check (which
reads the base branch on GitHub, not any local ref) are the layers for an
adversary; this hook is friction for an honest agent.

## What this does not do

  - It does not see writes. `python3 -c "open('ANCHORS.md','w')"`,
    `ed ANCHORS.md < script`, `git config -f WORKPLAN.md`, and a redirect to
    `../prompts/` are all allowed here, exactly as the installed hook allows
    them. The sandbox is the layer that covers them.
  - Heredoc bodies are not specially delimited. Delimiting a body soundly
    needs the same shell parsing this design exists to avoid, so body lines
    are read as ordinary text: one that names a protected path beside a
    write-capable word is refused, and one that happens to be shaped like a
    whitelisted prose flag is redacted like any other text, which can only
    remove a match, never smuggle a write. Write the file with the Write tool
    and pass it as `--body-file`/`commit -F` instead.
  - A read-only command that names a protected path is still refused when it
    carries a write-capable word: `cp ANCHORS.md /tmp/` is a read, and it is
    refused. That is the cost of the floor, paid deliberately. The grant does
    not change this for ungranted paths.
  - Prose outside the whitelisted flags is not exempt, including `git tag -m`,
    `gh issue`, and any body passed positionally.
  - Attribution reads the name, not the binary. A command that rebinds the
    word `git` inside its own text — a shell function, a `PATH=.` prefix, a
    `hash -p` — still earns the exemption for what is now an arbitrary
    executable. A blocklist of rebinding spellings would be the same losing
    game as parsing writes, so the limit is stated instead: this is deliberate
    circumvention, of the same order as an interpreter assembling a protected
    name at runtime, which both hooks already pass. The sandbox and the server
    are the layers for an adversary; this hook is friction for an honest one.

This is friction, not a security boundary. The OS sandbox confines shell
commands; the server (ruleset, required checks) is the backstop.

Exit 0 allows the tool call. Exit 2 blocks it; stderr is shown to the agent.
"""
from __future__ import annotations
import json
import os
import re
import subprocess
import sys

WRITE_TOOLS = ("Edit", "Write", "NotebookEdit")

# The installed hook's patterns, verbatim. They are the floor; changing one
# here would change what this hook is measured against.
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

# Flags whose quoted operand is prose. Keyed by the command that gives them
# that meaning, because `-m` and `-t` mean other things elsewhere.
COMMIT_FLAGS = ("-m", "--message")
GH_TEXT_FLAGS = ("--body", "--title")
GH_TEXT_SUBCOMMANDS = ("create", "comment", "edit")
GIT_GLOBALS_WITH_VALUE = ("-C", "-c", "--git-dir", "--work-tree", "--namespace")
GIT_GLOBALS_ALONE = ("-P", "--no-pager", "--paginate", "--bare")

# A quoted argument that contains one of these runs a command when the shell
# expands it, so its span is text the shell executes and must not be blanked.
SUBSTITUTION = re.compile(r"\$\(|`")
SEGMENT_BREAKS = ";|&()\n"

# The only protected entries a task file can grant (the ADR's fixed sublist).
# The constitution, the ADR template, the settings file, and .claude/hooks/
# are deliberately absent and must stay absent: everything else in this file
# is enforced by what is on this tuple.
GRANTABLE = ("scripts/gates.sh", ".github/", "contracts/")

# One grant lookup per process. The value is the owns list from origin/main's
# copy of the claimed task file, or [] when anything at all went wrong.
_GRANT_CACHE: dict[str, list[str]] = {}


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


class Word:
    """One shell word, with the span of its quotes when it has exactly one pair.

    `plain` is the word's text when nothing in it was quoted, and None
    otherwise; attribution reads only plain words, so a quoted `"git"` never
    passes for the command name.
    """

    def __init__(self, text: str, prefix: str, span: tuple[int, int] | None) -> None:
        self.text = text
        self.prefix = prefix
        self.span = span

    @property
    def plain(self) -> str | None:
        return self.text if self.span is None and self.prefix == self.text else None


def walk(command: str) -> tuple[list[list[Word]], bool]:
    """Split the text into segments of words, tracking quote state.

    The second value is False when a quote is never closed. Segments break on
    unquoted `;`, `|`, `&`, parentheses, and newlines; that split is used only
    to attribute a flag to the command it belongs to, never to decide a refusal,
    so a segmentation this misses cannot hide anything.
    """
    segments: list[list[Word]] = [[]]
    chars: list[str] = []
    prefix: list[str] = []
    quoted: list[tuple[int, int]] = []
    mixed = False

    def flush() -> None:
        nonlocal chars, prefix, quoted, mixed
        if chars:
            span = quoted[0] if len(quoted) == 1 and not mixed else None
            segments[-1].append(Word("".join(chars), "".join(prefix), span))
        chars, prefix, quoted, mixed = [], [], [], False

    index = 0
    length = len(command)
    while index < length:
        char = command[index]
        if char == "#" and not chars:
            while index < length and command[index] != "\n":
                index += 1
            continue
        if char in " \t":
            flush()
            index += 1
            continue
        if char in SEGMENT_BREAKS:
            flush()
            segments.append([])
            index += 1
            continue
        if char == "\\" and index + 1 < length:
            chars.append(command[index : index + 2])
            if not quoted:
                prefix.append(command[index : index + 2])
            else:
                mixed = True
            index += 2
            continue
        if char in "'\"":
            start = index + 1
            index += 1
            while index < length:
                if char == '"' and command[index] == "\\" and index + 1 < length:
                    index += 2
                    continue
                if command[index] == char:
                    break
                index += 1
            if index >= length:
                return segments, False
            quoted.append((start, index))
            chars.append(command[start - 1 : index + 1])
            index += 1
            continue
        chars.append(char)
        if not quoted:
            prefix.append(char)
        else:
            mixed = True
        index += 1
    flush()
    return segments, True


def prose_flags(words: list[Word]) -> tuple[str, ...]:
    """The flags whose operand is prose in this segment, or () when the segment
    is not one of the whitelisted commands. Anything unrecognised gives ()."""
    names = [word.plain for word in words]
    if not names or not names[0]:
        return ()
    # Only the bare, unpathed name is attributed: `./git` or `tools/git` is an
    # arbitrary executable that happens to be called git, and an exemption it
    # earned would hold exactly as long as the impostor behaves. Real pathed
    # invocations lose the exemption too and fall back to the floor, which is
    # a refusal — the fail-closed direction.
    command = names[0]
    if "/" in command:
        return ()
    if command == "git":
        index = 1
        while index < len(names) and names[index] and names[index].startswith("-"):
            option = names[index]
            if option in GIT_GLOBALS_WITH_VALUE:
                index += 2
            elif option in GIT_GLOBALS_ALONE or "=" in option:
                index += 1
            else:
                return ()
        if index < len(names) and names[index] == "commit":
            return COMMIT_FLAGS
        return ()
    if command == "gh":
        if len(names) >= 3 and names[1] == "pr" and names[2] in GH_TEXT_SUBCOMMANDS:
            return GH_TEXT_FLAGS
    return ()


def redaction_spans(command: str) -> list[tuple[int, int]]:
    """Spans between the quotes of whitelisted prose operands.

    An empty list means nothing is exempt and the floor applies to the text as
    written. That is what an unwalkable command gets: a quote the walk never
    closes exempts nothing, rather than being refused. Refusing would add no
    safety — the floor is already the verdict — and it would cost the common
    case of an apostrophe inside a heredoc body, which the walk reads as an
    unclosed quote and bash runs without complaint.
    """
    if SUBSTITUTION.search(command):
        return []
    segments, closed = walk(command)
    if not closed:
        return []
    spans: list[tuple[int, int]] = []
    for words in segments:
        flags = prose_flags(words)
        if not flags:
            continue
        for position, word in enumerate(words):
            if word.span and any(word.prefix == flag + "=" for flag in flags):
                spans.append(word.span)
                continue
            if word.plain not in flags or position + 1 >= len(words):
                continue
            operand = words[position + 1]
            if operand.span and operand.prefix == "":
                spans.append(operand.span)
    return spans


def redact(command: str, spans: list[tuple[int, int]]) -> str:
    """Blank each span, keeping the quotes around it so no two characters that
    were apart become adjacent — redaction can only remove a match, never make
    one."""
    if not spans:
        return command
    kept: list[str] = []
    cursor = 0
    for start, end in sorted(spans):
        kept.append(command[cursor:start])
        cursor = max(cursor, end)
    kept.append(command[cursor:])
    return "".join(kept)


def checkout_root(path: str, fallback: str) -> str:
    """The git checkout a path belongs to, so worktrees match their own list."""
    directory = path if os.path.isdir(path) else os.path.dirname(path)
    while True:
        if os.path.exists(os.path.join(directory, ".git")):
            return directory
        parent = os.path.dirname(directory)
        if parent == directory:
            return fallback
        directory = parent


def protected_match(resolved: str, base: str, protected: list[str]) -> str | None:
    rel = os.path.relpath(resolved, base)
    for entry in protected:
        bare = entry.rstrip("/")
        if rel == bare or rel.startswith(bare + os.sep):
            return entry
    return None


def scratch_prefixes(root: str) -> list[str]:
    """Where a write outside the checkout is allowed to land: this run's own
    scratch — $TMPDIR and the harness's claude temp roots — and nothing wider.
    The system-wide temp roots are not exempt, so a stranger's files under
    /tmp keep the protection of being outside the workspace. A scratch root
    that contains the checkout is not scratch for this run — otherwise a
    checkout living under /tmp would exempt everything beside it."""
    candidates = []
    tmpdir = os.environ.get("TMPDIR")
    if tmpdir:
        candidates.append(os.path.realpath(tmpdir))
    uid = os.getuid()
    for base in ("/tmp", "/private/tmp"):
        candidates.append("%s/claude" % base)
        candidates.append("%s/claude-%d" % (base, uid))
    return [
        prefix.rstrip(os.sep)
        for prefix in candidates
        if not root.startswith(prefix.rstrip(os.sep) + os.sep)
    ]


def is_scratch(resolved: str, root: str) -> bool:
    if not any(resolved.startswith(prefix + os.sep) for prefix in scratch_prefixes(root)):
        return False
    # A git checkout parked under a scratch root is not scratch: its files
    # keep the protection of the checkout they belong to. Bash can still work
    # in scratch repositories; only the file tools defer to the floor here.
    return checkout_root(resolved, "") == ""


# --------------------------------------------------------------------------
# The owns grant. Everything below returns the empty grant on any surprise.
# --------------------------------------------------------------------------

OWNS_KEY = re.compile(r"^owns:\s*(.*)$")
OWNS_KEY_VARIANT = re.compile(r"^\s*[Oo][Ww][Nn][Ss]\s*:")
OWNS_ITEM = re.compile(r"^\s*-\s*(.+?)\s*$")


def parse_owns(text: str) -> list[str]:
    """The owns list from task-file frontmatter, or [] when the file cannot be
    vouched for. Exactly one `owns:` key, spelled exactly, at column zero;
    inline `[a, b]` or a block list. A variant spelling (case, indentation,
    space before the colon) is counted as a sighting but parsed by nothing,
    so it yields the empty grant — the same fail-closed reading tasks.sh
    applies to its own fields."""
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return []
    sightings = 0
    owns: list[str] = []
    in_block = False
    for line in lines[1:]:
        if line.strip() == "---":
            break
        if OWNS_KEY_VARIANT.match(line):
            sightings += 1
            exact = OWNS_KEY.match(line)
            in_block = False
            if not exact:
                continue
            value = exact.group(1).strip()
            if value == "":
                in_block = True
            elif value.startswith("[") and value.endswith("]"):
                owns = [
                    item.strip().strip("'\"")
                    for item in value[1:-1].split(",")
                    if item.strip()
                ]
            continue
        if in_block:
            item = OWNS_ITEM.match(line)
            if item:
                owns.append(item.group(1).strip("'\""))
                continue
            if line.strip():
                in_block = False
    if sightings != 1:
        return []
    return [own.replace("\t", " ").strip() for own in owns if own.strip()]


def grant_owns(root: str) -> list[str]:
    """The claimed task's owns list, read from origin/main, cached per process.

    The branch is the claim (WORKPLAN.md), so the branch name keys the task
    file. main and a detached HEAD claim nothing. The working tree is never
    consulted — `git cat-file` against refs/remotes/origin/main reads what the
    planner committed, not what the run may have edited."""
    if root in _GRANT_CACHE:
        return _GRANT_CACHE[root]
    owns: list[str] = []
    try:
        head = subprocess.run(
            ["git", "-C", root, "symbolic-ref", "--short", "-q", "HEAD"],
            capture_output=True, text=True, timeout=5,
        )
        branch = head.stdout.strip()
        if head.returncode == 0 and branch and branch not in ("main", "master"):
            blob = subprocess.run(
                ["git", "-C", root, "cat-file", "blob",
                 "refs/remotes/origin/main:tasks/%s.md" % branch],
                capture_output=True, text=True, timeout=5,
            )
            if blob.returncode == 0:
                owns = parse_owns(blob.stdout)
    except Exception:
        owns = []
    _GRANT_CACHE[root] = owns
    return owns


def within(rel: str, entry: str) -> bool:
    bare = entry.rstrip("/")
    return rel == bare or rel.startswith(bare + os.sep)


def file_write_granted(rel: str, root: str) -> bool:
    """A file-tool target is granted when it falls inside an owns entry and
    inside a GRANTABLE entry. Both conditions read the target, so an owns
    entry wider than the grantable sublist grants only the grantable part."""
    owns = grant_owns(root)
    return any(within(rel, own) for own in owns) and any(
        within(rel, g) for g in GRANTABLE
    )


def bash_granted_entries(protected: list[str], root: str) -> set[str]:
    """Protected entries erased from the bash name scan: only those the task
    owns by the entry's own spelling. Bash is never parsed for targets, so the
    grant here is all-or-nothing per entry — which is why an owns entry
    narrower than the protected entry (a file inside .github/) grants bash
    nothing at all."""
    owns = {own.rstrip("/") for own in grant_owns(root)}
    grantable = {g.rstrip("/") for g in GRANTABLE}
    return {
        entry
        for entry in protected
        if entry.rstrip("/") in owns and entry.rstrip("/") in grantable
    }


def check_authority(text: str) -> None:
    if PUSH_TO_MAIN.search(text):
        deny(
            "Pushing to main is forbidden. Work on a task branch and open "
            "a PR; the decider merges."
        )
    if MERGE_COMMAND.search(text) and os.environ.get("VFI_ROLE") != "decider":
        deny(
            "Merging is the decider's job. This run is not the decider "
            "(VFI_ROLE is not 'decider'), so it merges nothing. Open a PR "
            "and stop."
        )
    if APPROVAL_LABEL.search(text):
        deny(
            "The human-approved label is the human's signature. No agent "
            "applies it, ever. If a protected-path change needs approval, "
            "write an escalation and wait."
        )


def check_bash(command: str, protected: list[str], root: str) -> None:
    redacted = redact(command, redaction_spans(command))
    check_authority(command)
    if redacted != command:
        check_authority(redacted)
    if not BASH_WRITE_HINTS.search(redacted):
        return
    for entry in protected:
        bare = entry.rstrip("/")
        base = os.path.basename(bare)
        named_base = base and re.search(r"(?<![\w/.-])" + re.escape(base), redacted)
        if bare in redacted or named_base:
            if entry in bash_granted_entries(protected, root):
                continue
            deny(
                f"This command names the protected path '{entry}' and "
                "contains a write-capable operation; refused. Use a "
                "read-only command if a read was intended. A commit message "
                "or a PR body may name it; other prose may not, so pass it "
                "with --body-file or commit -F. Changing protected paths "
                "requires human sign-off."
            )


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
            if is_scratch(resolved, root):
                sys.exit(0)
            deny(
                f"Write outside the project directory refused: {resolved}. "
                "Agents work only inside the workspace."
            )
        # Against the project root, as the installed hook does, and against the
        # checkout the path sits in, so a worktree's own ANCHORS.md is covered.
        # The grant is judged against the project root only: a claim held in
        # this checkout says nothing about files in some other checkout.
        for base in (root, checkout_root(resolved, root)):
            entry = protected_match(resolved, base, protected)
            if entry:
                rel = os.path.relpath(resolved, base)
                if base == root and file_write_granted(rel, root):
                    continue
                deny(
                    f"'{rel}' is a protected path "
                    f"(matches '{entry}'). Changing it requires human sign-off "
                    "recorded in an ADR. Stop and write an escalation instead."
                )
        sys.exit(0)

    if tool == "Bash":
        check_bash(tool_input.get("command", "") or "", protected, root)
        sys.exit(0)

    sys.exit(0)


if __name__ == "__main__":
    main()
