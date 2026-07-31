# The protect-paths hook keeps its refusals and exempts prose

- **Status:** Proposed
- **Authority:** Constitutional
- **Proposed:** 2026-07-30, by `frictions-remedies`. Rewritten the same day,
  after two adversarial reviews of PR #19 rejected the design first proposed
  here.
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

This ADR first proposed the obvious repair: parse each command, derive the
files it writes, and judge those. That draft was written, tested against a
56-case corpus, and reviewed twice. Both reviews broke it.

The first found three fail-open regressions — a heredoc marker inside a quoted
string switched the guard off for the rest of the command, and `sed -i.bak` was
not recognised as in-place. Those were fixed. The second review then found
**twelve commands the installed hook refuses and the fixed draft allowed**,
each confirmed under `/bin/bash` to really write the file, across five
mechanisms: a heredoc marker inside a `#` comment, glued punctuation that the
tokenizer emitted as one separator run (`echo hi;(rm ANCHORS.md)`), command
substitution never being recursed into, wrapper flags becoming the command name
(`nice -n 5 rm …`), and `~` not being expanded. Zero false positives in either
review; every failure was in the permissive direction.

That is the shape of the problem, and it is not a patch away. Reimplementing
bash's grammar fails open at every gap, and the gaps are found by whoever looks
next. The installed hook, blunt as it is, caught all twelve — because each
command plainly names the path it destroys.

## Decision

The hook keeps the installed matching as a **refusal floor** and subtracts from
it in one narrow, stated place.

For a Bash command the new hook guarantees:

    allows(new)  ⊆  allows(installed hook applied to prose-redacted text)

Redaction is the whole of the exemption. Before the protected-name scan — and
before nothing else — the span between the quotes of an operand of
`git commit -m/--message` and of `gh pr create|comment|edit --body/--title` is
blanked. The push-to-main, merge, and approval-label guards read the text as
written, and the redacted text as well, so redaction can neither hide an
authority violation nor manufacture one. There is no model of what a command
writes, so there is no parser whose gaps could let a write through: a shape the
redactor fails to recognise is a refusal, not an escape.

Redaction is skipped, leaving the floor untouched, wherever it cannot be done
soundly. A command containing `$(` or a backtick is never redacted, because a
quoted argument can execute. An operand that is not exactly one quoted string is
never redacted. A flag is attributed only when the segment's first word is the
bare, unpathed, unquoted name `git` or `gh` — a `./git` or `tools/git` is an
arbitrary executable and earns no exemption, and a genuinely pathed
`/usr/bin/git` loses it too, falling back to the floor's refusal, which is the
safe direction. Attribution reads the name as written, not what it resolves
to: a command that rebinds the word `git` in its own text (a shell function, a
`PATH=.` prefix, a `hash -p`) still earns the exemption. That limit is stated
in the hook header rather than patched, because a blocklist of rebinding
spellings is the same losing game this decision rejects, and the same
deliberate circumvention is already available through an interpreter
assembling a protected name at runtime, which both hooks pass. A command whose
quotes the walk
cannot close exempts nothing at all, which is the whole of its penalty: refusing
it outright would add no safety, since the floor is already the verdict, and it
would cost the common case of an apostrophe in a heredoc body.

One boundary moves with this, outside Bash. The run's scratch directory becomes
writable by the file tools rather than by bash alone: the harness designates it
for temp files, and the file tools refusing it is what pushes agents into
writing through shell heredocs instead. Scratch means this run's own roots —
`$TMPDIR` and the harness's claude temp directories — not the system-wide temp
trees. A scratch root that contains the checkout is not treated as scratch, and
a git checkout parked under a scratch root is not scratch either: its files
keep the protection of the checkout they belong to. What remains outside the
workspace stays refused.

The replacement is `docs/proposals/protect_paths.py`, with
`docs/proposals/protect_paths_tests.py` running 87 tool calls through both the
installed hook and the replacement, refusing every one of the twelve reported
bypasses, and cross-checking the guarantee over 3,024 generated commands.
Nothing is installed by this ADR. A human installs the file, because it is a
protected path and because a guard an agent can replace is not a guard.

## Alternatives

