# The dividend growth rate is the compound rate over `dividend_growth_years` years, read at the computing year and at the year that many years before it and at no year between; an excess between them moves nothing, and one at either end moves the rate through the n-th root, up at the computing year and down at the base year

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-20, by `M5-42`
- **Decided:** 2026-10-22, by the decider
- **Touches:** the dividend growth rate, which GOALS.md's fifth M5 criterion
  names and which `which-duration-precedes-a-duration.md` (Accepted) leaves
  to "that metric's task": which of its n + 1 years it reads, its formula,
  and how it tolerates the one-period excess that answer 6 of
  `canonical-concepts-open-questions.md` (Accepted) says "lands on M5". It
  fixes the name and meaning of one metric, which cross `analyze-store` v1 as
  characters. It applies the accepted `declined` reason to six conditions:
  four that the series record already defines, the streak's
  `negative_dividends_declared_per_share`, and one new one. It adds no
  reason. It takes three operations from the arithmetic record's table and
  none from outside it. The setting `dividend_growth_years` is the series
  record's, and arrives with the metric. It puts no number in `constants/`.
  No accepted record is superseded. No contract's bytes, no schema, no edge,
  no gate, no dependency and no entry field move. Nothing here is
  implemented.

## Context

GOALS.md's fifth M5 criterion names the dividend growth rate beside the
streak. The streak is built (M5-37). The rate is not, because its formula
was left open by name.

Some things are already fixed, and this record decides nothing of them. It
restates them as fixed:

- **The window.** `which-duration-precedes-a-duration.md` (Accepted), built
  by M5-36 in `crates/analyze/src/metrics/reading/year.rs`, fixes these:
  - The rate reads n + 1 years: the computing year and the n years before
    it.
  - n is the setting `dividend_growth_years`. Its domain is the positive
    integers, and its preset is 5. The preset's reason is the record's: "Five
    annual steps hold five dividend decisions", so one year's special
    dividend or one skipped raise "moves the rate through one step of five
    rather than through the whole rate". Also, "six fiscal years are inside
    the structured history of any filer that has filed XBRL since the 2011
    phase-in".
  - The n + 1 years are the computing year and the year-before seek repeated
    from it. M5-36 names them `year`, `Year::series` and `PRECEDING_YEAR`.
  - The result is stated at the computing year.
  - The rate "reads at least the first and the last" of the years, and
    "every read it names is made".
  - A duration that is not a year carries the metric absent under
    `start_not_a_date`, `end_not_a_date`, `not_a_year` or
    `overlaps_another_year`, the first that applies, and nothing is read.
  - That record's own proof already pictures the rate this way: "with n of 2
    at 2022: it reads 2022 and 2020, through 2021."
- **The excess.** v2's `dividends_declared_per_share` is "regular and special
  together". Answer 6 of the open-questions record refuses the split, because
  "the fetch → normalize contract carries no axis and no member". It also
  says that "inferring 'special' from the size of a period's figure would be
  the guess this milestone exists to prevent", which is GOALS.md's M4
  "Nothing is ever guessed". It hands M5 the task that "the streak and the
  growth rate must be defined so that a single-period excess which does not
  recur is not read as a cut".
- **A suspension is a zero.** Under the series record, a year whose figure
  is a `Value` of zero did not pay, however the zero was set. That record
  makes "a zero at the growth rate's base year" and "a negative figure" this
  metric's own `declined` conditions.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) computes in exact rationals over a closed table of operations.
  It names an n-th root as what "a compound growth rate over n years needs".
  It takes the root's index from "a constant …, from a setting, or from a
  count", and never from a literal. It excludes a negative radicand and a
  zero index. Only a root rounds inside the derivation.
- **Absence.** `what-analyze-takes-and-returns.md` (Accepted) closes the six
  reasons and orders them by input. The series record orders them by year:
  the computing year's reads first, then each further year's, in series
  order.

Two words need fixing here, because the records use "first" both ways. In
series order the computing year is first. The acceptance and the open-
questions record count by time, and so does the source below, where the
first observation is the earliest. This record says:

- **the computing year**, the year the rate is stated at, which is the latest
  in time and first in series order
- **the base year**, the n-th year before it, which is the earliest in time
  and last in series order. This is the series record's "base year". It is
  also the acceptance's "first year".
