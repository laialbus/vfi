# The share-basis rule compares dates through the one ordering in `date.rs`, which now serves the alone clause and that rule alone; a date the rule compares that is outside the grammar declines under one condition defined beside the rule

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-24, by `M5-48`
- **Decided:** 2026-10-26, by the decider
- **Touches:** one sentence of the accepted
  `which-duration-precedes-a-duration.md`, said of the ordering it adds to
  `date.rs`: "It is used by the alone clause and by nothing else. It never
  seeks a row, never sorts, and never picks the latest." This record
  supersedes that sentence with a statement of its own (below), naming the
  ordering's two uses. It answers the question
  `discounted-cash-flow-and-margin-of-safety.md` (Accepted) leaves open, and
  the one `which-other-period-a-metric-reads.md` (Accepted) leaves to the
  share-basis rule's task: what the rule does with a trading date outside the
  grammar. It fixes the characters of one `declined` condition, which crosses
  `analyze-store` v1 as an open string, and adds no reason. No anchor, no
  contract byte, no schema, no gate, no protected path and no milestone scope
  moves. Nothing here is implemented.

## Context

`what-analyze-takes-and-returns.md` (Accepted) fixes the share-basis rule in
three clauses that compare dates. Its Enforcement says the rule holds once it
is written as "the one function every price-dependent metric calls". The
discounted cash flow record makes the margin of safety the first metric to read
a price, so the margin's task is the first to write the rule.

Three sentences of accepted records meet that task:

- **The series record**, of the one ordering it adds to `date.rs`: "It is used
  by the alone clause and by nothing else. It never seeks a row, never sorts,
  and never picks the latest."
- **The discounted cash flow record**, which lists under "Open, and not this
  record's": "Whether the share-basis rule may use the ordering the series
  record added." Of the sentence above, it says: "The first task to write the
  share-basis rule meets that sentence. This record adds no comparison of its
  own. Whether the rule may use that ordering, or needs a record to say so, is
  not decided here. If that task reads the sentence as forbidding it, it
  stops."
- **The period record**, of `date.rs`: "One that needs more, such as ordering,
  adds it here under its own record, and never beside it. What the share-basis
  rule does with a trading date outside the grammar is its own task's."

The rule's covered range is the earliest and the latest trading date among the
prices. That is picking the latest. Read as written, the series record's
sentence forbids the rule the only ordering it may use, and the period record
forbids it a second one. This record settles it, as `sources-a-task-can-read.md`
settled one sentence of the period record.

Some things are already fixed:

- **The crossing carries dates as characters.** `fetch-analyze` v1: "dates,
  closes and factors cross as characters: a date asked as the caller named it,
  everything else as the source published it". A trading date or a split's
  date can be outside the grammar, and nothing upstream checks it.
- **`date.rs` is the one reader.** `Date::read` accepts RFC 3339's
  `full-date` and nothing else. `Date::on_or_before` is the one ordering, and
  its one caller today is the alone clause in `metrics/reading/year.rs`.
- **Absence is closed.** Six reasons. `basis_unproven` has three witnesses:
  `splits_absent`, `not_covered` and `split_within`. `analyze-store` v1 gives
  `not_covered` the covered range, the period end and the trading date, and
  `split_within` the split, the period end and the trading date.
- **The series record's year test** reads a duration's end first and gives
  `end_not_a_date` where it is outside the grammar.

## Decision

### The comparisons the rule makes

Each is named from the analyze record's text. None is added.

1. **The covered range.** It "runs from the earliest trading date any price in
   the crossing carries, through the latest". That is a choice of the earliest
   and the latest among the trading dates of the prices answered at the
   valuation date and at the earlier dates. "A date asked whose answer is an
   absence proves nothing about the range, so it does not widen it."
2. **The earlier and the later of E and T.** "The earlier of E and T" and "the
   later": a choice between the figure's period end and the close's trading
   date.
3. **The second clause.** The covered range "starts on or before the earlier
   of E and T, and ends on or after the later".
4. **The third clause.** The list "holds no split dated on or after the
   earlier of E and T".

**The first clause compares no dates.** It asks only whether the split list is
present.

### How they are made

**Every comparison is made through `Date::on_or_before`, the one ordering
`date.rs` holds.** "On or after" is the same ordering with its two dates
swapped. No second ordering is defined, and no date is compared as
characters.

- **The covered range.** Its start is the trading date that is on or before
  every other, and its end is the one that every other is on or before. In the
  grammar a date has exactly one spelling, so the characters `not_covered`
  carries for each end are determined. The answers at the earlier dates are a
  set whose order means nothing, and the range does not depend on their order.
