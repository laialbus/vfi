# Capital employed is total assets less current liabilities, invested capital is shareholders' equity plus net debt, and net operating profit after tax applies the effective rate only where that rate is a share of a profit; each is formed once

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-15, by `M5-31`
- **Decided:** 2026-10-16, by the decider
- **Touches:** the three terms `canonical-concepts.md` names only in part for
  return on capital employed and return on invested capital: capital
  employed, invested capital, and the effective tax rate that turns operating
  income into NOPAT. It fixes the names and meanings of two metrics that cross
  `analyze-store` v1 as characters, and no byte of that contract changes. It
  forms net debt and total debt once, as parts of invested capital, for every
  later metric that reads them. It names five concepts read by a metric the
  catalogue does not list them for: `total_assets` for return on capital
  employed, and `short_term_debt`, `long_term_debt`, `cash_and_equivalents`
  and `short_term_investments` for return on invested capital. It edits
  neither catalogue record. It applies the accepted `declined` reason to four
  new conditions, `zero_capital_employed`, `zero_invested_capital`,
  `non_positive_pretax_income` and `tax_rate_outside_zero_to_one`, and adds no
  reason. It names two claims v2 cannot carry, non-controlling interests and
  redeemable preferred outside permanent equity, and leaves them open for a
  record of their own. No accepted record is superseded. No contract's bytes,
  no schema, no edge, no gate, no dependency and no entry field move. Nothing
  here is implemented.

## Context

GOALS.md's fourth M5 criterion names returns on capital. The accepted period
record fixes how all five read their balances. Return on assets and asset
turnover (M5-29) and return on equity (M5-30) are queued. Return on capital
employed and return on invested capital are held, because their denominators
are terms no record forms. If each metric task formed its own terms, two tasks
would each make method choices that enterprise value and the discounted cash
flow will read again. Anchor 5 and the one-source-of-truth invariant forbid
that.

Some things are already fixed:

- **What the catalogue names.** `canonical-concepts.md` (Accepted) lists, by
  consumer:
  - `operating_income` for return on capital employed and on invested capital
  - `current_liabilities` as "the capital employed denominator of return on
    capital employed", naming no minuend
  - `shareholders_equity` as "invested capital for return on invested
    capital", naming no other component
  - `pretax_income` and `income_tax_expense` for "the effective tax rate that
    turns operating income into NOPAT for return on invested capital"

  It leaves "the effective tax rate" to analysis by name, as a derived value.
- **What v2's concepts mean.**
  - `operating_income` is before interest and tax, and excludes interest and
    investment income.
  - `short_term_debt` and `long_term_debt` are interest-bearing borrowings.
    They include finance lease obligations and exclude operating lease
    liabilities. Both read silence as `Unknown`.
  - `cash_and_equivalents` is the narrow balance the cash flow statement
    reconciles to. `short_term_investments` is the marketable securities held
    outside it and within current assets, and reads silence as zero.
  - `shareholders_equity` is the parent's claim, with the preferred claim
    included and non-controlling interests excluded.
  - `income_tax_expense` is negative when it is a benefit.
- **Applicability.** `operating_income` applies to `operating` and `reit`.
  `current_liabilities` and `short_term_investments` apply to `operating`
  alone. Every other concept here applies to every kind.
- **Net debt.** `canonical-concepts-open-questions.md` (Accepted) says "net
  debt is formed in analysis" from `cash_and_equivalents` and
  `short_term_investments` beside the debt. Inside net debt,
  `short_term_investments` is the one term a `NotApplicable` lets contribute
  nothing, for the four kinds that are not `operating`.
- **Absence is closed.** The accepted analyze record allows six reasons. A
  metric built on another carries the other's reasons.
- **The balance reading.** `which-other-period-a-metric-reads.md` (Accepted)
  reads both metrics' balances as averaged, citing the CFA text. Reads run in
  the entry's input order. An averaged balance is two reads, opening then
  closing.
- **Total debt is formed today, inline.** `debt_to_equity.rs` writes
  `short_term_debt` plus `long_term_debt` itself.

## Decision

### The terms, each formed once

The terms are one module of the derivation, `crates/analyze/src/metrics/capital.rs`.
Every metric that reads one of them reads it through that module. No metric
writes any part of one itself.

**Capital employed** is `total_assets` less `current_liabilities`, at one
instant.

- It is the long-term funding of the business: total equity, non-controlling
  interests included, and every non-current liability.
