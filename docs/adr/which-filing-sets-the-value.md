# Among the filings that answer a period, the one filed last sets the value, whether or not the earlier ones agree

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-09-11, by `M4-23`
- **Decided:** —
- **Touches:** Rule 3 of the accepted `docs/adr/period-alignment.md`, and nothing
  else of it. An accepted record is never edited, so a rule that contradicts
  itself is repaired by a record that states it whole; superseding one is above a
  worker, which is why this is a proposal and not a commit. No contract changes
  version, no schema moves, no anchor is edited, no gate is weakened, and nothing
  here is implemented.

## Context

`docs/adr/period-alignment.md` is accepted, and its Rule 3 says two different
things in the same record.

Its Decision: "among the answering filings whose attempt produced a `Value`: the
value is the one from the filing with the greatest `filed`, and the `Value` names
that filing." Every value, agreeing ones included. Its paragraph on the company
that changed tags reads it the same way: "Rule 3 takes the later filing, so the
resolved value records the later tag even when the number did not change."

Its Enforcement, writing the restatement fixture that same rule must pass, reads
it the other way: the concepts that agree across the two filings must resolve
"naming *that* accession" — the earlier one — "because those filings agree and
Rule 3 has nothing to move", and "a fixture that showed all ten switching to the
later filing would be recording a rule nobody stated."

Call the first reading **latest**, the second **latest-that-changed**. They never
differ about a number: both take the figure the last filing to state it states,
so where the filings agree the digits are the same either way. What they decide
is which filing, which source tag and which rule every agreeing value in the
corpus records — which is the whole of a resolved value's provenance, and, for a
filer that restated nothing but its tagging, the whole of what changed.

Nothing has been resolved under either. `crates/normalize/src/filings.rs`, which
M4-18 landed, runs the attempt inside every answering filing and then stops: it
orders no two filings, parses no `filed`, and reads no form, saying in its own
header that the record states the rule twice in ways that do not agree and that
this is the decider's to settle. So this is not a second revision changing values
a first revision stored. It is the first statement of Rule 3 that can be
implemented at all.

Four surfaces are fixed here and read rather than reopened.

**`docs/adr/candidate-choice.md`.** Everything inside one filing: which of a
filing's durations answers a flow, whether an instant answers a balance, which of
several eligible rules wins, and what an attempt that settles nothing produces.
This record asks that procedure for an answer per filing and chooses among the
answers; it never reaches inside one.

**`contracts/canonical-concepts/v1.toml`.** A `Value` carries "the source tag it
was read from, the filing it was reported in, and the rule that set it".
`Unknown` is built "from an attempt that ran, and nothing else". A concept
nothing reaches falls to the silence reading the vocabulary publishes for it,
which for `short_term_investments` is a zero and for the two dividend concepts is
a zero on a stated condition.

**`contracts/fetch-normalize/v1.toml`.** Eight fields per fact. `form`, `filed`
and `accession` cross for this ruleset and have no other consumer. Every value
crosses as the characters the document publishes, unparsed. Normalize reads those
characters into an exact decimal where a `sum` or a `difference` asks it to
compose one, and nowhere else: no rule in the stage compares two values to prefer
one of them, and candidate choice says so in as many words when it leaves two
agreeing survivors `Unknown`.

**Rules 1 and 2 of the accepted record**, which this one does not touch, together
with what it decided about a fiscal-year change, what a resolved value records,
the field the boundary withholds, and the `v2` it proposes.

What is readable is the two merged fetch fixtures and `registry/`, whose 74
elements are the same set since #137 landed them on 2026-09-02 — two days before
the accepted record was proposed, and #155 added only `reading` and `answers`.
Every count below was re-derived against the committed registry by running the
landed procedure, not taken from the accepted record.

This is Structural, and within reach: it supersedes one rule of a Structural
record with a record of the same kind, decides nothing an anchor fixes, adds no
field to any contract, moves no protected path and no milestone scope, and leaves
every other rule of the superseded record standing. It is flagged for later human
review, as the tier requires.

## Decision

### What this supersedes, and what stands

