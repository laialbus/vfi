# A filer's periods are the ones its own facts date, every filing that reports a period answers it, and a value is the one the latest filing states

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-09-04, by `M4-14`
- **Decided:** —
- **Touches:** one field the fetch → normalize contract does not carry, proposed
  here as `v2` and not written. A contract version is above a worker, which is
  why this is a proposal and not a commit. Nothing else published changes: the
  vocabulary keeps its bytes, the registry gains no field, and the ruleset
  itself is inside `vfi-normalize`.

## Context

`docs/adr/candidate-choice.md` is accepted and draws this record's boundary in
one sentence: "Alignment names a period and a filing. This rule turns that pair
into a value or an absence." It then says what it leaves: "The alignment ruleset
decides: which canonical periods exist, which filing answers each, what a later
filing restating an earlier one does, a fiscal-year change, and an amendment. It
reads `form`, `filed` and `accession` to prefer one filing over another."

The seam falls there because of what the fetch boundary withholds. A filing
publishes many durations and many instants and nothing in the eight published
fields says which of them is the period the filing is about, so that record
refused to pick one from inside a filing — "the longest duration, the
latest-ending one, the one nearest 365 days — and each of those is a heuristic
that reads well and is wrong for some filer." Every rule below is written under
that refusal: nothing here picks a period out of one filing by its shape.

Three surfaces are fixed and this record reads them rather than reopening them.

**`contracts/fetch-normalize/v1.toml`.** Eight fields per fact — taxonomy, tag,
unit, period, value, accession, form, filed — and a period is "an instant, which
is one date, or a duration, which is two", never both and never neither. Fetch
filters nothing and parses nothing. `form` crosses because "the amendment
ruleset has to tell an original from an amendment, and the form is the only
field that says which it is"; `filed` crosses because "the restatement ruleset
orders two reports of one period by when they were filed". Those two fields have
no other consumer, and this is the record that reads them.

**`contracts/canonical-concepts/v1.toml`.** A `Value` carries "the source tag it
was read from, the filing it was reported in, and the rule that set it". It does
not carry the period. `Unknown` is built "from an attempt that ran, and nothing
else" and carries the candidates and the rule that declined each.

**`docs/adr/candidate-choice.md`.** Resolution runs over "the facts of one
filing", for one concept and one period, in four steps. A flow is answered only
by a duration and a balance only by an instant; "the dates must be equal,
exactly, as the contract publishes them"; every operand of a `sum` or a
`difference` is read at the same period and in the same filing; and a caller's
period "whose dates the alignment ruleset reads off the facts resolves; one it
constructs from a calendar of its own resolves to `Unknown` for every concept."
One entry may declare that it `answers` "the period of the filing it was
reported in" rather than its own, which exists for the cover-page share count.

What is readable now is the merged company-facts fixture,
`fixtures/fetch/every-fact-a-filer-reported`, and — for the same filer, CIK
0002003750 — the submissions fixture under
`fixtures/fetch/a-ticker-resolves-to-its-history`. Read rather than assumed,
they say six things this record uses.

- **A filer's facts carry far more periods than it has reporting periods.** The
  1,643 facts fall at 50 distinct periods: 24 durations and 26 instants. Among
  them are three lease terms carried by `OperatingLeasePayments`, two calendar
  tax years carried by the effective-tax-rate elements, three instants that are
  lease dates, and ten instants that are cover dates — one per filing, each
  equal to that filing's `filed`.
- **Two of those strays are annual-length.** The tax year 2024-01-01 to
  2024-12-31 is 366 days and the lease term 2024-12-01 to 2025-11-30 is 365; the
  filer's own fiscal years, 2023-10-01 to 2024-09-30 and 2024-10-01 to
  2025-09-30, are 366 and 365. Nothing about a duration's length separates them.
- **A period appears in many filings, and the number of facts at it varies by
  more than an order of magnitude.** The quarter 2024-10-01 to 2024-12-31 is
  carried by six filings: 40 facts in the 10-Q filed 2025-02-14, 37 in the 10-Q
  filed 2026-02-20, and two in each of the other four. Those two are
  `NetIncomeLoss` and an other-comprehensive-income line — the equity
  roll-forward and the cash flow statement quoting a period they are not about.
