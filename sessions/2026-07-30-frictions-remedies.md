# 2026-07-30 — frictions-remedies

Branch `frictions-remedies`. Direct human commission, no task file.

Wrote the remedies for `docs/proposals/agent-tooling-frictions.md`: an ADR
proposing that the protect-paths hook match write targets rather than command
text (Constitutional, so it stops for a human), the tested replacement hook and
its 32-case corpus in `docs/proposals/`, and a replacement lead prompt that
gives each teammate its own clone instead of a worktree.

The corpus passes: the observed false positives are now allowed, and three real
writes the installed hook missed are now refused (an interpreter writing an
anchor, a write to a worktree's own anchor, a redirect outside the workspace).
Nothing is installed — every artifact lands in a protected path or outside the
repo. The harness behaviours are marked upstream in the proposal; they need a
vendor bug report, not a commit here.

Three more frictions arrived mid-run and are folded in: the file tools refusing
the scratch directory (fixed in the same hook — scratch is now writable by
every tool), `$TMPDIR` differing between sandboxed and unsandboxed bash
(upstream), and a `refs/heads/main` refusal in an unrelated repository (kept, as
a documented limitation with its own test case).

Also added rows to `docs/proposals/README.md` for the new drafts, which is one
file past the stated boundary.

Continued on the same branch after the decider's review of PR #19 found the
draft hook fail-open in three places. Fixed: heredoc openers are now found by a
quote-aware scan, so a `<<WORD` inside a string no longer discards the rest of
the command, and an unterminated heredoc is refused rather than swallowing what
follows it; `sed -i.bak`, `-ni.bak` and `--in-place=.bak` are matched; `ed`,
`ex`, the `vi` family and `git config -f/--file` are modelled as writers. The
interpreter that writes *outside* the checkout stays open — closing it means
reading the code string — and is now stated in the hook's header and the ADR
instead of being covered by a claim the hook does not meet. Corpus is 56 cases,
0 failing.

Continued again after a second review found twelve commands the installed hook
refuses and that draft allowed. Rebuilt the hook on a refusal floor, per the
human's direction: the installed hook's decision stands, with one exemption —
the quoted operands of `git commit -m/--message` and `gh pr
create|comment|edit --body/--title` are blanked before the name scan and before
nothing else. The write-target model is gone, so its refusals are given back,
marked `regression:` in the corpus; the ADR is rewritten around the narrower
claim. Corpus is 87 cases, 0 failing, all twelve bypasses refused, and `--floor`
checks the guarantee over 3,024 generated commands.

`docs/proposals/agent-tooling-frictions.md` still describes the rejected fix and
the old case count; outside this task's boundary, left for the next run.
