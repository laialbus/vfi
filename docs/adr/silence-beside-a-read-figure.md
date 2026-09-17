# A silence reading is read over every filing that answers the period, so a later filing's silence supplies no zero beside a figure an earlier filing read

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-14, by `M4-30`
- **Decided:** 2026-09-15, by the decider
- **Touches:** how far the silence readings `contracts/canonical-concepts/v1.toml`
  publishes for `short_term_investments`, `dividends_declared_per_share` and
  `dividends_paid` reach once several filings answer one period — readings
  `docs/adr/canonical-concepts-open-questions.md` set, and through them the
  normalize → analyze contract (anchor 3). No byte of that contract moves, no
  field is added, no accepted record is edited, Rule 3 gains no clause, and
  nothing here is implemented.

## Context

`docs/adr/which-filing-sets-the-value.md` states Rule 3 whole: among the answering
filings whose attempt produced a `Value`, the attempt in the filing with the
greatest `filed` stands, and an `Unknown` displaces nothing. The decider accepted
it on 2026-09-14; its Status and review ride in #188, and the Rule 3 read here is
that accepted text, which #188 does not change. Under its own Consequences it
names a case and leaves it to this record: "a thin later filing that mentions the
period at all resolves them to a `Value` of zero, and a `Value` beats nothing —
the first clause holds back an `Unknown`, and a silence zero is not one."

The case exists because a silence reading is made inside one filing today.
Candidate choice's step 4 sends an attempt that finds no candidate to "the silence
reading the vocabulary publishes for it", and the landed procedure reads that
silence per filing — `Matched::Nothing` in `crates/normalize/src/settling.rs` is
"what 'the period's facts carry nothing this concept resolves from' is, read over
one filing". That reading predates anything that compared two filings. The
vocabulary's own words are about the period: the property "silence is silence in
the fact set" holds that "a silence reading below is read over the facts the
filer tagged for the period, not over the statements it presented."

The silence test, as `docs/adr/canonical-concepts-open-questions.md` states it:
what silence supports "is only that the filer reported facts from this filing and
none of them is under a tag the registry recognises for this concept, and that has
two causes: the filer has none of the thing, or the registry does not recognise
the element the filer used. So: **silence reads as zero only where a wrongly-read
zero cannot flatter the company on any metric that consumes the concept;
everywhere else it reads `Unknown`.**" The two dividend zeroes rest on a further
support, the pair: "a wrongly-read zero requires both members to fail at once".

Read and not reopened: `docs/adr/candidate-choice.md`'s four steps inside one
filing, including that a partial composition is not a candidate; Rules 1 and 2 of
`docs/adr/period-alignment.md`; Rule 3 as accepted, every clause of it; and the
vocabulary's three states — a `Value` carries "the source tag it was read from,
the filing it was reported in, and the rule that set it", an `Unknown` is built
"from an attempt that ran, and nothing else".

What is readable is the four merged fetch fixtures that carry a filer's facts and
`registry/concepts/`, whose entries for the three concepts name eight elements:
four for `short_term_investments`, one for `dividends_declared_per_share`, three
for `dividends_paid`. Every claim below was re-derived on main at `493ac66`.

Structural, and within reach. It settles how a published silence reading applies
across the filings Rule 2 makes answer a period, the same kind of reading the
record that set those readings made. It edits no anchor, adds no field, moves no
contract byte, protected path or milestone scope, and weakens no gate. It is
flagged for later human review, as the tier requires.

## Decision

### The two readings

**Per filing:** each answering filing's silence zero is a `Value` like any other.
Rule 3 as accepted weighs it by `filed`, so a later thin filing's zero displaces
an earlier filing's read figure. **Per period:** a silence reading is not made
while any filing answering the period reaches the concept. This record takes the
second.

### The rule, stated once

**For one concept and one period, the vocabulary's silence zero — outright for
`short_term_investments`, and on its pair condition for the two dividend concepts
— is supplied only where no filing answering the period reached the concept: where
in none of them did an assertion settle it or candidate choice's first step find a
candidate for it, and, for a conditional reading, where none reached the concept
its condition names either. Where one did, each answering filing whose attempt
found nothing comes to `Unknown`, carrying what it attempted, and Rule 3 as
accepted takes it from there.**

In the case named: the earlier filing's attempt found a candidate and read a
figure, so the later filing's attempt is `Unknown`. Rule 3's first clause holds it
back, and the read figure stands with its tag, filing and rule.

