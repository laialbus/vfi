# Analyze is handed one filer's history, its price answers and its settings, and returns per period a value or an absence naming its input, under a method version and settings it records

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-10-02, by `M5-06`
- **Decided:** —
- **Touches:** two new contracts, `fetch-analyze` and `analyze-store`. Two new
  entries in `allowed_edges`, `vfi-analyze>vfi-contracts` and
  `vfi-store>vfi-contracts`, both onto the leaf crate and neither between
  stages. Normalize's `History` moves into `vfi-contracts`, and
  `canonical-concepts` v2's bytes do not change. `price-provider.md`'s
  interface stands as accepted: what a caller asks and what it gets back are
  unchanged, and the filer, the ticker and which answer is the valuation
  date's are attached outside the provider, by whoever asks it. No accepted
  record is superseded. The record applies anchors 2 to 5 and the
  per-user-state invariant, and edits none of them. It sets the place the
  anchor 5 literal lint will allow. No schema moves, no gate is weakened, and
  nothing here is implemented.

## Context

The last three criteria of GOALS.md's M5 are these. Every free parameter
arrives as a setting, and every default states its reason. Each result
records the settings and method version it was computed from. A metric with
no price is absent with a reason, and never a wrong number. Anchor 5 adds
that nothing arbitrary enters the derivation. `canonical-concepts` v2's
`[elsewhere]` leaves two things here by name. "How an absent input makes a
metric absent" is assigned to analyze, and "thresholds and defaults" to the
settings layer. Every metric task is a function whose signature is this
record, and so is the literal lint.

Some things are already fixed:

- Analyze is pure (anchor 4).
- `vfi-analyze` depends on nothing today.
- `allowed_edges` gives it no edge to `vfi-contracts`.
- Normalize's `History` and `Row` are normalize's own
  (`crates/normalize/src/history.rs`). Only the per-concept types they hold,
  such as `Resolution`, are in `vfi-contracts`.
- `docs/adr/price-provider.md` (M5-05, Accepted) keeps the price's type and
  retrieval. It moves the types into `vfi-contracts` if this record publishes
  them as a contract. Its Decision review hands this record one duty: the
  split list covers only the range asked, an empty list reads as no split,
  and the record that carries a price to analyze must close that.
- v2's `Value` names the filing that set it by accession and carries no filed
  date, so a per-share figure's share basis is not in the history.
- `docs/adr/canonical-concepts-open-questions.md` hands M5 one composition
  rule. A `NotApplicable` term of a sum contributes nothing, while an
  `Unknown` one makes the sum absent.

## Decision

### The call

Analyze has one entry point, a function in `vfi-analyze`. It takes three
inputs, all explicit, on every call:

- one filer's history
- that filer's price answers, or none
- the settings

It returns that filer's analysis. It reads nothing else, keeps nothing
between calls, and has no `static`, cache or thread-local that outlives a
call. The same three inputs give the same output.

It has one error. If the price answers name a filer other than the
history's, the call is broken. That is a stop, the way normalize's `Undated`
is, and not a state. Nothing about the data is an error. Which crate calls
analyze is not decided here (see Consequences).

### What analyze is handed from normalize

The input is one filer's history as `canonical-concepts` v2's `[boundary]`
line already publishes it. That is the CIK, then the periods Rule 1 admits as
a set, then at each period every concept once in one of the three states.

The type that holds it, today `History` and `Row` in
`crates/normalize/src/history.rs`, moves to
`vfi-contracts::canonical_concepts` unchanged in shape. Normalize builds it
and analyze compiles against it. Like every type in that crate, it carries
the comparison test against `v2.toml`'s carries line. v2's bytes stay as they
are, because the line already states this, so no v3 is published.

The kind does not cross. At v2 only a `NotApplicable` carries one. A metric
that needs the filer's kind brings the v3 record that adds it.

### Prices: the `fetch-analyze` contract

Prices are produced by fetch's provider and consumed by analyze, so they
cross a stage boundary. They are published at `contracts/fetch-analyze/`,
holding `v1.toml` and a `versions` file with its one `v1 <sha256>` line. The
module is `vfi-contracts::fetch_analyze`. It is named for the edge, as
`fetch-normalize` was, because one edge reads it.

