# Relative yield is a year's dividend yield over the mean of the filer's own yields at the `relative_dividend_yield_years` years before it, each of those a year's dividends declared per common share over the unadjusted close at that year's own end; a past year that declared nothing declines, and the past yield is a term stated nowhere

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-11-22, by `M5-69`
- **Decided:** 2026-11-23, by the decider
- **Touches:** the relative yield that GOALS.md's fifth M5 criterion names in
  "yield and relative yield", and relative yield alone. It fixes the name and
  meaning of one metric, which cross `analyze-store` v1 as characters, and no
  byte of that contract changes. It answers the question `dividend-yield.md`
  (Accepted) leaves open by name, whether any metric states a yield at a past
  close: none does, and that yield is a term. It adds one setting,
  `relative_dividend_yield_years`, and one `declined` condition,
  `zero_past_year_dividends_declared_per_share`, and reaches two existing
  conditions from their one definitions. It adds no reason. It takes two
  operations from the arithmetic record's table and none from outside it. It
  puts no number in `constants/`. It reads the past period ends
  `price-provider.md` (Accepted) lists "for relative yield only", and is the
  first metric to give `date_not_asked` a witness. It asks for one reading
  added beside the one function `M5-54` built, which changes nothing that task
  built. No accepted record is superseded. No contract's bytes, no schema, no
  edge, no gate and no dependency move. Nothing here is implemented.

## Context

GOALS.md's fifth M5 criterion opens its list of dividend metrics with "yield
and relative yield". The yield has its record. Relative yield's form, the
years it reads, its conditions, its name and its source have none.

Some things are already fixed, and this record restates them as fixed:

- **The provider record.** `price-provider.md` (Accepted):
  - lists a second kind of date: "At each past fiscal period end the history
    holds, for relative yield only.", of which "The price is the close on the
    last trading day on or before each period end.";
  - says whose reading it is: "This reads relative yield as a company's
    current yield against its own past yields (Weiss and Lowe, *Dividends
    Don't Lie*, 1988).";
  - says of the other reading: "Spare's relative dividend yield (*Relative
    Dividend Yield*, 1992) divides by the market's yield instead. That
    divisor is neither a filing concept nor a price of the filer, so this
    provider cannot supply it. A metric task that chooses Spare's reading
    brings its own record for that source.";
  - fixes the close as unadjusted, and says "`divCash` goes unread, because
    yield reads the dividend from the filing."
- **The analyze record.** `what-analyze-takes-and-returns.md` (Accepted):
  - a price-dependent metric reads "a past period end's from the earlier
    answer whose date asked equals that period end character for character";
  - `no_price` has the witness `date_not_asked`: "Built from a crossing whose
    answers hold none for the date the metric reads. It names that date. This
    is relative yield at a past period end when the caller asked the
    valuation date alone.";
  - of what the caller must ask: "Asking every period end the history holds
    does that, costs no request (`price-provider.md`), and is what relative
    yield asks already.";
  - the share-basis rule has three clauses, the third "The list holds no
    split dated on or after the earlier of E and T."
- **The yield record.** `dividend-yield.md` (Accepted) fixes
  `dividend_yield_per_share`, a year's `dividends_declared_per_share` over the
  close at the valuation date. It says "Relative yield is left to a record of
  its own." It leaves open, by name: "Whether any metric states a yield at a
  past close is left open, to the record on relative yield or to one of its
  own."
- **The concept.** `canonical-concepts.md` (Accepted) lists "yield and
  relative yield" first among the consumers of `dividends_declared_per_share`.
  `canonical-concepts-open-questions.md` (Accepted) names among the total's
  movers, "for filers that declare a special only: yield and relative yield".
  It also fixes the silence test: "silence reads as zero only where a
  wrongly-read zero cannot flatter the company on any metric that consumes the
  concept". v2 gives the concept a conditional silence, zero when "the
  period's facts are silent on this concept and on dividends_paid alike".
- **The common-shareholder record.** `common-shareholder-forms.md` (Accepted)
  lists "yield and relative yield" among the metrics "Common by the concept
  they read, with no term to form", and fixes the naming rule.
- **The share-basis dates record.** `share-basis-rule-dates.md` (Accepted):
  "For a metric stated at a duration, as both discounted cash flow metrics
  are, E is the duration's end".
- **The series record.** `which-duration-precedes-a-duration.md` (Accepted):
  "A metric is stated over a year when its source states it over a fiscal
  year." It orders a window's reads: the computing year's first, "then each
  further year's, in series order", and "Identical reasons are not merged".
- **The window's precedents.** `dividend-growth-rate.md` (Accepted) reads its
  years through `Year::series` under the setting `dividend_growth_years`.
  `dividend-discount-valuation-and-margin-of-safety.md` (Accepted) forms a
  mean over `dividend_discount_base_years` years, each read, each counted
  once.
- **The arithmetic.** `how-analyze-computes-and-writes-a-figure.md`
  (Accepted) fixes exact rationals, one rounding when a figure is written,
  and a table of operations.
- **The price read is built.** `M5-54` wrote
  `crates/analyze/src/metrics/reading/price.rs`: the share-basis rule as one
  function, and the reading of the close at the valuation date as one
  function. That reading takes the answer by its place in the crossing, so it
  reads the valuation date and no other.
- **Absence is closed.** Six reasons.
- **The entry shape.** M5-18 fixed it. `Unit` carries `FractionOfOne`,
  `Multiple`, `Years`, `CurrencyPerShare` and `Signals`.

## Decision

### One metric, one term, one setting

| Name | What it is | Unit | Stated at |
| :--- | :--- | :--- | :--- |
| `relative_dividend_yield_per_share` | A year's dividend yield over the mean of the filer's own yields at the n years before it, each at its own year's end | `Multiple` | a year |

n is the setting `relative_dividend_yield_years`. The metric reads a price at
n + 1 dates. Nothing is added under `constants/`.

**The yield at a year's own close is a term, and no metric.** It is a year's
`dividends_declared_per_share` over the close at that year's own end. It has
no catalogue entry, and no name crosses `analyze-store` v1 for it. That
answers the question the yield record leaves open.

### The derivation, in formula order

> y₀ = D₀ ÷ P₀
>
> yₖ = Dₖ ÷ Pₖ, for each k from 1 to n
>
> m = (y₁ + … + yₙ) ÷ n
>
> relative yield = y₀ ÷ m

- D₀ is `dividends_declared_per_share` over the year Y the figure is stated
  at, and P₀ is the close at the valuation date. y₀ is the yield record's
  `dividend_yield_per_share` at Y.
- Dₖ is `dividends_declared_per_share` over the k-th year before Y, and Pₖ is
  the close at that year's own end.
- n is `relative_dividend_yield_years`, read from the `Settings` the call was
  handed.

| Step | Operation, as the table names it |
| :--- | :--- |
| D₀ ÷ P₀, and each Dₖ ÷ Pₖ | division. Each close is above zero wherever the division is made, since `non_positive_close` is asked before it. |
| y₁ + … + yₙ | addition |
| the sum ÷ n | division. n is a positive integer by the setting's domain, so no zero divisor arises. |
| y₀ ÷ m | division. m is above zero wherever it is made, since the conditions below leave every yₖ above zero. |