**This record supersedes Rule 3 of `docs/adr/period-alignment.md` and nothing
else of it.** Rule 1 — a canonical period is a pair of dates the filer's own
facts carry, and is the filer's where at least one concept its kind admits
resolves at it — stands as accepted. Rule 2 — every filing carrying a fact stated
for the period answers it, and each attempt is read inside one filing — stands as
accepted. So do that record's decision that a fiscal-year change needs no clause,
its account of what a resolved value records and what it deliberately does not,
its condition for admitting the cover-page entry with its refusal of that entry
at `fetch-normalize` v1, and the `v2` it proposes. None of them is reopened,
qualified or read differently here. What a resolved value records is added to in
one place and changed in none: that section's sentence is written for a value a
rule read out of a filing, and below it is said what it comes to for one the
vocabulary's silence reading supplied, which it did not reach.

Because an accepted record is never edited, the rule below is stated whole rather
than as a patch: a reader who needs Rule 3 finds all of it here, and the sentence
in the accepted record that disagrees with it is superseded in both of its
spellings — the Decision's and the Enforcement section's.

### Rule 3, whole, as it will stand

For one period and one concept, among the answering filings whose attempt
produced a `Value`:

**The value that stands is the one produced by the attempt in the filing with the
greatest `filed`. The `Value` is that attempt's, and it carries what that attempt
gave it. Whether an earlier filing's attempt produced the same figure is not read,
and nothing else about the earlier filings is read either.**

This is the accepted Decision's choice, made once and unqualified. What changes is
the Enforcement section's exception for agreeing values, which is withdrawn, and
the reason is that it is a preference this ruleset may not have: to know whether
two filings agree, the rule must compare their values, and a rule that reads the
number to decide which filing a value names prefers by the number. The accepted
record's own sentence forbids that in the same paragraph — "Never the larger,
never the more common, never the one that agrees with last year" — and anchor 5
forbids it generally. The choice as stated reads `filed`, and on a tie `form`, and
reads nothing else, so no value anywhere in the proof is preferred for what it
says.

One consequence of "that attempt's, as the attempt gave it" is worth stating
rather than leaving to be inferred, because the accepted record's shorter phrase —
"the `Value` names that filing" — is not true of every value. An attempt can
produce a `Value` the vocabulary's silence reading supplied, which names no tag,
no filing and no rule, there being none to name. Such a value is a `Value` and so
is in the contest, and where it wins on `filed` it stands as it was built. The
rule chooses an attempt; it does not stamp a filing onto whatever the attempt
returned.

Three clauses complete it. The first two are the accepted record's, restated here
unchanged and cited to it. The third is the accepted record's too, with one
sentence of its reasoning made explicit.

**Silence in a later filing is not a restatement.** As the accepted record has it:
an attempt that produced an `Unknown` — no fact answering the period, a partial
composition, a contest that did not settle — does not displace a `Value` from an
earlier filing, because "a later filing that does not mention a line makes no
claim about it, and reading its silence as a withdrawal would turn the ordinary
thinness of a comparative column into the erasure of a figure the filer never took
back." Unchanged, and it is what keeps the choice above from costing anything: the
last filing to mention a period is usually the thinnest, and what it does not
mention it does not take away.

**An amendment is a filing.** As the accepted record has it: its facts carry their
own accession, their own `form` ending in `/A`, and their own `filed`, and it wins
for what it states because it states it later. No rule prefers it and none
discounts it; a Part III-only 10-K/A carries no facts and changes nothing.
Unchanged.

**`form` is read once, to break a tie on `filed`.** Where two answering filings
share the greatest `filed`, a filing whose form is another's with `/A` appended
supersedes it — that is what an amendment is, and `filed` is a date with no time
on this boundary, so a same-day amendment is not otherwise orderable. Any tie that
survives is `Unknown`, carrying both accessions.

This is the accepted record's clause with one condition removed, and the reason is
the choice above. That record reached the tie-break only "where two answering
filings share a `filed` date **and their attempts disagree**", because under its
Enforcement reading two agreeing attempts had nothing to settle. Under this rule
they do: the choice fixes the tag, the filing and the rule a value records, so two
same-day filings that state the same figure still leave it undecided which of them
the value was read out of. Keeping the condition would make the rule ask whether
two values agree, which is the preference it may not have. So the condition goes,
and a tie is a tie whatever the figures say. The cost is an `Unknown` where a
reader would have accepted either filing, and it is the answer candidate choice
already gives one step in: two survivors that agree are still `Unknown`, because a
value records the rule that set it, singular. The recovery is the same one — a
per-filer `exclude` or `assert`, a file with a rule id behind it.