- **A period the filer reported in full may have no filing of its own.** The
  quarter 2023-10-01 to 2023-12-31 is reported with 38 facts, and the filing
  that reports it is the 10-Q for the *following* year's first quarter, filed
  2025-02-14. The filer's own 10-Q for that quarter does not exist: its first
  periodic report is the 10-Q filed 2024-05-08.
- **The fixture already holds a restatement, and it is a correction.** For the
  half-year 2023-10-01 to 2024-03-31, ten mapped elements are carried by both
  the 10-Q filed 2024-05-08 and the 10-Q filed 2025-05-14. Nine agree to the
  digit. `EarningsPerShareDiluted` does not: −0.002 first, −0.0015 a year later.
  Net income (−77,402) and diluted shares (50,301,639) are unchanged in both,
  and their quotient is −0.001539 — so the first figure was wrong and the second
  is right. The quarter 2024-01-01 to 2024-03-31 carries the same correction, 0
  to −0.0004.
- **The report's own period is published, and not by the document that publishes
  the facts.** The company-facts document publishes numeric facts only — this
  filer's `dei` elements are `EntityCommonStockSharesOutstanding` and
  `EntityPublicFloat`, and there is no `DocumentPeriodEndDate` because it is not
  a number. The submissions document, which the funnel already retrieves,
  publishes `reportDate` per filing, and for all nine periodic filings the two
  fixtures share it is exactly the period end this filer's statements use:
  2024-03-31, 2024-06-30, 2024-09-30, 2024-12-31, 2025-03-31, 2025-06-30,
  2025-09-30, 2025-12-31, 2026-03-31.

This is Structural rather than Constitutional: it proposes one field on a
contract, which is the tier's own example. No anchor is edited, no gate
weakened, no protected-path entry changed, no scope in GOALS.md moved.

## Decision

### What this ruleset is asked

Given a filer and every fact fetch handed over for it: **which periods does this
filer have, which filings answer each, and where two filings answer one period
differently, which one sets the value.** It answers with a set of periods, and
per period and concept a `Value` or an absence that candidate choice produced.
It never reads a concept's meaning, never reads a tag to prefer one filing over
another, and never produces a number no filing states.

Three rules, in order. Everything after them is those three applied.

### Rule 1 — a period is a pair of dates the facts carry, and the set is the set at which something resolves

**A canonical period is a period a fact of this filer carries, as the contract
publishes it: one date for an instant, two for a duration.** Nothing is
constructed, nothing is rounded to a calendar, nothing is synthesised. A quarter
the filer never reported is not a period, and a fourth quarter is not built from
a year less nine months — candidate choice already refuses to build a flow by
subtraction and this record does not build one either.

**Of those, a period is the filer's when at least one concept its kind admits
resolves to a `Value` at it, through entries that answer the period asked for.**
The clause at the end is what keeps the cover dates out: an entry that answers
"the period of the filing it was reported in" borrows a period named elsewhere,
so what it resolves is no evidence that the period exists. Everything else falls
out of resolution as candidate choice already defines it, asked concept-first —
this ruleset never asks the registry about a tag, which the registry record
forbids in as many words: "It is never asked about a tag."

This is a statement of which set, not of how to compute it. Read against the
fixture, at the starting content `docs/adr/tag-concept-registry.md` publishes
for a registry that does not exist yet, the facts under elements the mapping names
fall at 32 of the 50 periods — 19 durations and 13 instants — and the 18 they
miss are exactly the strays: three lease terms, two calendar tax years, three
lease instants, and all ten cover dates. Resolution can only narrow that set further,
never widen it, because every rule beyond the tag makes a candidate harder to
find.

Two periods in the surviving 32 are worth naming, because they show the rule
declining to correct the filer. The instant 2022-09-30 carries one fact, equity
of 60,000, a year before this filer was incorporated. The duration 2022-10-01 to
2023-09-30 carries a net loss of −40,502, the same figure the filer also reports
against its 24-day inception stub. Both are periods the filer stated, both
resolve to what it stated, and both name the filing that said so. Dropping them
would be this ruleset deciding that a filer's own dates are wrong, which is not
a judgement it has any evidence for.

