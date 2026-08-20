# The canonical concepts are what M5's metrics consume, and each resolves to a value, a kind it does not apply to, or a mapping that failed

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-08-17, by `M4-02`
- **Decided:** 2026-08-18, by `decider`
- **Touches:** the normalize → analyze contract (anchor 3), and the canonical
  concept definitions ANCHORS.md names among the surfaces reached only through
  one explicit interface. Either alone is above a worker, which is why this is a
  proposal and not a commit.

## Context

M4 turns filings into comparable numbers, and it asks for three things before
any mapping can be written: a canonical set of concepts with the meaning of each
one written down, a set holding exactly what analysis needs and nothing else,
and three resolution states of which two — not applicable and unknown — are
never confused. Nothing of this exists yet. `contracts/` has never been created,
and the contracts gate says so in its own comment: it runs over an empty set and
is waiting for the ADR that decides what goes inside a contract file.

Two constraints shape the answer rather than decorate it. The first is that the
set is a derivation, not a catalogue. GOALS.md M5 says which metrics will be
computed; those metrics have inputs; the inputs are the set. Assembling it from
what filings publish instead would produce a larger set in which no definition
could be checked, because a concept with no consumer has no test for whether its
meaning is right.

The second is that a correct absence and a failed mapping look identical
downstream. A bank has no gross profit. A mapping that did not find gross profit
also produces no gross profit. One of those is the answer and the other is the
project's stated worst failure mode — a wrong answer that looks right. Keeping
them apart by discipline is not keeping them apart, so the difference has to be
in the shape of the answer.

This is Structural rather than Constitutional: it adds a contract and its
fields, which is the tier's own example. No anchor is edited, no gate weakened,
no protected-path entry changed, and no scope in GOALS.md moved. It applies
anchor 3, anchor 5's ban on the unsourced value, and the one-source-of-truth
invariant.

## Decision

The canonical set is the twenty-five concepts below, derived from the metrics
GOALS.md M5 names. Each concept is defined here, applies to a stated set of
filer kinds, and resolves to exactly one of three states. The vocabulary is
published as one versioned contract at `contracts/canonical-concepts/`.

### The three states

A concept resolves to exactly one of `Value`, `NotApplicable`, or `Unknown`.
The set is closed — there is no fourth state, no default, and no empty case, so
silence in an implementation produces nothing rather than an absence.

What keeps the two absences apart is that each is constructible only from a
witness the other cannot obtain, and the two witnesses come from disjoint
inputs:

- **`NotApplicable`** is constructible only from a filer kind together with the
  applicability clause of the concept that excludes that kind. Both are read out
  of this vocabulary. No filing is consulted, and for a filer whose kind admits
  the concept no such clause exists, so the state cannot be built.
- **`Unknown`** is constructible only from a resolution attempt that ran and
  returned nothing, and it carries what was attempted — the candidate tags
  considered and the rule that declined each. Where no attempt ran there is
  nothing to carry, so the state cannot be built.
- **`Value`** carries the provenance M4 requires of every resolved value: the
  source tag, the filing it was reported in, and the rule that set it.

Applicability is asked first, from the concept and the kind alone, and a concept
that is not applicable is never looked up. So a lookup failure has no path to
`NotApplicable`: the code that would build one has no clause to point at, and
the code that builds `Unknown` has no kind to point at. This is the whole of the
separation, and it holds by what each state can be made from rather than by
which branch a reader remembers to take.

Three properties follow, and each is worth stating because each is a mistake
this shape makes impossible:

- **No kind, no `NotApplicable`.** The state is a positive claim about a
  company. A filer whose kind has not been established resolves every concept
  through the attempt, to `Value` or `Unknown`, and never to a correct absence
  nobody established.
- **`NotApplicable` does not vary by filing.** It is a function of the kind, so
  it holds for every period that kind holds. A concept not applicable in one
  period and a value in the next means the kind changed, which is an event on
  the filer's record, not a judgement someone made about a filing.
- **An absence keeps its state.** The state travels with the concept into
  storage and presentation, because M5 requires an absent metric to be absent
  with a reason and M7 requires absence to be shown as absence with the reason.
  The state and its witness are that reason.

### What makes a concept not applicable

