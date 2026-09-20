# The equity statement's declared-cash dividend elements stand in for `dividends_paid`, and its preferred line for `preferred_dividends`

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-19, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-19, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** the registry's general mapping for two concepts, which the
  decider carried since 2026-09-14 as "a registry or vocabulary question" for a
  human. No contract byte moves; the vocabulary is not edited; the exclusions
  the registry record wrote down stand. The registry version changes, as it
  does on every entry added, and nothing pins the current digest.

## Context

Two fixture filers state dividends under elements the registry does not name.

CIK 0001778784, the dividend-suspension filer, a `bank`: `PaymentsOfDividends`
is stated year-to-date only, and `DividendsCommonStockCash` at every discrete
quarter as well, 2020 to 2022, with the same figures wherever both are stated.
In 2023 it states `DividendsCash` 0 at three periods — the suspension, tagged —
and in 2024 and 2025 `Dividends` 0. Under the registry as committed,
`dividends_paid` is `Unknown` at every discrete quarter the filer declared in,
because the per-share figure is read and the pair condition holds the zero
back; and the suspension is a silence zero naming no filing, the same number the
filer tagged, with no filing behind it.

CIK 0001859199 states `DividendsPreferredStock` through 2025 and 2026, and
`Dividends` equal to it over 2025. Its `preferred_dividends` is `Unknown` at
every period. Its one `DividendsCash` fact is at an instant and answers no flow.

The registry record's exclusions name `CommonStockDividendsPerShareCashPaid`
and `PaymentsOfDividendsMinorityInterest`; the declared-cash elements were not
weighed there.

## Decision

The registry, not the vocabulary, and three `stand_in` entries:

- `dividends_paid`: `us-gaap:DividendsCash`, and `us-gaap:DividendsCommonStockCash`.
- `preferred_dividends`: `us-gaap:DividendsPreferredStock`.

These are the equity statement's declared cash dividends. The concept is cash
paid from financing. They are the same population and differ by timing — the
declared-but-unpaid balance at each period end, which for a regular payer nets
out inside a year — and a stand-in is, by the registry's own definition, "the
concept only for a filer holding nothing else". Exact before stand-in drops them
wherever `PaymentsOfDividends` is stated in the same filing, and the resolved
value records the tag and the reading, so a reader sees a declared figure standing
in for a paid one rather than mistaking it for one. `DividendsCommonStockCash`
understates for a filer that also pays preferred; a filer stating both stand-ins
and no exact comes to `Unknown`, two survivors, recoverable by a per-filer
`exclude` as the candidate-choice record provides. `DividendsPreferredStock` is
the dividends alone without the accretion and redemption adjustments, the same
reading the existing `PreferredStockDividendsIncomeStatementImpact` stand-in
already takes.

**One exclusion worth writing down.** `Dividends` is not `dividends_paid`: it is
dividends declared for all classes, cash and stock, paid and unpaid. On CIK
0001778784 its 2024 and 2025 zeros leave both dividend concepts to the silence
zero, which is the same number by the vocabulary's pair reading.

**What this leaves to the next vocabulary version, on record so it is not lost.**
v2 is fixed by the accepted emit record and M4-37 publishes it; the vocabulary
changes below are exclusive tasks with their own record, after v2:

1. A currency `dividends_declared` concept, the equity statement's own line, so a
   declared figure no longer stands in for a paid one.
2. The pair reading's zero requires a witness that the period's facts carry the
   statement the concept lives on: `dividends_paid` reads zero only where the
   same answering filing reaches `operating_cash_flow` at that duration, and
   `dividends_declared_per_share` only where it reaches `net_income` or
   `earnings_per_share_diluted` there. This is what removes the Q4 2020 and Q4
   2021 zeros on CIK 0001778784, where the 10-K answers the quarter through
   selected quarterly data and no cash flow statement, and the filer paid.
   It reads facts, not statements, so it keeps to "silence is silence in the
   fact set".
3. `dividends_paid` is `Unknown`, not zero, where `preferred_dividends` resolves
   non-zero, since the concept's meaning takes preferred and common together.

## Alternatives

**The vocabulary now.** The right home for a declared concept, but v2's content
is fixed and queued; a change now stalls M4-37 and M4-38 for a concept no metric
consumes yet.

**Per-filer assertions.** One line per filer per period does not scale, and a
Q4 assertion would be a year less nine months, which Rule 1 forbids
constructing.

**Leave it.** A zero where the filer paid reads payout as perfectly covered and
breaks a streak the filer kept, which is the one outcome the vocabulary's silence
test exists to refuse.

## Consequences

**Easier.** Quarterly `dividends_paid` on filers that state the equity line at
the quarter. A tagged suspension reads 0 from a filing rather than from
silence. `preferred_dividends` resolves on CIK 0001859199.

**Harder.** `dividends_declared_per_share` at the 2023 periods on CIK 0001778784
becomes `Unknown`: the pair is no longer silent alike and the paid figure is
zero, so neither published condition supplies a zero. Safe and imprecise, which
is what the vocabulary says an unread concept gets.

**Expensive to reverse.** Nothing is stored yet.

## Enforcement

Not an anchor. The registry gate reads the entries. Task M4-42 writes them and
re-derives, by a throwaway run through the crate's own surface, the claims
below; where one does not re-derive, it stops and escalates rather than
adjusting the claim. On CIK 0001778784: the quarter 2020-04-01 to 2020-06-30
reads `dividends_paid` 583,000 from `DividendsCommonStockCash` in
`0001778784-21-000046` beside a read 0.03; the year 2022 reads 1,989,000 from
`DividendsCommonStockCash` in `0001778784-24-000006`, the later filing being the
one that states only the stand-in; the year 2023 reads 0 from `DividendsCash` in
`0001778784-24-000006` beside an `Unknown` per-share figure; the years 2024 and
2025 remain the silence zero for both; the quarter 2020-10-01 to 2020-12-31
remains the silence zero for both, the priced cost item 2 above removes. On CIK
0001859199: the year 2025 reads `preferred_dividends` 122,877 from
`DividendsPreferredStock` in `0001213900-26-026653`, and `dividends_paid` is
unchanged at every period. The counts M4-38 pins on CIK 0002003750 do not move:
no fixture but these two carries any of the four elements.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: general registry
  entries, which tasks add in the ordinary course, plus a direction for a
  vocabulary version that is proposed here and written nowhere.
- **Checked:** anchor 5, since a stand-in records its tag and reading and
  nothing enters uncited; the registry record's rule forms, stand-in reading and
  exclusions; the candidate-choice record's exact-before-stand-in step and its
  two-survivors rule; the vocabulary's meanings and pair conditions for the two
  dividend concepts and `preferred_dividends`; every fact under the four elements
  in both fixtures, with accessions.
- **Verdict and why:** accepted. Where the exact element is stated the stand-in
  is dropped, and where it is not, a declared cash figure with its tag on it is
  nearer the truth than a zero with nothing behind it.
- **What would have changed it:** a filer in the fixtures whose declared and
  paid figures differ at a period where both are stated. None does.