- **a year between**, any of the n − 1 years the series finds between them

## Decision

### The formula

The rate is the **compound rate over n years**: the computing year's figure
divided by the base year's figure, its n-th root taken, less one.

> rate = (D_computing ÷ D_base)^(1/n) − 1

Here D is `dividends_declared_per_share` at the year named, and n is
`dividend_growth_years` read from the `Settings` the call was handed.

The arithmetic record supplies three operations from its table, and the
formula takes no others:

| Step | Operation, as the table names it |
| :--- | :--- |
| D_computing ÷ D_base | division. A zero divisor is the metric's own `declined` condition, below. |
| its n-th root | `k`-th root, with `k` the positive integer n, of a radicand not below zero |
| less one | subtraction, of one, which is on the allowlist |

Comparison and sign, from the same table, ask the conditions.

**The root's index** is n read from the setting, as the arithmetic record
places an index. It is never a literal and never a count read off the
history. The record's two exclusions cannot arise:

- **A zero index.** The setting's domain is the positive integers. The
  `Settings` constructor refuses zero as the caller's error, before analyze
  runs, as the series record and the analyze record fix.
- **A negative radicand.** The root is taken only once the conditions below
  have passed. Then D_base is above zero and D_computing is not below zero,
  so their quotient is not below zero. Every way the radicand could be
  negative is a negative figure at a year read, which is
  `negative_dividends_declared_per_share` and is asked first.

So the entry lists neither exclusion under a name of its own. The arithmetic
record says a metric that takes a root "lists a negative radicand and a zero
index among its own conditions, as it lists a zero divisor". This metric
meets that sentence the way every metric meets the zero divisor: it names the
input condition under which the formula is undefined. A zero divisor is
`zero_base_year_dividends_declared_per_share`. A negative radicand is
`negative_dividends_declared_per_share`. A zero index is outside the
setting's domain. The conditions are asked before the root, so the root is
never handed a value it excludes, and the implementing task treats an
excluded root as unreachable, not as an absence.

**Rounding.** The root rounds, to `SIGNIFICANT_DIGITS` significant digits,
ties away from zero, as the arithmetic record says a root does. Subtracting
one is exact. The one writer then writes the difference.

- Where the root is at least one tenth, which is every rate from −0.9 up,
  the difference has no more significant digits than the root had. So it is
  written exactly, and the figure is the rounded growth factor less one.
- Below that, the difference can carry more digits, and the writer rounds
  it under its own rule. This is the arithmetic record's root followed by
  its writer, and nothing is written and read back.
- Where the rate is near zero, it carries fewer significant digits than the
  factor had (see What no check can see).
- The root rounds even at n of one, which the proof pins.

**Why this form.**

- It is the form the arithmetic record names for this metric.
- It is a rate per year that compounds back to the figures it was read from:
  D_base × (1 + rate)^n is D_computing, up to the root's rounding. It is
  therefore the rate a reader can carry forward.
- It reads two figures, so an excess at a year between moves nothing (below).
- The source states it as the better reading of past growth, and states why
  the arithmetic mean is worse (Source, below).

### Which years are read

**Two reads.** The rate reads `dividends_declared_per_share` at the
computing year, then at the base year. These are the reads it names, and
each is always made.

**The years between are found and not read.** The base year is reached only
through `Year::series`, the year-before seek repeated n times. So each year
between must be found as a year by the series record's test, or the base
year is not reached at all. Its figure is not read. An `Unknown`,
`NotApplicable` or unreadable figure at a year between is not a reason, and
it does not make the rate absent.

**Nothing is skipped and nothing is substituted.**

- No year is stepped around. A year the chain cannot find stands between the
  computing year and the base year, so the base year's read is `no_period`
  with `PRECEDING_YEAR`, as the series record says of every read beyond a
  gap.
- No window shorter than n is read where a year is absent. The rate is never
  taken over the years the chain did find.
- No figure stands in for a figure. An absent or zero base year is never
  replaced by a nearer or later one.

### The one-period excess

Let the regular dividend grow at a steady g, and let one year's figure carry
a special of e times its regular dividend, with e above zero. The rate gives
the following, depending on which year carries it:

| The year carrying it | What the rate gives |
| :--- | :--- |
| a year between | g, exactly. The figure is not read. |
| the computing year | (1 + g) × (1 + e)^(1/n) − 1. It is above g: the excess reads as growth. |
| the base year | (1 + g) ÷ (1 + e)^(1/n) − 1. It is below g: the excess reads as slower growth, or as a decline. |

