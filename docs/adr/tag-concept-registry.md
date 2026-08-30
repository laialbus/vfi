# The tag mapping is data under `registry/`, versioned by the digest of its own bytes, reached through one interface, and overridden one filer at a time

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-08-30, by `M4-08`
- **Decided:** not yet, by the decider
- **Touches:** a new versioned data surface and the gate over it, a new explicit
  interface for the component ANCHORS.md names as the tag mapping, and a new
  top-level directory, which is an amendment to `docs/layout.md`. Any one of the
  three is above a worker, which is why this is a proposal and not a commit.

## Context

Both surfaces this sits between are published and frozen. `fetch-normalize` v1
hands normalize every fact the filer tagged, each carrying its taxonomy, tag,
unit, period, value, accession, form and filing date, filtered by nothing.
`canonical-concepts` v1 publishes twenty-eight concepts, five filer kinds, and
three states a concept resolves to. Each names this registry in its own
`[elsewhere]` table — `tag_to_concept = "the mapping registry, versioned
separately, with its per-company overrides"` — and neither says anything more
about it.

ANCHORS.md fixes what it must be: "The mapping from filing tags to canonical
concepts is a versioned data registry with per-company overrides — not a pile of
branching logic," and "Every resolved fact records where it came from and which
rule set it." It also names the tag mapping among the components that are
"reached only through an explicit interface," whose internal shape is private.
GOALS.md M4 asks for the same thing in one line: "Mapping happens through a
versioned registry with per-company overrides — data, not branching code."

What is open is everything about the shape: where the registry lives, in what
format, how a version is named and what a change to one means, how a rule is
identified, how an override is written and how it meets the general mapping. A
run that had to write a mapping today would decide all of it mid-run, one
plausible `if` at a time, which is the branching pile the anchor bans.

This is Structural rather than Constitutional: it adds a data surface, a gate and
an interface, which are the tier's own examples. No anchor is edited, no gate
weakened, no protected-path entry changed, and no scope in GOALS.md moved.

## Decision

### What the registry answers, and what it cannot

The registry is asked one question, per filer and per concept: which rules may
reach this concept for this filer, and what is each called. It is never asked
about a tag. Resolution runs concept-first — the vocabulary names twenty-eight
concepts and a filer's document publishes hundreds of elements, almost none of
which any metric consumes. **A tag no concept claims is therefore not a case the
registry has.** It is never looked at, produces no failure, and is recorded
nowhere. Finding tags worth adding is a different job, and the layout already
holds a home for it: a development entry point under `crates/normalize/src/bin/`,
reading the same registry through the same interface as the stage.

The registry produces one of two outcomes: the rules that matched facts, or
nothing matched. It cannot produce `NotApplicable`. The vocabulary makes that
state constructible only "from a filer kind together with the applicability
clause below that excludes that kind, and nothing else. No filing is consulted",
and it asks applicability first, so a concept that reaches the registry at all is
one the filer's kind admits. That is the vocabulary's own rule about what each
state can be built from; this record restates it and does not reopen it.

"Nothing matched" is handed to the silence reading the vocabulary publishes for
that concept: `Unknown` for twenty-five of them, a `Value` of zero for
`short_term_investments`, and the conditional pair for the two dividend concepts.
So **a concept no tag reaches resolves to `Unknown`, or for those three to a zero
the vocabulary chose deliberately, and never to the state that means a correct
absence.** A concept with no entry at all in the registry resolves the same way,
for the same reason: the attempt ran over an empty set of rules, and an attempt
that ran and returned nothing is exactly what `Unknown` is built from.

### Where it lives

`registry/`, a new top-level directory, holding one file per concept at
`registry/concepts/<concept>.toml` and one file per filer at
`registry/filers/<cik>.toml`, in TOML.

The data sits at the root and the code that reads it lives with the crate that
uses it. That is not a new shape: `docs/layout.md` states it for `fixtures/`,
`benchmarks/` and `scripts/tests/`, and states it again as the reason the type
the stages compile against is a crate rather than something inside `contracts/`.
One file per item, for the reason the layout gives for the queue and the session
entries: two runs adding overrides for two filers must not be two runs editing
one file. TOML because it is the format both published contracts already use, and
because — by the interface below — it is the cheapest thing in this record to
change later.

