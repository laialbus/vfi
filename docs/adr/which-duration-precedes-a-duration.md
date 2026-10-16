# A duration is a year when its own dates run twelve months to a month's end or fifty-two or fifty-three weeks and no other such duration shares a day with it; the year before a year is the year that ends the day before it starts, and a series is that seek repeated

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-14, by `M5-32`
- **Decided:** 2026-10-15, by the decider
- **Touches:** the record `which-other-period-a-metric-reads.md` (Accepted)
  leaves "to a record of their own": the preceding duration and the series.
  It fixes the characters of two more `sought_by` values that `analyze-store`
  v1 carries, and of three more `declined` conditions, and no byte of that
  contract changes. It adds no reason. It adds one operation to
  `crates/analyze/src/date.rs`, the ordering of two dates, under the accepted
  record's own clause that ordering "adds it here under its own record". It
  puts three numbers under `crates/analyze/src/constants/`, one of
  `bare-literal-lint.md`'s three places. It names two settings the dividend
  metrics add. No accepted record is superseded. No contract's bytes, no
  schema, no edge, no gate, no dependency and no entry field move. Nothing here
  is implemented.

## Context

GOALS.md's fourth M5 criterion names financial health and bankruptcy scoring
and the established screens. Its fifth names the dividend growth rate and
streak. These wait on this record:

- Piotroski's ΔROA, ΔLEVER, ΔLIQUID, ΔMARGIN and ΔTURN, each comparing a year
  with the year before
- Altman's score and Piotroski's score, which their authors state over a year
- the dividend growth rate and the dividend streak, which read a run of years

Every metric that reads a preceding duration or a series works under this
record and decides nothing of it alone.

Some things are already fixed:

- **A period is its dates.** `canonical-concepts` v2 names a period "by its
  dates alone": "no year, no quarter, no fiscal label, no length, no ordinal".
  Two periods are one only when their dates are equal character for character.
- **The history is not uniform.** `period-alignment.md` admits every period a
  filer states and labels none. It refused to label a year by a day count, as
  "the window candidate choice already refused". It said the answer "may find
  it needs a filer's fiscal year end — which is a decision and another version
  bump". It said "a dividend streak is read across consecutive periods, so a
  period this ruleset fails to admit reads as a suspension the filer never
  had".
- **The pairing rule.** `which-other-period-a-metric-reads.md` (Accepted)
  reads a balance at the instant a duration closes at, opens at, or both. It
  names the opening instant by stepping the start back one day through
  `date.rs`, compares character for character and orders nothing. Its
  `sought_by` names are `opening_instant` and `closing_instant`. It applies
  `declined` with `start_not_a_date` where the start states no day. It left
  the year before a year and the series to this record, and said that Altman's
  and Piotroski's scores wait for it.
- **Absence is closed.** The accepted analyze record gives six reasons, one
  per absent input, in the metric's input order. `no_period` carries
  `computing_at` and `sought_by`. A metric read over several periods is stated
  "at the last of them by that metric's own rule".
- **The entry shape.** M5-18 fixed a metric's catalogue entry: name, meaning,
  unit, `stated_at`, which is `Instant` or `Duration`, its inputs in reason
  order, and its own `declined` conditions. A metric is computed at every
  period of its shape.
- **A dividend suspension is a zero.** v2's `dividends_declared_per_share` is
  a `Value` of zero by silence when "the period's facts are silent on this
  concept and on dividends_paid alike". The catalogue fixed this so that "a
  suspension has to read as a zero or the streak cannot be measured". It
  handed M5 that "the dividend streak and growth rate must tolerate a
  one-period excess that does not recur".
- **The method version** moves with "which periods or dates a metric reads"
  and with "a rule of absence or composition".

What the fixtures show:

- **`every-fact-a-filer-reported`, CIK 0002003750.** The facts carry 24
  durations of 11 lengths: 24, 90, 91, 92, 182, 183, 273, 274, 365, 366 and
  457 days. Five are a year long by count. Three are the filer's fiscal years,
  October to September. The other two are a calendar tax year, 2024-01-01 to
  2024-12-31, and a lease term, 2024-12-01 to 2025-11-30. Rule 1 keeps
  both of those out of the history, which holds 19 durations.
- **`a-filer-that-changed-its-fiscal-year`, CIK 0001859199.** Two durations
  end on 2023-12-31, the day before the year 2024-01-01 to 2024-12-31 starts.
  - 2023-05-01 to 2023-12-31 is the transition period, 245 days.
  - 2023-01-01 to 2023-12-31 is 365 days. Its five facts are notes in the 10-K
    `0001213900-25-027581` and the 10-K/A `0001213900-25-042783`. They hold
    pretax income of −1,046,973, which is the transition period's own figure.
    They also hold current federal and state tax of 166,478 and 37,808, which
    sum to 204,286, the transition period's `income_tax_expense`. So this
    "calendar year" holds the eight-month period's figures under the filer's
    twelve-month dates.
  - It overlaps the last April year, 2022-05-01 to 2023-04-30, by four months.
  - Form does not separate them. The 10-K and 10-K/A of 2025 answer the
    transition period, the calendar year 2023, the April year and the year
    2024 alike.
  - The submissions document publishes one `fiscalYearEnd`, `1231`. That is
    the current value only, and neither April year ends on it.