The test is whether the company's accounting shape contains the thing the
concept names — whether this filer could present the line and be understood. Not
whether it did, not whether the amount is small, and never whether the mapping
found it.

A bank's income statement has no cost of revenue. Its costs are interest paid on
deposits and borrowings, provisions for credit losses, and operating expenses;
there is no top line to subtract a cost of sales from. Gross profit for a bank
is not a number that is missing, it is a number that does not exist, and this
vocabulary states that from the kind alone, before any filing is opened.

The counter-examples are what keep the state honest. A software company presents
no inventory line: inventory is defined for it and it simply has none, so the
answer is a value of zero. A manufacturer whose filing carries no inventory tag
the registry recognises is `Unknown`. Three companies, three states, and which
one arrives is decided by the shape of the company and the outcome of the
attempt.

Applicability is about the concept's existence, not the metric's usefulness.
Interest expense exists for a bank and applies to it; that a bank's interest
coverage means little is M5's business, reached through its own inputs. Widening
`NotApplicable` to cover "not meaningful here" would make it the dumping ground
this decision exists to prevent.

### The filer kinds

Applicability is stated over a closed set of three kinds, each named for the
accounting shape it presents:

- **`operating`** — a non-financial filer presenting a classified balance sheet
  (current against non-current) and an income statement running revenue → cost
  of revenue → operating expenses → operating income.
- **`bank`** — a depository or lender earning net interest income and fees,
  funded by deposits and borrowings, presenting an unclassified balance sheet
  ordered by liquidity rather than by term.
- **`insurer`** — a filer earning premiums and investment income against claims
  and reserves, presenting an unclassified balance sheet and no cost of revenue.

A kind is a fact about the filer, resolved once and recorded with the resolved
facts. How a filer is assigned one is the registry's decision, not this one;
what this fixes is that the assignment happens before any concept is looked up
and is never re-derived per filing. Whether three kinds are enough is open
below.

### The set

Twenty-five concepts. Each carries what it means, the kinds it applies to,
whether silence in the filing's presented statements reads as a zero or as
`Unknown`, and the M5 metric that consumes it — a concept no metric consumes is
not in this list.

Every monetary amount is a single figure in the filing's reporting currency at a
scale of one, never a figure left implicitly in thousands. Counts are in shares.
Each concept is either a flow measured over a period or a balance measured at an
instant, marked below; which period or instant a figure belongs to is the
alignment ruleset's, not this one's. Amounts are signed as the statement
presents the thing they name, except for the three marked as magnitudes, which
are positive when the company spent the money — they appear negative in a cash
flow statement and positive in an expense line, and a ratio that divides by one
cannot tell which it received. Negative is a value and never an absence:
GOALS.md names negative equity among the fixtures for exactly this reason.

**Income statement — flows over a period**

- **`revenue`** — sales of goods and services in the period, net of returns,
  discounts and allowances, before any cost; the top line. For an `operating`
  filer it excludes interest and investment income and gains on disposals. What
  it names for a `bank` or an `insurer` is open below. *Applies to:* all kinds.
  *Silence:* `Unknown`. *Consumed by:* gross, operating and net margin;
  price/sales; Piotroski's ΔTURN; Altman's X₅.
- **`gross_profit`** — revenue less the cost of producing what was sold: direct
  materials, direct labour, and the overheads the filer charges to cost of
  revenue. Where a filer presents a cost of revenue rather than a gross profit,
  this is revenue less that cost — one concept, arrived at by a rule the
  registry states and the resolved value records. *Applies to:* `operating`.
  *Silence:* `Unknown` — an operating filer presenting neither a gross profit
  nor a cost of revenue leaves this unknown, not zero. *Consumed by:* gross
  margin; Piotroski's ΔMARGIN.
- **`operating_income`** — profit from the ordinary business after all operating
  costs and before interest and tax, as the income statement presents it.
  Excludes interest expense, interest and investment income, and tax. *Applies
  to:* `operating`; for a `bank` or an `insurer` interest is a cost of the
  ordinary business, so a profit measured before it is not a figure those
  filers present or a reader of them would use. *Silence:* `Unknown`. *Consumed
  by:* operating margin; return on capital employed and on invested capital;
  EV/EBIT and, with depreciation and amortisation, EV/EBITDA; interest coverage;
  Altman's X₃.
