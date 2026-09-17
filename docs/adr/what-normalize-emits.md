# Normalize emits a filer's periods keyed by their dates alone, every concept in one state at each, and a value that names which of three ways set it, published as `canonical-concepts` v2

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-09-16, by `M4-32`
- **Decided:** —
- **Touches:** `contracts/canonical-concepts/`, at v2 — the normalize → analyze
  contract (anchor 3). Its boundary's `carries` line and what its `Value` and
  `Unknown` states carry change, and it gains the two period shapes; proposed
  here and written nowhere. A contract version is above a worker and
  `contracts/` is a protected path, which is why this is a proposal and not a
  commit. No anchor, gate or accepted record is edited, and nothing is
  implemented.

## Context

`crates/normalize/src/lib.rs` says what is missing: "Nothing runs over a filer's
history end to end through them yet." Both rules it names are decided on record —
Rule 1 by the accepted `docs/adr/period-alignment.md`, Rule 3 by
`docs/adr/which-filing-sets-the-value.md`, and the silence case that record left
open by `docs/adr/silence-beside-a-read-figure.md`, accepted on 2026-09-15. That
acceptance rides in #192, still open, and changes only its Status and review; the
text read here is the text it accepts. What the task that wires the stage would
still have to invent is the shape it emits, which the 2026-09-09 decider sweep
named: "the vocabulary's surface carries one state per concept per filer with no
period, and its `Value` asks for a source tag and a filing that an asserted value
and a silence-supplied zero do not have."

Two surfaces cannot hold that output as they stand.

**`contracts/canonical-concepts/v1.toml`.** The boundary `carries = "one of the
three states below per concept, per filer"`, which leaves no room for a history
of periods. A `Value` carries "the source tag it was read from, the filing it was
reported in, and the rule that set it"; an `Unknown`, "what was attempted: the
candidates considered and the rule that declined each". The state set is "closed:
no fourth state, no default, and no empty case, so silence in an implementation
produces nothing rather than an absence."

**`crates/contracts/src/canonical_concepts.rs`**, that surface as Rust.
`Resolution::Value` takes four required strings — `amount`, `source_tag`,
`filing`, `rule`. M4-18 did not write into it: `settling::SetBy` carries what set
a value instead — a rule over named facts, an assertion, or the silence reading —
because "only a value a rule read out of a filing's facts has all three. An
asserted value has a rule and a cited filing and no tag; a value the vocabulary's
silence reading supplies has none of the three."

Read and not reopened. `docs/adr/candidate-choice.md`, including "a flow concept
is answered only by a duration fact, a balance concept only by an instant fact"
and its six decline reasons. Rules 1 and 2 of the alignment record — a canonical
period is "a period a fact of this filer carries, as the contract publishes it:
one date for an instant, two for a duration", and it is the filer's "when at
least one concept its kind admits resolves to a `Value` at it, through entries
that answer the period asked for" — with its cover-page condition, its "the
`Value` does not carry the period; the period is the key of the row the value
sits in", and the method version it defers to "a second revision that changes a
number". Rule 3 as the superseding record states it: its carriage for the three
ways a value is set, "the rule chooses an attempt; it does not stamp a filing
onto whatever the attempt returned", a tie that "is `Unknown`, carrying both
accessions", and its two restatement tables. And M4-30's record whole, including
"each answering filing whose attempt found nothing comes to `Unknown`, carrying
what it attempted" and the version bump it names and declines.

What is readable is the five merged fetch fixtures that carry a filer's facts and
`registry/`, where no filer file includes, excludes or asserts anything, the two
kinds assigned are both `operating`, and all twenty-eight concepts admit
`operating`. Every fixture claim below was re-derived on `71365cc` by a throwaway
program outside the tracked tree that runs the landed procedure — `settling` over
`filings::answering` — and applies Rule 3 and M4-30's reading over the attempts.
It reproduces the Rule 3 record's two restatement tables and the 32 periods the
alignment record counts for CIK 0002003750.