- **`a-filer-that-suspended-its-dividend`, CIK 0001778784.**
  - Calendar years 2018 to 2024 read `dividends_declared_per_share` as 0, 0,
    0.09, 0.15, 0.12, `Unknown` and 0.
  - Its quarters are not safe to read in their place. The year 2021 reads 0.15
    and its nine months 0.11. Even so, the quarter 2021-10-01 to 2021-12-31
    reads 0 by silence.

## Decision

### What it governs

A metric is **stated over a year** when its source states it over a fiscal
year. That covers:

- every term of Altman's score and the score itself
- every one of Piotroski's signals and his score
- the dividend growth rate
- the dividend streak

Its `stated_at` stays `Duration`, so it is computed at every duration the
history holds, as M5-18's shape says. At a duration that is not a year it
gives the reason below. Every other duration metric is unchanged by this
record.

### What a year is

A duration is a **year** when both of the following hold. Each is read
from dates through `date.rs`. The **opening date** is the start stepped back
one day, by the accepted step.

1. **Its own length is a year,** by either of two clauses:
   - **Twelve months to a month's end.** The opening date and the end are each
     the last day of their month, in the same month, and the end's year is the
     opening date's year plus one. Examples:
     - 2024-01-01 to 2024-12-31
     - 2023-10-01 to 2024-09-30
     - 2023-03-01 to 2024-02-29, since 2023-02-28 and 2024-02-29 each end
       February
   - **Fifty-two or fifty-three weeks.** Stepping the end back one day 364
     times, or 371 times, gives the opening date, compared character for
     character.
2. **It is alone.** No other duration of the history whose own length is a
   year shares a day with it. Two durations share a day when each starts on or
   before the other ends.

Which of the two length clauses holds makes no difference: a year is a year.

**The source.** Internal Revenue Code, 26 U.S.C. §441:

- (d), a calendar year, "a period of 12 months ending on December 31"
- (e), a fiscal year, "a period of 12 months ending on the last day of any
  month other than December"
- (f)(1), an annual period that "varies from 52 to 53 weeks"

These are the annual periods a US filer may keep, and the periods Piotroski's
and Altman's annual data are drawn on. The constants' documentation cites the
subsections, and the implementing task gives each as it reads it. A reading
that says otherwise is a stop.

Section 441(f)(1) also requires that such a year "always ends on the same day
of the week". That is a property of a filer's run of years, not of one
duration, so it is not read.

**Why alone.** A filer keeps one annual accounting period at a time. Two
overlapping year-long spans cannot both be its years, and nothing in the
history says which is. So neither is taken. This is Rule 3's answer to a tie,
applied to years: neither the larger nor the more common, and neither the one
that fits the run. The clause needs no number. It is what keeps the
fixture's calendar year 2023 out, and with it the transition period's figures.
It would do the same for a stray tax year or lease term, if one ever reached a
history. That costs absence, and never gives a wrong year.

**What it reads.** Only the dates of the history's durations. It labels
nothing in the history, adds nothing to v2, and is computed afresh wherever a
metric asks.

### A duration that is not a year

A metric stated over a year, computing at a duration, first asks the test.
Where the test fails, the outcome is `Absent` with exactly one `declined`
reason. It is the first of these that applies:

- **`start_not_a_date`**, the accepted condition, where the start is outside
  the grammar. The opening date cannot be named, as the accepted record says.
- **`end_not_a_date`**, where the end is outside the grammar.
- **`not_a_year`**, where neither length clause holds.
- **`overlaps_another_year`**, where its length is a year and it is not alone.

The last three are each a `const` defined once beside the rule.

**No read is made.** Every read of such a metric is defined at a year.
Altman's sales over a quarter are not a term of his score, and "the year
before" a quarter does not exist. So the metric reads nothing there, and an
`Unknown` input at a quarter is not reported.

**Why `declined`.** The period is present, and the metric's definition is
undefined at it. The accepted record already applies `declined` to a period's
own dates, in `start_not_a_date`. It is not `no_period`, because no row is
missing.

### What the fixtures' durations become

**`every-fact-a-filer-reported`**, by the eleven lengths the facts carry:

