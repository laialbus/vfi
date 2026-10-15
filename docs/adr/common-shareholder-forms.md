# The common-shareholder forms read common equity and income available to common, each formed once; the common dividend has no total v2 can form, so payout and coverage take their per-share forms as the common ones, and retention and sustainable growth are built on those

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-14, by `M5-27`
- **Decided:** 2026-10-14, by the decider
- **Touches:** the composition `canonical-concepts-open-questions.md` leaves to
  analysis when it admits `preferred_equity` and `preferred_dividends` for "the
  common-shareholder forms". It defines retention and sustainable growth,
  which GOALS.md's fifth M5 criterion names and no record states. It fixes the
  names and meanings of metrics that cross `analyze-store` v1 as characters,
  and no byte of that contract changes. It applies the accepted `declined`
  reason to one new condition, `zero_common_equity`, and adds no reason. It
  names one term v2 cannot form, the common dividend, and leaves it open for
  a record of its own. No accepted record is superseded. No contract's bytes,
  no schema, no edge, no gate, no dependency and no entry field move. Nothing
  here is implemented.

## Context

GOALS.md's fourth M5 criterion names returns on capital and valuation
multiples. Its fifth names payout on earnings, coverage, retention and
sustainable growth. The open-questions record fixes that "M5's metrics are
common-shareholder measures" and admits two concepts for exactly the common
forms of these. Five metrics read one or more of three common terms. If each
metric task formed them, there would be five readings of one rule, which
anchor 5 and the one-source-of-truth invariant forbid.

Some things are already fixed:

- **Common equity.** The open-questions record states it:
  `shareholders_equity` less `preferred_equity`. v2's meaning of
  `shareholders_equity` repeats it.
- **The preferred deduction.** v2 defines `preferred_dividends` as "the
  deduction between the period's profit and the profit available to common
  shareholders": preferred dividends declared for the period, with the
  accretion and redemption adjustments charged in the same reconciliation.
  It is a magnitude. It is positive when it reduces income available to
  common, and negative where the filer's reconciliation adds to it.
- **No common dividend total.** `dividends_paid` is cash paid from financing
  to "all classes of shareholder, common and preferred together".
  `dividends_declared_per_share` is per common share. No concept carries the
  common dividend as a total.
- **The registry already reads `dividends_paid` narrower at times.**
  `declared-cash-dividends-stand-in.md` (Accepted) admits
  `DividendsCommonStockCash` as a stand-in for it, and says that element
  "leaves out the preferred that a filer paying both also pays". It admits
  `DividendsPreferredStock` as a stand-in for `preferred_dividends`, "the
  dividends alone without the accretion and redemption adjustments".
- **The two forms M5 already computes.** M5-22 computes payout on earnings,
  payout on free cash flow and coverage in their total form, over
  `dividends_paid`, as all-holder measures. M5-23 computes payout and coverage
  per share, over `dividends_declared_per_share` and
  `earnings_per_share_diluted`. The open-questions record calls these
  "already common-specific".
- **Neither preferred concept is ever `NotApplicable`.** v2 applies both to
  every kind. Silence reads `Unknown` for both.
- **Absence is closed.** The accepted analyze record allows six reasons. One
  `NotApplicable` exception exists, `short_term_investments` inside net debt.
  A metric built on another carries the other's reasons.
- **Which period a balance is read at** is
  `which-other-period-a-metric-reads.md` (M5-26). On main as this is written,
  that record is **Proposed, not Accepted**.

What the fixtures show, read off the five normalize fixtures' claims:

- `preferred_dividends` is `Unknown` at ten of the eleven rows claimed for it.
  It resolves once: 122,877 from `DividendsPreferredStock`, a stand-in, on
  CIK 0001859199.
- `preferred_equity` is `Unknown` at nine rows and a read `Value` of 0 at
  three.

## Decision

### The three terms, each formed once

