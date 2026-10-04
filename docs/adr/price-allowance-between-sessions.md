# The price provider's spent record is an append-only journal fetch keeps at a path it is handed, one entry per request, stamped with the wall-clock moment it left and read back for both trailing windows and the pace

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-10-03, by `M5-07`
- **Decided:** —
- **Touches:** a new interface, `SpentRecord`, and the storage schema of the
  entries behind it. That is one more file the engine writes in its local data
  directory. The record answers the question `price-provider.md` left open by
  name, where the spent record is kept between sessions, and supersedes nothing
  in it. The source, the `PriceProvider` interface and its closed reasons, the
  limits, the trailing windows and the unadjusted close all stand as accepted.
  The record applies anchor 2, the storage-behind-an-interface invariant and
  the per-user-state invariant, and edits none of them. No contract is
  published, no edge enters `allowed_edges`, no gate is weakened, and nothing
  here is implemented.

## Context

GOALS.md's second M5 criterion asks that valuing a shortlist fit inside the
free tier's allowance. `price-provider.md` (M5-05, Accepted) counts that
allowance in three parts:

- **Requests per hour, 50.** The pace holds this one, at 72 s between
  requests.
- **Requests per day, 1000.** Counted over any trailing 24 hours.
- **Unique symbols per month, 500.** Counted as distinct tickers over any
  trailing 31 days.

It makes the record of what was spent an explicit input and output of the
provider, not state the engine keeps between calls. Where that record is kept
between sessions it left to "store's (M6)". Its Decision review refused that
answer:

- The spent record has no home between sessions.
- So a restart forgets the counts and the pace, and the limits hold only by
  the source refusing.
- `filer-decision-ledger.md` rejected store implementing what fetch is handed.

Two facts are fixed:

- **Anchor 2.** `allowed_edges` holds `vfi-fetch>vfi-normalize`,
  `vfi-normalize>vfi-analyze`, `vfi-analyze>vfi-store`,
  `vfi-fetch>vfi-contracts` and `vfi-normalize>vfi-contracts`. Fetch cannot
  reach store, and store cannot implement a trait fetch defines.
- **The pace is monotonic.** `Pace` counts against a monotonic `Instant`. An
  `Instant` is meaningful only inside the process that read it, so a later
  session can do nothing with one.

## Decision

### Where the record lives

The record is fetch's own, kept the way `filer-decision-ledger.md` keeps
verdicts:

- **The interface.** `SpentRecord` is a trait in
  `crates/fetch/src/price/spent/`, owned by `vfi-fetch`.
- **The implementations.** Both live beside the trait in `vfi-fetch`: a
  journal, which keeps entries in an append-only file, and an in-memory record
  for tests. Neither `vfi-store` nor any other crate implements or opens it.
- **Where the bytes live.** In the engine's local data directory, in a file of
  their own. The path is a parameter of the journal's constructor, supplied by
  the caller, with no default and nothing read from an environment or config
  file to find it. The file is never in the repository, never in the metrics
  store that `analyze → store` writes, and never inside the filer ledger's
  journal.
- **One record per source.** Tiingo's provider is handed Tiingo's record. A
  second source would get a file of its own.

The trait carries two operations:

- **Record one entry.** It is appended and nothing already kept is rewritten.
  This can fail, returning `Unkept` as the ledger does.
- **Read back every entry stamped after a moment.** This does not fail. An
  implementation reads what it holds when it is opened, and afterwards
  answers from that plus every entry it has been handed since, whether or not
  the write landed.

**The record stays an explicit input and output.** Every ask of the provider
is handed `&mut` a `SpentRecord`, beside the ticker and dates `price-provider.md`
already names. The provider reads the record and appends to it during the ask,
and keeps no field of it. Nothing about what was spent stays in the provider
between asks, and nothing stays in the engine between sessions except through
a record a caller opened at a path it chose. The parameter is not optional.
Where a caller has nothing kept it hands an empty record, so a missing record
always has to be written out at the call site.

### What one entry carries

Three things, all of which go on record:

- **The moment the request left.** This is wall-clock time, UTC, written as
  whole seconds from the Unix epoch plus the nanoseconds after that second.
  `ledger/journal/written.rs` already writes a moment this way.
- **The ticker.** Written as it went into the request path, so the monthly
  window counts what was actually sent. If the provider sent two spellings of
  one symbol, they count as two. That over-counts, which is the safe
  direction.
- **The source's host,** `api.tiingo.com`. The journal refuses to open a file
  holding entries for another host, so a record handed to the wrong provider
  is caught when it is opened instead of quietly counting nothing.

Those fields answer all three limits:

- **The daily window.** The number of entries stamped inside the trailing 24
  hours that end at the moment of asking.
- **The monthly window.** The set of distinct tickers stamped inside the
  trailing 31 days.
- **The pace.** The newest entry's moment is when the last request left.

The pace is not kept anywhere else. The newest entry already is that moment,
and keeping it twice would give one value two sources.

There is no key, no price, no status and no count in an entry. A count is
derived from the entries, never stored beside them.