| Days | Durations | Under the test |
| ---: | :--- | :--- |
| 24 | 2023-09-07 to 2023-09-30 | `not_a_year` |
| 90, 91, 92 | the quarters | `not_a_year` |
| 91 | the lease term 2024-09-01 to 2024-11-30, not in the history | would be `not_a_year` |
| 182, 183 | the half years | `not_a_year` |
| 273, 274 | the nine months | `not_a_year` |
| 274 | the tax span 2024-01-01 to 2024-09-30, not in the history | would be `not_a_year` |
| 365 | 2022-10-01 to 2023-09-30 | a year. The year before: `no_period` |
| 366 | 2023-10-01 to 2024-09-30 | a year. The year before: 2022-10-01 to 2023-09-30 |
| 365 | 2024-10-01 to 2025-09-30 | a year. The year before: 2023-10-01 to 2024-09-30 |
| 366 | the tax year 2024-01-01 to 2024-12-31, not in the history | would be `overlaps_another_year`, and so would the two fiscal years it overlaps |
| 365 | the lease term 2024-12-01 to 2025-11-30, not in the history | would be `overlaps_another_year`, and so would 2024-10-01 to 2025-09-30 |
| 457 | 2023-09-01 to 2024-11-30, not in the history | would be `not_a_year` |

The year 2022-10-01 to 2023-09-30 is a year the filer dated, and it holds the
same net loss as its 24-day inception stub. `period-alignment.md` takes it as
meant, and so does this rule.

**`a-filer-that-changed-its-fiscal-year`**, all 28 durations:

| Duration | Days | Under the test | The year before |
| :--- | ---: | :--- | :--- |
| 2021-05-01 to 2022-04-30 | 365 | a year | `no_period` |
| 2022-05-01 to 2023-04-30 | 365 | `overlaps_another_year` | |
| 2023-01-01 to 2023-12-31 | 365 | `overlaps_another_year` | |
| 2024-01-01 to 2024-12-31 | 366 | a year | `no_period` |
| 2025-01-01 to 2025-12-31 | 365 | a year | 2024-01-01 to 2024-12-31 |
| 2023-05-01 to 2023-12-31, the transition period | 245 | `not_a_year` | |
| 2022-05-01 to 2022-10-31, 2023-05-01 to 2023-10-31 | 184 | `not_a_year` | |
| 2023-01-01 to 2023-06-30, 2024-01-01 to 2024-06-30, 2025-01-01 to 2025-06-30, 2026-01-01 to 2026-06-30 | 181, 182 | `not_a_year` | |
| 2023-01-01 to 2023-09-30, 2024-01-01 to 2024-09-30, 2025-01-01 to 2025-09-30 | 273, 274 | `not_a_year` | |
| the thirteen quarters | 90 to 92 | `not_a_year` | |

**`a-filer-that-suspended-its-dividend`.** Each calendar year 2018 to 2024 is a
year. The year before each is the one before it, except for 2018, whose year
before is `no_period`. Every other duration is `not_a_year`.

### The year before a year

The **year before** a year Y is found from Y's dates:

- Y's opening date is written in the grammar by `date.rs`.
- The year before is the duration of the history whose end equals that date,
  character for character, and which is a year by the test above.
- Its row is the history's row whose period equals it. A repeated period is
  taken at its first row, as the accepted record takes it.

There is exactly one such year or none. Two years ending on one day would
share that day, so neither would be alone. The seek never chooses.

These are never taken:

- **A duration that is not a year**, even though it ends the day before:
  - the quarter 2023-10-01 to 2023-12-31
  - the half year 2023-07-01 to 2023-12-31
  - the transition period 2023-05-01 to 2023-12-31
- **A year that ends on another day.** Nothing is ordered to find one.
- **The nearest earlier year.**

Where no year is found, every read at it is `no_period`.

**Its instants.**

- **Its closing instant** is Y's opening instant: one date and one row. A
  balance at the close of the year before is read by the accepted
  `opening_instant` seek from Y.
- **Its opening instant** is the day before its own start. That seek is named
  `preceding_opening_instant`, so that a missing opening of the year before
  is never mistaken for a missing opening of Y.

No metric named here reads an instant at a year further back. One that does
needs a name of its own, in a record of its own.

**Every comparison seeks the year before.** That includes ΔLIQUID, which reads
only balances at Y's close and at the year before's close. Y's opening seek
alone could name the second, but then a signal could be compared with
whatever balance sheet precedes a stub. Seeking the year before first means
it is compared only with a year the rule found. So all five change signals
are present or absent together.

**Across a fiscal-year change** the year before the first year on the new
calendar is `no_period`. In the fixture, the year before 2024-01-01 to
2024-12-31 is sought at 2023-12-31. Two durations end there:

- The transition period is not a year.
- The calendar year 2023 is not alone. It overlaps the last April year, and
  it holds the transition period's figures under twelve-month dates.

The last April year ends 2023-04-30. It is never sought, because nothing is
ordered. So the year before is `no_period`, and nothing is compared across
the change. A filer that dated no calendar year beside its transition period
gives the same answer, more simply. The year 2025 has 2024 as its year before.

### A series of years

A series is the year-before seek repeated. The computing year is first. Each
next year is the year before the last one found. Nothing else finds a year
further back. The k-th year back is not named by stepping the calendar k
years, because 52- and 53-week years drift and a fiscal-year change moves
the dates.

