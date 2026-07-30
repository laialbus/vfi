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