- **The earlier and the later.** Where E is on or before T, E is the earlier
  and T the later. Otherwise T is the earlier and E the later.
- **The second clause** holds where the range's start is on or before the
  earlier, and the later is on or before the range's end.
- **The third clause** fails at a split where the earlier is on or before the
  split's date. Where several splits fail it, `split_within` names the first
  the list holds, in the order the crossing holds it. No split is chosen by
  its date, and the list is not sorted.

Whatever the rule reads, these hold:

- **No row of the history is found by order.** The figure's period is the one
  the metric is computing at, and E is read from it. The rule seeks nothing.
- **The history is never sorted,** and neither is the crossing or its list.
- **Of dates, the rule reads only these:** each price's trading date and each
  split's date, from the crossing, and the figure's period end. It reads no
  date asked, so neither the valuation date nor an earlier date asked is
  compared. Beyond dates, it reads only whether the split list is present.

### What the ordering's sentence becomes

The series record's sentence is superseded by this one:

> The ordering is used by the alone clause of
> `which-duration-precedes-a-duration.md`, and by the share-basis rule of
> `what-analyze-takes-and-returns.md` with its covered range, and by nothing
> else. It never seeks a row and never sorts. It picks the earliest and the
> latest only among the trading dates of the prices one crossing carries, to
> form the covered range, and the earlier and the later only of a figure's
> period end and a close's trading date. It picks no period, row or date of
> the history.

Of the old sentence, "never seeks a row" and "never sorts" still hold as
written. "Never picks the latest" holds for the history, and does not hold for
the covered range, which picks the latest trading date among the prices. That
is the one change. The period record's "never beside it" holds, because no
ordering is added. The `date.rs` module comment, which today says "for the
test of whether a duration is a year and for nothing else", is the
implementing task's to rewrite under this record.

### A date outside the grammar

Any date the rule compares can be outside the grammar `Date::read` accepts.
Every such date gives one outcome: `Absent` with exactly one `declined`
reason, the condition **`share_basis_date_not_a_date`**.

- **What it covers.** A trading date of the paired close; a trading date of
  any other price in the crossing; a split's date; and E.
- **Why `declined`.** The price was answered, and the rule, which is part of
  the metric's definition, is undefined at a date that states no day. This is
  the argument the series record makes for `start_not_a_date` and
  `end_not_a_date`, and the discounted cash flow record for a close outside the
  decimal grammar under `input_not_a_decimal`.
- **Why not a `basis_unproven` witness.** `not_covered` names a covered range
  in characters, and there is none to name when one trading date cannot be
  ordered. `split_within` is built from a split "dated inside the span", and a
  split whose date states no day is not shown to be there. A fourth witness
  would change the analyze record's three and `analyze-store` v1's bytes.
- **The valuation date, or any date asked, outside the grammar gives
  nothing.** The rule does not read it. The margin reads the answer at the
  valuation date by its place in the crossing and compares the trading date
  that answer carries.
- **E for a metric stated over a year** never reaches the rule outside the
  grammar. The year test asks first and gives `end_not_a_date`. For a metric
  stated at an instant, or at another duration, an E outside the grammar gives
  `share_basis_date_not_a_date`.

**Where it is asked.** Each clause reads only the dates it compares, in
clause order, and the first clause that fails or cannot be read gives the one
reason:

1. The first clause reads no date. An absent list gives `splits_absent`,
   whatever the crossing's dates are.
2. The second clause reads E, T and every price's trading date. If any is
   outside the grammar, the reason is `share_basis_date_not_a_date`.
   Otherwise, a failing clause gives `not_covered`.
3. The third clause reads every split's date before comparing any. If any is
   outside the grammar, the reason is `share_basis_date_not_a_date`.
   Otherwise, a failing clause gives `split_within`.

**Defined once, listed by each entry.** The condition is a `const` defined
once, beside the rule. Every metric that calls the rule lists it among its
`conditions`, as every metric stated over a year lists the series record's
four. Today that is the margin alone. Its entry lists it after the four year
conditions. The value per share reads no price and does not list it.

### One price, and none

- **One price,** the valuation date's. The covered range runs from that
  close's trading date through the same date. The second clause holds only
  where E is that date. So a crossing asked the valuation date alone, with a
  close after E, gives `not_covered`, naming the one-day range.