Structural, and within reach: it proposes contract lines, the tier's own example.
It edits no anchor, moves no protected-path entry or milestone scope, and weakens
no gate. It is flagged for later human review, as the tier requires.

## Decision

### What the stage hands the next stage

**For one filer: the filer, and the set of periods Rule 1 admits, each named by
its dates and by nothing else. At each period, every concept the vocabulary
publishes, exactly once, in exactly one of the three states.**

- **The filer is named by its CIK** in the one spelling `fetch-normalize` v2
  carries, ten digits left-padded. v1's "per filer" names no key, and the stage
  on the far side reads this contract and no other — "a stage knows the contract
  on either side of it and nothing else about its neighbors" — so periods that do
  not say whose they are cannot be used.
- **A period is its dates, as the facts carried them:** one for an instant, two
  for a duration, crossing as the characters the fetch boundary published, which
  are the characters Rule 1 keys a period by. No year, quarter, `fy`, `fp`,
  `frame`, length or ordinal. Two periods are one exactly when their dates are
  equal character for character, so an instant and a duration are never one. The
  periods are a set: the order they cross in means nothing, and the last one
  handed over is not "the latest". This is the alignment record's "a period is
  its dates and has no other name", carried as the key.
- **The two period shapes are restated in v2, not referred to.** Analyze may not
  read `fetch-normalize`, so v2 states them itself. Both files are frozen, so the
  two statements cannot drift in place; a later `fetch-normalize` version that
  changed its shape reaches analyze only through normalize, the stage that reads
  both, and the version that publishes that change owns the translation.
- **Every concept at every period.** The set stays closed: no concept missing
  from a period and none twice. A `NotApplicable` repeats unchanged at every
  period of a filer whose kind excludes the concept, which is the published
  property that it "holds for every period that kind holds".
- **A period's row mixes filings per value**, as the Rule 3 record already says,
  and names no filing of its own.

### A concept whose measure the period cannot answer still takes a state

No fact answers a balance concept at a duration, or a flow at an instant. The
concept resolves to what the procedure gives it anyway: `Unknown` for the
twenty-five whose silence reads `unknown`, carrying the nothing each answering
filing's attempt found, and the silence zero for three —
`short_term_investments` at every duration, both dividend concepts at every
instant. No registry rule can reach a concept there, so M4-30's rule does not
withhold those zeros.

This record does not choose that; accepted records pin it. The Rule 3 record's
tables show `short_term_investments` at 0, "no filing: the vocabulary's zero", at
the half-year 2023-10-01 to 2024-03-31 and at the quarter 2024-01-01 to
2024-03-31, both durations, and M4-30's record says those rows stand. The
vocabulary publishes every concept's measure; which rows mean anything for a
concept, like which period a metric consumes, is M5's. The price is under
Consequences, and the shape that avoids it is argued under Alternatives.

### What a `Value` carries: the way it was set, named

**A `Value` carries its amount and exactly one of three named ways it was set,
each with its own carriage. The way is carried, never inferred from which fields
are empty.**

- **`read`** — a registry rule read it out of an answering filing's facts, and
  Rule 3 chose that filing's attempt. It carries each source tag the rule read in
  that filing, as taxonomy and tag: one for a `tag`, one per operand for a `sum`,
  the element term for a `difference`, whose concept operand the rule id names
  and replay resolves again, as `settling` already has it. It carries the filing,
  as the accession of the attempt Rule 3 chose, and the rule, as the pair of
  registry version and rule id.
- **`asserted`** — an override in the filer's registry file stated it for the
  period. It carries the rule, as the pair of registry version and the
  assertion's id, and the filing, as the accession the assertion cites. No tag:
  it was read from none. The filing is the cited one and not the answering filing
  Rule 3 chose, because an assertion is total and every answering attempt returns
  the same one, so which of them was filed last says nothing about where the
  figure came from; recording it would be the stamping the Rule 3 record refuses.