- Its numerator, `operating_income`, is the whole consolidated operating
  profit, before interest, tax and the non-controlling share. Both sides are
  consolidated, so they match.
- The source: Atrill, P., and McLaney, E., *Accounting and Finance for
  Non-Specialists*, 9th ed., Pearson, 2015, the chapter "Analysing and
  interpreting financial statements". It states return on capital employed
  as operating profit over share capital, reserves and non-current
  liabilities. That sum is total assets less current liabilities.
- The catalogue already names `current_liabilities` as this term's
  subtrahend, and this form is the only one that reads it.
- The inputs are read in this order: `total_assets`, then
  `current_liabilities`.

**Total debt** is `short_term_debt` plus `long_term_debt`, at one instant.
This is the sum `debt_to_equity` already writes. The task that builds the
module moves `debt_to_equity` onto it. That is a refactor with identical
outcomes, and it needs no bump.

**Net debt** is total debt less `cash_and_equivalents` and less
`short_term_investments`, at one instant.

- This record does not decide it. It carries the open-questions record's
  statement and that record's one exception.
- Enterprise value and the discounted cash flow bridge are its other named
  consumers. They read it here when they are built. Whether enterprise value
  adds the preferred claim or non-controlling interests beside it is that
  metric's own record.

**Invested capital** is `shareholders_equity` plus net debt, at one instant.

- **Debt is a component.** Both concepts are, through total debt.
- **Cash is netted.** Both `cash_and_equivalents` and `short_term_investments`
  are netted. `operating_income` excludes interest and investment income by
  v2's meaning, so the assets that earn that income are left out of the
  capital the income is set against.
- **The preferred claim is inside**, as `shareholders_equity` carries it.
  NOPAT is before every financing claim, the preferred one included, so the
  capital is too. `preferred_equity` is not read. So no `common` term is
  formed, and the common-shareholder record's naming test holds.
- **Operating leases are outside on both sides.** v2's debt excludes them,
  and `operating_income` is after the full operating lease cost. The
  open-questions record's fourth answer fixes that pairing, and this term
  keeps it.
- **The source:** Damodaran, A., "Return on Capital (ROC), Return on Invested
  Capital (ROIC) and Return on Equity (ROE): Measurement and Implications",
  Stern School of Business, New York University, 2007. Damodaran states
  invested capital as the book value of debt plus the book value of equity,
  less cash and marketable securities.
- The inputs are read in this order: `shareholders_equity`,
  `short_term_debt`, `long_term_debt`, `cash_and_equivalents`,
  `short_term_investments`.

**The effective tax rate** is `income_tax_expense` over `pretax_income`, over
one duration.

- This record does not choose the effective rate. The catalogue names "the
  effective tax rate" from these two concepts, and this record states its
  edges.
- The sources:
  - Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
    *International Financial Statement Analysis*, 3rd ed., CFA Institute
    Investment Series, Wiley, 2015, the chapter "Income Taxes". It states
    the effective tax rate as income tax expense over pretax income.
  - Financial Accounting Standards Board, ASC 740-10-50-12, and Securities
    and Exchange Commission, Regulation S-X, Rule 4-08(h). Each filer
    reconciles its tax charge to the statutory rate on pretax income. So the
    rate is the filer's own two figures, and no third number enters.
- The inputs are read in this order: `income_tax_expense`, then
  `pretax_income`.

**Net operating profit after tax** is `operating_income` times one less the
effective tax rate, over one duration.

- The source: Damodaran 2007, which states the numerator of return on
  capital as EBIT times one less the tax rate. v2's `operating_income` is
  EBIT by its meaning.
- The inputs are read in this order: `operating_income`, then the rate's.

### Where a term reads a concept the catalogue does not list for it

Five do:

| Concept | Read by | The catalogue lists it for |
| :--- | :--- | :--- |
| `total_assets` | capital employed | return on assets, asset turnover, liabilities to assets, Altman, Piotroski |
| `short_term_debt` | invested capital, through net debt | debt/equity, net debt in enterprise value and the bridge |
| `long_term_debt` | the same | the same, and Piotroski's ΔLEVER |
| `cash_and_equivalents` | the same | the cash ratio, net debt, the bridge |
| `short_term_investments` | the same | net debt, the bridge |

**This is within this record's authority.** It does not need either catalogue
record amended.

