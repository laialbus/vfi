# A concept takes the candidate that reads it exactly over the one that stands in for it, at the period asked for and no other, and a contest that survives both is `Unknown`

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-08-31, by `M4-12`
- **Decided:** 2026-08-31, by the decider
- **Touches:** two fields on a registry entry, and the gate over them — a surface
  `docs/adr/tag-concept-registry.md` fixes and `registry/` does not yet hold.
  Adding a field to a versioned surface is above a worker, which is why this is a
  proposal and not a commit. No contract changes version; the procedure itself is
  inside `vfi-normalize`.

## Context

The registry record answers which rules may reach a concept for a filer, and it
says in its own words that it answers nothing more: "Entries for one concept are
a **set, not a list**: their order in the file means nothing. Where two of them
match facts in one filing, choosing between those facts belongs to the record
named at the end of this one." It then hands seven cases on by name —
`net_income` from the continuing-operations element against the tagged total,
`revenue` from a subtotal against a component sum, `gross_profit` from its own
tag against the difference, `income_tax_expense` from the total against current
plus deferred, `earnings_per_share_diluted` continuing against total,
`dividends_paid` from the total against common plus preferred, and
`capital_expenditure` from one payments element against a sum — and adds that
"naming them is not deciding them." Until they are decided, resolution cannot be
written: the first run to try would settle seven methodology questions inside a
function, one plausible line at a time, which is the mistake this milestone
exists to prevent.

Three surfaces are fixed, and this record reads them rather than reopening them.

**`contracts/fetch-normalize/v1.toml`.** A fact crosses with eight fields —
taxonomy, tag, unit, period, value, accession, form, filed — and a period is an
instant with one date or a duration with two, "never both and never neither."
Fetch filters nothing and parses nothing. One property is load-bearing here: "a
fact is identified by five fields — taxonomy, tag, unit, period and accession
identify a fact, and no two facts crossing share those five with different
values." What does not cross is as load-bearing: no axis, no member, and no
statement of which period the filing itself reports.

**`contracts/canonical-concepts/v1.toml`.** Three states, closed. `Value` carries
"the source tag it was read from, the filing it was reported in, and the rule
that set it." `Unknown` is constructible "from an attempt that ran, and nothing
else" and carries "the candidates considered and the rule that declined each."
`NotApplicable` is constructible only "from a filer kind together with the
applicability clause below that excludes that kind, and nothing else. No filing
is consulted." Applicability is asked first. A concept nothing reaches falls to
its published silence reading, and `docs/adr/canonical-concepts-open-questions.md`
is where those readings were set, by a test this record leans on twice: "silence
reads as zero only where a wrongly-read zero cannot flatter the company on any
metric that consumes the concept; everywhere else it reads `Unknown`."

**`docs/adr/tag-concept-registry.md`.** Four rule forms — `tag`, `sum`,
`difference`, `assert` — kind-scoped, unordered, each identified by an id
rendered from its own fields, under a version that is the digest of the
registry's bytes. An assertion is total and no lookup runs beneath it. Nothing in
the registry reads `form`, `filed` or `accession` to prefer one filing over
another; those cross for the alignment ruleset.

What is readable now that was not when the earlier records deferred this is the
merged company-facts fixture, `fixtures/fetch/every-fact-a-filer-reported`. It
holds one filer's 1,643 facts across 117 elements and ten accessions — two 10-K,
eight 10-Q. Read rather than assumed, it says four things this record uses:

- **No fact carries a dimension.** Every fact object in the document has exactly
  the fields `accn`, `end`, `filed`, `form`, `fp`, `fy`, `val`, plus `start` on a
  duration and `frame` on some. There is no axis, no member and no segment
  anywhere in it, which is what the fetch record already said of the document it
  reads: the filing's "dimensional breakdown" is "not declined — not published by
  this document at all."
- **The five-field key holds.** 1,643 facts render 1,643 distinct
  (taxonomy, tag, unit, period, accession) keys. No collisions.
- **One filing carries many periods for one element.** Durations of 24, 90, 91,
  92, 182, 183, 273, 274, 365 and 366 days appear across this filer's facts; the
  10-K `0001213900-24-101777` carries `NetIncomeLoss` at three durations at once,
  and two of them — the prior year at 365 days and a 24-day inception stub —
  carry the same value, −40,502. The 10-Q filed 2026-08-13 carries
  `StockholdersEquity` at eight instants.