What crosses between filings is one thing per attempt: whether it reached the
concept, which is the line between a value read from a fact and one a silence
reading supplied. It is the question the landed pair condition already asks of a
partner inside one filing, where "a member an override settled is not silent,
whatever its number". Never the figure — a read zero reaches the concept like
any other read — and never `filed`, `form`, the exactness of a rule, the number of
facts a filing carries, or whether the attempt settled. No fact, figure or
composition of one filing enters another's attempt. Inside each filing the four
steps run over that filing's facts and no other's, as Rule 2 has it. The rule
names no filer, tag or number.

### Why: the silence test, applied to the case

The test licenses a zero because silence inside one filing is ambiguous between
two causes, and it picks between them by which wrong answer is safe. Where another
filing answering the same period reached the concept, neither cause is left. The
filer has the thing, since it stated a figure for this very period. The registry
recognises the element, since a rule of the concept found a candidate. What is
left is a third cause the test never had to weigh inside one filing: this filing
did not repeat the line. That supports nothing about the amount. A zero made there
is not a reading of ambiguous silence; it contradicts the filer's own statement.
So the reading is the one the test gives wherever its zero is unsupported,
`Unknown` for that attempt.

The flattery half of the test does not rescue the zero for
`short_term_investments`. That half chooses between readings of an ambiguous
silence, and it does not license overruling a stated figure. For the two dividend
concepts the zero fails even on its own ground. The pair's support is that a
wrong zero "requires both members to fail at once"; across filings, that failure
is one comparative column that quotes neither line. And the exposure the vocabulary priced — payout reading "as
perfectly covered", a streak breaking — would then fall on a filer that paid.

This is Rule 3's first clause carried to the three concepts it did not reach. "A
later filing that does not mention a line makes no claim about it, and reading its
silence as a withdrawal would turn the ordinary thinness of a comparative column
into the erasure of a figure the filer never took back." That reason names no
concept. Under the per-filing reading it holds for twenty-five concepts and fails
for these three, only because the vocabulary turns their silence into a `Value`
before Rule 3 sees it.

### Why it lives beside the silence readings and not in Rule 3

M4-23's record declined a Rule 3 clause that prefers an attempt by how it was set.
This record agrees and shows that the clause belongs to the vocabulary, for four
reasons.

- **The question is the silence test's.** Whether a zero is supported is what the
  test answers. Which filing's statement is current is what Rule 3 answers, and
  that question does not arise here: the silent filing states nothing to be
  current.
- **Rule 3 stays what was accepted.** It remains a maximum over `filed` with its
  tie on `form`. It reads nothing else, and it receives an `Unknown` it already
  knows how to hold back.
- **Only the silence reading sees both members of the pair.** Take an earlier
  filing that reads a non-zero `dividends_declared_per_share` and is silent on
  `dividends_paid`, and a later one silent on both concepts. As a Rule 3 clause,
  the declared figure would stand. But `dividends_paid` would take the later
  filing's zero, the only `Value` it has,
  since the earlier filing's attempt is `Unknown` by the pair condition. The
  stored row would pair a non-zero declared figure with a zero paid. The published
  `unknown_when` forbids exactly that result, and payout on cash flow reads it as
  covered. Read over the period, `dividends_paid` is not silent alike with its
  partner, and it is `Unknown`.
- **A contest that did not settle is not silence either.** Inside one filing the
  landed pair condition already treats a member whose first step found candidates
  as not silent, whether or not they settled. A clause over `Value`s could hold a
  zero back only beside a read figure. Silence would then mean "found nothing"
  inside a filing and "read nothing" across filings. Asked as whether an attempt
  reached the concept, it means one thing in both places.

### What it does beyond the named case, so it is not inferred

- **An earlier undecided contest holds a later zero back too.** Where one
  answering filing's first step found candidates that did not settle and a later
  one found none, the period is `Unknown`, not zero. The same sentence covers it,
  and Rule 3's text is unchanged. What changes is what the vocabulary hands Rule 3.
- **An earlier silence and a later read figure** resolve to the read figure under
  either reading. The number and the provenance are the same.
- **Every answering filing silent** is the zero, as before, naming no filing. So a
  suspension — both members silent in every filing that answers the period — reads
  zero for both, as the vocabulary record requires. M4-23's two tables stand as
  accepted, including their three silence rows at the half-year and the quarter.
- **A period one filing answers** is exactly what the landed procedure does today.
- **An assertion** is total over the period in every answering filing, so no
  silence reading is reached beside one.

