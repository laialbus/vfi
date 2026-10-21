# The discounted cash flow grows a year's free cash flow over a forecast and a perpetuity under four settings, subtracts net debt and divides by the diluted share count; its margin of safety is that value less the valuation date's close, as a fraction of the value

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-19, by `M5-43`
- **Decided:** 2026-10-19, by the decider
- **Touches:** the discounted cash flow with margin of safety that GOALS.md's
  fourth M5 criterion names and no record forms. It fixes the names and
  meanings of two metrics that cross `analyze-store` v1 as characters, and no
  byte of that contract changes. It forms free cash flow once, in a module
  that `dividend_payout_on_free_cash_flow` moves onto with identical outcomes.
  It names four settings, each defined once under the analyze record's
  definition. It applies the accepted `declined` reason to five new
  conditions and adds no reason. It reads net debt from the capital module as
  the capital record forms it, and reads the price as the analyze record's
  price-dependent section fixes, making the margin the first metric to read
  the crossing. It names, and leaves open, two questions about accepted
  records: which instant the bridge reads net debt at, and whether the
  share-basis rule may compare dates through the ordering the series record
  added. It adds a value of the
  catalogue entry's `unit`, currency per share, and no field. No accepted
  record is superseded. No contract's bytes, no schema, no edge, no gate and
  no dependency move. Nothing here is implemented.

## Context

GOALS.md's fourth M5 criterion ends with "discounted cash flow with margin of
safety". Every term around it is fixed in part, and the derivation itself is
not.

Some things are already fixed:

- **Settings.** `what-analyze-takes-and-returns.md` (Accepted) defines a
  setting as a value "neither the filings nor the metric's cited source
  fixes", names "the discount rate, terminal growth and forecast horizon" as
  examples, and leaves which settings exist to the metric's task. Each has a
  name, meaning, unit, domain, preset and the preset's reason, defined once in
  `crates/analyze/src/settings/`, behind a constructor that refuses a value
  outside its domain and never clamps one. A threshold applied to stored
  metrics is a ranking criterion, which is M6's.
- **The consumers the catalogue names.** `canonical-concepts.md` (Accepted)
  names free cash flow a consumer of `operating_cash_flow` and
  `capital_expenditure`, and through it "the discounted cash flow". It names
  "the per-share step of the discounted cash flow" a consumer of
  `diluted_shares_weighted_average`. It names "the equity bridge in the
  discounted cash flow" a consumer of `cash_and_equivalents`, and the
  discounted cash flow bridge a consumer of both debts.
  `canonical-concepts-open-questions.md` (Accepted) names "the equity bridge
  in the discounted cash flow" a consumer of `short_term_investments`, and
  inside net debt makes that concept the one term a `NotApplicable` lets
  contribute nothing.
- **What v2's concepts mean.** `operating_cash_flow` is "after working capital
  movements and after interest and tax where the filer classifies them
  there". `capital_expenditure` is a magnitude, and its silence is `Unknown`,
  because "reading it as zero would overstate free cash flow by the whole of
  it". `diluted_shares_weighted_average` is a flow-weighted count over the
  period. All three apply to every kind.
- **Net debt.** `capital-employed-invested-capital-and-nopat.md` (Accepted)
  forms net debt once in `crates/analyze/src/metrics/capital.rs`, as total
  debt less `cash_and_equivalents` and less `short_term_investments` at one
  instant, carrying the one exception. It names "the discounted cash flow
  bridge" its other consumer, which "read[s] it here". It leaves
  non-controlling interests and redeemable preferred outside permanent equity
  to a v3, and reads no `preferred_equity`. Total debt is in the module today.
  Net debt arrives with `M5-41`.
- **Free cash flow is formed today, inline.**
  `dividend_payout_on_free_cash_flow.rs` writes `operating_cash_flow` less
  `capital_expenditure` itself.
- **The price.** `price-provider.md` (Accepted) names "the margin of safety in
  the discounted cash flow" a consumer of the unadjusted close at the
  valuation date, "comparing a per-share value with the price". The analyze
  record fixes how a price-dependent metric reads the crossing, its three
  `no_price` witnesses, and the share-basis rule with its three
  `basis_unproven` witnesses. No metric reads the crossing yet: `analyze`
  holds it and hands `metrics::at` the history and the settings alone.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written, and
  a table of operations. It names integer powers as what "discounting uses".
- **The periods.** `which-other-period-a-metric-reads.md` (Accepted) fixes how
  a duration metric reads a balance. `which-duration-precedes-a-duration.md`
  (Accepted) fixes what a year is, the four conditions at a duration that is
  not one, and the series.
- **Absence is closed.** Six reasons. A metric built on another carries the
  other's reasons.

## Decision

### Two metrics

| Name | What it is | Unit |
| :--- | :--- | :--- |
| `discounted_cash_flow_value_per_share` | The equity value the derivation below arrives at, per diluted share | currency per share |
| `discounted_cash_flow_margin_of_safety` | That value less the close at the valuation date, over that value | fraction of one |

Both are stated over a year (below). The first reads no price. The second is
built on the first and reads one.

### Free cash flow, formed once

**Free cash flow** is `operating_cash_flow` less `capital_expenditure`, over
one duration. This is the form `dividend_payout_on_free_cash_flow` writes
today.

- **The source.** Securities and Exchange Commission, Division of Corporation
  Finance, *Compliance and Disclosure Interpretations: Non-GAAP Financial
  Measures*, Question 102.07, May 17, 2016,
  <https://www.sec.gov/corpfin/non-gaap-financial-measures>, read in this run:
  "Some companies present a measure of "free cash flow," which is typically
  calculated as cash flows from operating activities as presented in the
  statement of cash flows under GAAP, less capital expenditures."
- **Where it lives.** One module of the derivation,
  `crates/analyze/src/metrics/free_cash_flow.rs`. Every metric that reads free
  cash flow reads it there: this one, `dividend_payout_on_free_cash_flow`, and
  the price/free-cash-flow multiple when it is built. No metric writes the
  subtraction itself.