- The catalogue's lists are the derivation that earns each concept its place
  in the set. `canonical-concepts.md` puts it this way: "a concept no metric
  consumes is not in this list". The lists say why a concept is in the set,
  not who may read it.
- v2 carries no consumer list. Its `[elsewhere]` line leaves that to "the
  derivation, in the two records this file transcribes".
- A new consumer changes no meaning, no applicability, no silence reading and
  no byte. It removes no concept's reason to be in the set.
- It does not reach a concept admitted narrowly. The open-questions record
  admitted `preferred_equity` and `preferred_dividends` for named forms, and
  the common-shareholder record makes another consumer of those "its own
  record". This record reads neither.

**What a new consumer owes** is the check the catalogue's lists exist for:
that the concept's meaning is the one the term wants. For each:

- **`total_assets`**: everything the company controls, as the balance sheet
  totals it. That is the minuend the source names.
- **The two debts**: interest-bearing borrowings, a claim financing the
  business. Leases are handled as the invested capital bullet above says.
- **`cash_and_equivalents` and `short_term_investments`**: the assets whose
  income `operating_income` excludes.

Neither accepted record is edited or superseded, so the ADR README's table
gains no row. A reader of the catalogue will not find these five consumers
there, and this record is where they are named. Whether the README should
list it anyway is the decider's call. This task owns this file alone.

### What v2 cannot carry

Two claims belong in invested capital on the sources' own terms, and v2 has
no concept for either:

- **Non-controlling interests.** `operating_income` includes the
  non-controlling share of a subsidiary's profit. `shareholders_equity`
  excludes the capital that earned it. Koller, Goedhart and Wessels count
  non-controlling interests among the funds invested (*Valuation: Measuring
  and Managing the Value of Companies*, 7th ed., McKinsey & Company, Wiley,
  2020).
- **Redeemable preferred outside permanent equity.** It is a financing claim.
  v2's `shareholders_equity` excludes it, and `preferred_equity` includes it
  only together with the permanent preferred that `shareholders_equity`
  already holds.

Without them, invested capital is understated by each, and return on invested
capital is overstated. This is the flattering direction. The term is formed
without them, and the gap is named, not hidden. A concept carrying either,
such as `noncontrolling_interest`, is a `canonical-concepts` v3 and is
proposed only in a record of its own.

Capital employed has neither gap. Total assets less current liabilities holds
both claims by construction.

### The effective tax rate at its edges

Each edge is decided here, once, beside the rate. None is left to a metric's
file. Two `declined` conditions are defined beside the rate. They are asked
only when every read gave a number, and in this order:

- **`non_positive_pretax_income`.** `pretax_income` is zero or negative.
- **`tax_rate_outside_zero_to_one`.** `pretax_income` is positive, and the
  rate is below zero or above one.

Each edge in turn:

- **A zero `pretax_income`.** The rate is a quotient with a zero divisor, so
  it is undefined. Declined under `non_positive_pretax_income`.
- **A negative `pretax_income`.** Declined under the same condition.
  - The rate is defined as a share of a profit charged as tax. At a loss,
    there is no profit to charge. The quotient is then the share of a loss
    the filer recovered, which is a different quantity.
  - Applied to an operating profit, it treats a recovery rate as a charge
    rate. Where a valuation allowance limits the benefit, that recovery rate
    is small, and NOPAT reads close to operating income. That flatters.
- **A negative `income_tax_expense`, which v2 reads as a benefit.** It is not
  a condition alone. Over a positive `pretax_income`, it gives a rate below
  zero, and the next edge declines it. Over a non-positive one, the edge
  above declines first.
- **A rate above one or below zero.** Declined under
  `tax_rate_outside_zero_to_one`.
  - A share of a profit lies between none of it and all of it. Outside those
    bounds, the period's charge is dominated by amounts that belong to other
    periods, such as a valuation allowance released or a deferred tax
    balance remeasured.
  - Applied to operating income, a rate below zero gives a NOPAT above
    operating income. A rate above one turns an operating profit into an
    after-tax loss. Neither is a profit after a tax on it.
  - The bounds are inclusive. A rate of zero, such as a REIT paying no
    entity-level tax, gives NOPAT equal to operating income. A rate of one
    gives NOPAT of zero.
  - **No bound is applied.** A rate outside the bounds is never clamped to
    them. A clamp would substitute a number no filing states.
  - The test needs no literal. The rate is below zero exactly when
    `income_tax_expense` is negative, and above one exactly when it exceeds
    `pretax_income`. Both are sign tests, and one is `Number::one`.

