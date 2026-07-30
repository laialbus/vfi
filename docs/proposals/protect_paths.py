#!/usr/bin/env python3
"""PreToolUse guard for VFI (GOALS.md, M0). DRAFT — NOT INSTALLED.

Destination: `.claude/hooks/protect_paths.py`, which is a protected path.
Installation is human-only, and only after `docs/adr/protect-paths-hook-matching.md`
is approved. Re-verify first with `docs/proposals/protect_paths_tests.py`.

Refuses:
  1. Any file-tool write to a protected path (list: protected-paths.txt) or to
     any location outside the project directory.
  2. Any Bash command that writes to a protected path or outside the project
     directory, and any git push to main.
  3. Merging a PR unless the run carries VFI_ROLE=decider. The wrapper sets
     the role outside the repo, so no agent can grant itself merge authority.
  4. Applying the human-approved label, from any agent, decider included.
     That label is the human sign-off for protected-path changes; only a
     human hand applies it.

Difference from the installed version: a Bash command is judged by what it
writes, not by what its text contains. Targets come from redirections and from
the operands of commands that write; text carried in a message or body
argument, or in a heredoc body, is never treated as a path. A command whose
targets cannot be seen (a patch, an interpreter, an unquotable string) falls
back to the old name matching, so the unparseable case stays refused.

Protected paths are matched relative to the git checkout the target sits in,
so a write to a worktree's own ANCHORS.md is refused like any other.

This is friction, not a security boundary. The OS sandbox confines shell
commands; the server (ruleset, required checks) is the backstop.

Exit 0 allows the tool call. Exit 2 blocks it; stderr is shown to the agent.
"""

import glob
import json
import os
import re
import shlex
import sys

WRITE_TOOLS = ("Edit", "Write", "NotebookEdit")

# How a command's operands relate to what it writes.
ALL_OPERANDS = "all"  # every operand is created, changed, or removed
DESTINATION = "dest"  # only the last operand (or -t's value) is written
UNSEEN = "unseen"  # writes targets not visible in the argument list

WRITERS = {
    "rm": ALL_OPERANDS,
    "rmdir": ALL_OPERANDS,
    "mv": ALL_OPERANDS,
    "touch": ALL_OPERANDS,
    "truncate": ALL_OPERANDS,
    "mkdir": ALL_OPERANDS,
    "chmod": ALL_OPERANDS,
    "chown": ALL_OPERANDS,
    "chgrp": ALL_OPERANDS,
    "shred": ALL_OPERANDS,
    "tee": ALL_OPERANDS,
    "cp": DESTINATION,
    "ln": DESTINATION,
    "install": DESTINATION,
    "patch": UNSEEN,
    "tar": UNSEEN,
    "unzip": UNSEEN,
    "rsync": UNSEEN,
    "python": UNSEEN,
    "python3": UNSEEN,
    "perl": UNSEEN,
    "ruby": UNSEEN,
    "node": UNSEEN,
    "awk": UNSEEN,
    "find": UNSEEN,
}
GIT_WRITERS = {
    "mv": ALL_OPERANDS,
    "rm": ALL_OPERANDS,
    "checkout": UNSEEN,
    "restore": UNSEEN,
    "apply": UNSEEN,
    "am": UNSEEN,
    "clean": UNSEEN,
    "stash": UNSEEN,
}
SHELLS = ("bash", "sh", "zsh", "dash", "ksh", "eval")
WRAPPERS = ("env", "timeout", "nice", "nohup", "sudo", "command", "stdbuf", "time", "xargs")
# Arguments that carry prose, not paths. Only for git and gh, where they are
# unambiguous; elsewhere a short flag of the same name means something else.
TEXT_FLAGS = (
    "-m", "--message", "-b", "--body", "-t", "--title",
    "--notes", "--subject", "--body-file", "-F", "--file",
)
SEPARATORS = (";", "&&", "||", "|", "&", "|&", "(", ")", "{", "}")

PUSH_TO_MAIN = re.compile(r"\bpush\b.*\b(main|master)\b")
MERGE_COMMAND = re.compile(r"\bpr\s+merge\b|\bapi\b.*/pulls/.*/merge\b")
APPROVAL_LABEL = re.compile(r"(--add-label\s+\S*human-approved|labels\b.*human-approved)")
HEREDOC = re.compile(r"<<-?\s*(['\"]?)(\w+)\1")
UNRESOLVABLE = re.compile(r"[$`]")


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


def protected_match(resolved: str, root: str, protected: list[str]) -> str | None:
    rel = os.path.relpath(resolved, checkout_root(resolved, root))
    for entry in protected:
        bare = entry.rstrip("/")
        if rel == bare or rel.startswith(bare + os.sep):
            return entry
    return None


