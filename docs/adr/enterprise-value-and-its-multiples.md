# Enterprise value is market capitalisation plus net debt, formed once and handed over nowhere, counting neither the preferred claim nor non-controlling interests; EV/EBIT sets it over a year's operating income and EV/EBITDA over that plus depreciation and amortisation, and each declines at an enterprise value or a denominator of zero or below

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-11-21, by `M5-66`
- **Decided:**
- **Touches:** the valuation multiples that GOALS.md's fourth M5 criterion
  names, for the two whose numerator is the value of the firm, and the
  enterprise value they share. It fixes the names and meanings of two metrics
  that cross `analyze-store` v1 as characters, and no byte of that contract
  changes. It forms enterprise value once, in a module of its own, as a term
  handed over nowhere, from market capitalisation and net debt, each read
  from the one place its accepted record forms it. It forms EBITDA once, in a
  module of its own. It applies the accepted `declined` reason to five new
  conditions, reaches two existing ones from their one definitions, and adds
  no reason. It does not count the preferred claim, so `preferred_equity`
  gains no consumer and the open-questions record's admission stands as it
  is. It names, and leaves open, three questions about accepted records or
  contracts: whether the period record's averaging sentence should reach the
  balances of a value the close is set beside, whether the share-basis rule
  should see a count read at an instant before E, and a v3 concept for
  non-controlling interests. It adds no value of the catalogue entry's `unit`
  and no field, and names the value enterprise value would need as a metric.
  No accepted record is superseded. No contract's bytes, no schema, no edge,
  no gate and no dependency move. Nothing here is implemented.

## Context

GOALS.md's fourth M5 criterion names "valuation multiples" among the value
metrics. `market-capitalisation-and-the-equity-multiples.md` (Accepted)
settled the four whose numerator is the market value of equity, and left this
record by name: "Enterprise value and its two multiples are left to a record
of their own." Its open item reads: "Enterprise value, EV/EBIT and EV/EBITDA:
whether enterprise value counts the preferred claim or non-controlling
interests, and which instant net debt is read at. The chapter's page 46 names
the second multiple."

Some things are already fixed, and this record restates them as fixed:

- **The concepts.** `canonical-concepts.md` (Accepted) names:
  - `shares_outstanding`, consumed by "market capitalisation, and through it
    price/book, price/sales, enterprise value and Altman's X₄".
  - `operating_income`: "profit from the ordinary business after all
    operating costs and before interest and tax, as the income statement
    presents it. Excludes interest expense, interest and investment income,
    and tax." Consumed by "operating margin; return on capital employed and
    on invested capital; EV/EBIT and, with depreciation and amortisation,
    EV/EBITDA; interest coverage; Altman's X₃."
  - `depreciation_and_amortization`: "depreciation of tangible assets and
    amortisation of intangibles charged to the period, read from the cash
    flow statement's reconciliation where the income statement does not
    present it separately." Consumed by "EBITDA, and through it EV/EBITDA."
  - `cash_and_equivalents`, consumed by "the cash ratio; net debt in
    enterprise value; the equity bridge in the discounted cash flow."
  - `short_term_debt`, consumed by "total debt in debt/equity; net debt in
    enterprise value and the discounted cash flow bridge."
  - Under "Derived values", left out of the set: "free cash flow, EBITDA,
    total debt, net debt, working capital, book value per share, the
    effective tax rate. Each is a combination of concepts above and belongs
    to analysis."
- **The open-questions record.** `canonical-concepts-open-questions.md`
  (Accepted):
  - names `short_term_investments` as consumed by "net debt in enterprise
    value, and through it EV/EBIT and EV/EBITDA; the equity bridge in the
    discounted cash flow.", and reads its silence as zero because "the
    `Unknown` reading would delete enterprise value for every filer holding
    no securities";
  - names `preferred_equity` as consumed by "the common-shareholder forms of
    return on equity, price/book and sustainable growth, each of which forms
    common equity as `shareholders_equity` less this.";
  - leaves operating lease liabilities out of debt, because "Capitalising the
    liability while the flows stand as presented counts the lease twice in
    every metric that pairs a stock against a flow — enterprise value against
    EBIT, the discounted cash flow's bridge against free cash flow."
- **Net debt.** `capital-employed-invested-capital-and-nopat.md` (Accepted)
  forms it once in `crates/analyze/src/metrics/capital.rs`, where it "is
  total debt less `cash_and_equivalents` and less `short_term_investments`,
  at one instant." It says: "Enterprise value and the discounted cash flow
  bridge are its other named consumers. They read it here when they are
  built. Whether enterprise value adds the preferred claim or non-controlling
  interests beside it is that metric's own record." It names as open "A
  concept for non-controlling interests, and for redeemable preferred outside
  permanent equity." It says "v2's `operating_income` is EBIT by its
  meaning."
- **The preferred claim.** `common-shareholder-forms.md` (Accepted) says "If
  enterprise value wants `preferred_equity`, that is a new consumer the
  open-questions record did not admit, and its own record."
- **Market capitalisation.** The market capitalisation record forms it once
  in `crates/analyze/src/metrics/market_capitalisation.rs`: "Every metric
  that reads market capitalisation reads it there: price/book, price/sales
  and price/free cash flow here, and enterprise value and Altman's X₄ when
  they are built. No metric writes the product itself." Its two conditions
  are `non_positive_close` and `non_positive_shares_outstanding`. It is
  formed "at each row that count was read at, before any mean". Of leverage
  it says: "Price/sales and price/free cash flow set an equity value over a
  flow to the whole firm, and read lower for a more levered filer, as the
  chapter's page 2 says. Enterprise value would remove it." Of an opening
  count on another share basis, it "names the exposure, pins it in a test,
  and leaves it open".
- **The bridge.** `discounted-cash-flow-and-margin-of-safety.md` (Accepted)
  says of net debt "Inside it, a `NotApplicable` `short_term_investments`
  contributes nothing. Nothing else does." It reads the bridge's net debt
  averaged, "Which instant net debt is read at is the period record's rule,
  and the rule reads it averaged.", and leaves open "Reading the bridge at
  Y's close needs the period record superseded or narrowed. That is a record
  of its own." It subtracts neither the preferred claim nor non-controlling
  interests.
- **The period record.** `which-other-period-a-metric-reads.md` (Accepted):
  "A metric whose formula pairs a flow with a balance is stated at a
  duration." And: "Every other metric that pairs a flow with a balance reads
  the average. This holds whatever source its formula cites. A balance its
  source names without saying when is read at closing, since that is the
  balance the period's own statement presents."
- **The series record.** `which-duration-precedes-a-duration.md` (Accepted):
  "A metric is stated over a year when its source states it over a fiscal
  year."
- **The analyze record.** `what-analyze-takes-and-returns.md` (Accepted):
  "Market capitalisation, and everything built on it, falls under the rule
  through `shares_outstanding`." Of `declined`: "Examples are a zero divisor
  and P/E on non-positive earnings. Each metric's task lists its own
  conditions."
- **The share-basis dates record.** `share-basis-rule-dates.md` (Accepted):
  "For a metric stated at a duration, as both discounted cash flow metrics
  are, E is the duration's end".
- **The provider record.** `price-provider.md` (Accepted) lists under the
  valuation date "market capitalisation, through `shares_outstanding`, and
  through it price/book, price/sales, price/free cash flow, enterprise value
  (so EV/EBIT and EV/EBITDA) and Altman's X₄, the market value of equity".
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written,
  and a table of operations.
- **Absence is closed.** Six reasons. A metric built on a term carries the
  term's reasons.
- **The entry shape.** M5-18 fixed it: name, meaning, `unit`, `stated_at`,
  inputs in reason order, and the metric's own `declined` conditions. `Unit`
  carries `FractionOfOne`, `Multiple`, `Years`, `CurrencyPerShare` and
  `Signals`, and no unit of currency.