- **Two of the contests this record decides are live in it.** The filer tags
  `GrossProfit`, `Revenues` and `CostOfRevenue`, so both `gross_profit`
  candidates match every one of the 27 times it reports a gross profit — 17
  distinct periods across ten filings — and agree to the unit every time. And it
  tags both share-count elements: `dei`'s cover-page count once per filing, at
  the filing's own cover date and never at a period end, and
  `us-gaap:CommonStockSharesOutstanding` twice per filing, at the two period ends
  the filing presents.

This is Structural rather than Constitutional: it adds two fields to a versioned
data surface and asks a gate to check them, which is the tier's own example. No
anchor is edited, no gate weakened, no protected-path entry changed, no scope in
the milestones moved, and no published contract changes version.

## Decision

### What this rule is asked, and what is settled before it runs

The rule is asked one question: **given a filer and its kind, a concept that kind
admits, one period, and the facts of one filing, what does the concept take?** It
answers with a `Value` or an `Unknown`, and with nothing else. It cannot produce
`NotApplicable`, for the reason the vocabulary gives: that state is constructible
from a kind and an applicability clause with no filing consulted, and a concept
that reaches this rule at all is one the kind admits.

Four things are settled before it runs, each by a record that owns them.
Applicability is asked first, by the vocabulary. An assertion is total, by the
registry: where a filer's file asserts the concept for the period, that is the
`Value` and no lookup runs. The eligible rules are the registry's answer, reached
through its one interface, already reduced by kind scope and by the filer's
`include` and `exclude`. And the period, together with the filing that answers
it, is named by the caller.

### Where the boundary with the alignment ruleset falls

**Alignment names a period and a filing. This rule turns that pair into a value
or an absence.** That is the whole of the seam, and it is drawn there because of
what does not cross the fetch boundary: the filing's own period of report. A
filing publishes many durations and many instants, and nothing in the eight
fields says which of them is the year the filing is about. The 10-K in the
fixture carries the current year, the prior year and a 24-day stub as three
durations of one element, distinguishable only by dates a caller supplies. A rule
that picked "this filing's year" from inside one filing would be inventing the
one field the boundary withholds — the longest duration, the latest-ending one,
the one nearest 365 days — and each of those is a heuristic that reads well and
is wrong for some filer.

So this record decides: which of a filing's durations answers a flow, whether an
instant answers a balance, and which of several eligible rules wins when more
than one matches. The alignment ruleset decides: which canonical periods exist,
which filing answers each, what a later filing restating an earlier one does, a
fiscal-year change, and an amendment. It reads `form`, `filed` and `accession` to
prefer one filing over another; this rule reads `accession` only as the key that
says which facts came from one filing, and reads `form` and `filed` not at all.

One requirement crosses that seam, and it is the caller's to meet: **a period is
named by dates, in the shape the fetch contract publishes** — one date for an
instant, two for a duration. A period whose dates the alignment ruleset reads off
the facts resolves; one it constructs from a calendar of its own resolves to
`Unknown` for every concept. That failure is visible and total rather than
partial and plausible, which is why the rule is stated this way and not with a
tolerance that would absorb the difference.

### Which facts can answer the period asked for

- **A flow concept is answered only by a duration fact, a balance concept only by
  an instant fact.** The vocabulary marks every concept one or the other. Neither
  is ever built from the other: two instants are not averaged into a flow, and a
  duration is not read at its end date as a balance.
- **The dates must be equal, exactly, as the contract publishes them.** A flow's
  duration matches when its start and its end are the period's start and end. A
  balance's instant matches when it is the period's end. There is no day-count
  test and no window. The fixture is the argument: with durations of 24, 90, 91,
  92, 182, 183, 273, 274, 365 and 366 days on one filer, any rule looser than
  equality has a year-to-date figure or a prior-year comparative sitting next to
  the right answer, under the same tag, in the same filing.
- **Every operand of a `sum` or a `difference` is read at the same period.** A
  composition mixing a quarter with a year-to-date is not a candidate; it is a
  number no filing states.
- **A flow the filing does not carry as one duration is not built by subtracting
  one duration from another.** A fourth quarter reconstructed from a year less
  nine months is a new rule form, and the registry's set of four is closed.
