# Piotroski's nine signals are each a metric beside his score, each a comparison over a year and the year before at the instants the period record fixed; his score is their count, and a filer whose shape lacks a signal's concept has none

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-10-20, by `M5-44`
- **Decided:**
- **Touches:** Piotroski's nine signals and his score, which GOALS.md's
  fourth M5 criterion names under financial health and the established
  screens and which no record forms. It fixes the names and meanings of ten
  metrics that cross `analyze-store` v1 as characters, and no byte of that
  contract changes. It forms the nine signals once, in one module. It puts
  the count of nine in `crates/analyze/src/constants/`, where
  `bare-literal-lint.md` places it. It applies the accepted `declined` reason
  to eight conditions, three already defined and five new, and adds no
  reason. It adds a value of the catalogue entry's `unit`, a count of
  signals, and no field. It adds no setting. It names two claims v2 cannot
  carry, the current portion of long-term debt alone and common equity
  issued in a year, and leaves each to a `canonical-concepts` v3 record of
  its own. No accepted record is superseded. No contract's bytes, no schema,
  no edge, no gate and no dependency move. Nothing here is implemented.

## Context

GOALS.md's fourth M5 criterion names financial health and the established
screens. Every record around Piotroski's score is accepted, and none forms
the signals: each comparison, the score as their sum, the conditions, and
what a filer whose shape has no gross profit or no current subtotal gets.

Some things are already fixed, and this record decides nothing of them:

- **The concepts each signal reads.** `canonical-concepts.md` (Accepted)
  names them by consumer:
  - `net_income` for ROA, ΔROA and ACCRUAL
  - `operating_cash_flow` for CFO and ACCRUAL
  - `revenue` for ΔTURN, and `gross_profit` for ΔMARGIN
  - `total_assets` for ROA, ΔROA, ΔLEVER and ΔTURN
  - `long_term_debt` for ΔLEVER
  - `current_assets` and `current_liabilities` for ΔLIQUID
  - `diluted_shares_weighted_average` for EQ_OFFER
- **Two of their meanings.** `canonical-concepts-open-questions.md`
  (Accepted) fixes `net_income` as income before discontinued operations and
  extraordinary items, citing Piotroski for it. It fixes `long_term_debt` as
  excluding the current portion and operating lease liabilities, and names
  ΔLEVER as what moves.
- **Applicability.** `gross_profit`, `current_assets` and
  `current_liabilities` apply to `operating` alone. Every other concept here
  applies to every kind. `long_term_debt` reads silence as `Unknown`.
- **The instant each term reads.** `which-other-period-a-metric-reads.md`
  (Accepted) gives Piotroski's rows of its table:
  - the opening balance for ROA, the ROA ΔROA compares, CFO, ACCRUAL and the
    asset turnover ΔTURN compares
  - long-term debt at closing over total assets averaged, for ΔLEVER
  - ΔLIQUID and ΔMARGIN pair no flow with a balance, and EQ_OFFER reads no
    balance
- **The year, and the year before.** `which-duration-precedes-a-duration.md`
  (Accepted) states every signal and the score over a year. It finds the
  year before by its seek, which M5-36 built in
  `crates/analyze/src/metrics/reading/year.rs` as `year`, `Year::before`,
  `Year::closing_of_before`, `Year::opening_of_before`, `PRECEDING_YEAR` and
  `PRECEDING_OPENING_INSTANT`. Every comparison seeks the year before, so
  that the five change signals are present or absent together. A duration
  that is not a year gets its `declined` conditions.
- **No price.** `price-provider.md` (Accepted) says the nine read no price,
  and screening makes no price call.
- **Constants and criteria.** `what-analyze-takes-and-returns.md` (Accepted)
  names Piotroski's thresholds as methodology constants. It names a
  threshold the user applies to stored metrics as M6's ranking criterion. It
  allows a `NotApplicable` term to contribute nothing only where the
  metric's definition states it, and names one such term:
  `short_term_investments` inside net debt.
- **The count.** `bare-literal-lint.md` (Accepted) places his count of nine
  in `constants/`, with the signals as `[bool; PIOTROSKI_SIGNALS]`.
- **The total form.** `common-shareholder-forms.md` (Accepted) leaves his
  signals in their total form, since their author fixes their inputs.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes the operations a derivation may take, and that "a count
  is a count".

Three things are not settled by any of these, and each is settled here:

- The copy of the paper this run read does not say one thing everywhere.
- EQ_OFFER names an event, an equity issue, that v2 carries only as a change
  in a weighted share count.
- Whether a bank, which has no gross margin and no current ratio, gets a
  score over seven signals or no score is a choice about what the score
  means.

## Decision

### The source

Under `sources-a-task-can-read.md` (Accepted):

- **The origin**, named as the period record names it: Piotroski, J. D.,
  "Value Investing: The Use of Historical Financial Statement Information to
  Separate Winners from Losers", *Journal of Accounting Research* 38,
  Supplement, 2000, pp. 1–41.
- **The open copy**: the author's paper reissued by his school, *Selected
  Paper 84*, The University of Chicago Graduate School of Business, 2002,
  read from
  <https://www.chicagobooth.edu/~/media/FE874EE65F624AAEBD0166B1974FD74D.pdf>.
  It is an open primary beside the origin. The journal printing was not read
  from the workspace.

Every locator here is written from that copy alone, by its section, footnote
and table numbers and its printed page numbers. The copy was read in this
run, by downloading it inside the sandbox and extracting its text with
`pdfminer.six` from PyPI. In that text the glyphs Δ, >, = and + come through
as `(cid:2)`, `(cid:3)`, `(cid:4)` and `(cid:5)`. The quotations below give
them as the characters they stand for.

