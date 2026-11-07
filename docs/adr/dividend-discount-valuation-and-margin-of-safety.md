# The dividend discount value per share is the mean of `dividend_discount_base_years` years' declared dividends, grown once at a growth setting and divided by a required return less that rate; its margin of safety is that value less the valuation date's close, as a fraction of the value

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-11-03, by `M5-56`
- **Decided:** 2026-11-06, by the decider
- **Touches:** the dividend-based valuation that GOALS.md's fifth M5 criterion
  names, and that `discounted-cash-flow-and-margin-of-safety.md` (Accepted) and
  `dividend-growth-rate.md` (Accepted) each leave to a record of its own. It
  fixes the names and meanings of two metrics that cross `analyze-store` v1 as
  characters, and no byte of that contract changes. It names three settings,
  each defined once under the analyze record's definition. It applies the
  accepted `declined` reason to one new condition, reaches three existing ones
  from their one definitions, and adds no reason. It reads the price as the
  analyze record and `share-basis-rule-dates.md` (Accepted) fix. It takes every
  operation from the arithmetic record's table. It names one exposure no
  accepted rule sees, a split inside the window, and leaves it open. No
  accepted record is superseded. No contract's bytes, no schema, no edge, no
  gate and no dependency move. Nothing here is implemented.

## Context

GOALS.md's fifth M5 criterion ends with "dividend-based valuation". Every
record around it is accepted and none forms it.

Some things are already fixed, and this record restates them as fixed:

- **The input.** `canonical-concepts.md` names `dividends_declared_per_share`
  "dividends declared per common share for the period", a flow applying to all
  kinds whose silence is zero, consumed by "the dividend discount valuation".
- **The constraint.** `canonical-concepts-open-questions.md`, answer 6, reads
  that concept as the total, regular and special together, and says "the
  dividend discount valuation must not take one period's declared total as a
  perpetuity — its input is a series, which storage holds". It gives the
  direction: a model "fed a special-dividend year values a one-off as though it
  repeated forever, which overstates value — the flattering direction".
- **The growth rate's warning.** `dividend-growth-rate.md` says of its rate, "A
  rate moved by one period's total and carried into a perpetuity takes that
  excess there through the rate."
- **The series.** `which-duration-precedes-a-duration.md` fixes what a year is,
  the seek, the gap rule, the four conditions at a duration that is not one,
  and that a metric is stated over a year "when its source states it over a
  fiscal year".
- **The price.** `price-provider.md` lists under the valuation date "the margin
  of safety in the discounted cash flow and in the dividend discount valuation,
  each of which compares a per-share value with the price".
  `what-analyze-takes-and-returns.md` fixes how a price-dependent metric reads
  the crossing and the share-basis rule. `share-basis-rule-dates.md` fixes the
  rule's comparisons and the condition `share_basis_date_not_a_date`, listed by
  every metric that calls the rule.
- **The names.** `common-shareholder-forms.md` fixes that `per_share` "appears
  in the name of a form that reads per-common-share concepts" and `common`
  "exactly when its derivation forms common equity or income available to
  common".
- **The margin's pair.** The discounted cash flow record fixes a margin's form
  and argues its denominator. `M5-54` and `M5-55` write the rule, the close's
  reading and that margin. This record reads them as those tasks state them.
- **Absence is closed.** Six reasons. A metric built on another carries the
  other's reasons.

## Decision

### Two metrics

| Name | What it is | Unit |
| :--- | :--- | :--- |
| `dividend_discount_value_per_share` | The value per common share the derivation below arrives at | currency per share |
| `dividend_discount_margin_of_safety` | That value less the close at the valuation date, over that value | fraction of one |

Both are stated over a year. The first reads no price. The second is built on
the first and reads one.

### The derivation, in formula order

Let Y be the year the metric is computed at. The settings are n, the base
years; g, the growth rate; and r, the required return (below). Every step is an
operation in the arithmetic record's table.

| Step | What it is | Operations |
| :--- | :--- | :--- |
| 1. The series | D₁ … Dₙ, `dividends_declared_per_share` at Y and at the n − 1 years before it | none: the seek and the reading |
| 2. The base dividend | B = (D₁ + … + Dₙ) ÷ n | addition, division |
| 3. The dividend expected one year on | E = B × (1 + g) | addition, multiplication |
| 4. The value per common share | V = E ÷ (r − g) | subtraction, division |

**The form is the stable-growth form.** Damodaran's two texts state it: the
dividend expected one year on over the required return less the growth rate.
The chapter's illustration, page 6, forms the dividend expected one year on as
the base grown once, "($2.19)(1.0349)", and step 3 is that. Neither text states
a rule for choosing the base from a history, which is the next section.

**A two-stage form is not taken.** It would add a forecast growth rate and a
horizon, and no source fixes either for a filer. Under the discounted cash flow
record's presets its two stages are already one perpetuity, so at a preset that
credits no excess growth, the two forms give one value. The price is stated:
The chapter's page 4 confines this form to "firms growing at a rate comparable
to or lower than the nominal growth in the economy", and the figure for a filer
growing faster is not a valuation he would make. A user who wants the excess
growth sets the growth rate, or a later record proposes the two-stage form.

**Undefined where r is not above g.** At equality the divisor is zero. Below
it, the chapter's page 4 says "the value per share becomes negative", and its
page 3 says that "as the growth rate converges on the discount rate, the value
goes to infinity". That is the one `declined` condition
`required_return_not_above_growth_rate`, asked at step 4 as a sign test on r
less g. It is not a joint domain, for the reason the discounted cash flow
record gives for its pair, adopted by name: each setter sees one value, and a
domain on the pair would make the order of the two setters matter.

**Nothing caps the value.** As r less g narrows the value grows without bound.
A cap would be a number with no source, as the discounted cash flow record says
of its own.

### The base dividend

**The base is the mean of the n years' declared totals.** Each year is reached
through `Year::series`, the seek repeated. Each is read. Each counts once. n is
the setting `dividend_discount_base_years`, and its domain starts at two. The
base form and the window are part of the method version; the preset is not.

**Why not the computing year alone.** Answer 6 says the valuation "must not
take one period's declared total as a perpetuity". A base of Y alone is that
sentence's case, with the whole of an excess carried for ever in the flattering
direction.

**What the mean does with an excess.** Let the regular dividend be R, and let
one year's total carry a special of e × R, with e above zero.