### Rule 2 — every filing that reports a period answers it, and each answer is read inside one filing

**A filing answers a period when it carries at least one fact whose own period
is exactly that period.** Resolution then runs once per period, per concept, per
answering filing, over that filing's facts and no other's.

One filing at a time, and not the union of them, because a `sum` or a
`difference` composed across filings is a number no filing states — the same
objection candidate choice makes to a composition that mixes a quarter with a
year-to-date. Every filing that reports the period and not only the newest,
because the newest is often the thinnest: on the fixture the last filing to
mention the quarter 2024-10-01 to 2024-12-31 carries two facts at it, so a rule
that took that filing alone would resolve `net_income` for the quarter and
report revenue, assets, equity and cash flow as absent for a quarter the filer
published in full.

### Rule 3 — the value is the one the latest filing states, and an absence never displaces one

For one period and one concept, among the answering filings whose attempt
produced a `Value`: **the value is the one from the filing with the greatest
`filed`, and the `Value` names that filing.** This is the whole of what a
restatement does. The earlier figure is not deleted and not corrected in place;
it simply is not what the filer says now, and re-resolution says what the filer
says now.

Three clauses complete it.

**Silence in a later filing is not a restatement.** An attempt that produced an
`Unknown` — no fact answering the period, a partial composition, a contest that
did not settle — does not displace a `Value` from an earlier filing. A later
filing that does not mention a line makes no claim about it, and reading its
silence as a withdrawal would turn the ordinary thinness of a comparative column
into the erasure of a figure the filer never took back.

**An amendment is a filing.** Its facts carry its own accession, its own `form`
ending in `/A`, and its own `filed`, and it wins for what it states because it
states it later. No rule prefers it and none discounts it. The common
Part III-only 10-K/A carries no facts and changes nothing, and needs no rule to
say so.

**`form` is read once, to break a tie on `filed`.** Where two answering filings
share a `filed` date and their attempts disagree, a filing whose form is
another's with `/A` appended supersedes it — that is what an amendment is, and
`filed` is a date with no time on this boundary, so a same-day amendment is not
otherwise orderable. Any tie that survives is `Unknown` carrying both
accessions. Never the larger, never the more common, never the one that agrees
with last year.

`filed` is compared as a date, which means normalize parses it. That is where
the fetch record already puts the parse — "the parse belongs where the rules
that depend on it live" — and this is the rule that depends on it.

### What a fiscal-year change does

**Nothing, and that is the decision.** A filer that moves its year end files a
transition period and then years on a new calendar, and each of those periods
arrives dated by its own two dates like every other. The old quarters and the
new quarters do not collide, because nothing here labels a period `FY2025` or
`Q3` — a period is its dates and has no other name. The transition period is
short and sits in the table at its own length; the year before it and the year
after it sit at theirs.

This is the reason Rule 1 reads dates and refuses a calendar, stated as the case
that would break any other reading. A ruleset that derived a filer's periods
from a fiscal year end would have to notice the change, decide when it took
effect, and decide what the old calendar's unreported quarters become — three
judgements with nothing on the boundary to settle them, in the one part of the
milestone where a plausible wrong answer is the failure mode.

What it costs is that a filer's periods stop being uniform, and no consumer may
assume one annual period per calendar year. That cost is real and it is named
under the open question below rather than absorbed here.

### What a resolved value records, and what it does not

**The `Value` names the filing whose attempt won, in the field the vocabulary
already requires it to carry.** Nothing is added to any contract. The vocabulary
fixes a `Value` as carrying "the source tag it was read from, the filing it was
reported in, and the rule that set it", and the filing this ruleset chose is
exactly the filing it was reported in — so a value taken from a comparative
column two years later names that later accession, and replaying it means
re-reading that filing.

Everything this ruleset did is recomputable from what the value already names.
Given the filer's facts and the registry version the value carries, the
answering filings are re-derivable, every losing attempt is re-runnable, and the
comparison on `filed` is deterministic. The losers are recomputed, not
remembered, which is the same answer candidate choice gave for the same
question.

