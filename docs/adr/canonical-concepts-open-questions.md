# The vocabulary's seven open questions close: three concepts join the set, two filer kinds join it, and silence reads as `Unknown` wherever a fabricated zero would flatter

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-08-24, by `M4-04`
- **Decided:** —
- **Touches:** the canonical concept definitions, and through them the
  normalize → analyze contract (anchor 3). It changes what four concepts mean,
  adds three, widens the closed set of filer kinds, and restates five silence
  readings — each of which is above a worker for the same reason
  `docs/adr/canonical-concepts.md` was.

## Context

`docs/adr/canonical-concepts.md` is accepted and cannot publish. Its own
decision says eight questions must close before v1 and that none of them closes
by picking the reading that lets the next task start. Three of the eight change
numbers rather than names. The eighth closed with
`docs/adr/fetch-normalize-contract.md`, which settled that silence is silence in
a fact set and not in a presented statement, and left the five zero readings
that rested on presentation to be restated here. This record closes the other
seven.

They are one record rather than seven because four of them are one question
asked about different concepts — is this one concept with a wide meaning, or two
concepts combined in analysis — and a question with one shape answered in four
places gets four answers. Split across records, preferred claims could be
carried and lease liabilities folded in, and nothing would catch the
disagreement. The same holds for the first and the last: answering what
`revenue` names for a bank makes the filer kind decide a meaning and not only an
applicability clause, which changes what the check on the kind set is checking.

Structural rather than Constitutional, for the reason the parent record gives:
it settles contract fields, the tier's own example. No anchor is edited, no gate
weakened, no protected-path entry changed, no scope in GOALS.md moved. It
applies anchor 3, anchor 5's ban on the unsourced value and its placement of
free parameters in the settings layer, and the one-source-of-truth invariant.

`docs/adr/canonical-concepts.md` is not edited. It is accepted, and an accepted
record is never edited. This closes what it left open the way its sibling closed
the eighth.

## Decision

### Three tests, applied seven times

The answers are not seven independent judgements. Three tests decide them, and
each test is stated here once so that the same question asked about a different
concept cannot get a different answer.

**The split test.** A concept splits into two when each piece is separately
consumed by a metric and each piece separately crosses the boundary. It stays
whole when only the combination is consumed, when the pieces are not separately
reported, or when splitting would relocate a judgement about a filer's shape
into the derivation, where the vocabulary cannot hold it. The bias is toward the
narrower concept, because the parent record prices the two moves and they are
not the same: adding a concept is a version bump, a new file and a new line,
while changing what a concept means is a re-normalization of everything already
stored. But the bias is bounded by the rule that record already states — a
concept no metric consumes does not enter the set — so a split has to be earned
by a consumer, not by symmetry.

**The silence test.** Over presented statements, "the filer presented no such
line" was a claim a reader could make. Over the fact set it is not. What silence
supports there is only that the filer reported facts from this filing and none
of them is under a tag the registry recognises for this concept, and that has
two causes: the filer has none of the thing, or the registry does not recognise
the element the filer used. So: **silence reads as zero only where a
wrongly-read zero cannot flatter the company on any metric that consumes the
concept; everywhere else it reads `Unknown`.** This is not a new rule. It is the
reason the parent record already gives for `capital_expenditure` — "reading it
as zero would overstate free cash flow by the whole of it" — with the test in
that sentence named and applied to the rest. A wrong `Unknown` costs a metric,
which M5 shows as absent with a reason and M7 shows as absence. A wrong zero
that flatters is a number that looks right, which is the failure this milestone
exists to prevent.

**What a kind decides.** A kind names an accounting shape. After the first
answer below it also decides what `revenue` names, so two shapes that agree on
every applicability clause and disagree on that are still two kinds. A kind is
earned by a clause or a meaning being wrong without it, never by an industry
existing.

### 1. What `revenue` names for a `bank` and an `insurer`

**The reading:** `revenue` is the figure the filer's own income statement
presents as the base from which the costs of running the business are
subtracted. One rule, read against each kind:

- `operating` — revenue as the parent record defines it. Unchanged.
- `bank` — net interest income together with non-interest income. A bank's
  statement subtracts interest expense above the operating-expense block, so the
  figure non-interest expense is subtracted from is the net one.
- `insurer` — total revenues as presented: premiums earned net of reinsurance
  ceded, net investment income, and the investment results the statement places
  there.