| Base | What the excess does |
| :--- | :--- |
| Y alone, at the year carrying it | R × (1 + e): the whole excess, in the one value computed at that year, and none at any other |
| the mean of n years | R × (1 + e ÷ n): one n-th of it, the same wherever in the window it stands, in each of the n values whose window holds it |

So the mean does not remove an excess. It divides it by n and holds it for n
years. The direction stays flattering. At the preset, a special equal to the
regular dividend lifts the base by a fifth, for five years, against the whole
of it for one. Which of the two forms costs more at the year M6 reads depends
on the year, and the record chooses the one answer 6's sentence allows.
Removing it needs the dimensional split, which answer 6 says does not cross.

**The mean never asks which year is special.** It reads every year the same
way, and no figure is compared with another. So it is not the inference from
magnitude answer 6 refuses, and not what `dividend-growth-rate.md` refuses
beside it: "capping a year's change, trimming the largest figure, or reading a
year against its neighbours". That sentence is why the lower of the years is
rejected below, though it would hold an excess out altogether.

**What it costs a grower.** The mean lags Y. For a dividend growing steadily at
γ, at n of 5, B is 0.9615 of Y's dividend at γ of 2% and 0.8340 at γ of 10%.
This understates, which is the conservative direction. Rolling the mean forward
to Y would take a power of (n − 1) ÷ 2, which at an even n is a real power
outside the table.

**What a year inside the window gives.**

- **A zero.** It is a number, and counts. The base falls by a share of the
  dividend it would have held. That holds whether the zero was read or set by
  silence, as the series record says of a zero. A filer that began paying two
  years ago has a base holding three zeros at the preset, and is understated. A
  wrong silence zero lowers the value and never raises it, the direction the
  catalogue requires. A zero at Y is the condition below.
- **`Unknown`.** `input_unknown` naming the concept and that year, under the
  analyze record's "Nothing is substituted". The metric is absent at every year
  whose window holds it. In the suspension fixture, 2023's `Unknown` makes the
  value absent from 2023 through 2027 at the preset. That is the same cost the
  discounted cash flow record names for an averaged base, and here the record
  pays it because answer 6 leaves no cheaper form.
- **A gap.** `no_period` with `preceding_year`, for that year and for every
  read beyond it, under the series record's gap rule. Identical reasons are not
  merged, as that record says. The window is never shortened and never closed
  up, and no year stands in for another.
- **A quarter** never enters: every member of the series is a year by the test.

**No source fixes this base.** Damodaran's chapter takes "Dividends per share
in 2000" for its illustration and states no rule for a special dividend. So the
mean is the record's own decision, with the reasons above. The nearest passage
in the copy is page 15, of buybacks, whose ratio "for any one year can be
skewed by the fact that stock buybacks, unlike dividends, are not smoothed
out", so that "a much better estimate of the modified payout ratio can be
obtained by looking at the average value over a four or five year period". He
says it of another flow, and it is cited for the reasoning only.

### The growth rate is a setting alone

The derivation reads `dividend_discount_growth_rate` from the `Settings` it is
handed. It never reads the computed `dividend_growth_rate`. So nothing here
gives a reason "where the rate is absent", and a computed rate at or above the
required return does not arise. Three reasons:

- **The warning.** `dividend_growth_rate` reads two declared totals, so an
  excess at the computing year raises it, and the mean above would carry the
  same excess through a second door. The perpetuity would take it twice: in B
  by a fifth, and in g through the fifth root, which at a special equal to the
  regular dividend moves the growth factor by about 1.149, on that record's
  working. Over a required return near 8%, that alone can reach the `declined`
  condition.
- **What the form asks.** Damodaran's g is the "Growth rate in dividends
  forever", and on the survey's page 11 "it cannot exceed the growth rate of
  the economy in which the firm operates". A compound rate over five past years
  is not that, and a filer's five-year rate above the required return is
  ordinary. It would be absent, or near the infinity page 3 warns of, by no
  decision of the user's.
- **Absence.** It would make the value absent wherever the rate is, at a zero
  base year and at n + 1 years' reach, for an input the form does not need.

A later record may propose the computed rate as a second metric. It would not
replace this setting.

### The settings

Three settings, defined once in
`crates/analyze/src/settings/dividend_discount.rs`, each a field of the one
`Settings`. The derivation reads no setting except from the `Settings` it is
handed, and never calls the preset constructor. A value outside a domain is
refused by the constructor as the caller's error, before analyze runs, and
never clamped. A rate's domain also admits only values of at most
`SIGNIFICANT_DIGITS` significant digits, as the arithmetic record requires, and
each value is written by the one writer.

| Name | Meaning | Unit | Domain | Preset |
| :--- | :--- | :--- | :--- | :--- |
| `dividend_discount_required_return` | The annual rate equity investors require of the share, which the perpetuity is divided by after the growth rate is taken from it | a rate per year, a fraction of one | rates above zero | `0.0802` |
| `dividend_discount_growth_rate` | The annual rate the dividend grows at for ever, from the base | a rate per year, a fraction of one | rates above minus one | `0.02` |
| `dividend_discount_base_years` | How many years the base dividend is the mean of, ending at the year computed at | years | the integers from two up | `5` |

**Why each domain.**

- **The required return is above zero.** At zero or below, a later dividend is
  worth at least an earlier one, which is no required return. This is the
  discount rate's argument in the discounted cash flow record.
- **The growth rate is above minus one.** At minus one every later dividend is
  zero, and below it they alternate in sign. Neither is growth.
- **The base years start at two.** One year is the base answer 6 forbids.
  Refusing it holds the sentence against the settings as well as the preset.
  The two is a literal in `settings/`, one of the three places, and it is the
  domain's and not the derivation's.
- **The base years' type holds the list of reasons.** Past a gap, each read
  gives a reason, so a window's length is the length of an absence's list. Its
  type is no wider than `u16`, so a value that could not be listed cannot be
  handed. No literal stands for this bound, and no cap is argued from a source:
  the type is the ceiling, as `dividend-growth-rate.md` lets a setting's type
  bound its value. The implementing task reads n − 1 back through
  `Year::series` without overflow.

**The presets and their reasons.**