**The provider is untouched.** It is asked what `price-provider.md` says a
caller asks, and it returns what that record says it returns. Its answer
types move into `vfi-contracts` unchanged and the provider returns them, as
that record anticipates. Whoever asks the provider assembles the crossing
from its own ask and the provider's answer.

v1 states this, for one ask:

- **The filer**, by its ten-digit CIK. The history is keyed by CIK and a
  ticker can be reassigned, so the crossing names the filer and not only the
  ticker.
- **The ticker** the provider was asked for.
- **The answer at the valuation date.** The provider's answer, price or
  absence, for the date the ask named as the valuation date. The valuation
  date is that answer's date asked. It is stated nowhere else, so it cannot
  disagree with the answer.
- **The answers at the earlier dates.** The provider's answer for each
  earlier date asked, as a set whose order means nothing.
- **The split list**, or its absence and reason, as the provider answered.

**The covered range** is defined by the surface, not written into it. It
runs from the earliest trading date any price in the crossing carries,
through the latest. Every such date is a row the request returned, so it lies
inside the range the request covered. `price-provider.md` makes the split
list complete inside that range. So any split dated inside the covered range
is in the list. A date asked whose answer is an absence proves nothing about
the range, so it does not widen it. The range holds even when the provider
shortened a request to fit its bandwidth budget. A split is a row on a
trading day, so no split falls between the latest price and the valuation
date when the latest price is the valuation date's. There is no covered
range when the crossing carries no price, and then no metric reads one.

**The pairing is the caller's claim.** The CIK records which filer the
caller asked on behalf of. Nothing in the crossing shows that the closes are
that filer's. Analyze checks only that the claimed CIK equals the history's.
A ticker reassigned inside the covered range would put another company's
closes beside this filer's figures. That is left visibly open (see
Enforcement).

Analyze is handed either one such crossing or none. None means no price was
asked. Screening asks none (`price-provider.md`).

**Why anchor 3 admits a boundary that skips normalize.** Anchor 3 makes every
boundary between stages a typed, versioned contract, and lets a stage know
the contract on each side of it and nothing else about its neighbours. A
price is fetch's output and analyze's input, so the boundary between them is
real, and anchor 3 asks only that it be typed and versioned. That is what
this contract is. What the `allowed_edges` comment leaves out on purpose is a
stage-skipping *edge*, `vfi-fetch>vfi-analyze` or its reverse. No such edge
is added. Fetch and analyze each reach the contract's type in
`vfi-contracts`, exactly as fetch and normalize reach `fetch_normalize`, and
neither learns anything about the other. Carrying the price through
normalize instead would hand normalize something two stages away, which is
what anchor 3 denies it (see Alternatives).

`v2.toml` says, in a comment, that analyze "reads this contract and no
other". The comment explains why v2 restates the period shapes rather than
reading `fetch-normalize`, and that reason still holds: analyze reads no
`fetch-normalize`. After this record the sentence is wider than true. It is
a comment in frozen bytes and changes nothing a gate or a type reads, so no
v3 is published for it.

### Settings

**A setting** is a free parameter of the derivation. It is a value a metric
needs that neither the filings nor the metric's cited source fixes, so two
careful users could choose differently and the method would still mean what
its source says. The discount rate, terminal growth and forecast horizon are
examples. Which settings exist is for each metric's task to say.

A threshold the user applies to stored metrics when screening or ranking is
not a setting. It is a ranking criterion, which is M6's, because it changes
no number analyze computes.

Each setting has these, defined once:

- a name and a meaning
- a unit
- the domain of values it admits
- its preset value
- the preset's stated reason, with a source wherever one exists

**Where.** All of that is defined in `crates/analyze/src/settings/`, one file
per method that has settings. Each is a field of one `Settings` type, built
only through a constructor that refuses a value outside its domain and never
clamps one. A refused value is the caller's error, raised before analyze
runs.