The terms are one module of the derivation,
`crates/analyze/src/metrics/common_shareholder.rs`. Every metric that reads a
common term reads it through that module. No metric writes the subtraction
itself.

**Common equity** is `shareholders_equity` less `preferred_equity`, at one
instant.

- This record does not decide it. It carries the open-questions record's
  statement, which v2's meaning repeats.
- The source: Penman, S. H., *Financial Statement Analysis and Security
  Valuation*, 5th ed., McGraw-Hill Education, 2013. Penman measures common
  shareholders' equity as the book value of equity less the preferred claim,
  which ranks ahead of the common holder. The CFA text below states return on
  common equity over common equity on the same terms.
- The inputs are read in this order: `shareholders_equity`, then
  `preferred_equity`.

**Income available to common** is `net_income` less `preferred_dividends`,
over one duration.

- v2 defines `preferred_dividends` as the deduction between the period's profit
  and the profit available to common. So the difference is income available to
  common by that definition, and no other term is needed.
- The sign works without a branch. A positive deduction lowers the income. A
  negative one, from a redemption below carrying amount, raises it above net
  income, as the source says it should.
- The inputs are read in this order: `net_income`, then `preferred_dividends`.
- Both are stated from continuing operations, attributable to the parent.
  Income available to common is deducted from that line, and v2's `net_income`
  is that line.
- The sources:
  - Financial Accounting Standards Board, Statement of Financial Accounting
    Standards No. 128, *Earnings per Share*, 1997, ¶9, codified at ASC
    260-10-45-11. Income available to common is income from continuing
    operations, and net income, less the dividends declared on preferred stock
    and the dividends accumulated for the period on cumulative preferred.
  - Emerging Issues Task Force, Topic No. D-42, *The Effect on the Calculation
    of Earnings per Share for the Redemption or Induced Conversion of Preferred
    Stock*, SEC staff announcement, 1995, codified at ASC 260-10-S99-2. A
    redemption's difference from carrying amount is subtracted from net income,
    or added to it, to reach income available to common.

**The common dividend has no total v2 can form.** The term is left open, by
name, as **the common dividend total**. A `canonical-concepts` v3 concept
carrying it is proposed only in a record of its own.

There are three candidates, and each gives a wrong number of the kind the
open-questions record describes: a figure that looks right and is not.

- **`dividends_paid` less `preferred_dividends`.** Three things go wrong:
  - *The preferred can be deducted twice.* The accepted stand-in record admits
    `DividendsCommonStockCash` for `dividends_paid`, and that element is the
    common dividend already. Subtracting `preferred_dividends` from it removes
    the preferred a second time. The resolved value records the stand-in, but
    a metric reads the amount and not how it was set.
  - *Accretion and redemption adjustments are not dividends paid.* Accretion
    is non-cash. A redemption premium is paid on the financing line for
    redeeming preferred, not on the dividends line. Both sit in
    `preferred_dividends`, so the subtraction takes them from a cash figure
    that never held them. Suppose a filer pays common dividends of 100 and
    preferred cash dividends of 10, and charges accretion of 40. Then
    `dividends_paid` is 110, `preferred_dividends` is 50, and the difference is
    60, not 100. The common payout reads 40% low and coverage two-thirds high,
    both in the flattering direction. A negative `preferred_dividends` gives a
    "common dividend" larger than everything paid.
  - *Declared is not paid.* A preferred dividend declared in one period and
    paid in the next is subtracted from the wrong period's cash.

  The sources that fix this: the open-questions record and v2 for what each
  concept holds; the stand-in record for what `dividends_paid` may resolve
  from; SFAS No. 128 ¶9 and EITF D-42 for why the deduction holds amounts that
  no cash dividend holds.
- **`dividends_declared_per_share` times a share count.**
  - `shares_outstanding` is a count at an instant "at or near the period end",
    and may be the cover page's date. A dividend declared per share over the
    period was paid on whatever shares stood at each record date, so buybacks
    and issuance move the product away from the total.
  - `diluted_shares_weighted_average` counts options and convertibles, which
    receive no dividend.
  - Either way, the product looks like the common dividend and differs from it
    by the share movements.
