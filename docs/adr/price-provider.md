# Prices come from Tiingo's end-of-day close, unadjusted, one request per company, behind a provider in the fetch stage that answers every date with a price or an absence and its reason

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-30, by `M5-05`
- **Decided:** 2026-09-30, by the decider
- **Touches:** a new interface, the price provider ANCHORS.md names, and its
  first implementation. It also touches the fetch stage's host list and
  chokepoint, which gain a second source. ANCHORS.md's three price and key
  invariants are applied here and none is edited. No contract is published, no
  schema moves, no edge enters `allowed_edges`, no gate is weakened, and
  nothing here is implemented.

## Context

GOALS.md's first two M5 criteria are that market prices arrive through the
price provider interface from one user-keyed source, and that a free-tier key's
published limits are enough for the work. Screening the corpus must need no
price call, and valuing a shortlist must fit in one day's allowance. ANCHORS.md
fixes three things. Prices sit behind a provider trait with one implementation
per source. The key is the user's and reaches the engine only as a parameter. A
missing price is a state, not an error. What is open is the source, where the
trait lives, what it returns, and which prices anything reads. A new interface
and a second source through the fetch chokepoint are both above a worker.

`docs/adr/canonical-concepts.md` keeps price and market capitalisation out of
the vocabulary and names their consumers. `docs/adr/which-filing-sets-the-value.md`
fixes that the filing filed last sets each value. That second fact decides the
adjustment question below.

## Decision

### The source, and what it publishes about being accessed

The first source is **Tiingo's end-of-day prices**, at `api.tiingo.com`, under
the user's own free key. The following was read on **2026-09-30**:

- <https://www.tiingo.com/about/pricing>. The free tier allows "Max Requests
  Per Hour" **50**, "Max Requests Per Day" **1000**, "Unique Symbols per
  Month" **500**, "Max Bandwidth Per Month" **1 GB** and "Price History"
  **30+ Years**. Its licence is "Internal Use Only", which the page defines as
  "you may only use the data for your own personal use and you may not display
  or share the data with another person or organization". The page states no
  per-minute limit, so none is assumed here. It also does not say whether "per
  day" and "per month" are calendar periods or rolling windows, so the counting
  below holds under either reading.