- **`dividend_discount_required_return`, `0.0802`.** The cost of equity of the
  United States market, financial firms included, as Damodaran publishes it:
  *Cost of Equity and Capital (US)*, "Data used is as of January 2026",
  <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/wacc.html>,
  opened in this run. The column "Cost of Equity" reads 8.02% at the row "Total
  Market".
  - **Why the column.** The form discounts a flow to equity, and its ke is the
    "Required rate of return for equity investors". The discounted cash flow
    record's cost of capital serves a flow to the firm.
  - **Why this row, and not the row without financials at 8.37%.** The
    discounted cash flow record took the row without financials because its
    form is the one Damodaran says is hard to apply to financial firms, citing
    the survey's page 15. That page says the opposite of this form: "There are two
    reasons why dividend discount model remain widely used to value financial
    service companies. The first is that estimating capital expenditures and
    working capital for a bank, an investment bank or an insurance company is
    difficult to do." This metric is computed for every kind, financial filers
    among them, so the row holding them is the row of the filers valued.
  - **The direction.** The lower row gives a higher value. At the growth
    preset, a base is multiplied by 16.94 rather than 16.01, a value 5.8%
    higher. A user who wants the other row sets it, and the result records it.
  - **What it is not.** It is the market's rate, not the filer's. Damodaran's
    illustration builds Con Edison's from a beta, a riskless rate and a risk
    premium, and no filing carries any of them. The source republishes each
    January. Moving the preset is a preset change and not a version bump.
- **`dividend_discount_growth_rate`, `0.02`.** The Federal Open Market
  Committee, *Statement on Longer-Run Goals and Monetary Policy Strategy*,
  "Adopted effective January 24, 2012; as reaffirmed effective January 27,
  2026",
  <https://www.federalreserve.gov/monetarypolicy/files/FOMC_LongerRunGoals.pdf>,
  opened in this run: "The Committee reaffirms its judgment that inflation at
  the rate of 2 percent, as measured by the annual change in the price index
  for personal consumption expenditures, is most consistent over the longer run
  with the Federal Reserve’s statutory maximum employment and price stability
  mandates." A dividend growing at expected inflation holds its purchasing
  power for ever and grows at no real rate. That is below the nominal growth of
  any economy that grows in real terms, which is the survey's page 11 limit.
  The required return's preset is nominal, so the growth beside it is too. The
  preset credits no filer with growth it has not shown, and a user who expects
  more sets it. It is this setting's own value. It stands in this method's
  file, and it is not read from `discounted_cash_flow_terminal_growth_rate`,
  which is the rate of a firm's flow after a forecast. Moving one does not move
  the other.
- **`dividend_discount_base_years`, `5`.** No source fixes it, so this is the
  record's own reason, in the series record's words for
  `dividend_growth_years`: "Five annual steps hold five dividend decisions",
  and "six fiscal years are inside the structured history of any filer that has
  filed XBRL since the 2011 phase-in", so five are. One year's special then
  moves the base by a fifth of itself. Page 15's "four or five year period"
  stands near it, as stated above. It is a setting of its own and not
  `dividend_growth_years`, which counts n steps across n + 1 years and reads
  two of them. A shorter window narrows the split exposure below and widens the
  excess. That trade is a preset change, not a version bump.

### The margin of safety

**The form.** The value per share less the close, over the value per share: (V
− P) ÷ V, formed exactly. This is the discounted cash flow margin's form, and
that record's argument is adopted by name: over the value, the margin is the
share of the estimate that can be lost before the purchase is no bargain. No
source read in this run fixes the denominator, as that record says. The origin
of the term stands as it cites it: Graham, B., *The Intelligent Investor*, the
chapter "Margin of Safety as the Central Concept of Investment", not read from
the workspace, for the concept and not the formula. The form is written once,
where `M5-55` writes it, and not a second time.

**Where it is stated.** At each year where the value per share is stated,
beside the close at the valuation date, as the provider record places it. Every
year's margin is that year's value against today's price. Which year a ranking
reads is M6's query, and nothing here orders the years or picks the latest.

**The price read.** Through the one function `M5-54` writes, with E the end of
Y:

- **No crossing.** `no_price` with `not_asked`.
- **An absence answered for the valuation date.** `no_price` with
  `answered_absent`, carrying the date asked.
- **No answer for the valuation date** cannot arise. The answer is read by its
  place, `at_valuation_date`, which every crossing carries, so `date_not_asked`
  has no witness here, as the discounted cash flow record says of its margin.
- **A price.** It goes through the one share-basis rule function, with E the
  end of Y and T the close's trading date, because the value is derived from a
  figure in currency per share. A failing clause gives `basis_unproven` with
  `splits_absent`, `not_covered` or `split_within`, by the first that fails. A
  date the rule compares outside the grammar gives `declined` with
  `share_basis_date_not_a_date`.
- **A close outside the decimal grammar.** `declined` with
  `input_not_a_decimal`.
- It never falls back to another date, to an adjusted close or to a stand-in.

**The value per share reads no price and is computed where the margin is
absent.** A user with no key sees the value and the reason the margin is
missing.

**No threshold.** The margin carries none. A required margin applied to stored
results is a ranking criterion, which is M6's, and not a setting.

**Its two conditions.** The discounted cash flow margin has two, and this
margin takes one:

- **`non_positive_close`** is that margin's condition, reached from its one
  definition. Over a close of zero the margin is one, and over a negative close
  it is above one, the most flattering figures the form gives.
- **`non_positive_value_per_share` cannot arise, and is not listed.** Where the
  conditions below have passed, D₁ is above zero and no year read is negative,
  so B is above zero. 1 + g is above zero by its domain and r − g by the
  condition, so V is above zero, exactly, before anything is written.
  `dividend-growth-rate.md` says why a condition that cannot be given is left
  out: "A condition the entry lists and the metric never gives tells a reader
  something false." The implementing task treats a non-positive V at the margin
  as unreachable.

### Which periods

Both metrics are **stated over a year**, under the series record's test. The
source states the form over one: DPS₁ is "Expected Dividends one year from now
(next period)", and a required return and a growth rate are each per year. A
quarter's total grown as a year's would be valued as a year's, and the
suspension fixture's quarter 2021-10-01 to 2021-12-31 reads 0 by silence inside
a year that paid 0.15. So at a duration that is not a year, each gives exactly
one `declined` reason, the first of `start_not_a_date`, `end_not_a_date`,
`not_a_year` and `overlaps_another_year`, and makes no read. Its `stated_at`
stays `Duration`.

### By kind

`dividends_declared_per_share` applies to every kind, so no kind is absent by
applicability.