- **No other concept.** None of v2's twenty-eight concepts carries any other
  common dividend amount.

The stand-in record's own list for the next vocabulary version names a
currency `dividends_declared`. That concept is all classes, so it would not
close this gap alone.

### Which metrics take a common form

| Metric GOALS.md names | Forms M5 computes | Name of each form |
| :--- | :--- | :--- |
| Return on equity | total and common | `return_on_equity`, `return_on_common_equity` |
| Price/book | common alone | `price_to_common_book` |
| Dividend payout on earnings | total, and per share as its common form | `dividend_payout_on_earnings`, `dividend_payout_per_share` |
| Dividend coverage | total, and per share as its common form | `dividend_coverage`, `dividend_coverage_per_share` |
| Retention | per share alone, a common form | `retention_per_share` |
| Sustainable growth | common alone | `sustainable_growth_on_common_equity` |

**Common alone.** Two metrics have no total form:

- **Price/book** sets market capitalisation, a common claim by construction,
  over book. Over total equity it is the incoherent figure the open-questions
  record refused.
- **Sustainable growth** is defined below on the common claim.

**Total alone.** Every other value or dividend metric GOALS.md names either
stands in its total form or forms no common term:

- **Total form alone:**
  - the margins
  - return on assets, on capital employed, on invested capital
  - asset turnover
  - debt to equity
  - liabilities to assets
  - the liquidity ratios
  - payout on free cash flow, since free cash flow has no common form
  - Altman's terms and Piotroski's signals, whose authors fix their own inputs
- **Common by the concept they read, with no term to form:** price/earnings,
  yield and relative yield, dividend growth and streak, and the dividend-based
  valuation. Each reads a per-common-share concept and needs nothing here.

None of these is a consumer the open-questions record admitted the preferred
concepts for. A later metric that reads common equity or income available to
common forms it through the one module, and takes the naming rule below.
Whether enterprise value counts the preferred claim, and what book value
Graham's screens read, are those metrics' own tasks. If either reads a common
term, it reads it here. If enterprise value wants `preferred_equity`, that is
a new consumer the open-questions record did not admit, and its own record.

**The per-share forms are the common forms** of payout on earnings and
coverage. No common total form stands beside them in M5, because the common
dividend total is open. If a v3 concept later carries that total, common total
forms can join beside the per-share ones. They would be named under the rule
below. The per-share forms would keep their names and meanings, and adding a
metric bumps nothing.

**Both forms are metrics of the method** wherever both exist. Each has its own
name, and ranking chooses between them over stored results. The total form of
return on equity is coherent: both sides include the preferred. It is also the
only return on equity most filers get, as "How absence composes" below shows.
`which-other-period-a-metric-reads.md` already names both.

### How the forms are named and described

- **`common`** appears in a metric's name exactly when its derivation forms
  common equity or income available to common. Its `meaning` names each such
  term in words, as the formula has it: "net income less preferred dividends",
  "shareholders' equity less preferred equity".
- **`per_share`** appears in the name of a form that reads per-common-share
  concepts. Its `meaning` says "per common share", or "each common share's",
  as M5-23's two entries already do.
- **A total form** has neither word. Where a term holds the preferred, its
  `meaning` says so: "every class", as M5-22's entries already say, or "the
  preferred claim included" for `return_on_equity`.
- **No existing name or meaning changes.** The six names in the table that
  are not yet computed are fixed here, so that five metric tasks do not each
  choose.

Read together, a pair's two entries cannot be taken for each other. One name
carries `common` or `per_share` and the other carries neither, and each
meaning names the claim its terms are on.

### Retention

**Retention is formed per share.** `retention_per_share` is one less
`dividends_declared_per_share` over `earnings_per_share_diluted`, over one
duration. It is the complement of `dividend_payout_per_share` at the same row,
in exact arithmetic.