- **`pretax_income`** — profit before income tax and after interest. *Applies
  to:* all kinds. *Silence:* `Unknown`. *Consumed by:* the effective tax rate
  that turns operating income into NOPAT for return on invested capital.
- **`income_tax_expense`** — total income tax charged against the period's
  profit, current and deferred together, as the income statement presents it —
  the charge, not the cash paid. Negative when it is a benefit. *Applies to:*
  all kinds. *Silence:* `Unknown`. *Consumed by:* the same effective tax rate.
- **`net_income`** — profit for the period after tax and after the share
  attributable to non-controlling interests: what belongs to the parent's
  shareholders. Whether this is the bottom line or income before discontinued
  and extraordinary items is open below. *Applies to:* all kinds. *Silence:*
  `Unknown`. *Consumed by:* net margin; return on assets and on equity; dividend
  payout on earnings; retention and sustainable growth; Piotroski's ROA, ΔROA
  and ACCRUAL.
- **`interest_expense`** — the cost of borrowed money charged to the period,
  gross of interest income, as a magnitude. Where a filer presents only a net
  interest figure, a rule the registry states resolves it, and a net figure that
  cannot be decomposed leaves this `Unknown` rather than being taken as gross.
  *Applies to:* all kinds. *Silence:* `Unknown` — netting is common enough that
  an absent line is more often a presentation than a debt-free company.
  *Consumed by:* interest coverage.
- **`depreciation_and_amortization`** — depreciation of tangible assets and
  amortisation of intangibles charged to the period, read from the cash flow
  statement's reconciliation where the income statement does not present it
  separately. *Applies to:* all kinds. *Silence:* `Unknown`. *Consumed by:*
  EBITDA, and through it EV/EBITDA.

**Shares and per-share amounts**

- **`earnings_per_share_diluted`** — profit per share on a fully diluted basis,
  as the filer reports it. A flow over the period. Not net income divided by a
  share count: dilution adjusts the numerator as well, and the two disagree
  wherever convertibles exist. *Applies to:* all kinds. *Silence:* `Unknown`.
  *Consumed by:* price/earnings; dividend coverage and payout in their
  per-share form; the earnings growth and stability tests in the established
  screens.
- **`diluted_shares_weighted_average`** — the weighted average share count that
  is the denominator of reported diluted earnings per share. A flow-weighted
  count over the period. *Applies to:* all kinds. *Silence:* `Unknown`.
  *Consumed by:* Piotroski's EQ_OFFER; the per-share step of the discounted cash
  flow.
- **`shares_outstanding`** — common shares outstanding at a stated date at or
  near the period end. A count at an instant, not an average, and not
  interchangeable with the one above. *Applies to:* all kinds. *Silence:*
  `Unknown`. *Consumed by:* market capitalisation, and through it price/book,
  price/sales, enterprise value and Altman's X₄.
- **`dividends_declared_per_share`** — dividends declared per common share for
  the period, in the reporting currency. A flow. Whether special dividends
  belong here is open below. *Applies to:* all kinds. *Silence:* zero — a filer
  declaring a dividend must present it, so silence is the statement that none
  was declared, and a suspension has to read as a zero or the streak cannot be
  measured. *Consumed by:* yield and relative yield; dividend growth rate and
  streak; the dividend discount valuation; payout and coverage in their
  per-share form.

**Balance sheet — balances at an instant**

- **`total_assets`** — everything the company controls at the period-end date,
  as the balance sheet totals it. *Applies to:* all kinds. *Silence:* `Unknown`.
  *Consumed by:* return on assets; asset turnover; liabilities to assets;
  Altman's X₁, X₂, X₃ and X₅; Piotroski's ROA, ΔROA, ΔLEVER and ΔTURN.
- **`current_assets`** — assets expected to become cash within the operating
  cycle or one year, as a classified balance sheet subtotals them. *Applies to:*
  `operating`; a `bank` or an `insurer` presents an unclassified balance sheet,
  so the split is not a fact those filers assert and there is no subtotal to
  read or to compute. *Silence:* `Unknown`. *Consumed by:* current and quick
  ratios; working capital for Altman's X₁; Piotroski's ΔLIQUID.
