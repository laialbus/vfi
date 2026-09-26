# An escalation parks its task from the moment it is pushed, so `tasks.sh available` reads the `escalated/` refs as well as the files on main

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-25, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-25, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** what `scripts/tasks.sh available` reads to decide a task is
  parked, which is how the queue derives status; and the sweep's duty when it
  folds an `escalated/` ref onto main. It amends the accepted
  `worker-terminal-state.md` by one consequence, stated below, and edits no
  protected path: `scripts/tasks.sh` is not on the list, and WORKPLAN.md's
  words already hold — "a task an open escalation names is parked until the
  escalation is resolved".

## Context

M4-38 was handed out three times in one day. Each run stopped on the same wall
and pushed its escalation to an `escalated/` ref, as the terminal-state record
prescribes. But `available` parks a task only on an escalation file in the
checked-out tree, and a file reaches main only when a sweep folds it. Between a
stop and the next sweep the task looks claimable, and here that gap spanned two
sweeps, since the 2026-09-24 sweep ran two hours before the first ref existed.
The third run wrote the gap down and asked for it to be closed.

## Decision

`available` treats a task as parked when either holds: an escalation file on
the checked-out tree names it, or a ref `escalated/<id>-*` exists on origin. The
refs are read in the same `ls-remote` that already reads the claims, so this
costs no second moment. A stop therefore parks its task the instant it is
pushed, and a worker scanning the queue never sees a task another run has just
stopped on.

The sweep's side, so a ref cannot park a task forever: when a sweep folds an
`escalated/` ref's file onto main, it deletes that ref in the same act — the
file on main parks the task from then on, and deleting the file, which is how
an escalation is resolved, un-parks it. A ref left behind after its file is on
main is a leak the next sweep removes. Crash records, the wrapper's exit-0
refs, are named for the task too and park it alike; a sweep that judges one
spurious deletes it and says so.

`check` is untouched, so the queue gate does not move.

## Alternatives

**Leave it to the sweep's cadence.** A stop then costs every run until the next
sweep, and three runs re-deriving one wall is the cost this record exists to
stop paying.

**Have the worker push an escalation file to main.** Main is branch-protected
and nothing lands on it without the decider; that is the boundary and it is
not moved for this.

**Have the worker leave its task branch as the signal.** The third run did, and
it works only as long as nothing sweeps abandoned branches, which WORKPLAN.md
says something must.

## Consequences

**Easier.** A wall is met once. The decider's fold and the worker's stop agree
on what is parked.

**Harder.** `available` needs origin, which it already does for claims. The
sweep gains one deletion per fold, and the prompt that drives it, kept outside
the repository, is the owner's to align.

**Expensive to reverse.** Nothing.

## Enforcement

Not an anchor. `scripts/tests/tasks.sh` gains the cases: a task with an
`escalated/<id>-*` ref on origin is not offered; deleting the ref with no file
on main offers it again; the file on main parks it with or without the ref.
Task M4-49 writes both.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: it changes how the
  queue reads status, on an unprotected script, without moving the queue gate.
- **Checked:** WORKPLAN.md's claiming rules; `worker-terminal-state.md`'s
  decision, which names the ref as the stop's terminal state; `tasks.sh`'s
  `parked`, `read_branches` and `available`; the three refs and the sweep
  entries of 2026-09-24 and 2026-09-25.
- **Verdict and why:** accepted. The terminal state the workers were told to
  reach was invisible to the tool that hands out work, and the fix is to let
  the tool see it.
- **What would have changed it:** a sweep that ran between every pair of worker
  runs. None does, by design.