It is not published as a contract. A contract under `contracts/` is a boundary
between two stages, and this is not one: it has a single reader, normalize, and
no producing stage at all. Four things follow from trying anyway, and together
they decide it. What a contract presents at version N is one whole file, so every
added tag would be a complete copy of the registry and the diff that adds one
line would be unreadable. `contracts/` is protected, so every added tag and every
override would need human sign-off and the `owns` grant — and an override is
meant to be ordinary per-filer data, so protection at that rate does not slow the
registry down, it stops it and pushes the pressure back toward branching code.
One file per version cannot hold one file per filer, so parallel override work
would collide on exactly the file the layout separates things to avoid. And
`contracts/` would come to mean two things, which costs the contracts gate the
one shape it reads.

A new top-level directory is an amendment to `docs/layout.md`, and the layout
says the amendment lands in the same diff as the work that needed it. That diff
is the task that creates `registry/`, not this one.

### What one rule is

An entry in the registry is a rule. The form set is closed, and adding a fifth
form is an ADR, on the same reasoning that makes a twenty-ninth concept one.

- **`tag`** — one element, named by its taxonomy and its tag together, because
  the boundary publishes both and two taxonomies may define one name.
- **`sum`** — two or more elements whose values add to the concept, for the filer
  that presents the components and no subtotal.
- **`difference`** — one concept less one element. It exists for `gross_profit`
  and for nothing else at v1: `cost_of_revenue` is not a concept, the vocabulary
  record having left it out and said in the same breath that it "is one of the
  two ways `gross_profit` is reported, which makes it the registry's business",
  so it appears here as an operand and nowhere else. An operand that is a concept
  draws an edge between concepts; the set of those edges must be acyclic, which
  the gate checks.
- **`assert`** — a value stated directly for one filer, one concept, one period.
  Available in a filer's file only, never in the general mapping.

An entry carries the kinds it applies to. Scope is required where the tag's
meaning *under that concept* depends on the kind, and omitted otherwise; an
entry with no scope applies to every filer, including one whose kind has not been
established. Two things follow. `revenue` is the one concept for which the
vocabulary publishes a reading per kind, so every one of its entries is scoped
and a filer with no kind resolves it `Unknown` — safe and imprecise, which is
what the vocabulary already says an unassigned filer gets, rather than a bank's
revenue read under an operating filer's meaning. And the trap
`canonical-concepts-open-questions.md` names — the subtotal an investment company
calls net investment income sharing its name with a component of an insurer's
revenue — is held apart by scope rather than by a rule about names.

The registry states no unit, no sign, no measure and no applicability. Each is
the vocabulary's, per concept, and an entry restating one would be the unchecked
duplication the one-source-of-truth invariant bans.

Entries for one concept are a **set, not a list**: their order in the file means
nothing. Where two of them match facts in one filing, choosing between those
facts belongs to the record named at the end of this one. A registry that ranked
its entries would be that record written here, in a file nobody would look in for
it.

**Identity.** A rule's id is the canonical rendering of exactly the fields that
make it that rule — its concept, its kind scope, its form, and its operands in
the order written, with the filer and the period for an assertion. It is derived
from those fields rather than written beside them, so it cannot be mistyped,
cannot be copied onto a second rule, and cannot drift from what it names; the
gate refuses a registry in which two rules render one id. A resolved value
records the pair **(registry version, rule id)** and never the id alone: the same
id under two versions may name different bytes, and the version is what pins
which.

### How it is versioned

The registry's version is the digest of its own bytes — a sha256 over the files
under `registry/`, taken in a fixed order over their repository-relative paths,
so that one tree always digests one way. Every distinct state of the registry is
a distinct version and the digest is its name. A resolved value names the version
that set it by carrying that digest, beside the rule id, the source tag and the
filing the vocabulary already requires of a `Value`.