- **`inventory`** — goods held for sale, in production, or as materials for
  production, at the period-end carrying amount. *Applies to:* `operating`.
  *Silence:* zero — a filer with material inventory must present it, so silence
  is a services company having none. *Consumed by:* the quick ratio.
- **`cash_and_equivalents`** — cash and demand deposits plus investments near
  enough to maturity that their value is not sensitive to rates: the balance the
  cash flow statement reconciles to. Whether short-term investments join it is
  open below. *Applies to:* all kinds. *Silence:* `Unknown`. *Consumed by:* the
  cash ratio; net debt in enterprise value; the equity bridge in the discounted
  cash flow.
- **`total_liabilities`** — everything owed at the period-end date. *Applies
  to:* all kinds. *Silence:* `Unknown`. *Consumed by:* liabilities to assets;
  Altman's X₄.
- **`current_liabilities`** — obligations due within the operating cycle or one
  year, as a classified balance sheet subtotals them. *Applies to:* `operating`,
  for the reason `current_assets` gives. *Silence:* `Unknown`. *Consumed by:*
  current, quick and cash ratios; working capital for Altman's X₁; Piotroski's
  ΔLIQUID; the capital employed denominator of return on capital employed.
- **`short_term_debt`** — interest-bearing borrowings due within a year,
  including the current portion of long-term debt, notes payable and commercial
  paper. Excludes trade payables, accrued expenses, and a bank's deposits, none
  of which is a borrowing in the leverage sense. *Applies to:* all kinds.
  *Silence:* zero. *Consumed by:* total debt in debt/equity; net debt in
  enterprise value and the discounted cash flow bridge.
- **`long_term_debt`** — interest-bearing borrowings due beyond a year,
  excluding the current portion. Whether operating lease liabilities belong here
  is open below. *Applies to:* all kinds. *Silence:* zero. *Consumed by:* the
  same as above, plus Piotroski's ΔLEVER.
- **`shareholders_equity`** — the parent's shareholders' residual claim: total
  equity less the portion attributable to non-controlling interests. Negative
  when liabilities exceed assets, which is a value. *Applies to:* all kinds.
  *Silence:* `Unknown`. *Consumed by:* return on equity; price/book;
  debt/equity; sustainable growth; invested capital for return on invested
  capital.
- **`retained_earnings`** — cumulative profit retained rather than distributed
  since inception, negative when accumulated losses exceed profits. *Applies
  to:* all kinds. *Silence:* `Unknown`. *Consumed by:* Altman's X₂.

**Cash flow — flows over a period**

- **`operating_cash_flow`** — net cash generated by operations in the period, as
  the cash flow statement subtotals it, after working capital movements and
  after interest and tax where the filer classifies them there. *Applies to:*
  all kinds. *Silence:* `Unknown`. *Consumed by:* free cash flow, and through it
  price/free cash flow, dividend payout on free cash flow and the discounted
  cash flow; Piotroski's CFO and ACCRUAL.
- **`capital_expenditure`** — cash spent in the period acquiring and improving
  property, plant and equipment and capitalised software, as a magnitude.
  *Applies to:* all kinds. *Silence:* `Unknown` — filers split and label this
  across several investing lines, so an absence is far more often a mapping that
  found nothing than a company that bought nothing, and reading it as zero would
  overstate free cash flow by the whole of it. *Consumed by:* free cash flow.
- **`dividends_paid`** — cash paid to shareholders as dividends during the
  period, from financing activities, as a magnitude. *Applies to:* all kinds.
  *Silence:* zero. *Consumed by:* dividend payout on free cash flow and on
  earnings in their total form; dividend coverage.

Five applicability clauses in twenty-five concepts, all of them `operating`
only: `gross_profit`, `operating_income`, `current_assets`,
`current_liabilities`, `inventory`. The surface is deliberately small, because
every clause is a place where a wrong applicability claim would hide a real
absence behind a correct-looking one.

Those five propagate: a `bank` has no working capital and no EBIT, so Altman's
X₁ and X₃ have no inputs and the Z-score is not computable for it at all. That
is the right answer arriving without anyone deciding it per filing — Altman's
discriminant was fitted on manufacturers, and financial firms were not in the
sample. How an inapplicable input makes a metric absent is M5's rule, not this
one's.