Two absences of record, both deliberate, both named again under Enforcement. The
`Value` does not carry the period; the period is the key of the row the value
sits in, and where that key lives is the storage boundary's. And nothing on the
value says which revision of these three rules produced it — at one revision the
accession is a complete account of what alignment chose, and the moment there is
a second revision that changes a number, a method version on the `Value` is a
`canonical-concepts` version bump and a decision, which this record does not
take on behalf of a revision that does not exist.

### The one field the boundary withholds

The cover-page share count is the case these rules cannot make honest, and it is
not a corner.

`shares_outstanding` is a balance, and the registry reaches it two ways:
`us-gaap:CommonStockSharesOutstanding`, an instant at a period end, and
`dei:EntityCommonStockSharesOutstanding`, the cover-page count, which candidate
choice lets an entry declare as answering the period of the filing it was
reported in. That record priced the cost: "a count at the filing's cover date,
which for a 10-K in the fixture is up to three months after the year end". Three
months is what it costs **when the filing is the one whose own period is being
asked about** — and which filing that is, is the variable this record sets.

On the fixture it goes wrong. The instant 2023-12-31 is answered by five
filings, the latest filed 2025-08-20, and no filing tags
`CommonStockSharesOutstanding` at that date. So the cover-page entry is the only
candidate, and Rule 3 hands `shares_outstanding` at 2023-12-31 the count on a
cover page nineteen months later: 60,500,000, where the filer's own counts at
2023-09-30 and 2024-03-31 are both 60,000,000. A wrong number that looks right,
which is the failure this milestone exists to prevent.

The condition that fixes it is one sentence and this record sets it, because it
is a statement about which filing answers which period and nothing else:
**a filing answers a period through an entry that answers the period of the
filing it was reported in only when that filing's own period of report ends on
the period asked for.** It is not a window, it has no number in it, and it
restores exactly the bound candidate choice assumed.

**At `fetch-normalize` v1 the condition cannot be stated, so it is never met and
such an entry is never a candidate.** The eight published fields do not say what
period a filing reports; that is the withholding the seam was drawn around. The
consequence is `shares_outstanding` resolving to `Unknown` for a filer that tags
no period-end count, which is an absence M5 shows with its reason, and no number
at all changes: at v1 the concept is an `Unknown` carrying its candidates and at
v2 it is a `Value` naming a rule id, and the two are not mistakable for each
other.

**So this record proposes `contracts/fetch-normalize/v2.toml`, adding one field
per fact:**

| field | what it is | why it crosses |
| --- | --- | --- |
| `report_period_end` | the date the filing's period of report ends, as the submissions document publishes it in `reportDate`, unparsed | the condition above. Without it, the one entry the vocabulary needs for `shares_outstanding` cannot be admitted without admitting a count from any later filing |

Four things about the proposal, so the decider is not asked to take any of them
on trust.

- **It is published, not derived.** `reportDate` is a field of the submissions
  document the funnel already retrieves for every filer it admits, and it
  crosses as the characters that document publishes. Checked against the two
  fixtures for CIK 0002003750: for all nine periodic filings both documents
  cover, `reportDate` is the end of the period the filer's statements report.
- **It repeats per fact, like the three provenance fields beside it.** The fetch
  record already rejected carrying filings once and joining by accession,
  because "a join has a failure mode — a fact pointing at a filing that is not
  there — whose result is a value whose filing cannot be named". The same
  objection applies here and the same answer is taken.
- **It is one date, not the filing history.** The fetch record declined the
  submissions history whole, because carrying it "would be a second copy of the
  same fact with nothing checking that the two agree". One field per fact is not
  a second copy of anything the facts already carry — no fact states the period
  its filing reports — so the objection does not reach it.
- **A filing with no `report_period_end` is the v1 case.** The submissions
  fixture shows the field empty on 20 of this filer's 41 filings — the
  registration statements, the correspondence, the ownership filings. None of
  them reaches the facts document, but a filing that carried facts and no report
  date would simply fail the condition, and the entry would not be a candidate.
  The absence needs no separate rule.