By content rather than by the `v<N>` sequence the contracts use, and the
difference between the two surfaces is the whole reason. A contract is published
rarely, by a single exclusive task, so a hand-written sequence is affordable and
its consecutiveness catches a version published with no record of it. The
registry changes whenever a filer uses a tag not yet listed and whenever an
override is written — the ordinary traffic of M4, in parallel — so a hand-written
sequence is a shared line two runs collide on and a number a run can get wrong. A
digest is computed rather than chosen, needs no shared file to collide on, and
freezes bytes by construction, which is the property the `versions` record exists
to provide.

**What a change means for values already resolved under an older version:**
nothing happens to them, and that is the point. A stored value keeps the digest
it was resolved under, so it is never silently reinterpreted — it is stale, not
wrong. Re-resolving is a deliberate act. Because both versions are identified
byte-for-byte, and each is a commit in this repository whose `registry/` digests
to it, the difference between the old reading and the new one is inspectable
rather than asserted. The one thing that must not be done is reading a stored
rule id against the current registry: that pair is only meaningful together.

### The per-filer override

An override is written in that filer's own file, `registry/filers/<cik>.toml`,
and binds to the CIK, ten digits left-padded with zeros — the identity the
fetch → normalize contract publishes and keys its retrieval by. Never a ticker: a
ticker is reassigned between companies, and the boundary does not carry one.

It edits the eligible set rather than competing with it. `include` adds a rule for
this filer; `exclude` removes a general rule, named by its id, for this filer;
`assert` states a value. So for the first two there is no precedence contest at
all — the eligible set for a filer and a concept is the general set for that
filer's kind, less what its file excludes, plus what its file includes, and the
result does not depend on the order the two are written in.

`assert` is the one precedence rule, and it is total: where an assertion covers
the filer, the concept and the period, the concept is that `Value` and no lookup
runs. Its rule id is what the resolved value records, so an asserted number is
visibly asserted rather than indistinguishable from a read one. This is the form
`canonical-concepts-open-questions.md` requires when it says that the recovery
for a filer that genuinely has no debt "is already in the constitution and needs
nothing from this record: the registry is a versioned mapping *with per-company
overrides*, and an override asserting that a concept is zero for a filer produces
a `Value` with a rule behind it".

An assertion carries the period it covers, in the shape the fetch contract
publishes — an instant's date, or a duration's two — and applies to no other. It
may not say "every period". The case that most needs an assertion is a true zero,
and a filer with no borrowings this year may borrow next year, so an unbounded
assertion is a machine for producing exactly the number that looks right and is
not. Which filing period a canonical period is built from is the alignment
ruleset's; the assertion only names dates.

An assertion also carries a `source`: the accession of the filing it is read
from, and the line in it. Nothing checks that the citation is true. What the
required field buys is that an assertion with no stated ground cannot be written
at all — anchor 5's rule about the bare uncited literal, applied to the one place
where a number enters normalize with no tag behind it. The gap that remains is
the one the open-questions record already named: nothing distinguishes an
override written from a filer's statements from one written to make a metric
appear.

**Two overrides that could both apply to one filer.** Mostly the shape makes it
unrepresentable: there is one file per filer and it is named for the CIK, so
there is no second file to disagree with, and `include` and `exclude` compose to
one set however they are written. What stays representable is a rule both
included and excluded for one filer, and two assertions for one concept whose
periods overlap. The gate refuses both, so a registry holding either never ships;
reached at run time, the registry fails to load and normalize does not run.
Neither is ever settled by picking one, and neither resolves to `Unknown` — an
ambiguous override is a defect in the data, and `Unknown` would hide it behind a
state that means a filing was consulted and gave nothing.

### The kind half of the registry

The vocabulary's `[elsewhere]` says `kind_assignment = "the same registry"`, so
the registry has a place for it: a `kind` field in the filer's file, asserted like
everything else there and carrying the same identity and the same version. At v1
that half is assertion-only. A filer has the kind its file gives it; a filer with
no file has none. Whether a kind may instead be *derived* from a filer's own
facts is a judgement about accounting shape rather than about tags, and this
record does not decide it. What an unassigned filer gets is already fixed by the
vocabulary and not reopened here: every concept resolved through the attempt, to
`Value` or `Unknown`, and never to a correct absence nobody established.

### The interface