**As the window rolls.** An excess in year t is at the computing year when
the rate is computed at t. It is between at t + 1 through t + n − 1, and at
the base year at t + n. After that it is outside the window. So of the n + 1
rates whose window holds it, it moves exactly two, once up and once down, each
by the n-th root of 1 + e. At the preset, a special equal to the regular
dividend moves the growth factor by 2^(1/5), which is about 1.149. A 10%
grower reads about 26% at the year of the special and about −4% five years
later, and 10% at the four years between.

**Where tolerance comes from.**

- **At a year between, tolerance is a property of the form.** The compound
  rate does not read the year, so no excess there can be read as a cut or as
  a raise.
- **At either end, tolerance is not reachable at v2.** It would need the
  dimensional split, which answer 6 says does not cross. Any form reads the
  end years' figures, since the rate is the change between them. Those
  figures are the total, regular and special together. No arrangement of
  the end years tells a special from a raise without the split.
- **The direction of the error is named.**
  - At the computing year it overstates the rate. That is the flattering
    direction, the one answer 6 warns of for the valuation.
  - At the base year it understates the rate. That is the conservative
    direction. A large enough excess there reads as a cut, which answer 6
    says must not happen, and this record says plainly it does at that one
    year of the window's n + 1.
- **A form that reads more years** cannot remove the end-year error either.
  It can only spread it, and each such form fails on its own terms (see
  Alternatives).

**Marking a year as special by the size of its figure** is rejected. Any
rule that says a figure is "too high to be regular" is the inference from
magnitude GOALS.md's M4 forbids, in answer 6's words "the guess this
milestone exists to prevent". The same goes for capping a year's change,
trimming the largest figure, or reading a year against its neighbours.

The same table holds for a timing shift: a board that declares a quarter's
dividend in December rather than January puts five declarations in one year
and three in the next. It is an excess like any other, read the same way.

### A zero, and a negative

- **A zero at the base year** is `declined` under
  `zero_base_year_dividends_declared_per_share`. The quotient has a zero
  divisor, and the source says the compound rate "is not meaningful" there.
  A filer that paid nothing at the base year has no rate of growth from it,
  however much it pays now.
- **A zero at the computing year** gives the figure **`-1`**, not a
  condition. The quotient is zero, and the root of zero is zero, which the
  table admits. So the rate is minus one at every n. That figure is the
  formula's exact value: a dividend that fell to nothing fell by all of
  itself.
  - It is the unflattering direction. A silence zero there may be wrong, and
    then the rate reads a cut that was never made. That is the direction the
    catalogue requires a wrong zero to err in, as it does for the streak.
  - A condition would hide a suspension behind an absence. The streak shows
    the same year as `0`, and the two then agree.
- **A negative figure** at either year read is `declined` under
  `negative_dividends_declared_per_share`. This is the streak's condition,
  with the same characters and meaning: "a year read declared a negative
  dividend per common share". It is defined once, where M5-37 put it, and
  the rate reaches that one definition. It never defines a second. A
  negative quotient has no real root the table takes. Two negatives have a
  positive quotient, which is no growth rate of a dividend.

**Their order.** The conditions are asked only once both reads give a
number. There is one reason, the first that holds:

1. `negative_dividends_declared_per_share`, where either figure is below
   zero
2. `zero_base_year_dividends_declared_per_share`, where the base year's
   figure is zero

A negative figure goes first because it makes the definition undefined
whatever the other figure is. A zero base year with a negative computing
year is therefore `negative_dividends_declared_per_share`. A zero at both
years is `zero_base_year_dividends_declared_per_share`.

`zero_base_year_dividends_declared_per_share` is a `const` defined once
beside the metric. It is new, and distinct from
`dividend_coverage_per_share`'s `zero_dividends_declared_per_share`. That
condition is a zero at the year the metric is stated at, and this one is a
zero at another year. The same characters for both would say the wrong year.

### How absence composes

This is the series record's order, applied to two reads. It adds no reason.

1. **The test first.** If the computing duration is not a year, the outcome
   is the one `declined` reason the series record gives: `start_not_a_date`,
   `end_not_a_date`, `not_a_year` or `overlaps_another_year`. Nothing is
   read.