- **The move.** The task that builds the module moves
  `dividend_payout_on_free_cash_flow` onto it. Its entry, inputs, conditions,
  readings, reasons and pinned figures are unchanged, and `METHOD_VERSION`
  stays at 1. That is a refactor with identical outcomes, as the capital
  record said of moving `debt_to_equity` onto total debt. The payout keeps
  reading free cash flow at every duration. Only this metric adds the year
  test.
- **No term contributes nothing.** An `Unknown` `capital_expenditure` gives
  `input_unknown` naming it. It is never read as zero, which is the absence
  the catalogue fixes.
- The inputs are read in this order: `operating_cash_flow`, then
  `capital_expenditure`.

### The derivation, in formula order

Let Y be the year the metric is computed at. The settings are r, the discount
rate; g, the growth rate over the forecast; n, the forecast years; and g∞, the
terminal growth rate (below). Every step uses only operations in the
arithmetic record's table.

| Step | What it is | Operations |
| :--- | :--- | :--- |
| 1. The base flow | F₀, free cash flow over Y alone | subtraction |
| 2. Each forecast year's flow | Fₜ = F₀ × (1 + g)ᵗ, for t from 1 to n | addition, multiplication, integer power |
| 3. Each forecast year discounted | Fₜ ÷ (1 + r)ᵗ | addition, integer power, division |
| 4. The terminal value at year n | T = Fₙ × (1 + g∞) ÷ (r − g∞) | addition, subtraction, multiplication, division |
| 5. The terminal value discounted | T ÷ (1 + r)ⁿ | integer power, division |
| 6. The value of operations | V = the sum of step 3 over t, plus step 5 | addition |
| 7. The bridge | E = V less net debt, read from the capital module | subtraction |
| 8. The per-share step | E ÷ `diluted_shares_weighted_average` over Y | division |

**The base is the computing year alone.** No other year is read, so the
series record's seek is not used.

**Every exponent is a count from a setting.** t runs from one to n, and n is
the setting. No literal enters. Computing each power as the running product
of the one before gives the same exact number, and the implementing task may
do either.

**The terminal value is a growing perpetuity.**

- The first year after the forecast is year n's flow grown once at g∞, and the
  perpetuity is that flow over r less g∞, valued at year n.
- It is undefined where r is not above g∞. At equality the divisor is zero.
  Below it, the perpetuity has no finite sum, and the quotient would be a
  negative value for a positive flow.
- That is the `declined` condition `discount_rate_not_above_terminal_growth`.
  The test is a sign test on r less g∞, and it needs no literal.

**The source.** Damodaran, A., *Valuation Approaches and Metrics: A Survey of
the Theory and Evidence*, Stern School of Business, New York University,
November 2006, <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>,
read in this run. It is the author's own paper, and open.

- Page 28: "The value of the firm, in the most general case, can be written as
  the present value of expected free cashflows to the firm." The same page
  writes the value of operating assets for a firm that "reaches steady state
  after n years" as the sum of FCFFₜ over (1 + WACC)ᵗ for t from 1 to n, plus
  FCFFₙ₊₁ over (WACC − gₙ), discounted by (1 + WACC)ⁿ. Steps 2 to 6 are that
  form, with FCFFₙ₊₁ taken as Fₙ grown once at g∞.
- Page 27, of the stable growth rate: "the growth rate used in the model has to
  be less than or equal to the growth rate in the economy – nominal growth if
  the cost of capital is in nominal terms".
- Page 29, the bridge: "To get from the value of operating assets to the value
  of equity, we have to first incorporate the value of non-operating assets
  that are owned by the firm and then subtract out all non-equity claims that
  may be outstanding against the firm." And: "The most common of the
  non-operating assets is cash and marketable securities".

Nothing in the derivation is outside the arithmetic record's table. No root,
logarithm or real power is taken.

### The flow is free cash flow, and not the flow to the firm

This is the method's largest known error, and it is stated here rather than
found later.

- Damodaran's form discounts the flow to the firm, which he calls "a pre-debt
  cashflow" (page 29), and then subtracts debt.
- v2's `operating_cash_flow` is after interest "where the filer classifies
  [it] there". United States GAAP classifies interest paid and interest
  received in operations (Financial Accounting Standards Board, ASC Topic
  230, *Statement of Cash Flows*, not read from the workspace). So for such a
  filer free cash flow is after net interest.
- The bridge then subtracts the net debt that bears that interest. Net
  interest is counted twice: once in the flow, once in the claim.
- **The direction.** For a filer that pays more interest than it receives, the
  value is understated by the present value of its net interest, which is
  the conservative direction. For a filer whose cash earns more than its debt
  costs, the value is overstated by the same amount, which flatters. The
  margin of safety inherits both.

The flow to the firm, free cash flow plus interest expense after tax, is
argued under Alternatives. It is rejected for this version because v2 does not
say where each filer classified interest, and adding back interest the flow
never deducted is the flattering error for every filer that classified it in
financing.

### The bridge

**Net debt is read from the capital module,** with its one exception as
accepted. Inside it, a `NotApplicable` `short_term_investments` contributes
nothing. Nothing else does. An `Unknown` debt makes the metric absent, unless
a registry override asserts the zero.

**Which instant net debt is read at is the period record's rule, and the rule
reads it averaged.** `which-other-period-a-metric-reads.md` says: "Every other
metric that pairs a flow with a balance reads the average. This holds whatever
source its formula cites." The discounted cash flow pairs a flow with
balances, and it is not a term of a named score. So the bridge subtracts the
average of net debt at the instants Y opens and closes at. The capital
record's count of two holds in that average.

This record names a question about that rule and leaves it open:

- The rule's reason is a ratio's: "The mean of the two ends is the balance the
  flow was earned on." The bridge does not set a flow against a balance. It
  subtracts the claims outstanding at the date the flows are discounted to,
  which is Y's close.
- Read as written, the bridge subtracts the year's mean claim. Where net debt
  rose over Y, the claim is understated and the value overstated. Where it
  fell, the reverse.
- Reading the bridge at Y's close needs the period record superseded or
  narrowed. That is a record of its own. This record does not read the rule
  narrowly and does not propose that record.

**Neither the preferred claim nor non-controlling interests is subtracted.**