ANCHORS.md names the tag mapping among the components reached only through an
explicit interface, so it has exactly one, owned by `vfi-normalize`: given a
filer, its kind and a concept, it returns the eligible rules each with its id, or
the asserted value, and it names the registry version. Nothing about files,
directories, TOML or overrides crosses it. Adding a second consumer needs an ADR,
which is the anchor's rule and not this record's to relax.

### What reaches each concept at v1

Elements are `us-gaap` unless another taxonomy is given. This is the registry's
starting content, not its finished content, and the two directions cost
differently: a tag left out resolves through to the concept's silence reading, so
an incomplete list costs precision, while a tag wrongly listed is a wrong number
that looks right. Each list below was therefore written against the concept's own
meaning clause in the published vocabulary and against nothing else, and where
the meaning does not settle an element, the element is left out and named under
the exclusions.

Income statement:

- **`revenue`** — every entry kind-scoped. `operating`:
  `RevenueFromContractWithCustomerExcludingAssessedTax`,
  `RevenueFromContractWithCustomerIncludingAssessedTax`, `Revenues`,
  `SalesRevenueNet`, `SalesRevenueGoodsNet`, `SalesRevenueServicesNet`. `bank`:
  `RevenuesNetOfInterestExpense`, and the sum of `InterestIncomeExpenseNet` with
  `NoninterestIncome`, which is the net reading answer 1 fixed. `insurer`:
  `Revenues`. `reit`: `Revenues`, `RealEstateRevenueNet`.
  `investment_company`: `GrossInvestmentIncomeOperating`, the total before the
  expenses of running the portfolio, which is the base its statement subtracts
  them from. That last entry is the least attested in the whole set and the first
  the fixtures will correct.
- **`gross_profit`** — `GrossProfit`; and the difference of `revenue` less one of
  `CostOfRevenue`, `CostOfGoodsAndServicesSold`, `CostOfGoodsSold`,
  `CostOfServices`.
- **`operating_income`** — `OperatingIncomeLoss`.
- **`pretax_income`** —
  `IncomeLossFromContinuingOperationsBeforeIncomeTaxesExtraordinaryItemsNoncontrollingInterest`,
  `IncomeLossFromContinuingOperationsBeforeIncomeTaxesMinorityInterestAndIncomeLossFromEquityMethodInvestments`.
- **`income_tax_expense`** — `IncomeTaxExpenseBenefit`; and the sum of
  `CurrentIncomeTaxExpenseBenefit` with `DeferredIncomeTaxExpenseBenefit`, which
  is the "current and deferred together" the meaning asks for.
- **`net_income`** — `IncomeLossFromContinuingOperations`, the continuing-
  operations figure attributable to the parent; and `NetIncomeLoss`, which is
  that same figure for a filer presenting no discontinued operations to separate.
- **`interest_expense`** — `InterestExpense`, `InterestExpenseDebt`,
  `InterestAndDebtExpense`, `InterestExpenseBorrowings`.
- **`depreciation_and_amortization`** — `DepreciationDepletionAndAmortization`,
  `DepreciationAndAmortization`, `DepreciationAmortizationAndAccretionNet`; and
  the sum of `Depreciation` with `AmortizationOfIntangibleAssets`.
- **`preferred_dividends`** — `PreferredStockDividendsAndOtherAdjustments`,
  `PreferredStockDividendsIncomeStatementImpact`.

Shares and per-share amounts:

- **`earnings_per_share_diluted`** —
  `IncomeLossFromContinuingOperationsPerDilutedShare`, `EarningsPerShareDiluted`,
  the second on the same reading as `NetIncomeLoss` above.
- **`diluted_shares_weighted_average`** —
  `WeightedAverageNumberOfDilutedSharesOutstanding`.
- **`shares_outstanding`** — `dei:EntityCommonStockSharesOutstanding`,
  `us-gaap:CommonStockSharesOutstanding`. The one concept reached across two
  taxonomies, and the reason an entry names the taxonomy rather than the tag
  alone.
- **`dividends_declared_per_share`** — `CommonStockDividendsPerShareDeclared`.
  The sum form answer 6 provides for — regular and special together, where a
  filer reports the special under a distinct element — is available and carries
  no element at v1, because none is attested. The day one is observed it is a
  line in a file, not a record.