- **No price.** The analyze record says "There is no covered range when the
  crossing carries no price, and then no metric reads one." The rule is asked
  only once a metric has a price at the date it reads. A crossing with no
  price gives that metric `no_price` by one of its three witnesses first, so
  the rule is never asked. When it is asked, the paired close is a price in
  the crossing, so a covered range always exists.

### E, and one day

- **How E is read.** E is "an instant's date or a duration's end", as the
  analyze record says. For a metric stated at a duration, as both discounted
  cash flow metrics are, E is the duration's end: for the margin, the end of
  Y. It is read through `Date::read`.
- **E and T on one day.** The earlier and the later are that one day. The
  second clause holds where the range starts on or before it and ends on or
  after it. The third fails at a split dated that day.
- **A split dated the earlier of E and T declines,** under "on or after". The
  discounted cash flow record's proof already requires it: "A split dated
  2024-12-31 itself: `split_within` too, under the clause's "on or after the
  earlier of E and T"".
- **E after T.** The second clause needs a price traded on or after E. Under
  `price-provider.md`, every close is on or before its date asked, and the
  request runs "up to the valuation date". So where Y ends after the valuation
  date, the margin is `not_covered`. That follows from the accepted clause, and
  this record adds nothing to it.

This record decides nothing of the discounted cash flow record's margin, of
the value per share, or of any metric. It changes no reason's carriage:
`not_covered` and `split_within` carry what `analyze-store` v1 gives them and
no more.

### Where the rule lives

The rule is written once, as the analyze record's Enforcement asks, "the one
function every price-dependent metric calls". Where it lives in the crate, and
how the crossing reaches it, are the implementing task's. The discounted cash
flow record sets the limits:

- The crossing is read only through the three `no_price` witnesses and the
  rule.
- No other metric's reads, reasons or figures change.
- What `analyze` hands store as `Prices` does not change.

### The method version

The rule, its clause order, the condition and the ordering's two uses are part
of the method `METHOD_VERSION` names, as the analyze record places "a rule of
absence or composition, the share-basis rule below included". A later change
to any of them is a version bump and a superseding record. It lands at version
1: no metric reads a price yet, so no stored outcome changes.

### The proof the implementing task brings

These tests are built through `History::of`, `Row::at` and a `Crossing` built
as the contract type. They read no file. Each stands in `tests/` or under
`#[cfg(test)]`. Unless a test says otherwise, they use the discounted cash flow
record's preset row: the year 2024-01-01 to 2024-12-31, a value per share of
`80`, a valuation date of 2025-06-30 answered with a close of 60 traded
2025-06-30, the earlier date 2024-12-31 answered with a close traded
2024-12-31, and an empty split list. That gives a margin of `0.25`. At the
least:

- **The four the discounted cash flow record lists:**
  - The split list absent: `splits_absent`.
  - A covered range that starts after 2024-12-31, the earlier date
    2025-03-31 answered with a close traded 2025-03-31: `not_covered`, naming
    2025-03-31 through 2025-06-30, the period end and the trading date.
  - A split dated 2025-03-14: `split_within`, naming the split, the period
    end and the trading date.
  - A split dated 2024-12-31 itself: `split_within`.
- **The far side of the bound.** A split dated 2024-12-30: `0.25`.
- **The valuation date alone.** A crossing asked the valuation date alone,
  holding no price at or before 2024-12-31: `not_covered`, naming 2025-06-30
  through 2025-06-30, and never `date_not_asked`.
- **The one-price crossing.** A valuation date of 2024-12-31 answered with a
  close of 60 traded 2024-12-31 and no earlier date: `0.25`. With a split
  dated 2024-12-31 beside it: `split_within`.
- **E after T.** A valuation date of 2024-06-28 answered with a close traded
  2024-06-28, the earlier date 2023-12-29 answered with a close traded
  2023-12-29: `not_covered`.
- **Each date outside the grammar** gives `declined` with
  `share_basis_date_not_a_date`:
  - the paired close's trading date `2025-6-30`
  - the earlier price's trading date `20241231`
  - a split's date `2025-02-30`
  - E `2024-12-1`, through the rule's own function under `#[cfg(test)]`,
    since no metric that calls it today hands it such an E
- **What gives nothing.**
  - The split list absent beside the trading date `2025-6-30`:
    `splits_absent`, since the first clause reads no date.
  - A valuation date asked as `2025-6-30`, answered with a close traded
    2025-06-30: `0.25`, since no date asked is read.
  - An earlier date asked as `20241231`, answered with an absence, beside the
    preset crossing's other answers: `0.25`.
