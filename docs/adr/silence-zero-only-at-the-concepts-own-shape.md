# A silence zero is supplied only at a period of the concept's own shape; at the other shape the concept is `Unknown`

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-19, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-19, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** the `short_term_investments` rows of the two tables in the
  accepted `which-filing-sets-the-value.md`, the sentence in the accepted
  `silence-beside-a-read-figure.md` that those tables' three silence rows
  stand, and the counts the accepted `what-normalize-emits.md` pins for CIK
  0002003750. None of the three is edited; this is the record the emit record
  says such a zero "is removed by". No contract byte, schema, anchor or gate
  moves.

## Context

The emit record has every period carry every concept, so a balance concept
takes a state at a duration and a flow at an instant, where no fact could ever
answer it. For the three concepts whose silence reads zero, that state is a
zero: on CIK 0002003750, 45 of 96 silence zeros — `short_term_investments` at
each of 19 durations, both dividend concepts at each of 13 instants. The record
prices it — "a consumer that ignores the published measure reads a zero that
looks right" — and declines to remove it by shape because two accepted records
pin such rows: the Rule 3 tables render `short_term_investments` at 0 over a
half-year and a quarter, and the silence record says they stand. It leaves the
judgement to a superseding record. The decider carried it for a human since
2026-09-16.

## Decision

Judged the plausible wrong answer, and removed at the reading rather than the
shape. The vocabulary's silence test licenses a zero because silence inside a
filing is ambiguous between the filer having none of the thing and the registry
not recognising the element, and it picks the safe wrong answer between those
two. At a period of the wrong shape neither cause is in play: no fact of that
shape can answer the concept, so the silence says nothing about the filer and
nothing about the registry. A zero there is not a reading of ambiguous silence;
it is a number with no support. So the zero is supplied only at a period of the
concept's own shape — an instant for a balance, a duration for a flow — and at
the other shape the concept is `Unknown`, carrying each answering filing's
attempt as any other `Unknown` does. The conditional pair is read the same way:
its zero is reached only at a duration, both members being flows.

Every period still carries every concept exactly once, in one of the three
states; the row shape, the key and v2 are untouched. Read over the period as
`silence-zero-supplied-once-per-period.md` has it, the rule is one condition
more on when the vocabulary supplies its zero, in the same place that record and
the silence record already put theirs.

What it supersedes, precisely: in the Rule 3 record's two tables, the
`short_term_investments` row is `Unknown`, not "0, no filing: the vocabulary's
zero"; the two dividend rows stand as silence zeros, both tables being at
durations. On CIK 0002003750: 51 silence values, every one at a period of the
concept's own shape, and 576 `Unknown`s; the 32 periods, the 12 and 8 read
values and their accessions are unchanged. On CIK 0001778784 nothing changes,
`short_term_investments` being `NotApplicable` there and the dividend zeros at
instants becoming `Unknown` alike.

The vocabulary's next version takes this into the `means` line of
`silence.zero` and `silence.conditional`, beside the changes
`declared-cash-dividends-stand-in.md` lists; until then the rule lives in the
reading, as the silence record's does.

## Alternatives

**Leave it to analyze.** M5 reads the measure and consumes a balance at instants
only, so a correct metric never sees these zeros. But the stored table would
carry 45 zeros per 32 periods that mean nothing, every consumer would have to
know that, and one that did not would read payout as covered at an instant. The
milestone's own words: a wrong answer that looks right.

**Each period carries only the concepts its shape can answer.** The emit record
rejected it because it reopened these rows; with the rows superseded here it
would still re-shape v2, which M4-37 is queued to publish, for a row-count
saving the storage boundary can make on its own.

**A fourth state.** The vocabulary closes the set at three, with "no fourth
state, no default, and no empty case", and `Unknown` already means an attempt
ran and found nothing, which is exactly what happened.

## Consequences

**Easier.** No zero anywhere in the table lacks support. The stored row for a
balance over a duration says it does not know, which is true.

**Harder.** M4-38's pinned counts move, and the fixture that renders the Rule 3
tables shows one `Unknown` where the tables said zero. Both are named above.

**Expensive to reverse.** Nothing is stored yet.

## Enforcement

Not an anchor. Task M4-43 makes the reading follow it and pins the counts
above; where one does not re-derive, it stops and escalates.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: it narrows when a
  published reading supplies a zero, as the accepted silence record did, and
  supersedes two table rows and three counts of Structural records.
- **Checked:** anchor 5, which it serves — a zero with no support is an
  unsourced value; the vocabulary's silence test as the silence record quotes
  it; the emit record's Alternatives, which invite exactly this record; the
  candidate-choice record's rule that a fact answers a concept only at its own
  shape; the arithmetic of 96 and 45 against 19 durations, 13 instants and
  three concepts.
- **Verdict and why:** accepted. The test that licenses the zero has no case
  to decide at a period no fact could answer, so the zero was never the test's
  answer there.
- **What would have changed it:** a metric in M5 that consumes a balance over a
  duration. None is named, and the vocabulary's measure says none can be.
