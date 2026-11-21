# The dividend yield is a year's dividends declared per common share over the unadjusted close at the valuation date, stated at each year beside that one close; a year that declared nothing yields zero, and a negative dividend or a close of zero or below declines

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-11-20, by `M5-67`
- **Decided:** 2026-11-20, by the decider
- **Touches:** the yield that GOALS.md's fifth M5 criterion names in "yield
  and relative yield", and the yield alone. It fixes the name and meaning of
  one metric, which cross `analyze-store` v1 as characters, and no byte of
  that contract changes. It reaches two existing `declined` conditions from
  their one definitions, and adds no condition and no reason. It reads the
  price as the analyze record and `share-basis-rule-dates.md` (Accepted) fix,
  through the one function `M5-54` built. It takes one operation from the
  arithmetic record's table. It reads no setting and puts no number in
  `constants/`. Relative yield is left to a record of its own. It names, and
  leaves open, one question about an accepted record: the chapter's own
  illustration sets each year's dividend against that year's closing price,
  which the provider record keeps from this metric. No accepted record is
  superseded. No contract's bytes, no schema, no edge, no gate and no
  dependency move. Nothing here is implemented.

## Context

GOALS.md's fifth M5 criterion opens its list of dividend metrics with "yield
and relative yield". Every record around the yield is accepted, and none
forms it. Its period, its conditions, its name and its source have no record.

Some things are already fixed, and this record restates them as fixed:

- **The concept.** `canonical-concepts.md` (Accepted) names
  `dividends_declared_per_share`: "dividends declared per common share for
  the period, in the reporting currency. A flow." It is consumed by "yield
  and relative yield; dividend growth rate and streak; the dividend discount
  valuation; payout and coverage in their per-share form."
- **It is the total.** `canonical-concepts-open-questions.md` (Accepted)
  reads the concept as "the total — dividends declared per common share for
  the period, regular and special together. It does not split." Among its
  movers it names, "for filers that declare a special only: yield and
  relative yield".
- **v2.** `contracts/canonical-concepts/v2.toml` gives the concept measure
  `flow`, unit `currency_per_share` and sign `as_presented`. It applies to
  all five kinds. Its silence is `conditional`: zero when "the period's facts
  are silent on this concept and on dividends_paid alike", unknown when
  "dividends_paid resolves to a non-zero value".
- **The provider record.** `price-provider.md` (Accepted):
  - lists under the valuation date "yield, through
    `dividends_declared_per_share`";
  - says "`divCash` goes unread, because yield reads the dividend from the
    filing.";
  - fixes the close as unadjusted, and says of the dividend adjustment "It
    lowers each past close by the dividends paid after it, so a yield on it
    is overstated and a multiple understated.";
  - lists the second kind of date as "At each past fiscal period end the
    history holds, for relative yield only.", and says "This reads relative
    yield as a company's current yield against its own past yields (Weiss and
    Lowe, *Dividends Don't Lie*, 1988)."
- **The common-shareholder record.** `common-shareholder-forms.md`
  (Accepted) lists "yield and relative yield" among the metrics "Common by
  the concept they read, with no term to form", of which "Each reads a
  per-common-share concept and needs nothing here." It fixes the naming rule:
  `per_share` "appears in the name of a form that reads per-common-share
  concepts", and `common` appears "exactly when its derivation forms common
  equity or income available to common".
- **The analyze record.** `what-analyze-takes-and-returns.md` (Accepted):
  - a price-dependent metric "reads the valuation date's answer from the
    crossing's answer at the valuation date";
  - the share-basis rule governs a figure "whose unit is
    `currency_per_share` or `shares`";
  - "A per-share metric at the valuation date pairs a figure at an earlier
    period end with the valuation date's close.";
  - of the rule's cost, "For a filer that split recently, P/E, yield and
    market capitalisation are absent until a period ending after the split is
    the latest."
- **The share-basis dates record.** `share-basis-rule-dates.md` (Accepted)
  says "For a metric stated at a duration, as both discounted cash flow
  metrics are, E is the duration's end". Every metric that calls the rule
  lists `share_basis_date_not_a_date` after the four year conditions.
- **The series record.** `which-duration-precedes-a-duration.md` (Accepted):
  "A metric is stated over a year when its source states it over a fiscal
  year." At a duration that is not a year, such a metric gives exactly one of
  `start_not_a_date`, `end_not_a_date`, `not_a_year` and
  `overlaps_another_year`, and makes no read.
- **The discounted cash flow record.**
  `discounted-cash-flow-and-margin-of-safety.md` (Accepted) states its margin
  "At each year Y where the value per share is stated, beside the close at
  the valuation date", with "Nothing here orders the years or picks the
  latest." It orders the price read's reasons: no crossing, an absence
  answered, the rule, then the close's grammar. Its `non_positive_close` is
  defined once, beside that margin.
  `market-capitalisation-and-the-equity-multiples.md` (Accepted) reaches it
  from there for each of its four multiples.