- **What is on main.** The market capitalisation module, the net debt
  function and the price read in `crates/analyze/src/metrics/reading/price.rs`
  are built. No metric reads or forms EBITDA, and none reads
  `depreciation_and_amortization`.

## Decision

### Two metrics and two terms

| Name | What it is | Unit | Stated at |
| :--- | :--- | :--- | :--- |
| `enterprise_value_to_ebit` | Enterprise value over a year's operating income | `Multiple` | a year |
| `enterprise_value_to_ebitda` | Enterprise value over a year's operating income plus depreciation and amortisation | `Multiple` | a year |

**Enterprise value** is the term both share. **EBITDA** is the term the
second divides by. Neither is a metric of the method.

Both metrics read a price. Neither reads a setting: each form is fixed by its
source or by this record, and neither has a free parameter.

### Enterprise value, formed once

**The form.** Market capitalisation plus net debt.

- Damodaran's chapter 18, page 46, sets the numerator of EV/EBITDA as the
  market value of equity plus the market value of debt less cash, and says
  the multiple "relates the total market value of the firm, net of cash, to
  the earnings before interest, taxes and depreciation of the firm."
- The market value of equity is market capitalisation, read from
  `crates/analyze/src/metrics/market_capitalisation.rs`. The debt less cash
  is net debt, read from the one function in
  `crates/analyze/src/metrics/capital.rs`, with its one exception as
  accepted: a `NotApplicable` `short_term_investments` contributes nothing,
  and nothing else does.
- No product and no sum already formed is written a second time. Enterprise
  value writes one addition: the market capitalisation the one module gives,
  plus the net debt the one function gives.
- The inputs are read in this order: `shares_outstanding`, `short_term_debt`,
  `long_term_debt`, `cash_and_equivalents`, `short_term_investments`, then
  the price. That is market capitalisation's order, then net debt's. The
  price is no concept.

**Four things about the form, each with its reason and its cost.**

- **Debt is at its carrying amount.** The chapter's equation sets the market
  value of debt in the numerator. v2 carries the borrowings as the balance
  sheet states them, and no market value of debt. The discounted cash flow
  bridge already subtracts the same carrying amounts. The chapter's own
  illustration on page 47 uses what is outstanding: "The market value of
  equity at Segovia is $1,529 million and the consolidated debt outstanding
  at the firm is $500 million."
  - The cost: where a filer's debt trades away from what it is carried at,
    enterprise value is off by the difference. Debt carried above what it
    trades at, as a distressed filer's is, overstates enterprise value. Debt
    carried below, as fixed-rate debt is after rates fall, understates it,
    which flatters. Nothing measures either.
- **`short_term_investments` are netted beside cash,** as the capital record
  forms net debt, against the chapter's one word "Cash".
  - The chapter's reason for netting is on page 46: "Since the interest
    income from the cash is not counted as part of the EBITDA, not netting
    out the cash will result in an overstatement of the true value to EBITDA
    multiple." v2's `operating_income` excludes "interest and investment
    income", so the reason reaches the securities that earn it.
  - The survey's page 29 names them together: "The most common of the
    non-operating assets is cash and marketable securities".
  - The cost: every dollar of cash is netted, the cash the business needs to
    run included, as the capital record says of invested capital. That
    understates enterprise value, which flatters. Securities a filer
    classifies as non-current are outside v2's `short_term_investments` and
    are not netted, which overstates it.
- **The preferred claim is not counted.** Enterprise value does not read
  `preferred_equity`.
  - No copy opened puts it in. The chapter's numerator has three words:
    equity, debt and cash. The survey's page 29 lists the non-equity claims
    as "not only all debt, but all capitalized leases as well as unfunded
    pension plan and health care obligations", and names no preferred.
  - The concept's silence is `Unknown`. The discounted cash flow record,
    accepted, weighed the same read for its bridge: "Reading it would make
    the discounted cash flow absent for most filers, which carry no
    preferred, to correct a claim few carry." That holds here word for word.
    A filer silent on `preferred_equity` gets both multiples as figures.
  - Enterprise value is the bridge read the other way. The bridge takes net
    debt from a value of operations to reach equity. Enterprise value adds
    net debt to the market's value of equity to reach the market's value of
    operations. Reading one net debt and the same claims on both sides keeps
    the two comparable.
  - Letting an `Unknown` preferred contribute nothing would extend the one
    accepted exception to a second concept and a second state. The capital
    record applies that exception "as accepted, not extended".
  - The cost: for a filer with preferred outstanding, enterprise value is
    understated by its carrying amount, and both multiples read low. That
    flatters. A `bank`, an `insurer` and an `investment_company` are absent
    from both multiples by `operating_income`'s applicability, so the cost
    falls on the `operating` and `reit` filers that carry the claim.
  - So `preferred_equity` gains no consumer. The common-shareholder record's
    sentence, "If enterprise value wants `preferred_equity`, that is a new
    consumer the open-questions record did not admit, and its own record.",
    is answered: it does not want it under this record. A reading of the
    preferred claim that does not cost every silent filer is the record the
    discounted cash flow record already names, and it would supersede this
    record and the bridge together.
- **Non-controlling interests are not counted,** because v2 carries no
  concept for them.
  - The chapter's pages 46 and 47 state the mismatch. Where a holding is a
    majority holding and consolidated: "The EBITDA includes 100% of the
    EBITDA of the holding, but the numerator reflects only the percentage
    (not 100%) of the holding that belongs to the firm. Thus, the value to
    EBITDA will be too low, leading it to be categorized as an undervalued
    stock."
  - So for a filer with non-controlling interests, both multiples read low.
    That flatters, and nothing measures it.
  - Counting them needs a concept such as `noncontrolling_interest`, which is
    a `canonical-concepts` v3. The capital record names it and leaves it to a
    record of its own. This record names it too and does not propose it.
  - The chapter states the opposite mismatch for a minority holding: "When a
    holding is categorized as a minority holding, the operating income of a
    firm does not reflect the income from the holding." The numerator then
    holds the value of a holding whose earnings the denominator lacks, and
    the multiple reads high. v2 carries no concept for those holdings
    either. The chapter's page 47 says "The correction for cross-holdings is
    tedious and difficult to do when the holdings are in private firms."

**Where it lives.** One module of the derivation,
`crates/analyze/src/metrics/enterprise_value.rs`. Both multiples read
enterprise value there, and so does any later metric that reads it. No metric
writes the addition itself. The term is formed at each instant its inputs
were read at, before any mean: the market capitalisation at that instant
plus the net debt at that instant. So an averaged enterprise value is the
average of the term, as the market capitalisation module and the capital
module form theirs.

**It is a term, handed over nowhere,** as market capitalisation, net debt and
free cash flow are.

- As a metric it would need an entry with a `unit` of currency. `Unit`
  carries none. That value is `Currency`, the one the market capitalisation
  record already names. It is named here and left open. This record does not
  add it.
- GOALS.md names "valuation multiples", and enterprise value is not one. Its
  consumers are the two multiples.
- A later record may make it a metric, for a size screen in M6, adding
  `Currency` to `Unit`. Adding a metric bumps nothing, and the term's module
  stays where it is.

**A negative enterprise value is a value of the term.** A filer holding more
cash and investments than its market capitalisation and debt together has an
enterprise value of zero or below. The term gives that number with its sign,
as the free cash flow module gives a negative flow. What a multiple does with
it is the multiple's condition, stated below.

### The two denominators

**EBIT is `operating_income`,** read at the year's own row.