The preset values are reached through one named constructor in that
directory. The caller calls it, and the derivation never does. Analyze never
invents a value for a setting: it reads only the `Settings` it was handed.

**One source.** The shell shows each setting's name, meaning, domain, preset
and reason. It reads them from a catalogue the engine generates from this one
definition, and never from a copy in Python. A result records each setting by
name, generated from the same type.

**No per-user state.** The settings are an argument on every call. Nothing in
the engine holds "the current settings". Where a user's chosen settings are
kept between sessions is not decided here. Whatever keeps them hands them in
explicitly.

### Methodology constants

A methodology constant is a value the metric's cited source fixes, such as
Piotroski's thresholds, Altman's coefficients and cut-offs, or Graham's
limits. It is not the user's to change. A different threshold is either a
setting the source left open or a different metric.

Each constant is a named `const` in `crates/analyze/src/constants/`, one file
per method. Its documentation cites the source by author, work and year, and
by page or table where the source gives one.

The constant files and the settings files are one per method, so parallel
metric tasks own different files. Only each directory's `mod.rs` is shared,
and the planner orders the tasks that touch it.

### Where a literal may stand

The derivation is every other source file in `crates/analyze/src/`: the
metrics, their composition, and absence. The anchor 5 literal lint allows a
bare numeric literal in exactly three places, beyond the small allowlist the
lint's own task fixes:

- `crates/analyze/src/constants/`
- `crates/analyze/src/settings/`
- the one `METHOD_VERSION` line in `crates/analyze/src/method.rs`

Whether tests are read is the lint task's decision.

### The method version

The method version names how analyze derives each metric from its inputs.
It is a positive integer, starting at 1, defined once as `METHOD_VERSION` in
`crates/analyze/src/method.rs`.

**It changes** when an outcome that a stored result could already hold would
come out differently from the same history, prices and settings. That covers
a change to any of these:

- a metric's formula
- a constant's value
- which periods or dates a metric reads
- a rule of absence or composition, the share-basis rule below included
- the arithmetic, or how a figure is written
- a setting's name, meaning or domain
- a metric's removal or rename

**It does not change** for these:

- **Adding a metric.** No held outcome changes, and a result that predates
  the metric simply lacks it. This is also what lets M5's metric tasks run in
  parallel without colliding on one line.
- **Changing a preset or its reason.** A result records the value it used, so
  a replay does not consult the preset.
- **A refactor with identical outcomes.**

### What a result is

A result is per filer and per canonical period, in v2's two shapes. The
period is the one the metric is stated at. For a metric read over several
periods, it is the last of them by that metric's own rule. Which periods a
metric reads is that metric's task.

For each metric the method version computes at that period, the result holds
exactly one **outcome**:

- **`Value`.** The figure, as the characters analyze writes. The unit is
  stated once, in the metric's catalogue entry beside its name and meaning.
- **`Absent`.** A non-empty list of reasons, one per absent input, in the
  metric's own input order, so every missing input shows and not only the
  first.

**The reasons are a closed set.** Each can be constructed only from its own
witness, as v2's two absences can:

- **`input_not_applicable`.** Names the concept and the period. It is built
  only from that concept's `NotApplicable` in the history.
- **`input_unknown`.** Names the concept and the period. It is built only
  from that concept's `Unknown` in the history.
- **`no_period`.** The history holds no row for a period the metric reads. It
  names the period the metric was computing at and the name of the rule that
  sought the other period.
- **`no_price`.** The metric reads a price at a date and has none. It
  carries exactly one of three, and between them they cover every way to
  have no price:
  - `not_asked`. Built from the crossing's absence: no price was asked.
  - `date_not_asked`. Built from a crossing whose answers hold none for the
    date the metric reads. It names that date. This is relative yield at a
    past period end when the caller asked the valuation date alone.
  - `answered_absent`. Built from the provider's absence for that date. It
    carries the date asked and the `price-provider.md` reason.

  A crossing holds exactly one answer per date asked, a price or an absence,
  so a metric that reads a date and finds no price meets exactly one of the
  three.