Sign, from the same table, asks the conditions. No other operation is taken:
no root, no power, no minimum, no maximum and no literal. The divisor n comes
from a setting, as the dividend discount record's mean takes its own.

- **The current yield is the yield record's, and is formed once.** The
  quotient of a year's dividend and a close, with its two conditions, is
  written where the yield's task writes it. Relative yield forms y₀ through
  that one function, and makes the yield's two reads through the readings the
  yield makes them through. It writes no second quotient.
- **Each past yield is the same quotient at another close.** yₖ is formed
  through the same function. Only the close differs: the past period end's,
  where the yield reads the valuation date's.
- **The dividend is the filing's, and never the provider's.** The provider
  record fixes it: "`divCash` goes unread, because yield reads the dividend
  from the filing." Every D is read from the history, and from nowhere else.
- **Every close is the unadjusted close.** The valuation date's is read
  through the one function `M5-54` built. Each past period end's is read
  through the reading below. The metric reads the crossing nowhere else.
- **Every D is the total,** regular and special together, as v2 carries it.
- **The mean is of the yields, and not a yield of means.** Each year's
  dividend is set over that year's own close before anything is summed.
- **Exact, and rounded once.** Every step is exact. The quotient is rounded
  when it is written, as the arithmetic record says of every figure.

### Why the mean

The provider record's sentence is "a company's current yield against its own
past yields". No source opened gives the past yields a statistic (see
Source). So the mean is this record's own decision, with these reasons:

- **It reads every year the same way.** No year is picked by the size of its
  figure. The dividend discount record says of its own mean that it "never
  asks which year is special", and `dividend-growth-rate.md` refuses
  "trimming the largest figure, or reading a year against its neighbours".
  The highest and the lowest are each one year, chosen by magnitude.
- **One year moves it by one n-th.** A special dividend, or a close that fell
  into a year's end, lifts one past yield. In the mean it carries one n-th of
  that. As the highest, it would be the whole reference for n years.
- **The closes are samples, and the mean is what samples estimate.** Each
  past close is one day's, a fiscal period end's. The yield's turning points
  fall between those days, so the highest yield the history shows is not the
  highest the market offered (see What no check can see). A mean of n
  year-ends states the level the yield stood at. It claims no extreme.
- **One figure, in a unit the entry shape has.** The current yield over the
  mean is one quotient of two yields. Above one, the yield stands above its
  own past level. Below one, beneath it.

**What the figure is not.** The newsletter's pages speak of a stock's
"historically repetitive areas of undervalued and overvalued dividend yield",
and its tables print a "Hi Yld" and a "Lo Yld". This figure states neither
area. It marks no stock undervalued or overvalued. It is the provider
record's sentence, formed.

### The term

The yield at a year's own close is formed once, in a module of its own, as
market capitalisation is.

- **Its reader** makes the dividend read at a found year and the price read
  at that year's end. The term's reasons are theirs, and no outcome says the
  term was absent.
- **Its quotient is the yield's.** The term reaches the one function the
  yield's task writes. Where that module does not hand the function out, the
  implementing task widens its visibility and changes nothing it computes.
- **Why a term, and no metric.**
  - The provider record lists the past period ends "for relative yield only".
    A metric that stated a yield at a past close under a name of its own
    would be a second consumer of them. That needs the sentence changed,
    which is a record of its own.
  - Stored per year, it would let M6 compose relative yield as a query. But
    GOALS.md names relative yield among M5's metrics, and the setting would
    then be a ranking criterion recorded in no result's premises.
- **Nothing here would have to be undone** for a later record to state it. It
  would read this module, under a name of its own.

### The reading of a past period end's close

`M5-54`'s reading takes the valuation date's answer by its place. It cannot
read a past period end as it stands. So the implementing task **adds one
reading beside it**, in `crates/analyze/src/metrics/reading/price.rs`, and
changes nothing `M5-54` built. Handed a period end, it gives the close or
exactly one reason, in the order the discounted cash flow record fixes for
the valuation date's:

1. no crossing: `no_price` with `not_asked`;
2. no answer whose date asked equals the period end, character for
   character: `no_price` with `date_not_asked`, naming the period end;
3. an absence answered for it: `no_price` with `answered_absent`, carrying
   the date asked;
4. a price: the share-basis rule, through the one function, as it stands;
5. a close outside the decimal grammar: `declined` with
   `input_not_a_decimal`.

It never reads another date's answer, an adjusted close or a stand-in. It
finds the answer by equality of characters, and by no ordering and no
nearness.

**A past period end that is the valuation date.** The analyze record reads a
past period end's price "from the earlier answer whose date asked equals that
period end". A crossing holds one answer per date asked, so where a past
period end read is the valuation date itself, no earlier answer names it.
`date_not_asked` cannot be built there: `analyze-store` v1 gives it to a
crossing that "holds no answer for the date the metric reads", and this one
holds one. So the reading takes the crossing's one answer for that date,
which is the valuation date's. No figure follows from it. The valuation date
is then before Y's end, so the current yield has no close: its pair is
`not_covered`, as the share-basis record says of E after T, or its date was
answered with an absence.

### Which periods, and which years

**Relative yield is stated over a year,** under the series record's test.

- The current yield is stated over a year, and its source's form names
  "Annual Dividends per Share". The newsletter's table sets "the current
  annual cash dividend" over the price.
- Each past yield is a year's too. The chapter's illustration is by year:
  "For each year we will also compute the dividend yield and dividend payout
  ratio for each firm."
- A quarter's dividend over a close is about a quarter of a year's yield, as
  the yield record says, and a mean of quarters and years would mix the two.

At a duration that is not a year, it gives exactly one `declined` reason, the
first of `start_not_a_date`, `end_not_a_date`, `not_a_year` and
`overlaps_another_year`, and makes no read. Its `stated_at` stays `Duration`.

**It reads Y and the n years before it,** through `Year::series`, as the
dividend growth rate reaches its window. Every one of the n + 1 years is
read. Y's own yield at Y's own end is not among the past yields: the current
yield is set against the years before it, and Y's dividend stands in the
numerator alone.

**Nothing is skipped and nothing is substituted,** as the series record
fixes:

- **A gap.** A year the chain cannot find, and every year beyond it, gives
  `no_period` with `preceding_year` and `computing_at` Y. There is one per
  year, and identical reasons are not merged.
- **A history shorter than the window** is a gap at its end, and gives the
  same.
- **No price is read at a year that was not found.** The date that read would
  name is the year's end, and the history holds no such year. So a missing
  year gives its one `no_period` and nothing more.
- The mean is never taken over the years the chain did find. No year stands
  in for another, and no quarter enters.

**Where it is stated.** At each year it is computed at, beside the close at
the valuation date, as the yield is. Nothing here orders the years or picks
the latest.

### The setting

`relative_dividend_yield_years` is added in `crates/analyze/src/settings/`,
in a file of the metric's own, as a field of the one `Settings`.

| Name | Meaning | Unit | Domain | Preset |
| :--- | :--- | :--- | :--- | :--- |
| `relative_dividend_yield_years` | How many years before the year stated at have their yields, each at its own year's end, taken into the mean the current yield is set over | years | the positive integers | `5` |