- v2 means by it "profit from the ordinary business after all operating costs
  and before interest and tax". That is earnings before interest and tax.
  The capital record, accepted, already reads it so: "v2's `operating_income`
  is EBIT by its meaning."
- Damodaran writes the two as one. The chapter's page 49 has "FCFF = EBIT
  (1-t)" where the survey's page 26 has "Free Cash Flow to Firm = After-tax
  Operating Income". The survey's pages 59 to 60 name the multiple by "the
  operating income".
- v2's figure excludes interest and investment income, which matches the
  chapter's page 46: "the interest income from the cash is not counted as
  part of the EBITDA".
- It needs no term and no module. The multiple reads the concept.

**EBITDA is `operating_income` plus `depreciation_and_amortization`,** each
read at the year's own row.

- The chapter's page 50 writes EBIT as "(EBITDA – DA)", so EBITDA is EBIT
  plus depreciation and amortisation. The catalogue names the concept for
  "EBITDA, and through it EV/EBITDA", and leaves EBITDA to analysis as a
  derived value.
- **Where it is formed.** Once, in a module of its own,
  `crates/analyze/src/metrics/ebitda.rs`, as free cash flow has its own. No
  metric on main reads or forms it. `enterprise_value_to_ebitda` reads it
  there and does not write the addition itself. A later metric that wants
  EBITDA reads it there.
- A negative EBITDA is a value of the term. What the multiple does with it is
  the multiple's condition.
- The inputs are read in this order: `operating_income`, then
  `depreciation_and_amortization`.

### The derivations, in formula order

Every step is an operation in the arithmetic record's table. No root, power,
logarithm or literal enters.