- **`basis_unproven`.** A price was answered, but the figure it is put beside
  cannot be shown to be on the same share basis (below). It carries exactly
  one of three:
  - `splits_absent`. Built from the split list's absence. It carries the
    `price-provider.md` reason.
  - `not_covered`. Built from a covered range that does not contain both
    dates. It names the covered range, the figure's period end and the
    price's trading date.
  - `split_within`. Built from a split in the list dated inside the span the
    rule reads. It names the split's date and factor as published, the
    figure's period end and the price's trading date.
- **`declined`.** Every input was present, but the metric's definition
  states it is undefined at those inputs. It names that condition as the
  metric's catalogue entry names it. Examples are a zero divisor and P/E on
  non-positive earnings. Each metric's task lists its own conditions.

A reason names the input's state and does not copy what that state carries.
The state's carriage is in the history and the crossing, which go to store
beside the results (below), so each stays in one copy.

**How absence composes:**

- An input that is `Unknown`, or a period the history lacks, always makes
  the metric absent. Nothing is substituted for it.
- An input that is `NotApplicable` makes the metric absent too. The one
  exception is where the metric's own definition states that the term
  contributes nothing. That is the open-questions record's rule for
  `short_term_investments` inside net debt, and only a `NotApplicable` can be
  treated that way.
- A metric built on another metric carries the other metric's reasons, not
  "metric X was absent". So every reason traces to an input or to a declined
  condition.

### Price-dependent metrics

A price-dependent metric takes the price as an optional input. It reads the
valuation date's answer from the crossing's answer at the valuation date,
and a past period end's from the earlier answer whose date asked equals that
period end character for character. Without a price there, it is `Absent`
with `no_price`. It does not fail the run. It does not fall back to another
date's price or to an adjusted close, and it is never computed from a
stand-in.

**The share-basis rule.** A price can be put beside a figure whose unit is
`currency_per_share` or `shares`, or beside anything derived from one, only
when all three hold. Let E be the figure's period end, an instant's date or
a duration's end, and let T be the close's trading date.

1. The split list is present.
2. The covered range starts on or before the earlier of E and T, and ends on
   or after the later.
3. The list holds no split dated on or after the earlier of E and T.

If any fails, the metric is `Absent` with `basis_unproven`. Market
capitalisation, and everything built on it, falls under the rule through
`shares_outstanding`.

Why a split anywhere after the earlier date declines, and not only one
between E and T: the figure is on the share basis of the filing that set it.
That filing was filed on or after E, possibly years later, and v2 does not
say when. A split after T that a later filing restated the figure for is a
basis change exactly as one between the two dates is. Without the filed
date, no metric can place the figure's basis relative to a split, so every
metric declines on one. `price-provider.md` leaves what a metric does about
a split between its dates to that metric's task. Under v2, this is the only
answer any metric can give. A metric that wants to compute across a split
brings the record that hands analyze the basis date. The bound is "on or
after" rather than "after" because whether a split's date is the first day
on the new basis or the last on the old has not been read off a real
response, and `price-provider.md` leaves reading the factor to its
implementing task.

**What the caller must ask.** A per-share metric at the valuation date pairs
a figure at an earlier period end with the valuation date's close. Its
covered range reaches that period end only if a price at or before it was
answered. Asking every period end the history holds does that, costs no
request (`price-provider.md`), and is what relative yield asks already.
Analyze knows what was asked from the crossing's dates asked, and a caller
that asked less gets `date_not_asked` or `not_covered`, never a number.

### What crosses to store: the `analyze-store` contract

Analyze returns its output to its caller and calls nothing. What the caller
hands store is published at `contracts/analyze-store/`, holding `v1.toml` and
a `versions` file. The module is `vfi-contracts::analyze_store`.

v1 states this, per filer by CIK:

- **The history analyze read,** as `canonical-concepts` v2 publishes it. This
  is the data premise. It is also the only path by which v2's "an absence
  keeps its state … into storage" can hold, since `allowed_edges` gives
  normalize no edge to store.
- **The price crossing it read,** as `fetch-analyze` v1 publishes it, or
  `not_asked`.