- **The dividend:** `dividends_declared_per_share`, declared per common share
  for the period.
- **The income:** `earnings_per_share_diluted`, from continuing operations and
  after preferred dividends by construction.
- **The sources:**
  - Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
    *International Financial Statement Analysis*, 3rd ed., CFA Institute
    Investment Series, Wiley, 2015, the chapter "Financial Analysis
    Techniques". It defines the retention rate as one less the payout ratio,
    with common share dividends over net income attributable to common shares.
  - Pinto, J. E., Henry, E., Robinson, T. R., and Stowe, J. D., *Equity Asset
    Valuation*, 3rd ed., CFA Institute Investment Series, Wiley, 2015, the
    chapter "Discounted Dividend Valuation". It states the same ratio per
    share, as dividends per share over earnings per share.

**Why per share.** The source's ratio is a common dividend over income
available to common. v2 cannot form the dividend's total, so the per-share pair
is the only common pair v2 carries. It is also the pair M5-23 already reads.

It differs from the total ratio in two places, and both are named:

- **Dilution.** For a profit, diluted earnings per share are at most basic. So
  the payout reads no lower and retention no higher. This is the conservative
  direction.
- **Declared, not paid.** The dividend declared for the period is set against
  the period's earnings. That matches the source's ratio more closely than
  cash paid would.

**Inputs and conditions.**

- The inputs are read in this order: `dividends_declared_per_share`, then
  `earnings_per_share_diluted`. That is `dividend_payout_per_share`'s order.
- Its one `declined` condition is `zero_earnings_per_share_diluted`. That is
  the condition M5-23 already defines, shared and not defined twice.
- A loss gives a figure above one, as the payout gives a negative figure. No
  condition is added for a loss. A retention that declined where its own
  complement did not would make two entries disagree over one pair of inputs.

### Sustainable growth

**Sustainable growth is common.** `sustainable_growth_on_common_equity` is
`return_on_common_equity` times `retention_per_share`, at one duration.

- **The source:** the same two CFA texts state the sustainable growth rate as
  the retention rate times return on equity.
- **Which return on equity:** the common one. The retention rate is the
  fraction of the common holder's earnings kept. The product is a growth rate
  only when the return is on the same claim. The open-questions record also
  admits both preferred concepts for this metric, and the total return on
  equity is the figure that record found biased.
- **Which retention:** `retention_per_share`, the one retention M5 computes.

**It is built on two metrics.** It forms no term itself. Its reasons are
`return_on_common_equity`'s, then `retention_per_share`'s, in that order. Its
`declined` conditions are theirs, and it adds none.

### Where a flow meets a balance

Return on equity, in both forms, and sustainable growth pair a flow with a
balance. Which instant the balance is read at, and so which rows the metric
reads, is the rule `which-other-period-a-metric-reads.md` fixes. This record
names that rule and decides nothing of it.

**That record is Proposed, not Accepted, on main as this is written.** So this
record leaves the pairing to it by name, or to whatever record supersedes it.
No metric here that reads a balance may be computed before an accepted record
fixes the pairing. `retention_per_share` reads none.

Sustainable growth reads `return_on_common_equity` as that metric is computed.
It adds no reading of its own.

### How absence composes

**A filer silent on `preferred_equity` or `preferred_dividends`.**

- v2 reads that silence as `Unknown`.
- The common form is absent with `input_unknown`, naming the concept and the
  period it was read at, through the accepted constructor.
- The total form stands beside it and is computed as before. So
  `return_on_equity` has a figure while `return_on_common_equity` is absent.
- `price_to_common_book` and `sustainable_growth_on_common_equity` have no
  total form, so they are simply absent.
- The per-share forms and `retention_per_share` read neither concept and are
  unaffected.

**Another reading of that silence is not this record's.** A zero, or a zero
conditioned on something, would change v2's `silence = "unknown"` for either
concept. Those are frozen bytes, so it would mean a v3 and an `analyze-store`
v2. Such a reading belongs in a record of its own.