### What a `Value` records under this reading

**Inside what the published vocabulary requires, and not a field more.** A value a
rule read carries its tag, its filing, and its rule as the pair of registry
version and rule id. An asserted value carries its rule and cited filing. A value
the silence reading supplied carries none of the three. The stored table tells
them apart by whether a value carries a rule at all. Under this reading a stored
silence zero also means, at its period, what the vocabulary's zero means: no
filing answering the period reached the concept. So no `canonical-concepts`
version bump is asked for.

The reading not taken could not say as much without one. There, a stored silence
zero is either a period nothing reached or a period whose read figure it
displaced, and the two look identical. Telling them apart would need the zero to
carry the accession it displaced, which is a field the vocabulary does not have: a
version bump and a decision. That is a reason against that reading, not a
workaround taken here.

### What the golden fixtures must show

**No merged fixture holds the case.** Four fetch fixtures carry a filer's facts:
CIK 0002003750 (1,643 facts in 10 filings), CIK 0001859199 (4,232 in 14), CIK
0001715819 (4,461 in 21) and CIK 0002011954 (1,157 in 4). None carries a fact
under any of the eight elements the registry names for the three concepts. So no
attempt in any of them reaches any of the three concepts, and every answering
filing is silent on them. Under this record the three concepts resolve in these
fixtures exactly as they would under the reading not taken, and nothing merged
would go red if an implementation got it wrong either way.

**The shape a fixture needs.** One of the three concepts. A period. Two filings
answering it:

- the earlier carries a fact at the period under an element a rule for the concept
  names, so that its first step finds a candidate — both operands, for the paid
  concept's sum;
- the later, with the greater `filed`, carries at least one fact at the period and
  nothing that makes any rule for the concept a candidate there — nor, for a
  dividend concept, any rule for its partner.

Such a fixture must show the concept at that period as the earlier filing's
figure, naming its accession, tag and rule. For a dividend concept whose partner
is silent in both filings, it must show the partner `Unknown`. A zero naming no
filing is the reading not taken.

**The shape is ordinary, so a fixture will meet it without being chosen for it.**
Across the four fixtures, `AssetsCurrent` — the subtotal
`short_term_investments` sits inside — is stated at 49 instants. At 38 of them the
answering filing with the greatest `filed` carries no `AssetsCurrent` fact at that
instant. At 2024-03-31 on CIK 0002003750, five filings answer. The 10-Q filed
2024-05-08, whose own balance sheet it is, carries 30 facts there. The last filed,
the 10-Q filed 2025-08-20, carries one, `StockholdersEquity`. On the cash flow
side, `NetCashProvidedByUsedInFinancingActivities`, the statement section
`dividends_paid` sits in, is stated for 59 durations, and at 13 of them the last
answering filing does not state it. None of this is a claim about securities or
dividends any fixture holds. It measures how often the later filing is silent on a
line an earlier filing stated, which is the case's whole precondition.

**The dividend suspension M4-28 records** is where the case is likeliest to
appear first. The filer paid before it suspended, and a paying period quoted later
by a filing without the dividend lines is this shape. There the fixture must show
the read figures and not zeroes. Otherwise the suspension appears to begin at the
first period a thin later filing quotes. At a suspended period where every
answering filing is silent on both concepts, both read zero, as the vocabulary
record requires. If a claim here does not re-derive against that fixture,
or against any later one, the run that records it stops and escalates rather than
adjusting the claim.

### What this does not decide

Anything inside one filing. Which periods exist and which filings answer them.
Which answering `Value` stands, beyond what the vocabulary hands Rule 3. Whether
the registry should name more elements: CIK 0001859199 states dividends under
`Dividends`, `DividendsCash` and `DividendsPreferredStock`, none of which the
registry names, so both dividend concepts are silent in every filing there and
read zero at every period under either reading. That is the registry or
vocabulary question the decider carried for a human in #188, and nothing here
touches it.

## Alternatives

