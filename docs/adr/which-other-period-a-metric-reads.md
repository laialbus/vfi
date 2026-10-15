# A metric that pairs a flow with a balance is stated at the flow's duration and reads the balance at the instant that duration closes at, opens at, or the two averaged, as its source fixes; finding the preceding duration is another record's

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-14, by `M5-26`
- **Decided:** 2026-10-14, by the decider
- **Touches:** the rule `what-analyze-takes-and-returns.md` leaves to "that
  metric's task" and names in `no_period` as "the rule that sought the other
  period". It fixes the characters `analyze-store` v1's `sought_by` carries,
  and no byte of that contract changes. It applies the accepted `declined`
  reason to a period start that states no calendar date, and adds no reason.
  It defines once the reading of a date that
  `how-analyze-computes-and-writes-a-figure.md` leaves to "the share-basis
  rule and each metric", and puts that reading's numbers under
  `crates/analyze/src/constants/`, one of `bare-literal-lint.md`'s three
  places. No accepted record is superseded. No contract's bytes, no schema, no
  edge, no gate, no dependency and no entry field move. Nothing here is
  implemented.

## Context

GOALS.md's fourth M5 criterion names returns on capital, and with them asset
turnover, Altman's score and Piotroski's signals. Each of them divides a flow
by a balance. So far every metric reads its own row and no other. The first of
these will be the first to read a second row.

Some things are already fixed:

- **A period is its dates.** `canonical-concepts` v2 states a flow at a
  duration and a balance at an instant. Two periods are one only when their
  dates are equal character for character, so an instant and a duration are
  never one. A `Row` holds one period.
- **Every concept states its measure.** v2 publishes `flow` or `balance` per
  concept, and `vfi-contracts` carries it as `Definition::measure`.
- **The history is not uniform.** `period-alignment.md` admits every period a
  filer states and labels none. It holds years, quarters, year-to-date spans
  and transition stubs side by side, and "no consumer may assume one annual
  period per calendar year". Telling a year from a quarter is left open there.
- **Absence is closed.** The accepted analyze record gives `no_period` its
  carriage: the period the metric was computing at, as `computing_at`, and the
  name of the rule that sought the other period, as `sought_by`. Reasons are
  one per absent input, in the metric's input order, and nothing is
  substituted for an input.
- **Dates are not numbers.** The arithmetic record reads amounts and closes,
  and leaves dates to "the share-basis rule and each metric".
- **The method version** moves with "which periods or dates a metric reads"
  and with "a rule of absence or composition".
- **The entry shape.** M5-18 fixed a metric's catalogue entry: name, meaning,
  unit, the shape it is stated at, its inputs in reason order, and its own
  `declined` conditions.

What the fixture `a-filer-that-changed-its-fiscal-year` shows, read off its
claims:

- The balance at a duration's opening is stated at the day before the
  duration starts. 2024-01-01 to 2024-12-31 opens at the instant 2023-12-31.
  The transition period 2023-05-01 to 2023-12-31 opens at 2023-04-30.
- An opening or closing instant can be missing. 2023-01-01 to 2023-12-31
  closes at 2023-12-31, but 2022-12-31 is no period: the filer was on an
  April year then. The quarter 2023-01-01 to 2023-03-31 has neither instant.
- One instant closes several durations. 2024-06-30 closes the quarter
  2024-04-01 to 2024-06-30 and the half year 2024-01-01 to 2024-06-30.

## Decision

### Where the result is stated

A metric whose formula pairs a flow with a balance is stated at a duration. It
is computed at every duration the history holds, like every duration metric
today, and its figure is over that duration. It is never stated at an
instant.

Why: from a duration, its two instants follow from its own dates. From an
instant, the duration is not determined, because one instant closes a
quarter, a half year and a year alike. Choosing one is the year-or-quarter
question `period-alignment.md` left open.

A metric stated at an instant reads only balances, at its own row, as the
liquidity and leverage metrics do now. This record changes nothing for them.

### The three readings

Every balance a duration metric reads is read at exactly one of these, named
once in the rule:

- **Closing.** The balance at the instant the duration closes at.
- **Opening.** The balance at the instant the duration opens at.
- **Averaged.** The mean of the opening and closing balances: their sum
  divided by how many were summed. The count starts at zero and steps by one,
  so the mean needs no literal.

A flow is read at the computing duration's own row. A balance is never read
at a duration's row.

### How a sought row is found