The path that exists today needs no record. A registry override that asserts a
zero for a filer gives a `Value` with a rule behind it, as the open-questions
record says.

**A preferred concept read as a `Value` of zero.**

- The common term then equals the total term.
- `return_on_common_equity` has the same figure as `return_on_equity`, in the
  same characters, since the arithmetic is exact and one writer writes both.

**The conditions each common form adds,** each named once:

- **`zero_common_equity`.** The common equity a metric divides by is zero. It
  is defined beside the common equity term and used by
  `return_on_common_equity` and `price_to_common_book`.
  `sustainable_growth_on_common_equity` meets it through the first.
- **Whether `price_to_common_book` declines at a negative common equity** is
  its own task's. The accepted record leaves P/E on non-positive earnings to
  P/E the same way.
- **No metric here divides by income available to common.** The forms that
  would are the common total payout and coverage, which are open. So
  `zero_income_available_to_common` is not defined now. The first metric that
  divides by that term defines it, once, beside the term.
- **`retention_per_share`** adds none, and shares M5-23's.

**No term contributes nothing.**

- Neither preferred concept can be `NotApplicable` under v2, so the accepted
  exception has nothing to apply to.
- An `Unknown` is never read as zero.
- A `preferred_dividends` is never inferred from a `preferred_equity` of zero.
- The `short_term_investments` exception is not extended.

**Reason order.** A term's concepts follow the term's own order, and the terms
follow the formula, numerator first. `return_on_common_equity`'s entry lists
`net_income`, `preferred_dividends`, `shareholders_equity`, `preferred_equity`.
How a balance read at other rows expands into reads is the pairing record's.

No seventh reason is needed, and no frozen byte changes.

### The method version

These are part of the method `METHOD_VERSION` names, which is where the
accepted analyze record places a rule of composition:

- the three term definitions and the one module that forms them
- the common dividend total left unformed
- the naming rule and the six names
- retention's and sustainable growth's definitions
- `zero_common_equity`

A later change to any of them is a version bump and a superseding record, and
never an edit to this one.

It lands at version 1. No common form is computed yet, so no stored outcome
changes. Calling M5-23's per-share forms the common forms changes no formula,
name, input or figure.

### What a metric's catalogue entry says

Nothing more than the shape M5-18 fixed. A metric's form is stated by its name
and its `meaning`, under the rule above. A `form` field would be a second copy
of what the derivation reads, unless the derivation were generated from it.
Adding one changes M5-18's shape and is a record of its own.

### The proof the implementing tasks bring

These tests are built through `History::of` and `Row::at` and read no file.
Each stands in `tests/` or under `#[cfg(test)]`. Where a balance is read, the
history holds whatever instants the accepted pairing rule reads, each with the
same balances, so that any reading gives the figures below. At the least:

- **The preferred concepts as `Value`s.** Net income 120, preferred dividends
  20, shareholders' equity 1000 and preferred equity 200 give
  `return_on_common_equity` `0.125`, and `return_on_equity` `0.12` at the same
  duration.
- **A negative deduction.** Preferred dividends of −20 give income available
  to common of 140 and `return_on_common_equity` `0.175`.
- **Each preferred concept as a `Value` of zero.** `return_on_common_equity`
  is written in the same characters as `return_on_equity`.
- **A preferred concept `Unknown`.**
  - `preferred_equity` `Unknown` gives `return_on_common_equity` absent with
    `input_unknown` naming `preferred_equity` at the instant it was read at.
    `return_on_equity` is a `Value` at the same duration.
  - `preferred_dividends` `Unknown` gives `input_unknown` naming it at the
    duration.
  - Both `Unknown` give two reasons, `preferred_dividends` first.
- **`zero_common_equity`.** Shareholders' equity equal to preferred equity
  gives `return_on_common_equity` `declined` with `zero_common_equity`. The
  task that builds `price_to_common_book` brings the same test for it.