- **The negative dividend.** `negative_dividends_declared_per_share` is
  defined once, beside the streak: "a year read declared a negative dividend
  per common share". `dividend-growth-rate.md` and
  `dividend-discount-valuation-and-margin-of-safety.md` (both Accepted) reach
  that one definition.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written,
  and a table of operations. Division is "Exact whenever the divisor is not
  zero."
- **The price read is built.** `M5-54` wrote
  `crates/analyze/src/metrics/reading/price.rs`: the share-basis rule as one
  function, and the reading of the close at the valuation date as one
  function, giving the close or exactly one reason. The two margins and
  price/earnings read through it on main.
- **Absence is closed.** Six reasons.
- **The entry shape.** M5-18 fixed it: name, meaning, `unit`, `stated_at`,
  inputs in reason order, and the metric's own `declined` conditions. `Unit`
  carries `FractionOfOne`, `Multiple`, `Years`, `CurrencyPerShare` and
  `Signals`.

## Decision

### One metric

| Name | What it is | Unit | Stated at |
| :--- | :--- | :--- | :--- |
| `dividend_yield_per_share` | A year's dividends declared per common share over the close at the valuation date | `FractionOfOne` | a year |

It reads a price. **It reads no setting.** The form is the source's, and it
has no free parameter: the dividend is the filing's, and the close is the
one the caller's valuation date names. The valuation date is carried by the
crossing and is no setting, as the analyze record decided. Nothing is added
under `settings/` or `constants/`.

### The derivation, in formula order

> yield = D ÷ P

D is `dividends_declared_per_share` over the year Y. P is the close at the
valuation date.

| Step | Operation, as the table names it |
| :--- | :--- |
| D ÷ P | division. P is above zero wherever it is made, since `non_positive_close` is asked before it. So no zero divisor arises. |

Sign, from the same table, asks the two conditions. No other operation is
taken: no root, no power, no mean, and no literal.

- **The form is the chapter's.** Page 3: "Dividend Yield = Annual Dividends
  per Share/Price per Share". Page 4's margin note: "Dividend Yield: The
  dollar dividend per share divided by the current price per share."
- **The dividend is the filing's, and never the provider's.** The provider
  record fixes it: "`divCash` goes unread, because yield reads the dividend
  from the filing." D is read from the history, and from nowhere else.
- **The close is the unadjusted close,** read through the one function
  `M5-54` built in `crates/analyze/src/metrics/reading/price.rs`. The yield
  reads the crossing nowhere else.
- **D is the total,** regular and special together, as v2 carries it.
- **Exact, and rounded once.** The quotient is held exactly and rounded when
  it is written, as the arithmetic record says of every figure.

### Which periods

**The yield is stated over a year,** under the series record's test.

- The chapter's form names "Annual Dividends per Share". That is a year's
  dividend, and the series record's test asks what the source states the
  metric over.
- **What the source does not fix** is which twelve months. This is the
  record's own decision: the fiscal year, because that is the year a filing
  states the figure over, and the one a `Value` in the history carries. The
  chapter's illustration on page 16 is by year too: "For each year we will
  also compute the dividend yield and dividend payout ratio for each firm."
- A quarter's dividend over the same close is about a quarter of a year's
  yield, and nothing would mark it. The chapter's page 2: "Firms in the
  United States generally pay dividends every quarter". So a quarter's figure
  is one declaration of the four a year's yield counts.
- A quarter can also read zero by silence inside a year that paid, as the
  dividend discount record says of the suspension fixture. A yield of `0`
  there would be a figure for a filer that paid.

At a duration that is not a year, the yield gives exactly one `declined`
reason, the first of `start_not_a_date`, `end_not_a_date`, `not_a_year` and
`overlaps_another_year`, and makes no read, as the series record says. Its
`stated_at` stays `Duration`.

### The share-basis rule

D is a figure in currency per share, so the rule governs it. The yield calls
the rule through the one function, once.

- **E is Y's end,** the duration's end, as the share-basis record fixes for a
  metric stated at a duration. T is the close's trading date.
- **E never reaches the rule outside the grammar.** The year test asks first
  and gives `end_not_a_date`, as the share-basis record says of a metric
  stated over a year. A trading date or a split's date outside it gives
  `share_basis_date_not_a_date`.
- **What the caller must ask.** The yield at a year ending E needs a price
  answered at or before E for the covered range to reach it. Asking every
  period end the history holds does that, as the analyze record says. A
  caller that asked less gets `not_covered`, never a number.
- **A split after Y's end** makes the yield absent with `split_within`, even
  where a later filing restated D. That is the cost the analyze record
  already names for the yield.

### Where it is stated

At each year it is computed at, beside the close at the valuation date, as
the discounted cash flow record places its margin.