- **The premises:** the method version, and every setting by name with the
  value used, preset or chosen alike. Every result in the hand-over was
  computed under them.
- **Per period, the results.** Each metric is named once with its outcome,
  using the outcome and reason sets above, closed. Periods are restated in
  v2's two shapes, as v2 restated `fetch-normalize`'s.

The surface names the outcome's shape and not the list of metrics. Metric
names are the method's, defined beside each metric in analyze and exposed to
the shell through the same generated catalogue as the settings.

**Replaying** a stored result means running analyze over the stored history
and price crossing, under the stored settings, at the stored method version.

### Edges, and anchor 2

`vfi-contracts` depends on nothing, so no edge onto it can close a cycle or
order two stages. This record needs exactly these two edges:

- **`vfi-analyze>vfi-contracts`**, for the history, the price crossing and
  the result.
- **`vfi-store>vfi-contracts`**, for the result.

`vfi-fetch>vfi-contracts` and `vfi-normalize>vfi-contracts` already exist.
Analyze takes no edge to `vfi-normalize`. That edge would point backward and,
beside the allowed `vfi-normalize>vfi-analyze`, would form a cycle. Analyze
takes no edge to `vfi-fetch` either, which would point backward too. It also
uses no edge to `vfi-store`. The edge is allowed, but a write is an effect,
and whatever database store links would enter analyze's resolved tree and
turn `purity` red. Every remaining edge still points forward, so anchor 2's
direction holds.

### What this does not decide

- Any metric's formula, or any constant's or preset's value. Those are later
  tasks, each working under this record.
- The arithmetic analyze computes in, and how a figure is written. These go
  to the first metric task, under the method version.
- How the price is retrieved, and its fields. Those are `price-provider.md`'s.
- The storage schema and row keys, including whether the premises are part
  of a row's key. Those are M6's.
- How the shell reaches the two catalogues, which belongs to the shell
  boundary.

## Alternatives

- **Settings as a published contract, presets in frozen bytes.** This keeps
  them as data and digestible. Rejected for two reasons. Every metric task
  that adds a setting would become an exclusive contract change. A preset
  change would cost a version, even though no replay reads presets. The
  shell's need for one source is met by generating its catalogue from the one
  Rust definition.
- **Each metric as a typed field of `analyze-store`.** This would let store
  type-check metric names. Rejected: forty-odd metric tasks would become
  serialized contract changes, and a stored metric's meaning is the method
  version's anyway.
- **Analyze takes normalize's `History` directly.** That needs
  `vfi-analyze>vfi-normalize`, which points backward and closes a cycle.
- **Price through normalize,** as `fetch-normalize` v3 and
  `canonical-concepts` v3. Rejected: the vocabulary keeps price out by name
  (`canonical-concepts.md`). Normalize would carry what it does not
  normalize, and two frozen M4 surfaces would bump for a value neither stage
  reads.
- **The provider marks the valuation date and takes the CIK.** Rejected: it
  changes "what a caller asks" and "what it gets back" in an accepted record,
  to carry two things the caller already holds. Attaching them outside the
  provider leaves that record standing.
- **The valuation date as a setting.** Rejected: its only consumers read it
  beside a price, and a setting with no price would be a premise nothing
  reads. A preset for it would need a clock, which analyze cannot have.
- **The covered range written into the crossing by the caller.** Rejected:
  the caller does not know the lookback or a shortened range, so the figure
  would be a claim. Read off the prices answered, it is evidence.
- **The basis date through `canonical-concepts` v3, carrying each value's
  filed date.** This would let a metric compute across a split rather than
  decline. Rejected here, not ruled out: it bumps a frozen M4 surface and
  changes normalize for a precision gain no metric task has asked for yet.
  The share-basis rule declines where v3 would compute, and never gives a
  number v3 would contradict.
- **One `Absent` with a free-text reason.** Rejected: nothing would keep
  `NotApplicable`, `Unknown` and no-price apart. That is the confusion v2's
  witnesses exist to prevent.
