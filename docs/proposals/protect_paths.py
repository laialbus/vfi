#!/usr/bin/env python3
"""PreToolUse guard for VFI (GOALS.md, M0). DRAFT — NOT INSTALLED.

Destination: `.claude/hooks/protect_paths.py`, which is a protected path.
Installation is human-only, and only after `docs/adr/protect-paths-hook-matching.md`
is approved. Re-verify first with `docs/proposals/protect_paths_tests.py`.

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

Concretely, the hook refuses a Bash command when either of these holds:

  - the installed hook's push-to-main, merge, or approval-label guard matches
    the original text *or* the redacted text; or
  - the installed hook's write-hint-plus-protected-name scan matches the
    redacted text.

Redaction is skipped, leaving the floor untouched, whenever it cannot be done
soundly: a command containing a command substitution (`$(` or a backtick) is
never redacted, since a quoted argument can execute; an operand that is not
exactly one quoted string is never redacted; a flag is attributed only when the
segment's first words are the unquoted command that gives it its meaning; and a
command whose quotes the walk cannot close exempts nothing at all.

The predecessor of this draft modelled write targets instead, and an adversarial
review found twelve commands the installed hook refuses and that draft allowed,
across five mechanisms — glued separators, command substitution, wrapper flags,
`~`, and a heredoc marker in a comment. Every one of them was refused by the
installed hook for naming a protected path. That is the evidence behind starting
from refusal and subtracting, rather than from parsing and hoping.

## What this does not do

  - It does not see writes. `python3 -c "open('ANCHORS.md','w')"`,
    `ed ANCHORS.md < script`, `git config -f WORKPLAN.md`, and a redirect to
    `../prompts/` are all allowed here, exactly as the installed hook allows
    them. The sandbox is the layer that covers them.
  - Heredoc bodies are not redacted. Delimiting a body soundly needs the same
    shell parsing this design exists to avoid — the review's comment-marker
    bypass was a mis-delimited body — so a heredoc whose body names a protected
    path beside a write-capable word is still refused. Write the file with the
    Write tool and pass it as `--body-file`/`commit -F` instead.
  - A read-only command that names a protected path is still refused when it
    carries a write-capable word: `cp ANCHORS.md /tmp/` is a read, and it is
    refused. That is the cost of the floor, paid deliberately.
  - Prose outside the whitelisted flags is not exempt, including `git tag -m`,
    `gh issue`, and any body passed positionally.

This is friction, not a security boundary. The OS sandbox confines shell
commands; the server (ruleset, required checks) is the backstop.

Exit 0 allows the tool call. Exit 2 blocks it; stderr is shown to the agent.
"""

import json
import os
import re
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
    command = os.path.basename(names[0])
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
    """Where a write outside the checkout is allowed to land: this run's temp
    directory and the system's. A temp root that contains the checkout is not
    scratch for this run — otherwise a checkout living under /tmp would exempt
    everything beside it."""
    candidates = ["/tmp", "/private/tmp", "/var/folders", "/private/var/folders", "/dev"]
    tmpdir = os.environ.get("TMPDIR")
    if tmpdir:
        candidates.append(os.path.realpath(tmpdir))
    return [
        prefix.rstrip(os.sep)
        for prefix in candidates
        if not root.startswith(prefix.rstrip(os.sep) + os.sep)
    ]


def is_scratch(resolved: str, root: str) -> bool:
    return any(resolved.startswith(prefix + os.sep) for prefix in scratch_prefixes(root))


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


def check_bash(command: str, protected: list[str]) -> None:
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
        for base in (root, checkout_root(resolved, root)):
            entry = protected_match(resolved, base, protected)
            if entry:
                deny(
                    f"'{os.path.relpath(resolved, base)}' is a protected path "
                    f"(matches '{entry}'). Changing it requires human sign-off "
                    "recorded in an ADR. Stop and write an escalation instead."
                )
        sys.exit(0)

    if tool == "Bash":
        check_bash(tool_input.get("command", "") or "", protected)
        sys.exit(0)

    sys.exit(0)


if __name__ == "__main__":
    main()