- `reit` — total revenues as presented.
- `investment_company` — total investment income, before the expenses of running
  the portfolio and therefore before its own interest, because that is the base
  its statement subtracts those expenses from.

Where the filer presents the subtotal, that is the value. Where it presents only
the components, the concept is their sum, by a rule the registry states and the
resolved value records — the way `gross_profit` already reaches revenue less
cost of revenue.

**What the other reading would have changed.** The gross reading for a bank —
total interest income plus non-interest income — differs by total interest
expense, which in a high-rate year is of the same order as net revenue itself.
Net margin falls with it and price/sales falls with it, so the same bank looks
close to twice as cheap on sales in a high-rate year as in a low-rate one, with
nothing about the business changed. Piotroski's ΔTURN turns positive for
essentially every bank in a rising-rate year, which hands out a point for a rate
move; his signals are defined on his sample and a signal that fires on the rate
cycle is not the one he defined. Altman's X₅ is not reached — a bank has no X₁
and no X₃, so the Z-score is not computable for it at all. For an insurer, the
gross reading is premiums before reinsurance ceded: revenue inflates by whatever
the insurer cedes, net margin and price/sales both understate, and two insurers
with the same retained book differ by reinsurance strategy alone.

**Why this one.** The question turns on whether revenue means the gross inflow
or the top line a reader of those statements would call revenue, and the filer's
own statement answers it without anyone preferring an answer. The split
alternative — carry gross interest income and let analysis net
`interest_expense` out of it — is refused for a reason the split test names: the
pieces are not separately consumed, since no M5 metric wants a bank's gross
interest income, and the netting would have to be conditional on the kind of
filer, which would put a judgement about accounting shape inside the
derivation. Applicability by kind
is what the vocabulary exists to hold; a kind switch inside a ratio is a place
where a wrong branch silently changes a number and nothing marks it.

### 2. Whether `net_income` is the bottom line

**The reading:** income before discontinued operations and extraordinary items,
after tax and after the share attributable to non-controlling interests — income
from continuing operations attributable to the parent. One concept, not two.
Extraordinary items ceased to be a GAAP classification for fiscal years
beginning after 2015, so that half of the clause governs only the older part of
a filer's history, which the fact set spans.

This carries `earnings_per_share_diluted` with it: that concept means diluted
earnings per share from continuing operations. Where a filer has discontinued
operations it presents per-share amounts for continuing operations and for the
total, and the first is the one meant; where it has none, the single figure it
presents is that figure.

**What the other reading would have changed.** The two readings are the same
number for every filer with neither discontinued operations nor extraordinary
items, so the change is confined to the filers where it decides something. For a
filer that sold a division at a gain, the bottom-line reading raises
`net_income` by the after-tax gain for one year: net margin, return on assets
and return on equity all rise, payout on earnings falls — the dividend looks
better covered in the year the coverage came from a disposal — retention and
sustainable growth rise, Piotroski's ROA and ΔROA gain points, and his ACCRUAL
signal deteriorates at the same time, because the gain is not in operating cash
flow. One disposal both hands the filer profitability points and takes its
accrual point.

**Why this one.** Two reasons, and the second decides it.

Piotroski defines his signals on income before extraordinary items, and
ANCHORS.md requires his thresholds to cite him. A different numerator silently
changes what the score means while the citation still reads as though it holds.

And the bottom-line reading is not commensurable with what it is divided by.
GAAP presents discontinued operations net of tax on one line below the tax line,
so revenue on the face is already a continuing-operations figure, and so are
`pretax_income` and `income_tax_expense`. Net margin under the bottom-line
reading divides a numerator that includes the discontinued business by a
denominator that excludes it. The tension the question named — that
price/earnings convention runs the other way — does not in fact arise among this
concept's consumers: price/earnings consumes `earnings_per_share_diluted`, not
`net_income`, and that concept moves with this one. So one concept serves every
consumer, and the set does not carry two.

### 3. Whether preferred claims are carried

**The reading:** carried, as two concepts — `preferred_equity` and
`preferred_dividends`, both stated in full below. `shareholders_equity` is not
changed: it remains the parent's shareholders' residual claim, preferred
included, and analysis forms common equity as the difference. That keeps the
combination in analysis, where a metric can choose it, rather than fixing it in
a meaning where nothing can.