2. **The computing year's read.** A number, or one reason:
   - `input_unknown` or `input_not_applicable`, naming
     `dividends_declared_per_share` and the computing year, through the
     accepted constructors handed the found row
   - `declined` with `input_not_a_decimal`, where the characters are outside
     the grammar
3. **The base year's read.** A number, or one reason:
   - `no_period`, where the series did not reach the base year: the base year
     itself was not found, or it stands beyond a year that was not. Its
     `computing_at` is the computing year, and its `sought_by` is
     `PRECEDING_YEAR`. There is one, however many years are missing.
   - `input_unknown` or `input_not_applicable`, naming the concept and the
     base year, the found period
   - `declined` with `input_not_a_decimal`
4. **Every read is made, and neither stops the other.** The computing year's
   reason stands first, and a missing year never hides it. Where both reads
   fail, both reasons show, in that order.
5. **Only when both reads give a number** are the two conditions above
   asked.

The years between give no reason of their own. They are not read. A year
missing among them shows as the base year's `no_period`, which is the read
that stands beyond it. So every year the rate reads is read, and every
missing one that keeps a read from being made shows.

**The series must stop at the gap.** `Year::series` yields `no_period` for
every place after the chain ends, and n can be as large as the setting's
type holds. The implementing task reaches the base year without stepping to
n once the chain has ended, and without overflow at n + 1. The proof pins
this.

### The catalogue entry

The streak's form is enough, and the entry needs nothing beyond the shape
M5-18 fixed:

- **`name`**: `dividend_growth_rate`.
- **`meaning`**: in words, that the figure is the compound rate per year at
  which the dividend declared per common share grew over
  `dividend_growth_years` years, ending at this year. It is this year's
  figure divided by the figure of the year `dividend_growth_years` years
  before it, to the root of `dividend_growth_years`, less one. It is stated
  over a year, and reads this year and that year, finding each year between
  and reading none of them. The setting is named in the meaning, as the
  streak names `dividend_streak_years`.
- **`unit`**: `FractionOfOne`, a rate per year as a fraction of one, so a
  10% grower is `0.1` and never `10`. The arithmetic record writes a rate
  that way.
- **`stated_at`**: `Duration`.
- **`inputs`**: `dividends_declared_per_share`, once.
- **`conditions`**, in this order:
  - `start_not_a_date`
  - `end_not_a_date`
  - `not_a_year`
  - `overlaps_another_year`
  - `negative_dividends_declared_per_share`
  - `zero_base_year_dividends_declared_per_share`
  - then `input_not_a_decimal`, generated from its one definition

The setting it reads is recorded in every result's premises by name and
value, as every setting is. The entry carries no field for it.

### The setting

`dividend_growth_years` is added in `crates/analyze/src/settings/`, in a file
of the metric's own. It carries:

- the meaning: how many years the rate compounds over, ending at the year it
  is computed at
- the unit: years
- the domain: the positive integers, never clamped
- the preset: 5
- the preset's reason: the series record's reason, quoted in Context

The task that builds the metric adds the setting, as the series record says.
A change to the preset is not a version bump.

### No number in `constants/`

The formula needs no constant. The index is the setting, "less one" uses the
allowlist's one, and every year is found by the seek. Nothing is added under
`crates/analyze/src/constants/`.

### The method version

The formula, the choice of the two years read, the two conditions and their
order, and the composition above are part of the method `METHOD_VERSION`
names. The analyze record places "a metric's formula", "which periods or
dates a metric reads" and "a rule of absence or composition" there. A later
change to any of them is a version bump and a superseding record, and never
an edit to this one.

It lands at version 1. The rate is not computed yet, so no stored outcome
holds it, and adding a metric is not a bump.

### The dividend discount valuation

This rate is not made the valuation's growth. Whether the dividend-based
valuation that GOALS.md's fifth criterion names reads this rate, and how,
is that valuation's own record. This record decides nothing of it.

It states what that record will find here. The rate reads two declared
totals, so an excess at the computing year raises it. Answer 6 says the
valuation "must not take one period's declared total as a perpetuity". A
rate moved by one period's total and carried into a perpetuity takes that
excess there through the rate.

### Source

**The form is Damodaran's geometric average.** Damodaran, A., *Investment
Valuation*, 2nd ed., John Wiley and Sons, 2002, chapter 11, "Estimating
Growth".