- **Why a setting.** No source opened fixes a count (see Source). Two careful
  users could choose differently and the figure would still mean a current
  yield against the filer's own past yields. That is the analyze record's
  test of a setting.
- **The domain.** The positive integers, never clamped. Zero is refused by
  the constructor as the caller's error, before analyze runs. One is
  admitted: the current yield over last year's is defined, and no record
  forbids it as answer 6 forbids a base of one year. Its type is no wider
  than `u16`, as `dividend_discount_base_years`'s is, and the implementing
  task takes a series that many years back without overflow.
- **The preset, `5`, and its reason.** No source fixes it, so the reason is
  the record's own, in three parts:
  - **The history holds it.** The figure reads n + 1 years, six at the
    preset. The series record's words for `dividend_growth_years` apply as
    they stand: "six fiscal years are inside the structured history of any
    filer that has filed XBRL since the 2011 phase-in".
  - **One year's excess is a fifth.** A special dividend in one past year
    moves the mean by a fifth of itself, as the dividend discount record says
    of its base at the same preset.
  - **Each further year widens the split exposure.** A split dated after the
    earliest past period end read makes the figure absent (below). A longer
    window makes more filers absent, for longer.
- **What the preset does not claim.** The newsletter's page says its areas
  "have been established over multiple market cycles". Five year-ends are not
  shown to span that, and the record does not say they do. A user who wants a
  longer history sets it, and the result records the value used.
- It is a setting of its own, and not `dividend_growth_years` or
  `dividend_discount_base_years`. Moving one moves no other. A change to the
  preset is not a version bump.

**No other free parameter.** The valuation date is carried by the crossing,
as the analyze record decided. The statistic and the form are the method's.
**No threshold and no band.** The page prints "a range of 10 percent above
and below Undervalue" and "30 percent or 3 standard deviations". Neither
enters the derivation, as a constant or otherwise: each is measured from the
newsletter's Undervalue area, which this figure does not form. A level
applied to the stored figure is a ranking criterion, which is M6's.

### The share-basis rule

Every D is a figure in currency per share, so the rule governs every pair.
**It is asked for every pair of a figure and a close the derivation
divides,** n + 1 in all, each through the one function.

| Pair | E | T |
| :--- | :--- | :--- |
| D₀ and P₀ | Y's end | the trading date of the close at the valuation date |
| Dₖ and Pₖ | the end of the k-th year before Y | the trading date of the close at that end |

- **E is the duration's end,** as the share-basis record fixes for a metric
  stated at a duration. For a past pair it is the past year's own end, since
  that is the period the figure was read at.
- **E never reaches the rule outside the grammar.** Y's end is asked by the
  year test first. Every past year is a year by the same test, or it was not
  found.
- **For a past pair, T is on or before E.** The close is "on the last trading
  day on or before each period end". So the earlier of the two is T.
- **What the caller must ask.** Every period end the history holds, as the
  analyze record says relative yield asks already. A caller that asked less
  gets `date_not_asked` or `not_covered`, never a number.

**A split after the earliest past period end read.** The third clause holds
"no split dated on or after the earlier of E and T". For the pair at the n-th
year before Y, that is every split from that year's closing trading day
forward. So a filer that split inside the window has no relative yield. The
outcome is `basis_unproven` with `split_within`, one for each pair whose
earlier date the split is dated on or after. That holds even where a later
filing restated the past dividends, since v2 carries no filed date to show
it. The figure returns at the first year whose earliest past close traded
after the split. The cost is stated under Consequences.

### The `declined` conditions

Each is asked only where every read and every price read gave a number. They
are asked in formula order, and the first that holds is the one reason. Each
is a sign or zero test and needs no literal.

| Condition | Holds when | Defined |
| :--- | :--- | :--- |
| `negative_dividends_declared_per_share` | a year read declared a negative dividend per common share | once, beside the streak; reached from there |
| `non_positive_close` | a close read is zero or below | once, beside the discounted cash flow margin; reached from there |
| `zero_past_year_dividends_declared_per_share` | a year read before Y declared a dividend per common share of zero | new, once, beside this metric |

**Formula order.** The yields are formed one after another: y₀, then each yₖ
in series order. Each asks the yield record's two conditions in that record's
order, the dividend's and then the close's. Then the mean asks the third,
over the past years. So where every read gave a number and the current yield
declines, relative yield declines under the same condition.

**A negative dividend,** at Y or at a past year, is the streak's condition,
with the same characters and meaning. The yield record's reasons hold at
every year read. A mean over a negative yield would state a level no share
yielded.

**A close of zero or below,** at the valuation date or at a past period end,
is the discounted cash flow margin's condition. A zero close is a zero
divisor. A close near zero gives a past yield without bound, and the largest
mean the form can give.

**A past year that declared nothing declines. It is not counted as a yield of
zero.** The yield record counts a non-payer's yield as zero, and at a past
year that zero is true. This record still declines on it:

- **Counted, a wrong zero would flatter.** A zero among the past yields
  lowers the mean and raises the figure. The catalogue set this concept's
  silence to zero under the test that "a wrongly-read zero cannot flatter the
  company on any metric that consumes the concept", and relative yield is a
  consumer it names. A past zero counted in the mean would make that
  sentence false. Declined, a wrong zero costs an absence, which does not
  flatter.
- **A true zero misleads the same way.** A filer that began paying inside the
  window would read far above one. That says the dividend is new. It says
  nothing of the price, and the figure would look like any other.
- **The source confines the comparison to payers.** The newsletter's page
  lists "At least 25 years of uninterrupted dividends" among the six
  qualifications a stock meets five of before its yield areas are sought,
  and its overview says to "completely avoid stocks which pay no dividend at
  all".
- **It hides nothing.** The outcome names the zero. The streak and the yield
  show the same year as `0`.
- **The characters are new.** `zero_dividends_declared_per_share` is "a zero
  at the year the metric is stated at", and
  `zero_base_year_dividends_declared_per_share` a zero at the growth rate's
  base year. This is a zero at any year read before Y. The same characters
  for either would say the wrong year.

**A zero divisor cannot arise, and is not listed.** Where the three
conditions have passed, every past dividend is above zero and every past
close is above zero. So every yₖ is above zero, and their sum and their mean
are. The one way m could be zero is every past year declaring nothing, which
is the third condition. The entry names that input condition, as the growth
rate's entry names its zero divisor. The implementing task treats a zero mean
at the last division as unreachable.

**A zero at Y gives the figure `0`. It is a figure, and no condition.**

- The current yield is `0`, as the yield record decides, and zero over a
  mean above zero is defined.
- It is true: a share that declared nothing yields none of its past level.
- It is the unflattering direction. A wrong silence zero at Y gives `0`, the
  lowest figure a payer can show.
- The yield shows `0`, the streak `0` and the growth rate `-1` at the same
  year, and the four agree.

**The first that holds.**

- A negative dividend at a past year beside a close of zero at the valuation
  date gives `non_positive_close`: the current yield is formed first.
- A past year declaring nothing beside a close of zero at that year's end
  gives `non_positive_close`, and never the third condition.
