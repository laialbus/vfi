# Market capitalisation is the valuation date's close times common shares outstanding, formed once and handed over nowhere; price/earnings sets the close over a year's diluted earnings per share, and price/book, price/sales and price/free cash flow set market capitalisation over common equity at an instant or a year's revenue or free cash flow; each declines at a zero or negative denominator

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-11-07, by `M5-58`
- **Decided:** 2026-11-08, by the decider
- **Touches:** the valuation multiples that GOALS.md's fourth M5 criterion
  names, for the four whose numerator is the market value of equity, and the
  market capitalisation three of them share. It fixes the names and meanings
  of four metrics that cross `analyze-store` v1 as characters, and no byte of
  that contract changes. It forms market capitalisation once, in a module of
  its own, as a term handed over nowhere. It applies the accepted `declined`
  reason to five new conditions, reaches five existing ones from their one
  definitions, and adds no reason. It reads the price as the analyze record
  and `share-basis-rule-dates.md` (Accepted) fix, through the one function
  `M5-54` builds. It takes every operation from the arithmetic record's table.
  It names, and leaves open, two questions about accepted records: whether
  the period record's averaging sentence should reach a count the price
  multiplies, and whether the share-basis rule should see a count read at an
  instant before E. It adds no value of the catalogue entry's `unit` and no
  field, and names the value market capitalisation would need as a metric.
  Enterprise value and its two multiples are left to a record of their own.
  No accepted record is superseded. No contract's bytes, no schema, no edge,
  no gate and no dependency move. Nothing here is implemented.

## Context

GOALS.md's fourth M5 criterion names "valuation multiples" among the value
metrics. Every record around them is accepted, and none forms them.

Some things are already fixed, and this record restates them as fixed:

- **The concepts.** `canonical-concepts.md` (Accepted) names:
  - `shares_outstanding`: "common shares outstanding at a stated date at or
    near the period end. A count at an instant, not an average, and not
    interchangeable with the one above." Consumed by "market capitalisation,
    and through it price/book, price/sales, enterprise value and Altman's X₄".
    v2 gives it measure `balance` and unit `shares`.
  - `earnings_per_share_diluted`: "profit per share on a fully diluted basis,
    as the filer reports it. A flow over the period." Consumed by
    "price/earnings".
  - `revenue`, consumed by "price/sales"; `operating_cash_flow`, consumed by
    "free cash flow, and through it price/free cash flow"; and
    `shareholders_equity`, consumed by "price/book".
  - Of "Price and market capitalisation. They are not in filings and do not
    cross this boundary."
- **Price/book is common.** `canonical-concepts-open-questions.md` (Accepted)
  says carrying no preferred claim "cannot be made coherent for price/book,
  whose numerator is market capitalisation and is common-only by
  construction. For a filer with preferred outstanding, price/book then
  divides a common claim by a common-and-preferred book: it understates, and
  the filer reads as cheaper than it is."
- **The common-shareholder record.** `common-shareholder-forms.md` (Accepted):
  - fixes the row "Price/book | common alone | `price_to_common_book`";
  - says "Price/book sets market capitalisation, a common claim by
    construction, over book. Over total equity it is the incoherent figure the
    open-questions record refused.";
  - lists "price/earnings" among the metrics "Common by the concept they read,
    with no term to form", each of which "reads a per-common-share concept and
    needs nothing here";
  - defines `zero_common_equity` "beside the common equity term and used by
    `return_on_common_equity` and `price_to_common_book`";
  - says "Whether `price_to_common_book` declines at a negative common equity
    is its own task's. The accepted record leaves P/E on non-positive earnings
    to P/E the same way.";
  - says that for a filer silent on `preferred_equity`,
    "`price_to_common_book` and `sustainable_growth_on_common_equity` have no
    total form, so they are simply absent.";
  - fixes the naming rule: "`common` appears in a metric's name exactly when
    its derivation forms common equity or income available to common";
    "`per_share` appears in the name of a form that reads per-common-share
    concepts. Its `meaning` says "per common share", or "each common share's"";
    and "A total form has neither word."
  - says "If enterprise value wants `preferred_equity`, that is a new consumer
    the open-questions record did not admit, and its own record."
- **The analyze record.** `what-analyze-takes-and-returns.md` (Accepted):
  - of `declined`, "Examples are a zero divisor and P/E on non-positive
    earnings. Each metric's task lists its own conditions.";
  - a price-dependent metric "reads the valuation date's answer from the
    crossing's answer at the valuation date";
  - the share-basis rule's three clauses, with "Market capitalisation, and
    everything built on it, falls under the rule through
    `shares_outstanding`.";
  - "A per-share metric at the valuation date pairs a figure at an earlier
    period end with the valuation date's close."
- **The share-basis dates record.** `share-basis-rule-dates.md` (Accepted)
  fixes the rule's comparisons through the one ordering in `date.rs`, and the
  condition `share_basis_date_not_a_date`, which every metric that calls the
  rule lists after the four year conditions. It says: "For a metric stated at
  a duration, as both discounted cash flow metrics are, E is the duration's
  end", and "For a metric stated at an instant, or at another duration, an E
  outside the grammar gives `share_basis_date_not_a_date`."
- **The provider record.** `price-provider.md` (Accepted) lists under the
  valuation date "price/earnings, through `earnings_per_share_diluted`" and
  "market capitalisation, through `shares_outstanding`, and through it
  price/book, price/sales, price/free cash flow, enterprise value (so EV/EBIT
  and EV/EBITDA) and Altman's X₄, the market value of equity". It lists the
  past period ends "for relative yield only". It fixes the close as
  unadjusted, and says "Dividing it by an adjusted close divides by a number
  on another basis. The result is off by the split ratio and looks exactly
  like a real multiple."
- **Free cash flow.** `discounted-cash-flow-and-margin-of-safety.md`
  (Accepted) forms it once in `crates/analyze/src/metrics/free_cash_flow.rs`:
  "Every metric that reads free cash flow reads it there: this one,
  `dividend_payout_on_free_cash_flow`, and the price/free-cash-flow multiple
  when it is built. No metric writes the subtraction itself." It lists the
  price read's reasons in one order: no crossing, an absence answered, the
  rule, then the close's grammar. It states its margin "At each year Y where
  the value per share is stated, beside the close at the valuation date",
  with "Nothing here orders the years or picks the latest." Its margin's
  `non_positive_close` is defined once, beside that margin, and the dividend
  discount record (Accepted) already reaches it from there.
- **The period record.** `which-other-period-a-metric-reads.md` (Accepted):
  "A metric whose formula pairs a flow with a balance is stated at a
  duration." "A metric stated at an instant reads only balances, at its own
  row." And: "Every other metric that pairs a flow with a balance reads the
  average. This holds whatever source its formula cites. A balance its source
  names without saying when is read at closing, since that is the balance the
  period's own statement presents." The rule reads each input's measure from
  v2.
- **The series record.** `which-duration-precedes-a-duration.md` (Accepted):
  "A metric is stated over a year when its source states it over a fiscal
  year." At a duration that is not a year, such a metric gives exactly one of
  `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year`, and makes no read.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written,
  and a table of operations: multiplication is "Always exact", and a zero
  divisor is `declined` under the condition the entry names.