| Kind | Where every read is a `Value` | What the sources say |
| :--- | :--- | :--- |
| `operating` | computed | the chapter's page 4: best suited to "well established dividend payout policies"; its page 5: "will under estimate the value of the stock in firms that consistently pay out less than they can afford" |
| `bank`, `insurer` | computed | the survey's page 15: the form is "widely used" for them, unlike the discounted cash flow's |
| `reit` | computed | the chapter's pages 7 and 8 value one by this form, since "REITs are required to return at least 95% of their earnings as dividends" |
| `investment_company` | computed | no source opened in this run speaks to it, and the direction of the error is not known |

The kind does not cross v2. A metric that needs it brings the v3 record, as the
analyze record says. This one does not need it.

### The `declined` conditions

Each is asked only when every read gives a number, in the order below, and the
first that holds is the one reason.

| Condition | Holds when | Metric | Defined |
| :--- | :--- | :--- | :--- |
| `negative_dividends_declared_per_share` | a year read declared a negative dividend per common share | both | once, beside the streak; reached from there |
| `zero_dividends_declared_per_share` | the figure at Y is zero | both | once, beside `dividend_coverage_per_share`; reached from there |
| `required_return_not_above_growth_rate` | r less g is zero or below | both | new, once, beside this metric |
| `non_positive_close` | the close is zero or below | the margin | once, beside the discounted cash flow margin; reached from there |

**A negative figure** at any year read gives the streak's condition, with the
same characters and meaning. A mean over a negative year would state a figure
the concept's meaning does not. It is asked first, as the growth rate asks it,
because it makes the definition undefined whatever else holds.

**A zero at Y is a condition, and not a figure of zero.**

- **The mean alone would flatter a suspension.** A filer that paid for four
  years and stopped at Y has a positive B, and the form would value a dividend
  that is not being declared as if it went on for ever at four fifths of
  itself. The chapter's page 4 confines the form to payout policies "that they
  intend to continue into the future". A year that declares nothing does not
  show one.
- **A non-payer has no value under this form.** Its window is all zeros, so
  step 4's formula would give `0`. The discounted cash flow record declines a
  zero base because it "is not a valuation either", and the same holds here.
  The survey's page 15 says a filer paying nothing "can still be valued based
  upon dividends that it is expected to pay out when the growth rate declines",
  which is a payout this form does not have. A `0` would rank every non-payer
  beneath every payer, as if it were worth nothing.
- **It hides nothing.** `dividend-growth-rate.md` rejects a condition at a
  computing-year zero because the formula is defined there and "an absence
  would hide a suspension". Here the outcome names the zero, and the streak
  shows `0` and the rate shows `-1` at the same year. A wrong silence zero
  gives an absence, which does not flatter.
- **The characters are `dividend_coverage_per_share`'s.** The growth rate
  record states what that condition means: "a zero at the year the metric is
  stated at". This is that. The implementing task widens the constant's
  visibility and defines no second one.

So no zero value arises, and neither metric declines on one.

**A zero at a year other than Y** is no condition. It counts in the mean, as
stated above.

The entries also list the four year conditions, as the series record requires,
and `input_not_a_decimal` reaches both from the one definition beside the
reader. The margin's entry lists `share_basis_date_not_a_date` after the four
year conditions, as the share-basis record places it. The value's does not,
since it calls no rule.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first.** At a duration that is not a year, its one reason is
   the outcome and nothing below is asked.
2. **The value's reads, in series order.** `dividends_declared_per_share` at Y,
   then at each year before it through the n-th. Every read is made, and none
   stops another. Each gives a number or one reason: `input_unknown` or
   `input_not_applicable` naming the concept and the found year, `no_period`
   with `preceding_year` and `computing_at` Y, or `declined` with
   `input_not_a_decimal`.
3. **The value's own conditions,** in the order above, only where every read
   gave a number.
4. **The margin is built on the value.** It derives the value through the
   value's own derivation and reads the price beside it, and neither stops the
   other. The value's reasons come first, exactly as the value gives them, its
   own `declined` included. The price read's one reason, if any, follows. The
   margin never says the value was absent. This is how
   `discounted_cash_flow_margin_of_safety` composes.
5. **The margin's own condition,** `non_positive_close`, only where the value
   is a figure and the price read gave a number.

So the order is the dividend reads, then the price. There is no rate read,
because the growth is a setting. No seventh reason is needed, and no frozen
byte changes.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, and no new value of `unit`:

- **`unit`.** `CurrencyPerShare` for the value, as the discounted cash flow's.
  `FractionOfOne` for the margin.
- **`stated_at`** is `Duration`.
- **`inputs`**, for both: `dividends_declared_per_share`, once. The price is
  not a concept and is not listed.
- **`conditions`**, for the value: `negative_dividends_declared_per_share`,
  `zero_dividends_declared_per_share`, `required_return_not_above_growth_rate`,
  then the four year conditions. For the margin: those three,
  `non_positive_close`, the four year conditions, then
  `share_basis_date_not_a_date`. For both, `input_not_a_decimal` is generated
  from its one definition.
- **`meaning`**, for the value, says the form in words: "The value per common
  share: the mean of the dividends declared per common share, regular and
  special together, over the dividend_discount_base_years years ending at this
  year, grown once at dividend_discount_growth_rate, over
  dividend_discount_required_return less that rate." The margin's says it is
  that value less the unadjusted close at the valuation date, over that value.

**The names.** The value reads a per-common-share concept, so its name carries
`per_share` and its meaning says "per common share". Neither metric forms
common equity or income available to common, so neither name carries `common`.
The margin is named for the value it is built on, as
`discounted_cash_flow_margin_of_safety` is.

The settings are recorded in every result's premises by name, as every setting
is. A field naming them in the entry would be a second copy.

### The method version

These are part of the method `METHOD_VERSION` names:

- the form, the base dividend's mean and its window, the setting-alone growth
  read
- the two names, their formulas, the margin's form and its date
- the conditions and their order, and a zero at Y as a condition
- the three settings' names, meanings and domains

A later change to any of them, the form, the base dividend or the growth read
included, is a version bump and a superseding record, and never an edit to this
one. A preset change is not a bump.

It lands at version 1. Neither metric is computed yet, so no stored outcome
changes, and adding a metric is not a bump.

### Sources

Under `sources-a-task-can-read.md`. Each copy below was opened in this run and
each quotation is as printed.