- **Non-controlling interests.** v2 has no concept for them, as the capital
  record says. The flow is the consolidated flow, so the minority's share of
  it is valued and none of it is taken out. The equity value is overstated by
  the minority's claim. That flatters.
- **The preferred claim.** v2 carries `preferred_equity`, and the bridge does
  not read it. Its silence is `Unknown`, and the common-shareholder record
  found it `Unknown` at nine of twelve rows in the fixtures. Reading it would
  make the discounted cash flow absent for most filers, which carry no
  preferred, to correct a claim few carry. The equity value is overstated by
  the carrying amount of the preferred claim, for filers that have one. That
  flatters.
- A v3 concept for non-controlling interests, and a reading of the preferred
  claim that does not cost every silent filer, close both. Each is a record of
  its own, and this record is superseded when one lands.
- **The other non-equity claims** Damodaran names on page 29 are "capitalized
  leases as well as unfunded pension plan and health care obligations".
  Finance leases are inside v2's debts. Operating lease liabilities are
  outside them, and their payments are inside the flow, as the
  open-questions record's fourth answer fixes, so each lease is counted once.
  Unfunded pension and health care obligations have no v2 concept. They are
  not subtracted, and the value is overstated by them.

Because no preferred term is formed, no common term is formed, and neither
name carries `common`, under the common-shareholder record's naming rule.

### The per-share step

The equity value is divided by `diluted_shares_weighted_average` over Y, as
the catalogue names it.

- It counts the options and convertibles that would share in the equity, so
  the value per share is per diluted share.
- It is read at Y's own row, as a flow.
- `shares_outstanding` is not read. The catalogue names the diluted count for
  this step, and nothing here sets that naming aside.
- A zero or negative count is the `declined` condition
  `non_positive_diluted_shares`. A zero count is a zero divisor, and a
  negative one is no count and would turn the sign of the value.

**The name.** The common-shareholder record puts `per_share` in "the name of a
form that reads per-common-share concepts". This metric reads none. It
divides by a share count. That record states `common` "exactly when", and
states `per_share` without "exactly". So the record reads its rule as naming
where `per_share` must appear, not forbidding it elsewhere. The metric's
`meaning` says "per diluted share", so no reader takes it for a reported
per-share concept.

### The settings

Four settings, defined once in `crates/analyze/src/settings/discounted_cash_flow.rs`,
each a field of the one `Settings`. The derivation reads them only from the
`Settings` it is handed. It never calls the preset constructor.

| Name | Meaning | Unit | Domain | Preset |
| :--- | :--- | :--- | :--- | :--- |
| `discounted_cash_flow_discount_rate` | The annual rate each year's flow and the terminal value are discounted at: the return required of the capital the flow serves | a rate per year, a fraction of one | rates above zero | `0.0772` |
| `discounted_cash_flow_growth_rate` | The annual rate the base flow grows at in each forecast year | a rate per year, a fraction of one | rates above minus one | `0.02` |
| `discounted_cash_flow_years` | How many years the forecast runs before the perpetuity | years | the positive integers | `5` |
| `discounted_cash_flow_terminal_growth_rate` | The annual rate the flow grows at for ever after the forecast | a rate per year, a fraction of one | rates above minus one | `0.02` |

A rate's domain also admits only values of at most `SIGNIFICANT_DIGITS`
significant digits, as the arithmetic record requires of every numeric
setting. Each value is written by the one writer.

**Why each domain.**

- **The discount rate is above zero.** At zero or below, a later flow is worth
  at least as much as an earlier one, which is no required return. At minus
  one, the discount divides by zero.
- **A growth rate is above minus one.** At minus one every later flow is zero,
  and below it the flows alternate in sign. Neither is a rate of growth.
- **The years are positive.** Zero years would leave the forecast rate unread.

**The relation is a `declined` condition, not a domain.** A discount rate not
above the terminal growth is `discount_rate_not_above_terminal_growth`, asked
at step 4. It is not refused by a constructor, because each setting's
constructor sees one value. A domain on the pair would make the order of the
two setters matter: moving both from their presets to a discount of 0.03 and
a terminal growth of 0.01 would be refused in one order and admitted in the
other. Every result records both values, so a declined figure shows why.

**The presets and their reasons.**

- **`discounted_cash_flow_discount_rate`, `0.0772`.** The cost of capital of
  the United States market outside financial firms, as Damodaran publishes
  it: *Cost of Equity and Capital (US)*, "Data used is as of January 2026",
  <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/wacc.html>,
  read in this run. The row "Total Market (without financials)" gives
  "Cost of Capital" 7.72%. A filer valued at the preset is asked to earn what
  the average non-financial firm's capital costs. The row without financials
  is taken because the form is the one Damodaran says is hard to apply to
  financial firms (page 15 of the survey, below). The source republishes each
  January. Moving the preset to a later figure is a preset change, which is
  not a version bump, since every result records the value it used.
- **`discounted_cash_flow_terminal_growth_rate`, `0.02`.** The Federal Open
  Market Committee, *Statement on Longer-Run Goals and Monetary Policy
  Strategy*, "Adopted effective January 24, 2012; as reaffirmed effective
  January 27, 2026",
  <https://www.federalreserve.gov/monetarypolicy/files/FOMC_LongerRunGoals.pdf>,
  read in this run: "The Committee reaffirms its judgment that inflation at the
  rate of 2 percent, as measured by the annual change in the price index for
  personal consumption expenditures, is most consistent over the longer run
  with the Federal Reserve’s statutory maximum employment and price stability
  mandates." A perpetuity growing at expected inflation grows at no real rate
  for ever. That is below the nominal growth of any economy that grows in
  real terms, which is Damodaran's condition on page 27. The discount preset
  is nominal, so the growth beside it is nominal too.
- **`discounted_cash_flow_growth_rate`, `0.02`, the terminal preset.** No
  source fixes a filer's excess growth, and the filings state a past, not a
  forecast. A preset above the terminal rate would credit every filer with
  growth it has not shown. So the preset assumes none: the forecast grows at
  the rate the perpetuity does. Under it, the two stages are one growing
  perpetuity, and the value is F₀ × (1 + g∞) ÷ (r − g∞), whatever n is. A
  user who expects more growth sets it, and the result records it.