Balance sheet:

- **`total_assets`** — `Assets`.
- **`current_assets`** — `AssetsCurrent`.
- **`inventory`** — `InventoryNet`.
- **`cash_and_equivalents`** — `CashAndCashEquivalentsAtCarryingValue`.
- **`short_term_investments`** — `ShortTermInvestments`, `MarketableSecuritiesCurrent`,
  `AvailableForSaleSecuritiesDebtSecuritiesCurrent`, `OtherShortTermInvestments`.
- **`total_liabilities`** — `Liabilities`.
- **`current_liabilities`** — `LiabilitiesCurrent`.
- **`short_term_debt`** — `DebtCurrent`; `LongTermDebtCurrent`,
  `ShortTermBorrowings`, `OtherShortTermBorrowings`, `CommercialPaper`,
  `NotesPayableCurrent`, `FinanceLeaseLiabilityCurrent`; and the sum of those
  components for the filer presenting no total.
- **`long_term_debt`** — `LongTermDebtNoncurrent`,
  `FinanceLeaseLiabilityNoncurrent`; and the sum of the two.
- **`shareholders_equity`** — `StockholdersEquity`, which is the parent's share
  and already excludes the non-controlling interest the meaning removes.
- **`preferred_equity`** — `PreferredStockValue`, `PreferredStockValueOutstanding`.
- **`retained_earnings`** — `RetainedEarningsAccumulatedDeficit`.

Cash flow:

- **`operating_cash_flow`** — `NetCashProvidedByUsedInOperatingActivities`,
  `NetCashProvidedByUsedInOperatingActivitiesContinuingOperations`.
- **`capital_expenditure`** — `PaymentsToAcquirePropertyPlantAndEquipment`,
  `PaymentsToAcquireProductiveAssets`, `PaymentsForCapitalImprovements`,
  `PaymentsToAcquireOtherPropertyPlantAndEquipment`, `PaymentsToDevelopSoftware`;
  and the sum of the property element with the software one.
- **`dividends_paid`** — `PaymentsOfDividends`; and the sum of
  `PaymentsOfDividendsCommonStock` with
  `PaymentsOfDividendsPreferredStockAndPreferenceStock`.

Every one of the twenty-eight is reached by mapping at v1; none is declared
unreachable. The declaration exists in the format all the same, so that the
accounting above is something a gate reads rather than something a reader
remembers.

### The exclusions worth writing down

These are elements a mapping would plausibly reach for and which the vocabulary's
own meanings refuse. Each is left out on purpose, and each is the shape of the
mistake this milestone exists to prevent.

- `LiabilitiesAndStockholdersEquity` is not `total_liabilities`. It is total
  assets under another name.
- `LongTermDebt` is not `long_term_debt`. It is the total including current
  maturities, which `short_term_debt` already carries.
- `StockholdersEquityIncludingPortionAttributableToNoncontrollingInterest` is not
  `shareholders_equity`; the meaning removes that portion.
- `ProfitLoss` is not `net_income`; it includes the non-controlling interest.
- `InterestIncomeExpenseNet` is not `interest_expense`. The meaning says a net
  figure that cannot be decomposed is `Unknown` rather than taken as gross, so it
  is never eligible — and it *is* eligible, as one operand of a sum, for a bank's
  `revenue`, which is what makes writing it down worthwhile.
- `InterestAndDividendIncomeOperating` is not a bank's `revenue`; it is the gross
  interest income answer 1 refused.
- `CommonStockDividendsPerShareCashPaid` is not `dividends_declared_per_share`;
  the concept is declared, not paid.
- `PaymentsOfDividendsMinorityInterest` is not part of `dividends_paid`; the
  non-controlling interest is not a shareholder of the parent.
- `AccountsPayableCurrent`, `AccruedLiabilitiesCurrent`, `Deposits` and
  `OperatingLeaseLiabilityCurrent` are not `short_term_debt`, and
  `OperatingLeaseLiabilityNoncurrent` is not `long_term_debt`. The meanings
  exclude each, and answer 4 settled the lease half.