**What the other reading would have changed.** Carrying nothing is coherent for
return on equity — numerator and denominator both include the preferred — and
cannot be made coherent for price/book, whose numerator is market capitalisation
and is common-only by construction. For a filer with preferred outstanding,
price/book then divides a common claim by a common-and-preferred book: it
understates, and the filer reads as cheaper than it is. Preferred is issued in
size by banks and insurers, so that error concentrates in two of the five kinds.
Return on equity's error changes sign: the all-holders reading understates the
common return whenever the filer earns more on equity than the preferred rate
and overstates it when it earns less, so the bias flips at exactly the filer in
distress and is not a bias a screen threshold can absorb. Payout on earnings
measures the common dividend against a numerator that has not yet paid the
preferred, understating the payout and flattering coverage.

**Why this one.** M5's metrics are common-shareholder measures, and the set
already says so twice. `shareholders_equity` is defined net of non-controlling
interests, so one non-common claim is already removed; and
`earnings_per_share_diluted` is by construction after preferred dividends, so
the set already carries a per-share figure net of preferred beside returns that
are gross of it. A set that removes the minority's claim but not the preferred's
is not consistent with itself. The split test is satisfied on both counts: each
piece is separately consumed — the equity by the book-value denominators, the
dividend by the earnings numerators — and each is separately reported.

One consequence has to be said rather than left to be discovered.
`dividends_paid` is cash paid to shareholders as dividends from financing
activities, and filers commonly present one line for all classes. Its meaning is
therefore all classes, common and preferred together, and this record states it
so that no reader has to decide. Payout on free cash flow and coverage in their
total form are then all-holder measures, which is the conservative direction for
a question about the common dividend, since they count a claim the common holder
stands behind; the per-share forms, which consume `dividends_declared_per_share`
and `earnings_per_share_diluted`, are already common-specific. Each form is
internally consistent and M5 has both.

### 4. Whether operating lease liabilities are debt

**The reading:** not debt. `short_term_debt` and `long_term_debt` are
interest-bearing borrowings including finance lease obligations — capital
leases, under the previous standard — and excluding operating lease liabilities.
No concept is added.

**What the other reading would have changed.** Including the operating lease
liability raises `long_term_debt` for lessee-heavy filers — retailers,
restaurants, airlines — often by more than their borrowings. Debt/equity rises,
net debt and enterprise value rise, EV/EBIT and EV/EBITDA rise, the discounted
cash flow's equity value per share falls, and Piotroski's ΔLEVER records an
increase in leverage for every such filer in its adoption year, when nothing
about the business changed.

**Why this one.** The decisive reason is that the flows are not restated to
match the stock. Since ASC 842 an operating lease payment runs through operating
cash flow, so free cash flow is already after it, and the single lease cost sits
inside operating expenses, so operating income and EBITDA are already after it.
Capitalising the liability while the flows stand as presented counts the lease
twice in every metric that pairs a stock against a flow — enterprise value
against EBIT, the discounted cash flow's bridge against free cash flow. The set
carries no operating lease expense and no lease payment, so the restatement that
would make the inclusion coherent is not available. That is also why this
question does not become a setting in M5: a free parameter is only free if both
of its settings produce a coherent number, and one of these does not.

The line the answer draws is the line the filer's own cash flow statement draws.
Finance lease principal sits in financing with the borrowings; operating lease
payments sit in operations with the rent. It is also the line that survives the
2019 boundary, since a finance lease was capitalised under the previous standard
too — so the series stays comparable across adoption without that comparability
being bought by ignoring the obligation.

And the obligation is not ignored. It sits inside `total_liabilities`, so
liabilities to assets and Altman's X₄ see it. What this answer decides is which
ratios see it, not whether the tool does.

### 5. Whether short-term investments join `cash_and_equivalents`

**The reading:** they do not. `cash_and_equivalents` keeps its narrow meaning
and its anchor — the balance the cash flow statement reconciles to — and
`short_term_investments` joins the set as its own concept, stated in full below.
Net debt is formed in analysis from both.

**What the other reading would have changed.** The wide reading raises
`cash_and_equivalents` by the securities portfolio, which lowers net debt and
enterprise value and raises the discounted cash flow's equity value per share
for every filer that holds its liquidity in marketable securities. It also
raises the cash ratio, which for such a filer can be a multiple of the narrow
figure — and the cash ratio exists to be the strictest of the three liquidity
ratios, so a cash ratio computed on cash plus securities is the quick ratio
under another name.