Nothing else enters. No window, no day count, no quorum of filings or of facts;
never the larger figure, never the more common one, never the one that agrees with
a neighbouring period; no preference for an annual report over an interim one, for
an original over an amendment, or for a filing that reports the period as its own
over one quoting it in a comparative column. `filed` is compared as a date, which
is where the fetch record already puts that parse.

### What a `Value` records under this reading

**Inside what the published vocabulary already requires, and not a field more.**
A value a registry rule read out of a filing's facts carries the source tag it was
read from, that filing, and the rule that set it — the pair of registry version
and rule id. An asserted value carries its rule and its cited filing. A value the
vocabulary's silence reading supplied carries none of the three, and that is not a
gap this rule opens: it is what a value nothing was read for has always carried,
and the settling procedure already builds it that way.

So no `canonical-concepts` version bump is asked for here, and none is worked
around. The accepted record named the one thing that would force one — a method
version on the `Value`, needed "the moment there is a second revision that changes
a number". This is not that moment: no value has been resolved under any reading
of Rule 3, and this record changes no number that any reading produces. That
condition stands as the accepted record wrote it, for whichever later revision
meets it.

The accepted record's replay claim holds, and holds more simply than it would
under the reading not taken. Given the filer's facts and the registry version a
value carries, the answering filings re-derive by Rule 2, every attempt re-runs by
candidate choice, and the comparison is a maximum over `filed` with the tie stated
above. Nothing in the choice depends on reading a number, so replay needs
no equality test over decimal literals the boundary publishes unparsed. The losing
attempts are recomputed rather than remembered, which is the answer both accepted
records already gave.

One thing the value's accession now means, stated so a reader does not assume the
other: it is the filing this value was read out of, which is the most recent filing
that stated the figure — not the filing that first stated it. The first is
recoverable by re-resolving every answering filing, which is what replay does
anyway.

### What each fixture tied to this rule must show

The three the accepted record names. Every claim below was re-derived by running
the landed procedure — `settling` over `filings::answering` — against the
committed registry, over the merged fetch fixture that holds the case.

**The restatement.** `fixtures/fetch/every-fact-a-filer-reported`, CIK 0002003750.
The accepted record describes this case as "ten mapped elements" of which "nine
agree to the digit". That is an undercount, and not a stale one: the registry's 74
elements are the set it was written against. Re-derived, the half-year 2023-10-01
to 2024-03-31 is answered by two filings — the 10-Q `0001213900-24-040632` filed
2024-05-08 and the 10-Q `0001213900-25-042964` filed 2025-05-14 — each carrying 38
facts at it and **thirteen** elements the registry names, of which **twelve agree
to the digit** and `EarningsPerShareDiluted` differs. The thirteenth element,
`CostOfRevenue`, resolves no concept of its own: it is an operand of
`gross_profit`'s difference, which loses to the tagged `GrossProfit` at step 2.

**Fifteen concepts resolve to a `Value` in both answering filings.** Under this
rule each takes the figure below and names the accession below.

| concept | value | names |
| --- | --- | --- |
| `revenue` | 264,956 | `0001213900-25-042964` |
| `gross_profit` | 103,206 | `0001213900-25-042964` |
| `operating_income` | −76,928 | `0001213900-25-042964` |
| `pretax_income` | −77,035 | `0001213900-25-042964` |
| `income_tax_expense` | 367 | `0001213900-25-042964` |
| `net_income` | −77,402 | `0001213900-25-042964` |
| `interest_expense` | 536 | `0001213900-25-042964` |
| `depreciation_and_amortization` | 401 | `0001213900-25-042964` |
| `operating_cash_flow` | −127,584 | `0001213900-25-042964` |
| `capital_expenditure` | 3,729 | `0001213900-25-042964` |
| `diluted_shares_weighted_average` | 50,301,639 | `0001213900-25-042964` |
| `earnings_per_share_diluted` | −0.0015, from −0.002 | `0001213900-25-042964` |
| `short_term_investments` | 0 | no filing: the vocabulary's zero |
| `dividends_declared_per_share` | 0 | no filing: the conditional zero |
| `dividends_paid` | 0 | no filing: the conditional zero |