| Metric | Formula | Operations |
| :--- | :--- | :--- |
| `enterprise_value_to_ebit` | (the average, over Y's opening and closing instants, of P × `shares_outstanding` plus net debt) ÷ `operating_income` over Y | multiplication, addition, subtraction, division by the count of two, division |
| `enterprise_value_to_ebitda` | that enterprise value ÷ (`operating_income` plus `depreciation_and_amortization` over Y) | the same, and one more addition |

P is the close at the valuation date. Y is a year. Net debt at an instant is
`short_term_debt` plus `long_term_debt` less `cash_and_equivalents` less
`short_term_investments`, as the capital module forms it.

- **EV/EBITDA** is the chapter's form, on page 46: the market value of equity
  plus the market value of debt less cash, over EBITDA.
- **EV/EBIT** is enterprise value over EBIT. What the copies say of it:
  - The chapter's page 55 names it as a variant: "While enterprise value to
    EBITDA may be the most widely used value multiple, there are close
    variants that are sometimes used by analysts". The page names three, and
    "Value/EBIT" is the first.
  - The survey's sentence that opens at the foot of page 59, "When buying a
    business,", and runs on at the head of page 60: "as opposed to just the
    equity in the business, it is common to examine the value of the firm as
    a multiple of the operating income or the earnings before interest,
    taxes, depreciation and amortization (EBITDA)."
  - **What they leave unstated** is what "Value" is in "Value/EBIT". Neither
    copy writes its numerator out. **The record's own reading** is that it is
    the enterprise value of page 46, net of cash. The reasons:
    - The chapter calls it a close variant of the page 46 multiple and
      changes only the denominator's name. It uses "value to EBITDA" and
      "enterprise value to EBITDA" for one multiple throughout pages 46 to
      55.
    - Page 46's reason for netting cash holds for EBIT exactly as for EBITDA:
      neither counts the interest income the cash earns.
    - It reuses the one term, so no second numerator is formed.

### Which periods

**Both are stated over a year,** under the series record's test.

- The chapter analyses the multiple from a yearly model. Page 49 derives it
  from the value of the operating assets as next year's free cash flow to the
  firm over the cost of capital less growth, and page 51 states that growth
  as "5% a year in perpetuity". Page 47 gives "an EBITDA of $120 million on
  capital invested of $250 million in the current year". Page 55 describes
  its dataset as "for the most recent year".
- No page sets the multiple over a quarter's earnings. A trailing figure adds
  four quarters, which no record forms.
- A quarter's EBIT or EBITDA under the same enterprise value gives about four
  times a year's multiple, and nothing would mark it.
- For EV/EBIT the sources state no period of their own. It takes EV/EBITDA's,
  as its close variant. That is the record's own reading.

At a duration that is not a year, each gives exactly one `declined` reason,
the first of `start_not_a_date`, `end_not_a_date`, `not_a_year` and
`overlaps_another_year`, and makes no read, as the series record says. Their
`stated_at` stays `Duration`.

### Which instants the count and net debt are read at

**Every balance is read averaged.** Each multiple pairs a flow,
`operating_income`, with five balances: `shares_outstanding` and the four of
net debt. Each has measure `balance` in v2. Neither multiple is a term of a
named score. So under the period record's rule as written, each balance is
read at the instants Y opens and closes at, and the multiple divides the mean
of enterprise value at those two instants.

That is how both accepted records before this one read the rule. The market
capitalisation record averages the count for price/sales and price/free cash
flow. The discounted cash flow record averages net debt in the bridge.

**The order the reads are made in.** Per concept, in input order, opening
then closing. That is the period rule's order, and the capital record kept it
when it rejected "Averaging the term at each instant" for "the read order
alone".

1. `shares_outstanding` at the opening instant, then the closing.
2. `short_term_debt`, opening then closing.
3. `long_term_debt`, opening then closing.
4. `cash_and_equivalents`, opening then closing.
5. `short_term_investments`, opening then closing.
6. `operating_income` at Y.
7. For EV/EBITDA, `depreciation_and_amortization` at Y.

EV/EBIT makes eleven reads and EV/EBITDA twelve.

**The term is formed at each instant, then averaged.** The reads are per
concept. The term is formed from them at each instant: market capitalisation
there, "at each row that count was read at, before any mean", plus net debt
there. Then the mean is taken. The mean is linear and the arithmetic is
exact, so this is the same figure as the close times the average count plus
the average net debt. There is one read order and one figure.

**Every average is over two instants.** Where `short_term_investments`
contributes nothing at an instant, that instant is still counted, as the
capital record fixes.

**The rule gives a worse answer than the term wants,** and this record says
so and leaves it:

- **The sentence.** "Every other metric that pairs a flow with a balance
  reads the average. This holds whatever source its formula cites."
- **The third sentence does not release it.** "A balance its source names
  without saying when is read at closing" could be read to put these balances
  at closing, since the chapter does not say when. Both accepted records read
  the rule as reaching such a balance, and the discounted cash flow record's
  review called that the rule "as written". This record reads it the same
  way, and does not read the rule narrowly.
- **The rule's reason does not fit.** It is a ratio's: "The mean of the two
  ends is the balance the flow was earned on, to first order, and neither
  end alone is." Enterprise value is not a balance the year's earnings were
  earned on. It is what a buyer pays for the
  firm at the valuation date: the market value of the equity then, plus the
  debt taken on, less the cash acquired. The balances nearest that date in
  the history are the closing ones.
- **What it costs, and its direction.** The averaged term is the value of the
  firm at no date. Where the count or net debt rose over Y, the term is
  understated and both multiples read low, which flatters. That is the filer
  that issued shares or borrowed during the year. Where they fell, the
  reverse. The same filer's market capitalisation differs between price/book
  at Y's closing instant and these multiples over Y.
- **What would change it.** Reading the five balances at closing needs that
  sentence narrowed, or the third sentence read as covering this case. That
  is a superseding record of its own, and the one the market capitalisation
  record and the discounted cash flow record each name. This record does not
  propose it. It is the third metric family to meet the sentence this way.

### The share-basis rule

Each multiple is built on the count through market capitalisation, so each
calls the rule, through the one function in
`crates/analyze/src/metrics/reading/price.rs`.

**E is Y's end,** the duration's end, as the share-basis record fixes for a
metric stated at a duration. It is also the date of the closing instant. T is
the close's trading date.

**The rule is asked once, with one E.**

- The rule governs a figure on a share basis or derived from one. The count
  is the only such figure either multiple reads. The two debts, the two cash
  concepts, `operating_income` and `depreciation_and_amortization` are in
  currency, and a split does not restate them.
- So asking the rule once per figure would ask it once anyway, with the E the
  share-basis record gives.

**The opening count reaches these multiples.** The averaged enterprise value
reads the count at the instant Y opens at, through market capitalisation. A
split dated after the opening instant and before the earlier of E and T
passes the rule, so the average can mix two share bases. The market
capitalisation record "names the exposure, pins it in a test, and leaves it
open", and it reaches these multiples through that record's term.

- It is smaller here than for price/sales, since net debt is unaffected and
  only the equity part of the term is understated. Both multiples still read
  low, which flatters.
- It is pinned the same way, in a test named below. Closing it is the
  superseding record the market capitalisation record names, or it falls away
  if the count is read at closing.

**What the caller must ask.** Each multiple at a year ending E needs a price
answered at or before E for the covered range to reach it. Asking every
period end the history holds does that, as the analyze record says. A caller
that asked less gets `not_covered`, never a number.

### Where each is stated

Each multiple is stated at each year it is computed at, beside the close at
the valuation date, as the market capitalisation record places its four.

- The provider record lists enterprise value and both multiples under the
  valuation date.
- So every year's multiple is that year's earnings and balances against
  today's close. Which period a ranking reads is M6's query over the stored
  table. Nothing here orders the periods or picks the latest.

### The `declined` conditions

Each is asked only where every read and the price read give a number. They
are asked in formula order, numerator first, and the first that holds is the
one reason. Each is a sign or zero test and needs no literal.

| Condition | Holds when | Metric | Defined |
| :--- | :--- | :--- | :--- |
| `non_positive_close` | the close is zero or below | both | once, beside the discounted cash flow margin; reached from there |
| `non_positive_shares_outstanding` | either count read is zero or below | both | once, beside the market capitalisation term; reached from there |
| `non_positive_enterprise_value` | the averaged enterprise value is zero or below | both | new, once, beside the enterprise value term |
| `zero_operating_income` | `operating_income` over Y is zero | EV/EBIT | new, once, beside `enterprise_value_to_ebit` |
| `negative_operating_income` | `operating_income` over Y is below zero | EV/EBIT | new, once, beside `enterprise_value_to_ebit` |
| `zero_ebitda` | EBITDA over Y is zero | EV/EBITDA | new, once, beside the EBITDA term |
| `negative_ebitda` | EBITDA over Y is below zero | EV/EBITDA | new, once, beside `enterprise_value_to_ebitda` |

The order for each metric:

- **EV/EBIT:** `non_positive_close`, `non_positive_shares_outstanding`,
  `non_positive_enterprise_value`, `zero_operating_income`,
  `negative_operating_income`.
- **EV/EBITDA:** `non_positive_close`, `non_positive_shares_outstanding`,
  `non_positive_enterprise_value`, `zero_ebitda`, `negative_ebitda`.

**The market capitalisation term's two** are asked where that term asks them,
in its one function. They are reached from their one definitions and defined
no second time. A reached constant's visibility is widened where it must be.

**An enterprise value of zero or below is a condition, and not a signed
figure.** No copy opened states either multiple there. This is the record's
own decision:

- **At zero** the multiple is zero whatever the earnings are. It says nothing
  of the denominator, and it is the cheapest figure the form can give.
- **Below zero** the multiple orders filers backwards. Of two filers with the
  same negative enterprise value, the one earning more has the multiple
  nearer zero, so it sorts as the dearer of the two. A screen for low
  multiples would rank the one earning less first.
- A multiple is the price paid for a unit of what the denominator measures.
  A negative enterprise value is no price paid: the buyer is handed more
  cash than the firm costs. That is worth knowing, and the reason names it.
  It is not a multiple of earnings.
- The test is on the enterprise value the multiple divides, which is the
  average. An enterprise value below zero at one instant and above at the
  other gives a figure where the average is above zero.
- **The cost.** A filer trading below its net cash is absent from both
  multiples. That is a filer a value investor most wants to find. It is
  found by its reason, and not by its rank. Whether M6 reads the reason is
  M6's.

**A zero denominator** is a zero divisor. `zero_operating_income` and
`zero_ebitda` are new, because no condition on main names either.

**A negative denominator is a condition, and not a signed figure,** under the
market capitalisation record's one rule for a negative denominator:

- That rule's reason: "A multiple is the price paid for a unit of what the
  denominator measures. Over a negative denominator it is a price for no
  unit, and its sign is the denominator's, not the price's." It holds for
  both multiples here.
- The chapter's page 46 treats a negative EBITDA as a firm the analysis
  loses: "there are far fewer firms with negative EBITDA than there are firms
  with negative earnings per share and thus fewer firms are lost from the
  analysis." A lost firm is an absent one, and not a negative multiple.
- No copy opened states EV/EBIT at a negative EBIT. The record applies the
  same rule to it as its own decision.
- **The cost.** Every year of operating loss is absent for EV/EBIT. Every
  year whose loss exceeds its depreciation and amortisation is absent for
  EV/EBITDA too. A year of operating loss smaller than its depreciation and
  amortisation has EV/EBITDA and no EV/EBIT, which is the chapter's point.

**Why each new condition stands where it does.**

- `non_positive_enterprise_value` is beside the term, since two multiples ask
  it and a later multiple of enterprise value would ask it too. The term's
  function does not ask it: the term gives a signed number.
- `zero_ebitda` is beside the EBITDA term, as `zero_free_cash_flow` is beside
  free cash flow and `zero_common_equity` beside common equity.
- `zero_operating_income` is beside `enterprise_value_to_ebit`, the first
  metric to divide by the concept, as `zero_earnings_per_share_diluted`
  stands beside the first metric that divided by its concept. A later metric
  that wants it reaches it there.
- The two negative conditions stand beside the multiple that asks each, as
  the market capitalisation record places its four: "the sign belongs to the
  multiple".

The entries also list the four year conditions, then
`share_basis_date_not_a_date`, as the share-basis record places it.
`input_not_a_decimal` reaches both from the one definition beside the reader.
No condition already defined is defined again.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first.** At a duration that is not a year, its one reason
   is the outcome and nothing below is asked.
2. **The history's reads, in input order.** Every read is made, and none
   stops another. Each gives a number or one reason: `input_unknown` or
   `input_not_applicable` naming the concept and the row read, `no_period`
   with `opening_instant` or `closing_instant`, or `declined` with
   `input_not_a_decimal`. The one exception is `short_term_investments`
   `NotApplicable`, which contributes nothing and gives no reason.
   - EV/EBIT: the five balances, each at Y's opening instant and then its
     closing instant, then `operating_income` at Y.
   - EV/EBITDA: the same, then `depreciation_and_amortization` at Y.
3. **The price read, last,** made whatever the reads gave, through the one
   function M5-54 built in `crates/analyze/src/metrics/reading/price.rs`,
   with E as above. It gives a number or exactly one reason, in the
   discounted cash flow record's order:
   - no crossing: `no_price` with `not_asked`;
   - an absence answered for the valuation date: `no_price` with
     `answered_absent`, carrying the date asked;
   - a price: the rule, giving `basis_unproven` with `splits_absent`,
     `not_covered` or `split_within` by the first clause that fails, or
     `declined` with `share_basis_date_not_a_date` where a date it compares
     is outside the grammar;
   - a close outside the decimal grammar: `declined` with
     `input_not_a_decimal`.

   The answer at the valuation date is read by its place in the crossing, so
   `date_not_asked` has no witness here. The read never falls back to another
   date, to an adjusted close or to a stand-in.
4. **The conditions,** in the order above, only where every read and the
   price read gave a number.

**A multiple built on a term carries the term's reasons.** Market
capitalisation's are the count's two reads and the price read. Net debt's are
its four concepts' reads. Enterprise value's are those together. EBITDA's are
its two flows' reads. No outcome says "the term was absent".

**With no key, no number.** Both multiples read a price. The caller with no
key asks nothing, so the crossing is `not_asked`, and each multiple is absent
with `no_price` and `not_asked`, after any reasons of its own reads. That is
GOALS.md's ninth M5 criterion: "A metric with no price data is absent with a
reason. It never fails the run and never shows a wrong number."

No seventh reason is needed, and no frozen byte changes.

### By kind

Read from `contracts/canonical-concepts/v2.toml`:

- `operating_income` applies to `operating` and `reit`.
- `short_term_investments` applies to `operating` alone.
- `shares_outstanding`, `short_term_debt`, `long_term_debt`,
  `cash_and_equivalents`, `preferred_equity` and
  `depreciation_and_amortization` apply to all five kinds.
- Every silence is `unknown` but that of `short_term_investments`, which is
  `zero`.

The catalogue's prose differs in two places, and v2 is what is read. The
prose applies `operating_income` to `operating` alone, where v2 adds `reit`.
It gives `short_term_debt` and `long_term_debt` a silence of zero, where v2
gives `unknown`.

| Kind | Where every other input is a `Value` |
| :--- | :--- |
| `operating` | both computed |
| `reit` | both computed, with `short_term_investments` contributing nothing at each instant it is `NotApplicable` |
| `bank`, `insurer`, `investment_company` | both absent with `input_not_applicable` for `operating_income` at Y, alone |

**A filer silent on a debt concept** has `Unknown` there, and both multiples
are absent with `input_unknown` naming the concept at the instant read. A
debt-free filer whose silence reads `Unknown` has neither multiple, unless a
registry override asserts the zero, as the capital record says of return on
invested capital. The same holds for `cash_and_equivalents`.

**A filer silent on `short_term_investments`** has a zero there, supplied
once per period as the silence records fix, and both multiples are figures
that net cash alone.

**A filer silent on `preferred_equity`** has both multiples as figures. The
concept is not read.

**A filer silent on `depreciation_and_amortization`** has EV/EBIT and no
EV/EBITDA, which is absent with `input_unknown` naming it at Y.

**Where the method is weakest.**

- **The three kinds it gives nothing for.** A `bank`, an `insurer` and an
  `investment_company` have no operating income in v2, so both multiples are
  absent. That is the right absence: a bank's borrowings are the material it
  lends, and net debt is not a claim on its operations in the sense the
  chapter's numerator means. The equity multiples are what these kinds have,
  as the market capitalisation record says of price/book.
- **A `reit`.** Its `short_term_investments` are `NotApplicable` and
  contribute nothing, so securities it holds outside cash are not netted and
  enterprise value is overstated by them. That is the conservative
  direction. Where it carries a preferred claim, that claim is not counted,
  which flatters.
- **A filer that consolidates subsidiaries it does not wholly own,** of any
  kind computed: both multiples read low, as the chapter's page 47 says.
- The kind does not cross v2. Declining a kind is the v3 record the analyze
  record names, and neither multiple needs it.

### The names

They cross `analyze-store` v1 as characters, under the common-shareholder
record's naming rule.

- **`enterprise_value_to_ebit`** and **`enterprise_value_to_ebitda`.**
  - Neither carries `common`. The rule puts that word in a name "exactly when
    its derivation forms common equity or income available to common".
    Neither derivation forms either. Market capitalisation is a common claim
    by its count, as `price_to_sales` reads it without the word.
  - Neither carries `per_share`. Each reads a count, which is not a
    per-common-share concept, and totals.
  - Each `meaning` says the equity inside enterprise value is the market
    value of the common shares, and that the preferred claim and
    non-controlling interests are not counted, so no reader takes it for an
    all-claims figure.
- The names follow the catalogue's "EV/EBIT" and "EV/EBITDA". EBIT is
  `operating_income`, and the `meaning` says so.
- **Enterprise value and EBITDA** have no name crossing the contract, since
  neither is a metric. Their modules are named `enterprise_value` and
  `ebitda`.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition and which periods or
dates a metric reads:

- the enterprise value term, its form, what it does not count, and its module
- the EBITDA term, its form and its module
- the two forms, EV/EBIT's reading of "Value" included
- that each is stated over a year, and every balance read averaged
- E for each multiple, and the rule asked once
- the two names and their meanings
- the five new conditions, and the order of all seven

A later change to any of them, a form, an instant read or a condition
included, is a version bump and a superseding record, and never an edit to
this one.

It lands at version 1. Neither multiple is computed yet, so no stored outcome
changes.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, and no new value of `unit`:

- **`unit`.** `Multiple`, for both: how many times the denominator the
  enterprise value is. Among the values `Unit` carries, it is the one the
  four equity multiples take. Enterprise value and EBITDA have no entry.
- **`stated_at`.** `Duration`, for both.
- **`inputs`,** in formula order, the price being no concept and not listed,
  each with its reading:
  - `enterprise_value_to_ebit`: `shares_outstanding`, `short_term_debt`,
    `long_term_debt`, `cash_and_equivalents` and `short_term_investments`,
    each averaged; then `operating_income`, a flow, with no reading.
  - `enterprise_value_to_ebitda`: the same six, then
    `depreciation_and_amortization`, a flow, with no reading.
- **`conditions`,** in the order asked: each metric's five from the order
  above, then `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year` by their constants, then
  `share_basis_date_not_a_date`. `input_not_a_decimal` is generated from its
  one definition.
- **`meaning`** says the form in words and the date of the close:
  - `enterprise_value_to_ebit`: "Enterprise value over operating income for
    the year. Enterprise value is the average, at the instants the year
    opens and closes at, of market capitalisation, the unadjusted close at
    the valuation date times common shares outstanding, plus net debt,
    short-term and long-term debt less cash and equivalents and short-term
    investments; the preferred claim and non-controlling interests are not
    counted."
  - `enterprise_value_to_ebitda`: the same enterprise value "over operating
    income plus depreciation and amortisation for the year."

### Sources

Under `sources-a-task-can-read.md` (Accepted). Each copy below was opened in
this run, downloaded whole and its text extracted inside the workspace with
`pdfminer.six`. Each transfer was closed early by the server and resumed. Each
quotation is as printed, checked against the copy with whitespace collapsed.
`pages.stern.nyu.edu` was read under a per-command allowance.

**The origin: Damodaran, A., *Investment Valuation*, 2nd ed., John Wiley and
Sons, 2002, chapter 18, "Earnings Multiples",** the author's own copy at
<https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch18.pdf>, 59 pages,
198,889 bytes, sha256
`a864e654892586e6d0ab936c5c0777cfd7fe1a23c4b8e9bd81dffc89e1652e28`. That is
the digest the market capitalisation record names, and this run computed it
again. Its printed page numbers match their PDF positions.

- Page 46: "Unlike the earnings multiples discussed so far in this chapter,
  the enterprise value to EBITDA multiple is a firm value multiple."
- Page 46: "there are far fewer firms with negative EBITDA than there are
  firms with negative earnings per share and thus fewer firms are lost from
  the analysis."
- Page 46: "this multiple can be compared far more easily across firms with
  different financial leverage".
- Page 46: "The enterprise value to EBITDA multiple relates the total market
  value of the firm, net of cash, to the earnings before interest, taxes and
  depreciation of the firm."
- **The equation on page 46** is set as an object whose words the extraction
  prints one to a line. The numerator stands on one baseline and reads, left
  to right, `Market`, `Value`, `of`, `Equity`, `+`, `Market`, `Value`, `of`,
  `-Debt`, `Cash`, over `EBITDA`. The minus sign is extracted ahead of the
  word it follows. This run read the words' positions from the page's layout
  and found them so. Read with the sentence's "net of cash", the numerator is
  the market value of equity, plus the market value of debt, less cash. The
  denominator is EBITDA.
- Page 46: "Since the interest income from the cash is not counted as part of
  the EBITDA, not netting out the cash will result in an overstatement of the
  true value to EBITDA multiple."
- Page 46: "When a holding is categorized as a minority holding, the
  operating income of a firm does not reflect the income from the holding."
- Page 47: "The EBITDA includes 100% of the EBITDA of the holding, but the
  numerator reflects only the percentage (not 100%) of the holding that
  belongs to the firm. Thus, the value to EBITDA will be too low, leading it
  to be categorized as an undervalued stock."
- Page 47: "The correction for cross-holdings is tedious and difficult to do
  when the holdings are in private firms."
- Page 47, the illustration: "The market value of equity at Segovia is $1,529
  million and the consolidated debt outstanding at the firm is $500 million."
  "None of the firms have significant cash balances." "LatinWorks is a
  private firm with an EBITDA of $120 million on capital invested of $250
  million in the current year".
- Page 48 works the equation with those figures: `1529`, `+`, `500` and `0`
  over `500`. That is equity plus debt less cash, over EBITDA, and agrees
  with the reading of page 46.
- Page 49: "we estimated the value of the operating assets (or enterprise
  value) of a firm." And, of the flow that value is formed from: "FCFF = EBIT
  (1-t)".