- **One element answers a period other than its own, and the vocabulary says
  which.** `shares_outstanding` means "a count at a stated date at or near the
  period end", and the cover-page element that carries it is stamped with the
  filing's cover date — in the fixture, once per filing and never at a period
  end. An entry may therefore declare that it answers the period of the filing it
  was reported in rather than its own; where it does, its instant fact in that
  filing answers the period asked for, and it is one fact because the five-field
  key makes it one. No window is needed and none is written.

### Consolidated over segment: what was read

**No segment-dimensioned fact reaches normalize.** The contract publishes no axis
and no member, and the document behind it publishes none either — checked against
the merged fixture, where not one of 1,643 facts carries an axis or a member.
Every fact that crosses is the undimensioned value, which is the consolidated
entity's. This is the same reading the open-questions record took when it refused
to split a special dividend out of `dividends_declared_per_share` — "the fetch →
normalize contract carries no axis and no member; a dimensional breakdown is not
published by the document it reads at all" — and the fixture is what turns that
reading into something checked. So the consolidated half of this rule is that
**there is no choice to make: normalize never ranks a segment against a total,
because a segment never arrives.** "Consolidated over segment" is satisfied by the
retrieval, and a rule written as though it were satisfied by resolution would be
a rule nobody could check.

If one ever does cross, it cannot cross quietly. A segment fact shares its
taxonomy, tag, unit, period and accession with the consolidated fact, so the two
collide on the five-field key, which the contract's property forbids and the
fixture that holds that property would go red on. **The response is not to prefer
the undimensioned value: with no axis crossing, normalize cannot tell which value
is the undimensioned one.** It would be picking the plausible-looking of two
numbers with nothing to read the difference off — the failure this milestone
exists to prevent. So the response is the one the fetch record already routes:
the boundary is broken, the run stops and escalates, and the contract that must
then carry the axis and the member is a version bump and a decision, not
something a run settles mid-flight. Reached at run time before anyone has stopped
it, a concept whose candidates collide on that key is `Unknown`, carrying the
collision.

### The rule

Over the facts of one filing, for one concept and one period:

1. **Candidates.** An eligible rule is a candidate when every fact it needs
   answers the period asked for, by the tests above: one fact for a `tag`, every
   operand for a `sum`, both terms for a `difference`. A composition missing an
   operand is not a candidate — reading a missing operand as zero is reading
   silence as zero for that component, which is the reading the vocabulary's
   silence test already refused for the concepts it matters to. A `difference`
   whose concept operand did not itself resolve to a `Value` is not a candidate.
2. **Exact before stand-in.** If any candidate reads the concept exactly, the
   candidates that only stand in for it are dropped.
3. **The whole before its part.** Among what survives, a candidate whose facts
   are strictly contained in another survivor's facts is dropped. A component
   never competes with a composition that contains it.
4. **Settle.** Exactly one survivor: the concept is that `Value`. More than one:
   the concept is `Unknown`, carrying every candidate and the reason each was
   dropped or left undecided. None: the concept falls to the silence reading the
   vocabulary publishes for it, which the registry record already fixed —
   `Unknown` for twenty-five concepts, a `Value` of zero for
   `short_term_investments`, and the conditional pair for the two dividend
   concepts.

A fact answers a concept only in the unit the vocabulary names for it: a count in
shares, an amount per share, a currency amount in a currency. The boundary
publishes the unit key and does not publish the filing's reporting currency, so
where two facts under one entry answer one period under two currency keys,
nothing on the surface says which currency the filing reports in and the concept
is `Unknown` rather than one of them. The fixture holds an element with facts in
both `CNY` and `USD`; no concept claims that element today, which is why this is
written down before it is met rather than after.

Two survivors that agree are still `Unknown`. A `Value` records "the rule that
set it", singular, and a rule chosen among equals is a choice this rule cannot
record; the recovery is the per-filer `exclude`, which puts the choice in a file
with an id behind it rather than in a tie-break nobody wrote down.

### Where the rule lives, and the two fields it asks the registry for

The ordering is **data**; the procedure is **code**. Concretely: the procedure in
`vfi-normalize` is the four steps above and contains no concept name and no tag
name, so it is one rule and not the branching pile the anchor bans. What tells it
which candidate reads the concept exactly is a field on the registry entry — and
that field does not exist yet, so this record proposes it and does not write it.