- **`discounted_cash_flow_years`, `5`.** No source fixes it, so this is the
  record's own reason. A forecast at the preset looks as far ahead as the
  dividend growth rate's preset window, `dividend_growth_years`, looks back,
  so the forecast span a user starts from is the span over which the history
  shows growth at its preset. At the growth preset, n changes nothing. It
  matters only once a user sets an excess growth.

**The growth rate is a setting alone.** It never reads a rate the derivation
computes. No record forms a growth rate of free cash flow. The dividend growth
rate, which `M5-42` proposes, is the dividend's and not the flow's. A compound
growth rate of free cash flow is undefined wherever the first year's flow is
not positive, which is common. A later record may propose a computed rate as
a second metric. It would not replace this setting.

### The margin of safety

**The form.** The value per share less the close, over the value per share:
(v − P) ÷ v, formed exactly. It is the fraction by which the value could be
too high before the price stops being below it.

- It is at most one for any positive close.
- It is zero where the price equals the value, and negative where the price
  is above it.
- **The origin of the term.** Graham, B., *The Intelligent Investor*, the
  chapter "Margin of Safety as the Central Concept of Investment", not read
  from the workspace. The record cites it for the concept and not for the
  formula. No source read in this run fixes the denominator, so the form
  over the value is the record's own decision, with the reason above. The
  form over the price is argued under Alternatives.

**Where it is stated.** At each year Y where the value per share is stated,
beside the close at the valuation date.

- The analyze record fixes that the valuation date's answer is read from the
  crossing's answer at the valuation date.
- The provider record lists the margin under the valuation date and not under
  the past period ends, which are relative yield's alone.
- So every year's margin is that year's value against today's price. Which
  year a ranking reads is M6's query over the stored table. Nothing here
  orders the years or picks the latest.

**The price read.**

- **No crossing.** `no_price` with `not_asked`.
- **An absence answered for the valuation date.** `no_price` with
  `answered_absent`.
- **No answer for the valuation date.** This cannot arise. `fetch-analyze`
  v1 holds the valuation date as the date asked of `at_valuation_date`, which
  every crossing carries. So `date_not_asked` has no witness for this metric.
  The proof below shows the type admits no such crossing, and shows what a
  crossing asked the valuation date alone gives instead.
- **A price.** The share-basis rule is applied, with E the end of Y and T the
  close's trading date. The value per share is a figure in currency per share
  derived from a share count, so the rule governs it. A failing clause gives
  `basis_unproven` with `splits_absent`, `not_covered` or `split_within`, by
  the first clause that fails, as the analyze record states them.
- **A close outside the decimal grammar.** `declined` with
  `input_not_a_decimal`.
- The margin never falls back to another date, to an adjusted close or to a
  stand-in.

**The value per share is computed where the margin is absent.** It reads no
price. A user with no key sees the value and the reason the margin is missing.

**No threshold.** The margin carries none. A required margin applied to stored
results is a ranking criterion, which is M6's, and not a setting.

**How the metric reaches the crossing** is the implementing task's to decide
inside the crate, under these limits:

- The crossing reaches the derivation only as `analyze` was handed it, and is
  read only through the analyze record's three `no_price` witnesses and the
  share-basis rule.
- The share-basis rule is written once, as the analyze record's Enforcement
  asks, as "the one function every price-dependent metric calls".
- No other metric's reads, reasons or figures change, and what `analyze`
  hands store as `Prices` does not change.

### Which periods

Both metrics are **stated over a year**, under the series record's test.

- The series record makes a metric stated over a year "when its source
  states it over a fiscal year". Damodaran's form does: page 28 reads FCFFₜ as
  "Free Cashflow to firm in year t", and page 27 reads FCFF₁ as "Expected
  FCFF next year".
- Its exponents count years, and every rate is per year. A quarter's or a
  nine-month span's flow grown as a year's would be valued as a year's flow.
- So at a duration that is not a year, each gives exactly one `declined`
  reason, the first of `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year`, and makes no read. That is the series record's rule,
  applied as accepted.
- Its `stated_at` stays `Duration`.

### By kind

`operating_cash_flow`, `capital_expenditure`, both debts,
`cash_and_equivalents` and `diluted_shares_weighted_average` apply to every
kind. `short_term_investments` applies to `operating` alone, and contributes
nothing inside net debt where it is `NotApplicable`.

| Kind | Where every other input is a `Value` |
| :--- | :--- |
| `operating` | computed |
| `reit`, `bank`, `insurer`, `investment_company` | computed, with `short_term_investments` contributing nothing at each instant it is `NotApplicable` |

So no kind is absent by applicability. This is where the method is weakest,
and it is stated here:

- Damodaran's survey, page 15: "estimating capital expenditures and working
  capital for a bank, an investment bank or an insurance company is difficult
  to do." A bank's operating cash flow carries flows that are its business,
  not its reinvestment, and its debt is partly the material it lends.
- For a `bank`, an `insurer` or an `investment_company`, the figure is
  computed and is not a valuation those sources would make. The direction of
  its error is unknown.
- The kind does not cross v2, except through a `NotApplicable`. The analyze
  record says a metric that needs the kind "brings the v3 record that adds
  it". Declining these kinds is that record's, and this record names it.

### The `declined` conditions

Each is named once, as a `const` beside the derivation, and asked only when
every read gives a number. They are asked in formula order, and the first that
holds is the one reason.

| Condition | Holds when | Metric |
| :--- | :--- | :--- |
| `non_positive_free_cash_flow` | F₀ is zero or below | both |
| `discount_rate_not_above_terminal_growth` | r less g∞ is zero or below | both |
| `non_positive_diluted_shares` | `diluted_shares_weighted_average` over Y is zero or below | both |
| `non_positive_value_per_share` | the value per share is zero or below | the margin |
| `non_positive_close` | the close is zero or below | the margin |

**Why a non-positive base declines.** The form grows the base. Grown at a
positive rate, a negative flow becomes a larger outflow each year and for
ever, and the value is that perpetual loss. Damodaran admits negative flows
in the near years of a forecast that later turns positive (page 22). This form
has one base and one growth rate, so a negative base never turns positive, and
no source read here values a business by compounding an outflow for ever. A
zero base gives a value of operations of zero, which is not a valuation
either. So both decline. The cost is that
every filer with a year of heavy investment is absent for that year.