**Why this one.** The wide reading breaks the concept's only definitional check.
`cash_and_equivalents` is defined as the balance the cash flow statement
reconciles to, and no wider figure reconciles to anything; a concept that can be
checked against a statement is worth keeping checkable. The split test is
satisfied plainly: the cash ratio consumes the narrow figure and net debt
consumes both, so each piece has a consumer that wants it and neither wants the
other's. And the split cannot be a regression, because the set today nets cash
only, which is arithmetically identical to carrying the concept and reading
every filer's securities as zero — which is exactly the silence reading below.
Carrying it is never worse than the status quo and is better wherever the
securities resolve.

### 6. Whether a special dividend belongs in `dividends_declared_per_share`

**The reading:** the concept is the total — dividends declared per common share
for the period, regular and special together. It does not split.

**What the other reading would have changed.** A regular-only concept would give
the growth rate, the streak and the dividend discount valuation a series a
one-off cannot disturb, and would leave yield and payout to be computed from
regular plus special. Under the total, a special lifts one period's figure and
falls out of the next, which reads as a cut and breaks a streak that never
broke; and a dividend discount model fed a special-dividend year values a
one-off as though it repeated forever, which overstates value — the flattering
direction.

**Why this one, in spite of that.** The split is not refused because it is
undesirable. It is refused because its input does not cross the boundary. The
fetch → normalize contract carries no axis and no member; a dimensional
breakdown is not published by the document it reads at all. Where a filer marks
a special dividend by a member on the same element, the distinction does not
reach normalize, and the undimensioned total is what crosses. A concept that
cannot be resolved is not a concept, and inferring "special" from the size of a
period's figure would be the guess this milestone exists to prevent. Where a
filer instead reports a special dividend under a distinct element, both cross,
and the concept is their sum by a rule the registry states — the same
arrangement `gross_profit` already has.

Two things follow, and both are recorded rather than left implicit. The exposure
is real and lands on M5: the streak and the growth rate must be defined so that
a single-period excess which does not recur is not read as a cut, and the
dividend discount valuation must not take one period's declared total as a
perpetuity — its input is a series, which storage holds. And what would change
this answer is a version of the fetch → normalize contract that carries the axis
and the member. The split becomes resolvable the day that lands, and it is a
version bump here.

### 7. Whether three filer kinds are enough

This one is a check, and is done as one. The five applicability clauses the
parent record states are all `operating`-only: `gross_profit`,
`operating_income`, `current_assets`, `current_liabilities`, `inventory`. Read
against the statements the three named shapes present, a clause **holds** where
typing the filer `operating` gives the right answer, and is **wrong** where the
concept does not exist for the shape, so that admitting it would produce
`Unknown` — a mapping that failed — where `NotApplicable` is the truth.

| clause | REIT | regulated utility | investment company |
| --- | --- | --- | --- |
| `gross_profit` | wrong | holds | wrong |
| `operating_income` | holds | holds | wrong |
| `current_assets` | wrong | holds | wrong |
| `current_liabilities` | wrong | holds | wrong |
| `inventory` | wrong | holds | wrong |

**A REIT.** Its balance sheet is unclassified: real estate carried at cost less
accumulated depreciation, then cash, receivables and other assets, with no
current subtotal on either side, so `current_assets` and `current_liabilities`
are wrong for it. It presents rental and related revenue against property
operating expenses and depreciation and no cost of revenue, so `gross_profit` is
wrong. The property it holds for investment is not goods held for sale, so
`inventory` is wrong. `operating_income` holds: the shape runs revenues through
property costs to a profit before interest, and where a REIT folds interest into
total expenses and presents no such subtotal the concept resolves `Unknown`,
which is the correct state for a line that exists in the shape and was not
found. Four clauses wrong: the kind set is short.

**A regulated utility.** Every clause holds. The utility format leads with plant
rather than with current assets, but it subtotals current assets and current
liabilities all the same. It presents operating revenues less operating expenses
as operating income. It carries fuel, materials and supplies as inventory. It
presents no gross profit subtotal, but fuel and purchased power are a cost of
revenue it could present and a reader would understand, so the concept exists
and a utility that does not subtotal it resolves `Unknown` — again the correct
state. No clause is wrong, so no kind is added. Ordering is not shape, and a
kind added here would be a kind added because an industry exists.