- **`silence`** — no filing answering the period reached the concept, and its
  published reading supplies a zero. It carries that reading, `zero` or
  `conditional`, and the registry version under which nothing reached the concept.
  No tag, no filing, no rule: it still carries none of the three, as the Rule 3
  record has it, because a registry version with no rule id beside it is not a
  rule.

Named, because v1's own words refuse the alternative: "no empty case, so silence
in an implementation produces nothing rather than an absence." Read off which
fields are present, a way is only as good as every writer's discipline: an
implementation that dropped a tag turns a read value into an asserted one, and
one that dropped a rule turns it into a silence zero, each a well-formed value of
another way that nothing would flag. M4-30's "the stored table tells them apart
by whether a value carries a rule at all" stays true — a silence value carries
none — and stops being the only thing the distinction rests on.

So a silence zero is never mistakable for a read one, and the merged fixtures
hold the two side by side. At the instant 2024-12-31 on CIK 0001859199,
`preferred_equity` is 0, read from `us-gaap:PreferredStockValue` in the 10-Q/A
`0002008589-25-000033` filed 2025-08-15, and `short_term_investments` is 0 that
no filing stated. The amounts are the same; the way is not.

### What an `Unknown` carries at a period

**What every filing that answered the period attempted, each attempt under that
filing's accession: the candidates it considered and the rule that declined each,
from candidate choice's closed set.**

- **The accession is what a period adds.** Inside one filing a candidate named by
  its rule was enough; across filings the same rules decline the same way in
  each. On CIK 0002003750, 387 of the 531 `Unknown`s at its admitted periods are
  attempts in more than one filing, and in all 387 the candidate names recur from
  filing to filing, so a flat list could not say which filing tried what.
- **An attempt that considered no candidate still names its filing.** Every
  `revenue` entry is kind-scoped, so on the three merged filers with no kind no
  rule is eligible and `revenue` is `Unknown` at every admitted period with no
  candidate in any attempt. At 2023-01-01 to 2023-12-31 on CIK 0001778784 that
  `Unknown` is five accessions and nothing else; without them it is the empty
  case.
- **An attempt whose silence zero M4-30's rule withheld** carries what its four
  steps found, as that record says. The landed `settle` builds the zero without
  keeping that list, so the wiring task keeps it.
- **Rule 3's surviving tie** carries the tied attempts under their accessions,
  and what left them undecided is the tie on `filed` that `form` did not break —
  Rule 3's clause, not a seventh reason of candidate choice's. No merged fixture
  holds a tie.

### Published as `canonical-concepts` v2, not as a contract beside it

**A new version of `contracts/canonical-concepts/`, and no other contract.** Three
reasons, the second decisive.

- **v1 is this boundary's contract.** It is `produced_by = "normalize"` and
  `consumed_by = "analyze"`, and its `carries` line is the line that changes. A
  contract beside it would be a second account of one boundary next to a first
  that still says "per concept, per filer", with nothing checking one against the
  other.
- **The vocabulary moves regardless.** The `Value` and `Unknown` lines sit inside
  v1's `[[state]]` entries and neither can say what this record decides, so a
  contract of its own would move two surfaces for one decision.
- **Anchor 3 gives a stage one contract on each side.** Two for one boundary is
  two to read, version and keep in step.

What v2 changes, and nothing more:

| line | v1 | v2 |
| --- | --- | --- |
| `[boundary] carries` | "one of the three states below per concept, per filer" | per filer, named by its ten-digit CIK: the periods alignment admits, each named by its dates alone in one of the two shapes below, as a set with no order; at each, every concept once, in one of the three states |
| period shapes | — | `instant`, one date; `duration`, two, its start and its end; as the characters the facts carried them |
| `Value` carries | "the source tag it was read from, the filing it was reported in, and the rule that set it" | the way it was set, one of three: `read` — each source tag it was read from, the filing it was reported in, and the rule as the pair of registry version and rule id; `asserted` — the rule as that pair and the filing the assertion cites, and no tag; `silence` — the silence reading that supplied it and the registry version under which no filing answering the period reached the concept, and no tag, filing or rule |
| `Unknown` carries | "what was attempted: the candidates considered and the rule that declined each" | what was attempted in each filing that answered the period, under its accession: the candidates considered and the rule that declined each; where Rule 3's tie left it undecided, the tied filings' accessions and the tie |