### The moment, and where it comes from

The provider reads a wall clock that is handed to it as a parameter, beside
the `Clock` the pace already takes. A test drives it, as it drives the
ledger's `When`. The machine's implementation reads `SystemTime::now()`.

The moment is taken after the pace's wait ends and before the entry is
appended. The `Instant` stays what the pace counts against inside one process.
The wall-clock moment is the only thing written down, because it is the only
moment a later process can compare with its own clock.

That reading stays in fetch. It is never a valuation date, which the caller
names and the engine never reads off a clock (`price-provider.md`). It never
enters a price answer, and it never reaches analyze. Anchor 4 binds analyze,
and analyze is not touched.

### One ask, in order

1. **Read the record** for entries stamped inside the last 31 days, against a
   wall reading taken now.
2. **Check the allowances.** If 1000 entries fall inside the trailing 24
   hours, every date asked is an absence naming the daily allowance, and
   nothing is sent. If the ticker is not among the window's distinct tickers
   and there are already 500, every date asked is an absence naming the
   month's symbols, and nothing is sent. Both are reasons `price-provider.md`
   already has. Waiting only empties the windows further, so a check passed
   here still holds after the wait.
3. **Seed the pace from the record.** The next request is owed at the newest
   entry's moment plus 72 s. If that is later than the wall reading, the pace
   is told that no turn comes before `Instant::now()` plus the difference. A
   stamp later than the wall reading means the clock was set back, and then
   the owed time is capped at one spacing from now. The pace takes whichever
   of that and its own next turn is later, then waits.
4. **Stamp and append** the entry.
5. **Send.**

**The entry lands before the request leaves.** A process killed while writing
an entry has not sent that entry's request, so the journal is right to pass
over an entry cut short. A process killed after the append and before the
send has an entry for a request that never left. That over-counts by one,
which is the safe direction. A request the source refused stays counted,
because whether the source counts refusals is not published.

### No record, an unreadable record, and a record gone stale

None of these fails a run, and none produces a wrong price.

- **Handed no record.** The type does not allow this. The nearest thing is an
  empty record, either a new file or the in-memory one. That reads as nothing
  spent, so the full allowance is there and the first request is free. That
  is exactly right on the first session there has ever been, and the source's
  refusal covers every other case (see below).
- **A record that cannot be read.** It is refused where it is opened and never
  reaches the provider. The journal reads its file back whole when it is opened,
  as the ledger's does. An I/O error, an entry that is whole but does not
  parse, or an entry for another host makes the open return
  `Unkept::Unread`, naming the path and the byte. Only a final entry cut short
  is passed over, for the reason given in the steps above. The caller then
  holds no record and cannot ask the provider. It can do one of two things:
  - **Ask no price.** Every price-dependent metric is then absent. How that
    absence reaches analyze is M5-06's to decide.
  - **Hand an empty record on purpose.** This is the case above, chosen in the
    caller's own code.

  Neither fails, and neither sends anything the record would have stopped.
  Which one the shell does is the shell's call.
- **A record whose entries have all left the windows.** No entry is inside 24
  hours or 31 days, and the newest is more than 72 s old. The provider has its
  full allowance and owes no wait. That is the correct answer, because the
  source's windows have emptied too. Entries older than 31 days are never read
  again.
- **An entry that cannot be kept.** If the append returns `Unkept`, the entry
  still counts for the rest of the session in the record's own view, and the
  request is sent. The journal keeps the write failure for its holder to read.
  The provider has no reason it could answer with. `price-provider.md`'s
  reasons are closed, and none of them means the record could not be kept. A
  later session reading that file under-counts by the entries that did not
  land, which is one of the gaps listed below.

### The store question

`price-provider.md` placed the record in store. This record places it
elsewhere, for four reasons:

- **The first alternative the ledger rejected.** A store-backed `SpentRecord`
  would have `vfi-store` implementing a trait `vfi-fetch` defines. That is the
  edge `vfi-store>vfi-fetch`, which runs backward, and the deps gate refuses
  it by name.
- **The other direction is closed too.** Fetch reading store is the edge
  `vfi-fetch>vfi-store`. It skips two stages and is not among the five
  allowed. Adding it means widening `allowed_edges`, which is Constitutional.
- **Store would not be using it.** Store holds analysis output, one row per
  company and period. A count of requests is not analysis output, no stage
  after fetch reads it, and M6 can rank nothing by it.
- **There is nothing to share yet.** The record has one consumer, the provider
  that writes it.

**Anchor 2 holds.** `vfi-fetch` gains no workspace dependency. The only
reader of the record is the stage that wrote it, reading its own earlier
output as its own input, so no later stage calls an earlier one. This record
needs no `allowed_edges` entry.

### What this leaves of GOALS.md's second criterion to the source refusing

Inside one engine with a record it can read, the limits hold by count and by
pace across restarts, and not by the source refusing. What is left to the
source is spending this engine cannot see:

- **The key spent elsewhere.** The same key used by another tool, or on
  another machine with its own record.