- **The copy opened.** The author's own copy of the chapter, at
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch11.pdf>. It was
  opened in this run. Its sha256 is
  `df01bdb8fc7706c1038c62b04aaebb7878f6b66acca85eeef3c8297020a44bf2`, and it
  has 52 pages with printed numbers matching.
- **The book.** The file prints its chapter number and title, but not the
  book's. The author's entry page,
  <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/Inv2ed.htm>, opened
  in the run, says it is "designed to support "Investment Valuation", the
  second edition", and that "The publisher is John Wiley and Sons." The year
  is not printed on either page opened, and is named here as the book is
  published.
- **Page 3**, "Arithmetic versus Geometric Averages": "the geometric mean
  takes into account the compounding that occurs from period to period". The
  geometric average is given as earnings now over "earnings in n years ago",
  to the power one over n, less one. "The geometric average is a much more
  accurate measure of true growth in past earnings, especially when
  year-to-year growth has been erratic."
- **Page 4**: "The geometric mean considers compounding but focuses on the
  first and the last earnings observations in the series - it ignores the
  information in the intermediate observations and any trend in growth rates
  that may have developed over the period."
- **Page 7**: "If the EPS in the initial time period is negative or zero,
  the geometric mean is not meaningful."

The quotations are as printed. The formula on page 3 is set as a fraction
with a superscript, so it is given here in words rather than quoted.

**What the source does not fix.** Damodaran states the form for earnings,
and says it "also applies to revenues". He does not state it for dividends.
No source opened in the run fixes the form for a series of dividends per
share. So applying the geometric average to `dividends_declared_per_share`
is this record's own decision. The reason is that a dividend per share is a
per-share flow stated once a year, like the earnings per share the source
reads. On page 4 the source counts "focuses on the first and the last" as a
cost. Here it is the reason for the choice: n − 1 of the n + 1 years are
between, four of six at the preset, and an excess at any of them moves
nothing. The window n is the series record's setting, which no source
fixes.

The page-7 sentence also stands behind
`zero_base_year_dividends_declared_per_share`, and behind declining a
negative base. The source does not reach a zero at the computing year. The
`-1` there is this record's reading of its formula, with the reason given
above.

### The proof the implementing task brings

Each test is built through `History::of` and `Row::at`, reads no file, and
stands in `tests/` or under `#[cfg(test)]`. Years are calendar years named by
their dates, and n is stated in each case. At the least:

- **The preset, by hand.** n is 5. The years 2019 to 2024 read 1, 1.1, 1.21,
  1.331, 1.4641 and 1.61051. At 2024 the quotient is 1.61051, whose fifth
  root is exactly 1.1, so the figure is `0.1`.
- **A special at a year between.** The same, with 2022 at 2.662, the regular
  1.331 doubled. The figure is still `0.1`. With 2022's figure `Unknown`
  instead, the figure is still `0.1`, and no reason names 2022.
- **A special at the computing year.** The same, with 2024 at 2.48832, the
  regular 1.61051 and a special of 0.87781. The quotient is 1.2 to the fifth,
  so the figure is `0.2`: the excess reads as growth.
- **A special at the base year.** The same, with 2019 at 1.61051, the regular
  1 and a special of 0.61051. The quotient is 1, so the figure is `0`.
- **Each condition.**
  - A negative figure at 2024 gives `negative_dividends_declared_per_share`,
    and so does one at 2019.
  - A zero at 2019 gives `zero_base_year_dividends_declared_per_share`, and
    so does a zero at both years.
  - A zero at 2019 with a negative 2024 gives
    `negative_dividends_declared_per_share`, alone.
  - A zero at 2024 with 2019 at 1 gives the figure `-1`.
  - Characters outside the grammar at 2024 and at 2019 give
    `input_not_a_decimal` twice, in that order.
- **A year missing inside the window.** The preset history without 2021 gives
  `no_period` at 2024, with `computing_at` 2024 and `sought_by`
  `preceding_year`, once. It is never the rate over 2024 back to 2022. With
  2024's figure also `Unknown`, the reasons are `input_unknown` naming
  `dividends_declared_per_share` and 2024, then that `no_period`.