- **`reading`**, one of `exact` or `stand_in`, on every entry. The test that
  assigns it is one question, answerable about one entry with the vocabulary's
  meaning clause in hand and without reading any other entry: *is there a filer
  in this entry's kind scope for which this element, or this composition, differs
  from the concept as the vocabulary defines it?* No — `exact`. Yes — `stand_in`,
  and the entry is the concept only for filers that lack what makes it differ: a
  discontinued operation to separate, a subtotal to present, a component to tag.
- **`answers`**, one of `the period asked for` — the default, and what every entry
  but one takes — or `the filing it was reported in`. It exists for the
  cover-page share count above, and the gate can hold it to the shape it was made
  for: it is admissible only on an entry for a concept the vocabulary measures as
  a balance.

The field spellings are the registry task's to fix; what this record decides is
the two closed value sets and the test that assigns the first.

**Why this is not the ranking the registry record refused.** That record rejected
"order the entries for a concept, most preferred first" because it "is candidate
choice under another name, decided inside a record that says it decides none of
it, in the file a later reader would not think to check." Both halves of the
objection are answered rather than argued around. The decision is here, in the
record named for it, and the registry would carry its consequence the way data
carries a decision made elsewhere. And a rank is *relational* — it is a claim
about an entry's position among the others, so two runs adding entries in
parallel can write ranks that contradict, and no gate can see it. `reading` is
*intrinsic*: it is a claim about one entry against the published meaning of one
concept, it is answerable with the rest of the file unread, and two runs adding
entries in parallel cannot make it inconsistent. That difference is the whole
argument, and it is why this is one closed-set field rather than an order.

Because `registry/` does not exist yet — M4-09 creates it — nothing is migrated
and no entry is revisited. The fields land with the first byte of registry data.

### The seven cases, each against the one rule

Each is the same four steps. Nothing below is a case-specific answer; it is the
rule applied and checked.

- **`net_income`** — `IncomeLossFromContinuingOperations` is `exact`: the
  vocabulary defines the concept as income from continuing operations
  attributable to the parent, which is what the element is. `NetIncomeLoss` is
  `stand_in`: for a filer with discontinued operations it includes what the
  meaning excludes, and the registry lists it on exactly that condition — "that
  same figure for a filer presenting no discontinued operations to separate."
  Both tagged: step 2 takes the continuing figure. Only `NetIncomeLoss` tagged:
  it is the sole candidate and resolves, which is the reading the registry
  already stated. The exposure that remains, named rather than fixed: a filer
  that *has* discontinued operations and tags only `NetIncomeLoss` resolves to a
  figure wider than the concept, and nothing on the boundary distinguishes it
  from a filer that has none. Reaching for a discontinued-operations element to
  detect it would be adding to the mapping, which this record may not do; the
  recovery is an override, and the fixtures the milestone names are where it
  surfaces.
- **`revenue`** — the presented subtotal is `exact` and the component sum is
  `stand_in`, which is the registry's own reading of the form: a `sum` is "for
  the filer that presents the components and no subtotal." For a `bank`,
  `RevenuesNetOfInterestExpense` beats the sum of `InterestIncomeExpenseNet` with
  `NoninterestIncome` when both match, and the sum resolves when the subtotal is
  absent. The same shape settles the `operating` entries: `Revenues` and
  `SalesRevenueNet` are totals and `exact`; the contract-revenue and the
  goods-and-services elements are `stand_in`, because a filer with revenue
  outside them differs from the concept by that much.
- **`gross_profit`** — `GrossProfit` is `exact`; the difference of `revenue` less
  a cost element is `stand_in`, on the vocabulary's own words: "Where a filer
  presents a cost of revenue rather than a gross profit, this is revenue less
  that cost." *Rather than* is the condition that makes it a stand-in. Both
  matched all 27 times the fixture reports one, and both give the same number
  every time, so the choice is exercised there and costs nothing; the
  filers where it bites are the ones whose cost line and gross profit do not
  reconcile. A filer tagging two different cost elements produces two stand-in
  differences that neither subsume nor agree, and the concept is `Unknown` — the
  right outcome for the right reason, since either cost alone is part of the
  cost.
- **`income_tax_expense`** — `IncomeTaxExpenseBenefit` is `exact`, the total
  charge the meaning asks for. The sum of `CurrentIncomeTaxExpenseBenefit` with
  `DeferredIncomeTaxExpenseBenefit` is `stand_in`. Both matched: the total. Only
  the two components: the sum.