def names_protected(text: str, protected: list[str]) -> str | None:
    for entry in protected:
        bare = entry.rstrip("/")
        base = os.path.basename(bare)
        if bare in text or (base and re.search(r"(?<![\w/.-])" + re.escape(base), text)):
            return entry
    return None


def temp_prefixes() -> list[str]:
    """Scratch space a write may land in. A run with its own TMPDIR gets that
    one and no more; exempting all of /tmp would exempt whatever else is
    parked there, including a checkout."""
    tmpdir = os.environ.get("TMPDIR")
    if tmpdir:
        return [os.path.realpath(tmpdir).rstrip(os.sep), "/dev"]
    return ["/tmp", "/private/tmp", "/dev"]


def newlines_to_separators(command: str) -> str:
    """Unquoted newlines separate commands; quoted ones are part of a string."""
    out: list[str] = []
    quote = None
    escaped = False
    for ch in command:
        if escaped:
            out.append(ch)
            escaped = False
        elif ch == "\\" and quote != "'":
            out.append(ch)
            escaped = True
        elif quote:
            out.append(ch)
            if ch == quote:
                quote = None
        elif ch in "'\"":
            out.append(ch)
            quote = ch
        else:
            out.append(";" if ch == "\n" else ch)
    return "".join(out)


def strip_heredocs(command: str) -> str:
    """Drop heredoc bodies: they are input text, never write targets."""
    pending = [m.group(2) for m in HEREDOC.finditer(command)]
    if not pending:
        return command
    kept: list[str] = []
    open_bodies: list[str] = []
    for line in command.split("\n"):
        if open_bodies:
            if line.strip() == open_bodies[0]:
                open_bodies.pop(0)
            continue
        kept.append(line)
        for _ in HEREDOC.findall(line):
            if pending:
                open_bodies.append(pending.pop(0))
    return "\n".join(kept)


def tokenize(command: str) -> list[str] | None:
    lexer = shlex.shlex(newlines_to_separators(command), posix=True, punctuation_chars=True)
    lexer.whitespace_split = True
    try:
        return list(lexer)
    except ValueError:
        return None


def is_redirect(token: str) -> bool:
    return ">" in token and set(token) <= set("<>&|")


def split_segments(tokens: list[str]) -> list[list[str]]:
    segments: list[list[str]] = [[]]
    for token in tokens:
        if token in SEPARATORS:
            segments.append([])
        else:
            segments[-1].append(token)
    return [s for s in segments if s]


def scannable(words: list[str]) -> str:
    """Segment text with prose arguments removed, for the git and gh guards."""
    kept: list[str] = []
    skip = False
    for word in words:
        if skip:
            skip = False
            continue
        if word in TEXT_FLAGS:
            skip = True
            continue
        if any(word.startswith(flag + "=") for flag in TEXT_FLAGS):
            continue
        kept.append(word)
    return " ".join(kept)


def resolve(target: str, cwd: str) -> list[str]:
    if UNRESOLVABLE.search(target):
        return []
    absolute = target if os.path.isabs(target) else os.path.join(cwd, target)
    if any(ch in target for ch in "*?["):
        matches = glob.glob(absolute)
        return [os.path.realpath(m) for m in matches]
    return [os.path.realpath(absolute)]


class Segment:
    """One simple command: what it writes, and whether we could see it all."""

    def __init__(self) -> None:
        self.targets: list[str] = []
        self.unseen = False
        self.nested: list[str] = []
        self.guard_text = ""