Everything else in v1 crosses into v2 unchanged in meaning: the concepts, kinds,
measures, units, signs, silence readings, applicability clauses, properties,
`[elsewhere]`, and `NotApplicable`'s "the kind and the clause". The publishing
task writes the bytes and their wording, not their content. v1 stays frozen, as
its own header requires. The registry gate reads "the highest version its own
record names", and v2's concept and kind names are v1's, so the registry needs no
edit.

### What it takes up in the same version, and what it declines

The vocabulary should move once, not twice. Three bumps are named on record.

- **A registry version on a silence-supplied value, which M4-30's record names:
  taken.** "A replay that must work from the value alone needs the version on
  it, which is a `canonical-concepts` version bump and a decision, not taken
  here." The Rule 3 record starts replay from "the filer's facts and the registry
  version a value carries", and a silence value is the one `Value` that sentence
  fails for. A zero is a claim about what one registry found, and anchor
  5 asks a result to record the premises it used. Taken now it costs one field;
  declined, it is v3.
- **The accession a silence zero displaced, which M4-30's record names as what
  the reading it rejected would need: declined.** That reading was rejected, and
  under the accepted one nothing is displaced.
- **A method version on the `Value`, which the alignment record defers:
  declined.** Its condition is a revision "that changes a number", and no value
  has been resolved or stored under any revision; neither the Rule 3 record nor
  M4-30's moves a stored number. The condition stands for the revision that meets
  it.

Named by no record and not taken: a registry version on an `Unknown`. Its
candidates carry the version in their pairs, one that considered none names no
version, and no accepted record claims replay for an absence. It is listed under
what nothing checks.

### What the golden fixtures must show

**What a normalize fixture's `expected` renders.** The fixture-writing task fixes
the spelling; the content is fixed here.

- The filer, then each admitted period by its dates as published — the fetch
  fixtures already spell a period `at <date>` and `from <start> to <end>` — in
  one fixed order that reads nothing but the characters of the dates.
- In each period, all twenty-eight concepts in the order the vocabulary publishes
  them, one to a line, each naming its state and what it carries. A `read` value:
  the amount, the way, each source tag, the accession, the rule pair. An
  `asserted` value: the amount, the way, the rule pair, the cited accession. A
  `silence` value: the amount, the way, the reading, the registry version. A
  `NotApplicable`: the kind and the kinds the clause admits. An `Unknown`: each
  attempt's accession, then its candidates with their reasons, and the tie where
  there is one.
- **The way is always rendered as a word.** A fixture that rendered a silence
  zero as a read one differs on that line even where the amounts agree, so it
  goes red.
- **The registry version renders as one word standing for the registry the
  fixture ran under wherever it is that version, and as its sixty-four characters
  wherever it is not.** A value naming another version still goes red. A registry
  edit elsewhere changes the digest without turning every normalize fixture red at
  once, which would invite writing `expected` from output — the thing the harness
  exists to refuse, since "an expected result the subject generated proves only
  that the subject agrees with itself."

**What the fixtures the milestone names must show under it.**

- **The restatement**, CIK 0002003750: 32 periods, 19 durations and 13 instants,
  each with its twenty-eight lines. The Rule 3 record's two tables render as that
  record states them: at the half-year twelve concepts and at the quarter eight
  render `read`, naming the accessions those tables name, and the three silence
  rows in each render `silence` with the registry version — rows v1 could not
  render without writing something into a tag and a filing.