- Page 50, the next line of the same derivation, opens "= (EBITDA – DA)
  (1-t)".
- Page 51: "The firm is in stable growth and its operating income is expected
  to grow 5% a year in perpetuity."
- Page 55: "multiples and fundamentals by industry group in the United States
  for the most recent year".
- Page 55: "While enterprise value to EBITDA may be the most widely used
  value multiple, there are close variants that are sometimes used by
  analysts", of which the first the page names is "Value/EBIT".

**Beside it: Damodaran, A., *Valuation Approaches and Metrics: A Survey of
the Theory and Evidence*, Stern School of Business, New York University,
November 2006,**
<https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
pages, 1,602,127 bytes, sha256
`9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. That is
the digest the market capitalisation record names, and this run computed it
again.

- Page 26: "Free Cash Flow to Firm = After-tax Operating Income – (Capital
  Expenditures – Depreciation) – Change in non-cash Working Capital".
- Page 29: "The most common of the non-operating assets is cash and
  marketable securities".
- Page 29, cited under what no check can see: "The non-equity claims that
  have to be subtracted out include not only all debt, but all capitalized
  leases as well as unfunded pension plan and health care obligations."
- The sentence that opens at the foot of page 59, "When buying a business,",
  and runs on at the head of page 60: "as opposed to just the equity in the
  business, it is common to examine the value of the firm as a multiple of
  the operating income or the earnings before interest, taxes, depreciation
  and amortization (EBITDA)."
- Page 60: "For firm value, this ratio can be modified as the enterprise
  value/to sales ratio (VS), where the numerator becomes the market value of
  the operating assets of the firm."

**Net debt's and market capitalisation's forms** are cited from their one
modules, as their records cite them. This record opens nothing more for
them.

**Where no source fixes a form, an instant or a condition,** it is the
record's own decision, with its reason above:

- that "Value" in the chapter's "Value/EBIT" is the enterprise value of page
  46, and that EV/EBIT is stated over a year
- debt at its carrying amount, and `short_term_investments` netted beside
  cash
- that the preferred claim is not counted
- every balance averaged, where the period rule reaches it
- an enterprise value of zero or below as a condition
- a zero or negative EBIT as conditions, and a zero EBITDA as one

No copy opened gave another digest, and none states a form otherwise than the
task quotes it. This record names no source outside the repository but these
two.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at`, and a `Crossing`
built as the contract type. They read no file. Each stands in `tests/` or
under `#[cfg(test)]`.

