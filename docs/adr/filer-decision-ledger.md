# Fetch records filer verdicts through a ledger it is handed, into a journal of its own

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-08-09, by `M3-04`
- **Decided:** —
- **Touches:** a new explicit interface, for the component ANCHORS.md names as
  the filer decision ledger, and the storage schema of the records behind it.
  Either alone is above a worker, which is why this is a proposal and not a
  commit.

## Context

M3 asks for a funnel — seed set, then a metadata gate, then history for
survivors — and for every filer it evaluates to carry a verdict and a reason on
record, the rejected ones included. The funnel is fetch's work: it decides which
filers are worth a history request. The record of that decision has to outlive
the run, because a filer dropped in August must still be explicable in December,
and because the next pass should be able to say what changed rather than
re-derive it.

Anchor 2 is what makes that awkward. The permitted edges are fetch → normalize →
analyze → store, and `scripts/gates.sh` writes down exactly those three: a
forward edge that skips a stage is left out on purpose, and a backward edge is
refused by name — the compiler catches only the pair that closes a cycle, so the
gate is what reads the rest of the graph. The stage that produces the verdicts
therefore cannot hand them to the stage that persists things. Nor can it be
handed a sink that `vfi-store` implements, because that is the same edge drawn
the other way. And there is nothing above the two to do the wiring: every crate
in the workspace is a library, nothing constructs the pipeline, and the shell
that will one day call the engine does not exist yet and may not write local
persistence when it does.

ANCHORS.md already fixes part of the answer. The filer decision ledger is named
there among the components many stages depend on, which are reached only through
an explicit interface, and storage sits behind an interface rather than a
concrete database. What is open is what that interface is, what one record
carries, and where the records live.

## Decision

The ledger is reached through one interface — a `FilerLedger` trait, defined in
`crates/fetch/src/ledger/` and owned by the fetch stage. The funnel is handed an
implementation as a parameter rather than constructing one, so nothing about
where records go is compiled into the funnel and the engine keeps no per-user
state. The trait carries two operations: record one verdict, and read back what
was recorded for one filer. Everything else — the file, the format, the
ordering — is behind it.

Recording is the only way through the funnel. Each step yields verdicts, and the
set that goes on to the next step is derived from the verdicts recorded, not
assembled beside them. A filer that reached the history step without a record is
then not something a reviewer has to notice, because no path produces one.

One verdict record carries:

- **Which filer** — the identifier EDGAR keys on, and the ticker and name it was
  seen under, as observed, because both change over time and the record is of a
  judgement made at a moment. A seed entry that resolves to no identifier is
  itself a verdict at the seed step, keyed by whatever there was.
- **Which step judged it** — seed, metadata, or history. The funnel has three,
  and a verdict without its step cannot be read: "rejected" means something
  different at each.
- **The verdict** — admitted, rejected, or not judged because the evaluation
  itself failed. The third is not a rejection. A filer nobody could judge and a
  filer judged and dropped are different states, and merging them is how a
  transport failure becomes a permanent exclusion.
- **The reason** — a named reason, defined once alongside the step that can
  issue it, together with the values it was judged on. "Rejected" with no reason
  is the defect this record exists to prevent, and a reason with no evidence
  cannot be checked afterwards.
- **Where that evidence came from** — the request that produced it, the same
  provenance every fetched record carries, so a verdict traces back to the bytes
  it was made from without asking the source again.
- **When, and under what** — the moment, the funnel pass it belongs to, and the
  version of the ruleset that judged it. A pass is long enough to be interrupted
  part way, and without the pass on the record, which verdicts belong to the
  last complete sweep is reconstructed from timestamps, which is a guess.

Those records live behind the interface, in one append-only journal in the
engine's local data directory, at a path the caller supplies — never in the
repository, and never in the metrics store that the analyze → store edge writes.
Append-only is the shape and not an implementation detail: nothing is
overwritten, and a later verdict does not replace an earlier one, so a rejection
reversed next month reads as the two facts it is. Current state is the latest
record for a filer and step. The file layout underneath is the implementation's
own business and may change behind the trait; what it must hold is that one
record appends whole, that the journal reads back whole, and that a process
killed mid-write costs at most the record it was writing.

This stays inside anchor 2 because fetch never calls store. The ledger is not a
later stage; it is a sink fetch defines and is handed, and the one implementation
of it lives in `vfi-fetch` beside the trait, with a second in memory for tests.
`vfi-fetch` gains no workspace dependency, so the three edges the deps gate
allows are untouched.

The day the ledger has a second consumer — presentation reading verdicts back at
M7, another stage, or a store-backed implementation — that is an ADR of its own,
by the same anchor that walls this component off, and that is where the
allowed-edge question is answered. Having the interface from the first day is
what makes that move cost an implementation rather than a rewrite of the funnel.

## Alternatives

- **`vfi-store` implements the sink.** The trait stays in fetch and the local
  store implements it. This is the textbook inversion, and here it is precisely
  the edge anchor 2 forbids: `vfi-store` would depend on `vfi-fetch`, which is
  backward, and the deps gate refuses it. Rejected — it is the violation with
  the arrow drawn the other way.
- **The trait lives in a crate both sides depend on** — `contracts/`, or a new
  shared crate. Structurally the cleanest, and where this ends up if the ledger
  ever has two consumers. Rejected for now on three counts: `allowed_edges`
  would have to learn an edge for each side, and widening a gate is
  Constitutional, so it is a human's decision and not the decider's; what goes
  inside a contract file is itself an undecided ADR, which the contracts gate
  says it is waiting for; and there is one consumer today, so the whole cost
  buys nothing yet.
- **The verdicts ride the pipeline as ordinary output, and store writes them at
  the end.** No new edge at all — they travel forward as data, the way anchor 2
  intends data to travel. Rejected: normalize and analyze would each carry a
  field neither of them reads, which is what anchor 3 denies them, and the
  records that matter most have nothing to ride on, since a rejected filer's
  history is exactly what is never fetched.
- **The funnel returns its verdicts and its caller persists them.** The honest
  form of the one above, and the right answer on the day the engine has a
  composition root. It has none today, and the only caller above fetch is a
  shell that does not exist and may not write local persistence when it does.
  Rejected as unbuildable now rather than as wrong.

## Consequences

Easier: the funnel can be built next, with no gate change, no contract decision,
and no new crate. Tests drive the ledger in memory and touch no disk. A verdict
outlives the pass that made it, so a later pass can report a change instead of
re-deriving one.

Harder: the engine acquires a second thing that persists, beside the metrics
store M6 builds, so "where does this live" has two answers where it had one. A
journal answers "what happened to this filer" well and "which filers were
rejected for this reason" badly; the day the second question matters, the
implementation moves behind the trait and the shared-crate question above comes
due. The read path that shows a user why a filer is absent is not settled here.

Expensive to reverse: the field set of a record, once journals exist on real
machines — that is a migration, and it is the part of this decision worth
arguing about now. The layout beneath the fields is cheap to change while the
only journals are ours, and stops being cheap the moment one is not.

## Enforcement

This changes no anchor; it applies two. The half that is already mechanical:
the deps gate reads every intra-workspace edge, dev edges included, against the
three anchor 2 allows, and this design adds none — so `vfi-fetch` depending on
`vfi-store`, the move that would break it, fails the gate the day it is written,
with no change to the gate.

What nothing checks: that a second consumer does not quietly appear, that the
funnel does not reach around the trait to the journal file, and that nothing
else opens that file for writing. Those are review, and saying so here is the
only guard they have.

## Decision review

Left for the decider.
