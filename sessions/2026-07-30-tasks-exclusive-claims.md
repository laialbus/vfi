# 2026-07-30 — tasks-exclusive-claims

Branch: `tasks-exclusive-claims`

`tasks.sh available` now honours the `exclusive` frontmatter field. It counts
claims in flight — branches on origin named for a task file still in the queue
— and prints an exclusive task only when that count is zero; while an exclusive
task's own claim is in flight it prints nothing at all. A stale branch whose
task file is gone is not a claim, so merged work does not block the queue.
Non-exclusive availability is unchanged, verified against the previous script
on the same fixtures.

Follow-up, per the decider's review: an unrecognised `exclusive` value now
refuses the whole listing on stderr with exit 3 instead of silently reading as
no. Only `yes`, `no`, and absent or empty are accepted.

Second follow-up, per the delta re-review: frontmatter is parsed and validated
in one view. Records carry a placeholder for empty fields so tabs cannot shift
them, tabs inside values become spaces before validation, every `exclusive`
occurrence is checked as it is seen, and a file that opens frontmatter without
an `id`, an unreadable task file, or a missing `tasks/` directory all refuse
the listing at exit 3 instead of reading as no work.