The task that publishes v2 owns one more thing: the registry's cover-page entry
must be `stand_in` and the period-end entry `exact`, or a filing carrying both
gives two survivors and `shares_outstanding` is `Unknown` for exactly the filers
that reported it best. That is the `reading` field M4-13 assigns, named here
because v2 is what makes the contest live.

### Where the ruleset lives

**Code, in `vfi-normalize`. No new data surface and no new file per filer.**

The anchor that makes the mapping data says the mapping: "The mapping from
filing tags to canonical concepts is a versioned data registry with per-company
overrides — not a pile of branching logic." Its next sentence keeps this
separate — "Reconciling amendments, restatements, and periods is a separate
ruleset" — and GOALS.md says the same, "separate from the mapping". The reason
the mapping is data is that it is a per-filer judgement that grows one line at a
time; none of that is true here. These three rules name no filer, no tag, no
concept and no threshold, and there is nothing to configure per filer, because a
filer's periods are what its own dates say and a fiscal-year change is a fact
about those dates rather than a setting somebody must remember to write down.

The one data surface it reads is the registry, through the interface
`vfi-normalize` already owns, concept-first, which is the only way that
interface answers.

### What this does not decide

**Whether a period is a year, a quarter or a transition stub.** Nothing here
labels one. A day count would be the window candidate choice already refused,
and the fixture is the argument: this filer's durations run 24, 90, 91, 92, 182,
183, 273, 274, 365, 366 and 457 days, and five of them are a year long — three
the filer dates as a year of its own, plus a calendar tax year and a lease
term. The consequence is real and
is not softened: a screen that wants
"the latest year" cannot ask this table for one yet. Where it gets answered is
the boundary that stores results, or M5 where a metric names what it consumes,
and either may find it needs a filer's fiscal year end — which is a decision and
another version bump, not something a run picks up in passing. This record stops
here rather than inventing a class.

**Which metric consumes which period, and how an absent period makes a metric
absent.** M5's.

**How results are keyed and stored.** The `Value` does not carry its period, so
whatever holds it must, and that is the storage boundary's.

**Anything inside one filing.** Which of a filing's durations answers a flow,
whether an instant answers a balance, and which of several eligible rules wins
are candidate choice's, decided and not reopened.

## Alternatives

- **Take the filing's period from `frame`, or from `fy` and `fp`.** The document
  publishes all three and they look like the answer. `frame` is another party's
  alignment of a fiscal period onto a calendar one, which the fetch record
  refused for precisely the reason it would apply here: taking it imports the
  judgement this ruleset exists to make. `fy` and `fp` are the report's fiscal
  year and period, and the fixture confirms they are the report's — each of the
  ten filings carries exactly one pair across every one of its facts. They are
  still rejected, and not for what they would invite: they are labels. `fy=2025,
  fp=FY` does not name two dates, and the seam requires a period named by dates
  in the shape the contract publishes. A label would have to be turned into
  dates by picking a duration out of the filing, which is the pick the whole
  seam exists to refuse.
- **One filing per period: the latest filing that mentions it, for every concept
  at once.** The obvious shape, and it keeps a period's row internally coherent
  — one filing's view of the period, with assets and liabilities and equity all
  from the same statement. Rejected on the fixture, where the last filing to
  mention the quarter 2024-10-01 to 2024-12-31 carries two facts at it, so the
  quarter would lose revenue, gross profit, tax, cash flow, and every balance,
  and keep net income. Coherence bought at the price of most of the data is not
  a trade this milestone can make, and the same filing's thinness is the
  ordinary case rather than the odd one: a comparative column quotes a few lines
  of a period it is not about.
- **One filing per period: the original, the first filing to report it.** It
  makes provenance simple and it is what a reader expects. Rejected twice over.
  It makes a restatement a no-op, so the milestone's own fixture would be a
  fixture of nothing. And on this filer the first filing to mention the quarter
  2023-10-01 to 2023-12-31 carries two stray facts at it, because the filer's
  first periodic report is a quarter later than the earliest period its facts
  describe — the original is not always there to be preferred.