- **The price read is being written.** `M5-54` builds
  `crates/analyze/src/metrics/reading/price.rs`: the share-basis rule as one
  function, and the reading of the close at the valuation date as one
  function from the crossing, or its absence, and E to the close or exactly
  one reason. No metric calls either yet. `M5-55` builds the discounted cash
  flow margin and its `non_positive_close`.
- **Absence is closed.** Six reasons. A metric built on a term carries the
  term's reasons.
- **The entry shape.** M5-18 fixed it: name, meaning, `unit`, `stated_at`,
  inputs in reason order, and the metric's own `declined` conditions. `Unit`
  carries `FractionOfOne`, `Multiple`, `Years`, `CurrencyPerShare` and
  `Signals`, and no unit of currency.

## Decision

### Four metrics and one term

| Name | What it is | Unit | Stated at |
| :--- | :--- | :--- | :--- |
| `price_to_earnings_per_share` | The close at the valuation date over diluted earnings per common share for a year | `Multiple` | a year |
| `price_to_common_book` | Market capitalisation at an instant over common equity at that instant | `Multiple` | an instant |
| `price_to_sales` | Market capitalisation over a year's revenue | `Multiple` | a year |
| `price_to_free_cash_flow` | Market capitalisation over a year's free cash flow | `Multiple` | a year |

**Market capitalisation** is the term three of them share. It is not a
metric of the method.

All four read a price. None reads a setting: each form is fixed by its source
or by this record, and none has a free parameter.

### Market capitalisation, formed once

**The form.** The close at the valuation date times `shares_outstanding`.

- Damodaran's chapter 19, page 2, and chapter 20, page 2, each set "Market
  Value of Equity" over the denominator. The survey's page 59 says "the price
  per share of a stock is a function both of the value of the equity in a
  company and the number of shares outstanding in the firm." So the market
  value of equity is the price per share times the count. No copy opened
  states the product in symbols. The product is the record's reading of these
  pages.
- The count is `shares_outstanding`, as the catalogue names it for market
  capitalisation. It is common shares, so the term is a common claim, as the
  open-questions record says.
- The inputs are read in this order: `shares_outstanding`, then the price.
  The price is no concept.

**Where it lives.** One module of the derivation,
`crates/analyze/src/metrics/market_capitalisation.rs`. Every metric that
reads market capitalisation reads it there: price/book, price/sales and
price/free cash flow here, and enterprise value and Altman's X₄ when they are
built. No metric writes the product itself. The term is formed from the close
the price read gives and the count the reader read, at each row that count
was read at, before any mean. So an averaged market capitalisation is the
average of the term, as the common-shareholder module forms its terms.

**It is a term, handed over nowhere,** as net debt and free cash flow are.

- **Why not a metric.** As a metric it would need an entry with a `unit` of
  currency. `Unit` carries none. That value, `Currency`, is named here and
  left open. This record does not add it.
- GOALS.md names "valuation multiples", and market capitalisation is not one.
  Its consumers are the multiples, enterprise value and Altman's X₄, each of
  which reads it as a term.
- A later record may make it a metric, for a size screen in M6, adding
  `Currency` to `Unit`. Adding a metric bumps nothing, and the term's module
  stays where it is.

**Its conditions.** Each is asked only where the price read and every count
read gave a number, in formula order:

- **`non_positive_close`**, the close is zero or below. It is the discounted
  cash flow margin's condition, reached from its one definition, as the
  dividend discount margin reaches it. Over a zero close every multiple is
  zero, the cheapest figure the form can give. A negative close is no price
  paid. The test is a sign test.
- **`non_positive_shares_outstanding`**, new, defined once beside the term:
  a count read is zero or below. A zero count makes the market value zero,
  which reads as a free filer. A negative count is no count. Where the
  reader averages, it holds where either count is zero or below: averaged in,
  a zero halves the term and looks like a real figure. The test is a sign
  test.

### The derivations, in formula order

Every step is an operation in the arithmetic record's table. No root, power,
logarithm or literal enters.