- **A quarter or a stub in a year's place.** With 2021 replaced by the
  quarter 2021-10-01 to 2021-12-31, the outcome at 2024 is the same
  `no_period`. At the quarter itself the one reason is `not_a_year`.
- **The suspension the bank fixture carries.** Take the calendar years 2018
  to 2024 of `a-filer-that-suspended-its-dividend`, by their dates and
  figures: 0, 0, 0.09, 0.15, 0.12, `Unknown` and 0. The zero at 2024 is set
  by silence and reads as the zero the series record says it is. With n of 2:
  - at 2018 and 2019, `no_period` with `preceding_year`, once each
  - at 2020 and 2021, `zero_base_year_dividends_declared_per_share`
  - at 2022, the quotient 0.12 ÷ 0.09 is four thirds, and the figure is
    `0.154700538379251529018297561003915`. That is the square root of four
    thirds rounded to 34 significant digits, 1.154700538379251529018297561003915
    (the next digits are 491…), less one.
  - at 2023, `input_unknown` naming `dividends_declared_per_share` and 2023,
    alone
  - at 2024, `-1`, from the silence zero
  - With n of 1, at 2022 the figure is `-0.2`. At 2021 it is
    `0.666666666666666666666666666666667`, the index-one root of five thirds
    rounded to 34 significant digits, less one, which is one digit shorter
    than two thirds written alone.
  - Under the preset, at 2024 the outcome is
    `zero_base_year_dividends_declared_per_share`, from 2019. 2023's
    `Unknown` is between and is not read.
- **A window far past the history.** The fixture years with
  `dividend_growth_years` at the largest value its type holds give one
  `no_period` at 2024, and the call returns without overflow.
- **The setting.** The preset is 5, zero is refused and never clamped, and
  each admitted value is written back as the value used.
- **The catalogue.** The entry states the name, the unit, `Duration`, the one
  input, the meaning's words for the compound rate, the two years read and
  the setting, and the conditions in the order above, with
  `input_not_a_decimal` last.

### What no check can see

- **Whether Damodaran says what this record says.** Only a reading of the
  copy sees it, and the quotations are given so it can be checked.
- **A special dividend, or a declaration moved across a year end, at either
  end of the window.** The rate is moved by it as the table says, and
  nothing in the history marks it. The figure looks like any other rate.
- **The path between the ends.** A suspension, a cut or a spike at a year
  between does not show in the rate. A filer that cut to zero and restored
  the dividend inside the window reads the same rate as one that never cut.
  The streak reads the path, and the rate does not.
- **A wrong silence zero.** At the computing year it gives `-1`, a cut never
  made. At the base year it gives an absence. Neither flatters.
- **The digits of a small rate.** The root rounds the growth factor to 34
  significant digits, so a rate near zero carries fewer. A rate of 10^-4
  carries about 30.
- **A derivation that reads the years between,** or takes the rate over a
  shorter window when the chain breaks. Only the proof's tests hold this, and
  review.
- **A wrong year** that the series record cannot see either: a filer-dated
  year that holds another period's figures with nothing overlapping it. That
  record names it, and it reaches this metric unchanged.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  formula needs no number. The preset stands in `settings/`, one of the three
  places.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and
  catalogue
- the arithmetic record's reading, operations, rule for a root and written
  form. The rate uses three operations of the table and its root's index is a
  setting, as that record places it.
- the period record's readings and seeks
- the series record's year test, seek, setting and names. The rate reads two
  of its n + 1 years, which that record leaves to this one. Its gap rule
  gives the base year's `no_period`.
- the frozen bytes of `canonical-concepts` v2 and `analyze-store` v1
- the entry shape M5-18 fixed

No question here needed one of them to change.

**Why Structural.** The record fixes the characters of a metric's name and
of one new `declined` condition, which cross `analyze-store` v1 and which
store and M6 will read. It also fixes a formula under the method version. It
needs nothing Constitutional: no anchor, no fourth place, no allowlist value,
no gate weakened, and no milestone moved.

## Alternatives