- The analyze record fixes that the valuation date's answer is read from the
  crossing's answer at the valuation date.
- The provider record lists the yield under the valuation date, and the past
  period ends "for relative yield only".
- The chapter's margin note sets the dividend over "the current price per
  share".
- So a past year's yield is that year's dividend against today's close.
  Which year a ranking reads is M6's query over the stored table. Nothing
  here orders the years or picks the latest.

**What the chapter's illustration does, and this record does not.** Page
16's table sets each year's "Dividends per share" over that year's "Stock
price at end of year". For one firm it reads $0.35 over $32.28 as 1.08% for
2007, and $0.35 over $22.69 as 1.54% for 2008. That is a past year's yield
at that year's own close. This record forms no such figure:

- It reads a past period end's close. The provider record lists those "for
  relative yield only", so the yield is not a consumer of them.
- For the latest year and a valuation date near its end, the two figures are
  close. For an older year they are not, and nothing marks the difference
  (see What no check can see).
- Whether any metric states a yield at a past close is left open, to the
  record on relative yield or to one of its own. Nothing here would have to
  be undone for it: it would be another figure, under another name.

### The `declined` conditions

Each is asked only where the read and the price read both give a number.
They are asked in formula order, the dividend and then the close, and the
first that holds is the one reason. Each is a sign or zero test and needs no
literal.

| Condition | Holds when | Defined |
| :--- | :--- | :--- |
| `negative_dividends_declared_per_share` | D over Y is below zero | once, beside the streak; reached from there |
| `non_positive_close` | the close is zero or below | once, beside the discounted cash flow margin; reached from there |

Both constants stand on main where a sibling metric reaches them. Neither is
defined a second time, and no visibility widens.

**A negative dividend is a condition, and not a signed figure.** No source
opened states the yield at a negative dividend. This is the record's own
decision:

- v2 gives the concept sign `as_presented`, so a negative figure can cross.
- The streak, the growth rate and the dividend discount value each decline
  at the same figure under this condition. A yield stored as a figure beside
  them would treat one input as usable in one column and undefined in three.
- The condition's words fit as they stand: the yield is stated over a year,
  and "a year read declared a negative dividend per common share".
- A negative yield is no share of a return. A small negative fraction in a
  column of fractions passes a glance. An absence that names the sign does
  not.

**A dividend of zero gives a yield of zero. It is a figure, and no
condition.**

- The chapter's page 4: "In both time periods, almost 65% of the overall
  sample of 7200 companies paid no dividends, making zero the median dividend
  yield across all companies." The source counts a non-payer's yield as zero.
- Zero over a close above zero is defined, and it is true: a share that
  declared nothing yielded nothing.
- A wrong silence zero gives `0`, the lowest yield a payer can show. That is
  the unflattering direction, as the growth rate's `-1` is.
- The same year shows `0` in the streak and `-1` in the growth rate, and the
  three agree.
- So `zero_dividends_declared_per_share` is not reached and not listed. The
  dividend discount value declines at a zero because a value of zero is no
  valuation. A yield of zero is a yield.

**A close of zero or below.** A zero close is a zero divisor. A negative
close is no price paid, and would turn a real dividend into a negative yield.
A close near zero gives the largest yield the form can give, so the test
sits on the flattering side. It is `non_positive_close`, reached from its one
definition, for both.

**The first that holds.** A negative dividend beside a close of zero gives
`negative_dividends_declared_per_share` alone. A dividend of zero beside a
close of zero gives `non_positive_close`, and never `0`.

The entry also lists the four year conditions, then
`share_basis_date_not_a_date`, as the share-basis record places it.
`input_not_a_decimal` reaches it from the one definition beside the reader.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first.** At a duration that is not a year, its one reason
   is the outcome and nothing below is asked.
2. **The read.** `dividends_declared_per_share` at Y, its own row. It gives a
   number or one reason: `input_unknown` or `input_not_applicable`, naming
   the concept and Y, or `declined` with `input_not_a_decimal`. No other row
   is read, so `no_period` does not arise.
3. **The price read, last,** made whatever the read gave, through the one
   function, with E the end of Y. It gives a number or exactly one reason, in
   the discounted cash flow record's order:
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
   `date_not_asked` has no witness here, as the discounted cash flow record
   says.
4. **The two conditions,** in the order above, only where the read and the
   price read both gave a number.

Both reads are made, and neither stops the other. Where both fail, the
read's reason stands first and the price read's follows.

**Never a fall-back.** The yield never reads another date's price, an
adjusted close, the provider's dividend or a stand-in. It never forms D from
`dividends_paid` and a share count.

**With no key, no number.** The yield reads a price. The caller with no key
asks nothing, so the crossing is absent, and the yield is absent with
`no_price` and `not_asked`, after any reason of its own read. That is
GOALS.md's ninth M5 criterion: "A metric with no price data is absent with a
reason. It never fails the run and never shows a wrong number."