- `WeightedAverageNumberOfSharesOutstandingBasic` is not
  `diluted_shares_weighted_average`; it is the other denominator.
- `InventoryGross` and the component inventory elements are not `inventory`. The
  first is before reserves; a sum of the second is right only if the filer
  tagged every component, and a partial sum is a number that looks right.
- `CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalents` and its
  variants are not `cash_and_equivalents` at v1. See the consequence below: the
  concept's own two clauses point different ways for a filer with restricted
  cash, and the narrow element is the one that cannot produce a wrong number.
- The temporary-equity elements are not `preferred_equity` at v1, for the same
  kind of reason: none of the attested ones is specific to the preferred claim,
  and they would carry redeemable common and redeemable non-controlling interests
  into a concept the vocabulary defines as the preferred claim.

### What this hands on, and what it does not decide

**Choosing among several candidates in one filing** — consolidated over segment,
and the right period basis — is its own record and nothing here settles any of
it. Because entries are a set with no order, that record inherits every case
where two eligible rules both match facts in one filing and the answer differs:
`net_income` from the continuing-operations element against the tagged total,
`revenue` from a subtotal against a component sum, `gross_profit` from its own tag
against the difference, `income_tax_expense` from the total against current plus
deferred, `earnings_per_share_diluted` continuing against total, `dividends_paid`
from the total against common plus preferred, `capital_expenditure` from one
payments element against a sum. Naming them is not deciding them.

**Amendments, restatements and period alignment** are their own ruleset, which
GOALS.md keeps separate from the mapping in the criterion itself — "separate from
the mapping". This record keeps them separate too: nothing in the registry reads
`form`, `filed` or `accession` to prefer one filing over another. Those fields
cross the boundary for that ruleset to use.

Also not decided here: which metric consumes a concept, and how an inapplicable
or unknown input makes a metric absent, which are M5's; thresholds and defaults,
which anchor 5 puts in the settings layer; and whether a filer's kind may be
derived rather than asserted.

## Alternatives

- **Publish the registry as a contract under `contracts/`.** Consistent — it is
  versioned data with a gate over it, which is what a contract is here. Rejected
  for the four reasons above: one whole file per version makes the diff that adds
  one tag unreadable; the protected path makes every added tag and every override
  need a signature, which stops the registry rather than slowing it; one file per
  version cannot hold one file per filer, so parallel override work collides on
  it; and `contracts/` stops meaning one thing, which costs the contracts gate
  the single shape it reads.
- **Keep the registry inside `crates/normalize/`.** It has one reader, so put it
  with its reader. Rejected: it inverts the shape the layout states for
  `fixtures/`, `benchmarks/` and `scripts/tests/` — data at the root, code with
  the crate — and it buries the versioned surface that every stored value cites
  inside a source tree, where the next agent would not look for data and where
  the gate over it would be reading source.
- **Version by a `v<N>` sequence, one whole file per version, as the contracts
  do.** Human-readable, and consecutiveness is a real check. Rejected: the
  sequence is a single line that two parallel runs both append to, and the copies
  are the dominant artifact once the registry changes at the rate overrides
  imply. The property the sequence exists to give — published bytes never change
  — the digest gives by construction.
- **Let an override replace the general set for a concept instead of editing
  it.** Total and legible: what the filer's file says is what runs for it.
  Rejected: every override then carries a copy of the general list, and the copy
  drifts unnoticed when the general list improves, which is the unchecked
  duplication the one-source-of-truth invariant bans. The cost of the chosen form
  is that a filer's eligible set is not readable in one place; `exclude` naming a
  rule by its id is what keeps it legible.
- **Order the entries for a concept, most preferred first.** The obvious shape,
  and it would let the registry answer a question resolution has to answer
  anyway. Rejected: it is candidate choice under another name, decided inside a
  record that says it decides none of it, in the file a later reader would not
  think to check.
- **Ask the registry tag-first: scan a filer's facts, report which reached a
  concept and which did not.** It would make coverage measurable. Rejected: it
  makes every unconsumed element a case the registry must have an answer for,
  when the vocabulary's whole premise is that almost none of them matters; and
  the useful half — finding tags worth adding — is a development entry point,
  which the layout already homes.

## Consequences