**The origins, not read from the workspace.** Gordon, M.J., *The Investment,
Financing and Valuation of the Corporation*, Richard D. Irwin, 1962, for the
form; and Williams, J.B., *The Theory of Investment Value*, 1938, for the
present value of dividends. Each is named by the work, as the open texts name
them.

**Damodaran, A., *Investment Valuation*, 2nd ed., John Wiley and Sons, 2002,
chapter 13, "Dividend Discount Models",** the author's own copy,
<https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch13.pdf>, 37 pages,
sha256 `bf8fbef1797f91f8a8cc24aeb4de12ca0d7e746d7257c946e6e3a37189ef98c1`,
which is the digest the task names. Its printed page numbers match the file's
positions.

- Page 2: "The Gordon growth model relates the value of a stock to its expected
  dividends in the next time period, the cost of equity and the expected growth
  rate in dividends." The equation, read from the rendered page, is DPS₁ over
  kₑ − g, with "DPS1 = Expected Dividends one year from now (next period)",
  "ke= Required rate of return for equity investors" and "g = Growth rate in
  dividends forever".
- Page 3: "as the growth rate converges on the discount rate, the value goes to
  infinity".
- Page 4: "If the growth rate exceeds the cost of equity, the value per share
  becomes negative." And: "the Gordon growth model is best suited for firms
  growing at a rate comparable to or lower than the nominal growth in the
  economy and which have well established dividend payout policies that they
  intend to continue into the future."
- Page 5: "this model will under estimate the value of the stock in firms that
  consistently pay out less than they can afford and accumulate cash in the
  process."
- Pages 5 to 6, Consolidated Edison: dividends per share of $2.19, a cost of
  equity of 9%, a growth rate of 3.49%, and the page prints "($2.19)(1.0349)"
  over "0.09 – 0.0349" as "$41.15". **That figure is not those inputs'
  quotient.** Taken exactly, they give 41.1330…. The growth rate behind it is
  page 5's (1 − 0.6997)(0.1163), or 0.03492489, before rounding, which gives
  41.1526…, and that rounds to the page's figure. The form is as stated. The
  proof pins both.
- Pages 7 and 8, Vornado, a real estate investment trust, valued by the same
  form, as quoted under "By kind".
- Page 15, of buybacks, as quoted under the base dividend.

**Damodaran, A., *Valuation Approaches and Metrics: A Survey of the Theory and
Evidence*, Stern School of Business, New York University, November 2006,**
<https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
pages. The copy opened has sha256
`9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`, from three
whole transfers of 1,602,127 bytes. **The task prints `9bb7f22c7a1b6a10…`,
which differs at four of the sixty-four characters, the rest being equal.**
Digests of two different files do not share sixty characters, so this reads as
a slip in copying the digest, and every quotation the task names was found at
its page. The record names the digest it computed.

- Page 9: "the value of a stock is the present value of dividends through
  infinity", with "E(DPSt) = Expected dividends per share in period t" and "ke
  = Cost of equity".
- Page 10: "a model designed to value stock in a stable-growth firm that pays
  out what it can afford to in dividends. The value of the stock can then be
  written as a function of its expected dividends in the next time period, the
  cost of equity and the expected growth rate in dividends." The equation, read
  from the rendered page, is "Expected Dividends next period" over "(Cost of
  equity - Expected growth rate in perpetuity". Footnote 11 names Williams.
- Page 11: "since the growth rate in the firm's dividends is expected to last
  forever, it cannot exceed the growth rate of the economy in which the firm
  operates." The same page states the two-stage form. Footnote 14, running onto
  page 12: "Gordon, M.J., 1962, The Investment, Financing and Valuation of the
  Corporation, Homewood, Illinois: Richard D. Irwin, Inc."
- Page 15, of financial firms, as quoted under the preset. Also: "a high-growth
  firm, paying no dividends currently, can still be valued based upon dividends
  that it is expected to pay out when the growth rate declines."

**Damodaran's *Cost of Equity and Capital (US)*,** at the URL above. The server
closed the page early on eleven fetches and served it whole on the twelfth,
405,334 bytes: "Data used is as of January 2026"; the row "Total Market" reads
8.02% and the row "Total Market (without financials)" 8.37%, each in the column
"Cost of Equity"; "Last Updated in January 2026".

**What no source fixes.** The mean as the base, the window, a zero at Y as a
condition, and the denominator of the margin. Each is the record's own
decision, with its reason above. No copy states the form otherwise than the
task quotes it.

### The proof the implementing tasks bring

These tests are built through `History::of`, `Row::at` and a `Crossing` built
as the contract type. They read no file. Each stands in `tests/` or under
`#[cfg(test)]`. Years are calendar years, 2020 to 2024 unless the test says
otherwise, computed at 2024, and figures are listed from 2024 back. The preset
row reads 3.21, 3.11, 3.01, 2.91 and 2.81. The preset crossing is the
share-basis record's: a valuation date of 2025-06-30 answered with a close
traded 2025-06-30, the earlier date 2024-12-31 answered with a close traded
2024-12-31, and an empty split list. At the least:

- **The value at the presets, by hand.** The figures sum to 15.05, so B is
  `3.01`. E is 3.01 × 1.02 = 3.0702. r − g is 0.0602. V is `51`.
- **The value off them, by hand.** n of 2, r of 0.1 and g of 0.05, with 2024 at
  2.2 and 2023 at 2. B is 2.1, E is 2.205, and V is `44.1`.
- **The source's check figure.** n of 2, r of 0.09, 2024 and 2023 each at 2.19.
  At g of 0.0349, `41.13304900181488203266787658802178`. At g of 0.03492489,
  `41.15262791304456768220708047609891`, which rounds to the page's $41.15.
- **The margin at a price,** over the preset row and crossing. A close of 38.25
  gives `0.25`, of 51 gives `0`, and of 63.75 gives `-0.25`.
- **A year's excess, and the base form.** 5.05, 2.5, 2.5, 2.5 and 2.5, the
  regular 2.5 with a special of 2.55 at 2024. The value is `51`. Y's figure
  alone would give 85.56…, and the regular dividend alone 42.35…. The same
  excess at 2021 instead also gives `51`: the mean does not ask where it
  stands. Computed at 2025, with 2025 at 2.5 and the excess a year back in the
  window, it is `51` again: the excess stays for n years.