**A negative equity value is a figure.** Where net debt exceeds the value of
operations, the value per share is negative, with its sign. It is defined, and
it says what the derivation found: the claims ahead of equity exceed what the
flows are worth. The margin declines there, under
`non_positive_value_per_share`. Over a negative value, (v − P) ÷ v is above
one for any positive close, which would read as the widest possible margin on
the filer least able to bear one. A zero value is a zero divisor.

**Why a non-positive close declines.** A close is a price paid. Over a close
of zero the margin is one, and over a negative close it is above one: the
most flattering figures the form can give. The test is a sign test, and no
magnitude is read.

The entries also list `start_not_a_date`, `end_not_a_date`, `not_a_year` and
`overlaps_another_year`, as the series record requires, and
`input_not_a_decimal` reaches both from the one definition beside the reader.

### How absence composes

1. **The year test first.** At a duration that is not a year, its one reason
   is the outcome, and nothing below is asked.
2. **The value's reads, in input order.** Eleven reads:
   - `operating_cash_flow` and `capital_expenditure` at Y
   - `short_term_debt`, `long_term_debt`, `cash_and_equivalents` and
     `short_term_investments`, each at Y's opening instant and then its
     closing instant, as net debt is read averaged
   - `diluted_shares_weighted_average` at Y

   Every read is made, and none stops another. Each gives a number or one
   reason: `input_unknown`, `input_not_applicable`, `no_period` with
   `opening_instant` or `closing_instant`, or `declined` with
   `input_not_a_decimal`. The one exception is `short_term_investments`
   `NotApplicable`, which contributes nothing and gives no reason.
3. **The value's own conditions,** in the order above, only where every read
   gave a number.
4. **The margin is built on the value.** It derives the value through the
   value's own derivation, and reads the price beside it. Neither stops the
   other. Where the value is absent, its reasons come first, exactly as the
   value gives them, including its own `declined`. The price's one reason, if
   any, follows. This is how `sustainable_growth_on_common_equity` composes
   the two metrics it is built on. The margin never says "the value was
   absent".
5. **The margin's own conditions,** `non_positive_value_per_share` and then
   `non_positive_close`, only where the value is a figure and the price read
   gave a number.

So the order is the flows, then the balances of net debt as the capital
record expands them, then the share count, then the price. No seventh reason
is needed, and no frozen byte changes.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition:

- the free cash flow module and its form
- the derivation, its eight steps and the terminal form
- the bridge as stated, and the per-share step
- the two names, their formulas, the margin's form and its date
- the five conditions
- the four settings' names, meanings and domains

A later change to any of them, the form, the bridge or the terminal value
included, is a version bump and a superseding record, and never an edit to
this one. A preset change is not a bump.

It lands at version 1. Neither metric is computed yet, so no stored outcome
changes. Moving `dividend_payout_on_free_cash_flow` onto the module changes no
figure.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, with one more value of `unit`:

- **`unit`.** The value per share is currency per share, which the arithmetic
  record names as "currency per one share". The entry's `Unit` gains that
  value, as it gained `Years` with the dividend streak. No field is added.
  The margin is a fraction of one.
- **`stated_at`** is `Duration`.
- **`inputs`**, for both: `operating_cash_flow`, `capital_expenditure`,
  `short_term_debt`, `long_term_debt`, `cash_and_equivalents`,
  `short_term_investments`, `diluted_shares_weighted_average`. The price is
  not a concept and is not listed.
- **`conditions`**: each metric's own from the table, then the four year
  conditions.
- **`meaning`** says the form in words, the years it reads and the settings
  it records. For the value: "free cash flow, operating cash flow less capital
  expenditure, over a year, grown for the forecast years at the forecast
  growth rate and then for ever at the terminal growth rate, discounted at the
  discount rate, less the average of net debt at the instants the year opens
  and closes at, per diluted share weighted over the year; the preferred claim
  and non-controlling interests are not subtracted". The margin's `meaning`
  says it is set against the unadjusted close at the valuation date.

The settings are recorded in every result's premises by name, as every
setting is. A field naming them in the entry would be a second copy.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at`, and a crossing built
as the contract type. They read no file. Each stands in `tests/` or under
`#[cfg(test)]`. Each history holds the year 2024-01-01 to 2024-12-31 and the
instants 2023-12-31 and 2024-12-31, unless the test says otherwise. Balances
are given opening/closing. At the least:

- **The value at the presets, worked by hand.**
  - Operating cash flow 800 and capital expenditure 228 give F₀ = 572.
  - Under the presets, r = 0.0772 and g = g∞ = 0.02, so the value of
    operations is 572 × 1.02 ÷ 0.0572 = 10200, at any n.
  - Short-term debt 100/100, long-term debt 900/1100, cash 300/500 and
    short-term investments 100/100 give net debt 600/600, so the average is
    600.
  - The equity value is 9600. Diluted shares 120 give `80`.
  - The same row with `discounted_cash_flow_years` at 1 and at 10 gives `80`.
- **The value off the presets, worked by hand.** r = 0.1, g = 0.1, n = 2 and
  g∞ = 0.
  - Operating cash flow 150 and capital expenditure 50 give F₀ = 100.
  - The forecast flows are 110 and 121, each discounted to 100. The terminal
    value is 121 ÷ 0.1 = 1210, discounted by 1.21 to 1000. The value of
    operations is 1200.
  - Short-term debt 50/50, long-term debt 300/300, cash 100/100 and
    short-term investments 50/50 give net debt 200. Diluted shares 10 give
    `100`.
- **The exception, and its count,** over the off-preset row:
  - `short_term_investments` `NotApplicable` at both instants: net debt 250,
    `95`.
  - `NotApplicable` at the opening and 50 at the closing: the average of
    short-term investments is 25 over a count of two, net debt 225, `97.5`. It
    is never `100`, which counting one read would give.
  - `Unknown` at the closing: `input_unknown` naming it at 2024-12-31.
- **The margin at a price,** over the preset row, with a crossing whose
  valuation date answers a close and whose earlier dates answer 2024-12-31:
  - A close of 60 gives `0.25`.
  - A close of 100 gives `-0.25`.