- **Retention.**
  - Dividends declared per share of 0.5 and diluted earnings per share of 2
    give `retention_per_share` `0.75`, and `dividend_payout_per_share` `0.25`
    at the same row.
  - Earnings per share of zero give `zero_earnings_per_share_diluted`.
- **Sustainable growth.**
  - The first row above with the retention row's per-share figures gives
    `0.09375`.
  - `preferred_equity` `Unknown` and `earnings_per_share_diluted` `Unknown`
    give two reasons in that order.
- **The catalogue.**
  - Every entry whose `inputs` hold `preferred_equity` or
    `preferred_dividends` has `common` in its name, and no other entry does.
  - No entry's `inputs` hold both `dividends_paid` and `preferred_dividends`.
    That keeps the open term open.

### What no check can see

- **Whether the sources say what this record says.** That covers Penman, the
  two CFA texts, SFAS No. 128 ¶9 and EITF D-42. Each metric's documentation
  cites its page as its task reads it. A source that on reading says otherwise
  is a stop for that task, not a choice.
- **A metric that forms a term itself,** such as `shareholders_equity` less
  `preferred_equity` written inline. The catalogue tests catch the open term's
  shape and nothing else. Only review sees the rest.
- **Redeemable preferred outside permanent equity.**
  - v2's `preferred_equity` includes it.
  - The registry resolves `shareholders_equity` from `StockholdersEquity`,
    which excludes temporary equity.
  - The registry maps `preferred_equity` to permanent-equity elements only, and
    its comment says such a filer "needs an override".
  - An override that brings mezzanine preferred into `preferred_equity`
    without bringing it into `shareholders_equity` subtracts it from a figure
    that never held it. Common equity is then understated by it: return on
    common equity is overstated, which flatters, and price/book is overstated.
  - Nothing compares the two. It is the registry's to get right for each such
    filer, and it is named here so the first override does not find it by
    accident.
- **A stand-in `preferred_dividends`.** `DividendsPreferredStock` and
  `PreferredStockDividendsIncomeStatementImpact` leave out accretion and
  redemption adjustments. Income available to common is overstated by them.
  The `Value` records its `stand_in` reading, and no metric reads it.
- **How far per-share retention is from the total ratio.** It differs by
  dilution and by share movements, and nothing measures how much.
- **A quarter beside a year,** carried over from the pairing record.

### What stands

Nothing proved gets narrower:

- No entry leaves `denied_packages`, `allowed_edges`, `ambient_names` or the
  expected gate set.
- No place, form or allowlist entry of `bare-literal-lint.md` moves.
  `retention_per_share` needs only one, which is on the allowlist. Every other
  number in the forms comes from the history.
- `vfi-analyze` takes no dependency.

These stay exactly as accepted:

- the analyze record's outcome, six closed reasons, three places and catalogue
- the arithmetic record's reading, operations and written form
- the open-questions record's readings of the preferred claims and of
  `dividends_paid`
- the frozen bytes of `canonical-concepts` v2 and `analyze-store` v1
- the entry shape M5-18 fixed

Two questions here would need one of those to change, and each is left open
and named: the common dividend total, which needs a v3 concept, and any
reading of preferred silence other than `Unknown`, which needs v2's bytes to
change. Neither is decided here.

**Why Structural.** The record fixes the meanings and names of metrics that
cross `analyze-store` v1, which store and M6 will read. It also fixes a rule
every common-shareholder metric shares. It needs nothing Constitutional: no
anchor, no gate, no protected path and no GOALS.md scope moves.

## Alternatives

- **The common dividend as `dividends_paid` less `preferred_dividends`.** It
  is the only total v2 seems to offer, and it would give common total forms of
  payout, coverage and retention now. Rejected: it deducts the preferred twice
  wherever `DividendsCommonStockCash` stands in, and it subtracts accretion
  and redemption adjustments from cash that never held them. That is the wrong
  number that looks right, in the flattering direction.