- **A suspension year stating zero.**
  - 0, 1, 1, 1 and 1: `zero_dividends_declared_per_share`, and never a positive
    value.
  - 0 five times, a non-payer: the same.
  - 3.7625, 3.7625, 0, 3.7625 and 3.7625, a zero inside the window: `51`.
  - The suspension fixture's years by their dates and figures, 2018 to 2024: 0,
    0, 0.09, 0.15, 0.12, `Unknown` and 0. With n of 2:
    - at 2018, `no_period` with `preceding_year`, once
    - at 2019, `zero_dividends_declared_per_share`
    - at 2020, `0.7624584717607973421926910299003322`
    - at 2021, `2.033222591362126245847176079734219`
    - at 2022, `2.287375415282392026578073089700997`
    - at 2023, `input_unknown` naming 2023, alone
    - at 2024, `input_unknown` naming 2023, alone: the zero at 2024 is not
      asked, since a read gave a reason
    - at the preset, at 2022, `1.219933554817275747508305647840532`
- **Each `declined` condition.**
  - A negative figure at 2024, and one at 2021:
    `negative_dividends_declared_per_share`. A negative at 2021 beside a zero
    at 2024: that alone.
  - `dividend_discount_growth_rate` at 0.0802, and at 0.09, beside the preset
    required return: `required_return_not_above_growth_rate`. A zero at 2024
    beside them: `zero_dividends_declared_per_share`, alone.
  - A close of 0, and of -1: `non_positive_close`.
- **No price, by each witness,** with the value `51` beside each.
  - No crossing: `no_price` with `not_asked`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying the date asked.
  - The test shows `Crossing` cannot be built without `at_valuation_date`. A
    crossing asked the valuation date alone gives `basis_unproven` with
    `not_covered`, and never `date_not_asked`.
- **The share-basis rule, by each clause,** over the preset row and crossing.
  - The split list absent: `splits_absent`.
  - The earlier date 2025-03-31 answered with a close traded then:
    `not_covered`.
  - A split dated 2025-03-14, and one dated 2024-12-31 itself: `split_within`.
  - The paired close's trading date `2025-6-30`, the earlier price's
    `20241231`, and a split dated `2025-02-30`: each `declined` with
    `share_basis_date_not_a_date`.
- **The exposure, pinned.** Over the preset row and crossing with a close of
  38.25, an added earlier date 2022-12-30 answered with a close traded then,
  and a split dated 2023-06-15: the rule passes and the margin is `0.25`. The
  test names this as the split inside the window that the rule does not see, so
  that a superseding record changes it knowingly.
- **An unreadable input.** A close of `1e2`: `declined` with
  `input_not_a_decimal`. Characters outside the grammar at 2024 and at 2022:
  that condition twice, in that order.
- **The order.** The history without 2021, with 2024 and 2022 `Unknown`, and no
  crossing. The value gives `input_unknown` at 2024, `input_unknown` at 2022,
  then `no_period` twice, at the reads of 2021 and 2020. The margin gives those
  four, then `no_price` with `not_asked`.
- **A declined value carried into the margin.** A zero at 2024 with no
  crossing: the value is `zero_dividends_declared_per_share`, and the margin is
  that reason followed by `not_asked`.
- **A window far past the history.** `dividend_discount_base_years` at the
  largest value its type holds: the value gives one `no_period` per read beyond
  the chain, and the call returns without overflow.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31 with the figure
  `Unknown` gives `not_a_year` alone, for both. A quarter in 2022's place gives
  `no_period` three times at 2024, for the reads of 2022, 2021 and 2020.
- **A filer of each kind.** The kind does not cross, so the test is the row.
  The preset figures, with every `operating`-only concept and
  `short_term_investments` `NotApplicable`, give `51`.
- **The settings.** Each refuses a value outside its domain and never clamps
  it: a required return of 0 and of -0.01, a growth rate of -1, base years of 0
  and of 1, and a rate of thirty-five significant digits. Each preset is
  written back as `0.0802`, `0.02` and `5`.
- **The catalogue.** Each entry states its unit, `Duration`, the one input and
  its conditions in the order above. The margin's lists
  `share_basis_date_not_a_date` and not `non_positive_value_per_share`. The
  value's lists neither. Neither name holds `common`, and neither entry's
  `inputs` holds `dividends_paid`, `shares_outstanding` or a metric.

No test reads the computed rate, because the derivation does not.

### What no check can see

- **A special dividend.** Answer 6's own case. The total crosses, nothing in
  the history marks it, and the mean carries a fifth of it for five years at
  the preset. The figure looks like any other value. This flatters. A
  declaration moved across a year end is the same.
- **A split inside the window.** This is the largest error the form can make
  that the method does not see.
  - The figures in the window can be on different share bases. Each was set by
    the last filing that quoted its period. A split at year S restates the
    years the filings after S still quote, and the older ones keep the old
    basis.
  - A 2-for-1 split with two old-basis years reads 1, 1, 1, 2 and 2, from Y
    back, for a dividend of 1 on today's shares. B is 1.4, or 40% too high.
    This flatters.
  - The value reads no crossing, so it cannot see it. The margin calls the rule
    with E the end of Y, and the rule looks at splits on or after the earlier
    of E and T, so a split before Y's end passes. The margin is a number. The
    test above pins that.
  - A figure on one filing's basis has no such error, and that is what the
    computing year alone would have offered. A narrower window narrows it.
  - It is not closed here. See "What stands". `dividend_growth_rate` reads two
    years across the same exposure, and its record does not name it.
- **Whether the sources say what this record says.** The copies are named and
  the quotations given so a reader can check. Gordon, Williams and Graham were
  not read. The implementing tasks cite each as the sources record says, and a
  source that on reading says otherwise is a stop for that task.
- **A filer outside the form.** One that pays out less than it can afford is
  understated (page 5), and page 15 says a focus on dividends misses
  "significant cash returned to stockholders in the form of stock buybacks".
  The survey's page 14 says one paying more than it can afford gets "value
  estimates that are too optimistic". Nothing in the figure says which a filer
  is.
- **A grower under the presets.** It is valued as growing at inflation from a
  base that lags, so the value at the presets is conservative for it. Nothing
  says by how much.
- **The value's weight on r − g.** Near equality a small change in a setting
  moves the value without bound, and nothing caps it.
- **An `investment_company`'s figure.** It is computed, with an
  error of no known direction.
- **The value at Y beside a price at T.** The value is not rolled forward to
  the valuation date, as the discounted cash flow record says of its own.