**No rate is a setting.** The accepted analyze record defines a setting as a
value "neither the filings nor the metric's cited source fixes". The filings
fix the effective rate, and where they do not fix it as a share of a profit,
the metric declines. A statutory rate is not a setting either:

- The law fixes it, by jurisdiction and year.
- v2 carries neither the jurisdiction nor the rate.
- One preset would be wrong across a change in the law. The United States
  federal corporate rate, graduated to a top rate of 35%, became a flat 21%
  for tax years beginning after 2017, under Public Law 115-97, §13001,
  amending 26 U.S.C. §11(b). A preset would also be wrong for every filer
  taxed elsewhere.

`Settings` stays empty.

### The two metrics

| Name | Formula | Inputs, in reason order |
| :--- | :--- | :--- |
| `return_on_capital_employed` | `operating_income` over the average capital employed | `operating_income`, `total_assets`, `current_liabilities` |
| `return_on_invested_capital` | NOPAT over the average invested capital | `operating_income`, `income_tax_expense`, `pretax_income`, `shareholders_equity`, `short_term_debt`, `long_term_debt`, `cash_and_equivalents`, `short_term_investments` |

- Both are stated at a duration, with the unit a fraction of one.
- Return on capital employed is before tax, as its source states it. Return
  on invested capital is after tax. Each name carries one meaning.
- The inputs follow the terms in formula order, numerator first, and each
  term's own order inside it.

### Where a flow meets a balance

Which instant each balance is read at is the rule
`which-other-period-a-metric-reads.md` fixes. It names both metrics as
averaged. This record decides nothing of it.

**The average of the term, or the term of the averages.** Both terms are sums
and differences of balances. The mean is linear, and the arithmetic is exact
rationals with no rounding before the one writer. So the two are one figure.
For capital employed:

- ((TA₀ − CL₀) + (TA₁ − CL₁)) / 2 = (TA₀ + TA₁) / 2 − (CL₀ + CL₁) / 2

**What is read is the term of the averages.** The period rule's reads are per
balance concept, in input order, opening then closing. So capital employed
reads `total_assets` at the opening, then at the closing, then
`current_liabilities` at each. The reasons follow that order.

**Every average is over two instants.** Where `short_term_investments`
contributes nothing at an instant, that read is summed as nothing and still
counted. The count is two, and never the number of reads that held a
`Value`.

- This matters only where the kind changed between the two instants. Then
  `short_term_investments` is `NotApplicable` at one and a `Value` at the
  other.
- Counting only the `Value` would make that one balance the average. The
  result would be a different figure that looks right.

### How absence composes

**Reads.** Return on capital employed makes five reads:

- `operating_income` at the duration
- `total_assets` opening, then closing
- `current_liabilities` opening, then closing

Return on invested capital makes thirteen:

- `operating_income`, `income_tax_expense` and `pretax_income` at the
  duration
- then each of its five balances, opening then closing

Every read is made. Each gives a number or one reason, as the period record
says. Missing rows give `no_period` with its two `sought_by` names, and a
start outside the grammar gives `start_not_a_date`.

**No term contributes nothing, but one.** The single exception is the one the
accepted records allow: `short_term_investments` inside net debt, where it is
`NotApplicable`. It applies to invested capital because invested capital reads
net debt. It is applied as accepted, not extended. Specifically:

- An `Unknown` `short_term_investments` makes the metric absent.
- A missing row gives `no_period` for it, as for every other balance.
- `current_liabilities` never contributes nothing. Capital employed for a
  REIT is not its total assets.
- `operating_income` never contributes nothing.
- Neither debt contributes nothing where it is `Unknown`. A debt-free filer
  whose silence reads `Unknown` has no return on invested capital, unless a
  registry override asserts the zero.

**By kind.** These hold where every other input is a `Value`:

| Kind | `return_on_capital_employed` | `return_on_invested_capital` |
| :--- | :--- | :--- |
| `operating` | computed | computed |
| `reit` | `input_not_applicable` for `current_liabilities` at the opening, then at the closing | computed, with `short_term_investments` contributing nothing |
| `bank`, `insurer`, `investment_company` | `input_not_applicable` for `operating_income` at the duration, then for `current_liabilities` at the opening and at the closing | `input_not_applicable` for `operating_income` at the duration, alone |