The closing instant is the instant whose date equals the duration's end,
character for character. No date is read.

The opening instant is the instant whose date equals the day before the
duration's start, written in the date grammar below and compared character for
character. The start is read as a calendar date and stepped back one day.

Nothing is ordered. No date is compared as earlier or later, and no other row
is ever substituted. The nearest earlier instant is never taken. When the
filer did not state the instant, a near one would be a different balance that
looks right.

Why the day before. In XBRL, a date with no time part in an `instant` or
`endDate` stands for midnight at the end of that day, and in a `startDate` for
midnight at its start. So an instant dated D is the same moment as the start
of a duration dated D plus one day. The source: Engel, P., Hamscher, W.,
Shuetrim, G., vun Kannon, D., and Wallis, H., *Extensible Business Reporting
Language (XBRL) 2.1*, XBRL International, Recommendation 2003-12-31 with
errata corrections to 2013-02-20, §4.7.2. The step function's documentation
cites it, and the implementing task gives the paragraph as it reads it.

Where the sought instant is found, its row is the history's row whose period
equals it. A repeated period is taken at its first row, as the entry point
already takes it.

### The date reading, defined once

The reading is one module of the derivation, `crates/analyze/src/date.rs`.
The opening seek reads through it, and it is the only place analyze reads a
date.

**The grammar** is RFC 3339's `full-date`: four digits, `-`, two digits,
`-`, two digits. The month is 01 to 12, and the day runs from 01 to the
month's length in that year. The source: Klyne, G., and Newman, C., *Date and
Time on the Internet: Timestamps*, RFC 3339, IETF, 2002, §5.6 for the grammar,
§5.7 for each month's length, and Appendix C for the leap year. This is the
grammar EDGAR's facts carry period dates in.

**What it reads a date as.** A year, a month and a day on the proleptic
Gregorian calendar. These are calendar fields and not amounts: they never
enter the arithmetic record's number type, and no figure is made of them.
Digits are read with `char::to_digit(RADIX)`, as the amount reader reads them.

**What it does.** It reads a date, steps back one day, and writes a date in
the same grammar. Reading returns a date or nothing, and never fails.
Stepping back across a month uses the previous month's length, across a year
uses December, and in February uses the leap-year rule.

**Every number it needs** stands in `crates/analyze/src/constants/calendar.rs`
under the RFC 3339 citation. That covers the field widths, the count of
months, each month's length and the three leap-year divisors. A number the
implementing task needs beyond those is an escalation. One day is the step,
and one is on the allowlist.

**A start outside the grammar** states no day, so its opening instant cannot
be named. That read is `declined` under one condition, `start_not_a_date`,
defined once beside the reading. The same holds for the one start in the
grammar that has no day before it, 0000-01-01. Why `declined`: the start is
present, and the metric's definition, the balance at the day before the start,
is undefined where the start states no day. This is the same argument as
`input_not_a_decimal`. It is not `no_period`, because no row can be shown
missing when it cannot be named. The closing seek reads no date, so an end
outside the grammar still matches an instant written the same way.

**No other reader.** So that two records never read a date two ways, any
later part of the derivation that reads a date reads it through this module:
the share-basis rule when it is built, and the preceding-duration record
below. One that needs more, such as ordering, adds it here under its own
record, and never beside it. What the share-basis rule does with a trading
date outside the grammar is its own task's.

### Which input is a balance

The rule reads `Definition::measure`, the one copy of v2's measure. The entry
does not list it. A metric's derivation names a reading for each input whose
measure is `balance`, and for no other. A reading named for a flow, or a
balance read with none, is a defect, and the implementing task's types or
tests refuse it.

### Which reading each metric reads

The rule has two parts:

- **A term of a named score reads what its author fixed.**
- **Every other metric that pairs a flow with a balance reads the average.**
  This holds whatever source its formula cites. A balance its source names
  without saying when is read at closing, since that is the balance the
  period's own statement presents.

Each metric's formula, and which concepts make each term, stay with the
metric's task. This table fixes only the instant a balance is read at:

| Metric or term | Balance reading | Source |
| :--- | :--- | :--- |
| Return on assets | averaged | CFA, below |
| Return on equity, and on common equity | averaged | CFA |
| Return on capital employed | averaged | CFA |
| Return on invested capital | averaged | CFA |
| Asset turnover | averaged | CFA |
| Altman's X1, working capital to total assets | closing | Altman |
| Altman's X2, retained earnings to total assets | closing | Altman |
| Altman's X3, earnings before interest and tax to total assets | closing | Altman |
| Altman's X4, the book value of total debt beside the market value of equity | closing | Altman. The price's date is the score's task, under the share-basis rule. |
| Altman's X5, sales to total assets | closing | Altman |
| Piotroski's ROA, and the ROA ΔROA compares | opening | Piotroski |
| Piotroski's CFO, and ACCRUAL | opening | Piotroski |
| The asset turnover ΔTURN compares | opening | Piotroski |
| The leverage ΔLEVER compares | long-term debt closing, total assets averaged | Piotroski |

ΔLIQUID and ΔMARGIN pair no flow with a balance. EQ_OFFER reads no balance.
None of them is this table's.

**The sources:**

- **Altman** reads every ratio off the financial statements of one report, so
  total assets is that report's balance sheet: the balance at the duration's
  close. Altman, E. I., "Financial Ratios, Discriminant Analysis and the
  Prediction of Corporate Bankruptcy", *The Journal of Finance* 23(4), 1968,
  pp. 589–609.
- **Piotroski** scales ROA, CFO, ACCRUAL and asset turnover by
  "beginning-of-the-year total assets", and measures leverage against
  "average total assets". Piotroski, J. D., "Value Investing: The Use of
  Historical Financial Statement Information to Separate Winners from
  Losers", *Journal of Accounting Research* 38, Supplement, 2000, pp. 1–41,
  §2.1.
- **CFA.** For a ratio that sets an income statement or cash flow figure
  against a balance sheet figure, the CFA Institute's text uses the average
  of the opening and closing balances. Its ratio exhibits define return on
  assets, return on equity, return on total capital and total asset turnover
  over average balances. Robinson, T. R., Henry, E., Pirie, W. L., and
  Broihahn, M. A., *International Financial Statement Analysis*, 3rd ed., CFA
  Institute Investment Series, Wiley, 2015, the chapter "Financial Analysis
  Techniques".

Each metric's documentation cites its source by page or exhibit as its task
reads it. A source that on reading says otherwise than this record says is a
stop for that task. It is not a choice.

**Where the sources differ, and why the average.** For a standalone return,
the sources disagree:

- Damodaran uses the capital at the period's start, the capital that earned
  the period's income. Damodaran, A., "Return on Capital (ROC), Return on
  Invested Capital (ROIC) and Return on Equity (ROE): Measurement and
  Implications", Stern School of Business, New York University, 2007.
- Data vendors commonly use the closing balance.
- The CFA text uses the average.

The average is followed, for three reasons:

- It is the one source of the three that states a rule for the whole family,
  not a ratio at a time. So one citation covers all five metrics and none of
  them chooses.
- A flow accrues across the duration while the balance moves. The mean of the
  two ends is the balance the flow was earned on, to first order, and neither
  end alone is.
- What it costs is absence: it needs both instants. That is an absence and
  never a wrong number.

It is not a setting. A setting is a value "neither the filings nor the
metric's cited source fixes", and once the source is cited, it fixes this.

### How absence composes

A metric's **reads** run in its entry's input order. A flow is one read, at
the computing row. A balance at closing or opening is one read, at that
instant. An averaged balance is two reads, opening and then closing.

Each read gives a number or exactly one reason:

- **The sought row is not in the history:** `no_period`, with `computing_at`
  the duration and `sought_by` one of two names, each a `const` defined once
  beside the rule:
  - `opening_instant` for the opening seek
  - `closing_instant` for the closing seek
- **The opening cannot be named:** `declined` with `start_not_a_date`.
- **The row is there:** the concept's state at that row. `Unknown` gives
  `input_unknown`, and `NotApplicable` gives `input_not_applicable`. Each
  names the concept and the sought period, through the accepted constructors
  handed the sought row. A `Value` outside the decimal grammar is `declined`
  with `input_not_a_decimal`.

Every read is made, and none stops another. The reasons follow the read
order, so a missing opening row never hides an `Unknown` flow at the
computing row. Only when every read gives a number are the metric's own
`declined` conditions, such as a zero divisor, asked. The accepted exception,
that a `NotApplicable` term contributes nothing where the definition says so,
is unchanged.

Two names, not one, because an averaged balance with both rows missing gives
two `no_period` reasons. They carry no period, so one name would make them
identical and hide which row is missing. Two balances read at one missing row
give two identical reasons. That is one per absent input, as the accepted
record states, and they are not merged.