- A zero at Y beside a past year declaring nothing gives
  `zero_past_year_dividends_declared_per_share`, and never `0`.

The entry also lists the four year conditions, then
`share_basis_date_not_a_date`, as the share-basis record places it.
`input_not_a_decimal` reaches it from the one definition beside the reader.

### How absence composes

Against the analyze record's closed reasons, and adding none.

1. **The year test first.** At a duration that is not a year, its one reason
   is the outcome and nothing below is asked.
2. **The current year.** The read of `dividends_declared_per_share` at Y,
   then the price read at the valuation date, with E the end of Y. Each
   gives a number or one reason, exactly as the yield record lists them.
3. **Each past year, in series order,** the year before Y first.
   - **The read.** A number, or one reason: `no_period` with `preceding_year`
     where the year was not found or stands beyond one that was not;
     `input_unknown` or `input_not_applicable`, naming the concept and the
     found year; or `declined` with `input_not_a_decimal`.
   - **The price read,** at that year's end, wherever the year was found,
     whatever its read gave. A number, or exactly one reason, in the order
     the reading above lists:
     - no crossing: `no_price` with `not_asked`;
     - the crossing holds no answer for that period end: `no_price` with
       `date_not_asked`, naming the date. This is the witness the analyze
       record keeps for this metric;
     - an absence answered for it: `no_price` with `answered_absent`,
       carrying the date asked;
     - a price: `basis_unproven` with `splits_absent`, `not_covered` or
       `split_within`, by the first clause that fails for the pair asked, or
       `declined` with `share_basis_date_not_a_date`;
     - a close outside the decimal grammar: `declined` with
       `input_not_a_decimal`.
4. **Every read and every price read is made, and none stops another.** The
   reasons stand in the order above: the current year's read and then its
   price read, then each past year's read and then its price read. Identical
   reasons are not merged, so a crossing that is absent gives one `not_asked`
   for each close the metric reads.
5. **The three conditions,** in formula order, only where every read and
   every price read gave a number.

**It is not built on the yield's outcome.** It makes the yield's reads and
forms the yield's quotient, and asks a condition only at step 5. So a
negative dividend at Y beside an `Unknown` past year gives the
`input_unknown` alone, where `dividend_yield_per_share` at the same row
gives its condition. The yield's own outcome is unchanged.

**Never a fall-back.** It never reads another date's price, an adjusted
close, the provider's dividend or a stand-in. It never takes a nearer
period end's close for one the crossing lacks, and never forms a dividend
from `dividends_paid` and a share count.

**With no key, no number.** The caller with no key asks nothing, so the
crossing is absent, and relative yield is absent with `no_price` and
`not_asked` for each close it reads, among any reasons of its dividend
reads. That is GOALS.md's ninth M5
criterion: "A metric with no price data is absent with a reason. It never
fails the run and never shows a wrong number." A caller that asked the
valuation date alone gets `date_not_asked` for each past period end, beside
`not_covered` for the current pair wherever its close traded after Y's end,
and never a number.

No seventh reason is needed, and no frozen byte changes.

### By kind

`dividends_declared_per_share` applies to every kind, so no kind is absent by
applicability.

| Kind | Where every read and every price read gives a number |
| :--- | :--- |
| `operating`, `bank`, `insurer`, `reit`, `investment_company` | computed |

**The two arms of the conditional silence:**

| Arm | At Y | At a past year |
| :--- | :--- | :--- |
| Zero, where the period's facts are silent on both dividend concepts | the figure `0` | `zero_past_year_dividends_declared_per_share` |
| `Unknown`, where `dividends_paid` resolves to a non-zero value | `input_unknown`, naming the concept and Y | `input_unknown`, naming the concept and that year |

It does not read `dividends_paid`. The kind does not cross v2, and the metric
does not need it.

### The name

`relative_dividend_yield_per_share`. It crosses `analyze-store` v1 as
characters, under the common-shareholder record's naming rule.

- **It carries `per_share`.** It reads `dividends_declared_per_share`, a
  per-common-share concept, and the rule puts `per_share` in "the name of a
  form that reads per-common-share concepts". It is the name of the yield it
  stands on, `dividend_yield_per_share`, with GOALS.md's word before it. Its
  `meaning` says "per common share".
- **It does not carry `common`.** It forms neither common equity nor income
  available to common.
- **The bare name stays free,** as the yield record leaves `dividend_yield`
  free for a total form.
- **The words are also Spare's title.** The name does not say whose reading
  it is. The `meaning` does: "the filer's own". Spare's reading, if a later
  record brings it, takes a name that says the market.

### What its catalogue entry says

Nothing beyond the shape M5-18 fixed, and no new value of `unit`:

- **`name`:** `relative_dividend_yield_per_share`.
- **`meaning`:** "The dividend yield per common share at this year, the
  year's dividends declared per common share, regular and special together,
  over the unadjusted close at the valuation date, over the mean of the
  filer's own yields at the relative_dividend_yield_years years before it,
  each of those a year's dividends declared per common share over the
  unadjusted close on the last trading day on or before that year's own
  end."
- **`unit`:** `Multiple`. A quotient of two yields is a pure number at a
  scale of one: `1.25` is a yield one and a quarter times its past mean, and
  never `125` or a percentage. It is not `FractionOfOne`: it is no share of
  a whole, and one is its middle and not its ceiling.
- **`stated_at`:** `Duration`.
- **`inputs`:** `dividends_declared_per_share`, once. The price is no concept
  and is not listed.
- **`conditions`,** in the order asked:
  - `negative_dividends_declared_per_share`
  - `non_positive_close`
  - `zero_past_year_dividends_declared_per_share`
  - `start_not_a_date`, `end_not_a_date`, `not_a_year` and
    `overlaps_another_year`, by their constants
  - `share_basis_date_not_a_date`
  - then `input_not_a_decimal`, generated from its one definition

The setting is recorded in every result's premises by name and value, as
every setting is. The entry carries no field for it, and names it in its
`meaning`, as the dividend growth rate's does. The entry shape says
everything the metric reads, and nothing is missing from it.

### The method version

These are part of the method `METHOD_VERSION` names:

- the form: the current yield over the mean of the past yields
- each past yield as a year's dividend over the close at that year's own end
- the years read, the n before Y, and the setting's name, meaning and domain
- E and T for each pair, and the rule asked for every pair
- the reading of a past period end's close, and its order of reasons
- the name and its meaning
- the three conditions and their order, a zero at Y as a figure, and a zero
  at a past year as a condition

A later change to any of them, the form, the years read, the statistic or a
condition included, is a version bump and a superseding record, and never an
edit to this one. A preset change is not a bump.

It lands at version 1. Relative yield is not computed yet, so no stored
outcome changes, and adding a metric is not a bump.

### Source

Under `sources-a-task-can-read.md`. Each copy below was opened in this run,
and each quotation is as printed.

**The origin.** Weiss and Lowe, *Dividends Don't Lie*, 1988, as the provider
record names it. It is not read from the workspace. No chapter, page, table
or quotation of it is written here, and nothing a summary says it holds.

**The open primary: the pages of Investment Quality Trends.** They are live
HTML with no stable digest. Each was fetched whole and read on 2026-11-22,
and what is quoted stood there that day.