### What is left out, and why

- **Derived values** — free cash flow, EBITDA, total debt, net debt, working
  capital, book value per share, the effective tax rate. Each is a combination
  of concepts above and belongs to analysis. Carried here, each would have one
  definition in normalize and another in analyze, which is the unchecked
  duplication the one-source-of-truth invariant bans, and a derived value's
  provenance is a rule rather than a source tag, so it could not satisfy M4's
  provenance requirement.
- **`cost_of_revenue`** — no metric consumes it. It is one of the two ways
  `gross_profit` is reported, which makes it the registry's business.
- **Everything no M5 metric consumes** — receivables, payables, goodwill,
  intangibles, SG&A, research and development, deferred taxes, segment lines,
  employee counts, funds from operations. Each would cost a mapping rule, an
  applicability clause and a fixture for a number nothing reads.
- **Price and market capitalisation.** They are not in filings and do not cross
  this boundary. Market data arrives through the price provider interface, and
  ANCHORS.md already makes every price-dependent metric optional at the type
  level, so a missing price is a state that interface owns rather than one this
  vocabulary needs.
- **Preferred equity and preferred dividends** — held out pending the open
  question below, which is what decides whether they are two concepts or none.

### The contract

The vocabulary is published at `contracts/canonical-concepts/`, in the shape
`scripts/gates.sh` already fixes:

- `contracts/canonical-concepts/versions` — one `v<N> <sha256>` line per
  published version, consecutive from v1. At first publication it holds exactly
  one line.
- `contracts/canonical-concepts/v1.<ext>` — the single file that is the surface
  at v1. Once its line is in `versions` it is frozen; every later change
  publishes `v2.<ext>` with its own line, edits nothing already published, and
  deletes nothing `versions` names.

The directory carries the name ANCHORS.md and GOALS.md already use for this
surface — the canonical concept definitions — rather than a name for the edge it
sits on, because a second name for one thing is the duplication the constitution
bans.

v1 states: the contract's identity and the boundary it sits on; the three
states as a closed set; the three filer kinds and what each names; and, per
concept, its name, its meaning, whether it is a flow or a balance, its unit and
sign convention, the kinds it applies to, and its silence reading.

v1 does not state the tags a concept maps from — that is the registry, versioned
separately, or a concept's meaning would change every time a filer changed a
tag. Nor does it state the metrics that consume each concept: those are the
derivation and they live in this record, because compiling M5's list into the
normalize boundary would hand a stage knowledge of something two stages away,
which anchor 3 denies it. Nor any threshold or default, which anchor 5 puts in
the settings layer and the constants file.

The file format is one decision for every contract in the repository, not this
contract's alone. Whichever of M4's two contract ADRs is decided first settles
it; this one proposes TOML if it is first, making the extension `v1.toml`, and
defers to `fetch-normalize-contract.md` without re-arguing if that record has
already chosen otherwise.

### What this does not decide

What normalize is handed is M4-01's. Which filing tags resolve to which concept,
and how a filer is assigned a kind, are the registry's. Choosing among several
candidates in one filing, and reconciling amendments, restatements and period
alignment, are their own rulesets. Which metric consumes what, and how an
inapplicable or unknown input makes a metric absent, are M5's. This decides the
vocabulary.

### Open questions

These are the places where two defensible readings produce different numbers.
Each must be closed before v1 is published; none is closed by picking the
reading that lets the next task start.

1. **What `revenue` names for a `bank` or an `insurer`.** Total interest income
   plus non-interest income, or net interest income plus non-interest income.
   The two differ by interest expense, which for a large bank is a large number,
   so price/sales and every margin differ with it. Turns on whether revenue
   means the gross inflow or the top line a reader of those statements would
   call revenue.
2. **Whether `net_income` is the bottom line or income before discontinued and
   extraordinary items.** Piotroski's signals are defined on income before
   extraordinary items, and ANCHORS.md requires his thresholds to cite him —
   a different numerator silently changes what the score means. Price/earnings
   convention runs the other way. Turns on whether one concept serves both or
   the set carries two.