**The dividend growth rate** reads n + 1 years: the computing year and the n
years before it.

- n is a **setting**, `dividend_growth_years`. Its domain is the positive
  integers. Its preset is 5.
- **The preset's reason.** Five annual steps hold five dividend decisions. So
  one year's special dividend, or one skipped raise, moves the rate through
  one step of five rather than through the whole rate. And six fiscal years
  are inside the structured history of any filer that has filed XBRL since
  the 2011 phase-in.
- No source fixes the window, so it is a setting and not a constant.
- Which of the n + 1 years its formula reads `dividends_declared_per_share`
  at, and how it tolerates the one-period excess, is that metric's task. It
  reads at least the first and the last. Every read it names is made.

**The dividend streak** counts the consecutive years whose
`dividends_declared_per_share` is above zero. The run ends at the computing
year, and it counts at most N years.

- N is a **setting**, `dividend_streak_years`. Its domain is the positive
  integers. Its preset is 10.
- **The preset's reason.** Ten years spans a business cycle with its downturn,
  which is when a dividend is cut. And ten fiscal years are inside the
  structured history of any filer that has filed XBRL since 2011, so most
  filers are measured rather than absent.
- **A run longer than N** reads as N. The metric's `meaning` says "counted to
  at most N years", and every result records the N it used.
- **The reads.** It reads the computing year, then each year before it while
  every year read so far is above zero and fewer than N have been read. It
  stops at the first of:
  - a year not above zero, which is read, and the figure counts the years
    after it
  - the N-th year
  - the first read that gives a reason
- **It counts paying years, not raising ones.** The catalogue ties the streak
  to the suspension it must measure. A raising streak would break on the
  one-period excess the catalogue hands M5, since a special dividend reads as
  a cut the next year. A paying streak cannot be broken by an excess. A
  raising streak is another metric, under a record of its own.

**A zero.** A year whose `dividends_declared_per_share` is a `Value` of zero
did not pay. That holds whether the zero was read or set by silence.

- At the computing year, the streak is `0`, a figure.
- Further back, the streak counts the years after the zero.
- How the zero was set is the history's carriage. Store holds it beside the
  result, and the derivation does not read it.
- This is what the catalogue fixed the silence zero for: a suspension "has to
  read as a zero or the streak cannot be measured". A wrong zero shortens a
  streak and never lengthens one. That is the direction the catalogue
  requires.
- A negative figure, and a zero at the growth rate's base year, are those
  metrics' own `declined` conditions.

**Quarters never enter a series.** Every member of the series is a year by the
test. The suspension fixture's quarter 2021-10-01 to 2021-12-31 reads 0 by
silence inside a year that paid 0.15. Over quarters, that would be the
suspension the filer never had, which `period-alignment.md` warned of.

**A gap.** Suppose the chain cannot find a year:

- The read at that year is `no_period` with `preceding_year`.
- Every later read the metric names is beyond the gap. The gap stands between
  it and the computing year, so it is `no_period` with `preceding_year`
  too.
- The series never steps around a gap to an earlier year. It never takes a
  non-year in a year's place, and it never closes up so the years either side
  read as consecutive.
- A missing period therefore never reads as a suspension.

**Where the result is stated.** At the computing year. Every other year is
sought from it, so it is the last of them by the rule, with nothing ordered.

### How absence composes

1. **The test first.** If the computing duration is not a year, the one
   `declined` reason above is the outcome, and nothing below is asked.
2. **The reads, in order.** The computing year's reads come first, then the
   year before's, then each further year's, in series order. Within a year,
   reads follow the metric's input order and the accepted readings: a flow at
   the year's row, and a balance at its opening, at its closing, or at both.
3. **Each read gives a number or exactly one reason:**
   - **The year it is read at was not found, or stands beyond one that was
     not:** `no_period`. Its
     `computing_at` is the computing year. Its `sought_by` is
     `preceding_year`, a `const` defined once beside the rule.
   - **The opening instant of the year before is not in the history:**
     `no_period` with `preceding_opening_instant`.
   - **The computing year's own instants** are as the accepted record says,
     with `opening_instant` and `closing_instant`.
   - **The row is there:** the concept's state at that row.
     - `Unknown` gives `input_unknown`.
     - `NotApplicable` gives `input_not_applicable`.
     - Each names the concept and the found period, through the accepted
       constructors handed the found row.
     - A `Value` outside the decimal grammar is `declined` with
       `input_not_a_decimal`.
4. **Every read is made, and none stops another.** The reasons follow the
   read order, so the computing year's own reasons stand first and a missing
   year never hides them. Identical reasons are not merged, as the accepted
   record says of two balances at one missing row. The streak is the one
   exception: its reads are defined by the figures read, so it makes no read
   past the first that gives a reason or does not pay.