- **Whose method the pages state.** <https://www.iqtrends.com/about.php>:
  "Investment Quality Trends is the creation of our Publisher Emerita, the
  incomparable Geraldine Weiss." And: "Mrs. Weiss co-authored the seminal
  work for dividend-centric value investing; Dividends Don’t Lie."
- **The notion.** <https://www.iqtrends.com/value_identification_basics.php>,
  headed "Value Identification Basics":
  - "Our Profiles of Value, the study and identification of the historically
    repetitive patterns of undervalue and overvalue areas of dividend- yield,
    is how we identify value, or when to buy, sell, or hold."
  - "When a stock has passed this filter for its qualitative
    characteristics, we then analyze it further to determine its
    historically repetitive areas of undervalued and overvalued dividend
    yield."
  - The filter is "six following qualifications", of which a stock meets "at
    least five". One reads "At least 25 years of uninterrupted dividends".
- **The past yields are the filer's own, and how far to trust them.**
  <https://www.iqtrends.com/dividends_yields.php>, headed "Dividend Increases
  & Yields to Watch": "The dividend yields indicating the Undervalue area
  have been established over multiple market cycles. Although these
  low-price/high-yield areas are repetitive, they are neither absolute nor
  inviolate." It goes on: "In fact, a range of 10 percent above and below
  Undervalue is normal." And it highlights a stock when its yield "deviates
  30 percent or 3 standard deviations or more from its historically
  repetitive area of Undervalue".
- **The yield the newsletter sets against them.**
  <https://www.iqtrends.com/understanding_data_tables.php>, headed
  "Understanding IQT Data Tables". This page is not one the task names, and
  was opened from the site's own menu:
  - "Price- the current price."
  - "Dividend- the current annual cash dividend."
  - "Yield- the current yield represented as a percent - (dividend/price) X
    100."
  - "Hi Yld- This is the historical yield at Undervalue (or high yield)."
    And: "Lo Yld- This is the historical yield at Overvalue (or low yield)."
  - Cited under What no check can see: "A yield that is unusually high may
    indicate a company has a problem and is being sold off."
- **Non-payers.** <https://www.iqtrends.com/content_overview.php>, headed
  "Newsletter Content Overview", opened the same way: "one should purchase
  the top dividend-paying stocks when the dividend yield is historically
  high, sell when the dividend yield declines to historic lows and completely
  avoid stocks which pay no dividend at all".

**What these pages do not state.** They state the notion: a stock's current
yield, judged against that stock's own past yields. They state:

- no formula for setting one against the other;
- no statistic of the past yields. The areas are "established", and no page
  says how;
- no count of years;
- no close at a fiscal period end, and no date for a past yield at all.

Each of those is this record's own decision, with its reason above, or the
provider record's where that record fixes it: the close at each past fiscal
period end is the provider record's. The pages' dividend is "the current
annual cash dividend", and this record's is the fiscal year's declared
total, as the yield record fixes.

**A past year's yield at that year's own close.** Damodaran, A., *Applied
Corporate Finance: A User's Manual*, 3rd ed., John Wiley and Sons, chapter
10, "Dividend Policy", the copy the yield record cites.

- **The copy opened.**
  <https://pages.stern.nyu.edu/~adamodar/pdfiles/acf3E/book/ch10.pdf>, 27
  sheets, 4,678,827 bytes, sha256
  `48ee94e4a9f200d5afc5ae51a0d9c876068c715aebd3992360ca136b648117d6`, the
  digest the task names. It was downloaded whole in this run. The server
  closed the transfer early on six tries, and the seventh arrived whole.
- **How it was read.** With `pdfminer.six`, taking each sheet's text boxes by
  the half they stand in, as the yield record describes. Printed page 16 is
  the right half of sheet 8, page 3 the left half of sheet 2 and page 4 its
  right half. Each quotation was checked with whitespace collapsed.
- **Page 3,** as the yield record cites it for the form: "Dividend Yield =
  Annual Dividends per Share/Price per Share".
- **Page 16,** "Illustration 10.1 Dividends, Dividend Yields, and Payout
  Ratios": "For each year we will also compute the dividend yield and
  dividend payout ratio for each firm." Its table sets "Dividends per share"
  over "Stock price at end of year". Its first firm reads $0.35 and $32.28
  beside 1.08%, and $0.35 and $22.69 beside 1.54%. The cells extract out of
  order, and each stands in the half-sheet's text. 0.35 over 32.28 is 1.08%,
  and over 22.69 is 1.54%.
- **Page 4,** as the yield record cites it for a zero dividend: "In both
  time periods, almost 65% of the overall sample of 7200 companies paid no
  dividends, making zero the median dividend yield across all companies."

Neither `www.iqtrends.com` nor `pages.stern.nyu.edu` is in the sandbox's
allowed domains. Each was read under a per-command allowance. No secondary
account of the method is cited. No page opened states the method otherwise
than the task quotes it.

### The proof the implementing task brings

These tests are built through `History::of` and `Row::at`, and a `Crossing`
built as the contract type. They read no file. Each stands in `tests/` or
under `#[cfg(test)]`.

**The worked history.** Six years, each from 01-01 to 12-31, with
`dividends_declared_per_share` as below. The setting is at its preset, 5.

**The worked crossing** is the share-basis record's preset, extended: a
valuation date of 2025-06-30 answered with a close of 60 traded 2025-06-30;
the earlier date 2024-12-31 answered with a close of 75 traded 2024-12-31;
an answer at each past period end read, as below; and an empty split list.

| Year | Dividend | Date asked | Trading date | Close | Yield at its own close |
| :--- | :--- | :--- | :--- | :--- | :--- |
| 2024 | 1.5 | 2024-12-31 | 2024-12-31 | 75 | not formed |
| 2023 | 1.2 | 2023-12-31 | 2023-12-29 | 60 | 0.02 |
| 2022 | 1.25 | 2022-12-31 | 2022-12-30 | 50 | 0.025 |
| 2021 | 1 | 2021-12-31 | 2021-12-31 | 50 | 0.02 |
| 2020 | 0.6 | 2020-12-31 | 2020-12-31 | 40 | 0.015 |
| 2019 | 0.8 | 2019-12-31 | 2019-12-31 | 40 | 0.02 |

At the least, computing at 2024 unless a test says otherwise:

- **The figure, worked by hand.** The current yield is 1.5 ÷ 60, which is
  0.025. The past yields sum to 0.1, and their mean is 0.02. 0.025 ÷ 0.02
  gives `1.25`.
- **The close at 2024-12-31 enters no figure.** Over 75 the current yield
  would be 0.02 and the figure `1`. It is never `1`.
- **Each past dividend over its own close.** Over the valuation date's 60,
  the past dividends would average 0.97 and the figure would be 1.5 ÷ 0.97.
  It is never that.
- **A quotient that does not terminate.** A 2024 dividend of 2: the current
  yield is one thirtieth, and the figure five thirds, written `1.` followed
  by thirty-two `6`s and a `7`.
- **The setting.** At 2: the mean of 0.02 and 0.025 is 0.0225, and the
  figure ten ninths, written `1.` followed by thirty-three `1`s. At 1,
  beside a crossing that holds no answer older than 2023-12-31: `1.25`, and
  no reason for a year not read.