| Metric | Formula | Operations |
| :--- | :--- | :--- |
| `price_to_earnings_per_share` | P ÷ `earnings_per_share_diluted` over Y | division |
| `price_to_common_book` | (P × `shares_outstanding` at I) ÷ (`shareholders_equity` at I less `preferred_equity` at I) | multiplication, subtraction, division |
| `price_to_sales` | (P × the average of `shares_outstanding` at Y's opening and closing instants) ÷ `revenue` over Y | addition, division by the count of two, multiplication, division |
| `price_to_free_cash_flow` | (P × that average) ÷ (`operating_cash_flow` less `capital_expenditure` over Y) | addition, division, multiplication, subtraction, division |

P is the close at the valuation date. Y is a year. I is an instant.

- **Price/earnings** is the chapter's form: "The price earnings ratio is the
  ratio of the market price per share to the earnings per share." P is the
  market price per share. The diluted figure is the one v2 carries and the
  chapter's "fully diluted earnings per share" is one of its named variants.
  P/E reads no count, so it reads no market capitalisation.
- **Price/book** is the chapter's total form, "using the total market value
  of equity and book value of equity, rather than per share values", over the
  common equity formed once in `crates/analyze/src/metrics/common_shareholder.rs`:
  `shareholders_equity` less `preferred_equity`. The chapter's page 2 says
  "You should not include the portion of the equity that is attributable to
  preferred stock in computing the book value of equity, since the price per
  share refers only to common equity." The total form needs no book value per
  share, which v2 does not carry.
- **Price/sales** is the chapter's form, "the multiple of the market value of
  equity to the revenues of a firm".
- **Price/free cash flow** is market capitalisation over free cash flow, read
  from `crates/analyze/src/metrics/free_cash_flow.rs`. No metric writes the
  subtraction itself.

**Price/free cash flow's form is this record's own decision.** No copy the
task opens defines the multiple. The discounted cash flow record names it,
"the price/free-cash-flow multiple", and places its free cash flow in the one
module. The form here sets the market value of equity over the flow, as the
chapter's price/sales sets it over revenue, for three reasons:

- It reuses the one term and the one module, so no second product and no
  second subtraction is written.
- v2 carries no free cash flow per share. The close over free cash flow per
  share, divided by the same count, is the same exact number, so the per-share
  form would add a division and no information.
- The flow is the firm's, before debt service where the filer classifies
  interest in financing, and the numerator is equity alone. The chapter's
  page 2 says the same of price/sales: "The price to sales ratio divides an
  equity value by revenues that are generated for the firm. Consequently, it
  will yield lower values for more highly levered firms". That holds here,
  and is stated under "What no check can see".

### Which periods

**Price/earnings is stated over a year,** under the series record's test.

- The chapter's page 1 says "PE ratios could be computed using current
  earnings per share, trailing earnings per share, forward earnings per share,
  fully diluted earnings per share and primary earnings per share." Its page
  27 defines the first: "If the expected growth rate in earnings per share is
  based upon earnings in the most recent year (current earnings), the PE ratio
  that should be used is the current PE ratio."
- The survey's page 59: "This price/earnings ratio can be estimated using
  current earnings per share, yielding a current PE, earnings over the last 4
  quarters, resulting in a trailing PE, or an expected earnings per share in
  the next year, providing a forward PE."
- Each reading the sources name is a year's earnings: the most recent year's,
  the last four quarters', or the next year's. None is a quarter's. A forward
  figure is a forecast and not in filings. A trailing figure adds four
  quarters, which no record forms. So the figure is the current P/E, over a
  fiscal year, which is what the series record's test asks of a source.
- A quarter's earnings over the same close gives about four times a year's
  multiple, and nothing would mark it.

**Price/sales is stated over a year.** The chapter's page 4 says "The price to
sales ratio for a stable firm can be extracted from a stable growth dividend
discount model", with "DPS1 = Expected dividends per share next year" and "r =
Required rate of return on equity". The record reads that as a year's
revenue: the form the ratio is extracted from sets a dividend "next year"
against a return per year.

**Price/free cash flow is stated over a year.** No source states it. This is
the record's own decision, and the reason is the price/sales reason: the same
equity value over a firm's flow, and a quarter's flow would give about four
times a year's multiple with nothing to mark it. The discounted cash flow
record states its own use of free cash flow over a year for the same reason.

For these three, at a duration that is not a year, each gives exactly one
`declined` reason, the first of `start_not_a_date`, `end_not_a_date`,
`not_a_year` and `overlaps_another_year`, and makes no read, as the series
record says. Their `stated_at` stays `Duration`.

**Price/book is stated at an instant.** It reads three balances and no flow.
Under the period record, "A metric stated at an instant reads only balances,
at its own row." The chapter's page 2 sets the price over "the current book
value of equity", a balance at a date. It is computed at every instant the
history holds, a quarter's balance sheet date included, and no year test is
made. A balance at an instant is that balance whatever duration it closes.

### Which instant the count and the common equity are read at

**Price/book** reads `shares_outstanding`, `shareholders_equity` and
`preferred_equity` at its own instant. No other row is read.

**Price/earnings** reads no balance.

**Price/sales and price/free cash flow read the count averaged.** Each pairs
a flow with a balance, `shares_outstanding`, whose measure v2 gives as
`balance`. Neither is a term of a named score. So under the period record's
rule as written, each reads the mean of the count at the instants Y opens and
closes at. That is two reads, opening then closing. Neither reads the common
equity.

**Whether the rule reaches a count the price multiplies.** The record reads
it as reaching it, and names a question about it:

- **As written, it reaches.** "Every other metric that pairs a flow with a
  balance reads the average. This holds whatever source its formula cites."
  The count is a balance by v2's measure, and the rule reads the measure.
- **The third sentence does not release it.** "A balance its source names
  without saying when is read at closing" could be read to put the count at
  closing, since no source here says when the count is taken. The discounted
  cash flow record, accepted, read the same rule for net debt in the bridge,
  a balance its source names without saying when, and averaged it, and its
  review called that the rule "as written". This record reads it the same
  way, and does not read the rule narrowly.
- **The rule's reason does not fit.** It is a ratio's: "The mean of the two
  ends is the balance the flow was earned on." The count is not a balance a
  flow was earned on. It is what the close multiplies, to give the market
  value of equity at the valuation date. The count nearest that date in the
  history is the closing one.
- **What it costs.** The averaged term is the market value at no date. Where
  the count rose over Y, the term is understated, and the two multiples read
  low, which flatters. Where it fell, the reverse. Price/book, at an instant,
  reads one count, so the same filer's market capitalisation differs between
  price/book at Y's closing instant and price/sales over Y.
- **What would change it.** Reading the count at closing needs the sentence
  "Every other metric that pairs a flow with a balance reads the average.
  This holds whatever source its formula cites." narrowed, or the third
  sentence read as covering this case. That is a record of its own. This
  record does not propose it.

### The share-basis rule

Every metric here reads a figure on a share basis or one derived from it.
P/E's denominator is in currency per share. The other three are built on the
count. So each calls the rule, through the one function `M5-54` builds, as
"the one function every price-dependent metric calls".

**E, for each metric.**

| Metric | E |
| :--- | :--- |
| `price_to_earnings_per_share` | Y's end, the duration's end |
| `price_to_common_book` | I, the instant's date |
| `price_to_sales` | Y's end, which is the date of the closing instant |
| `price_to_free_cash_flow` | Y's end, which is the date of the closing instant |

T is the close's trading date.

**The rule is asked once, with one E.** Price/sales and price/free cash flow
each pair a count at instants and a flow over a duration with one close. The
rule is asked once for each:

- The share-basis record fixes it: "For a metric stated at a duration ... E
  is the duration's end".
- The rule governs a figure "whose unit is `currency_per_share` or `shares`,
  or ... anything derived from one". `revenue`, `operating_cash_flow` and
  `capital_expenditure` are in currency. A split does not restate them. So the
  flow is no second figure for the rule to see, and asking it per figure
  would ask it once anyway.
- The closing count is at an instant dated Y's end, so the count's instant
  and the duration's end are one E.
- For price/book, the count and the common equity are at one instant, I, and
  the common equity is in currency. One E.

**What it does not see: the opening count.** The averaged count also reads
the count at the instant Y opens at. That count is on the share basis of the
filing that set it. A split dated after the opening instant and before the
earlier of E and T passes the rule, so the average can mix two bases.

- A 2-for-1 split inside Y, with the opening count on the old basis, reads 55
  and 110 for 110 shares today. The average is 82.5, or 25% low. Both
  multiples read low, which flatters.
- Asking the rule with E at the opening instant would see it. That needs the
  share-basis record's sentence on E for a metric stated at a duration
  changed, or the count read at closing, which needs the period record's
  sentence changed. Either is a record of its own. This record names the
  exposure, pins it in a test, and leaves it open, as the dividend discount
  record left the split inside its window.

**What the caller must ask.** Price/book at an instant I, and every other
metric here at a year ending I, needs a price answered at or before I for the
covered range to reach it. Asking every period end the history holds does
that, as the analyze record says. A caller that asked less gets
`not_covered`, never a number.

### Where each is stated

Each metric is stated at each period it is computed at, beside the close at
the valuation date, as the discounted cash flow record places its margin.

- The analyze record fixes that the valuation date's answer is read from the
  crossing's answer at the valuation date.
- The provider record lists all four under the valuation date, and the past
  period ends for relative yield only.
- So every year's P/E is that year's earnings against today's close, and
  every instant's price/book is that instant's book against today's
  capitalisation. Which period a ranking reads is M6's query over the stored
  table. Nothing here orders the periods or picks the latest.

### The `declined` conditions

Each is asked only when every read and the price read give a number. They are
asked in formula order, and the first that holds is the one reason. Each is a
sign or zero test and needs no literal.

| Condition | Holds when | Metric | Defined |
| :--- | :--- | :--- | :--- |
| `non_positive_close` | the close is zero or below | all four | once, beside the discounted cash flow margin; reached from there |
| `non_positive_shares_outstanding` | a count read is zero or below | price/book, price/sales, price/free cash flow | new, once, beside the market capitalisation term |
| `zero_earnings_per_share_diluted` | the figure over Y is zero | price/earnings | once, beside `dividend_payout_per_share`; reached from there |
| `negative_earnings_per_share_diluted` | the figure over Y is below zero | price/earnings | new, once, beside this metric |
| `zero_common_equity` | common equity at I is zero | price/book | once, beside the common equity term; reached from there |
| `negative_common_equity` | common equity at I is below zero | price/book | new, once, beside this metric |
| `zero_revenue` | revenue over Y is zero | price/sales | once, in `crates/analyze/src/metrics/mod.rs`; reached from there |
| `negative_revenue` | revenue over Y is below zero | price/sales | new, once, beside this metric |
| `zero_free_cash_flow` | free cash flow over Y is zero | price/free cash flow | once, beside the free cash flow term; reached from there |
| `negative_free_cash_flow` | free cash flow over Y is below zero | price/free cash flow | new, once, beside this metric |

The order for each metric:

- **Price/earnings:** `non_positive_close`, `zero_earnings_per_share_diluted`,
  `negative_earnings_per_share_diluted`.
- **Price/book:** `non_positive_close`, `non_positive_shares_outstanding`,
  `zero_common_equity`, `negative_common_equity`.
- **Price/sales:** `non_positive_close`, `non_positive_shares_outstanding`,
  `zero_revenue`, `negative_revenue`.
- **Price/free cash flow:** `non_positive_close`,
  `non_positive_shares_outstanding`, `zero_free_cash_flow`,
  `negative_free_cash_flow`.

**A zero denominator** is a zero divisor. Each reaches the zero condition
already defined beside its term or in `mod.rs`, so no zero condition is
defined twice. A reached constant's visibility is widened where it must be,
and no second one is defined.

**A negative denominator is a condition, and not a signed figure.** This is
the record's decision for all four, on one reason:

- **Price/earnings.** The chapter's page 43: "The price earnings ratio cannot
  be estimated for firms with negative earnings per share." The analyze
  record's example is "P/E on non-positive earnings". Both say a condition.
- **Price/book.** The chapter says each. Page 2: "the book value of equity can
  become negative if a firm has a sustained string of negative earnings
  reports, leading to a negative price-book value ratio." Page 5: "there are
  firms with negative book values of equity – the result of continuously
  losing money – where price to book ratios cannot be computed." The record
  takes page 5's. Page 2 states what the arithmetic gives. Page 5 states what
  the chapter does with it, and its sample leaves those firms out.
- **Price/sales and price/free cash flow.** No source states either at a
  negative denominator. Chapter 20's page 1 says of the others: "unlike
  earnings and book value ratios, which can become negative for many firms and
  thus not meaningful, revenue multiples are available even for the most
  troubled firms and for very young firms." So a negative multiple is "not
  meaningful" in the chapter's own words, and the record applies that to
  these two as its own decision.
- **The one reason.** A multiple is the price paid for a unit of what the
  denominator measures. Over a negative denominator it is a price for no
  unit, and its sign is the denominator's, not the price's. Stored as a
  figure, it sorts below every real multiple, so a screen for low multiples
  would rank the filer losing the most first. That is a wrong number that
  looks right. An absence that names the sign does not flatter.
- **The cost.** Every year of loss, of negative free cash flow, and every
  instant of negative common equity is absent. Free cash flow is negative in
  any year of heavy investment, so price/free cash flow is absent there, as
  the discounted cash flow value is.

**Why the negative conditions are each metric's own.** The discounted cash
flow value's `non_positive_free_cash_flow` is that metric's, beside it, and
holds at zero too, where this metric reaches `zero_free_cash_flow`. The free
cash flow module says "a negative free cash flow is a value", and
`return_on_common_equity` does not decline at a negative common equity. So
the sign belongs to the multiple, and each new condition stands beside the
multiple that asks it. A later metric that wants one reaches it there.

The entries also list the four year conditions where the metric is stated
over a year, then `share_basis_date_not_a_date`, as the share-basis record
places it. `input_not_a_decimal` reaches all four from the one definition
beside the reader.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first,** for the three stated over a year. At a duration
   that is not a year, its one reason is the outcome and nothing below is
   asked. Price/book makes no year test.
2. **The history's reads, in input order.** Every read is made, and none stops
   another. Each gives a number or one reason: `input_unknown` or
   `input_not_applicable` naming the concept and the row read, `no_period`
   with `opening_instant` or `closing_instant`, or `declined` with
   `input_not_a_decimal`.
   - Price/earnings: `earnings_per_share_diluted` at Y.
   - Price/book: `shares_outstanding`, `shareholders_equity` and
     `preferred_equity`, each at I.
   - Price/sales: `shares_outstanding` at Y's opening instant and then its
     closing instant, then `revenue` at Y.
   - Price/free cash flow: `shares_outstanding` opening and closing, then
     `operating_cash_flow` and `capital_expenditure` at Y.
3. **The price read, last,** made whatever the reads gave, through the one
   function `M5-54` builds, with the E above. It gives a number or exactly one
   reason, in the discounted cash flow record's order:
   - no crossing: `no_price` with `not_asked`;
   - an absence answered for the valuation date: `no_price` with
     `answered_absent`, carrying the date asked;
   - a price: the rule, giving `basis_unproven` with `splits_absent`,
     `not_covered` or `split_within` by the first clause that fails, or
     `declined` with `share_basis_date_not_a_date` where a date it compares is
     outside the grammar;
   - a close outside the decimal grammar: `declined` with
     `input_not_a_decimal`.

   The answer at the valuation date is read by its place in the crossing, so
   `date_not_asked` has no witness here, as the discounted cash flow record
   says. The read never falls back to another date, to an adjusted close or
   to a stand-in.
4. **The conditions,** in the order above, only where every read and the
   price read gave a number.

**A metric built on a term carries the term's reasons.** Price/book's reads of
the common equity are the common equity term's, in its order. Price/free cash
flow's are the free cash flow term's. The market capitalisation term's are
the count's, and its price read. No outcome says "the term was absent".

**With no key, no number.** Every metric here reads a price. The caller with
no key asks nothing, so the crossing is `not_asked`, and each metric is absent
with `no_price` and `not_asked`, after any reasons of its own reads. That is
GOALS.md's ninth M5 criterion: "A metric with no price data is absent with a
reason. It never fails the run and never shows a wrong number."

No seventh reason is needed, and no frozen byte changes.

### By kind

`shares_outstanding`, `earnings_per_share_diluted`, `revenue`,
`shareholders_equity`, `operating_cash_flow` and `capital_expenditure` apply to
every kind. `preferred_equity` applies to every kind, and its silence is
`Unknown`.

| Kind | Where every read is a `Value` |
| :--- | :--- |
| `operating`, `bank`, `insurer`, `reit`, `investment_company` | all four computed |

So no kind is absent by applicability. **A filer silent on
`preferred_equity`** has `price_to_common_book` absent with `input_unknown`,
naming it at I, while the other three are figures, as the common-shareholder
record says.

**Where the method is weakest.**

- **Price/free cash flow for a financial filer.** The discounted cash flow
  record states it of the same flow: "A bank's operating cash flow carries
  flows that are its business, not its reinvestment", and quotes the survey's
  page 15: "estimating capital expenditures and working capital for a bank, an
  investment bank or an insurance company is difficult to do." For a `bank`,
  an `insurer` or an `investment_company`, the figure is computed and its
  error has no known direction.
- **Price/sales for a financial filer.** v2 reads `revenue` per kind: for a
  `bank`, "net interest income together with non-interest income". A bank's
  price/sales is set over a net figure and an operating filer's over a gross
  one, so the two do not compare.
- **Price/book is strongest there.** The survey's page 61 names "price to book
  ratios common in financial service company valuations".
- The kind does not cross v2. Declining a kind is the v3 record the analyze
  record names, and none of these needs it.

### The names

They cross `analyze-store` v1 as characters, under the common-shareholder
record's naming rule.

- **`price_to_common_book`,** as that record fixed it. It forms common equity,
  so it carries `common`. It reads no per-common-share concept, so it does not
  carry `per_share`.
- **`price_to_earnings_per_share`.** It reads `earnings_per_share_diluted`, a
  per-common-share concept, and the rule says `per_share` "appears in the name
  of a form that reads per-common-share concepts". So it carries `per_share`,
  and its `meaning` says "per common share". It forms neither common term, so
  it does not carry `common`.
- **`price_to_sales`** and **`price_to_free_cash_flow`.** Each reads a count,
  which is not a per-share concept, and totals. Neither forms a common term.
  So each carries neither word. Each `meaning` says the numerator is the
  market value of the common shares, so no reader takes it for an all-class
  figure.
- **Market capitalisation** has no name crossing the contract, since it is no
  metric. Its module is named `market_capitalisation`.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition and which periods or
dates a metric reads:

- the market capitalisation term, its form and its module
- the four forms, price/free cash flow's included
- which periods each is stated at, and the count averaged where it is read so
- E for each metric, and the rule asked once
- the four names and their meanings
- the five new conditions, and the order of all ten

A later change to any of them, a form, an instant read or a condition
included, is a version bump and a superseding record, and never an edit to
this one.

It lands at version 1. None of the metrics is computed yet, so no stored
outcome changes.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, and no new value of `unit`:

- **`unit`.** `Multiple`, for all four: how many times the denominator the
  price or the market value is. Market capitalisation has no entry.
- **`stated_at`.** `Duration` for the three stated over a year, `Instant` for
  `price_to_common_book`.
- **`inputs`,** in formula order, the price being no concept and not listed:
  - `price_to_earnings_per_share`: `earnings_per_share_diluted`
  - `price_to_common_book`: `shares_outstanding`, `shareholders_equity`,
    `preferred_equity`
  - `price_to_sales`: `shares_outstanding`, `revenue`
  - `price_to_free_cash_flow`: `shares_outstanding`, `operating_cash_flow`,
    `capital_expenditure`
- **`conditions`,** in the order asked: each metric's own from the order
  above, then `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year` by their constants for the three stated over a
  year, then `share_basis_date_not_a_date`. `price_to_common_book` lists its
  four, then `share_basis_date_not_a_date`. `input_not_a_decimal` is
  generated from its one definition.
- **`meaning`** says the form in words and the date of the close:
  - `price_to_earnings_per_share`: "The unadjusted close at the valuation
    date over diluted earnings per common share for a year, as the filer
    reports it."
  - `price_to_common_book`: "Market capitalisation, the unadjusted close at
    the valuation date times common shares outstanding at the instant, over
    common equity at that instant, shareholders' equity less preferred
    equity."
  - `price_to_sales`: "Market capitalisation, the unadjusted close at the
    valuation date times the average of common shares outstanding at the
    instants the year opens and closes at, over revenue for the year."
  - `price_to_free_cash_flow`: the same market capitalisation "over free cash
    flow for the year, operating cash flow less capital expenditure."

### Sources

Under `sources-a-task-can-read.md`. Each copy below was opened in this run and
each quotation is as printed. Each copy's printed page numbers match its PDF
positions. The equations were read from the rendered pages, since the text
extraction scrambles their words.

**Damodaran, A., *Investment Valuation*, 2nd ed., John Wiley and Sons, 2002,**
the author's own copies of three chapters.

- **Chapter 18, "Earnings Multiples",**
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch18.pdf>, 59 pages,
  198,889 bytes, sha256
  `a864e654892586e6d0ab936c5c0777cfd7fe1a23c4b8e9bd81dffc89e1652e28`, the
  digest the task names.
  - Page 1: "The price earnings ratio is the ratio of the market price per
    share to the earnings per share." The equation is "PE =" "Market Price
    per share" over "Earnings per share". "The PE ratio is consistently
    defined, with the numerator being the value of equity per share and the
    denominator measuring earnings per share". "PE ratios could be computed
    using current earnings per share, trailing earnings per share, forward
    earnings per share, fully diluted earnings per share and primary earnings
    per share."
  - Page 27: "If the expected growth rate in earnings per share is based upon
    earnings in the most recent year (current earnings), the PE ratio that
    should be used is the current PE ratio."
  - Page 43: "The price earnings ratio cannot be estimated for firms with
    negative earnings per share."
  - Page 46, for the open item: "The enterprise value to EBITDA multiple
    relates the total market value of the firm, net of cash, to the earnings
    before interest, taxes and depreciation of the firm."
- **Chapter 19, "Book Value Multiples",**
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch19.pdf>, 42 pages,
  106,558 bytes, sha256
  `575088a41dcdf8d98b34101b96a0658545c31c2693783401005e811507db36d3`, the
  digest the task names.
  - Page 1: "The book value of equity is the difference between the book
    value of assets and the book value of liabilities, a number that is
    largely determined by accounting conventions."
  - Page 2: "the book value of equity can become negative if a firm has a
    sustained string of negative earnings reports, leading to a negative
    price-book value ratio." "The price to book ratio is computed by dividing
    the market price per share by the current book value of equity per
    share." "If there are multiple classes of shares outstanding, the price
    per share can be different for different classes of shares". "You should
    not include the portion of the equity that is attributable to preferred
    stock in computing the book value of equity, since the price per share
    refers only to common equity." "Some of the problems can be alleviated by
    computing the price to book ratio using the total market value of equity
    and book value of equity, rather than per share values." The equation is
    "Price to Book Ratio = PBV =" "Market Value of Equity" over "Book value of
    equity".
  - Page 5: "there are firms with negative book values of equity – the result
    of continuously losing money – where price to book ratios cannot be
    computed."
- **Chapter 20, "Revenue Multiples and Sector-Specific Multiples",**
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch20.pdf>, 42 pages,
  103,261 bytes, sha256
  `995b0d71d0a5b69322ae5ebcf3384fe6d8cb41accdb9abfd50be32546e61d246`, the
  digest the task names.
  - Page 1: "unlike earnings and book value ratios, which can become negative
    for many firms and thus not meaningful, revenue multiples are available
    even for the most troubled firms and for very young firms."
  - Page 2: "The first, and more popular one, is the multiple of the market
    value of equity to the revenues of a firm – this is termed the price to
    sales ratio." The equation is "Price to Sales Ratio =" "Market Value of
    Equity" over "Revenues". "The price to sales ratio divides an equity value
    by revenues that are generated for the firm. Consequently, it will yield
    lower values for more highly levered firms".
  - Page 4: "The price to sales ratio for a stable firm can be extracted from
    a stable growth dividend discount model.", with "DPS1 = Expected dividends
    per share next year" and "r = Required rate of return on equity".

**Damodaran, A., *Valuation Approaches and Metrics: A Survey of the Theory and
Evidence*, Stern School of Business, New York University, November 2006,**
<https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
pages, 1,602,127 bytes, sha256
`9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`, the
digest the task names and the dividend discount record computed.

- Page 15: "estimating capital expenditures and working capital for a bank,
  an investment bank or an insurance company is difficult to do."
- Page 59: "the price per share of a stock is a function both of the value of
  the equity in a company and the number of shares outstanding in the firm.
  Thus, a stock split that doubles the number of units will approximately
  halve the stock price." And: "This price/earnings ratio can be estimated
  using current earnings per share, yielding a current PE, earnings over the
  last 4 quarters, resulting in a trailing PE, or an expected earnings per
  share in the next year, providing a forward PE."
- Page 60: "the price/sales ratio (PS), where the market value of equity is
  divided by the revenues generated by the firm".
- Page 61: "price to book ratios common in financial service company
  valuations".

**Free cash flow's form** is cited from the one module, as the discounted
cash flow record cites it: the Commission's C&DI 102.07. This record opens
nothing more for it.

**Where no source fixes a form or a condition,** it is the record's own
decision, with its reason above: the product that forms market capitalisation
in symbols, price/free cash flow's form and its year, the averaged count
where the period rule reaches it, and the negative conditions of price/sales
and price/free cash flow. No copy opened states a form otherwise than the
task quotes it.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at`, and a `Crossing`
built as the contract type. They read no file. Each stands in `tests/` or
under `#[cfg(test)]`.

**The worked history.** The year 2024-01-01 to 2024-12-31, and the instants
2023-12-31 and 2024-12-31. Balances are given opening/closing.

- `earnings_per_share_diluted` 4 over the year.
- `shares_outstanding` 90/110.
- `shareholders_equity` 2000/2400 and `preferred_equity` 200/200.
- `revenue` 12000, `operating_cash_flow` 800 and `capital_expenditure` 300
  over the year.

**The preset crossing** is the share-basis record's: a valuation date of
2025-06-30 answered with a close of 60 traded 2025-06-30, the earlier date
2024-12-31 answered with a close traded 2024-12-31, and an empty split list.
E is 2024-12-31 for every figure below unless a test says otherwise.

At the least:

- **The figures, worked by hand.**
  - `price_to_earnings_per_share` over 2024: 60 ÷ 4 gives `15`.
  - `price_to_common_book` at 2024-12-31: market capitalisation 60 × 110 is
    6600, common equity 2400 less 200 is 2200, so `3`.
  - `price_to_sales` over 2024: the average count is 100, market
    capitalisation 6000, so 6000 ÷ 12000 gives `0.5`.
  - `price_to_free_cash_flow` over 2024: free cash flow 800 less 300 is 500,
    so 6000 ÷ 500 gives `12`.
  - **The average, not the close.** Price/sales is never `0.55` and
    price/free cash flow never `13.2`, which the closing count alone would
    give.
  - **The earlier instant.** `price_to_common_book` at 2023-12-31, with E
    2023-12-31, gives `basis_unproven` with `not_covered` naming 2024-12-31
    through 2025-06-30, 2023-12-31 and 2025-06-30. The caller asked no date at
    or before it.
  - **A zero preferred.** `preferred_equity` 0 at 2024-12-31 gives `2.75`.
- **Each `declined` condition.**
  - A close of 0, and of -1: `non_positive_close`, for all four.
  - `shares_outstanding` 90/0: `non_positive_shares_outstanding` for
    price/book at 2024-12-31 and for price/sales and price/free cash flow,
    never a figure over the average of 45, with P/E `15`. 0/110: the same for
    the two averaged, while price/book at 2024-12-31 is `3`.
  - Earnings per share 0: `zero_earnings_per_share_diluted`. -4:
    `negative_earnings_per_share_diluted`.
  - Shareholders' equity 200 at 2024-12-31: `zero_common_equity`. 100:
    `negative_common_equity`.
  - Revenue 0: `zero_revenue`. -12000: `negative_revenue`.
  - Capital expenditure 800: `zero_free_cash_flow`. 900:
    `negative_free_cash_flow`.
  - **The first that holds.** A close of 0 beside earnings per share of 0
    gives `non_positive_close` alone.
- **No price, by each witness,** for all four:
  - No crossing: `no_price` with `not_asked`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying 2025-06-30.
  - `date_not_asked` has no witness: the test shows `Crossing` cannot be
    built without `at_valuation_date`.
- **The share-basis rule, by each clause,** for all four:
  - The split list absent: `splits_absent`.
  - The valuation date alone, no earlier date answered: `not_covered` naming
    2025-06-30 through 2025-06-30, 2024-12-31 and 2025-06-30.
  - A split dated 2025-03-14: `split_within` naming it, 2024-12-31 and
    2025-06-30. A split dated 2024-12-31: `split_within`.
- **A date outside the grammar.** The paired close's trading date
  `2025-6-30`: `declined` with `share_basis_date_not_a_date`, for all four.
  `price_to_common_book` at an instant dated `2024-12-1` holding the closing
  balances: `share_basis_date_not_a_date`, its E being outside the grammar.
- **An unreadable close.** `1e2`: `declined` with `input_not_a_decimal`.
- **The exposure, pinned.** `shares_outstanding` 55/110 and a split dated
  2024-06-15, beside the preset crossing's answers: the rule passes,
  price/sales is `0.4125` and price/free cash flow `9.9`. The test names this
  as the split before E that the rule does not see, so that a superseding
  record changes it knowingly.
- **A filer silent on `preferred_equity`.** `Unknown` at 2024-12-31:
  `price_to_common_book` there is `input_unknown` naming it at 2024-12-31,
  and the other three are `15`, `0.5` and `12`.
- **A filer of each kind.** For each of the five kinds, the worked history
  with every concept v2 does not apply to that kind `NotApplicable` gives
  `15`, `3`, `0.5` and `12`.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31 with every input
  `Unknown`: `not_a_year` alone, for the three stated over a year.
- **The order of reasons.** The instant 2023-12-31 missing,
  `capital_expenditure` `Unknown`, and no crossing. Price/free cash flow gives
  `no_period` with `opening_instant`, then `input_unknown` for capital
  expenditure, then `no_price` with `not_asked`. Price/sales gives the first
  and the last. With `preferred_equity` also `Unknown` at 2024-12-31,
  price/book there gives `input_unknown` for it, then `not_asked`.
- **The catalogue.** Each entry's `inputs` and `conditions` are as listed
  above. No entry lists `diluted_shares_weighted_average` or `net_income`. No
  entry is named for market capitalisation. Only `price_to_common_book`'s
  name holds `common`, and only `price_to_earnings_per_share`'s holds
  `per_share`.

### What no check can see

- **Whether the sources say what this record says.** The three chapters and
  the survey were opened in this run and are quoted as printed. The
  implementing tasks cite each as the sources record says. A copy that on
  reading says otherwise is a stop for that task, not a choice.
- **The class of a share.** The rule covers the dates of a count and a close,
  and not their class. The chapter's page 2: "If there are multiple classes
  of shares outstanding, the price per share can be different for different
  classes of shares". `shares_outstanding` may count every common class while
  the close is one class's. The figure is then off by the classes' price
  difference, and nothing in v2 or the crossing names the class.
- **The averaged count,** where the market value at the valuation date is
  wanted, and the split before E inside it, as stated above.
- **The count's date.** `shares_outstanding` is at "a stated date at or near
  the period end", and may be a cover page's. Shares issued or bought back
  between it and T are not seen. A split is seen, by the rule.
- **A figure at Y beside a close at T.** The figure is not rolled forward to
  the valuation date. Where Y ends after the valuation date, every metric is
  `not_covered`, as the share-basis record says. Where it ends long before,
  an old year's P/E is stated beside today's close, and nothing marks it.
- **Leverage.** Price/sales and price/free cash flow set an equity value over
  a flow to the whole firm, and read lower for a more levered filer, as the
  chapter's page 2 says. Enterprise value would remove it.
- **A financial filer's price/free cash flow and price/sales,** as stated
  under "By kind".
- **A quarter's price/book beside a year's.** It is computed at every
  instant, and nothing marks a quarter-end's book.
- **The currency.** The close's currency and the filer's reporting currency
  are not compared, which `price-provider.md` already leaves open.
- **A metric that forms market capitalisation itself.** Only review sees it.

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
  share-basis rule, three places and catalogue
- the share-basis record's comparisons and condition, and its E for a metric
  stated at a duration
- the arithmetic record's reading, operations and written form
- the series record's year test, seek and series
- the period record's table and rule, read as written
- the common-shareholder record's terms, names and conditions
- the discounted cash flow record's two metrics, module and margin, its
  `non_positive_close` reached and not moved
- the capital record's terms and module
- the provider record's ask and answer
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains no value of `unit` and no field

Two questions here would need an accepted record to change, and each is left
open and named:

- **Whether the averaging sentence reaches a count the price multiplies.**
  The period record's "Every other metric that pairs a flow with a balance
  reads the average. This holds whatever source its formula cites." reaches
  it as written. Reading the count at closing is a superseding record of its
  own.
- **Whether the rule sees a count read before E.** The share-basis record's
  "For a metric stated at a duration ... E is the duration's end" leaves the
  opening count unseen. Asking the rule at the opening instant is a
  superseding record of its own, or falls away if the count is read at
  closing.

**Why Structural.** The record fixes the names and meanings of four metrics
that cross `analyze-store` v1, which store and M6 will read, and a term that
three metrics here and two later ones share. It needs nothing
Constitutional: no anchor, no gate, no protected path and no GOALS.md scope
moves.

## Alternatives

- **Market capitalisation over `diluted_shares_weighted_average`.** It counts
  the options and convertibles that would share in the equity, and it is a
  flow, so the period record would not reach it. Rejected: the catalogue
  names `shares_outstanding` for market capitalisation and through it the
  multiples, and the diluted count for the discounted cash flow's per-share
  step. A weighted average over Y is the count of no date. The "Market Value
  of Equity" of chapter 20's page 2 values the shares outstanding, not the
  shares that might be.
- **Price/earnings as market capitalisation over `net_income`.** Rejected:
  the chapter's form is a price per share over earnings per share. Net income
  includes the preferred claim's share and is set over common shares at an
  instant, so it mixes two claims and two dates. The catalogue names
  `earnings_per_share_diluted` for price/earnings, and the common-shareholder
  record lists price/earnings as reading a per-common-share concept with no
  term to form.
- **Price/book over total equity.** Rejected by name in the
  common-shareholder record: "Over total equity it is the incoherent figure
  the open-questions record refused." It understates for a filer with
  preferred outstanding.
- **Each multiple at each period against that period's close,** as relative
  yield reads past closes. It would make a past year's P/E the P/E the market
  paid then. Rejected: the provider record lists the past period ends "for
  relative yield only" and the multiples under the valuation date. A past
  multiple answers a question no one buying today asks.
- **A negative multiple as a signed figure.** The arithmetic gives it, and
  the chapter's page 2 names "a negative price-book value ratio". Rejected:
  it sorts below every real multiple and ranks the worst filer first, the
  chapter's page 43 and page 5 say the ratio "cannot be estimated" or "cannot
  be computed", and the analyze record's example is "P/E on non-positive
  earnings".
- **A zero denominator as a signed figure's limit, or a negative one as the
  same condition as zero.** Rejected: a zero condition is already defined
  beside each term, and a second condition holding at zero would define one
  case twice. Each negative condition holds below zero alone.
- **A negative revenue or free cash flow as a figure,** while P/E and
  price/book decline. Rejected: one rule for four multiples. Chapter 20's
  page 1 calls negative ratios "not meaningful".
- **Enterprise value and EV/EBIT and EV/EBITDA in this record.** Rejected:
  whether it counts the preferred claim is "its own record" by the
  common-shareholder record's words, it would read `preferred_equity` as a
  consumer the open-questions record did not admit, and which instant net
  debt is read at is open in the discounted cash flow record. That record
  settles them on this one's market capitalisation, once accepted.
- **Market capitalisation as a metric.** Rejected here: it needs a `unit` of
  currency that `Unit` does not carry, and GOALS.md names it as no multiple.
  The value is named and left open. A later record can add it, beside the
  term.
- **The count at closing for price/sales and price/free cash flow,** reading
  the period record's third sentence as covering it. This is the reading the
  term wants. Not rejected on its merits: it reads an accepted rule narrowly,
  against how the discounted cash flow record read it, so it is a record of
  its own.
- **The rule asked once per count read, with E at the opening instant too.**
  It would see the split inside Y. Not rejected on its merits: it sets aside
  the share-basis record's E for a metric stated at a duration, so it is a
  record of its own. It would also make every price/sales and price/free cash
  flow need a price at or before Y's opening instant.
- **Price/sales and price/free cash flow per share,** each flow over
  `diluted_shares_weighted_average`, with the close over it. Every input would
  be a flow, so the period record would not reach any. Rejected: it sets the
  close over a count the catalogue does not name for these, and makes the
  multiple a price over a per-diluted-share figure that the filer does not
  report.
- **P/E at every duration.** Rejected: a quarter's P/E is about four times a
  year's, and the sources name a year's earnings in every reading.
- **A trailing P/E over the last four quarters.** Rejected here: it adds four
  quarters, which no record forms, and the history may hold year-to-date
  spans and transition stubs beside them. A record that forms trailing spans
  can add it as a second metric.
- **Price/book stated over a year,** at the year's closing instant alone.
  Rejected: it reads no flow, and the period record states such a metric at
  an instant. A quarter-end balance is a balance.
- **Leaving the multiples out of M5 until the provider is built.** Rejected:
  a test builds the crossing as the contract type, so each figure and each
  `no_price` is provable today, and the share-basis rule is on main or in
  flight. Deferring would also change milestone scope, which is
  Constitutional.
- **Each metric forming market capitalisation itself.** Rejected: three
  readings of one product, and two more to come, which the one-source-of-truth
  invariant forbids.

## Consequences

**Easier:**

- Four of GOALS.md's valuation multiples have their forms, periods, instants,
  conditions, names and sources. Each implementing task has only its code
  left to decide.
- Market capitalisation is formed once before its first reader, so
  enterprise value and Altman's X₄ read it there.
- A user with no key sees each multiple absent with `no_price`, and every
  other metric computed.

**Harder:**

- **Every year of loss, negative free cash flow or negative revenue, and every
  instant of negative common equity, is absent.** So is every quarter, for the
  three stated over a year.
- **Price/sales and price/free cash flow need both of Y's instants,** and the
  averaged count reads low where shares were issued, which flatters, and mixes
  bases across a split inside Y.
- **Price/book is absent for most filers,** which are silent on
  `preferred_equity`, unless a registry override asserts the zero.
- **The multiples wait on two tasks in flight:** `M5-54`'s price read, and
  `M5-55`'s `non_positive_close`.

**Expensive to reverse:**

- The four names and their forms, once results are stored and ranked on.
- The market capitalisation module, once five metrics read it.
- A negative denominator as a condition. Turning it into a figure later makes
  stored absences into numbers, under a bump.

**Open, and not this record's:**

- Enterprise value, EV/EBIT and EV/EBITDA: whether enterprise value counts the
  preferred claim or non-controlling interests, and which instant net debt is
  read at. The chapter's page 46 names the second multiple.
- Whether the period record's averaging sentence reaches a count the price
  multiplies.
- Whether the share-basis rule sees a count read before E.
- `Currency` as a value of `Unit`, if market capitalisation becomes a metric.
- The class of a share, which neither v2 nor the crossing names.
- A trailing P/E, which needs trailing spans formed.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **No literal.** `literals` keeps every number but zero and one out of the
  derivation. The multiples need none.
- **One definition of market capitalisation.** The proof's figures hold the
  three readers to the module.
  - **Gap:** nothing forces a metric through the module rather than around
    it, as the discounted cash flow record says of free cash flow.
- **One definition of each condition.** Each new condition is a `const`
  beside its term or metric, and each reached one is the existing `const`.
  - **Gap:** nothing stops a second `const` with the same characters, except
    review.
- **No wrong number across a split.** The share-basis tests hold it for every
  multiple at E.
  - **Gap:** the opening count of an averaged read, pinned by a test as the
    exposure it is.
- **No multiple without a price.** The witness tests hold it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the names and
  meanings of four metrics that cross `analyze-store` v1 as characters, adds
  five `declined` conditions under the accepted reason, and places one term
  in a module of its own, all inside analyze. It edits no anchor, no
  protected path, no gate, no contract byte, no schema and no milestone
  scope, and asks for no allowlist entry. Flagged for later human review, as
  the tier requires.
- **Checked:**
  - GOALS.md's fourth M5 criterion names "valuation multiples", and no
    accepted record forms one. `canonical-concepts.md` names
    `shares_outstanding` for market capitalisation and the diluted figure for
    price/earnings, which is what the record reads for each.
  - `common-shareholder-forms.md` (Accepted): `price_to_common_book` and its
    common form are taken as fixed, its naming rule gives
    `price_to_earnings_per_share` its `per_share` and the two totals neither
    word, and `zero_common_equity` is reached from where that record defines
    it. The negative-equity question that record left to "its own task" is
    decided here with the chapter's page 5 against its page 2.
  - `what-analyze-takes-and-returns.md` (Accepted): the share-basis rule
    reaches every metric through `shares_outstanding` or a per-share
    denominator, as the record says, and its example "P/E on non-positive
    earnings" is the record's condition. `share-basis-rule-dates.md`
    (Accepted) fixes E as the duration's end, and the record follows it
    without proposing a change.
  - `which-other-period-a-metric-reads.md` (Accepted): the record reads the
    averaging sentence as reaching the count, the reading the discounted cash
    flow record's review called "as written" for net debt. It names the
    closing-count reading as the one the term wants and leaves it to a
    superseding record. That is the right shape: no accepted sentence is
    narrowed here.
  - `discounted-cash-flow-and-margin-of-safety.md` (Accepted): free cash flow
    is read from its one module, `non_positive_close` is reached from beside
    that margin, and the price read's order of reasons is the same.
  - The five reached constants exist on main under the characters the record
    names: `ZERO_EARNINGS_PER_SHARE_DILUTED` beside
    `dividend_payout_per_share`, `ZERO_COMMON_EQUITY`, `ZERO_REVENUE` in
    `mod.rs`, `ZERO_FREE_CASH_FLOW`, and `non_positive_close` where M5-55
    will define it. `Unit` carries no currency, as the record says.
  - The proof's figures re-derive by hand: `15`, `3`, `0.5`, `12`, `2.75`,
    `0.4125` and `9.9`, the `0.55` and `13.2` the closing count alone would
    give, and `not_covered` at 2023-12-31 under the preset crossing.
  - Every quotation and digest the task names is present; the four sha256
    digests match the task's.
  - The alternatives as argued: the diluted count, net income, total equity,
    a past close per period, a signed negative multiple, enterprise value
    here, and deferral past the provider are each rejected on a quoted
    source or an accepted record. The two "not rejected on its merits" items
    are the open questions, correctly left to records of their own.
- **Verdict and why:** Accepted. It decides what the four multiples are from
  the sources it opened, reuses every term and condition already defined
  rather than forming a second, and is honest about the two places where an
  accepted sentence gives a worse answer than the term wants, pinning the
  cost in a test instead of editing the sentence. A negative denominator as
  a condition for all four is a choice a human may want to revisit, since it
  absents every loss year; the record's reason, that a signed multiple sorts
  the worst filer first, is the one that holds for a screening tool.
- **What would have changed it:** a metric here forming market capitalisation
  itself, or a second `const` for a reached condition; a form a quoted page
  states otherwise; the averaged count presented as the count the term wants
  rather than as the rule's cost; or any edit to the period or share-basis
  sentence instead of an open question.
- **Flagged for the human:** the two open questions the record names, the
  averaging sentence's reach over a count the price multiplies and the
  opening count the rule does not see; a negative denominator as a condition
  for all four; and price/sales read over a year on chapter 20's page 4.