5. **Only when every read gives a number** are the metric's own `declined`
   conditions asked.

`computing_at` is always the year the metric is stated at. Which step of a
series failed is not carried. The history goes to store beside the result,
and a replay shows it.

No seventh reason is needed, and no frozen byte changes.

### Where it lives

- **The test, the year-before seek and the series** are one module of the
  derivation, beside the accepted seeks. Its constants are defined there:
  `preceding_year`, `preceding_opening_instant`, `end_not_a_date`,
  `not_a_year` and `overlaps_another_year`. `start_not_a_date` stays where
  the accepted record defines it.
- **`date.rs` gains, under this record:**
  - whether a date is the last day of its month, by the month's length that
    is already in `constants/calendar.rs`
  - whether two dates are in one month with the second's year one more, read
    from the calendar fields
  - whether one date is on or before another

  The last is the ordering the accepted record says is added "here under its
  own record, and never beside it". It is used by the alone clause and by
  nothing else. It never seeks a row, never sorts, and never picks the
  latest. Stepping back 364 or 371 days is the accepted step, repeated.
- **The numbers** are seven, the days of a week; fifty-two; and fifty-three.
  They stand in `crates/analyze/src/constants/`, in a file of their own under
  the §441(f) citation. 364 and 371 are their products. The twelve-month
  clause needs no number beyond one, which is on the allowlist.
- **The two settings** are added in `crates/analyze/src/settings/`, each with
  its meaning, domain, preset and reason. The metric tasks that read them add
  them, and the rule's task does not.

### The method version

The rule is part of the method `METHOD_VERSION` names. The accepted record
places there which periods a metric reads and every rule of composition. That
covers:

- the test, its clauses and its order
- the year-before seek and the series
- the three conditions
- the two `sought_by` names
- the ordering in `date.rs`, and the constants

A later change to any of them is a version bump and a superseding record, and
never an edit to this one. A change to a setting's preset is not a bump: a
result records the value it used.

It lands at version 1. No metric reads a preceding duration or tests for a
year yet, so no stored outcome changes. The two settings arrive with their
metrics, and adding a metric is not a bump.

### What a metric's catalogue entry says

Nothing beyond the shape M5-18 fixed:

- **`stated_at`** is `Duration`.
- **`inputs`** lists the concepts, once each, as today.
- **`meaning`** says in words that the figure is over a year, and which years
  it reads. For example: "net income over a year divided by total assets at
  the instant it opens at, less the same over the year before it". The streak
  and growth rate say their setting, as "counted to at most N years".
- **`conditions`** lists `start_not_a_date`, `end_not_a_date`, `not_a_year`
  and `overlaps_another_year`, by their constants, for every metric stated
  over a year.

The `sought_by` names are not conditions, and no entry repeats them.

A `Shape::Year` would let the entry say "a year" instead of a condition. It
changes the shape M5-18 fixed, and is a record of its own if anyone wants it.

### The proof the implementing task brings

The task that implements the rule, alone or with the first metric that reads
it, brings these tests. Each is built through `History::of` and `Row::at`, and
reads no file. Each stands in `tests/` or under `#[cfg(test)]`. At the least:

- **The test.**
  - These are years:
    - 2024-01-01 to 2024-12-31
    - 2023-10-01 to 2024-09-30
    - 2023-03-01 to 2024-02-29
    - 2024-03-01 to 2025-02-28
    - 2023-10-01 to 2024-09-28, 364 days
    - 2023-01-01 to 2023-12-30, 364 days
    - 2022-09-25 to 2023-09-30, 371 days
  - These are `not_a_year`:
    - 2023-05-01 to 2023-12-31
    - 2024-07-15 to 2025-07-14, twelve months not to a month's end
    - 2023-09-01 to 2024-11-30
    - every quarter, half year and nine months in the fixtures
  - A start outside the grammar gives `start_not_a_date`, and an end outside
    it gives `end_not_a_date`.
  - At a quarter whose inputs are `Unknown`, the one reason is `not_a_year`,
    and no `input_unknown` stands beside it.
- **Two consecutive years.** A history holding 2023-01-01 to 2023-12-31 and
  2024-01-01 to 2024-12-31 finds the first as the year before the second, and
  the metric gives its change.
- **The year before missing.** A history holding 2024-01-01 to 2024-12-31 and
  not 2023 gives `no_period`, with `computing_at` the year 2024 and
  `sought_by` `preceding_year`.
- **A quarter, never the year.**
  - A history holding the year 2024 and the quarter 2023-10-01 to 2023-12-31,
    but not the year 2023, gives `no_period` with `preceding_year`. It never
    gives the quarter's figure as the year's.
  - So does the transition period 2023-05-01 to 2023-12-31 in the quarter's
    place.
- **No near year.** A history holding 2022 and 2024 and not 2023 gives
  `no_period` for 2024, never 2022's figure.
