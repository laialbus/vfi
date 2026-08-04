# WORKPLAN

The task queue. This file says what work is available, who owns what while it
happens, and what order things must go in. It does not say how to do the work.

This file is a specification, not a whiteboard. Workers read it. They do not
edit it to claim a task — claiming happens through branch creation (below). Only
the planner writes here. That is what keeps several agents from colliding on this
one file.

Tasks are derived from the active milestone in GOALS.md. When a milestone is
finished, the queue is refilled from the next one.

## How a task is written

Every task has these fields, written as machine-readable frontmatter at the top
of the file so a tool can read them without opening the whole thing. An agent
scanning for work reads headers only, and opens in full just the one task it
claims.

A task missing any field is not ready to be claimed, and an agent that finds one
should escalate rather than interpret it.

- **id** — short and stable, e.g. `M1-04`. Becomes the branch name.
- **title** — one line, plain.
- **milestone** — which milestone in GOALS.md this advances. A task that
  advances none does not belong here.
- **owns** — the crates and paths this task may modify. Exclusive. Nothing
  outside this list may be touched.
- **depends on** — task ids that must be merged first. Empty means it can start
  now.
- **exclusive** — yes or no. Yes means it touches a shared surface and runs
  alone, with no other task in flight. Default is no.
- **acceptance** — what specifically must be true when this task is done.
  Concrete enough to check. This is per-task and sits alongside the gates, which
  always apply and are never restated here.

### Example

```
---
id: M1-02
title: Create the Rust workspace, one crate per stage
milestone: M1
owns:
  - Cargo.toml
  - crates/
depends_on: [M1-01]
exclusive: no
acceptance:
  - The workspace compiles, with one empty crate per pipeline stage.
  - Crate names follow one stated convention, recorded in the layout doc.
  - No crate depends on another yet; edges are added only by later tasks.
---
```

## Claiming a task

Claim by creating and pushing the task's branch, named for its id. This is
atomic: if two agents try the same task, the second push is rejected because the
ref already exists, and that agent knows it lost and picks another.

The guarantee that matters: a claim either succeeds outright or fails outright,
never both. Any mechanism with that property is acceptable. This one is chosen
because it needs no extra service and folds claiming and branching into one step.

A task whose branch exists is taken. A task whose dependencies are unmerged is
not available yet. A task an open escalation names is parked until the
escalation is resolved.

## Rules for writing good tasks

These exist so that parallel work cannot collide, rather than colliding and being
untangled afterward.

- **One run, one task.** Size each task so a single agent can finish it, gates
  and all, inside one run. If it is too big, split it.
- **Ownership must not overlap.** Two tasks available at the same time never list
  the same path under `owns`. If they would, one depends on the other instead.
- **Shared surfaces are exclusive.** Anything touching anchors, contracts,
  canonical concept definitions, storage schemas, or the gates themselves is
  marked exclusive and runs alone. These are never a side effect of ordinary
  work.
- **Acceptance is written before the work starts,** not after. A criterion
  invented at the end describes what happened rather than what was wanted.
- **Say what, not how.** The task states the outcome. The agent decides the
  implementation.

## Who fills this in

The planner turns the active milestone into tasks. This is higher-leverage than
any single task — a bad split wastes a whole run across the whole team — so
early on a human supervises it. It moves to a planner agent once the shape it
produces is trusted.

Workers never add tasks. An agent that thinks a task is missing writes an
escalation.

## Where the queue lives

Not here. This file is the format; the queue is the `tasks/` directory, one file
per task, named by its id — `tasks/M1-04.md`. A task is added by adding a file,
never by editing a shared list, so two planners can work at once without
colliding.

This file is protected and changes rarely. The queue changes constantly. That is
why they are separate.

The available work is what is in `tasks/` whose dependencies are merged, whose
branch does not yet exist, and that no open escalation names. No status field is written anywhere, and no index file
is kept — status is derived from the branch, the merge history, and the open escalations, so it cannot
go stale or disagree with reality.

To see what is claimable, run `scripts/tasks.sh available`. It reads the
directory and git and prints the answer. There is no list to maintain, because a
maintained list would be a second source of truth and would drift.

## Retiring a task

**The branch that finishes a task deletes that task's file, in the same diff.**
When the work merges, the file is gone. The queue drains itself: no cleanup job,
no agent remembering to tidy, and no stack of finished tasks for future agents to
read past.

Every task implicitly owns its own file. Deleting it is not reaching outside the
declared boundary.

Nothing is lost by deleting. The record of what was done is the pull request, the
commit, and the session entry. Keeping the task file after merge would be a third
copy of the same history.

## Abandoned claims

A run that dies mid-task leaves its branch behind, and the task then looks
claimed forever. So a claim must be released when it fails: the run deletes its
own branch on any exit that is not a merged pull request, and any branch left
behind past a set age is swept.

Without this, tasks leak out of the pool one at a time and the queue quietly
shrinks.