A filer with no kind resolves every concept through the attempt. So where a
bank's `current_liabilities` is not found, it gives `input_unknown`, never
`input_not_applicable`.

**The `declined` conditions,** each named once and defined beside its term:

- **`zero_capital_employed`.** The average capital employed is zero.
- **`zero_invested_capital`.** The average invested capital is zero.
- **`non_positive_pretax_income`** and **`tax_rate_outside_zero_to_one`**, as
  stated above, beside the rate.

Each is asked only when every read gives a number. They are asked in formula
order, numerator first, and the first that holds is the one reason. So the
order for return on invested capital is the two rate conditions, then
`zero_invested_capital`. Both entries also list `start_not_a_date`, and
`input_not_a_decimal` reaches them from the one definition beside the reader.

**A negative capital is a value.** A negative average capital employed or
invested capital gives a figure, with its sign. It is never `declined`. This
is what `return_on_equity` does under M5-30 and what `debt_to_equity` does
today. One family of returns keeps one rule. Cash netting makes a negative
invested capital more common than a negative equity. The cost is stated under
Consequences, and the alternative is argued.

No seventh reason is needed, and no frozen byte changes.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition:

- the five terms and the one module that forms them
- the rate's edges and the four conditions
- the two names and their formulas
- the count of two in every average where a term contributes nothing

A later change to any of them is a version bump and a superseding record, and
never an edit to this one.

It lands at version 1. Neither metric is computed yet, so no stored outcome
changes. Moving `debt_to_equity` onto the module's total debt changes no
figure.

### What a metric's catalogue entry says

Nothing more than the shape M5-18 fixed:

- `stated_at` is `Duration`.
- `inputs` are as the table above lists them.
- `conditions` are the term conditions the metric meets, then
  `start_not_a_date`.
- `meaning` names each term in words, as the formula has it. For example:
  "operating income over the duration, divided by the average of total assets
  less current liabilities at the instants it opens and closes at". Return on
  invested capital's `meaning` says that the preferred claim is included and
  that cash and short-term investments are netted.

A field naming the terms would be a second copy of what the derivation reads,
unless the derivation were generated from it.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at` and read no file.
Each stands in `tests/` or under `#[cfg(test)]`. Each history holds the
duration 2024-01-01 to 2024-12-31 and the instants 2023-12-31 and 2024-12-31,
unless the test says otherwise. Balances are given opening/closing. At the
least:

- **Return on capital employed, worked by hand.**
  - Operating income 135, total assets 1000/1400 and current liabilities
    200/400 give capital employed 800/1000. The average is 900, which is also
    1200 − 300. The figure is `0.15`.
  - Total assets 300/300 and current liabilities 400/500 give `-0.9`.
- **`zero_capital_employed`.**
  - Total assets equal to current liabilities at both instants.
  - Capital employed 100/−100, from total assets 600/400 and current
    liabilities 500/500. The condition is on the average.
- **Return on invested capital, worked by hand.**
  - The inputs:
    - operating income 120, pretax income 100, income tax expense 25
    - shareholders' equity 700/900
    - short-term debt 50/50, long-term debt 150/250
    - cash 40/60, short-term investments 100/100
  - The rate is 0.25 and NOPAT is 90. Invested capital is 760/1040, so the
    average is 900. The figure is `0.1`.
- **The edges of the rate**, over that row:
  - Pretax income 0 gives `non_positive_pretax_income`.
  - Pretax income −100 with tax −25 gives `non_positive_pretax_income`, even
    though the quotient would be 0.25.
  - Tax −10 gives `tax_rate_outside_zero_to_one`.
  - Tax 110 gives `tax_rate_outside_zero_to_one`.
  - Tax 100 gives `0`, at the inclusive bound of one.
  - Pretax income 0 together with a zero invested capital gives
    `non_positive_pretax_income` alone.
- **`zero_invested_capital`.** Equity 100, short-term debt 0, long-term debt
  50, cash 100 and short-term investments 50 at both instants.
- **The exception, and its count.**
  - The row above with `short_term_investments` `NotApplicable` at both
    instants: invested capital 860/1140, average 1000, figure `0.09`.
  - The same with tax 0: `0.12`.
  - `NotApplicable` at the opening and 200 at the closing: invested capital
    860/940, average 900, figure `0.1`. It is never `0.1125`, which counting
    one read would give.
  - `short_term_investments` `Unknown` at the closing gives `input_unknown`
    naming it at 2024-12-31.