- **Alone.**
  - Beside 2022-05-01 to 2023-04-30, the year 2023-01-01 to 2023-12-31 gives
    `overlaps_another_year`, and so does the April year.
  - The year 2024 then gives `no_period` with `preceding_year`, and never the
    calendar year's figures.
- **The fiscal-year change.** The 28 durations of
  `a-filer-that-changed-its-fiscal-year`, built by their dates, give what its
  table above says at each. The 19 of `every-fact-a-filer-reported` give what
  its table says.
- **The instants of the year before.** With the opening instant of the year
  2023 missing, the reason is `no_period` with `preceding_opening_instant`.
  With the instant 2023-12-31 missing, it is `opening_instant`. Each is
  distinct where only one is missing.
- **The order.** An `Unknown` flow at the computing year, with the year
  before missing, gives `input_unknown` first, then `no_period` for each read
  at the year before.
- **The streak,** on the suspension fixture's years built by their dates and
  figures, under a stated `dividend_streak_years`:
  - at 2022, `3`
  - at 2021, `2`
  - at 2020, `1`
  - at 2019 and at 2024, `0`, the second from a silence zero
  - at 2023, `input_unknown` naming `dividends_declared_per_share` and the
    year 2023
  - at 2022 with N of 2, `2`
  - with 2021 removed, at 2022, `no_period` with `preceding_year`, once
- **The growth rate,** with n of 2 at 2022: it reads 2022 and 2020, through
  2021. With 2021 removed, each read at or beyond 2021 is `no_period` with
  `preceding_year`, and no read past the gap is answered from an earlier
  year.
- **The catalogue.** Every derivation that can give one of the four
  conditions lists it among its conditions.

### What no check can see

- **Whether §441 says what this record says.** Only the implementing task's
  reading sees it.
- **A span a filer dated as a year that is not its fiscal year, when nothing
  overlaps it.** A trailing twelve months, or a calendar tax year in a history
  with no fiscal year beside it, is read as a year. A dividend silent there
  reads zero, and the streak there is `0`.
- **A filer-dated year that holds another period's figures, when nothing
  overlaps it.** The fixture's calendar year 2023 is caught only because the
  April year overlaps it. Take the April year away and it is the year before
  2024, holding the transition period's pretax income. This is
  `period-alignment.md`'s "a period the filer dated is a period the filer
  meant", met again one row back.
- **A span of 364 or 371 days that is not a 52- or 53-week year.**
- **A 53-week year compared with a 52-week one.** The longer year carries
  one fifty-second more of every flow, and the change signals read it as
  growth.
- **A metric that finds its year itself,** for example by taking the latest
  year that ends before its start. `History::periods()` is public, and only
  review sees a derivation that skips the rule.
- **Which step of a series failed.** The reason does not carry it, and a
  replay is needed to see it.
- **A capped streak.** Any run of N years or more reads N. The meaning says
  so, and nothing checks that a reader reads it.
- **A duration whose dates are outside the grammar.** It is never a year, and
  never counts against another year in the alone clause.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The three
  numbers stand in `constants/`, one of its three places, and the two presets
  in `settings/`, another.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and catalogue
- the arithmetic record's reading, operations and written form, since no date
  enters the number type
- the period record's readings, its seeks, its date reading and its
  `start_not_a_date`
- the period-alignment record's three rules, since nothing here labels a
  period in the history or reaches normalize
- the frozen bytes of `canonical-concepts` v2, `fetch-normalize` v2 and
  `analyze-store` v1
- the entry shape M5-18 fixed

No question here needed one of them to change.

**Why Structural.** The record fixes the characters of two `sought_by` values
and three `declined` conditions, which cross `analyze-store` v1. It also fixes
a rule that every metric stated over a year shares. It needs nothing
Constitutional: no fourth place, no allowlist value, no gate weakened and no
milestone moved.

## Alternatives

- **The duration that ends the day before the start, whatever its length.**
  It needs only equality, and no test. Rejected:
  - Before the year 2024 in the fixture, two durations end on that day, the
    transition period and the calendar year, and choosing between them is the
    question itself.
  - For a calendar filer, the duration that most often ends there is the
    fourth quarter or the half year. Its figure would be compared with a
    year's as if it were the year's.
- **A length class from the day count,** such as 360 to 371 days, or 365 with
  a tolerance. Rejected:
  - The window and its tolerance are numbers no source fixes.
    `period-alignment.md` refused them for that reason.
  - The two clauses here count days only where §441(f) fixes the count,
    exactly, and otherwise read months.
  - A window also admits a twelve-month span that ends mid-month. No fiscal
    year does that.
- **A fiscal year end carried from the filings in a later vocabulary.**
  Rejected:
  - The one carrier on the boundary is the submissions document's
    `fiscalYearEnd`. It holds the current value only. For the fixture filer
    that is `1231`, which makes both of its April years `not_a_year`.
  - For a 52- or 53-week filer, the year end moves from year to year, so one
    value misses most of its years.
  - `dei:CurrentFiscalYearEndDate` is not a number, so company facts never
    carries it.
  - Carrying it is `fetch-normalize` v3, `canonical-concepts` v3 and
    `analyze-store` v2.
  - The fixture's calendar year 2023 ends on `1231`, so it would read as a
    year. This rule's alone clause keeps it out.