- **`earnings_per_share_diluted`** —
  `IncomeLossFromContinuingOperationsPerDilutedShare` is `exact`,
  `EarningsPerShareDiluted` is `stand_in`. This case is the one the vocabulary
  already decided, and the rule reproduces its answer: "Where a filer has
  discontinued operations and presents per-share amounts for continuing
  operations and for the total, the first is meant; where it has none, the single
  figure it presents is that figure." Steps 1 and 2 are that sentence,
  generalised.
- **`dividends_paid`** — `PaymentsOfDividends` is `exact`, covering all classes as
  the meaning requires. The sum of the common and the preferred elements is
  `stand_in`. Both matched: the total. Only the pair: the sum. Only the common
  element: the sum is not a candidate and no `tag` entry names it, so nothing
  matches and the concept falls to its conditional silence reading, where
  `dividends_declared_per_share` decides between a zero and an `Unknown`.
- **`capital_expenditure`** — this is the case that runs the other way, and the
  reason the rule ranks candidates by what an entry reads rather than by whether
  a filer tagged it or arithmetic composed it. The concept covers property,
  plant and equipment *and* capitalised software, so each single payments element
  is `stand_in` — it omits what the filer capitalised elsewhere — and the sum of
  the property element with the software one is `exact`. A filer tagging both:
  step 2 keeps the sum. A filer tagging only the property element: it is the sole
  candidate and resolves. Step 3 is what would keep the property element out of
  the contest even if it and the sum were ever read alike. A filer tagging two
  single elements
  that no sum entry joins — property and capital improvements, say — is `Unknown`
  until either the registry gains that sum or the filer gains an override; both
  are data, neither is a record.

### What a resolved value records about the choice

A `Value` carries what the vocabulary fixed — the source tag, the filing, and the
rule that set it — which the registry record reads as the pair **(registry
version, rule id)**, "and never the id alone." Nothing more is needed, and this
record adds no field to the contract, because the rule id renders the winning
entry's concept, kind scope, form and operands, and the choice itself is a pure
function of things already recorded: the registry version fixes the eligible set
and every entry's `reading`, the filing fixes the facts, the period fixes which
of them answer, and the four steps are deterministic over those. Replaying a
value means re-reading the named filing under the named version and running the
same steps; every candidate that lost is recomputed, not remembered.

Where the outcome is `Unknown`, the vocabulary already requires the candidates
and "the rule that declined each", and this record supplies the closed set of
reasons a rule can be declined for:

- no fact answering the period asked for;
- an operand with no fact answering it, so the composition was partial;
- an operand concept that did not resolve to a `Value`;
- a stand-in dropped behind an exact reading;
- a candidate contained in a longer composition;
- undecided among survivors, which names the others.

### When the rule does not settle

**`Unknown`, and never a pick.** Not `NotApplicable`: that state is a positive
claim about the filer's accounting shape, constructible only from a kind and an
applicability clause with no filing consulted, and a contest between candidates
is an attempt that ran — which is exactly and only what `Unknown` is built from.
Not the larger, not the more common, not the one that agrees with last year:
every one of those is a plausible pick, and a plausible pick is the failure this
milestone exists to prevent.

Two things are a stop rather than an `Unknown`, and neither is this rule's to
settle: facts colliding on the five-field key, which breaks the contract's own
property and is the escalation the fetch record routes; and a registry the gate
would have refused — a rule both included and excluded, or two assertions whose
periods overlap — which the registry record already says fails to load rather
than resolving to anything.

The recovery from an `Unknown` is the one the constitution already provides and
this record does not extend: a per-filer `exclude` that removes the losing entry
for that filer, or an `assert` that states the value with its period and its
cited source. Both leave a rule id on the result, which is the difference between
a choice someone made and signed and a choice a function made quietly.

### What this does not decide

Amendments, restatements and period alignment, by name: which canonical periods
exist, which filing answers each, and what a restatement does to a period already
resolved. The mapping's content: no element is added, removed or rescoped here,
and the seven cases are decided over the entries the registry already lists.
Which metric consumes a concept, and how an absent input makes a metric absent,
which are M5's. Whether a filer's kind may be derived rather than asserted. And
the registry's own coverage — that the `short_term_debt` component sum fires only
for a filer that tagged every one of its operands is an observation this record
leaves in the open, for a registry version to act on if the fixtures ask.