- **The order.** `income_tax_expense` `Unknown` with the opening instant
  missing gives six reasons: `input_unknown` for the tax at the duration,
  then `no_period` with `opening_instant` for each of the five balances.
  `short_term_investments` gives its `no_period` too.
- **By kind.** A bank's row, with `operating_income`, `current_liabilities`
  and `short_term_investments` `NotApplicable` and every other concept a
  `Value`:
  - `return_on_capital_employed` gives the three `input_not_applicable`
    reasons the table states, in that order.
  - `return_on_invested_capital` gives the one.
  - A REIT's row gives the two for the first and a figure for the second.
- **Total debt.** `debt_to_equity` gives its pinned figures unchanged after
  it reads the module.
- **The catalogue.** No entry whose `inputs` hold `preferred_equity` is
  among the two here.

### What no check can see

- **Whether the sources say what this record says.** That covers Atrill and
  McLaney, Damodaran, the CFA text's "Income Taxes" chapter, ASC
  740-10-50-12, Regulation S-X Rule 4-08(h), and Koller, Goedhart and
  Wessels. Each metric's documentation cites its page as its task reads it.
  A source that on reading says otherwise is a stop for that task, not a
  choice. Two are the most exposed:
  - The edition and chapter given for Atrill and McLaney.
  - Whether Damodaran nets marketable securities beside cash.
- **A metric that forms a term itself,** such as `total_assets` less
  `current_liabilities` written inline. Only review sees it.
- **Non-controlling interests and redeemable preferred**, named above. Return
  on invested capital is overstated for every filer carrying either, and
  nothing measures by how much.
- **All cash is netted, including the cash the business needs to run.** Koller,
  Goedhart and Wessels net only excess cash. v2 cannot tell the two apart. So
  invested capital is understated by the operating cash, which flatters.
- **Return on capital employed holds cash in its denominator** while its
  numerator excludes the income on it. This is the source's form. It reads a
  cash-rich filer low, which is the conservative direction.
- **Whether `pretax_income` and `income_tax_expense` are stated on the same
  scope as `operating_income`.** v2 states `net_income` from continuing
  operations, and states neither of these so. A filer whose tax line carries
  discontinued operations gives a rate on a mixed base.
- **A rate inside the bounds that is still not the rate on operating profit.**
  Pretax income is after interest and non-operating items, and the tax on
  them is in the charge. One-off items that leave the rate between zero and
  one pass.
- **A rate applied to an operating loss.** Where operating income is
  negative and the rate is in bounds, NOPAT is a smaller loss: a tax shield
  the filer may never realise.
- **A quarter beside a year,** carried over from the period record.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The terms
  need only one, through `Number::one`, and the mean's count needs no literal,
  as the period record says. Every other number comes from the history.
- `vfi-analyze` takes no dependency, and `constants/` and `settings/` gain no
  file.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and catalogue
- the arithmetic record's reading, operations and written form
- the period record's readings, seeks and date reading
- the open-questions record's net debt and its one exception
- the frozen bytes of `canonical-concepts` v2 and `analyze-store` v1
- the entry shape M5-18 fixed

One question here would need one of those to change, and it is left open and
named: a concept for non-controlling interests or for redeemable preferred
outside permanent equity, which needs a v3. It is not decided here.

**Why Structural.** The record fixes the meanings and names of two metrics
that cross `analyze-store` v1, which store and M6 will read. It also fixes
terms that enterprise value and the discounted cash flow will share, and it
names five new consumers. It needs nothing Constitutional: no anchor, no
gate, no protected path and no GOALS.md scope moves.

## Alternatives

- **Capital employed as equity plus debt.** This is the CFA text's return on
  total capital: EBIT over short- and long-term debt and equity. It reads no
  `current_liabilities`. Rejected:
  - The catalogue names `current_liabilities` as this term's subtrahend, and
    this form would leave that consumer reading nothing.
  - It would make capital employed invested capital without cash netting, so
    the two returns would differ only by tax and cash.
  - It reads `shareholders_equity`, so it carries the non-controlling gap.
    Total assets less current liabilities does not.
- **Invested capital without cash netted**, as debt plus equity. It is never
  negative unless equity is, and it reads two concepts fewer. Rejected:
  `operating_income` excludes the income cash earns, so a denominator that
  holds the cash sets income against capital that did not earn it. Not
  netting is the conservative error, but it is an error the source corrects.