**An investment company.** All five are wrong. It presents a statement of assets
and liabilities, unclassified, so both current clauses fail. Its investment
income has no cost of revenue, so `gross_profit` fails. Its portfolio is carried
at fair value and is not goods held for sale. And the expenses of running it
include its own interest, so the subtotal it presents is after interest and a
profit measured before interest is not a figure it presents or a reader of it
would use — the reason `bank` already has for the same clause.

**Verdict: the kind set is short by two.** `reit` and `investment_company` join
it; the regulated utility is an `operating` filer. Their definitions are below.

The investment company earns its kind twice over, and the second time is why
these questions had to be one record. Its clause profile is identical to
`bank`'s — every one of the five is `operating`-only, so both get
`NotApplicable` on all five — and the clauses alone would not have forced a
separate kind. But the first answer above makes a kind decide what `revenue`
names, and the two name different things: a bank's revenue is net of its
interest expense and an investment company's is gross of it, because that is
where each statement puts the subtraction. Answered in a separate record, this
check would have read the five clauses, found them satisfied by `bank`, and
typed a fund as a depository lender.

Two things bound this check. It is over the three shapes the parent record
names, and other shapes exist. That is safe rather than lucky: that record
already fixes that `NotApplicable` is constructible only from a kind and a
clause, so a filer whose shape fits no kind is assigned none, resolves every
concept through the attempt, and reaches `Unknown` — never a fabricated correct
absence. A short kind set is a loss of precision and never a wrong answer, which
is what makes re-running this check later cheap. And there is no default kind. A
filer that does not fit is unassigned, not `operating`; a default would produce
exactly the four wrong clauses this check just found for REITs.

### The five silence readings, restated over the fact set

The parent record stated five zero readings over the filing's presented
statements. `docs/adr/fetch-normalize-contract.md` removed the ground under
them: what normalize is handed is everything the filer tagged, not everything it
presented. Under the silence test above, they read:

- **`inventory` — `Unknown`.** A zero raises the quick ratio to the current
  ratio, and the quick ratio exists to be the conservative form, so the
  fabricated zero flatters precisely the filer whose inventory it should have
  excluded. What the reading costs: an `operating` filer that genuinely holds
  none loses its quick ratio. Its current and cash ratios still resolve, so
  liquidity is not lost, only its strictest form.
- **`short_term_debt` — `Unknown`.** A zero lowers total debt, lowers
  debt/equity, lowers net debt and enterprise value, and raises the discounted
  cash flow's equity value per share. Every direction is toward a company that
  looks safer and cheaper. The concept is also assembled from captions a filer
  chooses freely — current maturities, notes payable, commercial paper, a
  revolving facility — so its failure is as often partial as total, and a zero
  is not even the right shape for a partial resolution. This is the reason the
  parent record already gave for `capital_expenditure`.
- **`long_term_debt` — `Unknown`.** The same, and it carries Piotroski's ΔLEVER
  with it.
- **`dividends_declared_per_share` — zero,** where the period's facts are silent
  on both this concept and `dividends_paid`; `Unknown` where the other member
  resolves to a non-zero value.
- **`dividends_paid` — zero,** on the same condition, with the members
  exchanged.

**What supports the two zeroes,** since the fact set alone does not — silence in
either one of them is as ambiguous as silence anywhere else. What supports them
is that they are a pair, and that a distribution which happened is reported
twice: as an amount per common share, and as a financing cash outflow. So a
wrongly-read zero requires both members to fail at once, which is a narrower
failure than either alone; and the case where one fails is not merely survivable
but visible — a filer that distributed cash and a filer that did not become
distinguishable, and the silent member is then a mapping that failed rather than
a period with no dividend.

This is also the reading the golden fixture needs. A dividend suspension is a
period in which the filer declared and paid nothing, so both members are silent,
both read zero, and the streak breaks where it should. Under `Unknown` a
suspension and a mapping failure would be one result and the streak could not be
measured at all, which is the fixture GOALS.md names.

What the zero costs when it is wrong: a filer that pays, and for which neither
member resolves, reads as a filer that pays nothing. That excludes it from every
dividend screen — an omission, not a wrong recommendation. The one exposure
running the other way is payout on earnings, which would read zero and so read
as perfectly covered, and a composite that rewards a low payout would rank such
a filer well; the pair condition is what holds that case to a double failure.