- **The fiscal-year change**, CIK 0001859199. At 2024-12-31, `preferred_equity`
  renders as a `read` 0 naming `0002008589-25-000033`, and `short_term_investments`
  as a `silence` 0. Nine filings answer that instant, two state
  `PreferredStockValue` there, and the four filed after the 10-Q/A are silent on
  it, so what keeps the read zero standing is Rule 3's first clause.
- **The dividend suspension**, CIK 0001778784. The quarter 2020-04-01 to
  2020-06-30 renders `dividends_declared_per_share` as a `read` 0.03 and
  `dividends_paid` as `Unknown`, by the published `unknown_when`. At 24 instants
  where one answering filing reaches `short_term_investments` and another finds
  nothing for it, the read figure renders, as M4-30's record requires. And the
  fiscal year 2023-01-01 to 2023-12-31 renders `short_term_investments` as a
  `silence` 0, while the instant 2023-12-31 renders 198,132,000 read in the 10-K
  filed 2025-03-31 — the measure seam above, in one fixture.
- **Negative equity, the tag change, and the statements that do not fit** render
  under the same lines. No merged fixture reaches a `NotApplicable`, an assertion
  or a tie, since every assigned kind is `operating`, no filer file asserts, and
  nowhere do two attempts that produced a value share the greatest `filed`. The
  first fixture for a filer of another kind, or with an assertion, is where those
  lines are first checked.

Where a claim here does not re-derive against the fixture that records it, the
run writing that fixture stops and escalates rather than adjusting the claim.

### What this does not decide

Which metric consumes which period, or which rows mean anything for a concept:
M5's. How results are keyed and stored: the storage boundary's, which receives the
period as the key of a row and decides nothing here. Whether a period is a year,
a quarter or a transition stub, which the alignment record left open. Which
periods exist and which value stands: Rules 1 and 3, read as given — so an
assertion for a period no fact of the filer carries is emitted nowhere, which is
Rule 1 and not a rule added here. Whether the filer's kind crosses beside its
periods: the vocabulary says a kind decides "for revenue, what one means", and on
this boundary only a `NotApplicable` carries one; whether a metric needs more is
M5's question, and a version bump if it does. And anything inside one filing.

## Alternatives

- **As published: one state per concept per filer, the period named by the
  caller.** Real merits: no version, v1's bytes stand, "the `Value` does not
  carry the period" holds trivially, and `settle` is already a function of a
  caller's period. Rejected on three grounds, the first decisive. **No caller can
  name the period.** Rule 1's set is computed over the filer's facts inside
  normalize, the next stage may not read those facts, and a period a caller
  constructs "resolves to `Unknown` for every concept"; so the set crosses the
  boundary anyway, under another name. **The natural caller is a backward call:**
  analyze naming a period to normalize is a later stage calling an earlier one,
  which anchor 2 makes a build failure. **And it needs the version regardless,**
  because `Value`'s line cannot hold an asserted value or a silence zero.
- **The period carried on the `Value` itself.** Flat and self-describing, the
  easiest shape to stream and store. Rejected. **The absences need it too:** an
  `Unknown` and a `NotApplicable` are per period as much as a value is, so the
  period would sit on every state and the boundary would be a bag of concept,
  period and state. "Exactly one state per concept per period" would then be a
  property of the bag that nothing in the shape holds: two values for one concept
  at one period become writable, and so does a period missing half its concepts.
  **And it reverses an accepted sentence:** "the `Value` does not carry the
  period; the period is the key of the row the value sits in", which is the shape
  taken here.
- **A contract of its own beside the vocabulary.** It keeps v1's bytes current
  and every quotation of them, and versions carriage apart from meaning. Rejected
  for the reasons above: the `Value` and `Unknown` lines are the vocabulary's and
  move anyway, and a second contract for one boundary is a second account of it.
- **The way inferred from which fields are present, tag, filing and rule
  optional.** Smaller, and it is what M4-30's sentence about a rule's presence
  describes. Rejected because the empty case is the failure: a dropped field reads
  as another way, well-formed and wrong, which is what v1 closes its state set
  against.