- **Netting `cash_and_equivalents` alone.** This avoids the exception
  entirely. Rejected: short-term investments earn the same excluded income.
  Leaving them in would make one filer's return depend on how it splits its
  liquidity between the two lines, which the open-questions record split for
  other reasons.
- **The tax rate as a statutory rate held as a setting.** It is defined at a
  loss and never outside the bounds, so it would give a figure where the
  effective rate declines. Rejected:
  - The catalogue names the effective rate.
  - The law fixes a statutory rate by jurisdiction and year, so it is not a
    free parameter.
  - A single preset is wrong across the 2017 change and for every filer
    taxed elsewhere.
- **The statutory rate as a fallback** where the effective rate declines.
  Rejected: it substitutes a number for an undefined input, which the
  accepted record forbids. It also mixes two methods under one name.
- **The rate clamped to zero and one.** Rejected: a clamp substitutes a
  bound for the filer's figure. A rate of 1.4 would become 1, which no filing
  states.
- **The rate read at a pretax loss.** ASC 740's reconciliation presents a
  rate at a loss too. Rejected: at a loss the quotient is a recovery rate,
  not a charge rate. Where a valuation allowance limits the benefit, it
  understates the charge on operating profit, which flatters.
- **A zero rate at a pretax loss,** so NOPAT equals operating income.
  Rejected: it substitutes zero for an undefined rate.
- **Return on invested capital left out of M5** until a later vocabulary
  carries non-controlling interests and redeemable preferred. This avoids
  the known overstatement. Rejected: v2 forms the term on Damodaran's own
  financing form. The two missing claims are named with their direction,
  and a v3 record can close them by superseding this one.
- **Equity as `total_assets` less `total_liabilities`.** This holds
  non-controlling interests and temporary equity by construction. Not
  rejected, but not here. It moves invested capital's equity off the concept
  the catalogue names, and it adds a sixth new consumer. A superseding record
  can argue it against a v3 concept.
- **Koller, Goedhart and Wessels's operating form,** with operating working
  capital, fixed assets and operating cash taxes. Rejected: it needs
  receivables, payables, goodwill and deferred taxes, which v2 leaves out.
- **Declining at a negative capital,** as `non_positive_invested_capital`.
  Over a negative invested capital, an operating loss reads as a positive
  return. Rejected here: no source this record reads makes the return
  undefined there. The quotient is defined, and `return_on_equity` and
  `debt_to_equity` give a figure at a negative denominator. A record that
  finds such a source supersedes this one and bumps the method for the whole
  family. This is the item most worth the decider's judgement.
- **Averaging the term at each instant**, rather than the averages of its
  concepts. The figure is the same. Rejected for the read order alone: the
  period rule's reads are per concept, and a second order would be a second
  rule.
- **Each metric forming its own terms.** Rejected: two readings of one rule,
  and net debt formed a third time by enterprise value.

## Consequences

**Easier:**

- Both returns have their terms, sources and names. Each implementing task has
  only its code left to decide.
- Enterprise value and the discounted cash flow bridge read net debt from one
  module, with the exception already applied.
- Total debt is formed once, and `debt_to_equity` reads it there.
- No setting is added, so no preset needs a reason.

**Harder:**

- **Return on capital employed is `operating` only.** Every REIT, bank,
  insurer and investment company is absent for it.
- **Return on invested capital needs eight concepts at thirteen reads.** Both
  debts read silence as `Unknown`. A debt-free filer with no override is
  absent, and so is any filer missing either instant.
- **A pretax loss, or a rate outside the bounds, makes it absent.** So does
  a year with a large valuation allowance release or a remeasurement of
  deferred tax, such as 2017 for many United States filers.
- **A negative invested capital gives a figure with its sign.** A filer
  holding more cash than its equity and debt combined reads a negative return
  on a profit and a positive return on a loss. A ranking that reads the
  figure alone misleads. Whether M6 reads it alone is M6's.
- **Return on invested capital is overstated** wherever non-controlling
  interests or redeemable preferred exist, until a v3 record carries them.

**Expensive to reverse:**

- The two names and their formulas, once results are stored under them.
- Net debt's form, once enterprise value is built on it.

**Open, and not this record's:**

- A concept for non-controlling interests, and for redeemable preferred
  outside permanent equity.
- Whether a return over a negative capital declines.
- Whether enterprise value adds the preferred claim or non-controlling
  interests to net debt.