- **Each `declined` condition.**
  - Capital expenditure 800 against operating cash flow 800, and 900 against
    it: `non_positive_free_cash_flow`.
  - `discounted_cash_flow_terminal_growth_rate` at 0.0772, and at 0.08,
    beside the preset discount rate: `discount_rate_not_above_terminal_growth`.
  - Diluted shares 0: `non_positive_diluted_shares`.
  - Long-term debt 1300/1300 in the off-preset row: the value is `0`, and the
    margin is `non_positive_value_per_share`. Long-term debt 1400/1400: the
    value is `-10`, and the margin is `non_positive_value_per_share`.
  - A close of 0: `non_positive_close`.
- **No price, by each witness,** with the value `80` beside each:
  - No crossing: `no_price` with `not_asked`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying the date asked.
  - No answer for the valuation date: the test shows that `Crossing` cannot be
    built without `at_valuation_date`. A crossing asked the valuation date
    alone, holding no price at or before 2024-12-31, gives `basis_unproven`
    with `not_covered`, and never `date_not_asked`.
- **The share-basis rule, by each clause:**
  - The split list absent: `splits_absent`.
  - A covered range that starts after 2024-12-31: `not_covered`.
  - A split dated between 2024-12-31 and the trading date: `split_within`.
  - A split dated 2024-12-31 itself: `split_within` too, under the clause's
    "on or after the earlier of E and T".
- **An unreadable close.** `1e2`: `declined` with `input_not_a_decimal`.
- **The order.** `capital_expenditure` `Unknown`, the instant 2023-12-31
  missing, `diluted_shares_weighted_average` `Unknown`, and no crossing. The
  value gives six reasons: `input_unknown` for capital expenditure, then
  `no_period` with `opening_instant` for each of the four balances, then
  `input_unknown` for the shares. The margin gives those six, then `no_price`
  with `not_asked`.
- **A declined value carried into the margin.** F₀ of zero with no crossing:
  the value is `non_positive_free_cash_flow`, and the margin is that reason
  followed by `not_asked`.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31 with every input
  `Unknown` gives `not_a_year` alone, for both.
- **A bank's row.** The off-preset row with `short_term_investments`
  `NotApplicable` at both instants and every other concept a `Value` gives
  `95`, and a margin at a price.
- **The settings.** Each refuses a value outside its domain and never clamps
  it: a discount rate of 0 and of -0.01, a growth rate of -1, zero years, and
  a rate of thirty-five significant digits. Each preset is written back as
  `0.0772`, `0.02`, `5` and `0.02`.
- **Free cash flow.** `dividend_payout_on_free_cash_flow` gives its pinned
  figures unchanged after it reads the module.
- **The catalogue.** Neither entry's `inputs` holds `preferred_equity` or
  `shares_outstanding`, and neither name holds `common`.

### What no check can see

- **Whether the sources say what this record says.** Damodaran's survey, the
  Commission's interpretation, Damodaran's dataset and the Committee's
  statement were opened in this run and are quoted as printed. Graham was not.
  The implementing tasks cite each as the sources record says. A source that
  on reading says otherwise is a stop for that task, not a choice.
- **Net interest counted twice,** as stated above. Nothing measures by how
  much, for any filer.
- **The claims not subtracted:** non-controlling interests, the preferred
  claim and unfunded pensions. Each overstates the value, and nothing measures
  by how much.
- **The average of net debt, where the claim at Y's close is wanted.** The
  direction follows the year's change in net debt.
- **A financial filer's figure.** It is computed, and its error has no known
  direction.
- **One year's flow as the base.** A year of heavy capital expenditure is
  declined or low, and the year after it is high. The value moves with the
  investment cycle, not only with the business.
- **The terminal value's weight.** At the presets the value is one growing
  perpetuity. As r less g∞ narrows, the value grows without bound, and nothing
  caps it. A cap would be a number with no source.
- **The value at Y's close beside a price at T.** The value is not rolled
  forward to the valuation date. Where Y ends after the valuation date, the
  margin compares figures not yet filed at T with the price at T. The caller
  names the valuation date, and nothing checks it.
- **The share count at Y beside a price at T.** Shares issued or bought back
  after Y are not seen. A split is seen, by the share-basis rule.
- **The currency.** The close's currency and the filer's reporting currency
  are not compared, which `price-provider.md` already leaves open.
- **A metric that forms free cash flow itself.** Only review sees it.
- **A stale preset.** The discount preset is January 2026's figure. Nothing
  says when it is moved.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The four
  presets stand in `settings/`, one of its three places. The derivation needs
  only one, through `Number::one`, and every exponent is a count from a
  setting. No constant is added.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule, three places and catalogue
- the arithmetic record's reading, operations and written form
- the period record's readings and seeks, read as written
- the series record's year test, its four conditions and its seek
- the capital record's terms, its module and net debt's one exception
- the provider record's ask and answer
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains a value of `unit` and no field

Two questions here would need an accepted record to change, and each is left
open and named:

- **Which instant the bridge reads net debt at.** The period record's rule
  reads it averaged. Reading it at Y's close is a superseding record of its
  own.
- **The share-basis rule's comparisons of dates.** The rule compares dates,
  "on or before" and "on or after". The series record adds the one ordering to
  `date.rs` and says it "is used by the alone clause and by nothing else". The
  first task to write the share-basis rule meets that sentence. This record
  adds no comparison of its own. Whether the rule may use that ordering, or
  needs a record to say so, is not decided here. If that task reads the
  sentence as forbidding it, it stops.

**Why Structural.** The record fixes the names and meanings of two metrics
that cross `analyze-store` v1, which store and M6 will read. It names four
settings that every result records, and a term that three metrics share. It
needs nothing Constitutional: no anchor, no gate, no protected path and no
GOALS.md scope moves.

## Alternatives

- **A flow to equity discounted at a cost of equity, with no debt in the
  bridge.** Free cash flow after interest is close to Damodaran's free cash
  flow to equity. His page 21 calls Buffett's owner earnings "a measure of
  free cash flow to equity that ignores cash flows from debt". Rejected:
  - The accepted catalogue names an equity bridge and four balances for it,
    and this form reads none of them.
  - The flow ignores net borrowing, so it is not free cash flow to equity
    either. Damodaran's free cash flow to equity subtracts "(New Debt Issued –
    Debt repayments)" (page 20), which v2 does not carry.
  - The cash in the bridge would go unvalued, which understates every
    cash-rich filer.