Eleven of the twelve read out of a filing agree across the two filings and move
their provenance anyway; the twelfth is the correction the fixture exists for. The
last three are the silence readings, identical in both filings and naming nothing
in either — the case the accepted record's "the `Value` names that filing" does
not describe, and the case the reading not taken cannot state at all.

The quarter 2024-01-01 to 2024-03-31 is answered by **four** filings, not two:
`0001213900-24-040632` (15 facts at the period), `0001213900-24-067900` filed
2024-08-13 (2), `0001213900-25-042964` (16), and `0001213900-25-078735` filed
2025-08-20 (2). The two full ones share nine registry-named elements, eight
agreeing, `CostOfRevenue` again the operand-only one. **Eleven concepts resolve in
more than one answering filing.**

| concept | value | names |
| --- | --- | --- |
| `revenue` | 140,986 | `0001213900-25-042964` |
| `gross_profit` | 55,075 | `0001213900-25-042964` |
| `operating_income` | −24,626 | `0001213900-25-042964` |
| `pretax_income` | −24,626 | `0001213900-25-042964` |
| `income_tax_expense` | 282 | `0001213900-25-042964` |
| `diluted_shares_weighted_average` | 60,000,000 | `0001213900-25-042964` |
| `earnings_per_share_diluted` | −0.0004, from 0 | `0001213900-25-042964` |
| `net_income` | −24,908 | `0001213900-25-078735` |
| `short_term_investments` | 0 | no filing: the vocabulary's zero |
| `dividends_declared_per_share` | 0 | no filing: the conditional zero |
| `dividends_paid` | 0 | no filing: the conditional zero |

`net_income` is the one worth reading twice. Four filings state −24,908 for the
quarter and the greatest `filed` among them is the 10-Q filed 2025-08-20, a filing
the accepted record does not name, quoting one comparative line of a quarter it is
not about. Under this rule that filing sets the value, and the quarter's row names
two different filings and, for three concepts, none — while the filing that
reported the quarter in full, and the one that first quoted its net income, win
nothing. That is what "alignment prefers a filing per value, not per period" means
when it is followed all the way.

The two figures the accepted record's fixture paragraph pins survive it unchanged:
`earnings_per_share_diluted` resolves to −0.0015 at the half-year and −0.0004 at
the quarter, each naming `0001213900-25-042964`. It is only the sentence about the
agreeing concepts naming the earlier accession that this record supersedes.

**The company that changed tags mid-history.** No fixture is recorded under that
name yet; `fixtures/fetch/a-filer-that-changed-its-fiscal-year`, CIK 0001859199,
already holds the case, and the claim is re-derived from it. The source tag moves
while the value stands still, and what moves it is the filer's move from
`EarningsPerShareDiluted`, which the registry reads as a stand-in, to
`IncomeLossFromContinuingOperationsPerDilutedShare`, which it reads as exact. At
six periods the tag moves and the figure does not: 2022-05-01 to 2023-04-30
(−0.13), 2024-01-01 to 2024-03-31 (−0.03), 2024-04-01 to 2024-06-30 (−0.03),
2024-07-01 to 2024-09-30 (−0.05), 2024-01-01 to 2024-06-30 (−0.07) and 2024-01-01
to 2024-09-30 (−0.11). A fixture recording this case must show, at each, the value
unchanged and the resolved provenance moved: the later accession, the later tag,
and the rule id of the entry that reads the concept exactly rather than the one
that stands in for it. It must also show the move is not one-way — in the filings
of 2026 the filer tags `EarningsPerShareDiluted` again, at 2025-04-01 to
2025-06-30 and 2025-01-01 to 2025-06-30, alongside a restatement, so a fixture that
expected a monotone drift toward the exact entry would be recording a habit rather
than a rule.

**The fiscal-year change.** The same fixture, and what it must show here is that
no clause is added for it. The transition period 2023-05-01 to 2023-12-31 is
answered by three filings — the 10-KT filed 2024-03-12, the 10-K filed 2025-04-02
and the 10-K/A filed 2025-05-13 — and is settled by the rule above with nothing
about the change consulted: `operating_income` −6,338,622 restated to −6,032,735,
`depreciation_and_amortization` 289,067 to 30,029, `net_income` −1,251,723 to
−949,130 under a tag that also moved, and `revenue` 121,690, `pretax_income`
−1,046,973 and `operating_cash_flow` −2,828,634 unmoved, every one of them naming
the 10-K/A filed 2025-05-13 because it was filed last. The amendment is ordered by
its own `filed`, six weeks after the annual report it amends, and no rule prefers
it for being one. The year before the change, 2022-05-01 to 2023-04-30, and the
first year after it, 2024-01-01 to 2024-12-31, are ordinary periods with ordinary
contests — three answering filings and seven — and so are the quarters on both
calendars. If a fixture recording the change needs a rule this record does
not state, the record is wrong and the implementing run stops rather than adding
one.