- Which instant a balance is read at, which is the period record's.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **One definition per term.** The total debt test holds `debt_to_equity` to
  the module.
  - **Gap:** nothing forces a metric through the module rather than around
    it.
- **Anchor 5.** `literals` keeps every number but one out of the terms. No
  constant is added.
  - **Gap:** nothing checks that a citation is true.
- **The count of two.** The kind-change test holds it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the names and
  meanings of two metrics that cross `analyze-store` v1 as characters, forms
  three terms every later reader of them shares, and names five consumers of
  concepts v2 carries no consumer list for, all inside analyze. It edits no
  anchor, no protected path, no gate, no contract byte, no schema and no
  milestone scope, adds no setting and asks for no allowlist entry. Flagged
  for later human review, as the tier requires.
- **Checked:**
  - `canonical-concepts.md`: the three partial namings are quoted as
    printed, and "a concept no metric consumes is not in this list" is the
    sentence the new-consumer argument rests on. It says why a concept is in
    the set, not who may read it, and v2's own file carries no consumer list.
  - The frozen v2 contract: `short_term_debt` and `long_term_debt` read
    silence as `unknown` and apply to every kind; `short_term_investments`
    reads silence as zero and applies to `operating` alone; `operating_income`
    applies to `operating` and `reit`. The record states each the same way.
  - `canonical-concepts-open-questions.md`: net debt is formed in analysis
    from both cash concepts, and `short_term_investments` inside it is the one
    term a `NotApplicable` lets contribute nothing. The record carries both
    and extends neither.
  - `what-analyze-takes-and-returns.md`: the six reasons stand and no seventh
    is added; the four conditions are `declined` naming a condition per the
    catalogue entry; a setting is a value "neither the filings nor the
    metric's cited source fixes", and the record's argument that the filings
    fix the effective rate and the law fixes a statutory one meets that
    definition.
  - `which-other-period-a-metric-reads.md`: both metrics are listed as
    averaged, an averaged balance is two reads, opening then closing, and the
    record decides nothing of the instant and shows the two means equal.
  - `common-shareholder-forms.md`: another consumer of `preferred_equity` is
    "its own record"; this record reads neither preferred concept.
  - `debt_to_equity.rs`: forms total debt inline and gives a signed figure at
    a negative equity, as the record says of it.
  - `bare-literal-lint.md`: no place, form or allowlist entry moves, and the
    one literal the terms need is `Number::one`.
  - The hand-worked figures in the proof section, each reproduced, including
    `0.1` against the `0.1125` that counting one read would give.
  - The alternatives as argued: equity plus debt, cash not netted, cash alone
    netted, a statutory rate as a setting or as a fallback, a clamp, a rate
    read at a loss, a zero rate at a loss, leaving the metric out of M5,
    equity as assets less liabilities, the operating form, declining at a
    negative capital, averaging the term, and each metric forming its own
    terms.
- **Verdict and why:** accepted. The two returns needed terms no record had
  formed, and forming them in a metric's file would have put a method choice
  where enterprise value and the discounted cash flow bridge would inherit it
  unseen. The record forms each term once under a cited source, and at every
  edge the filings do not fix the rate it declines rather than substitutes:
  no clamp, no statutory rate, no zero at a loss. The new-consumer reading is
  right: the catalogue's lists are the derivation that earned each concept its
  place, not a grant, and v2 carries no list to amend. The gaps it cannot
  close, non-controlling interests, redeemable preferred outside permanent
  equity and the operating cash inside the netting, are named with their
  direction, which flatters in each case, and left to a v3 record rather than
  approximated here.
- **What would have changed it:** a seventh reason; a changed byte of
  `canonical-concepts` v2 or `analyze-store` v1; a rate clamped, defaulted or
  read at a loss; a term formed in a metric's file; a read of
  `preferred_equity`; or a source that on reading forms capital employed or
  invested capital otherwise than stated, which the record makes a stop for
  the implementing task. Two items are flagged for the human beside the tier:
  the signed figure at a negative capital, kept for one rule across the family
  of returns and argued either way in the record, and the overstatement of
  return on invested capital wherever non-controlling interests or redeemable
  preferred exist, which nothing measures. One more sits outside the record:
  it was written before `sources-a-task-can-read.md` was accepted, and its
  "cites its page as its task reads it" now reads under that record, so the
  implementing tasks cite the two closed books by chapter alone and the open
  primaries by the locator they have.