- **The form each period was reported on, carried per period.** Rejected:
  - Rule 3 sets each value from the latest filing that states it. So a
    period's values come from several filings, of several forms, and a period
    has no one form.
  - In the fixture, the 10-K and 10-K/A of 2025 answer the transition period,
    the calendar year 2023, the April year and the year 2024 alike. "Reported
    on a 10-K" separates none of them.
  - Nothing guarantees that a transition period is filed as a 10-KT rather
    than a 10-K.
  - It is `canonical-concepts` v3 and `analyze-store` v2.
- **The span between consecutive annual reports' period ends,** read from
  `report_period_end` and form. It would keep the fixture's calendar year 2023
  out without the alone clause. Rejected:
  - Where a transition period is filed as a 10-K, it reads the transition
    period as a year, and that is the wrong year this record exists to
    refuse.
  - It needs the same two contract versions as the form above.
- **Every year-over-year metric left out of M5.** Rejected:
  - GOALS.md's fourth and fifth criteria name these metrics, so deferring them
    is a change of milestone scope, which is Constitutional.
  - The dates answer the question with no wrong year in any fixture, at the
    cost of absences named above.
- **No alone clause,** with every year-length span a year. It is simpler, and
  needs no ordering. Rejected: in the fixture, the year before 2024 would be
  the calendar year 2023, which holds the transition period's pretax income
  and tax under twelve-month dates. That is a plausible wrong year, in the
  fixture this record must answer.
- **The transition period, or the last year on the old calendar, as the year
  before the first new year.** Rejected:
  - The transition period is eight months, and comparing a year with it
    reads four months of flow as growth.
  - The last old year ends eight months before the new one starts. Finding it
    needs "the latest year that ends before", which is the nearest-row reading
    the accepted record refused.
- **The k-th year back named by stepping the calendar back k years,** so each
  member of a series is named independently and a gap gives one reason per
  missing year with later years still read. Rejected:
  - A 52- or 53-week filer's years do not fall on the same dates a year apart,
    so every one of its series would be `no_period`.
  - Across a fiscal-year change it names dates the filer never used.
- **An uncapped streak, absent where the history runs out before a break.**
  Rejected: structured histories start in 2009 to 2011, so every filer that
  has paid throughout, which is the case a streak exists to show, would be
  absent.
- **The streak as a lower bound where the history runs out.** Rejected: the
  figure would mean "exactly k" for one filer and "at least k" for another,
  under one name.
- **A raising streak.** Rejected for this metric: a special dividend in one
  year reads as a cut in the next, which is the excess the catalogue hands M5
  to tolerate. It may be another metric, under a record of its own.
- **The series over quarters, or over every duration.** Rejected: in the
  suspension fixture, a quarter reads 0 by silence inside a year that paid
  0.15. That would be a suspension the filer never had.
- **`no_period` for a duration that is not a year.** Rejected: no row is
  missing, and "the year before" a quarter is not a row anyone could add.
- **Every read made at a duration that is not a year,** with `not_a_year`
  beside the inputs' reasons. Rejected: an `Unknown` input at a quarter is not
  why Altman's score is absent there. Reporting it would ask someone to fix a
  mapping that cannot make the metric present.
- **One `sought_by` name for both openings.** Rejected: with one of the two
  opening instants missing, the reason would not say which.
- **`Shape::Year` in the entry.** Rejected here: it changes the shape M5-18
  fixed. It is left to a record of its own.

## Consequences

**Easier:**

- Altman's and Piotroski's scores, the five change signals, the growth rate
  and the streak can be built. Each reads years under one rule, and none
  chooses.
- A quarter, a half year, a year-to-date span and a transition period never
  enter a comparison or a series as a year.
- A missing year reads as `no_period`, never as a suspension.
- The only new thing in `date.rs` is one ordering, used in one place.

**Harder:**

- The first year of every history has no year before. Neither does the first
  year after every fiscal-year change, so a change costs the comparisons and
  series across it. Where the filer also dated a recast calendar year, the
  last old year is lost to `overlaps_another_year` too.
- Every metric stated over a year is absent with `not_a_year` at every
  quarter, half year and year-to-date span. That is more than four in five of
  the durations in each fixture.
- A stray year-long span that reaches a history costs the real years it
  overlaps.
- A growth rate over n years needs n + 1 consecutive years. A streak shorter
  than N needs a break inside the history.
- The derivation reads every duration of the history to test one.

**Expensive to reverse:**

- What a year is, and the alone clause, once results are stored under them.
- The two `sought_by` names and three conditions, once stored.
- The paying form of the streak, once ranked on.

**Open, and not this record's:**

