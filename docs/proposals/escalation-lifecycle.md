# Proposal: escalations drain, sessions accumulate

Drafted at the human's request. Amends `docs/layout.md`, which is the declared
source of truth for placement, so it ships only with sign-off — either folded
into the M1-01 redo (that document is already being revised) or as its own
task once M1-01 merges.

## Problem

The layout defines how session entries and escalations are *created* — one
file per item — but not what happens to them afterward. Sessions are history
and can accumulate harmlessly. Escalations are open items awaiting a human;
with no lifecycle, `escalations/` fills with a mix of open and resolved stops
and no way to tell them apart. That is the same staleness the workplan designs
out of the task queue.

## Amendment

Add to the "One file per item" section of `docs/layout.md`, after the
paragraph on task-file deletion:

> An escalation is an open item, not history. The change that resolves it
> deletes its file in the same diff, exactly as the branch that finishes a
> task deletes the task file: `escalations/` holds only what is still
> waiting, and status is derived from presence, never from an annotation. A
> resolution that produces no repository change — a question answered, a
> decision recorded elsewhere — is closed by the human deleting the file, or
> by the first run that acts on the answer. The record of what was asked and
> how it was settled is the file's git history and the resolving pull
> request.
>
> Session entries are the opposite case. They are the durable record that
> makes deleting task files and escalation files safe, so they are never
> pruned. The directory is self-sorting by its date-prefixed names and
> nothing enumerates it; growth costs nothing.

## Why this shape

Presence-as-status is the pattern the repo already trusts: the task queue
drains itself, and no status field exists anywhere to go stale. Extending it
to escalations keeps one rule instead of two and adds no index, sweep job, or
archive directory to maintain.