- **The arithmetic mean of the n year-over-year changes.** Rejected:
  - It reads every year, so an excess between is read twice, up then down,
    and the two do not cancel. Up by (1 + g)(1 + e) − 1 and down by
    (1 + g) ÷ (1 + e) − 1 sum to more than 2g whenever e is not zero. In the
    preset case with 2022 doubled, the changes are 0.1, 0.1, 1.2, −0.45 and
    0.1. Their mean is `0.21` against the compound rate's `0.1`. The excess
    reads as growth at every window that holds both changes, which is the
    flattering direction at n − 1 windows rather than at one.
  - A zero at any year but the computing year is a zero divisor in the next
    change. So every window that spans a suspension is absent, even where the
    dividend was restored and grew.
  - The source prefers the geometric average as "a much more accurate measure
    of true growth", and the arithmetic record names the root for this
    metric.
- **Reading every year, by a form that weighs them all.** Rejected:
  - The source's log-linear regression needs a logarithm, which the
    arithmetic record's table does not hold. Adding it would be a superseding
    record.
  - A linear regression of the figure on time, divided by the mean figure,
    stays inside the table. But it is a linear rate and not a compound one,
    and n + 1 reads make an `Unknown` at any year an absence. It also spreads
    an excess at every year into every window that holds it.
  - A median of the changes is not in the source. At n of 1 or 2 it is not
    robust. Halving an even count needs a number the derivation cannot cite.
  - None of them removes the end-year error, because the ends are read in
    every form. Each moves an error the compound rate does not make, at the
    years between.
- **A raising streak in place of a rate.** Rejected, as the series record
  rejected it for the streak. A special in one year reads as a cut in the
  next, which is the excess answer 6 hands M5. It is a count and not a rate,
  and GOALS.md names a growth rate. It may be another metric, under a record
  of its own.
- **Leaving the rate out of M5 until the split crosses.** Rejected:
  - GOALS.md's fifth criterion names the rate, so deferring it changes the
    milestone's scope, which is Constitutional.
  - Under the compound form, the split's absence costs at the two ends alone,
    with a named direction at each. That is a stated exposure and not a wrong
    number for every filer.
  - The split waits on a `fetch-normalize` version carrying the axis, which
    no record proposes.
- **Telling a special from a raise by its size.** Rejected: it is the
  inference from magnitude that M4 and answer 6 forbid, and any threshold
  would be a number no source fixes.
- **A zero at the computing year as a condition.** Rejected: the formula is
  defined there, the root of zero is in the table, and an absence would hide
  a suspension the streak shows as `0`.
- **The two root exclusions listed as conditions of their own,** such as
  `negative_radicand` and `zero_dividend_growth_years`. Rejected: neither can
  be reached. The setting's domain excludes zero, and the negative figure is
  asked first. A condition the entry lists and the metric never gives tells a
  reader something false. The input conditions already name both.
- **The base year's zero under `dividend_coverage_per_share`'s
  `zero_dividends_declared_per_share`.** Rejected: that condition names a
  zero at the year stated at, and here the zero is n years earlier.
- **Trailing averages at each end**, such as the mean of the two years at
  each end. Rejected: they dampen an end-year excess without removing it. The
  span averaged is a number no source fixes, and they read the years beside
  the ends, so an excess there moves the rate after all.
- **The rate over the years the chain did find, when it breaks.** Rejected by
  the series record: no window shorter than n, and no closing up across a
  gap.

## Consequences

**Easier:**

- The rate can be built from the accepted seek, three operations and one
  setting, with no constant and no new reason.
- An excess, a timing shift, a gap in reporting or an `Unknown` at a year
  between changes nothing.
- A suspension at the computing year shows as `-1` beside a streak of `0`.

**Harder:**

- An excess at either end moves the rate, up at the computing year and down
  at the base year, and nothing marks it.
- The rate says nothing about the path between its ends. A reader wanting
  the path reads the streak.
- A filer that paid nothing at the base year has no rate until n years after
  it first paid.
- The figure is long, and its last digits carry the root's rounding.

**Expensive to reverse:**

- The form, once rates are stored and ranked on.
- `zero_base_year_dividends_declared_per_share` and `-1` at a computing-year
  zero, once stored.

**Open, and not this record's:**

- Whether the dividend-based valuation reads this rate. That is its own
  record.
- The regular dividend alone, which the end-year error waits on. It arrives
  with a `fetch-normalize` version that carries the axis and member, then
  `canonical-concepts` v3, and answer 6 says so.
- A raising streak, if anyone wants one.

## Enforcement

This applies anchors 4 and 5, and edits neither.

- **Anchor 4.** The rate reads its history and settings, and only through
  the accepted seek and arithmetic. So `purity` and `ambient` read what they
  read today.
