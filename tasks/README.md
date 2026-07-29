# Tasks

The queue. One file per task, named `<task-id>.md` — `M1-04.md`. WORKPLAN.md is
the format and the claim rules; this directory is only where the files live.

What belongs: an unclaimed or in-flight task, carrying every field WORKPLAN.md
requires. What does not: an index, a list of who has what, or any status field —
status is derived from branches and merge history, so it cannot go stale.

The branch that finishes a task deletes that task's file in the same diff. What
is here is what is left to do.