| Locator | Quoted as printed |
| :--- | :--- |
| §2.3, p. 6 | "I chose nine fundamental signals to measure three areas of the firm's financial condition: profitability, financial leverage/liquidity, and operating efficiency." "An indicator variable for the signal is equal to one (zero) if the signal's realization is good (bad). I define the aggregate signal measure, F_SCORE, as the sum of the nine binary signals." |
| §2.3.1, p. 7 | "I define ROA and CFO as net income before extraordinary items and cash flow from operations, respectively, scaled by beginning of the year total assets. If the firm's ROA (CFO) is positive, I define the indicator variable F_ROA (F_CFO) equal to one, zero otherwise." "I define ΔROA as the current year's ROA less the prior year's ROA. If ΔROA > 0, the indicator variable F_ΔROA equals one, zero otherwise." "I define the variable ACCRUAL as current year's net income before extraordinary items less cash flow from operations, scaled by beginning of the year total assets. The indicator variable F_ACCRUAL equals one if CFO > ROA, zero otherwise." |
| footnote 3, p. 7 | "The benchmarks of zero profits and zero cash flow from operations were chosen for two reasons. First, a substantial portion of high BM firms (41.6%) experience a loss in the prior two fiscal years; therefore, positive earnings realizations are nontrivial events for these firms. Second, this is an easy benchmark to implement since it does not rely on industry, market-level, or time-specific comparisons." |
| §2.3.2, pp. 7–8 | "I assume that an increase in leverage, a deterioration of liquidity, or the use of external financing is a bad signal about financial risk." "I measure ΔLEVER as the historical change in the ratio of total long-term debt to average total assets, and view an increase (decrease) in financial leverage as a negative (positive) signal." "I define the indicator variable F_ΔLEVER to equal one (zero) if the firm's leverage ratio fell (rose) in the year preceding portfolio formation." "... where I define the current ratio as the ratio of current assets to current liabilities at fiscal year-end. I assume that an improvement in liquidity (i.e., ΔLIQUID > 0) is a good signal ..." "The indicator variable F_ΔLIQUID equals one if the firm's liquidity improved, zero otherwise." "I define the indicator variable EQ_OFFER to equal one if the firm did not issue common equity in the year preceding portfolio formation, zero otherwise." |
| §2.3.3, p. 8 | "I define ΔMARGIN as the firm's current gross margin ratio (gross margin scaled by total sales) less the prior year's gross margin ratio." "The indicator variable F_ΔMARGIN equals one if ΔMARGIN is positive, zero otherwise." "I define ΔTURN as the firm's current year asset turnover ratio (total sales scaled by beginning of the year total assets) less the prior year's asset turnover ratio." "The indicator variable F_ΔTURN equals one if ΔTURN is positive, zero otherwise." |
| §2.3.4, p. 9 | "F_SCORE = F_ROA + F_ΔROA + F_CFO + F_ACCRUAL + F_ΔMARGIN + F_ΔTURN + F_ΔLEVER + F_ΔLIQUID + EQ_OFFER." "Given the nine underlying signals, F_SCORE can range from a low of 0 to a high of 9". |
| §3.1, p. 10 | "After the BM quintiles are formed, I retain firms in the highest BM quintile with sufficient financial statement data to calculate the various financial performance signals." |
| §3.3, pp. 10–11 | "I classify firms with the lowest aggregate signals (F_SCORE equals 0 or 1) as low F_SCORE firms" and "firms receiving the highest score (i.e., F_SCORE equals 8 or 9) have the strongest fundamental signals and are classified as high F_SCORE firms". |
| Table 1 (continued), "Variable definitions", p. 13 | ΔLEVER: "Change in the firm's debt-to-assets ratio between the end of year t and year t-1. The debt-to-asset ratio is defined as the firm's total long-term debt (including the portion of long-term debt classified as current) scaled by average total assets." ΔTURN: "Change in the firm's asset turnover ratio between the end of year t and year t-1. The asset turnover ratio is defined as net sales scaled by average total assets for the year." |

The copy forms each signal as quoted, and the score as the sum of nine
indicators. So the copy gives no stop.

The constant's documentation and each metric's documentation cite the origin
and the copy, by the locators above. A copy that on an implementing task's
own reading forms a signal otherwise is a stop for that task, not a choice.

### Ten metrics

| Name | The source's name | Unit |
| :--- | :--- | :--- |
| `piotroski_f_roa` | F_ROA | a count of signals |
| `piotroski_f_delta_roa` | F_ΔROA | a count of signals |
| `piotroski_f_cfo` | F_CFO | a count of signals |
| `piotroski_f_accrual` | F_ACCRUAL | a count of signals |
| `piotroski_f_delta_margin` | F_ΔMARGIN | a count of signals |
| `piotroski_f_delta_turn` | F_ΔTURN | a count of signals |
| `piotroski_f_delta_lever` | F_ΔLEVER | a count of signals |
| `piotroski_f_delta_liquid` | F_ΔLIQUID | a count of signals |
| `piotroski_eq_offer` | EQ_OFFER | a count of signals |
| `piotroski_f_score` | F_SCORE | a count of signals |

Each signal is written `1` or `0`. The score is written as a count from `0`
to `9`. Every name carries `piotroski_`, so his ROA never shares a name with
`return_on_assets`, which reads the average. The period record asks for
exactly that separation.

**Why each signal is a metric beside the score.**

- M6 composes ranking late, over stored results, and a new screen never
  recomputes from filings. A signal that is stored can be screened on, such
  as "F_ACCRUAL is one". A signal that is not stored cannot be, without
  recomputation.
- A filer whose shape admits seven signals still has those seven stored,
  each with its meaning intact (see "By kind").
- The score's figure is the sum of figures that stand beside it. A reader can
  check it from the table, which is the proof replayed one step.

The cost is nine more names and nine more outcomes per year. Adding a metric
is not a bump.

### The nine signals, in formula order

Y is the year the metric is computed at, and Y−1 is the year before it. "At
Y's opening" is the instant Y opens at. The year before's closing instant is
Y's opening instant: one date and one row.