No seventh reason is needed, and no frozen byte changes.

### By kind

`dividends_declared_per_share` applies to every kind, so no kind is absent by
applicability.

| Kind | Where the read is a `Value` |
| :--- | :--- |
| `operating`, `bank`, `insurer`, `reit`, `investment_company` | computed |

**The two arms of the conditional silence:**

- **Zero,** where the period's facts are silent on both dividend concepts.
  The read is a `Value` of zero, and the yield is `0`.
- **`Unknown`,** where `dividends_paid` resolves to a non-zero value and the
  per-share figure does not resolve. The yield is absent with
  `input_unknown`, naming `dividends_declared_per_share` at Y. It does not
  read `dividends_paid`.

The kind does not cross v2, and the yield does not need it.

### The name

`dividend_yield_per_share`. It crosses `analyze-store` v1 as characters,
under the common-shareholder record's naming rule.

- **It carries `per_share`.** It reads `dividends_declared_per_share`, a
  per-common-share concept, and the rule puts `per_share` in "the name of a
  form that reads per-common-share concepts". The market capitalisation
  record read the rule the same way for `price_to_earnings_per_share`, the
  other ratio of the close and one per-share figure. Its `meaning` says "per
  common share".
- **It does not carry `common`.** It forms neither common equity nor income
  available to common.
- **The bare name stays free.** A total form, dividends paid over market
  capitalisation, is argued under Alternatives and not taken. If a later
  record takes it, the rule gives it neither word, and the two entries cannot
  be taken for each other, as `dividend_payout_on_earnings` and
  `dividend_payout_per_share` cannot.
- `dividend_growth_rate` and `dividend_streak` read the same concept and
  carry no `per_share`. Their records fixed those names, and this record
  changes neither.

### What its catalogue entry says

Nothing beyond the shape M5-18 fixed, and no new value of `unit`:

- **`name`:** `dividend_yield_per_share`.
- **`meaning`:** "The dividends declared per common share for a year,
  regular and special together, over the unadjusted close at the valuation
  date."
- **`unit`:** `FractionOfOne`. A yield is a fraction of one, so two and a
  half percent is `0.025` and never `2.5`. Whether the shell shows a
  percentage is the shell boundary's, as the arithmetic record says.
- **`stated_at`:** `Duration`.
- **`inputs`:** `dividends_declared_per_share`, once. The price is no concept
  and is not listed.
- **`conditions`,** in the order asked:
  - `negative_dividends_declared_per_share`
  - `non_positive_close`
  - `start_not_a_date`, `end_not_a_date`, `not_a_year` and
    `overlaps_another_year`, by their constants
  - `share_basis_date_not_a_date`
  - then `input_not_a_decimal`, generated from its one definition

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a metric's formula, which periods or dates it
reads, and a rule of absence:

- the form, the filing's dividend over the unadjusted close
- the year it is stated over, and the valuation date's close beside each year
- E, and the rule asked once
- the name and its meaning
- the two conditions and their order, and a zero dividend as a figure

A later change to any of them, the form, the period or a condition included,
is a version bump and a superseding record, and never an edit to this one.

It lands at version 1. The yield is not computed yet, so no stored outcome
changes, and adding a metric is not a bump.

### Source

Under `sources-a-task-can-read.md`. Each copy below was opened in this run,
and each quotation is as printed.

**The origin.** Damodaran, A., *Applied Corporate Finance: A User's Manual*,
3rd ed., John Wiley and Sons, chapter 10, "Dividend Policy".

- **The copy opened.** The author's own copy of the chapter,
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/acf3E/book/ch10.pdf>, 27
  sheets, 4,678,827 bytes, sha256
  `48ee94e4a9f200d5afc5ae51a0d9c876068c715aebd3992360ca136b648117d6`, the
  digest the task names. It was downloaded whole in this run.
- **The book.** The file prints "CHAPTER 10" and "DIVIDEND POLICY". It does
  not name the book it is a chapter of, nor that book's edition or year. What
  ties it to the book is its path on the author's site, and the author's page
  for that edition,
  <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/ACF3E/appldCF3E.htm>,
  opened in this run. It reads: "This web site is designed to support the
  third edition of "Applied Corporate Finance: A User's Manual". The
  publisher is John Wiley and Sons." The server closed that transfer early on
  each of two tries and serves no byte ranges, so the page was read in part,
  with those sentences in the part that arrived. No year of publication was
  read, and none is written here.
- **How it was read.** Each sheet carries two printed pages side by side,
  each printing its own number, 1 to 54. Printed page 3 is the left half of
  sheet 2 and page 4 its right half. Page 2 is the right half of sheet 1, and
  page 16 the right half of sheet 8. The text was extracted with
  `pdfminer.six`, taking each sheet's text boxes by the half they stand in.
  Each quotation was checked with whitespace collapsed and is written with
  the spacing the page prints. Page 16's table was read from a rendering of
  the page, since its cells extract out of order.