def analyze_segment(tokens: list[str]) -> Segment:
    result = Segment()
    words: list[str] = []
    index = 0
    while index < len(tokens):
        token = tokens[index]
        if is_redirect(token):
            index += 1
            if index < len(tokens):
                target = tokens[index]
                if not ("&" in token and target.isdigit()):
                    result.targets.append(target)
                index += 1
            continue
        words.append(token)
        index += 1

    while words and (os.path.basename(words[0]) in WRAPPERS or "=" in words[0]):
        popped = os.path.basename(words[0])
        words = words[1:]
        if popped in ("env", "xargs"):
            while words and words[0].startswith("-"):
                words = words[1:]
        elif popped == "timeout":
            while words and (words[0].startswith("-") or words[0].replace(".", "").isdigit()):
                words = words[1:]
    if not words:
        return result

    command = os.path.basename(words[0])
    args = words[1:]

    if command in SHELLS:
        for position, arg in enumerate(args):
            if arg in ("-c", "-lc", "-ic") and position + 1 < len(args):
                result.nested.append(args[position + 1])
                return result
        if command == "eval" and args:
            result.nested.append(" ".join(args))
            return result
        result.unseen = True
        return result

    if command in ("git", "gh"):
        result.guard_text = scannable(words)
        if command == "gh":
            return result
        subcommand = next((a for a in args if not a.startswith("-")), "")
        rule = GIT_WRITERS.get(subcommand)
        if rule == ALL_OPERANDS:
            rest = args[args.index(subcommand) + 1:]
            result.targets += [a for a in rest if not a.startswith("-")]
        elif rule == UNSEEN:
            result.unseen = True
        return result

    if command == "dd":
        result.targets += [a.split("=", 1)[1] for a in args if a.startswith("of=")]
        return result

    if command == "sed":
        if any(a == "--in-place" or re.fullmatch(r"-[a-zA-Z]*i[a-zA-Z]*", a) for a in args):
            result.targets += [a for a in args if not a.startswith("-")]
        return result

    rule = WRITERS.get(command)
    if rule is None:
        return result
    if rule == UNSEEN:
        result.unseen = True
        return result

    operands = [a for a in args if not a.startswith("-")]
    if rule == ALL_OPERANDS:
        result.targets += operands
        if command in ("rm", "tee") and not operands:
            result.unseen = True
    elif rule == DESTINATION:
        for position, arg in enumerate(args):
            if arg in ("-t", "--target-directory") and position + 1 < len(args):
                result.targets.append(args[position + 1])
                return result
            if arg.startswith("--target-directory="):
                result.targets.append(arg.split("=", 1)[1])
                return result
        if operands:
            result.targets.append(operands[-1])
    return result


def check_command(command: str, root: str, cwd: str, protected: list[str], depth: int = 0) -> None:
    if depth > 3:
        return
    tokens = tokenize(strip_heredocs(command))
    if tokens is None:
        # Unparseable: keep the old, blunt behaviour rather than allow blind.
        if names_protected(command, protected):
            deny(
                "This command could not be parsed and names a protected path; "
                "refused. Rewrite it as a simpler command."
            )
        return

    for segment_tokens in split_segments(tokens):
        segment = analyze_segment(segment_tokens)
        text = segment.guard_text
        if text:
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

        for target in segment.targets:
            for resolved in resolve(target, cwd):
                if resolved != root and not resolved.startswith(root + os.sep):
                    if any(resolved.startswith(p + os.sep) for p in temp_prefixes()):
                        continue
                    deny(
                        f"This command writes outside the project directory: "
                        f"{resolved}. Agents work only inside the workspace."
                    )
                entry = protected_match(resolved, root, protected)
                if entry:
                    deny(
                        f"This command writes to the protected path '{entry}'; "
                        "refused. Changing a protected path requires human "
                        "sign-off recorded in an ADR. Write an escalation instead."
                    )
            if not resolve(target, cwd):
                entry = names_protected(target, protected)
                if entry:
                    deny(
                        f"This command writes to a target naming the protected "
                        f"path '{entry}'; refused. Write an escalation instead."
                    )

        if segment.unseen:
            entry = names_protected(scannable(segment_tokens), protected)
            if entry:
                deny(
                    f"This command names the protected path '{entry}' and writes "
                    "targets the guard cannot see; refused. Use an explicit "
                    "command if a write elsewhere was intended."
                )

        for nested in segment.nested:
            check_command(nested, root, cwd, protected, depth + 1)


def main() -> None:
    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError:
        deny("protect_paths hook: could not parse tool-call JSON; refusing.")

    root = os.environ.get("CLAUDE_PROJECT_DIR") or data.get("cwd") or os.getcwd()
    root = os.path.realpath(root)
    cwd = os.path.realpath(data.get("cwd") or root)
    protected = load_protected(root)
    tool = data.get("tool_name", "")
    tool_input = data.get("tool_input") or {}

    if tool in WRITE_TOOLS:
        path = tool_input.get("file_path") or tool_input.get("notebook_path") or ""
        if not path:
            sys.exit(0)
        resolved = os.path.realpath(path if os.path.isabs(path) else os.path.join(cwd, path))
        if resolved != root and not resolved.startswith(root + os.sep):
            deny(
                f"Write outside the project directory refused: {resolved}. "
                "Agents work only inside the workspace."
            )
        entry = protected_match(resolved, root, protected)
        if entry:
            rel = os.path.relpath(resolved, checkout_root(resolved, root))
            deny(
                f"'{rel}' is a protected path (matches '{entry}'). "
                "Changing it requires human sign-off recorded in an ADR. "
                "Stop and write an escalation instead."
            )
        sys.exit(0)

    if tool == "Bash":
        check_command(tool_input.get("command", "") or "", root, cwd, protected)
        sys.exit(0)

    sys.exit(0)


if __name__ == "__main__":
    main()