### What this does not decide

Anything inside one filing, which is `docs/adr/candidate-choice.md`'s and is read
as given. Which canonical periods exist and which filings answer them, which are
Rules 1 and 2 as accepted. Whether a period is a year, a quarter or a transition
stub, which the accepted record left open and this one does not reopen. How
results are keyed or stored. And which metric consumes which period, which is
M5's.

## Alternatives

- **Latest-that-changed: the value is the latest figure, and it names the earliest
  filing that states it.** The reading the accepted record's Enforcement section
  takes, and the one a reader expects: provenance stops churning, a value's
  accession means "where this figure was first stated", and a change of accession
  means a restatement rather than another comparative column. Rejected on four
  grounds, the first of which is decisive. **It prefers a filing by comparing
  values.** Which filing an agreeing value names is a function of the numbers
  themselves, which is the preference Rule 3's own last sentence and anchor 5 both
  refuse; the ruleset would be reading a figure to decide the provenance of that
  figure. **And "agree" would have to be defined.** It is an equality
  test over published literals, and the stage can do it exactly — `figure` already
  reads a decimal where a composition asks for one — so the objection is not that
  it cannot be written. It is what writing it decides: whether `0` and `0.0000` are
  one value, and with it which filing a value names, becomes a reading of the
  numbers that someone has to settle, for every concept, including the ones a
  composition renders at its own scale. Neither fixture holds two literals spelling
  one number today, so this is a hole rather than a live failure, which is exactly
  when it is cheap to refuse. **It is undefined where a value changes and changes back.**
  `current_liabilities` at 2023-12-31 on CIK 0001859199 is stated 2,062,834 by the
  10-KT filed 2024-03-12, then 1,469,084 by three 10-Qs, then 2,062,834 again by
  the 10-K filed 2025-04-02 and the 10-K/A filed 2025-05-13. The earliest filing
  agreeing with the surviving figure is one whose statement was withdrawn and
  restored, and the record says nothing about which of the agreeing filings is
  meant. **And it cannot state itself for a value nothing was read for.** Three of
  the fifteen concepts at the restatement fixture's half-year are silence readings
  naming no filing, so there is no earlier filing to keep. What it costs to reject
  it is real and is priced under Consequences.
- **The original: the first filing to state a figure sets it.** Simple, stable,
  and what a reader expects of an accession. Rejected by the accepted record for
  reasons this record does not revisit — it makes a restatement a no-op, and the
  fixture's own quarter 2023-10-01 to 2023-12-31 has no original to prefer — and
  named here only because it is the other end of the axis this record chooses on.
- **Prefer the filing whose attempt read the concept exactly over one that stood
  in for it, and only then take the greatest `filed`.** It would hand the
  tag-change case the answer a reader wants for the right-sounding reason: the
  value recorded by the entry that reads the concept best. Rejected because the
  preference is on the rule that produced the value rather than on `filed`, `form`
  or `accession` — a second candidate-choice step run across filings, after
  candidate choice has already settled each one, and the first of a family (prefer
  the filing with more facts, prefer the one that reports the period as its own)
  each of which reads well and none of which is a date.
- **Record both: the winning filing and the first filing that stated the figure.**
  It would end the argument by keeping what each reading wants. Rejected: it is a
  field on the `Value` the vocabulary does not carry, so it is a
  `canonical-concepts` version bump for a convenience, and the first-stating filing
  is recomputable from the facts the value already points at. One source of truth
  per value, and the losers are recomputed rather than remembered.
- **Leave Rule 3 as accepted and let the implementing run choose.** The cheapest
  path, and it is what the milestone's schedule would prefer. Rejected because it
  is the failure mode the constitution names: a plausible reading picked inside a
  function, settling the recorded provenance of every agreeing value in the corpus
  in a file no later reader would think to check. The run that met it was right to
  stop.