- Each metric's formula, including how the growth rate tolerates a one-period
  excess.
- A raising streak, if anyone wants one.
- Whether a contract should carry the span of each annual report. That would
  be the remedy if the corpus shows filer-dated years without an overlap
  often enough to matter.
- A `Shape::Year` in the entry.

## Enforcement

This applies anchors 4 and 5, and edits neither.

- **Anchor 4.** The test, the seek and the ordering read only the characters
  they are handed, with no clock and no dependency. So `purity` and `ambient`
  read what they read today.
- **Anchor 5.** `literals` keeps the week's numbers out of the derivation, and
  they stand in `constants/` under §441(f). The two presets stand in
  `settings/` with their reasons.
  - **Gap:** nothing checks that a citation is true.
- **No near year, and no quarter for a year.** The proof's tests hold the
  rule.
  - **Gap:** nothing forces a metric through the rule rather than around it.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the characters of
  two `sought_by` values and three `declined` conditions, fields
  `analyze-store` v1 carries as open strings, and a rule every year-stated
  metric shares, all inside analyze. It edits no anchor, no protected path,
  no gate, no contract byte and no milestone scope, asks for no allowlist
  entry, and adds no setting of its own. Flagged for later human review, as
  the tier requires.
- **Checked:**
  - `which-other-period-a-metric-reads.md`: it leaves the preceding duration
    and the series "to a record of their own" that reads dates through
    `date.rs`, and says ordering "adds it here under its own record". This
    record is that one, adds the ordering there and nowhere else, and keeps
    `start_not_a_date` where the accepted record defines it.
  - `what-analyze-takes-and-returns.md`: the six reasons stand and no seventh
    is added. `no_period` carries `computing_at` and `sought_by`;
    `declined` names a condition per the catalogue entry; a metric read over
    several periods is stated at the last of them by its own rule, which is
    the computing year here. The two settings follow its settings section:
    name, domain, preset and reason, in `settings/`.
  - `analyze-store` v1: `sought_by` and `condition` are strings the
    contract leaves to analyze, and settings cross by name, so no byte moves.
  - `period-alignment.md`: it refused a day count and left the year a
    decision; this record counts days only where §441(f) fixes the count and
    otherwise reads months, and labels nothing in the history. Its warning
    that a missed period reads as a suspension is met by `no_period` at a
    gap and by keeping quarters out of a series.
  - `bare-literal-lint.md`: seven, fifty-two and fifty-three go under
    `constants/` and the two presets under `settings/`, two of the three
    places; nothing on the allowlist moves.
  - The fixtures, read from `fixtures/normalize/*/expected`:
    `a-filer-that-changed-its-fiscal-year` holds the 28 durations the table
    lists, and its calendar year 2023 carries the transition period's pretax
    income under twelve-month dates, so the alone clause is what keeps a
    wrong year out of the fiscal-year change. `every-fact-a-filer-reported`
    holds 19 durations in the history. The suspension fixture's calendar
    years read as the Context states them.
  - The two length clauses against the proof's cases: each listed year
    satisfies one clause, and 2024-07-15 to 2025-07-14 fails both.
  - The alternatives as argued: the day-before duration of any length, a
    length class, a carried fiscal year end, a per-period form, the span
    between annual reports, leaving the metrics out of M5, no alone clause,
    the stub or the old year across a change, calendar stepping, an uncapped
    streak, a lower bound, a raising streak, a series over quarters,
    `no_period` for a non-year, every read at a non-year, one opening name,
    and `Shape::Year`.
- **Verdict and why:** accepted. The record answers the question both
  accepted records refused in passing, and answers it in the one way that
  gives no wrong year in any fixture: a year is read from its own dates under
  a cited statute, the year before is found by equality and never by nearness,
  and where two year-length spans overlap, neither is taken. Every path to a
  plausible wrong year — the quarter that ends the day before, the transition
  stub, the recast calendar year, the nearest old year, a calendar-stepped
  series — ends in absence, which the accepted records already make the safe
  side. Counting the streak in paying years rather than raising ones is what
  makes the catalogue's one-period excess tolerable, and the settings' presets
  carry stated reasons where no source fixes a window. The cost is real and
  stated: a first year after a fiscal-year change has no year before, and a
  stray year-long span costs the real years it overlaps.
- **What would have changed it:** a seventh reason, a changed byte of
  `analyze-store` v1 or `canonical-concepts` v2, a date ordered anywhere but
  `date.rs`, a number for the week or the year anywhere but `constants/`, or a
  reading under which a quarter, a stub or a near year was ever taken as the
  year before. So would a §441 reading that, on the implementing task's own
  reading, fixes a year otherwise than the two clauses say; the record makes
  that a stop, and that stop is the first item flagged for the human. The
  second is the residual the record names and no check sees: a filer-dated
  year holding another period's figures with nothing overlapping it is read
  as a year, and only the corpus can say how often that happens.