- **A past period end not asked.**
  - The worked crossing without its answer at 2021-12-31: `no_price` with
    `date_not_asked`, naming 2021-12-31, alone.
  - The share-basis record's preset crossing, unextended: five
    `date_not_asked`, naming 2023-12-31, 2022-12-31, 2021-12-31, 2020-12-31
    and 2019-12-31, in that order.
  - The valuation date alone: `not_covered` for the current pair, naming
    2025-06-30 through 2025-06-30, 2024-12-31 and 2025-06-30, then the same
    five `date_not_asked`.
- **A past period end answered with an absence.** 2021-12-31 answered with
  an absence: `no_price` with `answered_absent`, carrying 2021-12-31, alone.
- **No crossing.** Six `no_price` with `not_asked`, and no number.
- **The share-basis rule, for every pair.**
  - A split dated 2022-06-01: three `split_within`, for the pairs at 2021,
    2020 and 2019 in that order, each naming the split, its own period end
    and its own trading date. It is never `1.25`, which would set three
    dividends over closes on another basis.
  - A split dated 2019-12-31: one `split_within`, for the pair at 2019. A
    split dated 2019-12-30: `1.25`.
  - A split dated 2025-03-14: six `split_within`, the current pair's first.
  - The split list absent: six `splits_absent`.
- **A date outside the grammar.** The trading date at 2021-12-31 written
  `2021-12-3`: six `declined` with `share_basis_date_not_a_date`, since the
  second clause reads every price's trading date.
- **Unreadable characters.** A close of `1e2` at 2021-12-31: `declined` with
  `input_not_a_decimal`, alone. A 2021 dividend of `1e2`: the same, from the
  read.
- **A past year declaring nothing.** A 2021 dividend of 0:
  `zero_past_year_dividends_declared_per_share`. It is never `1.5625`, the
  figure a mean counting the zero would give, and never `1.25`.
- **A zero divisor.** Every past dividend 0: the same condition, and no
  division by a zero mean is made.
- **A negative figure.** A 2021 dividend of -1:
  `negative_dividends_declared_per_share`. A 2024 dividend of -1.5: the same.
- **A close of zero.** A close of 0 at 2021-12-31: `non_positive_close`. A
  close of 0 at the valuation date: the same.
- **The year computed at declaring nothing.** A 2024 dividend of 0: the
  figure `0`.
- **The first that holds.**
  - A 2021 dividend of -1 beside a close of 0 at the valuation date:
    `non_positive_close` alone.
  - A 2021 dividend of 0 beside a close of 0 at 2021-12-31:
    `non_positive_close` alone.
  - A 2024 dividend of 0 beside a 2021 dividend of 0:
    `zero_past_year_dividends_declared_per_share`, and never `0`.
  - A 2024 dividend of -1.5 beside a 2022 dividend `Unknown`: `input_unknown`
    naming the concept and 2022, alone, since a condition is asked only
    where every read gave a number.
- **A gap.** The history without 2021: three `no_period` with
  `preceding_year` and `computing_at` 2024, for 2021, 2020 and 2019. No
  price reason stands for them, though the crossing answers their dates.
- **A history shorter than the window.** The years 2022 to 2024 alone: the
  same three `no_period`.
- **The conditional silence at a past year.** A 2022 dividend `Unknown`
  beside `dividends_paid` 300: `input_unknown`, naming
  `dividends_declared_per_share` and 2022, alone, and never a figure formed
  from `dividends_paid`.
- **A filer of each kind.** For each of the five kinds, the worked history
  with every concept v2 does not apply to that kind `NotApplicable` gives
  `1.25`.
- **Not a year.** The quarter 2024-10-01 to 2024-12-31, with the dividend
  `Unknown` and no crossing: `not_a_year` alone.
- **The order of reasons.** A 2024 dividend `Unknown`, a 2022 dividend `1e2`
  and no crossing: `input_unknown` naming the concept and 2024, `not_asked`,
  `not_asked`, `input_not_a_decimal`, then four more `not_asked`. The
  history without 2021 and no crossing: three `not_asked`, then three
  `no_period`.
- **A past period end that is the valuation date.** At a setting of 1,
  computing at 2022, with a valuation date of 2021-12-31 answered with a
  close traded 2021-12-31 and no earlier date: `not_covered` for the current
  pair, alone, and never `date_not_asked` naming 2021-12-31.
- **The yield beside it.** `dividend_yield_per_share` gives its pinned
  figures and reasons unchanged: `0.025` at 2024 and `0.02` at 2023 over the
  worked crossing, and `negative_dividends_declared_per_share` at a 2024
  dividend of -1.5 whatever a past year holds.
- **The catalogue.** The entry's name, meaning, unit, `Duration`, one input
  and conditions are as listed above, with `input_not_a_decimal` last. Its
  name holds `per_share` and not `common`. Its conditions do not hold
  `zero_dividends_declared_per_share`. No entry is named for the term.
- **The setting.** Zero is refused and never clamped. The preset is written
  `5`, and the result's premises record the value used.

### What no check can see

- **Whether the pages say what this record says.** They are live HTML, and
  were quoted as they stood on 2026-11-22. The implementing task opens them
  again. A page that no longer carries a sentence quoted here is a stop for
  that task, not a choice.
- **The past closes are fiscal period ends',** and not the turning points of
  a yield's cycle. The highest yield the history shows is not the highest
  the market offered, nor the lowest the lowest. The mean is of n days, one
  a year, each the same day of the fiscal calendar.
- **The figure is not the newsletter's.** Its areas are "established over
  multiple market cycles", by a method no page states, from "the current
  annual cash dividend". This is a fiscal year's declared total against a
  mean of n year-ends. A reader who takes it for an Undervalue or Overvalue
  reading takes more than it states.
- **A special dividend.** D is the total, as the open-questions record says
  of it. A special at a past year lifts that year's yield and lowers the
  figure by its share of the mean, for the n years whose window holds it. A
  special at Y lifts the figure by the whole of itself. That is the
  flattering direction, and nothing in the history marks it.
- **A high figure from a fallen price before a cut.** The form cannot tell a
  fallen price from value. The page's own words: "A yield that is unusually
  high may indicate a company has a problem and is being sold off." The
  streak, the growth rate and coverage are the metrics that say which.
- **The age of a figure at a past year.** At an older Y, the current yield
  is that year's dividend at today's close, as the yield record says of its
  own, set against the years before it. Nothing marks its age.
- **The class of a share.** Every close, the valuation date's and each past
  period end's, is one ticker's, and D is per common share. Neither v2 nor
  the crossing names a class.
- **A ticker reassigned inside the window.** The window reaches n years
  further back than any other price-dependent metric, so the analyze
  record's open gap, that nothing shows the ticker named this filer across
  the covered range, is wider here.
- **The currency.** The closes' currency and the filer's reporting currency
  are not compared, which `price-provider.md` already leaves open. A wrong
  currency in every close cancels in the quotient of yields, and one in some
  closes does not.
- **A wrong silence zero.** At Y it gives `0`. At a past year it gives an
  absence. Neither flatters.