- **Per filing: the silence zero is a `Value` like any other, and Rule 3 weighs it
  by `filed`.** It has real merits. It needs nothing across filings and keeps a
  silence reading decidable inside the filing that makes it, which is how the
  landed procedure already works. Rule 3 stays a pure maximum with nothing upstream
  waiting on other filings. And for `short_term_investments` the silence test,
  read literally, is satisfied: a wrong zero only raises net debt. Rejected on
  three grounds, and the first is decisive. **It erases stated figures routinely,
  not at the margin.** The case's precondition holds at 38 of 49 current-asset
  instants and 13 of 59 financing periods in the merged fixtures. A filer shaped
  like them that held securities would read zero at most of its balance-sheet
  dates, set by a filing that quoted something else; one that paid dividends
  would read zero at a share of its periods. A wrong number that looks right is
  the failure this milestone exists to prevent. That the number errs conservative for one concept does not
  make it right. **It reverses Rule 3's first clause for three concepts** by a
  route that clause never considered: silence is a withdrawal for these three and
  no claim for the other twenty-five, and that difference is stated nowhere. **And
  it breaks the pair it rests on.** A later filing silent on both concepts zeroes
  both over an earlier read figure. The double failure the zeroes rest on becomes
  one thin filing, the suspension fixture can read a suspension in a period the
  filer paid, and the stored table cannot tell that zero from a true one without a field
  the vocabulary does not carry. What rejecting it costs is under Consequences.
- **The same outcome, as a Rule 3 clause: a read `Value` over a silence-supplied
  one, then the greatest `filed`.** It settles the named case identically, and it
  keeps the vocabulary untouched. Rejected for the pair and the contest argued
  above. It leaves `dividends_paid` zero beside a non-zero declared figure, which
  the published `unknown_when` forbids, and it makes silence mean two things. It is
  also the preference by how a value was set that M4-23's record declined, placed
  in the one rule that record kept free of it.
- **Hold a zero back only beside a figure read, not beside a contest that did not
  settle.** Narrower, and it reaches exactly the case the task names. Rejected
  because the narrowing is not neutral. It needs silence to mean "no read `Value`"
  across filings while the landed pair condition means "no candidate" inside one.
  And it hands a zero to a period where the filer's facts gave the registry more
  than one candidate and nothing could choose between them.
- **Read these three silences as `Unknown` wherever more than one filing answers
  the period.** The simplest way never to displace anything. Rejected because
  most periods have more than one answering filing — 42 of the 49 current-asset
  instants and 36 of the 59 financing periods above. So it withdraws the zero at
  most of them, and a suspension becomes indistinguishable from a mapping
  failure. That is the result the vocabulary
  record refused, because GOALS.md names the suspension fixture.

## Consequences

**Easier.** Rule 3 needs no clause and stays the rule the decider accepted, so the
task that implements it inherits one comparison. The three silence-read concepts
obey Rule 3's first clause like the other twenty-five, so "silence is not a
restatement" becomes true without an exception list. The pair condition means the
same thing in a stored row as in one filing, so a declared figure never sits
beside a fabricated paid zero. And a stored silence zero means one thing: nothing
answering the period reached the concept.

**Harder.** A silence reading can no longer be settled filing by filing. The
landed `settle` supplies the zero per filing, so the implementing run must find
every answering filing's first-step outcome before any of them takes a silence
zero. Otherwise a zero enters and is withdrawn later, or never withdrawn. A newly
fetched filing that reaches the concept at a period withdraws the zero from every
other attempt there. Where that filing is the last filed, the resolved number is
its figure under either reading. And the cost Rule 3's first clause priced for
every concept now reaches these three. If a filer stops presenting a
line because the amount is gone — securities folded into cash equivalents in a
restatement whose comparative column drops the line — the old figure stands. For
`short_term_investments` that can count the portfolio twice in net debt, beside a
cash figure that now includes it: the flattering direction. The recovery is the
one the vocabulary record already gives, a per-filer assertion with a rule behind
it. Nothing on the boundary tells a withdrawn line from a short column, as
`docs/adr/period-alignment.md` said for every other concept.

**Expensive to reverse.** Once values are stored, reversing to the per-filing
reading turns read figures for the three concepts into zeroes at every period a
later filing quoted without the line. On the fixtures' shape, those changed
numbers fall at most balance-sheet dates of a filer holding securities. Nothing
is stored yet, which is why this is decided before Rule 3 is implemented.

## Enforcement

This changes no anchor; it applies two. Anchor 5's ban on the unsourced value: a
zero no filing states enters the proof only where the vocabulary's test supports
it, and a filer's own figure for the period removes that support. And the
invariant that every resolved fact records where it came from, which is why the
read figure keeps its tag, filing and rule rather than yielding to a value that
names none.

There is no mechanical half. No lint reads whether a silence reading was made
across filings. What checks it is a golden fixture holding the shape above, and
none is merged.

What nothing checks, said plainly.