- **Prefer the annual report over the quarterly: audited beats unaudited.**
  Genuinely attractive, and the distinction is real — interim statements are not
  audited. Rejected because it inverts the restatement in the case that matters:
  a later 10-Q's comparative column is the filer's current statement of the
  period, and preferring the 10-K keeps a figure the filer has since revised.
  The shape is attested in the fixture, where a 10-Q filed 2026-02-20 revises a
  balance at 2025-09-30 that the 10-K filed 2026-01-09 reported — 5,170 against
  637, under an element no concept claims today, so the case is real even though
  this instance costs nothing. It also has no answer for the pairs it does not
  order — a 10-K/A against
  a later 10-K, a 10-KT against a 10-Q — and an ordering over forms with
  unanswerable pairs is a rank nothing can check.
- **Take every period the facts carry, unfiltered.** No registry dependency, no
  concept-first pass, and nothing about a filer's dates is ever second-guessed.
  Rejected on what it manufactures: on this filer, 18 periods no statement
  reports — three lease terms, two calendar tax years, three lease instants, and
  ten cover dates — each a row of absences in the wide table. Two of them, a tax
  year of 366 days and a lease term of 365, sit beside fiscal years of 366 and
  365 days, and are separable from them by nothing this boundary carries.
- **Construct a filer's periods from its fiscal calendar: a year end, four
  quarters, repeated back through the history.** It produces exactly the periods
  a reader wants and no strays at all. Rejected because candidate choice already
  decided what happens to it — a period "constructed from a calendar of its own
  resolves to `Unknown` for every concept" — and because a fiscal-year change
  breaks it in the one way that is hardest to see: the constructed quarters
  after the change are off by the length of the transition, they match no fact,
  and the filer's data goes quietly absent rather than visibly wrong.
- **A period exists only where a filing reports it as its own.** With
  `report_period_end` in hand this is stateable, and it is the tidiest set: no
  strays, and every period the responsibility of one filing. Rejected on the
  fixture, where it drops the quarter 2023-10-01 to 2023-12-31 — 38 facts,
  revenue and net income among them, published by the filer in a comparative
  column — because the filer never filed a 10-Q for that quarter. A period the
  filer stated in full is a period, and a set built on filings rather than on
  facts cannot see it.

## Consequences

**Easier.** Period alignment becomes three rules with no filer, tag, concept or
number in any of them, so a filer with an unusual calendar needs no entry
anywhere and a fiscal-year change needs no handling at all. A restatement takes
effect by re-resolving, and the value that changes names the filing that changed
it, so the difference between the old reading and the new one is inspectable
rather than asserted. Comparatives stop being waste: the fixture's quarter with
no 10-Q of its own resolves from the column that reported it, which no rule
keyed to filings could have reached.

**Harder.** A period's row is assembled per concept, so it can mix filings — a
restated figure from a later comparative sitting beside an unrestated one from
the original, when the later filing carried only the first. Nothing makes the
row balance, and a filer whose restatement moved assets without moving
liabilities will show a period that does not add up. The alternative is a
coherent row missing most of its values, and every value here is at least the
filer's most recent statement of that value, each naming the filing that made
it.

The same seam runs through a composition: `gross_profit` resolved as revenue
less a cost is resolved inside one filing, so it may be built from that filing's
revenue while the `revenue` reported for the period comes from a later one. Both
values name their filings and neither is invented, but they can disagree by a
restatement.

The wide table gains rows for every year-to-date period a filer reports, beside
its quarters and its years, and gains no row at all for a fourth quarter unless
the filer reported one — which most do not. Both are consequences of reporting
what the filer stated and synthesising nothing, and both land on whoever asks
the table a question.

`shares_outstanding` costs a concept until v2 lands, for filers that tag no
period-end count. The two dividend concepts depend on this ruleset more than
they look: a dividend streak is read across consecutive periods, so a period
this ruleset fails to admit reads as a suspension the filer never had. That is
the sharpest reason the period set is defined by what resolves rather than by a
shape.