- **A metric that reads `divCash`, finds a close by nearness, or forms the
  quotient itself.** Only review sees it.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The
  derivation needs no literal: it is divisions, one sum and sign tests. The
  preset is a literal in `settings/`, one of the three places.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, price-dependent section,
  share-basis rule and catalogue
- the share-basis record's comparisons and condition, and its E for a metric
  stated at a duration
- the arithmetic record's reading, operations and written form
- the series record's year test, series and order of reads
- the common-shareholder record's naming rule
- the yield record's metric, name, conditions and its close at the valuation
  date
- the dividend growth rate record's setting, and the dividend discount
  record's
- `negative_dividends_declared_per_share` and `non_positive_close`, each
  reached and not moved
- the provider record's ask and answer, its past period ends read by
  relative yield only
- the frozen bytes of `canonical-concepts` v2, `fetch-analyze` v1 and
  `analyze-store` v1
- the entry shape M5-18 fixed, which gains no value of `unit` and no field
- everything `M5-54` built in `reading/price.rs`

No sentence of an accepted record is superseded. Two places are named for
the decider, and neither needs one to change:

- **A past period end that is the valuation date.** The analyze record's
  "from the earlier answer" covers every case a figure can come from. In the
  one case it does not, `analyze-store` v1's witness decides, as stated
  above, and no figure follows either way.
- **Whether a yield at a past close is ever a stored metric.** This record
  says no metric states one now. A record that wants one meets the provider
  record's "for relative yield only".

**Why Structural.** The record fixes the name and meaning of a metric that
crosses `analyze-store` v1, a setting's name, meaning and domain, a new
condition's characters and a formula under the method version. It needs
nothing Constitutional: no anchor, no gate, no protected path and no
GOALS.md scope moves.

## Alternatives

- **Spare's reading, the filer's yield over the market's.** Set aside by the
  provider record: the market's yield "is neither a filing concept nor a
  price of the filer, so this provider cannot supply it". It needs a source
  and a record of its own, and no copy of Spare was opened.
- **Each past yield over the valuation date's close,** and not its own
  period end's. It reads no past close, so it needs no past answer and is
  absent less often. Rejected: with one close in every yield, the close
  cancels, and the figure is the current dividend over the mean of past
  dividends. That is a growth figure, which `dividend_growth_rate` already
  states. It says nothing of where the price stands against the filer's own
  past, which is the notion. The provider record lists the past period ends
  for this metric and no other.
- **The highest and the lowest of the past yields,** as two figures or as
  the current yield's place between them. It is the nearer shape to the
  newsletter's "Hi Yld" and "Lo Yld". Rejected:
  - Each is one year, picked by magnitude, and one special dividend or one
    fallen year-end close would be the whole reference for n years.
  - The areas the pages describe are "historically repetitive", and a single
    highest year-end is not shown to repeat. A highest of n sampled days is
    not the high the pages mean, and would be read as if it were.
  - A place between them divides by their difference, which is zero for a
    steady yield and at a window of one.
- **The median.** It ignores an outlier wholly. Rejected: the table has
  minimum, maximum and comparison, but a median orders the yields, and at an
  even n it averages the middle two. It reads each year against the others.
- **Stating only the past yield at each year,** and leaving the comparison
  to M6's query over stored results. It needs no setting, and it answers the
  yield record's open question the other way. Rejected:
  - GOALS.md names relative yield among M5's metrics.
  - It makes the yield at a past close a stored metric, a second consumer of
    the past period ends against "for relative yield only".
  - The count of years would be a ranking criterion, recorded in no result's
    premises.
- **Leaving relative yield out of M5 until the provider is built.**
  Rejected: a test builds the crossing as the contract type, so each figure
  and each absence is provable today, and the rule is on main. Deferring
  would also change milestone scope, which is Constitutional.
- **Every past year the history holds,** with no count. It needs no setting.
  Rejected: the figure would mean a different thing for each filer, with
  nothing in the result saying how many years stand behind it. It could not
  tell a short history from a gap, and the series stops at a gap by rule. A
  longer history would also widen the split exposure without limit.
- **A past year that declared nothing counted as a yield of zero,** as the
  yield record counts it and the dividend discount record's mean does.
  Rejected for the reasons above: it lowers the mean, so a wrong silence
  zero would flatter, against the test the catalogue set this silence on.
  In the dividend discount record's mean a zero lowers the value, which is
  why it may count there.
- **A past zero left out of the mean,** averaging the paying years alone.
  Rejected: it shortens the window by the figures read, which is the
  inference from magnitude, and the figure would stand on fewer years than
  the setting says.
- **Y's own yield at Y's own end among the past yields.** Rejected: Y's
  dividend would stand in the numerator and in the mean, and the figure
  would be pulled toward one.
- **The current yield at Y's own close,** and not the valuation date's. It
  reads no valuation date and is a figure of the past alone. Rejected: the
  provider record's sentence is the "current yield", and the yield record
  fixes that at the valuation date.
- **Quarter ends as further samples.** The provider record's past period
  ends include them. Rejected here: a year's dividend at a quarter's end is
  a trailing span, which no record forms, as the yield record says of a
  trailing yield.
- **The "10 percent" range as a named constant,** to state whether the yield
  stands inside it. Rejected: the page measures it from its Undervalue area,
  which this figure does not form, and a level applied to a stored figure is
  M6's.
- **The window's count shared with `dividend_growth_years`.** Rejected: one
  counts steps between two years read, the other years averaged. Moving one
  for its own reason would move the other.
- **The conditions grouped by kind,** every dividend's sign before any
  close's, as the growth rate asks a negative figure at either year first.
  Rejected: formed one after another, the current yield's condition is the
  one the yield gives at the same row, and the two columns agree.
- **The name `relative_dividend_yield`.** It is GOALS.md's phrase with the
  concept's word. Rejected: the naming rule puts `per_share` in the name of
  a form that reads a per-common-share concept, and the yield's name
  carries it.
- **One `not_asked` however many closes are read.** It is shorter to show.
  Rejected: the analyze record lists "one per absent input", and the series
  record does not merge identical reasons.

## Consequences

**Easier:**

- Relative yield has its form, years, setting, conditions, name and source.
  Its implementing task has only its code left to decide.
- The yield record's open question is answered, and the provider record's
  past period ends have their one reader.
- `date_not_asked` has a metric that gives it.
- A user with no key sees relative yield absent with `no_price`, and every
  dividend metric that reads no price computed.
- GOALS.md's fifth criterion has a record for each of its dividend metrics
  named so far.

**Harder:**

- **A filer that split inside the window is absent.** A split in fiscal year
  S leaves no figure at any year through S + n − 1. At the preset, a filer
  that split in its fiscal 2022 has no relative yield until the figure at
  fiscal 2027. This costs coverage and never gives a wrong number. Only a
  record that hands analyze the basis date narrows it.
- **A filer that declared nothing in any of n past years is absent,** a
  recent initiator and a filer that suspended once alike, until n paying
  years stand behind the year computed at.
- **Every quarter is absent,** with `not_a_year`, and so is every year with
  fewer than n years before it.
- **The caller must ask every period end the window reads.** A caller that
  asked less sees `date_not_asked` for each one missing.