- <https://www.tiingo.com/documentation/end-of-day>. Historical prices come
  from `https://api.tiingo.com/tiingo/daily/<ticker>/prices?startDate=…&endDate=…`,
  one request for the whole range. Each row carries `date`, `close` ("The
  closing price for the asset on the given date"), `adjClose`, `divCash` and
  `splitFactor` ("The factor used to adjust prices when a company splits").
  `adjClose` "incorporates both split and dividend adjustments" under CRSP's
  method. Most US equity prices are available at 5:30 PM EST, with corrections
  until 8 PM EST.
- <https://www.tiingo.com/documentation/general/connecting>. The token goes
  either in the `Authorization` header as `"Token " + your API token`, or in
  the URL as `token=`.

None of the three pages says what the source answers once a limit is spent.
This record therefore relies on no particular answer (see below).

### Which prices M5 consumes

There are two kinds of date, and each takes one field: `close`, **unadjusted**.

- **At the valuation date.** The caller names this date explicitly, and the
  engine never takes it from a clock. The price is the close on the last
  trading day on or before that date. It is consumed by:
  - price/earnings, through `earnings_per_share_diluted`
  - market capitalisation, through `shares_outstanding`, and through it
    price/book, price/sales, price/free cash flow, enterprise value (so EV/EBIT
    and EV/EBITDA) and Altman's X₄, the market value of equity
  - yield, through `dividends_declared_per_share`
  - the margin of safety in the discounted cash flow and in the dividend
    discount valuation, each of which compares a per-share value with the
    price
  - the price tests in the established screens

  Profitability, returns on capital, liquidity, leverage, Piotroski's nine
  signals, payout, coverage, growth, streak and retention read no price.
- **At each past fiscal period end the history holds, for relative yield
  only.** The price is the close on the last trading day on or before each
  period end. This reads relative yield as a company's current yield against
  its own past yields (Weiss and Lowe, *Dividends Don't Lie*, 1988). Spare's
  relative dividend yield (*Relative Dividend Yield*, 1992) divides by the
  market's yield instead. That divisor is neither a filing concept nor a price
  of the filer, so this provider cannot supply it. A metric task that chooses
  Spare's reading brings its own record for that source. The request count
  below does not depend on this reading, because past dates cost no extra
  request.

No other price is returned. `divCash` goes unread, because yield reads the
dividend from the filing. Open, high, low and volume go unread too, because
no metric consumes them.

**Why unadjusted.** An adjusted close restates every past close onto today's
share basis. A per-share figure is on the basis of the filing that set it,
which is the last filing to quote its period. That filing is often years older
than the latest split. Dividing it by an adjusted close divides by a number on
another basis. The result is off by the split ratio and looks exactly like a
real multiple. The dividend adjustment is wrong for every metric here. It
lowers each past close by the dividends paid after it, so a yield on it is
overstated and a multiple understated. It is a construction for measuring
returns, not a price anyone paid.

The unadjusted close is right only when no split falls between a per-share
figure's basis and the price's date. So every answer also carries the split
factors the source publishes inside the range the request covered, each with
its date, as published. The consumer can then see a split rather than divide
across it. What a metric does when a split lies between its two dates belongs
to that metric's task.

### The interface

`PriceProvider` is a trait in `vfi-fetch`. It has one implementation per
source, and the first is Tiingo's.

- **What a caller asks.** A ticker, the valuation date, and zero or more
  earlier dates.
- **What it gets back.** One answer per date asked. Each answer is either:
  - a **price**: the date asked, the trading date the close is for, the
    unadjusted close as the characters the source published (never a binary
    float, as with EDGAR's amounts), and the `Source` of the request that
    produced it; or
  - an **absence**: the date asked and a reason.

  Beside those comes the list of split factors inside the covered range, or
  that list's own absence with the same reason.
- **The reasons are closed:**
  - no key was given
  - the key cannot be sent as a header value
  - an allowance is spent, naming which allowance
  - the source answered with a status other than success, carrying the status
  - the source could not be reached, carrying why
  - the answer could not be read, carrying why
  - the answer holds no close on or before the date asked

  None of these is an `Err`. The trait has no error type, so a caller cannot
  treat a missing price as a failure and stop the run.

Until M5-06 decides how a price reaches analyze, these types are defined in
`vfi-fetch`. If M5-06 publishes them as a contract, their definitions move to
`vfi-contracts` and the provider returns the contract's types. `company_facts`
already works this way, over the `fetch-normalize` boundary.

### How the key reaches the engine

The Tiingo implementation takes the key as a constructor parameter,
`Option<ApiKey>`. This follows the precedent of the EDGAR `Declaration`, which
names the user and is also passed in. The engine reads no keychain, file or
environment variable for a key. How the shell stores the key is the shell's
business and is not decided here.

- **With `None`,** every date asked is an absence reading "no key", nothing is
  sent, and nothing fails.
- **Header, not URL.** The key goes only in the `Authorization` header, never
  as `token=` in the URL. A `Source` is the URL, and a URL with the key in it
  would copy the key into every price's provenance and from there into
  storage.
- **Never printed.** `ApiKey`'s `Debug` and `Display` print no part of it.
- **Checked where it is given.** Like the `Declaration`, a key that is not a
  valid header value is refused when it is given, as an absence rather than a
  send. That means non-ASCII, or containing a control character.

### Placement, edges and egress

- **Where the code goes.** The trait and its types go in
  `crates/fetch/src/price/`, and Tiingo's request and parsing in
  `crates/fetch/src/price/tiingo.rs`. The sending itself stays in
  `crates/fetch/src/egress/`, the one directory where the `egress` gate allows
  a connection to be opened.
- **Edges and layout.** No workspace edge is added, so `allowed_edges` needs
  nothing. The crate needs no new dependency: it already holds TLS and JSON.
  There is no layout change, since `docs/layout.md` places crates and not
  modules.
- **Host list.** `crates/fetch/src/hosts.rs` gains `api.tiingo.com`, in the
  task that first fetches from it, as that file's own comment anticipates.
- **Egress per source.** The chokepoint becomes per source. Each host entry
  names its source, and each source has its own pace and its own headers. A
  request to `api.tiingo.com` takes Tiingo's pace and carries the key and
  nothing else. A request to EDGAR takes EDGAR's pace and carries the
  `Declaration`. Neither can leave under the other's rate. The user's e-mail
  address is never sent to Tiingo, and the key is never sent to the SEC.
- **Rate limit.** Tiingo's limits are named once, with the citation and read
  date above, where the crate keeps EDGAR's in `policy.rs`:
  - requests per hour, 50
  - requests per day, 1000
  - unique symbols per month, 500

  The spacing is derived from the hourly limit, not written down beside it:
  3,600 s ÷ 50 = **72 s** between requests. `MINIMUM_SPACING` is derived from
  EDGAR's rate the same way.
- **Counting the daily and monthly limits.** The provider counts them over
  trailing windows, which hold under either reading of the page:
  - **Daily.** At most 1000 requests in any 24 hours. Every calendar day, in
    any time zone, is a 24-hour window.
  - **Monthly.** At most 500 distinct tickers in any 31 days. Every calendar
    month, and every 30-day rolling window, fits inside one.

  The record of what was spent is an explicit input and output of the
  provider, not state the engine keeps between calls. Where it is kept between
  sessions is store's (M6), and is not decided here.

### The second criterion, by arithmetic

**Screening makes no price call.** Screening and ranking are queries over
stored results (ANCHORS.md, GOALS.md M6). Neither store nor normalize nor the
funnel holds a provider, so none of them can reach one. A provider is reached
only by an explicit ask that names a ticker.

**Valuing one company costs one request and one unique symbol.** The single
request covers the range from the earliest date asked (less a short lookback,
so that a period ending on a non-trading day still finds its close) up to the
valuation date. That range holds every price and every split factor above.

**What one day's allowance values:**

| Limit | Cap on companies | Reason |
| :--- | :--- | :--- |
| per day | 1000 | 1000 requests ÷ 1 request per company |
| per hour | 1000 in 20 h | 50 companies per hour; 24 × 50 = 1,200, so the daily cap binds first |
| per month | 500 distinct | 500 unique symbols; valuing the same company again costs requests but no new symbol |

So one day's allowance values a shortlist of up to **500 companies**, if none
was valued earlier in the month. That is half the day's requests, spread over
(500 − 1) × 72 s ≈ **10 h**. A shortlist of 50 fits within one hour. Each such
valuation runs as a job (`vfi-jobs`) so the interface stays responsive.

**Bandwidth** is a fourth limit. It is not in requests, and the pages do not
give a row's size. The budget is 1 GB ÷ 500 symbols ≈ 2 MB per company-month
if every symbol is used. A daily row is a few hundred bytes of JSON at about
252 trading days a year, so roughly two decades of history per company fit
inside it. This is an estimate, not a read limit. The implementing task
measures one real response and states the history a request may span inside
the budget. If a request would pass that span, the answer is shorter, never a
blown budget.

**Once an allowance is spent,** every further date asked is an absence naming
which allowance, with nothing sent. That covers the daily requests, and the
month's symbols for a ticker not yet counted in the window. The hourly limit
never produces an absence, because the pace waits instead. If the source
refuses anyway, the answer is an absence carrying the status the source sent.
That happens, for example, when a session starts without the record of an
earlier one. It is never an error.

How a price reaches analyze, and what a price-dependent metric does when the
price is absent, belong to M5-06's record and are not decided here.

## Alternatives

- **Alpha Vantage.** Read 2026-09-30 at <https://www.alphavantage.co/premium/>
  and <https://www.alphavantage.co/documentation/>. Rejected on three counts:
  - **Allowance.** The free key allows "25 API requests per day", so a
    shortlist is capped at 25 against 500 here.
  - **History.** `TIME_SERIES_DAILY` is free, but "The 'full' outputsize is
    available to premium keys", and `compact` "returns only the latest 100
    data points". Relative yield's past period ends are out of a free key's
    reach, and `TIME_SERIES_DAILY_ADJUSTED` is "a premium API function".
  - **Key placement.** The key travels as `apikey=` in the URL, so it would
    sit in every price's `Source`.
- **A crate of its own, `vfi-prices`.** Rejected. Retrieving from an outside
  source is what the fetch stage is. The `egress` gate allows a connection
  only in fetch's chokepoint, so a separate crate would need either a second
  chokepoint or an edge to `vfi-fetch`. That edge is a new entry in
  `allowed_edges` and a layout change, bought to put a fetch somewhere other
  than fetch.
- **The adjusted close.** Rejected for the reasons under "Why unadjusted". It
  would remove the split question only by hiding it, and every dividend
  adjustment is wrong for these metrics.
- **One request per date.** Rejected. A company with ten past period ends
  costs eleven requests instead of one. The shortlist falls from 500 a day to
  90 (1000 ÷ 11), and from 50 an hour to 4, for the same prices.

## Consequences

- **Easier:**
  - Every price-dependent metric has one price field, one basis and one
    provenance to cite.
  - A second source is a second implementation behind the same trait and a
    second host entry with its own pace, touching no caller.
  - A missing key, a spent allowance and a refused request are one shape, so
    M5-06 has one absence to carry.
- **Harder:**
  - Valuing a large shortlist takes hours at 72 s per company.
  - A split between a figure's basis and the price's date is a case every
    per-share metric must meet, not one the price hides.
- **Open, for the implementing task, to read and not assume:**
  - The currency of `close` is not stated on the page read. A filer reporting
    in a currency other than the price's is a wrong number of the same kind as
    an adjusted close. The implementing task reads where the source states the
    currency. Until it does, a price for such a filer is not assumed to match.
  - The page does not say which `splitFactor` value marks a day without a
    split. The implementing task reads it off a real response.
- **Licence.** "Internal Use Only" fits one user valuing with their own key.
  An engine later serving many users under one key would breach it. The
  bring-your-own-key decision already rules that case out, and a change to it
  would need a record of its own.
- **Expensive to reverse:** the unadjusted basis. Once results are stored with
  it, a change of basis is a re-derivation of every stored price-dependent
  metric.

## Enforcement

This applies anchor-derived invariants. What holds each, with every test
added by the implementing task:

- **Key in the header only, reaching the right host only.** The per-source
  chokepoint holds this, with the key header added only for Tiingo's hosts. A
  test proves it with a transport that records what it was handed.
- **Nothing fails on a missing price.** The trait's return type holds this,
  since it has no error type. A test with no key shows every date comes back
  absent and nothing is sent.
- **Screening makes no call.** A test runs the funnel over its fixture with a
  recording transport and finds no `api.tiingo.com` request.
- **Rate.** A test proves the spacing is the cited hourly limit, as
  `the_spacing_is_the_published_rate_said_the_other_way` already does for
  EDGAR's.
- **Gap: the engine reading the key for itself.** Nothing yet stops
  `vfi-fetch` from reading `std::env` or a file for a key. M5-04's check
  reads only `crates/analyze/`, and the `egress` gate reads only for
  connections. The constructor parameter is the only path this record
  designs, but it is not the only one the compiler allows. Extending a
  source-name check to the fetch crate is a new gate, above this record,
  so this gap is left visible rather than covered.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: a new interface and a new
  implementation inside one crate. No anchor is edited, no edge enters
  `allowed_edges`, no gate moves, and the host it adds is the one GOALS.md's
  M3 and M5 already name. Flagged for later human review.
- **Checked:**
  - The three price and key invariants in ANCHORS.md, and anchor 2: the
    provider sits in fetch and nothing later is called.
  - `filer-decision-ledger.md`, for what fetch may be handed, and
    `canonical-concepts.md`, whose price consumers the list here matches.
  - The five cited pages, fetched again on 2026-09-30. Every limit and every
    quoted line reads as the record gives it.
  - `hosts.rs`, `policy.rs`, the chokepoint and `Source`, which are as the
    record describes them.
  - The arithmetic, and the four alternatives as argued.
- **Verdict and why:** accepted. The basis argument is the one that matters: a
  filing's per-share figure is on its own filing's share basis, and only the
  unadjusted close can be put beside it without a hidden ratio. The rest
  follows the EDGAR precedents already on main. Three things are left for
  later records and none is settled here:
  - The split list covers only the range asked. A caller that asks the
    valuation date alone gets a list that is silent about a split between the
    figure's filing and that date, and an empty list reads as no split. The
    record that carries a price to analyze must close this.
  - The spent record has no home between sessions. Until it does, a restart
    forgets the counts and the pace, and the limits hold only by the source
    refusing. "Store's" is not accepted here as the answer; the ledger record
    rejected store implementing what fetch is handed.
  - The gap under Enforcement stands open, and no queued task holds it.
- **What would have changed it:** a per-minute limit on the pricing page, or a
  free tier that withheld `close` or `splitFactor`; either breaks the
  arithmetic or the basis. So would an interface that could not be asked from
  an earlier date, since the split gap above would then have no way to close.