- **No order is read.**
  - The earlier answers in either order give the same outcome.
  - Two splits dated 2025-01-15 and 2025-03-14: `split_within` names the
    first the list holds, in each order.
- **The catalogue.** The margin's entry lists `share_basis_date_not_a_date`,
  and the value per share's does not. The constant is written as the record
  names it.

### What no check can see

- **Whether the source publishes its dates in the grammar.**
  `fetch-analyze` v1 carries a trading date and a split's date "as the
  characters the source published". Tiingo's
  <https://www.tiingo.com/documentation/end-of-day>, read in this run, gives
  the field `date` the data type "date" and the description "The date this
  data pertains to." It does not say which characters. If the source writes a
  date-time, every price-dependent metric is `share_basis_date_not_a_date` for
  every filer, and every gate stays green. The provider's implementing task
  reads the characters off a real response, and nothing compares them with
  `Date::read`.
- **A third caller of the ordering.** `on_or_before` is visible across the
  crate, and only review sees a use beyond the two named.
- **A metric that compares dates itself,** as characters or by a second
  ordering, or that finds a price by nearness. Only review sees it, as the
  period record says of a metric that finds a row itself.
- **The named split where several qualify.** It follows the list's order. A
  store that does not keep that order replays the same reason naming another
  split. How store keeps the list is M6's.
- **A split after the latest price,** which the analyze record's Enforcement
  already names. Nothing here narrows it.
- **Whether "on or after" is the right edge.** The analyze record chose it
  because a split's date "has not been read off a real response". It still has
  not.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves. The rule
  needs no number.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three clauses, and three
  witnesses of each kind
- the series record's year test, seek and conditions
- the frozen bytes of `fetch-analyze` v1 and `analyze-store` v1
- the discounted cash flow record's two metrics, settings and conditions. The
  margin's entry lists one more condition, which is the rule's, as it lists
  the series record's four, which are the series record's. Its own five do
  not change.

No question here needed one of these to change.

**Why Structural.** The record supersedes one sentence of an accepted record,
as `sources-a-task-can-read.md` narrowed one, and fixes the characters of a
`declined` condition that crosses `analyze-store` v1, as the series record
fixed three. It needs nothing Constitutional.

The ADR README's table of partial supersessions should gain this record's row.
The README is outside this task's owns, so that is a human's or a later
task's.

## Alternatives

- **A second ordering in a module of the rule's own.** It leaves the series
  record's sentence untouched. Rejected:
  - Two definitions of one relation is unchecked duplication, which the
    one-source-of-truth invariant of ANCHORS.md bans. Nothing would fail if
    the two disagreed.
  - The period record says an ordering is added to `date.rs` "and never
    beside it". So this would supersede a sentence too, and a weaker one to
    lose.
- **Comparing the dates as characters.** In the grammar, fixed-width digits
  order as the calendar does, so it needs no ordering. Rejected: outside the
  grammar it orders wrongly and says nothing. `2025-6-30` sorts after
  `2025-12-31`, since `6` follows `1`, so a June close reads as later than
  December's and a covered range can appear to reach a date it does not.
  That is a wrong number that looks right. Reading each date through
  `Date::read` first, to refuse such a date, makes the character comparison a
  second ordering anyway.
- **The covered range written into the crossing by the caller.** Rejected, as
  the analyze record rejected it: the caller does not know the lookback or a
  shortened range, so the range would be a claim and not evidence. It would
  also change `fetch-analyze` v1's bytes.
- **Leaving the margin of safety out of M5 until a human answers.** Rejected:
  GOALS.md's fourth criterion names it, so deferring it changes milestone
  scope, which is Constitutional. The question needs one sentence superseded,
  which a Structural record can do.
- **Skipping a price whose trading date is outside the grammar** when forming
  the covered range. A narrower range is still inside the range the request
  covered, so it never gives a wrong number. Rejected: it reads "any price in
  the crossing" as "any price whose date can be read", which narrows an
  accepted definition without saying so. It also chooses among the prices.
- **A split with an unreadable date read as `split_within`.** Rejected: the
  witness is a split "dated inside the span", and this one is not shown to be.
- **`end_not_a_date` or `start_not_a_date` for the crossing's dates.**
  Rejected: each names a duration's own date under the year test or the
  opening seek. Reading one for a trading date would make one condition mean
  two things.
- **A condition per kind of date,** one for a trading date and one for a
  split's. It says which date failed. Rejected: the crossing goes to store
  beside the result, so a replay shows which. One condition is one entry line
  in every entry that lists it.
