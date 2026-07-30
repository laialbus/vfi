# 2026-07-30 — frictions-remedies

Branch `frictions-remedies`. Direct human commission, no task file.

Wrote the remedies for `docs/proposals/agent-tooling-frictions.md`: an ADR
proposing that the protect-paths hook match write targets rather than command
text (Constitutional, so it stops for a human), the tested replacement hook and
its 32-case corpus in `docs/proposals/`, and a replacement lead prompt that
gives each teammate its own clone instead of a worktree.

The corpus passes: four observed false positives are now allowed, and three
real writes the installed hook missed are now refused (an interpreter writing
an anchor, a write to a worktree's own anchor, a redirect outside the
workspace). Nothing is installed — every artifact lands in a protected path or
outside the repo. The three harness behaviours are marked upstream in the
proposal; they need a vendor bug report, not a commit here.

Also added rows to `docs/proposals/README.md` for the new drafts, which is one
file past the stated boundary.