- **The currency.** The close's and the filer's reporting currency are not
  compared, which `price-provider.md` already leaves open.
- **Two presets moving apart.** Nothing holds this record's growth preset to
  the discounted cash flow's terminal preset, and nothing should. Only review
  sees a stale one, and nothing says when the January figure moves.
- **A metric that reads the computed rate, or forms the margin itself.** Only
  review sees it.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The three
  presets and the domain's two stand in `settings/`. The derivation needs only
  one, through `Number::one`, and n is a count from a setting. No constant is
  added.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule, three places and catalogue
- the share-basis record's comparisons and condition
- the arithmetic record's reading, operations and written form
- the series record's year test, seek, gap rule and series
- the growth rate record's form, window and setting
- the discounted cash flow record's two metrics, settings, conditions and
  margin
- the provider record's ask and answer
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains no field and no value of `unit`

One question here would need an accepted record to change, or a surface to
cross, and it is left open and named:

- **A split inside the window.** To see it for the margin, the rule would run
  at the end of the window's earliest year rather than at Y's. The analyze
  record lets E be "the figure's period end", and this figure has n of them, so
  that may be a reading of the rule or a change to it, and this record's task
  names E the end of the year. It would not help the value, which reads no
  crossing. Closing it for both needs each value's basis across
  `canonical-concepts`, which the analyze record's alternatives name, "The
  basis date through `canonical-concepts` v3". This record proposes neither. A
  record of its own may.

**Why Structural.** The record fixes the names and meanings of two metrics that
cross `analyze-store` v1, which store and M6 will read, and three settings that
every result records. It needs nothing Constitutional: no anchor, no gate, no
protected path and no GOALS.md scope moves.

## Alternatives

- **The computed `dividend_growth_rate` as g.** It is the filer's own history,
  and it needs no preset. Rejected, for the three reasons under "The growth
  rate is a setting alone".
- **The computing year's dividend alone as the base.** One read, no window, no
  setting, no `Unknown` from a past year, no mixed share basis. Damodaran's
  illustration reads it so. Rejected: it is what answer 6 says the valuation
  must not do, in the direction answer 6 names. It would be the better form if
  the regular dividend crossed, and the record that brings the split can
  supersede this one.
- **The lower of the window's years as the base.** It holds a one-period excess
  out altogether, wherever it stands. And a suspension at Y falls to it with no
  condition. Rejected: it keeps a figure by its size against its neighbours,
  and at n of 2 it is "trimming the largest figure", which
  `dividend-growth-rate.md` refuses with the inference from magnitude. It also
  still takes one period's total as the perpetuity, and it understates a grower
  by the whole window.
- **The median of the window.** Robust to one excess. Rejected:
  `dividend-growth-rate.md` says of it that "Halving an even count needs a
  number the derivation cannot cite", and like the lower of the years it keeps
  or drops a figure by its size.
- **Reading `dividend_growth_years` as the window.** One setting fewer.
  Rejected: it counts steps and not years, and moving the rate's window would
  silently move this base.
- **A weighted mean, favouring later years.** It lags less. Rejected: each
  weight is a number no source fixes.
- **Skipping an `Unknown` year, or closing up across a gap.** It keeps the
  value present. Rejected: the analyze record's "Nothing is substituted", and
  the series record's gap rule.
- **A two-stage form on the discounted cash flow's pattern.** The survey's page
  11 states it, and it would fit a growing filer. Rejected for this version: it
  adds a forecast growth rate and a horizon with no source, and at any preset
  that credits no excess growth it equals this form. Page 3 says it is the
  better tool past stable growth, and a superseding record can bring it.
- **The margin over the price,** (V − P) ÷ P, or the price over the value.
  Rejected, by the discounted cash flow record's argument, adopted by name. Two
  margins with two denominators would also not be comparable in one table.
- **`dividends_paid` over a share count as the dividend.** It would read cash,
  not a declaration. Rejected: the common-shareholder record rejects the
  product form, since "a count at one instant, or a diluted average, is not the
  shares each dividend was paid on", and the same holds of the quotient.
  `dividends_paid` is every class, and the catalogue names the per-share
  concept for this consumer.
- **A zero at Y as a figure of `0`.** It agrees with the streak's `0`.
  Rejected: under the mean, the formula's value at that year is not zero, so
  this would be a rule and not the formula, and for a non-payer it is a figure
  that ranks as worthless.
- **A zero at Y read through the mean.** It is the formula with no exception.
  Rejected: it values a declared nothing as a continuing dividend, which
  flatters.
- **`non_positive_value_per_share` listed for symmetry.** Rejected: it cannot
  be given, as stated.
- **The relation r > g as a joint domain.** Rejected, by the discounted cash
  flow record's argument, adopted by name.
- **The row "Total Market (without financials)" for the required return,**
  8.37%. It is the higher, so the more conservative, and it is the row the
  discounted cash flow's rate is read from. Rejected: that row's reason is the
  one the survey says runs the other way for this form. Not a large error
  either way, and it is a preset.
- **A filer's own cost of equity from a beta,** as the illustration builds it.
  Rejected: no filing and no provider field carries a beta, and a preset for it
  is the market's rate again.
- **Reusing `discounted_cash_flow_terminal_growth_rate` for g.** One setting
  fewer. Rejected: it would make one method's preset the other's input. A
  user's view of a firm's flow after a forecast is not their view of its
  dividend.
- **Running the share-basis rule at the window's earliest year.** It would make
  the margin decline across a split in the window. Not rejected on its merits:
  it is left open under "What stands", since it leaves the value exposed and
  its task names E the end of the year.
- **Leaving the valuation out of M5 until the provider is built.** Rejected:
  the value reads no price, and the margin's absence without one is fixed by
  accepted records. A test builds the crossing as the contract type. Deferring
  would also change milestone scope, which is Constitutional.

## Consequences

**Easier:**

- The dividend discount valuation has its form, settings, presets and sources,
  and its implementing tasks have only their code to decide.
- Answer 6's constraint has a named form, and each excess's size in the value
  can be read off it.
- A user with no key sees the value and why the margin is missing.
- It needs one concept, no bridge and no share count. A filer that declares a
  dividend and reports it for n years is measured.

**Harder:**

- **n reads make the value absent more often than a one-year base.** An
  `Unknown` holds it absent for n years, and a filer with fewer than n years of
  history has none.
- **A special dividend stays in the value for n years,** a fifth at a time at
  the preset, where a one-year base would forget it next year.