**The zero that is not a reading.** A filer that genuinely has no short-term
debt, no long-term debt or no inventory now resolves `Unknown` and loses the
metrics that consume it. The recovery is already in the constitution and needs
nothing from this record: the registry is a versioned mapping *with per-company
overrides*, and an override asserting that a concept is zero for a filer
produces a `Value` with a rule behind it, recorded like any other resolved
value. The zero is not banned. It stops being the default reading of silence and
becomes an assertion someone made and signed, which is the difference between a
fabricated number and a sourced one.

### The three concepts that join the set

Stated the way the parent record states one: meaning, flow or balance, unit and
sign, the kinds it applies to, its silence reading, and the M5 metric that
consumes it.

- **`preferred_equity`** — the carrying amount of the preferred claim
  outstanding at the period-end date: preferred stock as the equity section
  presents it, together with redeemable preferred a filer presents outside
  permanent equity, since both rank ahead of the common shareholder and both
  must leave a common-shareholder denominator. A **balance**, in the reporting
  currency at a scale of one, positive as the balance sheet presents it.
  *Applies to:* all kinds. *Silence:* `Unknown` — a zero overstates common book
  value and understates price/book, so the filer reads as cheaper than it is.
  *Consumed by:* the common-shareholder forms of return on equity, price/book
  and sustainable growth, each of which forms common equity as
  `shareholders_equity` less this.
- **`preferred_dividends`** — the deduction between the period's profit and the
  profit available to common shareholders: preferred dividends declared for the
  period together with the accretion and redemption adjustments the filer
  charges in the same reconciliation, because what the metrics need is the whole
  of what leaves the common numerator and a narrower reading would leave part of
  it behind. A **flow**, in the reporting currency at a scale of one, as a
  magnitude — positive when it reduces income available to common, negative in
  the case where the filer's own reconciliation adds to it, which a redemption
  below carrying amount produces. *Applies to:* all kinds. *Silence:* `Unknown`
  — a zero overstates income available to common. *Consumed by:* the
  common-shareholder forms of return on equity, dividend payout on earnings,
  dividend coverage, and retention and sustainable growth.
- **`short_term_investments`** — investments in marketable securities held at
  the period-end date that the balance sheet presents outside cash and
  equivalents and within current assets: the liquid portfolio a filer holds
  instead of cash. Excludes cash and equivalents, which `cash_and_equivalents`
  carries, and excludes holdings the filer classifies as non-current. A
  **balance**, in the reporting currency at a scale of one, positive as
  presented. *Applies to:* `operating` — the concept is defined by the
  current/non-current split, which a `bank`, an `insurer` and an
  `investment_company` do not assert and a `reit` does not present, for the
  reason `current_assets` already gives. *Silence:* zero — the one concept whose
  wrongly-read zero cannot flatter, since it only ever raises net debt and
  lowers the equity value the discounted cash flow arrives at, and since the
  `Unknown` reading would delete enterprise value for every filer holding no
  securities, which is most of them, in order to protect the filers where the
  zero is conservative anyway. *Consumed by:* net debt in enterprise value, and
  through it EV/EBIT and EV/EBITDA; the equity bridge in the discounted cash
  flow.

### The two kinds that join the closed set

- **`reit`** — a real estate investment trust earning rental and related income
  from property it owns, presenting an unclassified balance sheet led by real
  estate carried at cost less accumulated depreciation, and an income statement
  running revenues → property operating expenses and depreciation → a profit
  before interest.
- **`investment_company`** — a filer whose business is holding a portfolio of
  investments carried at fair value, presenting a statement of assets and
  liabilities rather than a classified balance sheet, and a statement of
  operations running investment income → the expenses of running the portfolio,
  its own interest among them → net investment income, with realised and
  unrealised results below.

### What changes for concepts already in the set

Each is named with the change and the consuming metrics that move with it.

- **`revenue`** — the reading is fixed for every kind by the rule in answer 1.
  Movers, for a `bank` and an `insurer` only: net margin, price/sales,
  Piotroski's ΔTURN. Altman's X₅ is unreached for both kinds.
- **`net_income`** — income before discontinued operations and extraordinary
  items, attributable to the parent. Movers, for filers reporting either only:
  net margin, return on assets, return on equity, dividend payout on earnings,
  retention and sustainable growth, Piotroski's ROA, ΔROA and ACCRUAL.
- **`earnings_per_share_diluted`** — from continuing operations, carried by the
  answer above. Movers, for the same filers: price/earnings, dividend coverage
  and payout in their per-share form, and the earnings growth and stability
  tests in the established screens.
