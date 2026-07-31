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
reviewer most needs to see. A later worker hit the same matching from another
side: a command containing the literal `refs/heads/main`, pushing the base
branch of a scratch repository that has nothing to do with this one, was read
as a push to our main and refused.

The same file draws its other boundary too tightly. It refuses every write
outside the workspace, including the session scratchpad the harness itself
designates for temp files, so workers fall back to bash heredocs into `$TMPDIR`
to write a scratch file — shell as a workaround for the file tools.

**Fix:** parse the command for actual filesystem-write targets instead of
substring-matching the whole string, and let every tool write to the run's
scratch directory, not just bash. Written and tested; needs human sign-off. See
`docs/adr/protect-paths-hook-matching.md` and the two files it names. The
`refs/heads/main` case survives as a documented limitation with a test of its
own: the guard reads the branch name, not which repository owns it, and
believing a `git -C` path would be trusting the one argument an agent could
forge.

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

**Fix, upstream:** all three are Claude Code behaviour, and no change in this
repository fixes them. Report them to the tool vendor: spawn each teammate with
cwd already in its own worktree; repoint the sandbox write allowlist when the
session's worktree changes; never relocate a session's cwd without surfacing
it.

**Fix, here:** stop sharing one checkout. `docs/proposals/lead-prompt.md` gives
each teammate its own clone instead of a worktree, which removes the shared
`.git` and the EnterWorktree dance that the relocations happen inside of. It
carries the workarounds explicitly, and it forbids deleting any directory that
might be a live checkout.

## 3. Smaller, same theme

- **Upstream.** EnterWorktree creates a branch named `worktree-<name>`;
  claiming requires a branch named exactly for the task id, so every worker
  renames. A worktree flag for the branch name would remove a step every run
  repeats. Moot for teammates once they use clones, but still true for any
  session that makes a worktree.
- **Upstream.** `$TMPDIR` is not the same directory in sandboxed and
  unsandboxed bash, so a file written in one mode is invisible to the other
  through `$TMPDIR`. Absolute paths are the workaround, and are worth using
  for any scratch file that outlives a single command.
- **Upstream.** `gh` inside a sandboxed script hits a TLS failure even though
  the same `gh` invocation works when run directly; anything that layers
  scripts over `gh` (as the operational scripts do) inherits this.
- Worktrees accumulate under `.claude/worktrees/` with no cleanup rule — one
  is currently parked on a branch that no longer exists on origin. A sweep
  policy (delete when the branch is merged or gone, as the abandoned-claims
  rule already does for branches) would keep the directory honest.

## What is being asked

Item 1 is written and waiting on a human:

| Artifact | What it is |
| :--- | :--- |
| `docs/adr/protect-paths-hook-matching.md` | The decision. Constitutional — it changes protected-path enforcement, so the decider does not accept it. |
| `docs/proposals/protect_paths.py` | The replacement hook, a drop-in for `.claude/hooks/protect_paths.py`. |
| `docs/proposals/protect_paths_tests.py` | 56 tool calls run through both the installed hook and the replacement, including the decider's adversarial set from PR #19. Run it before installing. |

Item 2's repo-side half is `docs/proposals/lead-prompt.md`, replacing
`../prompts/lead.md`; its header lists the two prerequisites that go in with
it, one of them a change to `run.sh`. Everything marked **Upstream** above is
Claude Code behaviour that no change here can reach — those go to the tool
vendor as bug reports, not into this queue.

This file drains when the remedies are installed, not before. Until then it is
the record that keeps the next session from rediscovering these one worker at a
time.