**The worked history.** The year 2024-01-01 to 2024-12-31, and the instants
2023-12-31 and 2024-12-31. Balances are given opening/closing.

- `shares_outstanding` 90/110.
- `short_term_debt` 100/200 and `long_term_debt` 900/1300.
- `cash_and_equivalents` 300/400 and `short_term_investments` 100/100.
- `operating_income` 800 and `depreciation_and_amortization` 200 over the
  year.

**The preset crossing** is the share-basis record's: a valuation date of
2025-06-30 answered with a close of 60 traded 2025-06-30, the earlier date
2024-12-31 answered with a close traded 2024-12-31, and an empty split list.
E is 2024-12-31 for every figure below.

At the least:

- **The figures, worked by hand.**
  - Market capitalisation is 60 × 90 = 5400 at the opening and 60 × 110 =
    6600 at the closing.
  - Net debt is 100 + 900 − 300 − 100 = 600 at the opening and 200 + 1300 −
    400 − 100 = 1000 at the closing.
  - Enterprise value is 6000 at the opening and 7600 at the closing. Its
    average is 6800, which is also 60 × 100 + 800.
  - `enterprise_value_to_ebit` over 2024: 6800 ÷ 800 gives `8.5`.
  - EBITDA is 800 + 200 = 1000. `enterprise_value_to_ebitda` over 2024: 6800
    ÷ 1000 gives `6.8`.
- **The other reading of each instant,** which the figures are never:

  | Count | Net debt | Enterprise value | EV/EBIT | EV/EBITDA |
  | :--- | :--- | :--- | :--- | :--- |
  | averaged | averaged | 6800 | `8.5` | `6.8` |
  | closing | closing | 7600 | `9.5` | `7.6` |
  | closing | averaged | 7400 | `9.25` | `7.4` |
  | averaged | closing | 7000 | `8.75` | `7` |

  The first row is the record's. The second is the reading the term wants.
- **The exception, and its count.**
  - `short_term_investments` `NotApplicable` at both instants: net debt
    700/1100, enterprise value 6900, `8.625` and `6.9`.
  - `NotApplicable` at the opening and 100 at the closing: net debt 700/1000,
    enterprise value 6850, `8.5625` and `6.85`. It is never `8.5` and `6.8`,
    which counting one read would give.
- **Each `declined` condition.**
  - A close of 0, and of -1: `non_positive_close`, for both.
  - `shares_outstanding` 90/0, and 0/110: `non_positive_shares_outstanding`,
    for both.
  - `cash_and_equivalents` 7900/6400: enterprise value is −1600/1600 and its
    average 0, so `non_positive_enterprise_value`, for both. 7900/6600: the
    average is −100, and the same, never `-0.125` and `-0.1`.
  - **The condition is on the average.** 7900/6200: enterprise value is
    −1600/1800 and its average 100, so `0.125` and `0.1`.
  - `operating_income` 0: `zero_operating_income` for EV/EBIT, and EV/EBITDA
    is 6800 ÷ 200, `34`.
  - `operating_income` −100: `negative_operating_income` for EV/EBIT, and
    EV/EBITDA is 6800 ÷ 100, `68`.
  - `operating_income` −200: `negative_operating_income`, and `zero_ebitda`.
  - `operating_income` −800: `negative_operating_income`, never `-8.5`, and
    `negative_ebitda`.
  - **The first that holds.** A close of 0 beside `operating_income` 0 gives
    `non_positive_close` alone. `cash_and_equivalents` 7900/6400 beside
    `operating_income` 0 gives `non_positive_enterprise_value` alone.