- **`short_term_debt`** and **`long_term_debt`** — including finance lease
  obligations, excluding operating lease liabilities. Movers, for lessees only:
  debt/equity, net debt in enterprise value and in the discounted cash flow
  bridge, and Piotroski's ΔLEVER for the long-term concept.
- **`dividends_declared_per_share`** — regular and special dividends together.
  Movers, for filers that declare a special only: yield and relative yield, the
  dividend growth rate and streak, the dividend discount valuation, and payout
  and coverage in their per-share form.
- **`dividends_paid`** — all classes of shareholder, common and preferred.
  Movers, for filers with preferred only, and in the conservative direction:
  payout on free cash flow and on earnings in their total form, and dividend
  coverage.
- **`inventory`**, **`short_term_debt`**, **`long_term_debt`** — silence reads
  `Unknown`. Movers: the quick ratio for the first; debt/equity, net debt and
  the discounted cash flow bridge for the other two — each absent with a reason
  where it previously carried a zero.
- **`operating_income`** — its applicability clause admits `reit` alongside
  `operating`. Nothing moves: a REIT typed `operating` was already admitted to
  it, and this keeps it admitted once the kind exists.
- **`cash_and_equivalents`** — unchanged. Answer 5 removes the open clause from
  its definition without changing the meaning it already had.

The set is twenty-eight concepts across five kinds, with six applicability
clauses: `gross_profit`, `current_assets`, `current_liabilities`, `inventory`
and `short_term_investments` for `operating` alone, and `operating_income` for
`operating` and `reit`.

### What this does not decide

Which tags resolve to which concept, how a filer is assigned a kind, and every
per-company override, are the registry's — including the trap that the subtotal
an investment company calls net investment income shares its name with a
component of an insurer's revenue, so that entry must be scoped by kind.
Choosing among candidates in one filing, and reconciling amendments,
restatements and period alignment, remain their own rulesets. M5 keeps its own:
how an inapplicable or unknown input makes a metric absent, and the settings and
defaults every free parameter arrives through.

Three things are handed to M5 by name, because this record's answers create
them. The dividend streak and growth rate must tolerate a one-period excess that
does not recur, and the dividend discount valuation must not take one period's
declared total as a perpetuity. `NotApplicable` now appears for the first time
as one term of a sum rather than as a whole numerator — `short_term_investments`
inside net debt for the four non-`operating` kinds — and the two absences settle
how it is treated without anyone choosing: a claim that the thing does not exist
for this filer contributes nothing, where an attempt that found nothing must
make the sum absent. And for an `investment_company`, `net_income` keeps its one
meaning and so contains realised and unrealised portfolio results, which makes a
fund's payout and coverage read best in a rising market and worst in a falling
one; a metric that needs a fund's distributable earnings would need net
investment income as its own concept, which no M5 metric names today and which
is a version bump on the day one does.

## Alternatives

- **Close each question in its own record.** Smaller diffs, each reviewable
  alone. Rejected: four of the seven are one question about different concepts,
  and the seventh reads the answer to the first. Separately, preferred claims
  could be carried while lease liabilities were folded in, and the kind check
  would have typed an investment company as a `bank` because their clause
  profiles agree — the disagreement is only visible when the answers are in one
  place.
- **Always split, wherever two readings exist.** Consistent, and it puts every
  choice in the settings layer where anchor 5 wants free parameters. Rejected:
  answer 4 splits into a concept whose inclusion cannot be made coherent for the
  metrics that would consume it, and answer 6 splits into a concept the boundary
  cannot resolve. A setting is only free if both of its settings produce a
  number; a concept only exists if it can be resolved and consumed.
- **Never split, and fix each meaning at the wide reading.** Twenty-five
  concepts stay twenty-five. Rejected: a meaning is the expensive thing to
  reverse — the parent record prices it as a re-normalization of everything
  stored, against a version bump for an added concept — so the wide reading
  spends the expensive currency to save the cheap one, and it fixes inside
  normalize a choice that answer 5 shows M5 wants both sides of.
- **Read every silence as `Unknown`, with no exception.** The purest reading of
  the fetch record's "the safe direction is `Unknown`", and it never fabricates.
  Rejected: a dividend suspension and a mapping that failed become one result,
  the streak cannot be measured, and GOALS.md names that fixture. The pair is
  what makes the exception something other than a convenience.