Easier. The first mapping run has a shape to fill rather than one to invent, and
its diff is data. An override costs a file in an unprotected path, so a filer
that needs one costs no signature — unlike the last three tasks on this milestone,
none of which could touch its surface without the `owns` grant and the
human-approved label. The registry cannot drift from the vocabulary, because the
gate reads the published concepts and kinds and refuses an entry naming anything
else; the day a twenty-ninth concept is published, the registry goes red until it
is accounted for. And the format — the part most likely to be wrong — is behind
the interface, so it is the cheapest thing here to change.

Harder. Version numbers are gone: "resolved under registry `a3f9…`" is not a
thing a person says, and comparing two versions means comparing digests and
reading git history. The kind half is assertion-only, so precision scales with a
file written per filer, and until a filer has one its `revenue` is `Unknown` and
every metric consuming revenue is absent — the largest single cost of this record,
taken deliberately, because the alternative is a filer's revenue read under the
wrong kind's meaning. Three concepts turn "nothing matched" into a `Value` rather
than an absence — `short_term_investments` outright, the two dividend concepts on
their condition — so for those three the registry's completeness is what the
vocabulary's priced-in risk is paid in.

Two concepts are deliberately under-mapped at v1, and both cost absences.
`cash_and_equivalents` maps only the element that excludes restricted cash: the
concept's own clauses point both ways, since "cash and demand deposits plus
investments near enough to maturity" excludes restricted cash while "the balance
the cash flow statement reconciles to" has included it since ASU 2016-18. Mapping
the narrow element can produce an absence; mapping the wide one can produce a
wrong number, so this record takes the absence and leaves the wider reading to a
version that fixtures ask for. `preferred_equity` maps only the permanent-equity
elements, so a filer whose redeemable preferred sits in mezzanine needs an
override — which is the override doing precisely the job it exists for.

Expensive to reverse: the version being a digest, once stored values carry one,
since moving to a sequence later leaves every stored value named in the old
scheme; the rule id's rendering, for the same reason, because changing which
fields it renders renames every rule and orphans every id already stored; and
`registry/` as a top-level directory, once the layout names it.

## Enforcement

This changes no anchor. It applies four: "Normalization is data, not code"; the
invariant that a widely-depended-on component is reached only through an explicit
interface; one source of truth per value; and anchor 5's ban on the unsourced
value, applied to the assertion, which is the one place a number enters normalize
with no tag behind it.

The mechanical half is a new gate, `registry`, added to the list in
`scripts/gates.sh` by the task that creates the directory — a new surface with no
gate over it is itself an escalation, so the gate lands with the data. What it
checks:

- every file parses, and carries only the declared fields and the four forms;
- every entry names a concept and kinds the published vocabulary publishes, read
  out of the published surface rather than restated;
- no two rules render one id;
- the concept edges the `difference` form draws are acyclic;
- every published concept is accounted for — at least one entry, or an explicit
  unreachable declaration carrying its reason;
- per filer: no rule both included and excluded, no two assertions for one
  concept whose periods overlap, and every assertion carrying a period and a
  source;
- every filer file named for a ten-digit CIK that matches its own `cik` field.

Each rule gets the proof-of-catch M2 requires: a case that violates it on purpose
and shows the gate failing.

What nothing checks, said plainly.

- **That a listed tag means what the concept means.** Only the golden fixtures
  reach that, and GOALS.md already names the ones that will — the company that
  changed tags mid-history is the case that exercises the override, and the
  filer whose statements do not fit the ordinary shape is the one that exercises
  the kind scope.
- **That an assertion's citation is true.** This is the gap
  `canonical-concepts-open-questions.md` named, unchanged in substance: a required
  `source` field puts something visible in front of it, and nothing behind that.
- **That a filer's asserted kind is its real accounting shape.** Assertion-only
  is assertion-only; a wrong kind is a wrong `revenue` reading and a wrong set of
  `NotApplicable`s, and no gate can see it.
- **That the digest a stored value carries corresponds to a commit anyone can
  find.** Nothing here walks history to check. What makes it findable is that the
  registry's bytes are committed and the digest recomputes from them.

## Decision review

By the decider, not the proposer.