- **A version per metric.** This is finer: one changed formula would not mark
  other stored metrics stale. Rejected: a change to shared machinery, such as
  market capitalisation or a composition rule, must bump every metric that
  depends on it, and a missed one is silent. A single version can only over-
  invalidate, and that is harmless.
- **The method version as a digest of analyze's source.** Rejected: analyze
  cannot read its own source under anchor 4 and M5-04. A comment edit would
  also mark every stored result stale.

## Consequences

**Easier:**

- Every metric task has a fixed signature, a fixed place for its constants
  and settings, and a closed set of ways to be absent.
- Every price-dependent metric reads the valuation date from one place and
  meets one share-basis rule, so parallel metric tasks do not each choose.
- Adding a metric touches no contract and needs no version bump.
- A stored result can be replayed from what store holds.
- The literal lint has three paths to allow.
- `purity` now reads `vfi-contracts`'s resolved tree as part of analyze's,
  which it did not before.

**Harder:**

- A split anywhere after a figure's period end makes every per-share price
  metric on that figure absent, even where the latest filing already
  restated it. For a filer that split recently, P/E, yield and market
  capitalisation are absent until a period ending after the split is the
  latest. This costs precision and never gives a wrong number.
- `analyze-store` repeats the history on every hand-over. Whether store keeps
  one copy is M6's.
- Store cannot type-check a metric name.
- The two `mod.rs` files order the metric tasks that add a method's first
  constant or setting.
- A `canonical-concepts` v3 forces an `analyze-store` v2, because the latter
  carries the former by version.

**Open, and not this record's:**

- **Which crate calls analyze.** Under `allowed_edges` only `vfi-normalize`
  may depend on `vfi-analyze`. A caller that holds settings and asks for
  prices is not normalize. Giving that caller a crate and its edges is a
  layout and gate decision of its own.
- **Whether `allowed_edges` keeps `vfi-analyze>vfi-store`.** This record uses
  that edge for nothing.

**Expensive to reverse:**

- The reason set, once results are stored with it.
- The rule that a stored result carries its history, because M6's schema
  will be built on it.

## Enforcement

This applies anchors 2 to 5 and the per-user-state invariant. Each item is
held by the check below, or left visibly open.

- **Anchor 2.** `deps` holds it once the two edges are added, in the task
  that first uses them.
- **Anchor 3.** `contracts` freezes both new directories, and each module's
  comparison test runs under `tests`.
- **Anchor 4.**
  - `purity` covers `vfi-contracts`'s dependencies once the edge exists.
  - **Gap:** M5-04's source check reads only `crates/analyze/`. A non-test
    function in `vfi-contracts` naming `std::fs` would reach analyze
    unseen. Today the crate does that only under `#[cfg(test)]`.
- **Anchor 5.**
  - **Gap:** the literal lint is not built, and this record only gives it its
    three places.
  - **Gap:** nothing yet stops the derivation from calling the preset
    constructor. The lint's task can ban that name outside `settings/`.
  - **Gap:** nothing checks that a citation is true.
- **The absences stay apart.** The types do this: each reason is constructible
  only from its witness. That holds once the types are written.
- **No wrong number across a split.**
  - The share-basis rule holds inside the covered range once it is written
    as the one function every price-dependent metric calls, with a fixture
    whose split falls after a figure's period end.
  - **Gap:** a split after the latest price, which a filing filed after the
    valuation date already restated a figure for. The rule cannot see it,
    and v2 carries no filed date to show such a filing exists. It cannot
    happen when the valuation date is on or after the day the history was
    fetched, but nothing checks that.
- **The prices are the filer's.**
  - **Gap:** the CIK is the caller's claim, and analyze checks only that it
    matches the history. Nothing shows the ticker named this filer across
    the covered range.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump. The check would freeze each analyze
    fixture's expected outcomes per method version, as `contracts` freezes a
    version. Building that is a new gate and a task of its own.
- **No per-user state.**
  - **Gap:** nothing reads analyze for a `static`, `thread_local!`,
    `OnceLock` or `LazyLock`. Repeatable fixtures show the symptom, not the
    absence.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