- **Each period carries only the concepts its shape can answer** — flows at a
  duration, balances at an instant. It removes the zeros of the wrong measure and
  halves every row. Rejected because it reopens accepted records: the Rule 3
  record's tables show `short_term_investments` at two durations and M4-30's
  record says they stand. If that zero is judged the plausible wrong answer, it is
  removed by a record superseding those rows, not by a shape chosen here.
- **Decline the registry version on a silence value.** One field fewer, and the
  accepted records already call such a value complete with none of the three.
  Rejected: the vocabulary would move twice for a bump already named, and a
  silence zero would stay the one value replay cannot start from.

## Consequences

**Easier.** The wiring task emits one shape, and `settling::SetBy` already carries
its three ways, so crossing it is a translation rather than a decision. The
golden fixtures can be written. A read figure, an assertion and a silence zero
look different everywhere they travel, every `Value` names the registry it was
resolved under, and an `Unknown` says which filing tried what, including where
nothing was eligible.

**Harder.** Every period carries every concept, including the states of concepts
its shape cannot answer, and three of those are zeros: on CIK 0002003750, 45 of
its 96 silence zeros sit at a period of the wrong shape. A consumer that ignores
the published measure reads a zero that looks right — on CIK 0001778784, no
short-term investments over 2023 beside 198,132,000 at its end. `NotApplicable`
repeats per period. Fixtures are long: with every attempt and candidate of an
`Unknown` on a line, CIK 0002003750 renders 6,765 lines against its fetch
fixture's 1,646. Everything written against v1 moves with it — the contracts
crate's transcription and its tests, the `applicability` header. And any registry
edit changes the version on every `read` and `silence` value, as it already did
on every rule pair.

**Expensive to reverse.** The key. Once rows are stored under a dates-only key,
moving the period onto each state, or back to a caller's argument, re-keys every
stored row. And the version itself: v2 is frozen once published, so a line it
gets wrong is a v3. Nothing is stored yet, which is why this is decided before
the wiring.

## Enforcement

This touches anchor 3, by proposing a contract version, and its mechanical half
is the one every version has: the contracts gate freezes v2's bytes, and the
contracts crate compares the state names it transcribes against them. Typed as
a `shapes!` enum, the ways would be one and never both and never neither, as the
states already are. It applies two more: one source of truth, which is why the
period shapes are restated in a frozen file rather than referred to, and the
invariant that every resolved fact records where it came from and which rule set
it, which the three ways are.

What nothing checks, said plainly.

- **The carriage beyond the names.** The crate keeps prose in the bytes. That a
  `read` value names the chosen attempt's filing, or an asserted value the cited
  one, is checked by a fixture only where a fixture holds the case.
- **`NotApplicable`, assertions and Rule 3's tie.** No merged fixture reaches any
  of them, so their lines are stated and unexercised.
- **That a consumer reads a concept's measure before its row.** M5's, and nothing
  holds it.
- **That v2's period shapes are `fetch-normalize` v2's.** Restated, not generated.
  Frozen, neither drifts in place; the publishing task can compare them once, and
  nothing compares them against a later version of either.
- **An `Unknown` with no candidate names no registry version.** Not taken, for the
  reason above.
- **A silence zero in a quarter the filer paid in.** The suspension fixture holds
  two. The quarters 2020-10-01 to 2020-12-31 and 2021-10-01 to 2021-12-31 are
  admitted — the 10-Ks answering them tag twelve elements there, none a dividend
  line — so both dividend concepts read the conditional zero, while the filer's
  own dividends paid rise from 1,167,000 at nine months to 1,636,000 for 2020,
  and from 1,895,000 to 2,560,000 for 2021. That is the vocabulary's pair reading
  and M4-30's rule applied as accepted; this record neither makes nor mends it.
  What it gives is that the zero renders as `silence` and names no filing, so it
  cannot pass for the filer's statement. Whether a quarter no filing tags a
  dividend line for should read zero is the vocabulary's question, carried for a
  human.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