- **The flow to the firm,** free cash flow plus `interest_expense` times one
  less the effective tax rate, with the bridge. This is Damodaran's form
  exactly, and it removes the double count. Rejected for this version:
  - v2 does not say where each filer classified interest paid. Adding back
    interest the flow never deducted overstates every filer that classified it
    in financing. That is the flattering error, where the chosen form errs
    conservatively for every net borrower.
  - `interest_expense` is the charge, not the cash paid, and its silence is
    `Unknown`.
  - The effective tax rate declines at every pretax loss and outside zero to
    one, under the capital record. The discounted cash flow would be absent
    in every such year.
  - A record that brings the classification across a contract can supersede
    this one and bump the method.
- **A terminal value by an exit multiple.** Rejected: a multiple is a price
  of comparable firms. It needs market data the provider does not supply, and
  it would make the value depend on price, so the margin would compare price
  with price.
- **No terminal value,** with a forecast alone. Rejected: it values a going
  concern as if it stopped after n years, and makes n the largest driver of
  the value.
- **The forecast growth computed from the history,** such as the compound
  growth of free cash flow over the series. Rejected here: no record forms it,
  it is undefined wherever the first year is not positive, and free cash flow
  is lumpy. It is not ruled out as a later metric.
- **The forecast growth preset above the terminal rate,** such as five
  percent. Rejected: no source fixes it, and it credits every filer with
  growth it has not shown.
- **The margin over the price,** (v − P) ÷ P. It reads as the gain if the
  price rose to the value, and it stays defined over a negative value.
  Rejected: it is not bounded, and it answers how much the price could rise
  rather than how wrong the value could be. Over the value, the margin is the
  share of the estimate that can be lost before the purchase is no bargain,
  which is what a margin of safety is for.
- **The margin as the price over the value,** P ÷ v. It is one minus the form
  chosen. Rejected: one form is enough, and a fraction that is better when
  smaller would invert every reading of "margin".
- **A base averaged over several years,** found by the series seek. This
  smooths a year of heavy investment. Not rejected, but not here: it needs a
  fifth setting with a reason, a gap anywhere in the window makes the value
  absent, and the window's reasons multiply. A superseding record can argue
  it.
- **Net debt at Y's close.** This is the reading the bridge wants. Not
  rejected on its merits: it needs the period record changed, which is a
  record of its own, as stated above.
- **Subtracting `preferred_equity`.** Rejected here: its silence is `Unknown`,
  so it would make the value absent for most filers, which have no preferred,
  and the common-shareholder record makes another consumer of it "its own
  record".
- **Declining the four non-operating kinds through `short_term_investments`'
  `NotApplicable`.** Rejected: the accepted exception makes that
  `NotApplicable` contribute nothing inside net debt, and net debt is read
  from the module as accepted. Reading a balance's applicability as the
  filer's kind is what the analyze record says needs a v3.
- **`shares_outstanding` for the per-share step.** It is the count at an
  instant near the period end, closer to the shares a buyer would hold.
  Rejected: the catalogue names the diluted count for this step, and the
  instant count leaves out the options and convertibles that share in the
  value.
- **The relation r > g∞ as a joint domain the constructor refuses.**
  Rejected: the setters are independent, and a joint domain makes their
  order matter.
- **A negative base valued as a signed figure.** Rejected: growing an outflow
  at a positive rate is not a valuation any source read here makes.
- **The margin at each year against that year's close,** as relative yield
  reads past closes. Rejected: the provider record lists the margin under the
  valuation date, and a past margin would answer a question no one buying
  today asks.
- **Leaving the discounted cash flow out of M5 until the provider is built.**
  Rejected: the value per share reads no price, and the margin's absence
  without one is fixed by accepted records. A test builds the crossing as the
  contract type, so `no_price` is provable today. Deferring would also change
  milestone scope, which is Constitutional.
- **Each metric forming free cash flow itself.** Rejected: three readings of
  one subtraction, which the one-source-of-truth invariant forbids.

## Consequences

**Easier:**

- The discounted cash flow has its form, settings, presets and sources. Each
  implementing task has only its code left to decide.
- Free cash flow is formed once, and the price/free-cash-flow multiple reads it
  there.
- The first metric to read the crossing fixes how a price-dependent metric is
  absent, and the share-basis rule gets its one function.
- A user with no key sees the value per share and why the margin is missing.

**Harder:**

- **It needs seven concepts at eleven reads,** both instants of Y and four
  balances at each. Both debts read silence as `Unknown`, so a debt-free filer
  with no override is absent.
- **Every year of negative or zero free cash flow is absent.** So is every
  quarter.
- **The value is overstated** where non-controlling interests, preferred
  stock or unfunded pensions exist, and where cash earns more than debt
  costs. It is understated for every net borrower, by its net interest.
- **Financial filers get a figure** that their sources would not compute.
- **The presets value every filer as growing at inflation,** so the margin at
  the presets is conservative for a growing business.

**Expensive to reverse:**

- The two names, their forms and the margin's denominator, once results are
  stored and ranked on.
- The free cash flow module, once three metrics read it.

**Open, and not this record's:**

- Which instant the bridge reads net debt at.
- Whether the share-basis rule may use the ordering the series record added.
- A concept for non-controlling interests, and a reading of the preferred
  claim that does not cost every silent filer.
- The filer's kind in v3, so a financial filer can be declined.
- Where interest paid is classified, so the flow to the firm can be formed.
- The dividend discount valuation and its margin, which the provider record
  pairs with this one. That is a record of its own.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **Every free parameter is a setting.** The four stand in `settings/` with
  their reasons, and `literals` keeps every number but zero and one out of the
  derivation.
  - **Gap:** nothing stops the derivation calling the preset constructor, as
    the analyze record already says.
  - **Gap:** nothing checks that a citation is true, or that a preset still
    matches its source.