- **A record lost.** The file deleted or moved, or replaced on purpose by an
  empty record after an unreadable open.
- **Two writers.** Two engines appending to one record at once. Each one's
  view is what it read at open plus its own appends, so each under-counts the
  other. One writer per file is the caller's to hold, as it is for the ledger.
- **Entries that did not land,** as described in the last case above.
- **The wall clock set forward between sessions.** Entries then look older
  than they are, so the windows empty early. A clock set back only slows the
  provider down.

In each of these cases, a request the source refuses is an absence carrying
the source's status, as `price-provider.md` already says, and never a wrong
price. The bandwidth budget stays the estimate that record left to its
implementing task.

## Alternatives

- **Store implements `SpentRecord`.** This is `price-provider.md`'s "store's".
  It is rejected because it is the backward edge `filer-decision-ledger.md`
  rejected first, as argued above.
- **The trait lives in `vfi-contracts`, with store implementing it.** Rejected.
  Store would need `vfi-store>vfi-contracts`, which M5-06 proposes but which is
  not accepted. A contract is a boundary between stages, and the record has
  only one side. The cost would buy a second consumer that does not exist.
- **The provider returns the entry it spent, and its caller keeps it.** This
  is the purest reading of "input and output", and it is rejected. The entry
  has to land before the request leaves. If the caller kept it after the ask
  returned, a process that died during the wait of up to 72 s, or during the
  send, would leave a request sent with no entry for it, and the next session
  would under-count. There is also no composition root to do the keeping, the
  same reason the ledger's fourth alternative failed.
- **An optional record, where `None` means count nothing.** Rejected. The
  result is the same as an empty record, but `None` reads at the call site
  like a default, and the protection would then be switched off by leaving
  something out.
- **The pace kept apart from the counts,** as a file holding the moment the
  last request left. Rejected. The newest entry is that moment, and two places
  for one value is what the one-source-of-truth invariant forbids.
- **Asking the source what is left of the allowance.** Rejected. None of the
  pages `price-provider.md` read publishes a way to ask, or what the source
  answers once a limit is spent.
- **Entries in the filer ledger's journal.** Rejected. The ledger is a walled
  component with its own interface and its own field set. Adding a second
  kind of record to it would be a second consumer, which needs an ADR of its
  own, and it would mix verdicts with request counts in one migration surface.

## Consequences

- **Easier:**
  - A restart remembers the counts and the pace. A session begun 30 s after
    another waits the 42 s still owed, and starts with what the trailing
    windows have left.
  - One entry answers all three limits, and a second source is a second
    record at a second path.
  - Tests run against the in-memory record with a wall clock they drive, and
    touch no disk.
  - No gate changes, and no crate gains a dependency.
- **Harder:**
  - The engine now writes three things locally: the filer journal, the
    metrics store, and this record. The caller supplies two of the paths.
  - The windows trust the wall clock, which can be set forward, as described
    above.
  - The journal only grows, by at most 1000 entries a day. Entries older than
    31 days are never read again, but opening still reads past them. Dropping
    them means rewriting the file, which append-only rules out, so compaction
    is left open until a record's size makes it matter.
  - An entry that cannot be kept is sent anyway. Failing closed instead would
    take a reason meaning the record could not be kept, which would change
    `price-provider.md`'s closed list. That is above this record and not done
    here.
- **Expensive to reverse:** the entry's field set, once records exist on real
  machines. The layout beneath the fields is the implementation's to choose.
  The framing `ledger/journal/written.rs` uses would fit. Whatever layout is
  chosen has to hold three things: an entry appends whole, the record reads
  back whole, and a kill during a write costs at most the entry being written.

## Enforcement

This applies anchor-derived invariants. What holds each, with every test
added by the implementing task:

- **A later session starts with neither a full allowance nor a free first
  request.** A test hands a fresh provider and a fresh `Pace` a record whose
  newest entry is 30 s old on a driven wall clock. It finds the first request
  held for the remaining 42 s. A record holding 1000 entries inside 24 hours
  gives an absence naming the daily allowance, and a recording transport is
  handed nothing.
- **The month's symbols.** With 500 distinct tickers inside 31 days, a new
  ticker gets an absence and a counted ticker is still sent.
- **A stale record is a full allowance.** With every entry older than 31 days,
  the request is sent at once.
- **The entry lands before the request leaves.** A transport that reads the
  record at the moment it is handed a request finds that request's entry
  already there.
- **Unreadable is refused at the open.** A whole entry that does not parse,
  and an entry for another host, each fail the journal's open. A final entry
  cut short is passed over, and every entry before it counts.
- **A clock set back slows the provider.** An entry stamped after the wall
  reading owes at most one spacing.
- **No edge.** The deps gate already refuses `vfi-store>vfi-fetch` and
  `vfi-fetch>vfi-store`, with no change to the gate.
- **Gap.** Nothing checks the following, which are review, as they are for the
  ledger:
  - that the provider keeps no field holding the record
  - that nothing outside `price/spent/` opens the file
  - that the path a caller supplies is outside the repository and the metrics
    store
  - that one record has one writer

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