**Expensive to reverse.** Rule 3's direction. Once values are stored having
taken the latest filing, reversing to prefer the original re-resolves everything
and silently changes numbers that were right under the old reading. The
concept-first period test likewise: it makes the period set a function of the
registry version, so a registry that gains a tag can gain a period, and every
stored row was keyed under one of them. Rule 1's refusal to construct is the
cheapest thing here to keep and the most expensive to undo, because a calendar
introduced later would have to decide what to do with every period already
stored that it does not contain.

## Enforcement

This changes no anchor. It applies four. Anchor 5's ban on the unsourced value,
which is why there is no window, no day count and no fact quorum anywhere in
these rules. "Normalization is data, not code", read with the sentence beside it
that makes this ruleset separate, which is why it is code with no name in it.
One source of truth per value, which is why the winning filing is recorded as
the accession the value already carries and the losers are recomputed.
And the invariant that every resolved fact records where it came from, which is
what Rule 3 has to leave behind and does.

There is no mechanical half. Nothing about these rules is checkable by a gate:
they are a claim about which of a filer's dates are its periods and which of its
filings is current, and no lint reads that. What checks them is the golden
fixtures, and the milestone names three that bear on this ruleset.

**The restatement.** The one that exists already carries it. For the half-year
2023-10-01 to 2024-03-31 and the quarter 2024-01-01 to 2024-03-31, the fixture
must show `earnings_per_share_diluted` resolving to −0.0015 and −0.0004 — the
figures the 10-Q filed 2025-05-14 states — each naming accession
`0001213900-25-042964`, and must show the nine other mapped concepts at the same
half-year resolving to the same digits they had from the 10-Q filed 2024-05-08
and naming *that* accession, because those filings agree and Rule 3 has nothing
to move. A fixture that showed all ten switching to the later filing would be
recording a rule nobody stated: alignment prefers a filing per value, not per
period.

**The fiscal-year change.** No filer in the repository has one yet. The fixture
that records one must show the transition period present at its own length with
its own two dates, the years either side present at theirs, no period
synthesised to fill the gap between them, and the quarters before and after the
change not colliding. What it must show above all is that the expected results
follow from these three rules with no clause added for the change — if the
fixture needs a rule this record does not state, the record is wrong and the
implementing run stops rather than adding one.

**The company that changed tags mid-history.** Its bearing here is narrow and
worth pinning: for a period two filings both report under *different* tags, Rule
3 takes the later filing, so the resolved value records the later tag even when
the number did not change. The fixture must show that — a source tag that moves
while the value stands still — because it is the case where a reader would
expect provenance to be stable and it is not, and because it is the difference
between a tag change and a restatement.

The dividend suspension is not a fixture about alignment, but it fails if
alignment is wrong: a missing period in the middle of a paying history reads as
a suspension. Whoever records it should say which periods it expects, not only
which dividends.

What nothing checks, said plainly.

- **That a period the filer dated is a period the filer meant.** The fixture
  holds equity at an instant a year before the filer was incorporated, and a net
  loss tagged against a full year that is the same figure it reports for a
  24-day stub. This ruleset resolves both, correctly by its own rule, and no
  check can tell a filer's mis-tagged context from an unusual but real one.
- **That the row of a period adds up.** Values are chosen per concept, so
  nothing holds assets, liabilities and equity to one filing. A check of that
  kind would be an analysis, which is a later stage's, and it is worth someone
  proposing there.
- **That silence in a later filing was not a withdrawal.** Rule 3 reads it as no
  claim. If a filer ever removes a line because the figure was withdrawn rather
  than because a comparative column is short, this ruleset keeps the old value
  and nothing on the boundary distinguishes the two.
- **That the registry version a period set was built under is the one a reader
  assumes.** A value carries its registry version, so the mapping is pinned, but
  the *set of periods* is a per-filer artefact of that version and is stored
  nowhere with a version on it.
- **Which revision of these three rules produced a value.** One revision today,
  so the accession is a complete account. The remedy on the second — a method
  version on the `Value`, which is a vocabulary version bump — is named above
  and not taken.
- **That `report_period_end` says what it is claimed to say.** The nine filings
  checked are one filer. The claim that EDGAR's `reportDate` is the end of the
  period a periodic filing reports holds for those nine and is not checked
  anywhere else, and the task that publishes v2 owns recording a fixture that
  holds it.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
