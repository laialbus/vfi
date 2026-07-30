# The protect-paths hook judges a command by what it writes, not what it names

- **Status:** Proposed
- **Authority:** Constitutional
- **Proposed:** 2026-07-30, by `frictions-remedies`
- **Decided:** —
- **Touches:** `.claude/hooks/protect_paths.py`, a protected path, and the
  enforcement AGENTS.md names as the second layer containing a run.

## Context

The hook refuses any write-capable Bash command whose *text* contains a
protected path as a substring. Text is not a target. In the 2026-07-29 session
every worker hit this: a `gh pr create` whose body mentioned AGENTS.md, a
`git commit -m` whose message mentioned `contracts/`, a PR body explaining that
`scripts/gates.sh` must not be created yet. Workers learned to reword the
message or route it through `--body-file` and `commit -F`, which hides from the
reviewer exactly the words that explain the change.

The same blunt matching is also why the guard misses real writes. It never sees
a target, so it cannot tell that `python3 -c "open('ANCHORS.md','w')"` writes an
anchor, that a write into a worktree's own `ANCHORS.md` is the same file by
another path, or that a redirect into `../prompts/` leaves the workspace
entirely. Reviewing the guard for the false positives surfaced all three.

## Decision

The hook will parse each Bash command into simple commands and derive its write
targets: redirection targets, the operands of commands that write (`rm`, `mv`,
`cp`'s destination, `sed -i`, `tee`, `dd of=`, `git mv|rm`, and the rest), and
the same analysis recursively inside `bash -c` and `eval`. A target is refused
when it resolves inside a protected path or outside the project directory,
where "inside a protected path" is judged relative to the git checkout the
target sits in, so worktrees are covered by their own list. Arguments that
carry prose — `-m`, `--body`, `--title` and their kin on `git` and `gh` — and
heredoc bodies are never treated as paths. Where targets cannot be seen (a
`patch`, an interpreter, a command that will not tokenize) the hook keeps the
present name matching and refuses, so the unparseable case stays closed. The
merge guard, the push-to-main guard, and the human-approved label guard keep
their behaviour and are applied to `git` and `gh` commands rather than to
arbitrary text.

The replacement is written and tested: `docs/proposals/protect_paths.py`, with
`docs/proposals/protect_paths_tests.py` running 32 tool calls through both the
installed hook and the replacement. Nothing is installed by this ADR. A human
installs the file, because it is a protected path and because a guard an agent
can replace is not a guard.

## Alternatives

**Exempt only the message arguments, keep substring matching elsewhere.**
Smaller, and it fixes the observed refusals. Rejected because it keeps a rule
that is wrong in both directions: it would still refuse `cp ANCHORS.md /tmp/`
(a read) and still miss `python3 -c` (a write). The false positives are the
symptom; matching names instead of targets is the defect.

**Leave the hook alone and keep the workarounds.** Rejected: the workarounds
teach agents to phrase commands so a guard does not notice them, which is the
habit least compatible with unattended work.

**Drop the hook and rely on the server.** Rejected: branch protection catches a
protected-path change at review, after a run has spent itself on work that
cannot merge. The hook's value is stopping it at the keystroke.

## Consequences

Commit messages and PR bodies may name protected paths, so reviewers see the
reasoning. Three classes of real write that the guard missed are now refused.
`cp` of a protected file to a scratch path is allowed, since a read is not a
write. Scratch writes are exempt only under the run's own `TMPDIR`; a run with
no `TMPDIR` falls back to `/tmp`.

The guard is now roughly three times the code, and it is parsing shell, which
is a thing that is never finished. It stays friction, not a boundary: an agent
that wants past it can still hide a write in a command shape the parser does
not model, exactly as it could before. The sandbox and the server remain the
layers that actually hold, and the test corpus is what keeps the parser honest
as cases are found.

Reverting is cheap — the previous file is one `git revert` away — but the
workarounds come back with it.

## Enforcement

The hook is itself the enforcement of the protected-path list, so its
correctness is checked by `protect_paths_tests.py`: every case names the tool
call, the expected verdict, and what the installed hook did. Run it before
installing, and add a case for any refusal or escape found later. This is not
yet wired into CI — `.github/workflows/` is protected and out of an agent's
reach — so until a human adds it, the corpus is run by hand. That gap is
stated here rather than assumed away.

## Decision review

By the decider, not the proposer.