## Alternatives

- **Order the entries for a concept in the registry, most preferred first.** The
  obvious shape, and the registry record rejected it for where it would have been
  decided. Rejected again here for what it is: a rank is a claim about an entry's
  place among the others, so two runs adding entries in parallel can write ranks
  that disagree and no gate can see the disagreement, and every new element makes
  a reader re-read the whole list to place one line. `reading` is answerable about
  one entry alone, which is the property that makes it safe for the write pattern
  the registry actually has.
- **Keep the preference in `vfi-normalize`, as a table of concepts and their
  favoured tags.** No registry change at all, and the seven cases are seven lines.
  Rejected: it is the branching pile "Normalization is data, not code" bans,
  written as a table instead of an `if`; the eighth case arrives with the first
  new element and is decided in a source file by whoever is passing; and it puts
  a second source of truth beside the registry for the same entries.
- **Prefer the figure the filer tagged over any figure arithmetic composed, in
  every case.** It reads well, it needs no new field — a form is already on every
  entry — and it settles six of the seven. Rejected on the seventh:
  `capital_expenditure` includes capitalised software, so for a filer that tags
  both payments elements the sum *is* the concept and the single element is short
  by the software. A rule that is right six times out of seven and silently low
  the seventh is precisely the shape of mistake this record exists to stop.
- **Take the value where candidates agree and `Unknown` only where they differ.**
  It keeps more data and never picks. Rejected on both halves: the cases that
  matter disagree by construction — a continuing-operations figure and a bottom
  line differ exactly for the filer the choice is about — so it decides nothing
  where deciding is needed; and where two candidates do agree it still has to
  record one rule as the one that set the value, which is the arbitrary choice
  step 4 refuses.
- **Match a period by day count — a duration within a few days of 365 answers a
  year.** It would let the caller name a year rather than two dates. Rejected: the
  window is a number with no source. It has to be wide enough for the 365- and
  366-day years the fixture holds and for the 52- and 53-week years a filer on
  that calendar reports, and every widening walks it toward the 274-day
  year-to-date figure sitting under the same tag in the same filing. Which side
  of the window a filer lands on is a property of its calendar, not of the rule.
- **Answer the seven cases here, one at a time, and leave the general question
  open.** Smallest possible record, and it unblocks resolution today. Rejected:
  seven answers are not a rule, so the eighth case — the first element a later
  registry version adds — is decided by whoever hits it, which is the situation
  this record was written to end.

## Consequences

**Easier.** Resolution is one procedure over data: four steps, no concept named
in it, and a new element is a line in a file with one question answered about it.
The two cases the vocabulary itself decided — `net_income` against the bottom
line, diluted EPS continuing against total — come out of the general rule rather
than being restated in it, which is the check that the rule is the vocabulary's
own logic and not a new one. And a value stays replayable with no new contract
field, because the choice is a function of the version, the filing and the period
that the value already names.

**Harder.** The rule is strict on purpose, and the strictness is paid in
absences. Two exact candidates that both match make the concept `Unknown` even
when they agree, so a filer that tags `Revenues` and `SalesRevenueNet` in one
filing loses revenue until an override excludes one. A partial composition is not
a candidate, so the `short_term_debt` component sum resolves only for a filer
that tagged all of its operands, and a filer tagging two of them resolves to
`Unknown` rather than to their sum. Two stand-ins that neither subsume nor agree
are `Unknown` — property plus capital improvements is the live example. Each of
these costs a metric that M5 shows as absent with a reason, and each is
recoverable by a file in an unprotected path; the alternative in every case is a
number that looks right.

The cover-page reading has a visible cost of its own: where a filer tags no
period-end share count, `shares_outstanding` is a count at the filing's cover
date, which for a 10-K in the fixture is up to three months after the year end
and moves with any issuance in between. The vocabulary priced that in when it
wrote "at or near the period end"; what this record adds is that the date is the
filing's, not a window, so what was read is legible.

The seam with alignment is now load-bearing in one direction: a period this rule
is asked about must be named by dates that exist in the facts. If the alignment
ruleset turns out to need the filing's own period of report to name them, that is
a `fetch-normalize` v2 carrying one more field — a decision and a version bump,
which this record does not take.