No seventh reason is needed, and no frozen byte changes.

### The preceding duration and the series: not this rule

These need a preceding duration, or a run of consecutive ones:

- Piotroski's ΔROA, ΔLEVER, ΔLIQUID, ΔMARGIN and ΔTURN, each comparing a year
  with the year before
- the dividend growth rate and the dividend streak

They do not follow this rule. Finding the year before a year means telling a
year from a quarter, a year-to-date span and a transition stub, all of which
the history holds. The duration that ends the day before 2024-01-01 starts
may be a quarter or a year. Across a fiscal-year change, as in the fixture,
the year before may not be a year at all. 52- and 53-week years make length
alone no test. That is the open question `period-alignment.md` names, and it
needs a fiscal year end or a length class that no source here fixes.

So they are left to a record of their own, and it reads dates through the one
module above. For the same reason, Altman's and Piotroski's scores, which
their authors state over a year, wait for that record before they are
computed. This record fixes which instant each of their terms reads, and it
does not make them computable at a quarter.

### The method version

The rule is part of the method `METHOD_VERSION` names, as the accepted record
places a rule of composition and which periods or dates a metric reads. That
covers:

- the three readings and the table
- the two seeks
- the date reading and its constants
- `start_not_a_date`
- the two `sought_by` names

A later change to any of them is a version bump and a superseding record, and
never an edit to this one.

It lands without a bump. No metric reads across periods yet, so no stored
outcome changes, and the method stands at version 1 as main holds it. Giving
each derivation the history beside its row is a refactor with identical
outcomes.

### What a metric's catalogue entry says

The entry needs nothing more than the shape M5-18 fixed:

- **`stated_at`** is `Duration`.
- **`inputs`** lists the concepts as today. Which of them are balances is read
  from v2.
- **`meaning`** says in words which balance is read and at which instant, as
  it already says the formula in words, for example "net income over the
  duration, divided by the average of total assets at the instants it opens
  and closes at".
- **`conditions`** lists `start_not_a_date`, by its one constant, for every
  metric that reads an opening instant.

The two `sought_by` names are not conditions, and no entry repeats them.

A structured field for the reading would be a second copy of what the
derivation names, unless the derivation were generated from it. That
restructuring, and the change to M5-18's shape it needs, is a record of its
own if anyone wants it.

### The proof the implementing task brings

The task that implements the rule, alone or with the first metric that reads
it, brings these tests. They are built through `History::of` and `Row::at` and
read no file. Each stands in `tests/` or under `#[cfg(test)]`. At the least:

- **The figure.**
  - A history holding the duration 2024-01-01 to 2024-12-31 and the instant
    2024-12-31 gives a closing-read figure.
  - Holding 2023-12-31 instead gives an opening-read figure.
  - Holding both, a net income of 10 over total assets of 90 and 110 averages
    to `0.1`.
- **The missing row.**
  - A history lacking the closing instant gives `no_period` with
    `computing_at` the duration and `sought_by` `closing_instant`.
  - Lacking the opening instant gives `opening_instant`.
  - Averaged with both lacking gives the two in that order.
- **No near row.** A history that holds 2023-09-30 and 2024-09-30 and not
  2023-12-31 gives `no_period` for the opening, never the 2023-09-30 balance.
  A duration ending 2024-12-31 does not stand in for the instant.
- **The state at the sought row.** Total assets `Unknown` at 2024-12-31 gives
  `input_unknown` naming `total_assets` and the instant 2024-12-31, not the
  duration.
- **The order.** A flow `Unknown` at the duration, the opening row missing and
  the closing balance `Unknown` give three reasons in that order.
- **The step.** Each of these starts gives the opening date shown:
  - 2024-01-15 gives 2024-01-14
  - 2023-05-01 gives 2023-04-30
  - 2024-01-01 gives 2023-12-31
  - 2024-03-01 gives 2024-02-29
  - 2023-03-01 gives 2023-02-28
  - 2000-03-01 gives 2000-02-29
  - 2100-03-01 gives 2100-02-28
- **Outside the grammar.** Each of these starts gives `declined` with
  `start_not_a_date` for an opening read: `2024-1-01`, `20240101`,
  `2024-13-01`, `2023-02-29`, `0000-01-01`, and the empty string. A
  closing-only read on such a duration still matches by equality.
- **The shape.** A history of instants alone states no flow-and-balance
  metric.
- **The catalogue.** Every derivation that gives `start_not_a_date` over a
  history whose duration starts outside the grammar lists it among its
  conditions.