- **No price, by each witness,** for both:
  - No crossing: `no_price` with `not_asked`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying 2025-06-30.
  - `date_not_asked` has no witness: the test shows `Crossing` cannot be
    built without `at_valuation_date`.
- **The share-basis rule, by each clause,** for both:
  - The split list absent: `splits_absent`.
  - The valuation date alone, no earlier date answered: `not_covered` naming
    2025-06-30 through 2025-06-30, 2024-12-31 and 2025-06-30.
  - A split dated 2025-03-14: `split_within` naming it, 2024-12-31 and
    2025-06-30. A split dated 2024-12-31: `split_within`.
- **A date outside the grammar.** The paired close's trading date
  `2025-6-30`: `declined` with `share_basis_date_not_a_date`, for both.
- **An unreadable close.** `1e2`: `declined` with `input_not_a_decimal`.
- **The exposure, pinned.** `shares_outstanding` 55/110 and a split dated
  2024-06-15, beside the preset crossing's answers: the rule passes,
  enterprise value is 3900/7600 and its average 5750, EV/EBIT is `7.1875` and
  EV/EBITDA `5.75`. On one share basis, 110/110, they would be `9.25` and
  `7.4`. The test names this as the split before E that the rule does not
  see, so that a superseding record changes it knowingly.
- **A filer silent on each debt concept.** `short_term_debt` `Unknown` at
  2024-12-31: `input_unknown` naming it at 2024-12-31, for both, and never a
  figure over a net debt without it. The same for `long_term_debt`.
- **A filer silent on `short_term_investments`.** A zero at both instants:
  `8.625` and `6.9`. `Unknown` at the closing: `input_unknown` naming it at
  2024-12-31, for both.
- **A filer silent on `preferred_equity`.** `Unknown` at both instants gives
  `8.5` and `6.8`. A `preferred_equity` of 500 at both instants gives `8.5`
  and `6.8` too: the claim is not read.
- **A filer silent on `depreciation_and_amortization`.** `Unknown` over the
  year: EV/EBIT is `8.5`, and EV/EBITDA is `input_unknown` naming it at the
  year.
- **A filer of each kind.** The worked history with every concept v2 does not
  apply to that kind `NotApplicable`:
  - `operating`: `8.5` and `6.8`.
  - `reit`: `8.625` and `6.9`.
  - `bank`, `insurer` and `investment_company`: `input_not_applicable` for
    `operating_income` at the year, alone, for both. `short_term_investments`
    gives no reason.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31 with every input
  `Unknown`: `not_a_year` alone, for both.
- **The order of reasons.** The instant 2023-12-31 missing,
  `depreciation_and_amortization` `Unknown`, and no crossing. EV/EBITDA gives
  `no_period` with `opening_instant` for each of the five balances in input
  order, then `input_unknown` for depreciation and amortisation, then
  `no_price` with `not_asked`. EV/EBIT gives the five and the last.
- **Each term still formed once.** `debt_to_equity`,
  `return_on_invested_capital`, `discounted_cash_flow_value_per_share`,
  `price_to_common_book`, `price_to_sales` and `price_to_free_cash_flow` give
  their pinned figures and reasons unchanged, each of the last two where it
  is on main.
- **The catalogue.** Each entry's `inputs` and `conditions` are as listed
  above. Neither entry's `inputs` holds `preferred_equity`. No entry is named
  for enterprise value or for EBITDA. Neither name holds `common` or
  `per_share`.

### What no check can see

- **Whether the sources say what this record says.** The chapter and the
  survey were opened in this run and are quoted as printed. The implementing
  tasks cite each as the sources record says. A copy that on reading says
  otherwise is a stop for that task, not a choice.
- **Debt at its carrying amount, and not its market value,** as stated above.
- **The claims v2 does not carry.**
  - Non-controlling interests: both multiples read low for a filer that has
    them.
  - The preferred claim, which v2 carries and this record does not count:
    both read low for a filer that has one.
  - The operating leases the open-questions record leaves out of debt. They
    are out of enterprise value, and their cost is inside `operating_income`,
    so each lease is counted once. A filer that leases is still not
    comparable with one that borrows to buy.
  - The survey's page 29 names "unfunded pension plan and health care
    obligations" among the non-equity claims. v2 has no concept for them.
    Enterprise value is understated by them, which flatters.
- **The cross-holdings of the chapter's pages 46 and 47.** A minority holding
  makes the multiple read high. A majority holding makes it read low. v2
  names neither.
- **All cash is netted,** the cash the business needs to run included, and
  securities classified as non-current are not, as stated above.
- **Whether the depreciation and amortisation read was all charged above
  operating income.** v2 reads it from the cash flow statement's
  reconciliation where the income statement does not present it. A charge in
  that line that `operating_income` never bore is added back all the same.
  EBITDA is then overstated and the multiple reads low.
- **What `operating_income` holds.** It is the line "as the income statement
  presents it". A year's restructuring charge or impairment is inside it, and
  nothing here adjusts for one.
- **The averaged enterprise value,** where the value at the valuation date is
  wanted, and the split before E inside the averaged count, as stated above.
- **A figure at Y beside a close at the valuation date,** as the market
  capitalisation record states it. The earnings and the balances are not
  rolled forward. Where Y ends after the valuation date, each multiple is
  `not_covered`. Where it ends long before, an old year's multiple is stated
  beside today's close, and nothing marks it.
- **The class of a share, the count's date and the currency,** each as the
  market capitalisation record states it of its own term.
- **A metric that forms enterprise value or EBITDA itself.** Only review sees
  it.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  derivation needs no literal: the average's count of two steps by one, as
  the period record states it, and every test is a sign or zero test. No
  constant and no setting is added.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule and catalogue
- the share-basis record's comparisons and condition, and its E for a metric
  stated at a duration
- the arithmetic record's reading, operations and written form
- the series record's year test
- the period record's table and rule, read as written
- the capital record's terms, module and net debt's one exception, applied
  and not extended
- the common-shareholder record's terms, names and conditions, and the
  open-questions record's admission of `preferred_equity`, which gains no
  consumer
- the discounted cash flow record's metrics, module and bridge, its
  `non_positive_close` reached and not moved
- the market capitalisation record's term, module, four metrics and
  conditions
- the provider record's ask and answer
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains no value of `unit` and no field

No sentence of an accepted record is superseded. Three questions here would
need an accepted record or a contract to change, and each is left open and
named:

- **Whether the averaging sentence reaches the balances of a value the close
  is set beside.** The period record's "Every other metric that pairs a flow
  with a balance reads the average. This holds whatever source its formula
  cites." reaches them as written. Reading them at closing is a superseding
  record of its own, and would answer the same question for the count in
  price/sales and for net debt in the bridge.
- **Whether the rule sees a count read before E.** The share-basis record's
  "For a metric stated at a duration, as both discounted cash flow metrics
  are, E is the duration's end" leaves the opening count unseen. That is the
  market capitalisation record's question, and it reaches these multiples.
- **A concept for non-controlling interests.** It needs a
  `canonical-concepts` v3, proposed only in a record of its own.

**Why Structural.** The record fixes the names and meanings of two metrics
that cross `analyze-store` v1, which store and M6 will read, and two terms
later metrics may share. It needs nothing Constitutional: no anchor, no gate,
no protected path and no GOALS.md scope moves.

## Alternatives