- **The case itself.** No merged fixture holds a period where one answering filing
  reaches any of the three concepts, so the named case, the undecided contest
  beside a later silence, and the pair across filings are all stated and
  unchecked. An implementation that kept the per-filing reading would pass every
  gate today.
- **The order of the implementation.** That no answering filing takes a silence
  zero before every answering filing's first step has run. A fixture holding the
  shape would catch the result, but not an implementation that is right only for
  the order its filings arrived in.
- **A line withdrawn by omission.** Named under Consequences. The old figure
  stands, and for `short_term_investments` it can flatter. Nothing distinguishes
  it from a short comparative column.
- **Elements the registry does not name.** This rule holds a zero back only where
  an answering filing reached the concept. A filer that states dividends under an
  element no rule names is silent in every filing and reads zero, as CIK
  0001859199 does. That is the registry's to catch, and nothing does.
- **Which registry found nothing.** A silence-supplied value carries no rule and
  so no registry version, under this reading as under the other. Replaying a zero
  means re-running every answering filing's first step under a version the value
  does not name. This reading widens that dependence from one filing to the
  period; it does not create it. A replay that must work from the value alone
  needs the version on it, which is a `canonical-concepts` version bump and a
  decision, not taken here.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach. It settles how a published
  silence reading applies across the filings that answer one period, the same
  kind of reading the accepted open-questions record made inside one filing. It
  moves no contract byte, adds no field, edits no accepted record, weakens no
  gate, changes no anchor, and leaves Rule 3 the maximum over `filed` accepted
  on 2026-09-14. Flagged for later human review, as the tier requires.
- **Checked:** anchor 5, which the record applies rather than crosses: a zero
  no filing states enters only where the vocabulary's test supports it, and a
  stated figure for the period removes that support. The invariant that a
  resolved fact records where it came from, which the reading not taken would
  break without a field the vocabulary lacks. `which-filing-sets-the-value.md`
  as accepted, whose Consequences name this case and whose first-clause reason
  is quoted to the word; `canonical-concepts-open-questions.md`, whose silence
  test and pair support are quoted to the word; the published vocabulary's
  `carries`, `constructible_from`, `unknown_when` and "silence is silence in the
  fact set" lines, each matching `v1.toml`; `settling.rs`, whose
  `Matched::Nothing` and override-settled comments match. Re-derived on 493ac66
  from the fixtures directly: 1,643 facts in 10 filings, 4,232 in 14, 4,461 in
  21 and 1,157 in 4; no fact under any of the eight registry elements in any of
  them; CIK 0001859199's dividends under `Dividends`, `DividendsCash` and
  `DividendsPreferredStock`; `AssetsCurrent` at 49 instants, the last-filed
  answering filing silent at 38 and more than one filing answering at 42; the
  financing total at 59 durations, silent at 13, more than one answering at 36;
  five filings answering 2024-03-31 on CIK 0002003750, the 10-Q filed
  2024-05-08 carrying 30 facts there and the 10-Q filed 2025-08-20 one,
  `StockholdersEquity`. The alternatives as argued: per filing falls first on
  the routine erasure the counts show; the Rule 3 clause falls on the pair,
  since `unknown_when` forbids a paid zero beside a non-zero declared figure;
  the narrower rule makes silence mean two things; `Unknown` wherever several
  filings answer withdraws the zero at most periods and leaves the suspension
  fixture unwritable.
- **Verdict and why:** accepted. The rule reads one bit per attempt, whether it
  reached the concept, and never a figure, `filed` or form, so it stays inside
  anchor 5 and inside what a `Value` already carries: a silence zero still names
  no filing, and it now means one thing wherever it is stored. Placing it beside
  the silence readings rather than in Rule 3 is right for the reason the record
  gives and the 2026-09-12 sweep recommended: whether a zero is supported is the
  silence test's question, and which statement is current is Rule 3's. The cost
  is priced honestly. A line withdrawn by omission keeps its old figure, which
  for `short_term_investments` can flatter, and the recovery is the assertion
  the vocabulary already provides.
- **What would have changed it:** a fixture in which the per-period reading
  produced a number the filer's own later statement contradicts, or a way for
  the per-filing reading to tell a displacing zero from a true one without a
  field. Neither exists. The suspension fixture merged in #191 the same sweep
  holds the record's shape at four dividend durations and at 24 of 26
  `ShortTermInvestments` instants, where the last-filed answering filing is
  silent on a line an earlier one stated. It is the fixture the record names,
  and it argues for the reading, not against it.