### What no check can see

- **Whether the sources say what this record says.** That covers Altman's
  closing balance, Piotroski's opening and average, the CFA text's average,
  XBRL 2.1 §4.7.2 and RFC 3339. Only the implementing tasks' reading sees it.
- **Whether a metric's `meaning` states the reading its derivation names.**
- **A metric that finds a row itself**, for example by taking the latest
  instant before its start. That is the one path to a wrong number that looks
  right. `History::periods()` is public in `vfi-contracts`, so the rule cannot
  be the only path without a contract change, and only review sees a
  derivation that skips it.
- **A filer that dates its balance sheet other than the day before its next
  duration starts.** That gives `no_period`, never a wrong number, and nothing
  counts how often it happens.
- **Whether normalize's dates are in the grammar.** Normalize passes them as
  published and nothing compares the two. A disagreement lands as
  `start_not_a_date` or `no_period`, so a metric absent across the whole
  corpus passes every gate.
- **A quarter's figure beside a year's.** A return over a quarter is a
  quarter's, and nothing marks it as one.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  calendar's numbers stand in `constants/`, one of its three places.
- `vfi-analyze` takes no dependency. The reading is written in the crate, and
  nothing enters its resolved tree.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and
  catalogue
- the arithmetic record's reading, operations and written form, since a date
  never enters the number type
- the frozen bytes of `canonical-concepts` v2 and `analyze-store` v1
- the entry shape M5-18 fixed

No question here needed one of them to change.

**Why Structural.** The record fixes the characters of a field that crosses
`analyze-store` v1, the `sought_by` values that store and M6 will read. It
also fixes a rule every cross-period metric shares. It needs nothing
Constitutional: no fourth place, no allowlist value and no gate weakened.

## Alternatives

- **The balance at the duration's end alone, for every metric.** It needs only
  equality, with no calendar and no constants, and it is the most often
  present. Rejected: Piotroski fixes the opening balance and the average, so
  his terms would be computed on a denominator he does not use. That is a
  number that looks like his and is not.
- **The average of the opening and closing balances, for every metric.**
  This gives one reading and no table. Rejected for the same reason the other
  way round: Altman reads one report's balance sheet, and Piotroski's ROA reads
  the opening.
- **The opening balance alone, Damodaran's reading, for the standalone
  returns.** It is defensible, and it costs one instant where the average costs
  two. Rejected: it covers return on capital and equity, but no source states
  it for the whole family, so asset turnover would cite a second source. The
  CFA rule covers all five in one citation.
- **The choice as a setting,** with a domain of the three readings and a
  stated preset. Rejected:
  - A setting is what the cited source leaves open, and each source here
    fixes it.
  - It would let one metric name carry three readings across stored results
    in different runs, which ranking over stored results would mix.
- **`canonical-concepts` v3 carrying the pairing**, such as each duration row
  holding its opening and closing balances. Rejected:
  - It bumps a frozen M4 surface, and through it `analyze-store` v2.
  - It moves normalize for a pairing analyze can find from dates it already
    holds.
  - Which balance a ratio reads is a method choice, which v2's `[elsewhere]`
    places in analyze. Normalize would be choosing analysis.
- **The nearest instant on or before the start**, by ordering. It needs no
  stepping, and it fills gaps. Rejected: where the true opening instant is
  missing, it takes an earlier quarter's balance. That is the wrong number
  that looks right.
- **The metric stated at the instant, seeking the duration that ends there.**
  Rejected: one instant closes a quarter, a half year and a year, and choosing
  among them is the year-or-quarter question this record does not answer.
- **One `sought_by` name.** Rejected: the averaged reading with both rows
  missing would give two identical reasons, and nothing would show which row
  is missing.
- **The reading as a field of the entry.** Rejected here: it changes the shape
  M5-18 fixed, and it is a second copy unless the derivation is generated from
  it. It is left to a record of its own.
- **The preceding duration decided here too**, by a length window such as 365
  days with a tolerance. Rejected: the window and its tolerance are numbers no
  source fixes, and `period-alignment.md` refused a day count for this reason.
  It is left to its own record, which reads dates through this one's module.
- **A date dependency**, such as `chrono` or `time`. Rejected: either brings
  source no check reads into analyze's tree, and a clock beside the calendar.
  The reading needs only one step back over a fixed grammar.