- **A `canonical-concepts` v3 concept carrying the common dividend as a
  total**, such as the equity statement's declared common cash dividend.
  Not rejected, but not here. It is a contract change above a worker, it
  forces an `analyze-store` v2, and it moves normalize and the registry. Once
  the per-share forms stand as the common forms, no M5 criterion needs it. This
  record names the gap so that record has its premise.
- **The per-share forms as the only common forms of payout and coverage,
  ruling out a common total form for good.** It is close to what this record
  does for M5. Rejected as a permanent rule: the per-share form departs from
  the total ratio by dilution and share movements. A v3 total would remove
  that gap, so it is left open rather than closed.
- **The total forms alone for M5, with the preferred concepts consumed by
  return on equity and price/book only.** It is simpler and needs no retention
  choice. Rejected: GOALS.md names retention and sustainable growth, and the
  open-questions record admits both preferred concepts for them. Sustainable
  growth over total return on equity carries the bias that record found, which
  flips sign at the filer in distress. A common retention is formable per
  share, so nothing forces the fallback.
- **The common dividend as `dividends_declared_per_share` times a share
  count.** Rejected: a count at one instant, or a diluted average, is not the
  shares each dividend was paid on. The product is wrong by every buyback,
  issue and dilutive security.
- **Preferred silence read as zero inside analyze.** It would give
  `return_on_common_equity` for most filers. Rejected: it substitutes for an
  `Unknown`, which the accepted record forbids, and it contradicts v2's
  silence reading. It is left to a record of its own.
- **`preferred_dividends` taken as zero where `preferred_equity` is a `Value`
  of zero.** Rejected:
  - It treats a term as contributing nothing, which the accepted record allows
    only for `short_term_investments`.
  - A zero carrying amount at an instant does not rule out a dividend in the
    period. Preferred redeemed within the period, or carried in mezzanine that
    the registry does not map, both pay one.
- **Return on equity in the common form alone.** Rejected: the total form is
  coherent, it is named in the pairing record's table, and it is the only
  return on equity a filer silent on preferred gets.
- **One name per metric, with a setting choosing its form.** Rejected:
  - A setting is what the cited source leaves open, and each source fixes its
    claim.
  - One name would carry two meanings across stored results, which ranking
    would mix.
- **A `form` field in the catalogue entry.** Rejected here: it changes the
  shape M5-18 fixed, and it is a second copy unless the derivation is
  generated from it.
- **Each metric forming its own terms.** It costs no shared module. Rejected:
  five readings of one rule is the duplication anchor 5 and the
  one-source-of-truth invariant forbid.

## Consequences

**Easier:**

- Every common-shareholder metric reads its terms from one module, under one
  naming rule, and none of them chooses.
- Payout and coverage have a common form today, the one M5-23 already
  computes.
- Retention and sustainable growth have definitions, sources and names. Each
  task that builds one has nothing left to decide but its code.
- The common dividend gap is named in one place. A v3 record can start from it.

**Harder:**

- **Most filers get no common form of return on equity, price/book or
  sustainable growth.** A filer with no preferred stock seldom tags a zero
  preferred dividend, so `preferred_dividends` is `Unknown`. Of the eleven
  rows the fixtures' claims name it at, it resolves at one. Unless a registry override asserts the zero,
  these three metrics are absent for nearly every filer. `return_on_equity`
  stands beside the first. Price/book and sustainable growth have no stand-in.
  This is an absence, never a wrong number, and M6's composites must cope with
  it.
- Payout and coverage are common only per share. A filer whose diluted
  earnings per share are `Unknown` has no common payout, even where its totals
  resolve.
- Two forms of one metric are two stored names, and a ranking must pick one.
- No common metric that reads a balance can be computed until the pairing
  record is accepted.

**Expensive to reverse:**

- The names, once results are stored under them.
- The per-share forms as the common forms, once rankings are built on them.

**Open, and not this record's:**