Each signal is a comparison giving one or zero. Every term is formed from
the operations of the arithmetic record's table, and from no other:
division, subtraction, addition (the mean, and the score's count), and
comparison and sign.

1. **F_ROA.** ROA is `net_income` over Y, divided by `total_assets` at Y's
   opening. One where ROA is above zero, and zero otherwise.
2. **F_ΔROA.** ΔROA is Y's ROA less Y−1's ROA. Y−1's ROA is `net_income`
   over Y−1, divided by `total_assets` at Y−1's opening. One where ΔROA is
   above zero, and zero otherwise.
3. **F_CFO.** CFO is `operating_cash_flow` over Y, divided by `total_assets`
   at Y's opening. One where CFO is above zero, and zero otherwise.
4. **F_ACCRUAL.** One where CFO is above ROA, and zero otherwise. This is
   stated on the two scaled terms, as the source states it.
   - It is the same comparison as `operating_cash_flow` above `net_income`
     wherever `total_assets` at Y's opening is above zero.
   - It is the reverse comparison wherever that divisor is below zero.
   - It is undefined where the divisor is zero, which is `declined`.
   - The derivation compares the scaled terms. So no branch on the
     divisor's sign is written, and the two forms cannot disagree.
   - The source's variable ACCRUAL is net income less cash flow from
     operations, scaled. CFO above ROA is that variable below zero.
5. **F_ΔMARGIN.** The margin is `gross_profit` over `revenue`, at one year.
   ΔMARGIN is Y's margin less Y−1's. One where ΔMARGIN is above zero, and
   zero otherwise.
6. **F_ΔTURN.** The turnover is `revenue` over a year, divided by
   `total_assets` at that year's opening. ΔTURN is Y's turnover less Y−1's.
   One where ΔTURN is above zero, and zero otherwise.
7. **F_ΔLEVER.** The leverage is `long_term_debt` at a year's closing,
   divided by the average of `total_assets` at that year's opening and
   closing. ΔLEVER is Y's leverage less Y−1's. One where ΔLEVER is below
   zero, and zero otherwise.
   - **An unchanged ratio gives zero.** The copy says "fell (rose)" and names
     no third case. Three reasons decide it. Every other signal gives its
     one only on a strict move in the good direction, and zero otherwise.
     §2.3 sets an indicator to one "if the signal's realization is good",
     and an unchanged ratio is not an improvement. And it never awards a
     point the copy does not state.
   - **What it costs.** A filer with `long_term_debt` zero at both closes has
     a ratio of zero both years, and gets zero. §2.3.2 calls only "an
     increase in leverage" bad, so the other reading is argued under
     Alternatives. This is the choice most worth the decider's judgement.
8. **F_ΔLIQUID.** The current ratio is `current_assets` over
   `current_liabilities`, at a year's closing. ΔLIQUID is Y's ratio less
   Y−1's. One where ΔLIQUID is above zero, and zero otherwise.
9. **EQ_OFFER.** One where `diluted_shares_weighted_average` over Y is not
   above the same over Y−1, and zero where it is above. Reading "did not
   issue common equity" through that count is argued below.

**F_SCORE** is the count of the nine that are one. The count starts at zero
and steps by one, so it needs no literal. It is never a fraction.

Every comparison is a sign test against zero, or a comparison of two figures
of one year pair. No benchmark needs a number beyond zero.

### Where the copy and the accepted records do not say one thing

There are three such places. At each, the record follows what is accepted,
and records the copy as printed.

**ΔTURN: the opening balance or the average.**

- §2.3.3 scales the asset turnover by "beginning of the year total assets".
- Table 1 (continued) defines it as "net sales scaled by average total
  assets for the year".
- The period record fixes the opening balance, and this record restates that
  as fixed. The body states it in the passage that defines F_ΔTURN, so the
  copy does not "say otherwise" in the period record's sense.
- Table 1's statement is named here so that the implementing task does not
  meet it unannounced. A reader who holds that the table governs argues it
  in a record that supersedes the period record's row, and bumps the method.
- ROA, CFO and ACCRUAL read "total assets at the beginning of year t" in the
  table too. Only ΔTURN differs.

**ΔLEVER: the current portion of long-term debt.**

- Table 1 (continued) reads total long-term debt "including the portion of
  long-term debt classified as current".
- v2's `long_term_debt` excludes the current portion.
- v2's `short_term_debt` holds the current portion, together with notes
  payable, commercial paper and current finance lease obligations, in one
  figure. So no v2 concept gives the copy's term.
- **This record reads `long_term_debt` alone**, as the catalogue names it.
- **Why `short_term_debt` cannot stand in.** Adding it would read the filer's
  short-term funding as long-term debt. A filer that rolls commercial paper,
  or draws a revolving facility, would move ΔLEVER on borrowings the copy's
  term excludes. That would be a second error put in place of the first, and
  one that looks like the copy's term.
- **The direction of the error.** A filer that reclassifies debt to current
  in Y has a lower `long_term_debt` at Y's close, while the copy's term is
  unchanged. Its leverage falls, and F_ΔLEVER reads one where the copy may
  give zero. This flatters. It flatters most where it matters most:
  - a maturity coming due within the year
  - a covenant breach, which classifies the whole debt as current

  Both are the distress the score exists to find. The reverse, a refinancing
  that moves current debt to long-term, reads as a rise, and costs a point
  the copy would give.
- **The remedy** is a concept for the current portion of long-term debt
  alone, which is a `canonical-concepts` v3. It is named, and left to a
  record of its own.

**The signals' section.**

- The period record cites §2.1 for Piotroski's opening and average
  readings, and quotes "beginning-of-the-year total assets".
- In this copy, §2.1 is "High book-to-market investment strategy" and states
  no reading. The signals are defined under §2.3.1 to §2.3.4, and the copy
  prints the phrase without hyphens.
- This record cites the section as this copy numbers it, and says so. Whether
  the journal numbers its sections otherwise is not known here, since the
  journal was not read.
- The period record's reading of each term agrees with what this copy
  states, ΔTURN's table apart.

### EQ_OFFER through the weighted share count

The catalogue names `diluted_shares_weighted_average` for EQ_OFFER. This
record restates it as fixed.

**The comparison** that stands for "did not issue common equity" is: Y's
count is not above Y−1's. An unchanged count is consistent with no issue,
so it gives one.

**What a weighted average over a year cannot see:**

- **An issue offset by a repurchase in the same year.** The count does not
  rise, and EQ_OFFER reads one where the filer issued common equity. This
  flatters.
- **A fall from a loss year.** In a loss year the diluted count is the basic
  count, since dilutive securities are anti-dilutive there. From a profitable
  Y−1 to a loss in Y, the count falls with no repurchase, and reads one.
  This flatters too.

**What it reads as an issue where none was made:**

- Dilutive securities entering the count, such as options coming into the
  money, or the step from a loss year to a profitable one. These read zero
  where the copy gives one.
- An issue late in Y−1, which a weighted average carries partly into Y−1
  and fully into Y. It reads as an issue in Y.
- A split or a share dividend that the year before's count is not restated
  for. Under Rule 3, the latest filing that states a period's count sets it,
  and a filing after the split commonly restates it. Nothing guarantees
  that it does.

So the error runs both ways. The case the copy's sentence is about, an issue
in the year, is missed in the flattering direction wherever a repurchase
offsets it.

**The faithful input** is a flow of the financing section: the proceeds from
issuing common equity over the year. EQ_OFFER would be one where it is zero.
v2 carries no such concept. It is named here, and left to a
`canonical-concepts` v3 record of its own. Reading it would also need that
record to decide whether shares issued to employees count as "common
equity" in the copy's sense.

### Constants, benchmarks, and no setting

- **`PIOTROSKI_SIGNALS`, nine**, stands in
  `crates/analyze/src/constants/piotroski.rs`. Its documentation cites the
  origin and the copy at §2.3, "nine fundamental signals", and §2.3.4,
  "range from a low of 0 to a high of 9". The nine signals are held as
  `[bool; PIOTROSKI_SIGNALS]`, as the lint record places it.
- **Each signal's benchmark is zero, the allowlist's zero.**
  - For ROA and CFO, the copy gives its reason for the zero benchmark in
    footnote 3, quoted above: a positive result is "nontrivial" for the
    firms it studies, and the benchmark "does not rely on industry,
    market-level, or time-specific comparisons". The documentation quotes
    it beside the constant file's citation.
  - For the five changes, zero is the no-change point of a difference. The
    copy states each as "positive", "> 0", "fell" or "improved".
  - ACCRUAL and EQ_OFFER compare two figures, and need no benchmark of
    their own.
- **No setting is added.** The source fixes every threshold and the count,
  so none is a value "neither the filings nor the metric's cited source
  fixes". `Settings` gains nothing for this method, and `settings/` gains no
  file.
- **The copy's portfolio cut-offs**, "F_SCORE equals 0 or 1" and "F_SCORE
  equals 8 or 9" in §3.3, are thresholds a user applies to a stored metric.
  The analyze record makes those M6's ranking criteria. They enter analyze
  nowhere: no constant, no setting and no metric.
- **The copy's population.** It forms the score over firms in the highest
  book-to-market quintile (§3.1). Analyze computes it for every filer, as its
  definition reads. Restricting to high book-to-market is a screen over
  stored results. It needs a price, so it is M6's.

### Formed once

The nine signals are formed in one module of the derivation,
`crates/analyze/src/metrics/piotroski.rs`. Each signal is a function there,
from its numbers to one or zero or its one condition. All ten derivations
reach the signals through it. No metric forms a signal itself, and the score
calls the same nine functions the signal metrics call.

The module reads through `reading.rs` and `reading/year.rs`. It finds no row
itself.

### By kind

`gross_profit`, `current_assets` and `current_liabilities` apply to
`operating` alone. So, where every other input is a `Value`:

| Kind | The seven signals | F_ΔMARGIN | F_ΔLIQUID | F_SCORE |
| :--- | :--- | :--- | :--- | :--- |
| `operating` | computed | computed | computed | computed |
| `bank`, `insurer`, `reit`, `investment_company` | computed | `input_not_applicable` for `gross_profit` at Y, then at Y−1 | `input_not_applicable` for `current_assets` and `current_liabilities` at Y's close, then at Y−1's | absent, with six `input_not_applicable` reasons |

The seven are F_ROA, F_ΔROA, F_CFO, F_ACCRUAL, F_ΔTURN, F_ΔLEVER and
EQ_OFFER.

The score's six reasons, in read order (see "How absence composes"):

- `gross_profit` at Y
- `current_assets` at Y's close
- `current_liabilities` at Y's close
- `gross_profit` at Y−1
- `current_assets` at Y−1's close
- `current_liabilities` at Y−1's close

**Why absent, and not a score over seven.**

- **The accepted rule.** A `NotApplicable` term contributes nothing only
  where the metric's definition states it. The one such term the analyze
  record names is `short_term_investments` inside net debt. Piotroski's
  definition states no signal that contributes nothing. To make one would
  extend the accepted exception, which is not this record's to do.
- **What the score means.** A score over seven signals is a different score
  from the one §2.3.4 states. It runs from 0 to 7 under a name that runs from
  0 to 9. A bank's 6 of 7 and an operating filer's 6 of 9 would read as one
  figure, and a ranking over stored results would mix them.
- **The source's own practice.** §3.1 keeps only firms "with sufficient
  financial statement data to calculate the various financial performance
  signals". A firm without the data for a signal is not scored.

A filer with no kind resolves every concept through the attempt. So where a
bank's `current_assets` is not found, it gives `input_unknown`, never
`input_not_applicable`.

The seven signals stored for a bank read `revenue` as the open-questions
record fixes it for a bank, and `long_term_debt` without deposits. Whether
they mean for a bank what they mean for the copy's sample is not a check this
record has (see "What no check can see").

### The `declined` conditions

Each is asked only when every read gives a number. They are asked in formula
order, which is §2.3.4's order of the sum. Within a signal, Y's term comes
before Y−1's. The first condition that holds is the one reason.

| Condition | Holds when | Signals | Defined |
| :--- | :--- | :--- | :--- |
| `zero_opening_total_assets` | `total_assets` at Y's opening is zero | F_ROA, F_ΔROA, F_CFO, F_ACCRUAL, F_ΔTURN | new, beside the module |
| `zero_preceding_opening_total_assets` | `total_assets` at Y−1's opening is zero | F_ΔROA, F_ΔTURN | new, beside the module |
| `zero_revenue` | `revenue` over Y is zero | F_ΔMARGIN | already, in `metrics/mod.rs` |
| `zero_preceding_revenue` | `revenue` over Y−1 is zero | F_ΔMARGIN | new, beside the module |
| `zero_average_total_assets` | the average of `total_assets` at Y's opening and closing is zero | F_ΔLEVER | already, in `metrics/mod.rs` |
| `zero_preceding_average_total_assets` | the average of `total_assets` at Y−1's opening and closing is zero | F_ΔLEVER | new, beside the module |
| `zero_current_liabilities` | `current_liabilities` at Y's close is zero | F_ΔLIQUID | already, in `metrics/mod.rs` |
| `zero_preceding_current_liabilities` | `current_liabilities` at Y−1's close is zero | F_ΔLIQUID | new, beside the module |

- The three already defined keep their meaning. Each states a zero divisor
  at the computing period, and each is named once. A second name for the
  same condition would be a second copy.
- The `preceding_` prefix is the one the series record uses for the year
  before, in `preceding_year` and `preceding_opening_instant`. A reader can
  tell Y's condition from Y−1's by the name alone.
- `zero_total_assets`, which `liabilities_to_assets` names, is not reused.
  It names no instant, and here two instants are divided by.
- The score asks all eight, in the table's order. Each signal asks its own,
  in the same order.
- EQ_OFFER divides nothing, and has no condition of its own.

**A negative figure is a value.** A negative figure at any term gives its
comparison, and is never a condition. This covers net income, operating cash
flow, gross profit, revenue, any balance, and any change. Nothing is
substituted for any figure, no signal is skipped, and no bound is applied.

The entries also list `start_not_a_date`, `end_not_a_date`, `not_a_year`
and `overlaps_another_year`, as the series record requires.
`input_not_a_decimal` reaches every entry from the one definition beside the
reader.

### How absence composes

1. **The year test first.** At a duration that is not a year, its one
   `declined` reason is the outcome, and no read is made.
2. **The reads,** in the input order the entry states:
   - Y's flows and balances come first, then Y−1's.
   - A flow is read at its year's row.
   - A balance is read at the instant its reading names.
   - Each read gives a number or exactly one reason. Every read is made, and
     none stops another. The reasons follow read order.
3. **The seeks.**
   - Y's own instants are read by `opening_instant` and `closing_instant`.
   - Y−1 is found by the year-before seek, and its rows are read once it is
     found.
   - Y−1's closing instant is Y's opening instant, read through
     `Year::closing_of_before` by the `opening_instant` seek.
   - Y−1's opening instant is read through `Year::opening_of_before`, and is
     `no_period` with `preceding_opening_instant` where the history lacks
     it.
   - A year the chain cannot find gives `no_period` with `sought_by`
     `preceding_year` for every read at it.
4. **The conditions,** as above, only where every read gave a number.

**`total_assets` at Y's opening is read once.** It is Y's opening balance,
and it is also Y−1's closing balance in Y−1's average for ΔLEVER. It is one
date and one row, so it is one read. A missing row gives one reason, not
two, as the analyze record's "one per absent input" asks.

**The score's reads.** Its inputs, in formula order, each concept where it
first appears in §2.3.4's sum: `net_income`, `total_assets`,
`operating_cash_flow`, `gross_profit`, `revenue`, `long_term_debt`,
`current_assets`, `current_liabilities`, `diluted_shares_weighted_average`.
It makes eighteen reads:

| # | Concept | Read at | `sought_by`, where the row is missing |
| ---: | :--- | :--- | :--- |
| 1 | `net_income` | Y | |
| 2 | `total_assets` | Y's opening instant | `opening_instant` |
| 3 | `total_assets` | Y's closing instant | `closing_instant` |
| 4 | `operating_cash_flow` | Y | |
| 5 | `gross_profit` | Y | |
| 6 | `revenue` | Y | |
| 7 | `long_term_debt` | Y's closing instant | `closing_instant` |
| 8 | `current_assets` | Y's closing instant | `closing_instant` |
| 9 | `current_liabilities` | Y's closing instant | `closing_instant` |
| 10 | `diluted_shares_weighted_average` | Y | |
| 11 | `net_income` | Y−1 | `preceding_year` |
| 12 | `total_assets` | Y−1's opening instant | `preceding_year`, or `preceding_opening_instant` |
| 13 | `gross_profit` | Y−1 | `preceding_year` |
| 14 | `revenue` | Y−1 | `preceding_year` |
| 15 | `long_term_debt` | Y−1's closing instant, which is Y's opening | `preceding_year`, or `opening_instant` |
| 16 | `current_assets` | the same | the same |
| 17 | `current_liabilities` | the same | the same |
| 18 | `diluted_shares_weighted_average` | Y−1 | `preceding_year` |

`operating_cash_flow` is not read at Y−1, since no signal reads it there.

**Each signal's reads** are its own inputs, in the same way:

| Signal | Inputs, in reason order | Reads |
| :--- | :--- | ---: |
| F_ROA | `net_income`, `total_assets` | 2: reads 1 and 2 above |
| F_ΔROA | `net_income`, `total_assets` | 4: reads 1, 2, 11 and 12 |
| F_CFO | `operating_cash_flow`, `total_assets` | 2: reads 4 and 2 |
| F_ACCRUAL | `net_income`, `operating_cash_flow`, `total_assets` | 3: reads 1, 4 and 2 |
| F_ΔMARGIN | `gross_profit`, `revenue` | 4: reads 5, 6, 13 and 14 |
| F_ΔTURN | `revenue`, `total_assets` | 4: reads 6, 2, 14 and 12 |
| F_ΔLEVER | `long_term_debt`, `total_assets` | 5: reads 7, 2, 3, 15 and 12 |
| F_ΔLIQUID | `current_assets`, `current_liabilities` | 4: reads 8, 9, 16 and 17 |
| EQ_OFFER | `diluted_shares_weighted_average` | 2: reads 10 and 18 |

Each signal reads in its own input order, Y first and then Y−1.

**A year before missing.** Six signals compare with Y−1. These are the five
change signals, and EQ_OFFER, which this record also reads as a comparison
with the year before. All six are absent together, each with `no_period`
and `preceding_year` for every read it makes at Y−1. The score gives eight
such reasons, reads 11 to 18. F_ROA, F_CFO and F_ACCRUAL read Y alone, and
are computed.

**The score is built on the signals, and carries their reasons.** Each
reason the score gives is a reason one of its signals gives, for the same
read. A read shared by several signals is made once, and gives its reason
once. Its `declined` is the first that its signals give, in formula order. It
never says "the signal was absent". Concatenating the nine signals' reasons
would repeat one missing input up to five times, against "one per absent
input".

No seventh reason is needed, and no frozen byte changes.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition:

- each signal's form, its benchmark and its comparison, including that an
  unchanged leverage gives zero and an unchanged share count gives one
- the reading of each term, as the period record fixes it
- EQ_OFFER through the weighted count
- the count of nine, and the score as the count of the nine
- the ten names, the eight conditions and their order
- the score's reads, and the one read of `total_assets` at Y's opening

A later change to any of them is a version bump and a superseding record,
and never an edit to this one.

It lands at version 1. No signal is computed yet, so no stored outcome
changes, and adding a metric is not a bump.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed, with one more value of `unit`:

- **`unit`.** `Unit` gains `Signals`: a count of Piotroski's signals that
  hold. `7` is seven, and a single signal is `1` or `0`. It is never `0.7`
  and never a percentage. `Years` counts years and is not reused. The entry
  gained `Years` the same way with the dividend streak. No field is added.
- **`stated_at`** is `Duration`.
- **`inputs`** are as the two tables above list them.
- **`conditions`** are `start_not_a_date`, `end_not_a_date`, `not_a_year`
  and `overlaps_another_year`, by their constants, then the metric's own
  from the conditions table, in the order they are asked.
- **`meaning`** says the comparison in words, and names the years it reads,
  as the series record asks. For example:
  - F_ΔROA: "One where net income over a year divided by total assets at the
    instant it opens at, less the same over the year before it, is above
    zero; zero otherwise."
  - F_ΔLEVER: "... an unchanged ratio gives zero."
  - EQ_OFFER: "One where the diluted weighted average share count over a
    year is not above the same over the year before it; zero otherwise. It
    stands for 'did not issue common equity' and cannot see an issue offset
    by a repurchase."
  - F_SCORE: "The count of Piotroski's nine signals that are one for a year
    and the year before it, from zero to nine."

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at` and read no file.
Each stands in `tests/` or under `#[cfg(test)]`. Unless a test says
otherwise, each history holds:

- the years 2023-01-01 to 2023-12-31 (Y−1) and 2024-01-01 to 2024-12-31 (Y)
- the instants 2022-12-31, 2023-12-31 and 2024-12-31

At the least:

**Two years, worked by hand.**

| Flow | 2023 | 2024 |
| :--- | ---: | ---: |
| `net_income` | 40 | 60 |
| `operating_cash_flow` | not read | 90 |
| `gross_profit` | 240 | 352 |
| `revenue` | 800 | 1100 |
| `diluted_shares_weighted_average` | 100 | 100 |

| Balance | 2022-12-31 | 2023-12-31 | 2024-12-31 |
| :--- | ---: | ---: | ---: |
| `total_assets` | 800 | 1000 | 1200 |
| `long_term_debt` | not read | 270 | 220 |
| `current_assets` | not read | 300 | 360 |
| `current_liabilities` | not read | 200 | 200 |

| Signal | Y | Y−1 | Change | Figure |
| :--- | :--- | :--- | :--- | ---: |
| F_ROA | 60 / 1000 = 0.06 | | | `1` |
| F_ΔROA | 0.06 | 40 / 800 = 0.05 | 0.01 | `1` |
| F_CFO | 90 / 1000 = 0.09 | | | `1` |
| F_ACCRUAL | CFO 0.09 against ROA 0.06 | | | `1` |
| F_ΔMARGIN | 352 / 1100 = 0.32 | 240 / 800 = 0.3 | 0.02 | `1` |
| F_ΔTURN | 1100 / 1000 = 1.1 | 800 / 800 = 1 | 0.1 | `1` |
| F_ΔLEVER | 220 / 1100 = 0.2 | 270 / 900 = 0.3 | −0.1 | `1` |
| F_ΔLIQUID | 360 / 200 = 1.8 | 300 / 200 = 1.5 | 0.3 | `1` |
| EQ_OFFER | 100 | 100 | not above | `1` |
| F_SCORE | | | | `9` |

**Each signal flipped alone.** Each changes the worked history as stated,
and only that signal moves:

| Signal | The change | What it gives | F_SCORE |
| :--- | :--- | :--- | ---: |
| F_ROA | `net_income` −30 in 2024 and −50 in 2023 | ROA −0.03. ΔROA is −0.03 less −0.0625, 0.0325, still `1`. ACCRUAL is 0.09 against −0.03, still `1`. | `8` |
| F_ΔROA | `net_income` 48 in 2023 | ROA before 0.06; a change of zero is not above zero | `8` |
| F_CFO | from the F_ROA row, `operating_cash_flow` −10 | CFO −0.01. ACCRUAL is −0.01 against −0.03, still `1`. | `7`, against that row's `8` |
| F_ACCRUAL | `operating_cash_flow` 60 | CFO 0.06, equal to ROA and not above it; CFO still above zero | `8` |
| F_ΔMARGIN | `gross_profit` 330 in 2024 | margin 0.3, a change of zero | `8` |
| F_ΔTURN | `revenue` 1000 and `gross_profit` 320 in 2024 | turnover 1, a change of zero; the margin stays 0.32 | `8` |
| F_ΔLEVER | `long_term_debt` 330 at 2024-12-31 | ratio 0.3, unchanged | `8` |
| F_ΔLIQUID | `current_assets` 300 at 2024-12-31 | ratio 1.5, unchanged | `8` |
| EQ_OFFER | `diluted_shares_weighted_average` 101 in 2024 | above the year before | `8` |

F_CFO cannot flip alone from the worked history. ACCRUAL is CFO above ROA.
So a CFO at or below zero, with ACCRUAL still one, needs ROA below it, and
ROA is above zero there. Its pair is therefore taken from the F_ROA row.

**At zero.**

- `net_income` 0 in 2024 and −40 in 2023 gives F_ROA `0`, since zero is not
  above zero, and F_ΔROA `1`. The score is `8`.
- `long_term_debt` 0 at both 2023-12-31 and 2024-12-31 gives F_ΔLEVER `0`.
  The score is `8`.
- `diluted_shares_weighted_average` 95 in 2024 gives EQ_OFFER `1`.

**Each `declined` condition**, over the worked history:

| The change | Signals that decline | F_SCORE |
| :--- | :--- | :--- |
| `total_assets` 0 at 2023-12-31 | F_ROA, F_ΔROA, F_CFO, F_ACCRUAL and F_ΔTURN, `zero_opening_total_assets`. F_ΔLEVER gives `1`: 220 / 600 against 270 / 400. | `zero_opening_total_assets` |
| `total_assets` 0 at 2022-12-31 | F_ΔROA and F_ΔTURN, `zero_preceding_opening_total_assets` | the same |
| `revenue` and `gross_profit` 0 in 2024 | F_ΔMARGIN, `zero_revenue`. F_ΔTURN gives `0`. | `zero_revenue` |
| `revenue` and `gross_profit` 0 in 2023 | F_ΔMARGIN, `zero_preceding_revenue`. F_ΔTURN gives `1`. | `zero_preceding_revenue` |
| `total_assets` −1000 at 2024-12-31 | F_ΔLEVER, `zero_average_total_assets` | the same |
| `total_assets` −1000 at 2022-12-31 | F_ΔLEVER, `zero_preceding_average_total_assets`. F_ΔROA and F_ΔTURN give `1` over the negative figure. | the same |
| `current_liabilities` 0 at 2024-12-31 | F_ΔLIQUID, `zero_current_liabilities` | the same |
| `current_liabilities` 0 at 2023-12-31 | F_ΔLIQUID, `zero_preceding_current_liabilities` | the same |

**The order of the conditions.**

- `revenue` 0 in 2024 with `current_liabilities` 0 at 2024-12-31 gives the
  score `zero_revenue` alone.
- `total_assets` 0 at 2023-12-31 with `revenue` 0 in 2024 gives
  `zero_opening_total_assets` alone.

**The year before missing.** The history without the year 2023:

- F_ROA, F_CFO and F_ACCRUAL give `1`.
- The other six give `no_period` with `computing_at` the year 2024 and
  `sought_by` `preceding_year`, once for each read they make at 2023.
- The score gives eight such reasons.

**The instants.**

- Without 2022-12-31: F_ΔROA, F_ΔTURN and F_ΔLEVER each give one `no_period`
  with `preceding_opening_instant`, and so does the score.
- Without 2023-12-31: the score gives four `no_period` with
  `opening_instant`, at reads 2, 15, 16 and 17. F_ΔLEVER gives two.

**The read order.** `net_income` `Unknown` in 2024, without the year 2023,
gives the score nine reasons: `input_unknown` naming `net_income` and the
year 2024, then the eight `preceding_year` reasons.

**The fiscal-year change.** Take the durations of
`a-filer-that-changed-its-fiscal-year` by their dates: 2022-05-01 to
2023-04-30, 2023-01-01 to 2023-12-31, 2023-05-01 to 2023-12-31,
2024-01-01 to 2024-12-31 and 2025-01-01 to 2025-12-31. Give every row a
figure, and hold the instants 2023-12-31, 2024-12-31 and 2025-12-31.

- At 2024, the first year on the new calendar, the six comparing signals and
  the score give `preceding_year` for every read at the year before.
- At 2025, the year before is 2024, and all ten are computed.

**A bank's row.** The worked history with `gross_profit`, `current_assets`
and `current_liabilities` `NotApplicable` at every row:

- The seven give `1`.
- F_ΔMARGIN and F_ΔLIQUID give the reasons the table under "By kind"
  states.
- The score gives the six reasons, in that order.

**Not a year.** At the quarter 2024-10-01 to 2024-12-31, with every input
`Unknown`, each of the ten gives `not_a_year` alone.

**Nothing else moves.** Every metric that M5-18 to M5-41 built gives its
pinned figures unchanged.

**The catalogue.**

- Each of the ten entries states `Unit::Signals`.
- Every derivation that can give one of the eight conditions or the four
  year conditions lists it.
- No two entries share a name.

### What no check can see

- **Whether the copy says what this record quotes.** This run read it, and
  the implementing task reads it again. Whether the 2000 journal printing
  says the same was not read.
- **The current portion of long-term debt**, named above. F_ΔLEVER flatters
  any filer that reclassifies debt to current, and nothing measures how
  often.
- **EQ_OFFER through the weighted count**, named above. It flatters an issue
  offset by a repurchase and a step into a loss year, and nothing measures
  either.
- **ΔTURN's two statements in the copy.** The record follows the body, and
  only review sees that the table differs.
- **A debt-free filer** gets zero for F_ΔLEVER. Under v2, `long_term_debt`
  reads silence as `Unknown`, so such a filer is absent unless a registry
  override asserts the zero, or the filer tags one.
- **`net_income` beyond the copy's words.** The copy says "before
  extraordinary items" and names no discontinued operations. v2 also
  excludes discontinued operations, as the open-questions record fixed. For
  a filer with discontinued operations, the figure is not literally the
  copy's term.
- **The score outside the copy's population.** The copy forms and tests it
  over high book-to-market firms. What a 7 means for a growth company, or
  for a bank's seven signals, is not something any check here can measure.
- **A 53-week year beside a 52-week one,** carried over from the series
  record. ΔROA and ΔTURN read one fifty-second more flow as improvement.
  ΔMARGIN and ΔLIQUID are ratios within a year, and do not.
- **A filer-dated year that holds another period's figures with nothing
  overlapping it,** carried over from the series record. It is compared as
  a year.
- **A signal formed outside the module,** such as ROA written inline in a
  metric's file. Only review sees it.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  count stands in `constants/`, which the record already names for it. Every
  benchmark is the allowlist's zero, and the score's count needs only zero
  and one.
- `vfi-analyze` takes no dependency, and `settings/` gains no file.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and
  catalogue
- the arithmetic record's reading, operations and written form
- the period record's readings, seeks, and its table's rows for Piotroski
- the series record's year test, seek, conditions and names
- the open-questions record's readings of `net_income` and `long_term_debt`
- the frozen bytes of `canonical-concepts` v2 and `analyze-store` v1
- the entry shape M5-18 fixed, which gains a value of `unit` and no field

Three questions here would need one of those to change. Each is left open,
and named:

- A concept for the current portion of long-term debt alone, which is a v3.
- A concept for common equity issued in a year, which is a v3.
- ΔTURN on the average, which would supersede the period record's row.

None is decided here. The ADR README's table gains no row, since no
accepted record is superseded.

**Why Structural.** The record fixes the names and meanings of ten metrics
that cross `analyze-store` v1, which store and M6 will read. It fixes the
characters of five new `declined` conditions, and a rule all ten share. It
needs nothing Constitutional: no anchor, no gate, no protected path and no
GOALS.md scope moves.

## Alternatives

- **The score alone, with no signal stored.** One name, and a tenth of the
  outcomes. Rejected:
  - M6 composes ranking late, over stored results, and could never screen
    on a signal without recomputing from filings, which the ranking
    invariant forbids.
  - A bank's seven signals would be lost behind its absent score.
- **A score over the signals a filer's shape admits.** A bank would get a
  figure. Rejected:
  - The accepted rule lets a `NotApplicable` term contribute nothing only
    where the definition states it, and names one such term. This record
    cannot extend it.
  - A count from 0 to 7 under a name that runs from 0 to 9 is a different
    score that looks like the same one.
  - §3.1 scores only firms with the data for every signal.
  - A rescaled count, such as seven signals times nine sevenths, is
    arithmetic no source states.
- **EQ_OFFER through `shares_outstanding` at the two closes.** This sees an
  issue late in the year fully, and does not move with dilutive securities
  or the step into a loss year. Rejected:
  - The catalogue names `diluted_shares_weighted_average` for EQ_OFFER, and
    this would leave that consumer reading nothing.
  - v2 dates `shares_outstanding` "at or near the period end". A count
    dated after the balance sheet, as a cover-page count is, is not at the
    closing instant, so the read would often be `no_period`.
  - It is no less blind to an issue offset by a repurchase.
  - A record that finds it the better stand-in can supersede this one.
- **EQ_OFFER through a financing concept v2 lacks,** the proceeds from
  issuing common equity. This is the faithful input. Rejected for now: it is
  a `canonical-concepts` v3, and through it an `analyze-store` v2, and a
  record of its own. It is named above.
- **ΔLEVER with the current portion read from `short_term_debt`.** It adds
  back what `long_term_debt` excludes. Rejected: it adds notes payable,
  commercial paper and current finance leases with it. So it reads a
  filer's short-term funding as long-term debt, which is an error that looks
  like the copy's term.
- **The score left out of M5** until a v3 settles EQ_OFFER and ΔLEVER.
  Rejected:
  - GOALS.md's fourth criterion names the established screens. The series
    record argued that deferring a named metric is a change of scope.
  - v2 forms every signal on the catalogue's own concepts. The two gaps are
    named with their direction, and a v3 record can close them by
    superseding this one.
- **An unchanged leverage gives one.** This reads "fell (rose)" as "did not
  rise". §2.3.2 calls only "an increase in leverage" bad, and a debt-free
  filer would get the point. Not taken:
  - Every other signal gives its one only on a strict improvement.
  - §2.3 gives one "if the signal's realization is good", and no change is
    no improvement.
  - It would award a point the copy does not state.

  This is argued either way, and flagged for the decider.
- **ΔTURN on the average,** as Table 1 (continued) defines it. Rejected
  here: the period record fixes the opening balance, citing the copy's body,
  and an edit to its row is a superseding record of its own.
- **The score as the nine signal metrics' outcomes, concatenated.** This
  needs no reads of its own. Rejected: a missing `total_assets` at Y's
  opening would appear five times, against one reason per absent input.
- **ACCRUAL as `operating_cash_flow` above `net_income`, unscaled.** This
  needs no divisor and never declines. Rejected: the source states it on the
  scaled terms. The two agree wherever the divisor is above zero, and the
  scaled form declines where it is zero rather than comparing unscaled
  figures the source never compares.
- **The continuous variables stored too**, such as ROA, ΔROA and ACCRUAL.
  Not here: the acceptance is the signals and the score. Piotroski's ROA
  needs a name of its own beside `return_on_assets`. A record that wants
  them can add them, and adding a metric is not a bump.
- **The cut-offs as a metric or a setting,** such as `piotroski_high`.
  Rejected: a threshold over a stored metric is M6's ranking criterion,
  under the analyze record.

## Consequences

**Easier:**

- The score and each signal have their source, readings, conditions and
  names. Each implementing task has only its code left to decide.
- Every signal is stored, and M6 can screen on any one of them.
- No setting is added, so no preset needs a reason.

**Harder:**

- **The score needs eighteen reads over two years and three instants.**
  The first year of every history has no score. Nor does the first year
  after a fiscal-year change, or any year missing one of the three instants.
- **Every non-`operating` kind has no score,** and no F_ΔMARGIN or
  F_ΔLIQUID.
- **`long_term_debt` reads silence as `Unknown`.** A filer that tags no
  long-term debt has no F_ΔLEVER, and so no score.
- **F_ΔLEVER flatters a reclassification to current,** and EQ_OFFER flatters
  an issue offset by a repurchase. Both are named, and neither is measured.
- **Ten more outcomes** per year per filer, in every hand-over.

**Expensive to reverse:**

- The ten names and their forms, once results are stored and ranked on.
- An unchanged leverage giving zero, once ranked on.

**Open, and not this record's:**

- A concept for the current portion of long-term debt, and one for common
  equity issued.
- Whether ΔTURN should read the average.
- The cut-offs, and any restriction to high book-to-market firms, which are
  M6's.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **Anchor 5.** `literals` keeps every number but zero and one out of the
  derivation. `PIOTROSKI_SIGNALS` stands in `constants/` under its citation.
  - **Gap:** nothing checks that a citation is true.
- **One definition per signal.** The flip tests and the score's worked
  history hold each signal to the module.
  - **Gap:** nothing forces a metric through the module rather than around
    it.
- **One read per row.** The missing-instant tests hold the score's reads.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
