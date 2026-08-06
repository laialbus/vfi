# AGENTS

How agents work on VFI. This file is binding. If it conflicts with a task
instruction, this file wins. If it conflicts with ANCHORS.md, ANCHORS.md wins.

Agents run unattended, in the background, and in parallel. No human is watching
while you work. That shapes every rule below: the goal is to make mistakes
impossible or cheap, not to rely on someone catching them.

## The one rule that matters most

**When in doubt, stop and escalate. Never guess.**

A wrong guess here is worse than no progress. Most of the project's correctness
risk lives in normalization, where a plausible-looking wrong answer is the
failure mode — the kind of mistake that passes a glance and corrupts results
quietly. So: if a task is ambiguous, if two instructions conflict, if you need
information you don't have, or if doing the task well would require changing an
anchor, a contract, or a schema — stop, write an escalation, and exit. Do not
pick the reading that lets you keep going.

## How a run is contained

Agents run with full permission inside the workspace and none outside it.
Three layers hold that line:

1. **The OS sandbox.** Shell commands write only inside the workspace and
   reach only the domains allowlisted in `.claude/settings.json`. `gh` runs
   outside the sandbox (a macOS TLS limitation) and relies on the other two
   layers.
2. **The pre-edit hook.** No editing tool writes a protected path or any file
   outside the workspace. The list is `.claude/hooks/protected-paths.txt`;
   changing anything on it requires human sign-off recorded in an ADR.
3. **The server.** Main is branch-protected on GitHub: nothing merges without
   the decider, and protected paths require human review.

The hook is friction; the server is the boundary. An agent that hits a
refusal stops and escalates. It does not look for a way around.

## What you may not decide alone

These require an ADR, written and approved, before any code:

- Anything that touches an anchor (see ANCHORS.md).
- Any change to a contract between stages.
- Any change to a storage schema.
- Adding a second consumer to a component that has no explicit interface yet.
- Choosing or changing the GUI toolkit (that is ADR-001).

Everything inside a boundary — how a thing is built within its own crate — is
yours to decide. The architecture is set up so that most work is this second
kind.

## One run, one task

Each run does exactly one task from WORKPLAN.md. One. Small diffs are the whole
point of unattended work: they are reviewable, and they are cheap to revert. A run
that attempts three tasks is hard to review and hard to undo.

A task is done only when every gate passes (see below). Half a task is not a
task. If you cannot finish it, revert and escalate.

## Working in parallel with other agents

Several agents may run at once. You will not coordinate with them by talking. You
coordinate through the repo, and the rules make collisions structurally
impossible rather than something to untangle afterward.

- **Claim before you work.** Take a task by claiming it atomically — WORKPLAN.md
  defines the mechanism. If the claim fails, the task is already taken; pick
  another. Never start a task without claiming it first. Two agents on one task is
  a wasted night.
- **Stay inside your task's boundary.** Every task names the crate and files it
  owns. You edit those and no others. If the work pulls you outside that
  boundary, the task was mis-scoped — stop and escalate rather than reaching into
  another agent's area.
- **Never touch shared surfaces on a normal task.** Contracts, anchors, and
  schemas belong to everyone. A task that changes them is a special,
  single-threaded task with an approved ADR — never a side effect of other work.
  This is how parallel agents avoid fighting over the same lines.
- **Conflicting tasks are ordered, not parallelized.** If two tasks would touch
  the same files, WORKPLAN.md marks one as depending on the other so they never
  run at the same time. You do not resolve merge conflicts between agents; the
  workplan is arranged so they cannot happen.

Short version: parallel work is safe only because tasks are split by ownership up
front and anything cross-cutting is serialized. Respect the split and there is
nothing to collide with.

## Every run, start to finish

1. **Preflight.** Pull latest. Confirm the working tree is clean and CI is green.
   If it is not, do not build on a broken base — escalate and exit.
2. **Claim.** Take one task from WORKPLAN.md, atomically. If you cannot claim,
   try another or exit.
3. **Branch.** Work on a branch named for the task. Never commit to main.
4. **Work.** Stay inside the task's boundary. Keep the diff small.
5. **Gates.** Run all of them. All must pass.
6. **Finish.**
   - On success: push the branch, open a PR, write a session entry. Do not
     merge.
   - On failure you cannot fix: revert cleanly, write an escalation, exit.
   - The run ends when your turn ends. A headless session does not outlive its
     final message, so work deferred to a background task or monitor is work that
     never happens. Reach a terminal state — PR open, or escalation pushed — before
     you finish.
7. **Timeout.** Every run has a hard wall-clock limit. An agent stuck in a loop
   overnight is worse than an agent that did nothing. If you hit the limit, revert
   and exit.

You never merge your own PR. The decider reviews and merges — an agent outside
the worker pool that claims no tasks and writes no code. That separation is what
makes unattended work safe: the thing checking the work has no stake in it.

Some decisions are above the decider too. Those are marked in the ADR template
and stop for a human.

## The gates

A task is done only when all of these pass. They are the definition of done, not
a suggestion.

- The engine builds.
- All tests pass.
- The golden fixtures still produce their expected results.
- The dependency graph has no cycles (one-way flow holds).
- The analyze crate's deny-list holds (purity holds).
- Contract compatibility holds.
- The benchmark shows no regression past the set threshold.

If a gate does not yet exist for something you changed, that is itself an
escalation. Say so and stop, rather than shipping the change unchecked.

## What you write down

Where these live is set by the workspace layout. What goes in them is set here.

- **A session entry** — one per successful run: the task, the branch, and what
  changed. **Keep it short. A few lines.** These are read by later runs and by a
  human scanning history, and every word costs context that could have gone to
  the work. Say what changed, not how you arrived at it, and never restate the
  task or paste a diff.
- **An escalation** — whenever you stop on ambiguity, a conflict, a missing gate,
  or a decision above your authority. Say what you were doing, what stopped you,
  and what you would need to proceed. Short, but specific enough that a human can
  act without re-deriving the problem. Vagueness here costs a round trip; length
  does not help.
- **An ADR** — when a decision is above your authority, this is where the
  proposal goes. Propose it; do not implement until it is approved.

Nothing here is a diary. If a detail is already in the commit, the diff, or the
pull request, do not repeat it.

## If a gate fails

Revert your branch cleanly, so the repo is exactly as you found it. Write the
failure as an escalation. Exit. Do not force a gate green by weakening it — a
disabled test or a loosened deny-list is a worse outcome than a failed run, and it
removes the very check that keeps the next agent honest.
