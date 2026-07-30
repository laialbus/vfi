# 2026-07-30 — tasks-exclusive-claims

Branch: `tasks-exclusive-claims`

`tasks.sh available` now honours the `exclusive` frontmatter field. It counts
claims in flight — branches on origin named for a task file still in the queue
— and prints an exclusive task only when that count is zero; while an exclusive
task's own claim is in flight it prints nothing at all. A stale branch whose
task file is gone is not a claim, so merged work does not block the queue.
Non-exclusive availability is unchanged, verified against the previous script
on the same fixtures.