- **One definition of free cash flow.** The payout's pinned figures hold it to
  the module.
  - **Gap:** nothing forces a metric through the module rather than around
    it.
- **No wrong number across a split.** The share-basis tests hold it for the
  margin.
- **No margin without a price.** The witness tests hold it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the names and
  meanings of two metrics that cross `analyze-store` v1 as characters, names
  four settings that every result records, forms free cash flow once for the
  three metrics that read it, and makes the margin the first reader of the
  crossing, all inside analyze. It edits no anchor, no protected path, no
  gate, no contract byte, no schema and no milestone scope, and asks for no
  allowlist entry. Flagged for later human review, as the tier requires.
- **Checked:**
  - `what-analyze-takes-and-returns.md`: a setting is a value "neither the
    filings nor the metric's cited source fixes", and "the discount rate,
    terminal growth and forecast horizon" are its examples; the six reasons
    stand and no seventh is added; the three `no_price` witnesses and the
    three `basis_unproven` witnesses are named as the record names them; the
    share-basis rule is "the one function every price-dependent metric
    calls".
  - `fetch-analyze` v1 as frozen: `Crossing` carries `at_valuation_date` as
    an `Answer` that every crossing holds, so the record's argument that
    `date_not_asked` has no witness for this metric holds against the
    contract, and the test it substitutes shows what the type gives instead.
  - `how-analyze-computes-and-writes-a-figure.md`: integer powers are what
    "discounting uses", a per-share figure "is currency per one share", and
    every numeric setting is held to `SIGNIFICANT_DIGITS`. Every step of the
    derivation is an operation in its table.
  - `which-other-period-a-metric-reads.md`: "Every other metric that pairs a
    flow with a balance reads the average. This holds whatever source its
    formula cites." The record reads that as written, averages net debt, and
    names the bridge's want of the closing claim as a question for a record
    of its own rather than reading the rule narrowly.
  - `which-duration-precedes-a-duration.md`: a metric is stated over a year
    "when its source states it over a fiscal year"; `dividend_growth_years`
    is its window setting, which the forecast preset's reason leans on; the
    ordering it adds "is used by the alone clause and by nothing else", which
    the record names as the second open question and does not resolve.
  - `capital-employed-invested-capital-and-nopat.md`: net debt is formed once
    and the bridge is a named consumer that reads "it here"; the record reads
    no `preferred_equity` and leaves non-controlling interests to a v3, both
    with their direction, which flatters.
  - `common-shareholder-forms.md`: `common` "exactly when", `per_share`
    without that word; the record's reading that the rule places `per_share`
    and does not forbid it is a fair reading of the sentence, and no catalogue
    test on main asserts otherwise. Another consumer of `preferred_equity` is
    "its own record", and this one reads neither preferred concept.
  - `canonical-concepts.md` and its open-questions record:
    `capital_expenditure` silence is `Unknown` because reading it as zero
    "would overstate free cash flow by the whole of it"; `operating_cash_flow`
    is after interest "where the filer classifies them there";
    `short_term_investments` inside net debt is the one `NotApplicable` that
    contributes nothing. The record carries each and extends none.
  - `price-provider.md`: the margin is listed under the valuation date, as a
    metric that "compares a per-share value with the price"; the record's
    Context quotes it as "comparing", the task's own paraphrase.
  - `bare-literal-lint.md`: no place, form or allowlist entry moves; the four
    presets stand in `settings/` and the derivation needs `Number::one` alone.
    The catalogue's `Unit` gained `Years` with the dividend streak, so a
    currency-per-share value is the same kind of addition and no field.
  - The four open sources, each opened in this run and quoted as printed:
    Damodaran's survey at the pages cited; his cost-of-capital table, "Data
    used is as of January 2026", the row "Total Market (without financials)"
    giving 7.72%; the FOMC statement "as reaffirmed effective January 27,
    2026"; and C&DI 102.07. Graham and ASC 230 are marked not read.
  - The hand-worked figures in the proof section, each reproduced: `80` at
    the presets at any n, `100` off them, `95` and `97.5` under the
    exception's count of two, `0.25` and `-0.25`, `0` and `-10`, and the
    six-then-seven reason order.
  - The alternatives as argued: the flow to equity, the flow to the firm, an
    exit multiple, no terminal value, a computed forecast rate, a preset above
    the terminal rate, the margin over the price and as price over value, an
    averaged base, net debt at the close, subtracting the preferred, declining
    the four kinds through the exception, `shares_outstanding`, a joint
    domain, a signed negative base, a past margin, leaving the metric out of
    M5, and each metric forming free cash flow itself.
- **Verdict and why:** accepted. The discounted cash flow has more free
  parameters than any metric before it, and the record puts each one in the
  settings layer with a preset whose reason is stated and, where a source
  exists, cited to a copy opened in the run. The two presets that lean on no
  source, the forecast rate at the terminal rate and the five years, assume
  the least: at the presets the value is one growing perpetuity and n changes
  nothing, so a user who has set nothing is not credited with growth the
  filings do not show. Free cash flow is formed once before a third reader
  exists. Where the method errs, the record says where and in which direction
  rather than correcting with a number it has no source for: net interest
  counted twice, the claims not subtracted, the averaged bridge, the
  financial kinds. And where the right reading needs an accepted record
  changed, it names the question and leaves it, as the capital record did.
- **What would have changed it:** a seventh reason; a changed byte of
  `fetch-analyze` v1 or `analyze-store` v1; a preset with no stated reason or
  a derivation reading a value the handed `Settings` does not carry; a
  constructor that clamps; a terminal value with no `declined` condition at
  r not above g∞; a bridge reading net debt at the close in silence against
  the period record; a read of `preferred_equity`; a cap on the terminal
  value, which would be a number with no source; or a source that on reading
  forms the perpetuity or the bridge otherwise than stated, which the record
  makes a stop for the implementing task. Flagged for the human beside the
  tier: the averaged bridge and the ordering the share-basis rule needs,
  the two open questions; the double count of net interest, which errs
  conservatively for a net borrower and flatters a net lender; a computed
  figure for a `bank`, an `insurer` or an `investment_company` with no known
  direction; and the discount preset as January 2026's figure, with nothing
  saying when it is moved.