- **Anchor 5.** The index is a setting with a stated preset and reason, and
  `literals` keeps every number but zero and one out of the formula. The form
  cites its source, quoted from a copy opened in the run.
  - **Gap:** nothing checks that a citation is true.
- **The two reads, and no shorter window.** The proof's tests hold them.
  - **Gap:** nothing forces a metric through the seek rather than around it,
    as the series record says.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the characters of
  one metric's name and of one new `declined` condition, which cross
  `analyze-store` v1, and a formula under the method version. It edits no
  anchor, no protected path, no gate, no contract byte, no schema and no
  milestone scope, and asks for no allowlist entry. Flagged for later human
  review, as the tier requires.
- **Checked:**
  - The author's copy, downloaded in this run and extracted with
    `pdfminer.six`: its sha256 is the record's, it has 52 pages with printed
    numbers matching, and every quotation was found as printed at the page
    the record gives (pages 3, 4 and 7), with "also applies to revenues" on
    page 3. The entry page's two sentences read as quoted, and neither page
    prints a year, as the record says.
  - `which-duration-precedes-a-duration.md`: "that metric's task", "reads at
    least the first and the last", "Every read it names is made", the
    negative figure and the base-year zero named as this metric's own
    conditions, the preset's reason and the n-of-2-at-2022 example, each as
    printed; `year.rs` holds `Year::series` and `PRECEDING_YEAR` as cited.
  - `canonical-concepts-open-questions.md`, answer 6: its four quotations as
    printed, and "must not take one period's declared total as a
    perpetuity" for the valuation section.
  - `how-analyze-computes-and-writes-a-figure.md`: the root as what a
    compound growth rate needs, the index from a constant, a setting or a
    count, the two exclusions sentence, and rounding ties away from zero.
  - `what-analyze-takes-and-returns.md`: the three items the record places
    under the method version, as printed.
  - The code: `negative_dividends_declared_per_share` is defined once in
    `dividend_streak.rs` with the meaning quoted, and
    `zero_dividends_declared_per_share` belongs to
    `dividend_coverage_per_share.rs`, so the new condition's name collides
    with neither. The suspension fixture's seven years in
    `tests/dividend_streak.rs` are the record's figures, 2024 a silent zero.
  - The arithmetic, each reproduced: 1.1 to the fifth, 1.2 to the fifth and
    the special it implies, the mean of changes at `0.21` with 2022 doubled,
    the fifth root of two and the 26% and -4% it gives a 10% grower, the
    square root of four thirds to 34 digits with 491 following, five thirds
    rounded then less one at 33 digits, and every fixture row under n of 2,
    1 and 5.
  - The alternatives as argued: the mean of changes, the regression and the
    median, the raising streak, leaving the rate out of M5, the size test,
    a zero computing year as a condition, two unreachable conditions, the
    coverage metric's zero, trailing averages, and the shortened window.
- **Verdict and why:** accepted. The form is the one the arithmetic record
  already anticipated, and the record takes it for the right reason: with
  two reads, the n - 1 years between cannot move the rate, so the exposure
  answer 6 hands M5 is confined to the two ends, each with its direction
  named, instead of spread over every window as the mean would spread it.
  `-1` at a computing-year zero is the formula's own value and errs the way
  the catalogue asks a wrong zero to err; a condition there would hide the
  suspension the streak shows as `0`. The root's two exclusions are met as
  every metric meets the zero divisor, by naming the input condition that
  reaches it, and nothing the metric can never give is listed. No constant,
  no invented setting, no new reason, no operation outside the table.
- **What would have changed it:** a seventh reason; a changed byte of
  `canonical-concepts` v2 or `analyze-store` v1; an operation outside the
  table, which the regression would have needed; a window shortened at a
  gap or a figure substituted; a size threshold marking a special; the
  root's index from a count or a literal; a quotation not found as printed;
  or a passage of the chapter stating the form for dividends, which was
  looked for and not found, the chapter stating it for earnings and
  revenues alone. Flagged for the human beside the tier: the end-year error
  has no fix at v2, and a large special at the base year reads as a cut in
  one window of the n + 1 that hold it; `-1` at a silence zero at the
  computing year is a cut never made where the silence is wrong; and the
  form is applied to dividends on the record's own reasoning, not a
  source's.
