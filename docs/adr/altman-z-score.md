# Altman's Z-score is the function his restatement prints as final, `0.012`, `0.014`, `0.033`, `0.006` and `0.999` over four ratios as percentages and a fifth as times, every balance at one year's close and the market value of equity through market capitalisation; four of the ratios are metrics beside it, and no cut-off enters analyze

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-11-26, by `M5-72`
- **Decided:**
- **Touches:** the bankruptcy score that GOALS.md's fourth M5 criterion names
  in "financial health and bankruptcy scoring", Altman's Z-score, and that
  score alone. It fixes the names and meanings of five metrics that cross
  `analyze-store` v1 as characters, and no byte of that contract changes. It
  puts six constants in a new file under `crates/analyze/src/constants/`,
  one of the three places `bare-literal-lint.md` allows. It forms working
  capital once, in a module of its own, and the five ratios once, in one
  module. It reads market capitalisation from the one place its accepted
  record forms it and writes the product no second time. It applies the
  accepted `declined` reason to six conditions, three reached from their one
  definitions and three new, and adds no reason. It adds one value of the
  catalogue entry's `unit`, a discriminant score, and no field. It adds no
  setting. It does not count the preferred claim, so `preferred_equity`
  gains no consumer. It names, and leaves open, one question it cannot
  decide without an accepted sentence changing: whether the fifth ratio is a
  metric, against the open-questions record's "Altman's X₅ is unreached for
  both kinds." No accepted record is superseded. No contract's bytes, no
  schema, no edge, no gate and no dependency move. Nothing here is
  implemented.

## Context

GOALS.md's fourth M5 criterion names "financial health and bankruptcy
scoring" among the value metrics. Piotroski's signals and score are
computed. The bankruptcy score is Altman's by every record that names one,
and none of them forms it: which printed specification it is, its
coefficients as constants, what the preferred claim and the cut-offs become,
its conditions, its names and its source have no record.

Some things are already fixed, and this record restates them as fixed:

- **Constants and criteria.** `what-analyze-takes-and-returns.md` (Accepted):
  "A methodology constant is a value the metric's cited source fixes, such as
  Piotroski's thresholds, Altman's coefficients and cut-offs, or Graham's
  limits. It is not the user's to change." Each is "a named `const` in
  `crates/analyze/src/constants/`, one file per method". And: "A threshold
  the user applies to stored metrics when screening or ranking is not a
  setting. It is a ranking criterion, which is M6's, because it changes no
  number analyze computes."
- **A term that contributes nothing.** The same record: "An input that is
  `NotApplicable` makes the metric absent too. The one exception is where the
  metric's own definition states that the term contributes nothing."
- **The instant each ratio reads.** `which-other-period-a-metric-reads.md`
  (Accepted): "A term of a named score reads what its author fixed." Its
  table reads each of the five ratios' balances at "closing", and names the
  fourth row "Altman's X4, the book value of total debt beside the market
  value of equity", with "The price's date is the score's task, under the
  share-basis rule." Of its source: "**Altman** reads every ratio off the
  financial statements of one report, so total assets is that report's
  balance sheet: the balance at the duration's close." Of the table itself:
  "Each metric's formula, and which concepts make each term, stay with the
  metric's task. This table fixes only the instant a balance is read at".
- **The year.** `which-duration-precedes-a-duration.md` (Accepted) lists
  "Altman's score and Piotroski's score, which their authors state over a
  year", says "A metric is **stated over a year** when its source states it
  over a fiscal year.", and covers "every term of Altman's score and the
  score itself".
- **The concepts.** `canonical-concepts.md` (Accepted) names the consumers:
  of `revenue`, "Altman's X₅"; of `operating_income`, "Altman's X₃"; of
  `total_assets`, "Altman's X₁, X₂, X₃ and X₅"; of `current_assets` and of
  `current_liabilities`, "working capital for Altman's X₁"; of
  `total_liabilities`, "liabilities to assets; Altman's X₄"; of
  `retained_earnings`, "Altman's X₂" and nothing else; and of
  `shares_outstanding`, "market capitalisation, and through it price/book,
  price/sales, enterprise value and Altman's X₄". It lists "working capital"
  among the derived values, of which "Each is a combination of concepts above
  and belongs to analysis." It says "a `bank` has no working capital and no
  EBIT, so Altman's X₁ and X₃ have no inputs and the Z-score is not
  computable for it at all."
- **Applicability,** read from `contracts/canonical-concepts/v2.toml`:
  `current_assets` and `current_liabilities` apply to `operating` alone,
  `operating_income` to `operating` and `reit`, and `revenue`,
  `total_assets`, `total_liabilities`, `retained_earnings`,
  `shares_outstanding` and `preferred_equity` to all five kinds. Every one
  reads silence as `unknown`. `operating_income` and `revenue` have measure
  `flow`, and the rest `balance`.
- **The open-questions record.** `canonical-concepts-open-questions.md`
  (Accepted) says of the operating lease obligation "It sits inside
  `total_liabilities`, so liabilities to assets and Altman's X₄ see it.", and
  of `revenue`'s reading for a `bank` and an `insurer`, "Altman's X₅ is
  unreached for both kinds."
- **The price.** `price-provider.md` (Accepted) lists, at the valuation
  date, "market capitalisation, through `shares_outstanding`, and through it
  price/book, price/sales, price/free cash flow, enterprise value (so EV/EBIT
  and EV/EBITDA) and Altman's X₄, the market value of equity".
- **Market capitalisation.**
  `market-capitalisation-and-the-equity-multiples.md` (Accepted): "Every
  metric that reads market capitalisation reads it there: price/book,
  price/sales and price/free cash flow here, and enterprise value and
  Altman's X₄ when they are built. No metric writes the product itself." Its
  two conditions are `non_positive_close` and
  `non_positive_shares_outstanding`.
- **The total form.** `common-shareholder-forms.md` (Accepted) lists under
  "Total form alone" "Altman's terms and Piotroski's signals, whose authors
  fix their own inputs", and says "If enterprise value wants
  `preferred_equity`, that is a new consumer the open-questions record did
  not admit, and its own record."
- **EBIT.** `enterprise-value-and-its-multiples.md` (Accepted): "**EBIT is
  `operating_income`,** read at the year's own row.", quoting the capital
  record's "v2's `operating_income` is EBIT by its meaning."
- **E.** `share-basis-rule-dates.md` (Accepted): "For a metric stated at a
  duration, as both discounted cash flow metrics are, E is the duration's
  end".
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written,
  and a table of operations. It says "a ratio is a fraction of one, `0.25`
  and never `25`", and that a factor which "belongs to a method" "stands in
  `constants/` under that method's citation".
- **Absence is closed.** Six reasons. A metric built on a term carries the
  term's reasons.
- **The entry shape.** M5-18 fixed it: name, meaning, `unit`, `stated_at`,
  inputs in reason order, and the metric's own `declined` conditions. `Unit`
  carries `FractionOfOne`, `Multiple`, `Years`, `CurrencyPerShare` and
  `Signals`.
- **What is on main.** The market capitalisation module and the price read in
  `crates/analyze/src/metrics/reading/price.rs` are built. `zero_total_assets`
  is defined once, in `crates/analyze/src/metrics/liabilities_to_assets.rs`.
  No metric reads `retained_earnings`, and none forms working capital.

Four things are settled by none of these, and each is settled here:

- The copy of the author's restatement prints two specifications that are
  not one function, and two cut-offs that are not one number.
- The copy's "total liabilities" stands beside the period record's "total
  debt", and its "all shares of stock, preferred and common" beside a term
  that counts common shares.
- Whether a cut-off is a constant analyze reads or a criterion of M6's is a
  reading of two sentences of the analyze record.
- Whether each ratio is stored beside the score, and what a filer whose
  shape lacks a ratio's concept gets.

## Decision

### Five metrics, one score among them

| Name | The copy's name | Unit | Reads a price |
| :--- | :--- | :--- | :--- |
| `altman_working_capital_to_total_assets` | X1, WC/TA | `FractionOfOne` | no |
| `altman_retained_earnings_to_total_assets` | X2, RE/TA | `FractionOfOne` | no |
| `altman_ebit_to_total_assets` | X3, EBIT/TA | `FractionOfOne` | no |
| `altman_market_value_of_equity_to_total_liabilities` | X4, MVE/TL | `FractionOfOne` | yes |
| `altman_z_score` | Z | `DiscriminantScore`, new | yes |

The fifth ratio, sales to total assets, is a term of the score, formed once
and handed over nowhere. Working capital is a term too.

None reads a setting. Each is stated over a year.

### Which model, and which specification

**The model is the function of the copy's page 9,** the one that reads a
price: "The final discriminant function is as follows:" and "Z = 0.012X1 +
0.014X2 + 0.033X3 + 0.006X4 +0.999X5". The copy prints two re-estimations,
Z’ on page 25 for privately held firms and Z" on page 27 for
non-manufacturers, each with coefficients of its own over a fourth ratio at
book value. Neither is this record's.

**The specification is page 9's, over percentages.** The copy prints the
function twice, and the two are not one function:

- **Page 9** prints `0.012`, `0.014`, `0.033`, `0.006` and `0.999`. Page 12
  says of it "Due to the original computer format arrangement, variables X1
  through X4 must be calculated as absolute percentage values." The sentence
  that opens there, "For instance, the firm whose net working capital to
  total assets (X1) is 10%", runs on at the head of page 13: "should be
  included as 10.0% and not 0.10." And: "Only variable X5 (sales to total
  assets) should be expressed in a different manner: that is, a S/TA ratio of
  200% should be included as 2.0."
- **Page 13** prints "a more convenient specification": "Z = 1.2X1 + 1.4X2 +
  3.3X3 + 0.6X4 + 1.0X5.", where "one inserts the more commonly written
  percentage, for example, 0.10 for 10%, for the first four variables (X1-X4)
  and rounds the last coefficient off to equal 1.0 (from 0.99)."
- **Where they differ.** A hundred times `0.012`, `0.014`, `0.033` and
  `0.006` is exactly `1.2`, `1.4`, `3.3` and `0.6`, so the first four terms
  agree. The fifth does not: page 9 prints `0.999` and page 13 `1.0`. Page 13
  says the last coefficient is rounded "(from 0.99)", where page 9 prints
  `0.999`. Page 13 says "The scores for individual firms and related group
  classification and cutoff scores remain identical." They do not, wherever
  sales are not zero: page 13's score is higher by one thousandth of the
  fifth ratio.

**Why page 9's.**

- It is the one the copy calls "The final discriminant function", and prints
  without qualification. Page 13's is introduced as what "many individuals
  have found" convenient, and its own sentence says its last coefficient is
  rounded.
- The copy's cut-offs are stated of it. Page 18's `2.675` "resulted in the
  lowest overall error in the original tests", and the function those tests
  classify by is the one page 9 prints as final.
- Page 13 states the number it rounds from as `0.99`, which page 9 does not
  print. Taking `1.0` would take a rounding of a number the copy prints two
  ways.
- It needs one more constant, the hundred, and no figure is written in
  another unit for it (see "The constants").

**What the other specification would give.** Over the worked history below,
page 9's function gives `3.2145` and page 13's `3.216`. The difference is
one thousandth of sales to total assets, `0.0015` there. A score within that
distance of a cut-off falls on the other side of it. Both statements are
recorded, and neither is corrected: the copy says the scores are identical,
and the arithmetic says they are not.

**A third printing of the fourth coefficient.** Page 25, comparing Z’ with
the original, says "note, for instance, the coefficient for X1 went from 1.2
to 0.7", and that X4 "showed a coefficient change to 0.42 from 0.6001". So
the copy speaks there in page 13's scale, and prints the fourth coefficient
as `0.6001` where page 9 gives `0.006`, a hundred times which is `0.6`, and
page 13 prints `0.6`. This record takes page 9's `0.006`. The `0.6001`
suggests the printed coefficients are rounded from an estimate the copy does
not print. Nobody here sees that estimate.

### The five ratios, in formula order

Y is the year the metric is computed at. "At Y's close" is the instant Y
closes at. Each ratio is a quotient of v2 concepts, and each is formed from
the operations of the arithmetic record's table and from no other:
subtraction, multiplication, division, and comparison and sign. No root,
power or logarithm enters, and no operation is missing.