- **Leaving dates outside the grammar to the implementing task,** as the
  period record did. Rejected: the condition's characters cross
  `analyze-store` v1 and every entry under the rule lists them. A code task
  naming them alone is the decision this record exists to take.

## Consequences

**Easier:**

- The margin's task can write the rule without stopping. Every comparison has
  one place to go.
- `date.rs` still holds one ordering, with its uses named.
- A date the rule cannot read is an absence that names itself, never a wrong
  number.

**Harder:**

- Every metric that calls the rule lists one more condition.
- A change to the ordering now moves two rules, and is a version bump for
  both.
- If the price source writes its dates outside the grammar, every
  price-dependent metric is absent until a contract version carries them in
  it.

**Expensive to reverse:**

- The condition's characters, once results are stored with them.
- The ordering's two uses, once the margin is stored and ranked on.

**Open, and not this record's:**

- Which characters the source publishes for a date. That is the provider's
  implementing task's to read.
- How store keeps the split list's order. That is M6's.

## Enforcement

This applies anchors 4 and 5 and the one-source-of-truth invariant, and edits
none of them.

- **Anchor 4.** The rule reads only the characters it is handed, with no clock
  and no dependency. `purity` and `ambient` read what they read today.
- **Anchor 5.** The rule needs no number, and `literals` keeps one out.
- **One ordering.** Only one is defined.
  - **Gap:** nothing forces a comparison of dates through `date.rs` rather
    than around it, as the period and series records already say.
- **No wrong number across a split.** The proof's tests hold the rule for the
  margin.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the analyze record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it supersedes one sentence
  of an accepted record, as `sources-a-task-can-read.md` did, and fixes the
  characters of one `declined` condition, which crosses `analyze-store` v1
  as an open string. It edits no anchor, no protected path, no gate, no
  contract byte, no schema and no milestone scope, and asks for no allowlist
  entry. Flagged for later human review, as the tier requires.
- **Checked:**
  - Every quotation, as printed: the series record's sentence; the
    discounted cash flow record's open item and the four sentences it says
    of it, and its "split dated 2024-12-31 itself" test; the period record's
    "never beside it" and "its own task's"; the analyze record's covered
    range, the absence that does not widen it, the two clauses, "the one
    function every price-dependent metric calls", "an instant's date or a
    duration's end", "no covered range" and "the share-basis rule below
    included"; `price-provider.md`'s "up to the valuation date"; and
    `fetch-analyze` v1's "a date asked as the caller named it, everything
    else as the source published it".
  - The code: `date.rs` line 7 reads "for nothing else" as the record
    says; `Date::on_or_before` is defined once, and its one caller outside
    tests is the alone clause at `year.rs` line 145.
  - The task's acceptance, item by item, against the record: the four
    comparisons and the first clause's none; the three choices weighed and
    what of the old sentence still holds; the condition within the six
    reasons, defined once, listed by each caller's entry, with the one-price
    and no-price cases; E, one day, and the split dated the earlier; the
    rule's place left to the implementing task within the discounted cash
    flow record's three limits; the proof, which carries the four tests that
    record lists, the valuation date alone, the one-price crossing, E after
    T, one test per kind of unreadable date and order independence; what no
    check can see; the tier; the nine alternatives, the three required
    among them; nothing proved narrower.
  - Tiingo's end-of-day page, fetched in this run: it renders by script and
    the copy carries no field table, so the "date" description is the
    proposer's reading alone. The record names the gap itself.
- **Verdict and why:** accepted. One ordering with its two uses named is
  the one-source-of-truth invariant applied; a second definition is a
  relation nothing would check, and comparing characters orders `2025-6-30`
  after `2025-12-31`, the wrong number that looks right. The condition is a
  `declined` one inside the six reasons, so the frozen bytes stand, and it
  is defined once and listed as the series record's four are. The old
  sentence loses exactly one clause, "never picks the latest", and the record
  says which and why instead of reading around it.
- **What would have changed it:** a seventh reason or a fourth
  `basis_unproven` witness; a changed byte of `fetch-analyze` v1 or
  `analyze-store` v1; a second ordering or a character comparison; a price
  skipped when forming the range; a sentence superseded without being named;
  or a quotation not found as printed. Flagged for the human beside the
  tier: the source's date characters are unread, and if they are outside the
  grammar every price-dependent metric is absent while every gate stays
  green; and the ADR README's supersession table lacks this record's row,
  which no task owns.