3. **Whether preferred claims are carried.** Return on equity, price/book and
   payout are common-shareholder measures, and a filer with preferred stock
   outstanding distorts each unless preferred equity leaves the denominator and
   preferred dividends leave the numerator. Turns on whether M5's metrics are
   stated for common shareholders or for all equity holders; the answer is
   either two more concepts or none.
4. **Whether operating lease liabilities are debt.** Since ASC 842 an operating
   lease sits on the balance sheet as a liability. Counting it in
   `long_term_debt` raises leverage and enterprise value for every retailer and
   airline; leaving it out keeps a series comparable across the 2019 boundary.
   Turns on which comparability matters more — and possibly on carrying the
   lease liability as its own concept, which would make the choice a setting in
   M5 rather than a meaning fixed here.
5. **Whether short-term investments join `cash_and_equivalents`.** Net debt and
   the cash ratio move with the answer, and a company holding its liquidity in
   marketable securities looks materially more leveraged under the narrow
   reading. Turns on the same fork as above: one concept with a wide meaning, or
   two combined in analysis.
6. **Whether a special dividend belongs in
   `dividends_declared_per_share`.** A one-off distribution lifts the declared
   figure for one year and then falls out, which reads as a cut and breaks a
   streak that never broke. Turns on whether the concept is one or splits into
   regular and special — a vocabulary question, which is why it is here rather
   than in M5.
7. **Whether three filer kinds are enough.** REITs, regulated utilities and
   investment companies each present statements that differ from an `operating`
   filer's. Turns on reading those presentations against the five applicability
   clauses above: if any of the five is wrong for them, the kind set is short.
   That is a check rather than a judgement, and it must be done before v1.
8. **What silence is silence in.** The zero readings above are stated over the
   filing's presented statements. If what normalize is handed is a subset of
   those — XBRL company facts, say — an absent fact is not an absent line, and
   every zero reading must fall back to `Unknown` or the mapping fabricates
   zeroes. Turns on M4-01, and closes when that record does.

## Alternatives

- **Take the set from what filings publish.** Normalize the elements most filers
  tag and let analysis pick what it wants. Rejected: GOALS.md M4 asks for
  exactly what analysis needs and nothing else, and every surplus concept costs
  a mapping rule, an applicability clause and a fixture for a number nobody
  reads. Worse, the set stops being derivable — a concept with no consumer has
  no test for whether its definition is right, which is the one check this
  vocabulary has.
- **Carry the derived values as concepts** — free cash flow, EBITDA, total debt,
  book value per share — since that is what consumers actually want. Rejected:
  the day one is carried it has a definition in normalize and a definition in
  analyze, which is the unchecked duplication the one-source-of-truth invariant
  bans; and its provenance would be a rule rather than a source tag, so M4's
  requirement that every resolved value record its tag and its filing could not
  be met for it.
- **One absence state carrying a reason.** A single `Absent` with text saying
  why. Rejected: this is precisely the convention the milestone forbids. Any
  code path can write any string, nothing stops a failed lookup writing "not
  applicable", and the distinction GOALS.md requires never to be confused would
  be held by a habit rather than by what the state can be built from.
- **Decide applicability per filing rather than per kind.** Mark a concept not
  applicable when the filing plainly has no such line. Rejected: it makes the
  two absences depend on the same evidence, which is the one thing that must not
  be true of them, and a company that merely stopped tagging a line would
  quietly become a company that never had one.
- **Name the directory for the boundary, `contracts/normalize-analyze/`.**
  Consistent with a contract being an edge rather than a thing. Rejected: the
  constitution already calls this surface the canonical concept definitions, and
  a second name for one surface is duplication; the vocabulary is also what the
  registry maps into and what M6's wide table is keyed by, so naming it for one
  edge understates what it fixes.

## Consequences

Easier: the registry task acquires a target — twenty-five concepts with a
definition and an applicability clause apiece — so its work is finding the tags
for these rather than deciding what matters. M5's metrics have a vocabulary
before they have code. The hard-case fixtures GOALS.md names become writable in
terms of expected states: a bank fixture asserts `NotApplicable` for five
concepts, a dividend suspension asserts a zero rather than an absence, and
negative equity asserts a negative value.