**What it says.**

- **Page 2:** "Firms in the United States generally pay dividends every
  quarter, whereas firms in other countries typically pay dividends on a
  semi-annual or annual basis." And: "Dividends in publicly traded firms are
  usually set by the board of directors and paid out to stockholders a few
  weeks later."
- **Page 3:** "We generally measure the dividends paid by a firm using one of
  two measures. The first is the dividend yield, which relates the dividend
  paid to the price of the stock:", followed by the line "Dividend Yield =
  Annual Dividends per Share/Price per Share".
- **Page 3:** "The dividend yield is significant because it provides a
  measure of that component of the total return that comes from dividends,
  with the balance coming from price appreciation."
- **Page 3:** "Some investors also use the dividend yield as a measure of
  risk and as an investment screen, that is, they invest in stocks with high
  dividend yields."
- **Page 3,** cited under What no check can see: "Second, the dividend can be
  a regular dividend, which is paid at regular intervals (quarterly,
  semi-annually, or annually), or a special dividend, which is paid in
  addition to the regular dividend."
- **Page 4,** the margin note, which the copy breaks over three lines that
  the extraction interleaves with the body's: "Dividend Yield: The dollar
  dividend per", "share divided by the current price per" and "share."
- **Page 4,** cited for a zero dividend giving a yield of zero: "In both time
  periods, almost 65% of the overall sample of 7200 companies paid no
  dividends, making zero the median dividend yield across all companies."
- **Page 16,** "Illustration 10.1 Dividends, Dividend Yields, and Payout
  Ratios": "For each year we will also compute the dividend yield and
  dividend payout ratio for each firm." Its table's rows are "Dividends per
  share", "Earnings per share", "Stock price at end of year", "Dividend
  Yield" and "Dividend Payout". Its first firm's columns read $0.35, $32.28
  and 1.08% for 2007, and $0.35, $22.69 and 1.54% for 2008. Its footnote 7:
  "The dividends for these years are actually paid in the subsequent years by
  these companies." Of a second firm: "The dividend policies are different
  for the two classes with preferred shares getting higher dividends."

**Where no source fixes a period or a condition,** it is this record's own
decision, with its reason above:

- the fiscal year as the chapter's "Annual"
- the declared figure where the chapter's page 3 says "the dividend paid".
  The catalogue and the provider record name the concept, and footnote 7
  shows the chapter itself counting a year's dividend before it is paid.
- a negative dividend as a condition
- the order of the two conditions

The close at the valuation date is the provider record's, and the margin
note's "current price". Page 16's table pairs each year with its own
year-end price, as stated above. No page opened states the form otherwise
than the task quotes it.

### The proof the implementing task brings

These tests are built through `History::of` and `Row::at`, and a `Crossing`
built as the contract type. They read no file. Each stands in `tests/` or
under `#[cfg(test)]`.

**The worked history.** The year 2024-01-01 to 2024-12-31, with
`dividends_declared_per_share` 1.5.

**The preset crossing** is the share-basis record's: a valuation date of
2025-06-30 answered with a close of 60 traded 2025-06-30, the earlier date
2024-12-31 answered with a close traded 2024-12-31, and an empty split list.
E is 2024-12-31 unless a test says otherwise.

At the least:

- **The figure, worked by hand.** 1.5 ÷ 60 gives `0.025`.
- **A quotient that does not terminate.** A dividend of 2: 2 ÷ 60 is one
  thirtieth, written `0.0` followed by thirty-four `3`s.
- **A past year beside today's close.** The year 2023 with a dividend of
  1.2, and a further earlier date 2023-12-31 answered with a close traded
  2023-12-29: 2023 gives `0.02` and 2024 `0.025`, each over the close of 60.
  The close traded 2023-12-29 enters no figure.
- **The earlier year, not asked.** The year 2023 beside the preset crossing:
  `basis_unproven` with `not_covered`, naming 2024-12-31 through 2025-06-30,
  2023-12-31 and 2025-06-30.
- **A year declaring nothing.** A dividend of 0: the figure `0`, and no
  condition.
- **A negative figure.** A dividend of -1.5:
  `negative_dividends_declared_per_share`, and never `-0.025`.
- **The close.** A close of 0, and of -1: `non_positive_close`.
- **The first that holds.**
  - A dividend of -1.5 beside a close of 0:
    `negative_dividends_declared_per_share` alone.
  - A dividend of 0 beside a close of 0: `non_positive_close`, and never `0`.
  - A dividend of -1.5 and no crossing: `no_price` with `not_asked` alone,
    since a condition is asked only where both reads gave a number.
- **No price, by each witness.**
  - No crossing: `no_price` with `not_asked`.
  - The valuation date answered with an absence: `no_price` with
    `answered_absent`, carrying 2025-06-30.