- The common dividend total, and a v3 concept for it.
- Any reading of preferred silence other than `Unknown`.
- Mezzanine preferred: what an override must do to `shareholders_equity` when
  it brings redeemable preferred into `preferred_equity`.
- Whether enterprise value counts the preferred claim.
- Which instant a balance is read at, which is the pairing record's.

## Enforcement

This applies anchor 5 and the one-source-of-truth invariant, and edits
neither.

- **One definition per term.** The catalogue tests above hold the naming rule
  and keep the open term open.
  - **Gap:** nothing forces a metric through the module rather than around
    it.
- **Anchor 5.** `literals` keeps every number but one out of the forms. No
  constant is added.
  - **Gap:** nothing checks that a citation is true.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: it fixes the names and
  meanings of six metrics that will cross `analyze-store` v1 as characters,
  and a rule every common-shareholder metric shares, both inside analyze. It
  edits no anchor, no protected path, no gate, no contract byte and no
  milestone scope, and asks for no allowlist entry. Flagged for later human
  review, as the tier requires.
- **Checked:**
  - `canonical-concepts` v2: `preferred_dividends` is defined as the
    deduction between the period's profit and the profit available to common,
    with accretion and redemption adjustments; `preferred_equity` includes
    redeemable preferred outside permanent equity; `dividends_paid` is every
    class; both preferred concepts apply to every kind with
    `silence = "unknown"`. The three terms are read off those bytes and
    change none.
  - `declared-cash-dividends-stand-in.md` (Accepted): `DividendsCommonStockCash`
    is admitted as a stand-in for `dividends_paid`. That is what makes the
    double deduction in the first alternative real and not hypothetical.
  - `canonical-concepts-open-questions.md` (Accepted): M5's metrics are
    common-shareholder measures, and common equity is `shareholders_equity`
    less `preferred_equity`. This record carries that and restates nothing.
  - `which-other-period-a-metric-reads.md` (Accepted this day, one PR
    earlier): its table names return on equity and on common equity as
    averaged, per CFA. This record leaves the pairing to it by name and reads
    no date itself.
  - `what-analyze-takes-and-returns.md`: six reasons, one `NotApplicable`
    exception for `short_term_investments`. `zero_common_equity` is a
    `declined` condition and not a seventh reason; preferred silence is
    `input_unknown` and never read as zero.
  - `bare-literal-lint.md`: the allowlist holds one, which is the only number
    `retention_per_share` needs.
  - `dividend_payout_per_share.rs`: `zero_earnings_per_share_diluted` exists
    and is shared, not redefined.
  - The worked figures under the proof: 100/800, 140/800, 1 − 0.5/2 and
    0.125 × 0.75 give what the record says.
  - The alternatives as argued, including the four the task required.
- **Verdict and why:** accepted. The record's weight is in what it refuses:
  it finds that v2 cannot form the common dividend total without a number that
  flatters, says so by name, and leaves the term open rather than closing it
  with the subtraction that looks obvious. Taking the per-share forms as the
  common forms of payout and coverage costs nothing today, since M5-23 already
  computes them, and it leaves room for a v3 total to stand beside them without
  renaming anything. The naming rule is checkable by a catalogue test, and the
  `Unknown`-beside-total reading of preferred silence makes the common forms
  absent for most filers, which is the safe side the accepted records already
  chose. The cost it names honestly: most filers get no common return on
  equity, price/book or sustainable growth until a registry override or a v3
  record changes that, and M6 must cope with that absence.
- **What would have changed it:** a seventh reason, a changed byte of
  `canonical-concepts` v2 or `analyze-store` v1, a reading of preferred silence
  as zero inside analyze, or a common dividend formed from `dividends_paid`
  less `preferred_dividends`. So would a source that, on reading, fixes
  retention or sustainable growth otherwise than the record says; the record
  makes that a stop for the implementing task, and that stop and the mezzanine
  preferred override under "what no check can see" are the items flagged for
  the human alongside the Structural tier.