- **A start outside the grammar as `no_period`.** Rejected: its witness is a
  row the history lacks, and a row that cannot be named cannot be shown
  missing.

## Consequences

**Easier:**

- Every metric that pairs a flow with a balance reads it under one rule, and
  none of them chooses.
- Returns on capital and asset turnover can be built under it.
- Every failed read shows, and `no_period` tells an opening from a closing.
- Dates have one reader in the engine's derivation.

**Harder:**

- The average needs both instants. The first duration of every history is
  absent, and so is every duration whose filer stated only one of its two
  instants. In the fixture that is 2023-01-01 to 2023-12-31 and the quarter
  2023-01-01 to 2023-03-31.
- A return over a quarter is stated beside a return over a year, and is
  roughly a quarter of it. Nothing in the table tells them apart until the
  length question is answered. A ranking that mixes them misleads.
- The derivation now takes the history beside the row.
- The calendar is a second reader, with its own constants and tests.
- The standalone return on assets and Piotroski's ROA are different numbers.
  They need different names, and the catalogue keeps them apart.

**Expensive to reverse:**

- The readings, and the two `sought_by` names, once results are stored with
  them.

**Open, and not this record's:**

- The preceding duration and the series. This also covers when Altman's and
  Piotroski's scores may be computed.
- What the share-basis rule does with a trading date outside the grammar.

## Enforcement

This applies anchors 4 and 5, and edits neither.

- **Anchor 4.** The reading reads only the characters it is handed, with no
  clock and no dependency, so `purity` and `ambient` read what they read
  today.
- **Anchor 5.** `literals` keeps the calendar's numbers out of the derivation,
  and they stand in `constants/calendar.rs` under RFC 3339.
  - **Gap:** nothing checks that a citation is true, here or for the readings
    table.
- **No near row.** The proof's no-near-row test holds the rule.
  - **Gap:** nothing forces a metric through the rule rather than around it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the characters of
  `sought_by`, a field `analyze-store` v1 already carries as a string, and a
  rule every cross-period metric shares, both inside analyze. It edits no
  anchor, no protected path, no gate, no contract byte and no milestone scope,
  and asks for no allowlist entry. Flagged for later human review, as the tier
  requires.
- **Checked:**
  - `what-analyze-takes-and-returns.md`: `no_period` carries `computing_at`
    and `sought_by`, and the rule that seeks the other period is left to "that
    metric's task". This record is that rule and adds no seventh reason.
  - `how-analyze-computes-and-writes-a-figure.md`: dates are not numbers and
    are read by "the share-basis rule and each metric". The one date module
    here reads calendar fields and never enters the number type, as it says.
  - `bare-literal-lint.md`: `crates/analyze/src/constants/` is one of the
    three places, so `constants/calendar.rs` under RFC 3339 needs no fourth
    place and no allowlist change.
  - `period-alignment.md`: telling a year from a quarter is its open question,
    and this record leaves the preceding duration and the series to a record
    of their own rather than answering it by a day count.
  - The entry shape in `metrics/mod.rs`: `stated_at`, `inputs` and
    `conditions` carry what the record asks of an entry, so M5-18's shape is
    not changed.
  - Anchors 4 and 5 as the Enforcement section applies them: no dependency,
    no clock, every calendar number in a cited constant.
  - The alternatives as argued: closing alone, average alone, Damodaran's
    opening, a setting, a v3 pairing, the nearest instant by ordering, the
    instant-stated metric, one `sought_by` name, a field in the entry, a
    length window, a date crate, and `no_period` for a bad start.
- **Verdict and why:** accepted. What carries it is that the record refuses
  every path to a wrong number that looks right: no near instant stands in, no
  row is ordered, a start outside the grammar is `declined` and not read
  around, and each metric's reading is the one its author fixed, so Piotroski's
  ROA and the standalone return on assets stay different names for different
  numbers. The cost it takes is absence, which the accepted records already
  make the safe side. Choosing the CFA average for the standalone family on
  the strength of one citation covering all five metrics is a reasoned choice
  that a later record can supersede without an edit, since the method version
  moves with it.
- **What would have changed it:** a seventh reason, a changed byte of
  `analyze-store` v1 or `canonical-concepts` v2, a calendar number anywhere
  but `constants/`, or a reading under which a missing opening instant was
  filled from the nearest earlier row. So would a stated source for any term
  that, on reading, fixes the balance otherwise than the table says; the
  record makes that a stop for the implementing task, and that stop is the
  item flagged for the human alongside the Structural tier.