- **The share-basis rule, by each clause.**
  - The split list absent: `splits_absent`.
  - The valuation date alone, no earlier date answered: `not_covered`, naming
    2025-06-30 through 2025-06-30, 2024-12-31 and 2025-06-30.
  - A split dated 2025-03-14, after the year's end and on or before the
    close's trading date: `split_within`, naming it, 2024-12-31 and
    2025-06-30. It is never `0.025`, which would set a dividend on one basis
    over a close on another.
  - A split dated 2024-12-31: `split_within`. A split dated 2024-12-30:
    `0.025`.
- **A date outside the grammar.** The paired close's trading date
  `2025-6-30`: `declined` with `share_basis_date_not_a_date`. A year ending
  `2024-12-1`: `end_not_a_date`, from the year test, and never the rule's
  condition.
- **Unreadable characters.** A close of `1e2`: `declined` with
  `input_not_a_decimal`. A dividend of `1e2`: the same, from the read.
- **The conditional silence.** The dividend `Unknown` beside `dividends_paid`
  300: `input_unknown`, naming `dividends_declared_per_share` and 2024,
  alone, and never a figure formed from `dividends_paid`.
- **A filer of each kind.** For each of the five kinds, the worked history
  with every concept v2 does not apply to that kind `NotApplicable` gives
  `0.025`.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31, with the dividend
  `Unknown` and no crossing: `not_a_year` alone. With a dividend of 0.375
  beside the preset crossing: `not_a_year`, and never `0.00625`.
- **The order of reasons.** The dividend `Unknown` and no crossing:
  `input_unknown` naming the concept and 2024, then `no_price` with
  `not_asked`. The dividend `1e2` beside the valuation date answered with an
  absence: `input_not_a_decimal`, then `answered_absent`.
- **The catalogue.** The entry's name, meaning, unit, `Duration`, one input
  and conditions are as listed above, with `input_not_a_decimal` last. Its
  name holds `per_share` and not `common`. Its inputs hold neither
  `dividends_paid` nor `shares_outstanding`. Its conditions do not hold
  `zero_dividends_declared_per_share`.

### What no check can see

- **Whether the chapter says what this record says.** The copy was opened in
  this run and is quoted as printed. The implementing task cites it as the
  sources record says. A copy that on reading says otherwise is a stop for
  that task, not a choice.
- **The figure is a fiscal year's declared dividend,** and not the dividend
  the next year will pay. A cut announced after Y's end is not in it. Nor is
  it the dividend paid in Y: the chapter's page 2 puts payment "a few weeks
  later" than the board's decision, so a declaration near the year's end is
  paid in the next.
- **A special dividend.** The chapter's page 3 names "a special dividend,
  which is paid in addition to the regular dividend". D is the total, so a
  special lifts one year's yield and falls out of the next, as the
  open-questions record says of the total. Nothing in the history marks it,
  and the lifted yield looks like any other. That is the flattering
  direction for a screen that seeks high yields.
- **The age of a past year's yield.** It is set beside today's close, and
  nothing marks its age. It is not the yield a buyer had that year, which
  page 16's table states at that year's own close. Where Y ends after the
  valuation date, the yield is `not_covered`, as the share-basis record says.
- **The class of a share.** The rule covers the dates of a figure and a
  close, and not the class of the share. D is per common share, and the close
  is one ticker's. Page 16 names a firm whose two classes are paid
  differently. Where they differ, the yield sets one class's dividend over
  another's price, and nothing in v2 or the crossing names the class.
- **A high yield from a fallen price.** The form cannot tell a generous
  dividend from a collapsed close. The streak, the growth rate and coverage
  are the metrics that say which.
- **The currency.** The close's currency and the filer's reporting currency
  are not compared, which `price-provider.md` already leaves open.
- **A wrong silence zero.** It gives `0`, a non-payer's yield, for a filer
  that paid. It does not flatter.
- **A metric that reads `divCash`, or forms the dividend itself.** Only
  review sees it.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  derivation needs no literal: it is one division and two sign tests. No
  constant and no setting is added.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule and catalogue
- the share-basis record's comparisons and condition, and its E for a metric
  stated at a duration
- the arithmetic record's reading, operations and written form
- the series record's year test
- the common-shareholder record's naming rule
- the dividend growth rate and dividend discount records' metrics, settings
  and conditions, `negative_dividends_declared_per_share` reached and not
  moved
- the discounted cash flow record's margin, its `non_positive_close` reached
  and not moved
- the provider record's ask and answer, its past period ends among them
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains no value of `unit` and no field

No sentence of an accepted record is superseded. One question would need one
to change, and it is left open and named:

- **Whether a year's yield is ever stated at that year's own close,** as
  page 16's table states it. The provider record's "for relative yield only"
  keeps the past period ends from the yield. A second consumer of them is a
  record of its own.