- **Restore the five zero readings as the parent record wrote them.** They were
  argued and accepted. Rejected: they were argued over presented statements, and
  the fetch record removed that ground. Three of them flatter when wrong, and
  two of those flatter in the direction of a company that looks safer and
  cheaper than it is.
- **Add a kind per industry — utility, REIT, fund, homebuilder,
  broker-dealer.** Complete, and no future check needed. Rejected: a kind is
  earned by a clause or a meaning being wrong without it, and the utility is the
  case where the answer is no. Every kind is a place where a wrong applicability
  claim hides a real absence behind a correct-looking one, so the set is widened
  only where the check forces it.
- **Leave the questions to M5 as settings, and publish v1 with the readings
  open.** It would unblock publication now. Rejected: it is the move the parent
  record's decision forbids by name, and a meaning left open is not a setting —
  it is a value each consumer picks for itself, which is the unchecked
  duplication the one-source-of-truth invariant bans.

## Consequences

Easier: v1 can be published. The vocabulary has no open question left, and every
field the parent record says v1 states now has one reading. The registry task
gains three more targets and two more kinds but loses the ambiguity that would
have made its tag choices unreviewable — a tag for `revenue` is right or wrong
against a stated rule rather than against a reader's sense of what a bank's
revenue is. M5 gains the common-shareholder forms of its equity metrics, and the
lease and cash questions arrive already settled rather than as two more places
where a metric could be defined twice.

Harder: three of the five zero readings become `Unknown`, so a filer that is
genuinely debt-free or genuinely holds no inventory loses metrics it would
previously have been given. That is the intended trade — an absence with a
reason instead of a number that looks right — but it lands on M6's default
composite, which must now cope with more absent inputs than banks alone
produced, and on the per-company overrides, which become the only way to assert
a true zero. The kind set at five means the registry must assign more shapes,
and an unassigned filer resolves everything through the attempt, which is safe
and imprecise. And the set at twenty-eight is still a floor: a twenty-ninth
concept is an ADR and a version bump, by the anchor that made this record one.

Expensive to reverse: the same thing the parent record names, now with four more
meanings inside it. `net_income` and `revenue` in particular are
re-normalizations if they move, and nothing in a stored value distinguishes the
old meaning from the new. `earnings_per_share_diluted` moving with `net_income`
is the shape to watch — the two are commensurable only while both are read from
continuing operations, and a later change to one that forgets the other would be
invisible in every ratio that uses them separately.

For the task that publishes v1: it transcribes this record and the parent
together, and where they differ this one is the later reading. `contracts/` is
protected and grantable, so that task must list `contracts/canonical-concepts/`
under `owns` in its task file as committed on `origin/main`, and the merge still
needs the human-approved label. The silence readings for the two dividend
concepts are conditional and must be published as stated here, not shortened to
the single word "zero" — the condition is what supports the zero.

## Enforcement

This changes no anchor; it applies three. The mechanical half is the one the
parent record already names: once `versions` names v1, that version's bytes are
frozen and an edit to the published surface goes red with no change to the gate.

What nothing checks, said plainly. None is a gap in this diff, which adds no
code.

- **The pair condition behind the two dividend zeroes.** It is the whole of what
  supports them, and until a resolver exists nothing enforces that a filer whose
  `dividends_paid` resolves to a non-zero value cannot take a zero for
  `dividends_declared_per_share`. The shape of the check is a fixture pair — a
  suspension, whose expected result is a zero for both, and a filing with the
  declared-per-share element removed, whose expected result is `Unknown` for
  that concept and the unchanged value for the other. It belongs to the task
  that builds the resolver, and GOALS.md already names the first of the two.
- **That a fabricated zero cannot enter through an override.** The per-company
  override is what makes a true zero assertable, and it is also the one path by
  which an unsupported zero could re-enter wearing a rule. Nothing distinguishes
  an override written from a filer's statements from one written to make a
  metric appear.
- **That the silence test is applied to the twenty-ninth concept.** It is stated
  here and read by whoever proposes the next concept; nothing re-checks it.
- **That every concept still has a consumer.** Unchanged from the parent record,
  and now with three more concepts inside it, one of which —
  `short_term_investments` — is consumed only through net debt. If M5 ever drops
  enterprise value and the discounted cash flow, that concept has no consumer
  and nothing goes red.

## Decision review

By the decider, not the proposer.