**Judge a command by the write targets it names.** What this ADR proposed
first. Rejected on evidence: two reviews of the implementation found fifteen
commands the installed hook refuses and it allowed. The claim it offers —
"refuses what writes a protected path" — is strictly stronger than the claim
the floor offers, and it could not be met. A weaker claim that holds is worth
more than a stronger one that does not.

**Keep the name-matching as an unconditional backstop underneath the target
model.** The second review's suggestion, and close to what is proposed here.
The difference is that the target model then earns nothing: with the backstop
in place, everything the parser adds is an extra refusal on top of a floor that
already caught all twelve bypasses, bought with several hundred lines that must
be right. Dropping it entirely is the same guarantee with none of the surface.
The cost is counted under Consequences.

**Exempt nothing; keep the hook as installed.** Rejected: the workarounds teach
agents to phrase commands so a guard does not notice them, which is the habit
least compatible with unattended work, and they strip the reasoning out of the
commit messages and PR bodies a reviewer reads.

**Drop the hook and rely on the server.** Rejected: branch protection catches a
protected-path change at review, after a run has spent itself on work that
cannot merge. The hook's value is stopping it at the keystroke.

## Consequences

Commit messages and PR bodies may name protected paths, so reviewers see the
reasoning. That is the only thing that gets easier, and it is the friction that
was actually reported.

Everything the target model refused and the floor does not is given back. Each
is a real write that the installed hook has always allowed and that the sandbox
is the layer to cover: an interpreter writing an anchor, `ed`/`ex`/`vim` driven
from a script, `git config -f`, `sed --in-place=.bak` and `-ni.bak`, a redirect
out of the workspace, and an unterminated heredoc. Ten cases in the corpus are
marked `regression:` so this is counted rather than described. One refusal from
that model is kept, because it needs no parsing: a file-tool write into a
worktree's own protected file.

The false positives that remain are the price of the floor, and they are
documented in the hook's header rather than discovered:

- A read-only command that names a protected path beside a write-capable word
  is still refused. `cp ANCHORS.md /tmp/` is a read; it is refused.
- Heredoc bodies are not redacted. Delimiting a body soundly needs the same
  shell parsing this design exists to avoid — the review's comment-marker
  bypass was a mis-delimited body — so a heredoc naming a protected path beside
  a write-capable word is refused. The workaround is now cheap and is the
  better habit anyway: write the file with the Write tool, which may write the
  scratch directory, and pass it as `--body-file` or `commit -F`. Content
  reaching the hook through a file is never scanned, and that does not change.
- Prose outside the four whitelisted flags is not exempt: `git tag -m`,
  `gh issue`, `gh pr review`, and any body passed positionally.
- A message given unquoted, or built with a command substitution, is not
  redacted.
- The push guard still reads a branch name, not the repository it belongs to,
  so `git -C <somewhere-else> push origin refs/heads/main` is refused even
  though that main is not ours. Telling the two apart means believing the `-C`
  path, which is the one argument an agent would forge.

The guard is 302 lines of code against the installed hook's 105 and the
rejected draft's 448 (counted the same way, docstrings aside), and none of it
models what a command writes: every line either copies the installed hook or
delimits the exemption. It remains friction, not a
boundary: an agent that wants past it can
still write through a path the text does not name, exactly as it could before.
What changes is that it can no longer do so by *confusing* the guard, because
there is nothing left to confuse.

Reverting is cheap — the previous file is one `git revert` away — but the
workarounds come back with it.

## Enforcement

The hook is itself the enforcement of the protected-path list, so its
correctness is checked by `protect_paths_tests.py`, in three parts: the corpus,
where no case may be allowed here and refused by the installed hook unless it is
named in `EXEMPT`; `--bypasses`, which puts the twelve reported escapes through
the draft alone and fails if any is allowed; and `--floor`, which crosses
command shapes with writes and checks the guarantee over thousands of generated
commands. Run all three before installing, and add a case for any refusal or
escape found later. This is not yet wired into CI — `.github/workflows/` is
protected and out of an agent's reach — so until a human adds it, the corpus is
run by hand. That gap is stated here rather than assumed away.

## Decision review

By the decider, not the proposer.