**Why Structural.** The record fixes the name and meaning of a metric that
crosses `analyze-store` v1, which store and M6 will read, and a formula under
the method version. It needs nothing Constitutional: no anchor, no gate, no
protected path and no GOALS.md scope moves.

## Alternatives

- **The yield at each year against that year's own close,** as relative
  yield reads past closes, and as the chapter's page 16 computes it. It makes
  a past year's figure the yield the market offered then. Not rejected on its
  merits: the provider record lists the yield under the valuation date and
  the past period ends "for relative yield only", so reading them here needs
  that sentence changed, which is a record of its own. Against today's close,
  every year answers one question, what this dividend is of today's price,
  and the latest year's answer is the one the margin note defines.
- **A trailing yield over the last four quarters.** It is nearer the
  valuation date than the last fiscal year. Rejected here: it adds four
  quarters, which no record forms, and the history holds year-to-date spans
  and transition stubs beside them. A record that forms trailing spans can
  add it as a second metric, as the market capitalisation record says of a
  trailing P/E.
- **The latest quarter's dividend times four.** Rejected: four is a number
  the derivation cannot cite from this source, the product is a forecast and
  not a figure in a filing, and a quarter holding a special would be read as
  four of them.
- **A total form, `dividends_paid` over market capitalisation.** Rejected:
  - The catalogue and the provider record name
    `dividends_declared_per_share` for the yield, and the chapter's form is
    per share.
  - v2's `dividends_paid` is "for all classes of shareholder — common and
    preferred together". Over a common claim it overstates the yield for a
    filer with preferred outstanding, which flatters.
  - It reads `shares_outstanding` beside a flow, so it meets the two
    questions the market capitalisation record leaves open.
- **The provider's `divCash` in place of the filing's figure.** Rejected by
  the provider record: "`divCash` goes unread, because yield reads the
  dividend from the filing." It would put a second source behind one figure,
  and make the dividend absent with no key.
- **A yield on the adjusted close.** Rejected by the provider record: the
  dividend adjustment "lowers each past close by the dividends paid after it,
  so a yield on it is overstated".
- **Relative yield in this record.** Rejected: no copy a run can open states
  it. The provider record's sentence is all that is accepted of it, and the
  book that sentence names is one no run can open. Its form would be set on
  the yield before the yield is fixed.
- **A negative dividend as a signed yield.** The arithmetic gives it.
  Rejected: three accepted metrics decline at the same figure under one
  condition, and a figure here would contradict them at the same row.
- **A zero dividend as a condition,** under
  `zero_dividends_declared_per_share`, as the dividend discount value
  declines. Rejected: the chapter's page 4 counts a non-payer's yield as
  zero, the quotient is defined, and an absence would hide a suspension that
  the streak shows as `0`.
- **The yield at every duration.** Rejected: a quarter's yield is about a
  quarter of a year's with nothing to mark it, and the chapter's form says
  "Annual".
- **The close's condition asked first,** as the multiples ask it. Rejected:
  the multiples have the close in the numerator, and here it is the divisor.
  Formula order is the dividend first, which is also the order of the reads.
  Both margins ask their figure's conditions before the close's.
- **The name `dividend_yield`.** It is the word every reader knows. Rejected:
  the naming rule puts `per_share` in the name of a form that reads a
  per-common-share concept, and the bare name is the one a total form would
  take.
- **A percentage as the unit.** Rejected by the arithmetic record: a ratio is
  a fraction of one, and analyze writes no scaled figure.
- **Leaving the yield out of M5 until the provider is built.** Rejected: a
  test builds the crossing as the contract type, so each figure and each
  `no_price` is provable today, and the price read is on main. Deferring
  would also change milestone scope, which is Constitutional.

## Consequences

**Easier:**

- The yield has its form, period, conditions, name and source. Its
  implementing task has only its code left to decide.
- It needs no setting, no constant, no new condition and no new reason.
- A non-payer shows `0`, beside a streak of `0`.
- A user with no key sees the yield absent with `no_price`, and every other
  dividend metric computed.
- Relative yield's record has a fixed yield to stand on.

**Harder:**

- **Every quarter is absent,** with `not_a_year`.
- **A filer that split after its latest year's end is absent,** until a year
  ending after the split is filed.
- **A past year's yield is not the yield of that year.** It is that year's
  dividend at today's price, and nothing marks it.
- **A special dividend reads as a high yield** for one year.
- The test files that pin which metrics a crossing moves name each price
  reader, so the implementing task changes them too, as `M5-61` and `M5-62`
  did.

**Expensive to reverse:**

- The name and the form, once results are stored and ranked on.
- A negative dividend as a condition and a zero as a figure. Changing either
  later turns stored absences into numbers or numbers into absences, under a
  bump.

**Open, and not this record's:**