- **An absence can list many reasons:** up to two for each of n + 1 years.
- **The reading added to `reading/price.rs`** is a second reader of the
  crossing's answers. The implementing task must own that file, and the
  test files that pin which metrics a crossing moves.
- The implementing task waits on the yield's, `M5-68`, since it reaches that
  task's quotient.

**Expensive to reverse:**

- The name and the form, once results are stored and ranked on.
- A past zero as a condition and a zero at Y as a figure. Changing either
  later turns stored absences into numbers or numbers into absences, under a
  bump.
- The setting's name, meaning and domain. Its preset moves freely.

**Most worth the decider's judgement:**

- The mean, where the newsletter's own tables print a high and a low yield
  and the pages give no statistic.
- A past year declaring nothing as a condition, a new one, against the yield
  record's zero as a figure.
- The preset of five year-ends, where the page says "multiple market
  cycles".
- The name, whose words are also Spare's title.
- The answer read where the crossing holds it, for a past period end that
  is the valuation date.
- Conditions asked only where every read gave a number, so that this metric
  and the yield can give different reasons at one row.

**Open, and not this record's:**

- Spare's reading, which needs its own source and record.
- Whether a yield at a past close is ever a stored metric.
- A profile of high and low yield areas, which needs closes between period
  ends that the provider record does not return.
- The regular dividend alone, which waits on a `fetch-normalize` version
  that carries the axis and member, as the open-questions record says.
- The basis date, which would let a metric compute across a split.

## Enforcement

This applies anchor 5, the price invariant and the one-source-of-truth
invariant, and edits none of them.

- **No literal.** `literals` keeps every number but zero and one out of the
  derivation. The metric needs none: the count is the setting.
- **The count is a setting.** The `Settings` constructor refuses a value
  outside the domain, and the result records the value used.
  - **Gap:** nothing stops the derivation calling the preset constructor, as
    the analyze record already says.
- **One definition of each condition.** Two are the existing `const`s, and
  the third is defined once beside the metric.
  - **Gap:** nothing stops a second `const` with the same characters, except
    review.
- **One quotient.** The yield's function is the one place a dividend is set
  over a close.
  - **Gap:** nothing stops a second division written beside it, except
    review.
- **The dividend is the filing's.** The crossing carries no dividend, so the
  metric cannot read the provider's.
- **No wrong number across a split.** The share-basis tests hold it for
  every pair, the split dated inside the window among them.
- **No figure without every price.** The witness tests hold it:
  `not_asked`, `date_not_asked` and `answered_absent`, each at a past period
  end.
- **A past close is found by its date asked alone.**
  - **Gap:** nothing stops a reading that takes a near date's answer, except
    review and the test of a period end not asked.
- **The source says what the record says.**
  - **Gap:** nothing checks that a citation is true, and the pages can
    change.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the name and meaning
  of one metric that crosses `analyze-store` v1 as characters, one setting's
  name, meaning and domain, one new condition's characters and a form under
  the method version, all inside analyze. It edits no anchor, no protected
  path, no gate, no contract byte, no schema and no milestone scope, and
  adds no reason and no constant. Flagged for later human review, as the
  tier requires.
- **Checked:**
  - Every quotation of an accepted record or contract against main,
    whitespace collapsed: all found, each record named reading Accepted. In
    `canonical-concepts` v2 the concept applies to all five kinds and its
    silence is conditional, in the words quoted. `analyze-store` v1 and the
    analyze record give `date_not_asked` to a crossing that holds no answer
    for the date, and one answer per date asked.
  - The code it leans on: `Year::series` stops at a gap as the record says;
    `negative_dividends_declared_per_share` is defined once beside the
    streak and `non_positive_close` once beside the discounted cash flow
    margin; `dividend_discount_base_years` is a `NonZeroU16`;
    `reading/price.rs` holds the rule and the one reading at the valuation
    date; a figure is written at 34 significant digits.
  - The five newsletter pages were fetched whole in the deciding run, on
    2026-11-23. All twenty-one quotations stand, the two pages the task did
    not name included. No page states a formula, a statistic of the past
    yields or a count of years. The data tables page ties its Undervalue and
    Overvalue prices to a "historical high yield profile" and a "historical
    low yield profile", which is the shape the record says it does not form.
  - Chapter 10 was downloaded whole again: 27 sheets, 4,678,827 bytes, at
    the sha256 the record names, the transfer resumed three times. Printed
    pages 3, 4 and 16 carry each quotation, the illustration's title, its two
    row labels and the five cells quoted.
  - The proof re-derives in exact fractions: `1.25`; `1` over the close of
    75; 1.5 ÷ 0.97 over the valuation date's close; five thirds and ten
    ninths as written; `1.25` at a setting of 1; `1.5625` for the counted
    zero. Each split case was walked against the third clause for its own
    pair, the trading date `2021-12-3` against the share-basis dates
    record's second clause, which reads every price's trading date, and each
    order of reasons against the series record's order of reads.
  - No accepted record is contradicted. The yield record's open question is
    answered inside what it left open. The provider record's past period
    ends keep their one consumer. The analyze record's "from the earlier
    answer" is not superseded: where a past period end is the valuation
    date, the current pair's later date lies past the covered range's end,
    so no figure can follow from either reading of that case.
  - The alternatives as argued: Spare's reading, one close under every
    yield, the highest and lowest, the median, the stored past yield, every
    year the history holds, the counted zero, the zero left out, and the
    wait for the provider.
- **Verdict and why:** Accepted. The provider record already fixed whose
  reading this is and which closes it may read, and the open primary states
  the notion and nothing that would fix a statistic. Given one close a year,
  the newsletter's high and low yield areas cannot be formed, and a highest
  of n year-ends would be read as if it were one. The mean claims less, is
  moved one n-th by any one year, and the `meaning` says exactly what was
  computed. Declining at a past year that declared nothing is the decision I
  weighed longest, since it costs every recent initiator the metric. It
  holds because the catalogue set this concept's silence to zero on the
  promise that a wrong zero cannot flatter a consumer, relative yield is a
  consumer it names, and a counted zero raises the figure. An absence that
  names the year keeps that promise, and a figure would break it. The count
  of years is a setting with a stated reason and no claim beyond it.
- **What would have changed it:** a page of the primary that states a
  statistic, a formula or a count of years; an accepted record that gives
  the past period ends to another use or reads relative yield another way;
  a silence test that did not name relative yield among the concept's
  consumers, which would have let a past zero count; a wording of
  `date_not_asked` that could be built where the crossing holds an answer
  for the date; a figure in the proof that did not re-derive; or a quotation
  that main or a copy does not carry.
- **Flagged for the human:** the mean as the record's own statistic, where
  the newsletter judges a yield against its historical high and low yield
  areas, which need closes between period ends; a past year that declared
  nothing as a condition, so a recent initiator or a filer that suspended
  once is absent until n paying years stand behind the year computed at; a
  split inside the window making the figure absent for up to n years; the
  preset of five year-ends, not shown to span "multiple market cycles"; a
  past period end that is the valuation date read from the crossing's one
  answer for it; at an older year, that year's dividend at today's close set
  against older yields at their own closes, with nothing marking its age;
  and the name, whose words are also Spare's title.