- **A split inside the window gives a wrong number nothing shows,** in the
  flattering direction, for as long as an old-basis year stays in the window.
  This is the cost of answer 6's form under v2.
- **Every non-payer and every suspended year is absent,** not `0`.
- **The presets value every filer as growing at inflation,** from a base that
  lags, so the margin at the presets is conservative for a grower.
- **Two margins and two perpetuity presets** now stand side by side, and a user
  must read them as two models' answers.

**Expensive to reverse:**

- The two names, the condition's characters, the mean as the base and the
  margin's denominator, once results are stored and ranked on.

**Open, and not this record's:**

- A split inside the window, as stated under "What stands".
- The regular dividend alone, which would let the base be Y's. It arrives with
  a `fetch-normalize` version that carries the axis and member, then
  `canonical-concepts` v3, as answer 6 says.
- A two-stage form, and the computed rate as a second metric.
- How the shell shows a value that is a model's answer beside a close. That is
  the shell boundary's.

## Enforcement

This applies anchors 4 and 5 and the one-source-of-truth invariant, and edits
none of them.

- **Anchor 4.** The two metrics read their history, the crossing and the
  settings, and only through the accepted seek, reading and rule. `purity` and
  `ambient` read what they read today.
- **Anchor 5.** Every free parameter is a setting. The three stand in
  `settings/` with their reasons, and `literals` keeps every number but zero
  and one out of the derivation.
  - **Gap:** nothing stops the derivation calling the preset constructor, as
    the analyze record already says.
  - **Gap:** nothing checks that a citation is true, or that a preset still
    matches its source.
- **One definition of each condition, and of the margin's form.** The catalogue
  test holds the characters.
  - **Gap:** nothing forces a metric through the one definition rather than
    around it.
- **No wrong number across a split.** The share-basis tests hold it for a split
  on or after the earlier of E and T.
  - **Gap:** nothing holds it for a split inside the window before E, as
    stated. The pinned test shows it and does not close it.
- **No margin without a price.** The witness tests hold it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the names and
  meanings of two metrics that cross `analyze-store` v1 as characters, names
  three settings that every result records, and adds one `declined`
  condition, all inside analyze. It edits no anchor, no protected path, no
  gate, no contract byte, no schema and no milestone scope, and asks for no
  allowlist entry. Flagged for later human review, as the tier requires.
- **Checked:**
  - GOALS.md's fifth M5 criterion ends in "dividend-based valuation", and the
    discounted cash flow and growth rate records each leave this valuation
    to "a record of its own", as the record quotes them.
  - `canonical-concepts-open-questions.md`, answer 6: the input is the total,
    regular and special together, and the valuation "must not take one
    period's declared total as a perpetuity". The mean over a window is a
    form that sentence allows; the record states what the mean does with an
    excess, in both directions, and does not claim it removes one.
  - `dividend-growth-rate.md` (Accepted, Structural): its warning about a
    rate "carried into a perpetuity" is the record's first reason for a
    setting alone, and its refusals of "trimming the largest figure" and a
    median are what reject the lower-of-years and median bases. Its sentence
    that a condition "the metric never gives tells a reader something false"
    is why `non_positive_value_per_share` is not listed, and the argument
    that it cannot arise holds: every read is positive once the two dividend
    conditions pass, and r − g is positive once the third does.
  - `discounted-cash-flow-and-margin-of-safety.md` (Accepted, Structural):
    the margin's form and denominator are adopted by name, the joint-domain
    argument is adopted by name, the FOMC statement is the same source for a
    growth preset of `0.02`, and `non_positive_close` is reached from where
    M5-55 will define it. The accepted entry on main lists its own
    conditions and then the four year conditions, which is the order this
    record gives for both entries.
  - `share-basis-rule-dates.md` (Accepted): the margin lists
    `share_basis_date_not_a_date` after the four year conditions and calls
    the one rule function with E the end of Y; the value calls no rule and
    lists nothing.
  - `what-analyze-takes-and-returns.md`: six closed reasons and no seventh;
    `at_valuation_date` is read by its place so `date_not_asked` has no
    witness, as the discounted cash flow record already said of its margin.
  - `common-shareholder-forms.md`: `per_share` on the value, `common` on
    neither, the margin named for the value it is built on.
  - On main, `negative_dividends_declared_per_share` is defined once in
    `dividend_streak.rs` and `zero_dividends_declared_per_share` once in
    `dividend_coverage_per_share.rs`, the latter private, so the record's
    "widens the constant's visibility and defines no second one" names a
    real edit for the implementing task.
  - The hand-worked figures, each reproduced: `51` at the presets, `44.1`
    off them, 41.1330… and 41.1526… for Consolidated Edison, the margins
    `0.25`, `0` and `-0.25`, `51` with the excess at 2024 and at 2021 and
    85.56… for Y alone, `51` with a zero inside the window, the four
    suspension-fixture values, the multipliers 16.94 and 16.01 and their
    5.8%, the grower lag 0.9615 and 0.8340, and the split example's 1.4.
  - The alternatives as argued, the six the task required among them.
  - Not checked here: the open sources were not re-opened in this run. The
    quotations are taken as the proposer found them, at the pages named, and
    the proposer's note that the survey's digest differs from the task's at
    four of sixty-four characters is carried as a flag, not resolved.
- **Verdict and why:** Accepted. The record makes the three choices the task
  left to it, the base, the growth read and the required return's row, each
  with a reason a reader can weigh and a direction a reader can see, and it
  names the one exposure it cannot close, a split inside the window, with a
  pinned test rather than a reading of the rule that would stretch the
  accepted analyze record. Its base is the conservative form on every path
  but the special dividend, where it removes nothing but divides by n; that
  is the honest account of what answer 6 leaves possible under v2. Four
  points stand flagged for the human who reviews Structural acceptances: the
  open split exposure; a zero at Y as a condition where the growth rate
  record chose a figure at its own zero; the "Total Market" row for the
  required return against the discounted cash flow record's row without
  financials; and the survey's digest.
- **What would have changed it:** a source opened in the run stating the
  form otherwise than the record quotes it; a base form that compared a
  year's figure with its neighbours, which the growth rate record refuses; a
  reading of the share-basis rule at the window's earliest year, which
  changes an accepted record's E without saying so; a growth read of the
  computed rate without an answer to that record's warning; or a condition
  listed that the derivation cannot give.