Harder: twenty-five is a floor. Every M5 metric that turns out to need a
twenty-sixth concept is a version bump — mechanically cheap, a new file and a
new line, but an ADR each time, by the anchor that made this one an ADR. The
five `operating`-only clauses mean a bank gets no Z-score and no return on
invested capital, so M6's default composite must either work with fewer inputs
for financials or exclude them; this record pushes that decision to M6 rather
than making it. And eight questions must be closed before v1 is published —
three of them (revenue for banks, preferred claims, lease liabilities) change
numbers rather than names.

For whoever queues the follow-on task: `contracts/` is protected and grantable,
so the task that writes v1 must list `contracts/canonical-concepts/` under
`owns` in its task file as committed on `origin/main`, or the hook refuses the
write; and the merge still needs the human-approved label. That is
`protect-paths-owns-grant.md` working as intended, not an obstacle, but it has
to be in the task file rather than discovered by the run.

Expensive to reverse: a concept's meaning, once fixtures assert values under it
and the store holds columns keyed by it. Adding a concept is cheap and bounded.
Changing what one means is a re-normalization of everything already stored, and
nothing in a stored value distinguishes the old meaning from the new — which is
why the definitions above are stated at the length they are.

## Enforcement

This changes no anchor; it applies three. The half that is already mechanical:
the contracts gate. Once `versions` names v1, that version's bytes are frozen,
and an edit to the published surface goes red with no change to the gate. That
is what makes a concept's meaning cost a version rather than a diff.

What nothing checks, said plainly:

- **That the three states stay three, and that no path builds `NotApplicable`
  from a failed lookup.** The vocabulary makes it constructible only from a kind
  and a clause; nothing yet proves an implementation obeys that. The shape of
  the check is a pair of fixtures — a bank whose expected result is
  `NotApplicable` for the five, and a filing with a recognised tag removed whose
  expected result is `Unknown` for the same concept — and it belongs to the task
  that builds the resolver. Until then this paragraph is the only guard it has.
- **That every concept in the file still has a consumer.** The derivation lives
  in this record and nothing re-checks it when M5 changes. A metric dropped at
  M5 leaves its concept behind and nothing goes red.
- **A collision waiting in `contracts/`.** `docs/layout.md` puts `contracts/` at
  the root as a workspace member, package `vfi-contracts`, while
  `scripts/gates.sh` reads every directory under `contracts/` as a contract. The
  day that crate exists with a `src/`, the gate reads `src` as a contract with
  no `versions` file and fails. This belongs to the task that creates
  `contracts/`, and neither file is this task's to edit; it is named here
  because it will be hit by the first contract written and it is cheaper to know
  now than to diagnose then.

## Decision review

- **Authority:** Structural, and within reach: a contract and its fields, the
  tier's own example. No anchor is edited, no gate weakened, no protected-path
  entry changed. The protected directory it names is written later, by a task
  carrying its own `owns` grant and the human-approved label — nothing here
  writes it.
- **Checked:** anchors 2, 3 and 5 as applied, and the one-source-of-truth
  invariant behind both the directory's name and the exclusion of derived
  values. `protect-paths-owns-grant.md`, which the follow-on note relies on,
  says what this record says it says. No accepted ADR is contradicted; M4-01's
  record does not exist yet, so this one is decided first and its TOML proposal
  settles the contract file format for the repository. The five alternatives
  are argued for real — per-filing applicability and the single reasoned
  `Absent` state are the two that would have quietly rebuilt the confusion M4
  forbids, and both rejections give the mechanism, not a preference.
- **Verdict and why:** accepted. The set is a derivation with its own check —
  every concept names the metric that consumes it, so a definition can be
  tested against its consumer — and the two absences are separated by what
  each state can be constructed from, not by convention. Where two defensible
  readings differ, the record stops and says so: eight open questions gate v1
  instead of being closed by whichever answer unblocks the next task. That is
  M4's "nothing is ever guessed" applied to a vocabulary, and it is why this
  is acceptable with the questions still open.
- **What would have changed it:** a concept with no named consumer, an
  applicability clause stated per filing rather than per kind, or any
  construction path from a failed lookup to `NotApplicable`. Any one would
  have meant the record's two shaping constraints did not actually hold, and
  it would have gone back as a rejection.

Flagged for later human review, as the tier requires. The open questions are
not this review's to close; three of them change numbers, and v1 does not
publish until they are decided.