- **Enterprise value with the preferred claim counted,** as
  `preferred_equity` added beside net debt. It is the fuller form, and the
  earnings it is set over are before every financing claim, the preferred
  one included. Rejected here:
  - No copy opened puts the preferred claim in the numerator.
  - The concept's silence is `Unknown`, so both multiples would be absent for
    every filer silent on it, which is most of them, to correct a claim few
    carry. The discounted cash flow record refused the same read for the
    same reason.
  - It would make enterprise value and the bridge count different claims.
  - It would give `preferred_equity` a consumer the open-questions record
    did not admit.

  This is the item most worth the decider's judgement. The cost of not
  counting it flatters, and is named.
- **The preferred claim counted where it is a `Value`, and contributing
  nothing where it is `Unknown`.** It would cost no silent filer. Rejected:
  it reads an `Unknown` as a zero, which is the substitution the accepted
  records forbid, and it extends the one exception from a `NotApplicable`
  `short_term_investments` to a second concept and a second state. A record
  that argues that reading argues it for the bridge too, and is a record of
  its own.
- **Non-controlling interests as `total_assets` less `total_liabilities` less
  `shareholders_equity`.** v2 can form it. Not rejected on its merits, and
  not here: the difference also holds redeemable preferred outside permanent
  equity, it is a carrying amount and not a market value, and it adds two
  consumers. The capital record leaves the same form to a superseding record
  argued against a v3 concept.
- **Net debt and the count at the year's closing instant.** This is the
  reading the term wants: the value of the firm at the date nearest the
  close. Not rejected on its merits. It reads an accepted rule narrowly,
  against how the market capitalisation record and the discounted cash flow
  record each read it, so it is a record of its own.
- **The count at closing and net debt averaged,** or the reverse. Rejected:
  it reads one sentence two ways inside one term.
- **Cash alone netted,** as the chapter's one word says. It would read one
  concept fewer and need no exception. Rejected: the capital record forms net
  debt with both, and a second net debt would be a second definition of one
  term. The chapter's reason for netting cash reaches the securities too, and
  the survey names them together.
- **Enterprise value as a metric.** Rejected here: it needs a `unit` of
  currency that `Unit` does not carry, and GOALS.md names it as no multiple.
  The value is named and left open. A later record can add it, beside the
  term.
- **EBITDA formed inside the multiple.** It has one reader today. Rejected:
  the catalogue names EBITDA as a derived value "and through it EV/EBITDA",
  which is a term with a reader. A second reader, such as net debt to EBITDA,
  would then form it a second time. Market capitalisation had its module
  before its first reader for the same reason.
- **EV/EBIT left out, for want of a stated numerator.** Neither copy writes
  "Value/EBIT" out. Rejected: the catalogue and the provider record both name
  EV/EBIT, the chapter names it as a close variant of a multiple whose
  numerator it does state, and the record's reading is marked as its own.
- **EV/sales.** The survey's page 60 names it: "the enterprise value/to sales
  ratio (VS)". Rejected here: GOALS.md and the catalogue do not name it, and
  the catalogue lists `revenue` for price/sales alone among the multiples. It
  would read the one term this record forms. A later record can add it as a
  third metric, and adding a metric bumps nothing.
- **A negative multiple as a signed figure,** over a negative EBIT or EBITDA.
  The arithmetic gives it. Rejected: it sorts below every real multiple and
  ranks the filer losing the most first, the chapter counts such firms as
  "lost from the analysis", and the market capitalisation record settled one
  rule for a negative denominator.
- **A negative enterprise value as a signed figure.** A negative multiple
  does mark a filer priced below its net cash, and it sorts first in a screen
  for low multiples, which is where a value investor would look. Rejected:
  among such filers it orders backwards, the more earnings the dearer, and at
  zero it says nothing of the earnings. The reason names the state without a
  number that misorders. A human may want to revisit it.
- **`non_positive_enterprise_value` asked inside the term,** as market
  capitalisation asks its two. Rejected: a negative enterprise value is a
  true value of the term, where a non-positive close or count is no price and
  no count. A later reader of the term that is not a multiple may want the
  sign.
- **One condition for a zero and a negative denominator,** as
  `non_positive_operating_income`. Rejected: the market capitalisation record
  keeps the zero divisor and the sign as two conditions for every multiple,
  and one family keeps one rule.
- **Each multiple at every duration.** Rejected: a quarter's multiple is
  about four times a year's, and the chapter states the earnings by the year.
- **A trailing multiple over the last four quarters.** Rejected here: it adds
  four quarters, which no record forms.
- **The rule asked once per count read, with E at the opening instant too.**
  It would see the split inside Y. Not rejected on its merits: it sets aside
  the share-basis record's E for a metric stated at a duration, so it is a
  record of its own.
- **Leaving both multiples out of M5 until the provider is built.** Rejected:
  a test builds the crossing as the contract type, so each figure and each
  `no_price` is provable today, and the price read, the market capitalisation
  module and the net debt function are on main. GOALS.md's ninth M5 criterion
  asks for a metric with no price to be absent with a reason, which is what
  each multiple is until a key is supplied.
- **Each multiple forming enterprise value itself.** Rejected: two readings
  of one sum, which the one-source-of-truth invariant forbids.

## Consequences

**Easier:**

- The two firm value multiples the catalogue names have their forms, periods,
  instants, conditions, names and sources. Each implementing task has only
  its code left to decide.
- Enterprise value and EBITDA are each formed once before a second reader
  arrives.
- The leverage the market capitalisation record names in price/sales and
  price/free cash flow is removed in these two: the numerator is the firm's
  and so is the denominator.
- A user with no key sees each multiple absent with `no_price`, and every
  other metric computed.

**Harder:**

- **Each multiple needs six or seven concepts at eleven or twelve reads,**
  both instants of Y included. Both debts and `cash_and_equivalents` read
  silence as `Unknown`, so a debt-free filer with no override is absent.
- **Every bank, insurer and investment company is absent from both.**
- **Every year of operating loss is absent for EV/EBIT,** and every filer
  priced at or below its net cash is absent from both.
- **Both multiples read low,** which flatters, wherever a filer carries a
  preferred claim, non-controlling interests or unfunded pension obligations,
  and wherever its count or net debt rose over the year.
- **The multiples wait on one module each:** the enterprise value term, and
  for EV/EBITDA the EBITDA term.

**Expensive to reverse:**

- The two names and their forms, once results are stored and ranked on.
- What enterprise value counts. Adding the preferred claim or
  non-controlling interests later raises every stored figure for the filers
  that carry them, under a bump.
- A non-positive enterprise value and a negative denominator as conditions.
  Turning either into a figure later makes stored absences into numbers,
  under a bump.

**Open, and not this record's:**

- Whether the period record's averaging sentence reaches the balances of a
  value the close is set beside.
- Whether the share-basis rule sees a count read before E.
- A concept for non-controlling interests, and a reading of the preferred
  claim that does not cost every silent filer.
- `Currency` as a value of `Unit`, if enterprise value becomes a metric.
- EV/sales, and a trailing multiple.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **No literal.** `literals` keeps every number but zero and one out of the
  derivation. The multiples need none.
- **One definition of enterprise value, and of EBITDA.** The proof's figures
  hold both multiples to the two modules, and the pinned figures of the six
  metrics named above hold net debt and market capitalisation to theirs.
  - **Gap:** nothing forces a metric through a module rather than around it,
    as the market capitalisation record says of its own.
- **One definition of each condition.** Each new condition is a `const`
  beside its term or metric, and each reached one is the existing `const`.
  - **Gap:** nothing stops a second `const` with the same characters, except
    review.
- **The preferred claim is not read.** The catalogue test holds
  `preferred_equity` out of both entries' `inputs`, and the test with a
  `preferred_equity` of 500 holds the figures.
- **No wrong number across a split.** The share-basis tests hold it for both
  multiples at E.
  - **Gap:** the opening count of the averaged read, pinned by a test as the
    exposure it is.
- **No multiple without a price.** The witness tests hold it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