| Ratio | Formula | The copy's definition, page 9 |
| :--- | :--- | :--- |
| X1 | (`current_assets` less `current_liabilities`) ÷ `total_assets`, each at Y's close | "X1 = working capital/total assets," |
| X2 | `retained_earnings` ÷ `total_assets`, each at Y's close | "X2 = retained earnings/total assets," |
| X3 | `operating_income` over Y ÷ `total_assets` at Y's close | "X3 = earnings before interest and taxes/total assets," |
| X4 | (P × `shares_outstanding` at Y's close) ÷ `total_liabilities` at Y's close | "X4 = market value equity/book value of total liabilities," |
| X5 | `revenue` over Y ÷ `total_assets` at Y's close | "X5 = sales/total assets, and" |

P is the close at the valuation date.

1. **X1.** Working capital is `current_assets` less `current_liabilities`.
   Page 10: "Working capital is defined as the difference between current
   assets and current liabilities." A negative working capital is a value,
   and gives a negative ratio.
2. **X2.** Page 10: "Retained earnings is the account which reports the total
   amount of reinvested earnings and/or losses of a firm over its entire
   life." v2's `retained_earnings` is "cumulative profit retained rather than
   distributed since inception", negative where losses exceed profits. A
   negative `retained_earnings` is a value.
3. **X3.** EBIT is `operating_income`, as the enterprise value record and the
   capital record read it. A negative `operating_income` is a value.
4. **X4.** The market value of equity is market capitalisation, read from the
   one function in `crates/analyze/src/metrics/market_capitalisation.rs`, at
   the count read at Y's close, with the close at the valuation date read
   through the one function M5-54 built in
   `crates/analyze/src/metrics/reading/price.rs`. The product is written no
   second time. The denominator is `total_liabilities` (see below).
5. **X5.** Sales are `revenue`. Page 13: "The last variable continues to be
   written in terms of number of times."

**Every balance is read at the instant Y closes at,** as the period record
fixes for each of the five rows, and each flow at Y's own row. No balance is
averaged and none is read at the opening.

**No ratio is rounded before its coefficient.** Each is held exactly, as the
arithmetic record requires, and the score is rounded once, when it is
written.

### The score

**Z is the sum of the five ratios, each times its coefficient, the first
four first multiplied by a hundred:**

Z = `0.012` × (100 × X1) + `0.014` × (100 × X2) + `0.033` × (100 × X3) +
`0.006` × (100 × X4) + `0.999` × X5.

- The operations are multiplication and addition. Both are exact.
- Page 10: "Note that the model does not contain a constant (Y-intercept)
  term." So nothing is added to the sum.
- Page 14: "Therefore, the greater a firm’s distress potential, the lower its
  discriminant score." A lower score is nearer distress. The score may be
  below zero, and then it is written with its sign.
- **No score is formed over fewer than five ratios.** The analyze record
  admits a term that contributes nothing only where the metric's definition
  states it. The copy states none. A filer with one ratio absent has no
  score, and the score's reasons say which input is missing.

### Where the copy and the accepted records do not say one thing

Beyond the two specifications, there are four such places. At each, both
statements are recorded, and neither is corrected silently.

**The fourth ratio's denominator: liabilities, or debt.**

- The copy's page 9 prints "book value of total liabilities", and page 11
  "liabilities include both current and long term". The concepts record
  names `total_liabilities` for "liabilities to assets; Altman's X₄".
- The period record's table names the row "the book value of total debt".
- **This record reads `total_liabilities`.** The copy defines the term by
  that name, and v2's concept means "everything owed at the period-end
  date", which is the copy's "both current and long term".
- **The period record's row is kept as it stands.** That record says of its
  table: "Each metric's formula, and which concepts make each term, stay
  with the metric's task. This table fixes only the instant a balance is read
  at". So the row fixes the closing instant, and its words title the term and
  choose no concept. The copy uses the same word for the same denominator:
  page 11 measures the assets "by market value of equity plus debt", and
  page 12 sets the ratio beside "net worth/total debt (book values)".
  Whether the row's words are the origin's is not seen here, the origin being
  unread.
- Reading total debt, `short_term_debt` plus `long_term_debt`, would leave
  out payables, accruals, deferred taxes and the lease obligation, against
  the copy's definition. It is rejected under Alternatives.

**The fourth ratio's numerator: the preferred claim.**

- Page 11: "Equity is measured by the combined market value of all shares of
  stock, preferred and common, while liabilities include both current and
  long term."
- Market capitalisation counts common shares at one ticker's close.
- **This record leaves the preferred claim out.** X4 does not read
  `preferred_equity`.
  - Adding `preferred_equity` would set a carrying amount where the copy
    says market value.
  - It would make the score a consumer the open-questions record did not
    admit, as the common-shareholder record says of enterprise value.
  - The concept's silence is `Unknown`, so the score would be absent for
    every filer silent on it, to correct a claim few carry.
- **The direction of the error.** For a filer with preferred outstanding,
  the numerator is understated by that claim's market value. X4 reads low,
  and the score reads low: nearer distress than the copy's function would
  put it. That does not flatter.

**The cut-off of lowest overall error: `2.675`, or `2.67`.**

- Page 18: "I found that the Z-Score model, using a cutoff score of 2.675,
  was between 82% and 94% accurate."
- Page 22: "In each test, the Type I accuracy using a cutoff score of 2.67
  ranged from 82-94%". Pages 20 and 23 print `2.67` too.
- This record takes neither, because no cut-off enters analyze (see "The
  cut-offs"). Both are recorded for the record that takes one.

**Two slips of numbering.**

- Page 22: "the original model is only applicable to publicly traded
  entities (since X1 requires stock price data)". By page 9's definitions the
  ratio that needs a price is X4, and page 18 has it so: "The majority of
  increase in average Z-Scores was due to the dramatic climb in stock prices
  and its impact on X4."
- Page 26: "a model without X1 - sales/total assets." By page 9 the ratio of
  sales to total assets is X5, and page 12 has it so: "we will specify an
  alternative model (Z”), without X5 at a later point."
- Each is read against page 9's definitions and recorded as a slip. Neither
  changes a term.

### The constants

**Six constants, in one new file,**
`crates/analyze/src/constants/altman.rs`, the one file for the method. Each
is a named `const`, and each cites the copy by its page and quotes the line
it is printed in.

| Constant | Characters | Cited |
| :--- | :--- | :--- |
| `ALTMAN_WORKING_CAPITAL_COEFFICIENT` | `0.012` | page 9, "Z = 0.012X1 + 0.014X2 + 0.033X3 + 0.006X4 +0.999X5", and "X1 = working capital/total assets," |
| `ALTMAN_RETAINED_EARNINGS_COEFFICIENT` | `0.014` | the same line, and "X2 = retained earnings/total assets," |
| `ALTMAN_EBIT_COEFFICIENT` | `0.033` | the same line, and "X3 = earnings before interest and taxes/total assets," |
| `ALTMAN_MARKET_VALUE_OF_EQUITY_COEFFICIENT` | `0.006` | the same line, and "X4 = market value equity/book value of total liabilities," |
| `ALTMAN_SALES_COEFFICIENT` | `0.999` | the same line, and "X5 = sales/total assets, and" |
| `ALTMAN_PERCENT` | `100` | pages 12 and 13, "should be included as 10.0% and not 0.10.", and page 13, "0.10 for 10%" |

- **How each is held.** As characters in the reading grammar, a `&str`, read
  through the one reader of the arithmetic, as a setting's preset is held in
  `settings/`. `0.012` is then exactly twelve thousandths. None is held as a
  binary float, and none as a numerator and a denominator: the characters are
  the line the page prints, so the constant can be compared with its citation
  by eye.
- **The hundred** turns a fraction of one into the percentage the function
  takes. It is the method's factor and not a scale between units: the copy
  fixes it by setting ten against a tenth, and no figure is written scaled by
  it. Each ratio is written as a fraction of one, as the arithmetic record
  requires, and the hundred enters only inside the score's sum. It is held as
  characters too, so that all six reach the arithmetic one way.
- **The constants are named in words, and not by the copy's numbers.** The
  copy itself misnumbers two of the ratios, as above.
- **The products are not constants.** `1.2`, `1.4`, `3.3` and `0.6` are page
  13's numbers. The derivation computes them exactly, as a hundred times page
  9's.

**No setting is added.** The source fixes every coefficient and the
percentage form, so none is a value "neither the filings nor the metric's
cited source fixes". `Settings` gains nothing for this method, and
`settings/` gains no file. GOALS.md's seventh M5 criterion, "Every free
parameter arrives as a setting. Every default states its reason.", is met
with no parameter to state.

### The cut-offs

**What the copy prints.**

- `1.81`, on page 18, as "the lower bond of the zone-of-ignorance", which the
  author advocates "as a more realistic cutoff Z-Score than the score 2.675".
- `2.675` on page 18, and `2.67` on pages 20, 22 and 23, as the cut-off of
  lowest overall error.
- **No upper bound of that zone for this model.** Page 20 defines the zone:
  "The zone of ignorance is that range of Z-Scores where misclassification
  can be observed." It says "10 have Z-Scores between 1.81 and 2.67", and
  page 21 speaks of "the entire overlap area" without bounding it. For Z’
  the copy does print an upper bound, on page 26: "Z'>2.90 = Zone II (no
  errors in nonbankruptcy classification):". For Z it prints none. The
  planner searched the extracted text of all 54 pages for `2.99` and found
  none, and so did this run.

**No cut-off enters analyze.** The score alone is stored. No metric states
which side of a cut-off the score falls on, and `constants/altman.rs` holds
no cut-off.

- **A cut-off changes no number analyze computes.** Which side of `1.81` a
  stored score falls on is a comparison of a stored figure with a threshold.
  The analyze record makes that "a ranking criterion, which is M6's".
- **How this reads beside "Altman's coefficients and cut-offs".** That
  sentence says what a cut-off is: a value the source fixes, which "is not
  the user's to change", and so never a setting. It does not say a metric
  must read one. A constant stands in `constants/` when the derivation reads
  it, and under this record no derivation does. A later record that makes a
  metric of the score's side of a cut-off puts that cut-off in
  `constants/altman.rs` as a named constant quoting its page, and nowhere
  else.
- **The Piotroski record did the same** with his portfolio cut-offs: "They
  enter analyze nowhere: no constant, no setting and no metric."
- **A three-zone metric cannot be sourced.** It would need an upper bound,
  and the copy prints none for this model. A number the copy does not print
  enters nowhere, an upper bound from memory or from a secondary account
  among them.
- **A two-sided metric would have to choose.** `2.675` and `2.67` are two
  numbers. The author advocates `1.81` over either. A record that takes one
  says why.

### Where the terms are formed

- **Working capital** is formed once, in a module of its own,
  `crates/analyze/src/metrics/working_capital.rs`: `current_assets` less
  `current_liabilities`, at one instant. The concepts record lists it as a
  derived value that "belongs to analysis". It is a term, handed over
  nowhere, and a negative working capital is a value of the term. X1 and the
  score read it there, and so does any later metric that wants it. It has
  its own module before a second reader for the reason EBITDA and market
  capitalisation have theirs.
- **The five ratios** are formed in one module,
  `crates/analyze/src/metrics/altman.rs`. Each ratio is a function there,
  from its numbers to its quotient or its one condition. The four ratio
  metrics and the score reach the ratios through it. No metric forms a ratio
  itself, and the score calls the same functions the ratio metrics call.
- **The module reads through `reading.rs` and `reading/price.rs`.** It finds
  no row and reads no crossing itself.

### Four ratios are metrics, and the fifth is a term

**X1 to X4 are each a metric beside the score,** as Piotroski's nine signals
are.

- GOALS.md's M6 composes ranking late, over stored results. A ratio that is
  stored can be screened on. One that is not cannot be, without
  recomputation.
- X1, X2 and X3 read no price. Stored, they stand for a user with no key,
  where the score cannot. M6 asks for "A default composite works with no
  price data, using filing-derived metrics only."
- A filer whose shape lacks one ratio's concept still has the others stored,
  each with its meaning intact (see "By kind").
- The copy's page 14 says each of the four discriminates alone: "Variables X1
  through X4 are all significant at the 0.001 level".

**X5 is a term of the score, and no metric.** This is the record's own
decision, and it leaves one question open.

- **The sentence it cannot decide against.** The open-questions record lists
  what moves with `revenue`'s reading for a `bank` and an `insurer`, and
  says "Altman's X₅ is unreached for both kinds." Its body gives the reason:
  "Altman's X₅ is not reached — a bank has no X₁ and no X₃, so the Z-score is
  not computable for it at all." `revenue` and `total_assets` apply to every
  kind. A stored metric of sales to total assets would be computed for a
  `bank` and an `insurer`, from the very reading of `revenue` that record
  fixes, and the sentence would no longer hold.
- **As a term it holds.** The score is absent for both kinds, so no X5 is
  formed for either, and no figure moves with their `revenue`.
- **The kind does not cross v2,** so a metric cannot decline a kind.
- **The copy says X5 does not stand alone.** Page 12: "In fact, based on the
  univariate statistical significance test, it would not have appeared at
  all." Page 14: "Variable X5 does not show a significant difference among
  groups". It also says the ratio varies by trade, on page 12: "Still, there
  is a wide variation among industries in asset turnover".
- **What it costs.** The score's figure cannot be re-added from the stored
  table, since one of its five terms is not there. A user with no key sees
  three of the five ratios, and not the fourth non-price one.
- **The open question.** Whether X5 is a metric, for every kind or for some,
  needs the open-questions record's sentence superseded, or the kind to
  cross. Either is a record of its own. This record does not propose it.
  Adding a metric later bumps nothing, and the term's function stays where
  it is.

### Which periods

**Each of the five is stated over a year,** under the series record's test,
which covers "every term of Altman's score and the score itself".

- The copy's page 8: "For the initial sample test, the data are derived from
  financial statements dated one annual reporting period prior to
  bankruptcy." Each ratio is read off one annual report's statements.
- Their `stated_at` is `Duration`. X1 and X2 read balances alone, and are
  still stated over a year, as terms of the score: a quarter's balance sheet
  is not the statement the copy's function reads.

At a duration that is not a year, each gives exactly one `declined` reason,
the first of `start_not_a_date`, `end_not_a_date`, `not_a_year` and
`overlaps_another_year`, and makes no read, as the series record says:
"Altman's sales over a quarter are not a term of his score".

**The balances are found by the accepted seek,** `closing_instant`. No metric
finds a row itself.

**The score makes eight reads,** in input order, each concept where it first
appears in the function, then the price read:

| # | Concept | Read at |
| ---: | :--- | :--- |
| 1 | `current_assets` | Y's closing instant |
| 2 | `current_liabilities` | Y's closing instant |
| 3 | `total_assets` | Y's closing instant |
| 4 | `retained_earnings` | Y's closing instant |
| 5 | `operating_income` | Y |
| 6 | `shares_outstanding` | Y's closing instant |
| 7 | `total_liabilities` | Y's closing instant |
| 8 | `revenue` | Y |

`total_assets` divides four ratios and is read once: one date and one row is
one read.

**Each ratio metric's reads** are its own inputs, in the same way:

| Metric | Inputs, in reason order | Reads |
| :--- | :--- | ---: |
| `altman_working_capital_to_total_assets` | `current_assets`, `current_liabilities`, `total_assets` | 3: reads 1, 2 and 3 |
| `altman_retained_earnings_to_total_assets` | `retained_earnings`, `total_assets` | 2: reads 4 and 3 |
| `altman_ebit_to_total_assets` | `operating_income`, `total_assets` | 2: reads 5 and 3 |
| `altman_market_value_of_equity_to_total_liabilities` | `shares_outstanding`, `total_liabilities` | 2: reads 6 and 7, then the price read |

### The share-basis rule

The score and X4 are built on the count through market capitalisation, so
each calls the rule, through the one function in
`crates/analyze/src/metrics/reading/price.rs`.

- **E is Y's end,** the duration's end, as the share-basis record fixes for a
  metric stated at a duration. It is also the date of the closing instant,
  where the count is read. **T is the close's trading date.**
- **The rule is asked once.** The count is the only figure on a share basis
  that either metric reads. Every other input is in currency, and a split
  does not restate it.
- **No opening count is read,** so the exposure the market capitalisation
  record names in its averaged count does not reach these two.
- **What the caller must ask.** Each needs a price answered at or before E
  for the covered range to reach it. A caller that asked less gets
  `not_covered`, never a number.

**The pair is not one date's.** The count is the closing instant's and the
close the valuation date's. The copy states no date for the market value of
equity. The provider record lists "Altman's X₄, the market value of equity"
at the valuation date, and the period record leaves the rest here: "The
price's date is the score's task, under the share-basis rule." So each
year's score is that year's statements beside today's close, as the market
capitalisation record places its multiples. Which period a ranking reads is
M6's.

### The `declined` conditions

Each is asked only where every read and the price read gave a number. They
are asked in formula order, and the first that holds is the one reason. Each
is a sign or zero test and needs no literal.

| Condition | Holds when | Asked by | Defined |
| :--- | :--- | :--- | :--- |
| `zero_total_assets` | `total_assets` at Y's close is zero | X1, X2, X3, the score | once, in `crates/analyze/src/metrics/liabilities_to_assets.rs`; reached from there |
| `negative_total_assets` | `total_assets` at Y's close is below zero | X1, X2, X3, the score | new, once, beside the ratios' module |
| `non_positive_close` | the close is zero or below | X4, the score | once, beside the discounted cash flow margin; reached from there |
| `non_positive_shares_outstanding` | the count at Y's close is zero or below | X4, the score | once, beside the market capitalisation term; reached from there |
| `zero_total_liabilities` | `total_liabilities` at Y's close is zero | X4, the score | new, once, beside the ratios' module |
| `negative_total_liabilities` | `total_liabilities` at Y's close is below zero | X4, the score | new, once, beside the ratios' module |

The order for each metric:

- **X1, X2 and X3:** `zero_total_assets`, `negative_total_assets`.
- **X4:** `non_positive_close`, `non_positive_shares_outstanding`,
  `zero_total_liabilities`, `negative_total_liabilities`.
- **The score:** `zero_total_assets`, `negative_total_assets`,
  `non_positive_close`, `non_positive_shares_outstanding`,
  `zero_total_liabilities`, `negative_total_liabilities`. That is the first
  ratio's divisor, then the fourth ratio's numerator, then its divisor. The
  fifth ratio's divisor is the first's, already asked.

**`zero_total_assets` is reached, and not defined again.** It names "Total
assets are zero", at the one instant its metric divides by. Each ratio here
divides by `total_assets` at one instant, Y's close, so the name says what
holds. The Piotroski record did not reuse it because two instants were
divided by there. A reached constant's visibility is widened where it must
be.

**A `total_assets` below zero is a condition, and not a signed figure.** No
page of the copy states a ratio there. This is the record's own decision:

- Over a negative divisor each of four ratios changes sign. A deficit in
  working capital, in retained earnings or in operating income would read as
  a surplus, and the score would rise with distress. That reverses page 14's
  "the greater a firm’s distress potential, the lower its discriminant
  score", and it is a wrong number that looks right.
- A balance sheet that totals below zero is no total. The cost is an absence
  for a filer no filing describes.
- `liabilities_to_assets` gives a signed figure there, and keeps doing so.
  A negative fraction of liabilities is visibly wrong. A flattering score is
  not.

**Liabilities of zero** are a zero divisor. No condition on main names it,
so `zero_total_liabilities` is new. **Liabilities below zero are a
condition** for the same reason as total assets: a total below zero is no
total, and X4 over it would be a market value over no claim, with the
divisor's sign.

**The market capitalisation term's two** are asked where that term asks
them, in its one function, and are reached from their one definitions.

**These are values, and never conditions:**

- a negative working capital, a negative `retained_earnings` and a negative
  `operating_income`. Each gives its ratio with its sign. The copy's Table 1,
  on page 13, prints negative means for the bankrupt group in its first three
  rows: `-6.1%`, `-62.6%` and `-31.8%`.
- a `revenue` of zero, which gives an X5 of zero, and a `revenue` below
  zero, which gives X5 with its sign.
- a `total_liabilities` above `total_assets`, and a score below zero.

The entries also list the four year conditions, and the two that read a
price list `share_basis_date_not_a_date`. `input_not_a_decimal` reaches all
five from the one definition beside the reader. No condition already defined
is defined again.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first.** At a duration that is not a year, its one reason
   is the outcome and nothing below is asked.
2. **The history's reads, in input order.** Every read is made, and none
   stops another. Each gives a number or exactly one reason:
   - `input_unknown` or `input_not_applicable`, naming the concept and the
     row read
   - `no_period` with `closing_instant`, where the history lacks that
     instant
   - `declined` with `input_not_a_decimal`
3. **The price read, last,** for the score and X4, made whatever the reads
   gave, through the one function M5-54 built, with E as above. It gives a
   number or exactly one reason, in the discounted cash flow record's order:
   - no crossing: `no_price` with `not_asked`
   - an absence answered for the valuation date: `no_price` with
     `answered_absent`, carrying the date asked
   - a price: the rule, giving `basis_unproven` with `splits_absent`,
     `not_covered` or `split_within` by the first clause that fails, or
     `declined` with `share_basis_date_not_a_date` where a date it compares
     is outside the grammar
   - a close outside the decimal grammar: `declined` with
     `input_not_a_decimal`

   The answer at the valuation date is read by its place in the crossing, so
   `date_not_asked` has no witness here.
4. **The conditions,** in the order above, only where every read and the
   price read gave a number.

**Nothing is substituted.** The price read never falls back to another date,
to an adjusted close, to a book value of equity or to a stand-in. The copy's
sentence that opens at the foot of page 22, "And, to be perfectly correct,
the Z-Score model is a publicly traded firm model and ad hoc", runs on at the
head of page 25: "adjustments are not scientifically valid." An `Unknown`
`retained_earnings` is never read as zero, and neither is any other input.

**The score is built on its terms, and carries their reasons.** Working
capital's are its two concepts' reads. Market capitalisation's are the
count's read and the price read. Each ratio's are its terms' and its
divisor's. A read shared by several ratios is made once and gives its reason
once. No outcome says a term was absent.

**With no key, no number.** The caller with no key asks nothing, so the
crossing is `not_asked`. The score and X4 are each absent with `no_price`
and `not_asked`, after any reasons of their own reads. That is GOALS.md's
ninth M5 criterion: "A metric with no price data is absent with a reason. It
never fails the run and never shows a wrong number."

- So the score is a metric of a valued shortlist, and of no screen over the
  corpus. GOALS.md's second M5 criterion keeps that screen free of price
  calls: "screening the corpus needs no price call".
- A user with no key still sees X1, X2 and X3, each computed from filings
  alone.

No seventh reason is needed, and no frozen byte changes.

### By kind

Where every other input is a `Value`:

| Kind | X1 | X2 | X3 | X4 | The score |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `operating` | computed | computed | computed | computed | computed |
| `reit` | absent, two reasons | computed | computed | computed | absent, two reasons |
| `bank`, `insurer`, `investment_company` | absent, two reasons | computed | absent, one reason | computed | absent, three reasons |

- **X1's two reasons** are `input_not_applicable` for `current_assets` and
  for `current_liabilities`, each at Y's closing instant.
- **X3's one** is `input_not_applicable` for `operating_income` at Y.
- **The score's** are those, in read order: the two for a `reit`, and the
  two and then `operating_income`'s for the other three kinds.

That is what the concepts record says arrives "without anyone deciding it
per filing": a `bank` has no X1 and no X3, and no score.

**What an `operating` filer that is no manufacturer gets** is the score. v2's
kinds do not tell a manufacturer from a retailer or a services firm, so the
function fitted on manufacturers is applied to every `operating` filer of
any trade. Nothing marks it (see "What no check can see").

**Z" is left to a record of its own.** The copy prints it on page 27 for
non-manufacturers, over a fourth ratio at book value and without the fifth.
It reads no price, so it would serve a user with no key and every trade.
This record does not propose it, and does not foreclose it:

- It is another score, with its own coefficients, its own book-value ratio
  and its own name. Adding a metric bumps nothing.
- The copy's page 27 adds, for one use, "a constant term of +3.25". A record
  for Z" decides whether that enters.
- It is no stand-in for Z where a price is missing. A filer's Z is absent
  with `no_price`, and never Z" under Z's name.

Z’ is the copy's re-estimation for privately held firms. It is another score
too, and this record proposes none for it.

**The ratios stored for a kind the score is absent for** are computed from
that kind's own statements. A `bank`'s X4 sets its market capitalisation
over liabilities that are mostly deposits, and its X2 reads retained earnings
against a balance sheet of loans. Whether either means for a bank what it
means for the copy's sample is not a check this record has.

**A filer with no kind** resolves every concept through the attempt, so a
concept not found is `Unknown` and gives `input_unknown`, never
`input_not_applicable`.

### The names

They cross `analyze-store` v1 as characters, under the common-shareholder
record's naming rule.

- **Every name carries `altman_`.** Each ratio reads its balance at the
  year's close, where the method's own ratios read the average. So his
  EBIT to total assets never shares a name with a return that reads
  otherwise, as the Piotroski record kept `piotroski_f_roa` apart from
  `return_on_assets`.
- **None carries `common`.** The rule puts that word in a name "exactly when
  its derivation forms common equity or income available to common". No
  derivation here forms either. Market capitalisation is a common claim by
  its count, and `price_to_sales` reads it without the word.
- **None carries `per_share`.** Each reads totals, and a count that is not a
  per-common-share concept.
- **The ratios are named in words.** The copy numbers them, and twice
  misnumbers them.
- **`ebit`** follows the copy's "EBIT/TA", as `enterprise_value_to_ebit`
  follows the catalogue. The `meaning` says it is `operating_income`.
- **`altman_z_score`** is Z. A later Z" takes a name of its own.
- **Working capital and sales to total assets** have no name crossing the
  contract, since neither is a metric. Working capital's module is named
  `working_capital`, and the fifth ratio is a function of the ratios' module.

### The method version

These are part of the method `METHOD_VERSION` names:

- the specification, page 9's, and the hundred
- each of the six constants
- each ratio's concepts, and the instant each is read at
- the working capital term and its module
- that X4's numerator is market capitalisation alone, without the preferred
  claim, and its denominator `total_liabilities`
- that each is stated over a year, E, and the rule asked once
- the five names and their meanings
- the six conditions and their order
- that no score is formed over fewer than five ratios

A later change to any of them, the specification, a coefficient, a ratio's
concepts or a condition included, is a version bump and a superseding
record, and never an edit to this one.

It lands at version 1. The score is not computed yet, so no stored outcome
changes, and adding a metric is not a bump.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, with one more value of `unit` and no
field.

- **`unit`.**
  - The four ratios state `FractionOfOne`: `0.15` is fifteen per cent, and
    never `15`. The copy writes each of the four as a percentage, X4
    included: Table 1 prints its means as `40.1%` and `247.7%`. So X4 takes
    the unit `debt_to_equity` takes, and not `Multiple`.
  - `Unit` gains `DiscriminantScore`: a weighted sum of ratios, a number on
    the scale its function gives. `2.675` is two and six hundred and
    seventy-five thousandths. It is never a fraction of one and never a
    percentage, it is no count, and it may be below zero. None of the five
    values on main says that: a score shown as a fraction of one could be
    read as a percentage, and a score is not how many times one amount covers
    another. The entry gained `Years` and `Signals` the same way.
- **`stated_at`.** `Duration`, for all five.
- **`inputs`,** in read order, the price being no concept and not listed,
  each balance with the reading `Closing` and each flow with none: for the
  score, the eight of the reads table, and for each ratio, its own as listed
  above.
- **`conditions`,** in the order asked: each metric's own from the order
  above; then `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year` by their constants; then, for X4 and the score,
  `share_basis_date_not_a_date`, after the four as the share-basis record
  places it. That is the order the discounted cash flow pair lists theirs in.
  `input_not_a_decimal` is generated from its one definition.
- **`meaning`** says the form in words, the instant, and the scale:
  - `altman_working_capital_to_total_assets`: "Altman's first ratio, as a
    fraction of one: current assets less current liabilities, over total
    assets, each at the instant the year closes at."
  - `altman_retained_earnings_to_total_assets`: "Altman's second ratio, as a
    fraction of one: retained earnings over total assets, each at the
    instant the year closes at."
  - `altman_ebit_to_total_assets`: "Altman's third ratio, as a fraction of
    one: operating income for the year, which is earnings before interest
    and tax, over total assets at the instant the year closes at."
  - `altman_market_value_of_equity_to_total_liabilities`: "Altman's fourth
    ratio, as a fraction of one: market capitalisation, the unadjusted close
    at the valuation date times common shares outstanding at the instant the
    year closes at, over total liabilities at that instant. The preferred
    claim is not counted."
  - `altman_z_score`: "Altman's Z-score, by his final discriminant function:
    0.012, 0.014, 0.033 and 0.006 times his first four ratios, each taken as
    a percentage, plus 0.999 times revenue for the year over total assets.
    The ratios are working capital, retained earnings and operating income,
    each over total assets, and market capitalisation, the unadjusted close
    at the valuation date times common shares outstanding, over total
    liabilities, every balance at the instant the year closes at. The
    preferred claim is not counted. A lower score is nearer distress."

  The score's meaning says which specification the figure is in, so that no
  reader takes a score over percentages for one over fractions with the
  rounded coefficient.

The entry shape can say everything these metrics read. Nothing is missing
from it.

### Sources

Under `sources-a-task-can-read.md` (Accepted).

**The origin,** named as the period record names it: Altman, E. I.,
"Financial Ratios, Discriminant Analysis and the Prediction of Corporate
Bankruptcy", *The Journal of Finance* 23(4), 1968, pp. 589–609. It was not
read from the workspace. No run has opened it: the task records that the
publisher's page answered 403 and JSTOR's served no paper. This record
writes no page, table, coefficient, cut-off or quotation from it, and
nothing a textbook or a summary says it holds.

**The open primary beside it,** the author's own restatement: Altman, E. I.,
"Predicting Financial Distress of Companies: Revisiting the Z-Score and
ZETA® Models", Stern School of Business, New York University, July 2000.

- **Its path on the author's own site,**
  <https://pages.stern.nyu.edu/~ealtman/Zscores.pdf>, answered 500 to this
  run's request, as the task says it did to the planning runs of 2026-10-19,
  2026-11-06 and 2026-11-22.
- **The copy read is an archive's capture of that path, and not the live
  page.** It is the Internet Archive's capture taken 2024-07-17, served as
  the bytes the author's site sent, at
  <https://web.archive.org/web/20240717121208id_/https://pages.stern.nyu.edu/~ealtman/Zscores.pdf>:
  54 pages, 134,818 bytes, sha256
  `cc9acbf25f3618afcd39820689d5950eeebaec478fea71e4d8d513fd7f763074`. That is
  the digest the task names, and this run computed it over its own download.
- **The archive's index of the path,** read in this run, lists 64 captures
  answered 200, from 2004-06-10 to 2024-07-17. 63 stand under the content
  digest `HJAK2HTORD6ALLQZKZH33EF4ZLSIVTGR`, which is the base32 SHA-1 this
  run computed over the downloaded bytes, and one, of 2008-12-05, under
  another. Every capture it lists after 2024-07-17 answered a redirect or
  500, the first of them a redirect of 2024-09-15.
- **How it was read.** The copy is a PDF whose text a fetch tool does not
  render. It was downloaded whole into a temporary directory outside the
  repository, and its text extracted there with `pdfminer.six` from PyPI.
  `web.archive.org` is not in the sandbox's allowed domains, so it was read
  under a per-command allowance. Each quotation was checked against the
  extracted text with whitespace collapsed, and is written with the spacing
  the page prints.
- **Pages.** The copy prints no page number the extraction gives. A
  quotation is located by its page's place in the file, the title page being
  page 1. Subscripts are extracted on the line, so the copy's X₁ reads `X1`.

| Page | Quoted as printed | Cited for |
| :--- | :--- | :--- |
| 1 | "PREDICTING FINANCIAL DISTRESS OF COMPANIES: REVISITING THE Z-SCORE AND ZETA® MODELS", "Edward I. Altman*", "July 2000", and "This paper is adapted and updated from E. Altman, “Financial Ratios, Discriminant Analysis and the Prediction of Corporate Bankruptcy,” Journal of Finance, September 1968" | whose restatement of which paper the copy is |
| 7 | "The initial sample is composed of 66 corporations with 33 firms in each of the two groups." "The bankrupt (distressed) group (Group 1) are manufacturers that filed a bankruptcy petition under Chapter X of the National Bankruptcy Act from 1946 through 1965." "Group 2 consists of a paired sample of manufacturing firms chosen on a stratified random basis." "The firms are stratified by industry and by size, with the asset size range restricted to between $1 and $25 million." | the sample |
| 8 | "For the initial sample test, the data are derived from financial statements dated one annual reporting period prior to bankruptcy." | the year, and one report's statements |
| 9 | "The final discriminant function is as follows:" "Z = 0.012X1 + 0.014X2 + 0.033X3 + 0.006X4 +0.999X5" "X1 = working capital/total assets," "X2 = retained earnings/total assets," "X3 = earnings before interest and taxes/total assets," "X4 = market value equity/book value of total liabilities," "X5 = sales/total assets, and" | the function and the five ratios |
| 10 | "Z = overall index." "Note that the model does not contain a constant (Y-intercept) term." "Working capital is defined as the difference between current assets and current liabilities." "Retained earnings is the account which reports the total amount of reinvested earnings and/or losses of a firm over its entire life." | the sum, and two terms |
| 10 | the retained earnings account "is subject to "manipulation" via corporate quasi-reorganizations and stock dividend declarations" | what no check can see |
| 11 | "Equity is measured by the combined market value of all shares of stock, preferred and common, while liabilities include both current and long term." "The measure shows how much the firm’s assets can decline in value (measured by market value of equity plus debt) before the liabilities exceed the assets and the firm becomes insolvent." | X4's two sides |
| 12 | "It also appears to be a more effective predictor of bankruptcy than a similar, more commonly used ratio; net worth/total debt (book values)." "At a later point, we will substitute the book value of net worth for the market value in order to derive a discriminant function for privately held firms (Z’) and for non-manufacturers (Z”)." | the word "debt", and the two re-estimations |
| 12 | "In fact, based on the univariate statistical significance test, it would not have appeared at all." "Still, there is a wide variation among industries in asset turnover, and we will specify an alternative model (Z”), without X5 at a later point." | X5 alone, and by trade |
| 12 | "Due to the original computer format arrangement, variables X1 through X4 must be calculated as absolute percentage values." | the percentage form |
| 12 to 13 | The sentence that opens at the foot of page 12, "For instance, the firm whose net working capital to total assets (X1) is 10%", and runs on at the head of page 13, "should be included as 10.0% and not 0.10." | the hundred |
| 13 | "Only variable X5 (sales to total assets) should be expressed in a different manner: that is, a S/TA ratio of 200% should be included as 2.0." | X5 as times |
| 13 | "Over the years many individuals have found that a more convenient specification of the model is of the form: Z = 1.2X1 + 1.4X2 + 3.3X3 + 0.6X4 + 1.0X5." "Using this formula, one inserts the more commonly written percentage, for example, 0.10 for 10%, for the first four variables (X1-X4) and rounds the last coefficient off to equal 1.0 (from 0.99)." "The last variable continues to be written in terms of number of times." "The scores for individual firms and related group classification and cutoff scores remain identical." | the other specification |
| 13 | Table 1, "Variable Means and Test Significance": the bankrupt group's means extract as a column, `-6.1%`, `-62.6%`, `-31.8%`, `40.1%` and `1.5X`, and the nonbankrupt group's as `41.4%`, `35.5%`, `15.4%`, `247.7%` and `1.9X`, each in the order of the rows X1 to X5 | negative ratios as values, and X4 as a percentage |
| 14 | "Variables X1 through X4 are all significant at the 0.001 level" "Variable X5 does not show a significant difference among groups" "Therefore, the greater a firm’s distress potential, the lower its discriminant score." | which ratios stand alone, and the score's direction |
| 18 | "I found that the Z-Score model, using a cutoff score of 2.675, was between 82% and 94% accurate." "The majority of increase in average Z-Scores was due to the dramatic climb in stock prices and its impact on X4." "I advocate using the lower bond of the zone-of-ignorance (1.81) as a more realistic cutoff Z-Score than the score 2.675. The latter resulted in the lowest overall error in the original tests." | the cut-offs, and X4 as the ratio that reads a price |
| 20 | "The zone of ignorance is that range of Z-Scores where misclassification can be observed." "Of the 14 misclassified firms in this secondary sample, 10 have Z-Scores between 1.81 and 2.67" | the zone, and `2.67` |
| 21 | "just under one-third of the 66 firms in this last sample have Z-Scores within the entire overlap area" | a zone with no upper bound printed |
| 22 | "In each test, the Type I accuracy using a cutoff score of 2.67 ranged from 82-94%" "the original model is only applicable to publicly traded entities (since X1 requires stock price data)" | `2.67`, and a slip |
| 22 to 25 | The sentence that opens at the foot of page 22, "And, to be perfectly correct, the Z-Score model is a publicly traded firm model and ad hoc", and runs on at the head of page 25, "adjustments are not scientifically valid." | the model's reach, and no stand-in for the price |
| 23 | "* Using 2.67 as cutoff score (1.81 cutoff accuracy in parenthesis)" | `2.67` |
| 25 | "Z’ = 0.717(X1) + 0.847(X2) + 3.107(X3) + 0.420(X4) + 0.998(X5)" "note, for instance, the coefficient for X1 went from 1.2 to 0.7" "showed a coefficient change to 0.42 from 0.6001" | Z’, and the third printing |
| 26 | "a model without X1 - sales/total assets." "Z'>2.90 = Zone II (no errors in nonbankruptcy classification):" | a slip, and an upper bound printed for Z’ alone |
| 27 | "Z" = 6.56 (X1)+ 3.26 (X2) + 6.72 (X3) + 1.05 (X4)" "In the emerging market model, we added a constant term of +3.25" | Z" |

Page 23 holds a table and page 24 nothing the extraction gives, which is
why the sentence of page 22 runs on at the head of page 25.

**Market capitalisation's form** is cited from its one module, as its record
cites it. This record opens nothing more for it.

**Where no page of the copy fixes a thing,** it is the record's own
decision, with its reason above:

- which of the two specifications is taken
- `total_liabilities` as v2's concept for the copy's term, `revenue` for its
  "sales" and `operating_income` for its EBIT
- the close at the valuation date as the price, the copy stating no date
- the preferred claim left out
- X1 to X4 as metrics, and X5 as a term
- a total below zero as a condition, for assets and for liabilities
- that no cut-off enters analyze
- that every `operating` filer is scored
- the unit, the names and the constants' names

No copy opened gave another digest, and the copy states the function as the
task quotes it. A secondary account of the score, a textbook's, an
encyclopedia's or a data vendor's, is cited nowhere. This record names no
source outside the repository but the origin, unread, and this one copy.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at`, and a `Crossing`
built as the contract type. They read no file. Each stands in `tests/` or
under `#[cfg(test)]`.

**The worked history.** The year 2024-01-01 to 2024-12-31, and the instants
2023-12-31 and 2024-12-31.

| Balance | 2023-12-31 | 2024-12-31 |
| :--- | ---: | ---: |
| `current_assets` | 300 | 400 |
| `current_liabilities` | 300 | 250 |
| `total_assets` | 800 | 1000 |
| `retained_earnings` | 200 | 300 |
| `shares_outstanding` | 8 | 10 |
| `total_liabilities` | 450 | 500 |

Over the year, `operating_income` is 120 and `revenue` 1500.

**The preset crossing** is the share-basis record's: a valuation date of
2025-06-30 answered with a close of 60 traded 2025-06-30, the earlier date
2024-12-31 answered with a close traded 2024-12-31, and an empty split list.
E is 2024-12-31 for every figure below.

At the least:

- **The figures, worked by hand.**

  | Term | Working | Figure |
  | :--- | :--- | ---: |
  | working capital | 400 − 250 | 150 |
  | X1 | 150 ÷ 1000 | `0.15` |
  | X2 | 300 ÷ 1000 | `0.3` |
  | X3 | 120 ÷ 1000 | `0.12` |
  | market capitalisation | 60 × 10 | 600 |
  | X4 | 600 ÷ 500 | `1.2` |
  | X5 | 1500 ÷ 1000 | 1.5 |

  The score's five products are 0.012 × 15 = 0.18, 0.014 × 30 = 0.42, 0.033
  × 12 = 0.396, 0.006 × 120 = 0.72 and 0.999 × 1.5 = 1.4985. Their sum is
  `3.2145`. The four ratio metrics give `0.15`, `0.3`, `0.12` and `1.2`.
- **The specification not taken.** 1.2 × 0.15 + 1.4 × 0.3 + 3.3 × 0.12 + 0.6
  × 1.2 + 1.0 × 1.5 is 0.18 + 0.42 + 0.396 + 0.72 + 1.5, which is `3.216`.
  The score is `3.2145` and never `3.216`. The two are not one.
- **A ratio is never taken as a fraction inside the sum.** 0.012 × 0.15 +
  0.014 × 0.3 + 0.033 × 0.12 + 0.006 × 1.2 + 0.999 × 1.5 is `1.51566`, which
  the score never is.
- **The opening balances enter no figure.** With the 2023-12-31 column as
  given, every figure is as above. X2 is never 300 ÷ 800, `0.375`, and never
  over the mean of 800 and 1000.
- **Negative terms are values.** `current_assets` 200, `retained_earnings`
  −400 and `operating_income` −80: X1 is `-0.05`, X2 `-0.4` and X3 `-0.08`.
  The score is −0.06 − 0.56 − 0.264 + 0.72 + 1.4985, `1.3345`. With
  `shares_outstanding` 1 and `revenue` 300 beside them, X4 is `0.12` and the
  score −0.06 − 0.56 − 0.264 + 0.072 + 0.2997, `-0.5123`, with its sign.
- **Each `declined` condition.**
  - `total_assets` 0: `zero_total_assets` for X1, X2, X3 and the score. X4
    is `1.2`.
  - `total_assets` −1000: `negative_total_assets` for the same four, never
    `-0.15`, `-0.3` and `-0.12`. X4 is `1.2`.
  - A close of 0, and of −1: `non_positive_close` for X4 and the score. X1,
    X2 and X3 are unchanged.
  - `shares_outstanding` 0, and −10: `non_positive_shares_outstanding` for
    X4 and the score.
  - `total_liabilities` 0: `zero_total_liabilities` for X4 and the score.
  - `total_liabilities` −500: `negative_total_liabilities` for both, never
    `-1.2`.
  - **The first that holds.** `total_assets` 0 beside a close of 0: the
    score gives `zero_total_assets` alone, and X4 `non_positive_close` alone.
    A close of 0 beside `shares_outstanding` 0 gives `non_positive_close`
    alone. `shares_outstanding` 0 beside `total_liabilities` 0 gives
    `non_positive_shares_outstanding` alone. `total_assets` −1000 beside
    `total_liabilities` 0: the score gives `negative_total_assets` alone.
  - **Only where every read gave a number.** `total_assets` 0 beside no
    crossing: the score gives `no_price` with `not_asked` alone, and X1
    gives `zero_total_assets`.
- **No price, by each witness,** for X4 and the score:
  - No crossing: `no_price` with `not_asked`. X1, X2 and X3 are `0.15`,
    `0.3` and `0.12`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying 2025-06-30.
  - `date_not_asked` has no witness: the test shows `Crossing` cannot be
    built without `at_valuation_date`.
- **The share-basis rule, by each clause,** for X4 and the score:
  - The split list absent: `splits_absent`.
  - The valuation date alone, no earlier date answered: `not_covered` naming
    2025-06-30 through 2025-06-30, 2024-12-31 and 2025-06-30.
  - A split dated 2025-03-14: `split_within` naming it, 2024-12-31 and
    2025-06-30, and never a figure. A split dated 2024-12-31: `split_within`.
  - A split dated 2024-06-15, inside the year: `1.2` and `3.2145`. The count
    read is the closing one alone, and the split is before E.
- **A date outside the grammar.** The paired close's trading date
  `2025-6-30`: `declined` with `share_basis_date_not_a_date`, for both.
- **An unreadable close.** `1e2`: `declined` with `input_not_a_decimal`.
- **The closing instant missing.** The score gives six `no_period` reasons
  with `closing_instant`, one for each balance in input order. X1 gives
  three, X2 two, X3 one and X4 two.
- **Each input `Unknown` in turn,** at the row it is read at. Each of the
  eight gives the score `input_unknown` naming that concept and that row,
  alone. Each ratio metric that reads the concept gives the same, and each
  that does not is unchanged.
  - `retained_earnings` `Unknown` at 2024-12-31: X2 and the score are
    `input_unknown` naming it there. The score is never `2.7945`, which
    reading it as zero would give.
- **The preferred claim is not read.** `preferred_equity` 500 at 2024-12-31
  gives `1.2` and `3.2145`. `Unknown` there gives `1.2` and `3.2145` too.
- **A filer of each kind.** The worked history with every concept v2 does
  not apply to that kind `NotApplicable`:
  - `operating`: `0.15`, `0.3`, `0.12`, `1.2` and `3.2145`.
  - `reit`: X1 and the score each give `input_not_applicable` for
    `current_assets` and then `current_liabilities`, at 2024-12-31. X2, X3
    and X4 are `0.3`, `0.12` and `1.2`.
  - `bank`, `insurer` and `investment_company`: X1 gives those two. X3 gives
    `input_not_applicable` for `operating_income` at the year. The score
    gives the three, in that order. X2 and X4 are `0.3` and `1.2`.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31 with every input
  `Unknown`: `not_a_year` alone, for all five.
- **The order of reasons.** `retained_earnings` `Unknown` at 2024-12-31,
  `revenue` `Unknown` over the year, and no crossing. The score gives
  `input_unknown` for `retained_earnings`, then `input_unknown` for
  `revenue`, then `no_price` with `not_asked`. X4 gives `no_price` alone,
  and X2 `input_unknown` alone.
- **Each term still formed once.** `price_to_common_book`, `price_to_sales`,
  `price_to_free_cash_flow` and `liabilities_to_assets` give their pinned
  figures and reasons unchanged beside these.
- **The catalogue.**
  - Each entry's `inputs`, readings and `conditions` are as listed above.
  - No entry's `inputs` holds `preferred_equity`.
  - The four ratios state `Unit::FractionOfOne` and the score
    `Unit::DiscriminantScore`.
  - No name holds `common` or `per_share`.
  - No entry is named for working capital or for sales to total assets.
  - Each constant read through the one reader is the number its characters
    state: `ALTMAN_PERCENT` times `ALTMAN_WORKING_CAPITAL_COEFFICIENT` is
    written `1.2`.

### What no check can see

- **Whether the copy says what this record says.** It was opened in this run
  and is quoted as printed. The implementing tasks cite it as the sources
  record says. A copy that on reading says otherwise is a stop for that
  task, not a choice.
- **Whether the restatement prints what the paper printed.** The origin
  stays unread, so whether the restatement of 2000 prints what the paper of
  1968 printed is seen by nobody here.
- **That the copy is an archive's capture.** The author's page has not
  answered a run. What the archive holds is what the page sent on the days
  it was captured, by the archive's own account.
- **The estimate behind the printed coefficients.** Page 25's `0.6001`
  beside page 9's `0.006` suggests the printed ones are rounded. The score
  is exact over what is printed.
- **The sample the function was fitted on.** The coefficients were fitted on
  66 manufacturers, 33 of them bankrupt between 1946 and 1965 and the 33
  paired with them restricted to assets between $1 and $25 million. They are
  applied to every `operating` filer of any trade and size. A retailer's
  sales to total assets raise its score for its trade alone.
- **The pair's dates.** The count is the closing instant's and the close the
  valuation date's. So the market value of equity is no value the market
  stated at the balance sheet's date. The score moves with the close: the
  copy's page 18 lays most of a rise in average scores to "the dramatic climb
  in stock prices and its impact on X4".
- **The preferred claim,** left out of X4, as stated above.
- **Retained earnings.** The copy cautions, on page 10, that the retained
  earnings account "is subject to "manipulation" via corporate
  quasi-reorganizations and stock dividend declarations". Nothing here
  adjusts for either.
- **What `operating_income` and `total_liabilities` hold.** Each is the line
  as presented. A year's impairment is inside the first. The operating lease
  obligation is inside the second, which the copy's sample predates.
- **The class of a share, the count's date and the currency,** each as the
  market capitalisation record states it of its own term.
- **A figure at Y beside a close at the valuation date.** Where Y ends long
  before, an old year's score is stated beside today's close, and nothing
  marks it.
- **A ratio stored for a kind the function was never fitted on,** as stated
  under "By kind".
- **A metric that forms a ratio, working capital or market capitalisation
  itself.** Only review sees it.
- **A cut-off written in the shell.** Anchor 1 keeps analysis out of the
  shell, and only review sees a threshold typed there.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The six
  constants stand in `constants/`, one of its three places, and the
  derivation needs no literal: every test is a sign or zero test.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule, three places and catalogue
- the share-basis record's comparisons and condition, and its E for a metric
  stated at a duration
- the arithmetic record's reading, operations and written form
- the period record's readings and seeks, its fourth row included
- the series record's year test
- the common-shareholder record's naming rule, and the open-questions
  record's admission of `preferred_equity`, which gains no consumer
- the market capitalisation record's term, its module and its two conditions
- the provider record's ask and answer
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains one value of `unit` and no field

No sentence of an accepted record is superseded. One question here would
need one to change, and it is left open and named:

- **Whether the fifth ratio is a metric.** The open-questions record's
  "Altman's X₅ is unreached for both kinds." holds while X5 is a term of a
  score those kinds do not have. A stored X5 needs that sentence superseded,
  or a v3 that carries the kind. Either is proposed only in a record of its
  own.

**Why Structural.** The record fixes the names and meanings of five metrics
that cross `analyze-store` v1, which store and M6 will read, and adds a
value of `unit`. It needs nothing Constitutional: no anchor, no gate, no
protected path and no GOALS.md scope moves.

### For the decider's judgement

- **The copy is an archive's capture.** Whether the Internet Archive's
  capture of the author's own page, named above by URL, date and digest, is
  a copy "the worker can read from the run" under the sources record.
- **Page 9's specification over page 13's.**
- **X5 left a term,** on one sentence of the open-questions record.
- **The preferred claim left out,** against the copy's "preferred and
  common".
- **No cut-off in analyze,** beside the analyze record's "Altman's
  coefficients and cut-offs".
- **A total below zero as a condition,** where `liabilities_to_assets` gives
  a signed figure.
- **Every `operating` filer scored** by a function fitted on manufacturers.

## Alternatives

- **Page 13's specification,** `1.2`, `1.4`, `3.3`, `0.6` and `1.0` over
  fractions. It is the form most readers know, and it needs no hundred.
  Rejected: the copy calls page 9's the final function, says of page 13's
  that its last coefficient is rounded, and names the number rounded as
  `0.99` where the function prints `0.999`. The scores differ by a
  thousandth of X5, against the copy's "remain identical". This is the item
  most worth the decider's judgement beside the fifth ratio.
- **`1.2`, `1.4`, `3.3`, `0.6` and `0.999` as the constants,** over
  fractions. It gives the same figure as page 9's and needs no hundred.
  Rejected: no line of the copy prints those five together. Each of the
  first four would cite a line that prints another number.
- **A coefficient as a numerator and a denominator,** 12 and 1000. Exact.
  Rejected: neither number is printed, and the characters `0.012` are. Read
  through the one reader they are the same exact number.
- **The score alone, and no ratio stored.** Five fewer names. Rejected: a
  user with no key would see nothing of the method, M6 could not screen on a
  ratio without recomputing it, and a `reit` would lose three ratios its
  statements give.
- **All five ratios stored.** The score could then be re-added from the
  table. Not rejected on its merits, and not here: a stored X5 is computed
  for a `bank` and an `insurer`, against "Altman's X₅ is unreached for both
  kinds." It is a record of its own.
- **X5 stored and X4 not,** so that every stored ratio stands without a
  price. Rejected: it meets the same sentence, and X4 is one of the four the
  copy says discriminate alone.
- **A metric that states the score's side of a cut-off.** A user would see
  "below 1.81" without composing a screen. Rejected here: it changes no
  number analyze computes, which the analyze record gives to M6. The copy
  prints two numbers for one cut-off and no upper bound of its zone, so a
  zone metric would need a number the copy does not print. A later record
  can add a metric for the one bound the author advocates, `1.81`, beside
  the score.
- **The preferred claim added at its carrying amount.** It would follow page
  11's "preferred and common" more nearly. Rejected: it sets a carrying
  amount where the copy says market value, it reads a concept whose silence
  is `Unknown` and so absents most filers, and it makes the score a consumer
  of `preferred_equity` the open-questions record did not admit.
- **The preferred claim counted where it is a `Value`, and nothing where it
  is `Unknown`.** Rejected: it reads an `Unknown` as a zero, which the
  accepted records forbid.
- **Total debt as X4's denominator,** `short_term_debt` plus
  `long_term_debt`, by the period record's row. Rejected: the copy defines
  the term as total liabilities, "both current and long term", and the
  concepts record names `total_liabilities` for it. The row fixes an
  instant and no concept.
- **Z" for every `operating` filer.** It reads no price, so it serves a user
  with no key, and the copy fits it for non-manufacturers. Rejected here:
  every accepted record names the fourth ratio as a reader of the market
  value of equity, and GOALS.md's criterion is met by the score those
  records name. Z" is another metric, left to a record of its own.
- **A book value of equity standing in where there is no price.** The score
  would never be absent. Rejected: the copy calls such adjustments "not
  scientifically valid", the analyze record forbids a stand-in, and a score
  under one name would mean two things.
- **Leaving the score out of M5** until the paper of 1968 opens, or the
  author's live page answers, or the provider is built. Rejected: the open
  primary is the author's own restatement, read whole at a digest 63
  captures share over twenty years. A test builds the crossing as the
  contract type, so each figure and each `no_price` is provable today. If
  the decider judges the capture no readable copy, this is the alternative
  that stands.
- **A `total_assets` below zero as a signed figure,** as
  `liabilities_to_assets` gives. Rejected: it turns three deficits into
  surpluses inside one score, and nothing would mark it.
- **One condition for a zero and a negative total,** as
  `non_positive_total_assets`. Rejected: `zero_total_assets` is already
  defined, and the accepted records keep the zero divisor and the sign as
  two conditions.
- **Working capital formed inside X1's derivation.** It has one reader
  today. Rejected: the concepts record names it as a derived value, and a
  second reader would form it a second time.
- **X1 and X2 stated at an instant,** since they read balances alone. They
  would be computed at every quarter's balance sheet date. Rejected: the
  series record states every term of the score over a year, and a quarter's
  ratio beside a year's score would not be the score's term.
- **The count averaged, or the close at the period end.** Rejected: the
  period record reads X4's balance at closing, and the provider record lists
  X4 at the valuation date.
- **`Multiple` or `FractionOfOne` for the score.** No new value of `unit`.
  Rejected: a score is no multiple of an amount, and a fraction of one may
  be shown as a percentage.
- **Names by the copy's numbers,** `altman_x1` to `altman_x4`. Shorter, and
  the source's own. Rejected: the copy misnumbers two of them, and a name in
  words cannot be misread that way.
- **A score over the ratios a kind has,** four for a `reit`. Rejected: the
  copy states no term that contributes nothing, and a `reit`'s score over
  four would read as one figure with an `operating` filer's over five.
- **Each ratio rounded to a tenth of a per cent,** as Table 1 prints its
  means. Rejected: no page says the function's inputs were rounded, and the
  arithmetic record rounds once, when a figure is written.

## Consequences

**Easier:**

- The bankruptcy score GOALS.md names has its specification, constants,
  periods, conditions, names and source. Each implementing task has only its
  code left to decide.
- Three ratios of the method stand for a user with no key, and for M6's
  composite without a price.
- Working capital is formed once before a second reader arrives.
- A user with no key sees the score absent with `no_price`, and every other
  metric computed.

**Harder:**

- **The score needs eight concepts and a price.** Every one reads silence as
  `Unknown`, so a filer silent on `retained_earnings` or on a current
  subtotal has no score.
- **Every `reit`, bank, insurer and investment company has no score.**
- **The score is absent in every screen over the corpus,** which asks no
  price.
- **The score cannot be re-added from the stored table,** X5 being no
  metric.
- **`constants/mod.rs` gains a line,** which orders the task that adds the
  constants file among those that touch that file.
- **M6 holds the cut-offs.** Where a screen's threshold stands and how it
  cites this copy is M6's to record.

**Expensive to reverse:**

- The five names and the specification, once results are stored and ranked
  on. Page 13's form later would move every stored score by a thousandth of
  X5, under a bump.
- What X4 counts. Adding the preferred claim later raises every stored X4
  and score for the filers that carry it, under a bump.
- A total below zero as a condition. Turning it into a figure later makes
  stored absences into numbers, under a bump.

**Open, and not this record's:**

- Whether the fifth ratio is a metric.
- Z", and a metric for the score's side of `1.81`.
- A reading of the preferred claim at market value, which v2 cannot carry.
- Where M6 holds a cut-off.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **Every constant named, defined once and cited.** `literals` keeps every
  number but zero and one out of the derivation. The six constants stand in
  `constants/altman.rs`, each under its page and line.
  - **Gap:** nothing checks that a citation is true. The proof's figures
    hold the six values, and not their source.
  - **Gap:** `literals` does not read a string's contents. A coefficient
    written as characters in the derivation would pass it. Only review sees
    one outside `constants/`.
- **The specification.** The proof holds the score to `3.2145`, against
  `3.216` and `1.51566`.
- **One definition of each term.** The proof's figures hold the four ratio
  metrics and the score to the one module, and the pinned figures of the
  four metrics named above hold market capitalisation and
  `zero_total_assets` to theirs.
  - **Gap:** nothing forces a metric through a module rather than around it,
    as the market capitalisation record says of its own.
- **One definition of each condition.** Each new condition is a `const`
  beside the ratios' module, and each reached one is the existing `const`.
  - **Gap:** nothing stops a second `const` with the same characters, except
    review.
- **The preferred claim is not read.** The catalogue test holds
  `preferred_equity` out of every entry's `inputs`, and the test with a
  `preferred_equity` of 500 holds the figures.
- **No wrong number across a split.** The share-basis tests hold it for X4
  and the score at E.
- **No score without a price,** and none over fewer than five ratios. The
  witness tests and the tests by kind hold both.
- **No cut-off in analyze.**
  - **Gap:** nothing reads for one. A constant added to `constants/altman.rs`
    without a record is seen by review alone.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