- **Relative yield.** `price-provider.md` (Accepted) reads it at "each past
  fiscal period end the history holds, for relative yield only", "as a
  company's current yield against its own past yields (Weiss and Lowe,
  *Dividends Don't Lie*, 1988)". So it sets the yield this record defines
  against past yields, and the book is one no run can open. This record
  attributes nothing to that book, states no form for relative yield, and
  decides nothing a later record on it would have to undo. It is a record of
  its own, after this one.
- Whether a year's yield is ever stated at that year's own close.
- A trailing yield, which needs trailing spans formed.
- The regular dividend alone, which waits on a `fetch-normalize` version
  that carries the axis and member, as the open-questions record says.
- The class of a share, which neither v2 nor the crossing names.

## Enforcement

This applies anchor 5, the price invariant and the one-source-of-truth
invariant, and edits none of them.

- **No literal.** `literals` keeps every number but zero and one out of the
  derivation. The yield needs none.
- **One definition of each condition.** Each is the existing `const`.
  - **Gap:** nothing stops a second `const` with the same characters, except
    review.
- **The dividend is the filing's.** The crossing carries no dividend, so the
  yield cannot read the provider's.
- **No wrong number across a split.** The share-basis tests hold it.
- **No yield without a price.** The witness tests hold it.
- **The source says what the record says.**
  - **Gap:** nothing checks that a citation is true.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the name and meaning
  of one metric that crosses `analyze-store` v1 as characters, and a formula
  under the method version, all inside analyze. It edits no anchor, no
  protected path, no gate, no contract byte, no schema and no milestone
  scope, and adds no condition, reason, setting or constant. Leaving
  relative yield to its own record defers a record, not GOALS.md's scope.
  Flagged for later human review, as the tier requires.
- **Checked:**
  - Every quotation of an accepted record against main, whitespace collapsed:
    the catalogue and open-questions records on the concept, the provider
    record's four sentences, the common-shareholder record's list and naming
    rule, the analyze record, the share-basis, series, discounted cash flow
    and arithmetic records. The v2 fields were read in the contract file:
    `flow`, `currency_per_share`, `as_presented`, all five kinds, and both
    arms of the conditional silence.
  - `negative_dividends_declared_per_share` stands once, beside the streak,
    under the words quoted, and `non_positive_close` once, beside the
    discounted cash flow margin. Each is already `pub(super)`, so the record
    is right that no visibility widens. The share-basis record says the year
    test gives `end_not_a_date` before E reaches the rule.
  - The chapter was downloaded whole again in the deciding run: 4,678,827
    bytes at the sha256 the record names. Each quotation was found in the
    half-sheet of the printed page named. Page 16's cells are as quoted, and
    $0.35 over $32.28 and over $22.69 give its 1.08% and 1.54%. The edition
    page arrived in part on each of three tries, both sentences in each part.
  - The proof re-derives by hand: `0.025`, `0.02`, one thirtieth as `0.0`
    and thirty-four `3`s at the arithmetic record's precision, `0.00625` as
    the quarter's figure never written, `not_covered` for 2023 under the
    preset crossing, and the split dated 2024-12-31 against 2024-12-30.
  - No accepted record fixes another name for the yield, and none is
    contradicted. The condition order, the dividend's before the close's, is
    the order both margins ask in; the multiples ask the close first where it
    is the numerator.
  - The alternatives as argued: the total form, `divCash`, the adjusted
    close, a quarter times four, every duration, a zero as a condition and a
    negative as a figure are each rejected on a quoted source or an accepted
    record. The year's own close is correctly left as "not rejected on its
    merits".
- **Verdict and why:** Accepted. The form is one line of a source the run
  opened, and everything else is reached rather than made: the price read,
  the two conditions, the year test. The record's one hard place is the
  chapter's own table, which sets each past year's dividend over that year's
  closing price, and it handled that the right way: it quoted the table,
  kept the close the provider record fixes, and named the question instead
  of narrowing "for relative yield only". For the latest year the two
  readings meet in the margin note's "current price"; for an older year the
  figure is that year's dividend at today's price, which is what every
  price-dependent metric on main already states at a past period. A zero as
  a figure is the chapter's own count and agrees with the streak; a negative
  as a condition keeps one input from being usable in one column and
  undefined in three.
- **What would have changed it:** a page of the chapter stating the form
  itself over another price or another dividend than quoted; a second
  definition of either condition, or a new one; the past period ends read
  here against the provider record's sentence; a form for relative yield, or
  anything attributed to the book no run can open; or a quotation of an
  accepted record that main does not carry.
- **Flagged for the human:** a past year's yield stated beside the
  valuation date's close and not that year's own, against the chapter's
  Illustration 10.1; the name `dividend_yield_per_share`, where
  `dividend_growth_rate` and `dividend_streak` read the same concept without
  `per_share`; a negative dividend as a condition and a zero as the figure
  `0`; and the fiscal year read as the chapter's "Annual".