**Expensive to reverse.** `reading` as a two-value field, once the registry
carries it on every entry and stored values were resolved under it: widening it
to three classes or to a rank is a new digest for every file and a re-resolution
of everything stored. The exact-date period test, for the same reason —
loosening it later changes which fact answered a period, silently, for every
value already resolved. Nothing else here is more than a line: the four steps are
internal to `vfi-normalize`, and the decline reasons ride on a state that already
carries them.

## Enforcement

This changes no anchor. It applies four: "Normalization is data, not code", which
is why the ordering is a field and the procedure has no concept in it; anchor 5's
ban on the unsourced value, applied to the period test, which is why there is no
window and no day count; one source of truth per value, which is why the choice
is recorded as its winner and recomputed rather than copied; and the vocabulary's
own rule about what each state is constructible from, which is why an unsettled
contest is `Unknown` and never the other absence.

The mechanical half is two checks on the `registry` gate, added by the task that
writes the fields:

- every entry carries `reading`, from its closed set, and `answers` where it is
  not the default, from its closed set;
- `answers = the filing it was reported in` appears only on an entry for a
  concept the published vocabulary measures as a balance, read out of the
  published surface rather than restated.

Each gets the proof-of-catch M2 requires: a registry that violates it on purpose,
and the gate failing on it.

The rule itself is checked by the golden fixtures, and two of its cases are
already live in the one that exists: `gross_profit`, with both candidates
matching 27 times over, and `shares_outstanding`, with a cover-date count and a
period-end count in every filing. The fixtures the milestone names — the company
that changed tags mid-history, the filer whose statements do not fit the ordinary
shape — are where the choices that cost something surface.

What nothing checks, said plainly.

- **That an entry's `reading` is the right one.** It is a judgement about an
  element against a meaning clause written in prose, and no gate reads prose. A
  stand-in wrongly marked `exact` silently outranks the entry that was right.
  Only a fixture whose expected value disagrees will catch it.
- **That the period a caller names is the period it meant.** This rule tests
  equality against dates it is given; it cannot tell a year from a year-to-date if
  the caller hands it the wrong pair. What limits the damage is that a wrong pair
  usually matches nothing at all.
- **That the procedure stays free of concept names.** Stated here as a property
  and enforced by nothing today. It is small enough to read, and if it stops being
  so, a check that the resolution module names no published concept is a gate
  someone can add.
- **That no dimensioned fact ever arrives.** The five-field property is what would
  catch it, and the fetch record already records that the property has no
  mechanical check beyond the fixture that holds it.

## Decision review

- **Authority:** Structural, and within reach: two closed-set fields on the
  registry entry surface and two strengthening checks on its gate are schema
  additions and new gates, the tier's own examples. No anchor is edited, no gate
  weakened, no protected-path entry or GOALS.md scope moved. Flagged for later
  human review, as the tier requires.
- **Checked:** the four anchors it names — normalization-is-data, anchor 5's
  unsourced-value ban, one-source-of-truth, and the vocabulary's construction
  rules — applied, none edited. Against the accepted records: the registry's
  "set, not a list" hand-off and its seven named cases match verbatim; the
  vocabulary's carriage for `Value` and `Unknown` and the silence test are
  quoted accurately; the fetch contract's five-field identity is as published;
  the merged fixture it reads exists on main. The six alternatives are argued
  honestly — the tagged-over-composed rejection in particular falls to the
  `capital_expenditure` counterexample, not to taste.
- **Verdict and why:** Accepted. The record turns seven handed-on cases into one
  four-step procedure with no concept name in it, and the intrinsic-versus-
  relational argument for `reading` is the answer the registry record's refusal
  of an ordering asked for: a field answerable about one entry alone survives
  the parallel write pattern that a rank cannot. Every strictness is priced in a
  recoverable absence rather than a plausible number — two agreeing survivors
  are still `Unknown` because a `Value` records one rule — which is the
  milestone's own rule. And the seam with alignment is drawn where the boundary
  forces it: nothing inside one filing can name the filing's own period of
  report, so the caller must.
- **What would have changed it:** a tie-break among agreeing survivors, a
  day-count window on the period test, or any read of `form` or `filed` — each
  is an unsourced pick or a smuggled piece of the alignment ruleset. Or a
  consolidated-over-segment rule written as a resolution-time preference, which
  the fixture shows would decide over facts that never arrive.