## Consequences

**Easier.** Rule 3 becomes decidable, and with it Rule 1, which waits on it, and
the restatement fixture the milestone names. The choice is a maximum over one
field with a stated tie-break, so an implementation is a comparison and a fixture
is a table of accessions; nothing in it reads a value, a tag, a concept or a
filer. Replay needs no equality test, and the question of when two decimal
literals are one number stays unasked by this rule. And the rule says the same
thing about all fifteen concepts at the fixture's half-year, including the three
that name no filing, which no reading that turns on agreement does.

**Harder.** Provenance churns. Every agreeing value's accession moves to whichever
filing most recently quoted the period, so a reader who opens a value expecting
the filing that first stated the figure finds a 10-Q from two years later
quoting one line of a comparative column, and eleven values at the fixture's
half-year change their recorded filing without changing a digit. A period's row
mixes filings by construction — the fixture's quarter names three of them and, for
three concepts, none — so "which filing is this row from" has no answer, and
anything that wants one has to ask per value. Re-resolution after any new filing
can rewrite the accession, tag and rule id of values whose numbers never moved,
which is churn a stored table absorbs.

**A later filing's zero can displace an earlier filing's figure, and no clause
here stops it.** This is Rule 3's shape as accepted, not something this record
introduces, and it is named rather than patched. `short_term_investments` reads
silence as zero and the two dividend concepts read it as zero on a condition, so a
thin later filing that mentions the period at all resolves them to a `Value` of
zero, and a `Value` beats nothing — the first clause holds back an `Unknown`, and a
silence zero is not one. Neither fixture exercises it: no filer in the repository
tags a short-term-investment element or pays a dividend, and no period in either
holds a silence value beside a read one. A clause for it would prefer an attempt
by how it was set, which is not `filed`, `form` or `accession`, so it is a
decision of its own and belongs in its own record — most likely beside the
vocabulary's silence readings rather than here.

**Expensive to reverse.** The direction, once values are stored. Reversing to the
reading not taken re-resolves every value in the corpus and rewrites the
provenance of each one that agrees across filings, without changing a number —
the kind of change that is invisible in every check that compares figures. That is
the same cost the accepted record priced for Rule 3's direction, and it is the
reason this record is written before the first implementation rather than after.

## Enforcement

This changes no anchor. It applies three. Anchor 5's ban on the unsourced value,
which is the whole of why the agreeing-value exception is withdrawn: a preference
that reads the number is a value entering the proof from nowhere. One source of
truth per value, which is why the winning filing is the accession the value
already carries and the losers are recomputed. And the invariant that every
resolved fact records where it came from and which rule set it, which is what this
rule decides the content of.

There is no mechanical half, for the reason the accepted record gave: no lint
reads a claim about which of a filer's filings is current. What checks it is the
golden fixtures, and the three above state what each must show. The restatement
fixture is the sharp one, because its two tables are the rule's entire content:
fifteen concepts at the half-year and eleven at the quarter, each with the
accession it names, and three in each naming none.

What nothing checks, said plainly.

- **The tie clause is unexercised.** No two filings share a `filed` date in either
  fixture — ten filings for CIK 0002003750, fourteen for CIK 0001859199 — so both
  halves of it, the `/A` tie-break and the surviving tie that resolves to `Unknown`
  carrying both accessions, are stated and unchecked, as is the condition this
  record removed from it. A fixture holding a same-day original and amendment would
  check the first; a same-day pair that is not one would check the second. Nothing
  in the repository holds either today.
- **That `filed` is when the filer's statement became current.** It is the date
  the boundary publishes, unread against anything. A filing whose `filed` is
  wrong on EDGAR silently sets or loses a value, and no gate sees it.
- **That the silence zero above is not displacing a read figure.** Named under
  Consequences; no fixture reaches it, so nothing would go red if an
  implementation got it wrong in either direction.
- **That provenance is right rather than merely consistent.** The fixtures pin
  which accession each value names, so a rule that moved provenance the wrong way
  would go red — but only for the periods and concepts a fixture covers. Outside
  them, a wrong accession changes no number and shows up nowhere.
- **That two decimal literals spelling one number would be read as one value.**
  Nothing here compares values, so the question does not arise for this rule. It
  is recorded because the rejected reading needs an answer to it, and because any
  later record that compares two published values inherits it.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
