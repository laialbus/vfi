# Proposal: fix the agent tooling frictions found in the M1 team session

Every worker in the 2026-07-29 team session hit the same few harness problems
and worked around them by hand. The workarounds succeeded, but two of them
normalize dangerous habits (disabling the sandbox, wording commands to dodge a
hook), so the causes deserve fixes. Each fix lands either in a protected path
or outside the repository, which is why this is a proposal and not a diff.

## 1. The protect-paths hook matches names, not targets

`.claude/hooks/protect_paths.py` refuses any write-capable command whose text
merely contains a protected path as a substring. Observed false positives: a
`gh pr create` whose body *mentioned* AGENTS.md, a `git commit -m` whose
message mentioned `contracts/`, and a PR body naming `scripts/gates.sh` while
explaining that it must not be created. Workers learned to reword messages or
route text through `--body-file` / `commit -F`, which hides exactly the words a
reviewer most needs to see.

**Fix:** parse the command for actual filesystem-write targets (or at minimum
exempt quoted message/body argument positions) instead of substring-matching
the whole string. Protected: needs human sign-off recorded in an ADR.

## 2. Sessions silently change worktree, and the sandbox does not follow

Three related behaviors, all observed:

- A worker spawns with its cwd in *another* agent's worktree and EnterWorktree
  refuses because the session is "already in" one; the workaround was
  ExitWorktree-then-reenter or a manual `git worktree add`.
- After switching, the bash sandbox's write allowlist stays pinned to the old
  worktree path, so every write in the new one fails with "Operation not
  permitted" until the worker disables the sandbox — training agents to reach
  for exactly the override the sandbox exists to make rare.
- The lead's shell was relocated between worktrees mid-session without any
  signal, twice. Files were written into the wrong checkout, and a cleanup of
  what looked like a stale directory was nearly an `rm -rf` of a live worktree
  (the sandbox blocked it).

**Fix:** spawn each teammate with cwd already in its own worktree; repoint the
sandbox allowlist when the worktree changes; never relocate a session's cwd
without surfacing it. All harness-side, outside the repository. Until fixed,
worker prompts should carry the workarounds explicitly, and any `rm` of a
worktree path should be treated as touching a live checkout.

## 3. Smaller, same theme

- EnterWorktree creates a branch named `worktree-<name>`; claiming requires a
  branch named exactly for the task id, so every worker renames. A worktree
  flag for the branch name would remove a step every run repeats.
- `gh` inside a sandboxed script hits a TLS failure even though the same `gh`
  invocation works when run directly; anything that layers scripts over `gh`
  (as the operational scripts do) inherits this.
- Worktrees accumulate under `.claude/worktrees/` with no cleanup rule — one
  is currently parked on a branch that no longer exists on origin. A sweep
  policy (delete when the branch is merged or gone, as the abandoned-claims
  rule already does for branches) would keep the directory honest.

## What is being asked

Human sign-off to fix the hook (item 1, ADR required), and harness changes for
items 2–3. Until then this file is the record that keeps the next session from
rediscovering these one worker at a time.
