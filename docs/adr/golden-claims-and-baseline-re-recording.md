# A normalize golden fixture pairs a hand-derived claims file with a recorded `expected`, and a stage that starts doing work re-records its benchmark baseline in the same diff

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-19, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-19, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** what the normalize golden harness checks, which is a gate, and
  what counts as moving the benchmark gate. Neither gate is weakened. The
  harness change is a later exclusive task; nothing is implemented here.

## Context

The 2026-09-18 planner left two questions before the golden fixtures and the
end-to-end wiring can be queued. First: the emit record counts 6,765 rendered
lines for the restatement filer alone and says "an expected result the subject
generated proves only that the subject agrees with itself", so how is an
`expected` of that size written by something other than the code it checks?
Second: `normalize()` is a one-line stub the benchmark measures today; when it
becomes the stage, does re-recording the baseline count as moving the gate?

## Decision

**Two files beside a normalize fixture's input.** `claims`, hand-derived from
the fetch fixture's facts and the accepted records, each claim naming the record
or the facts it was derived from: at minimum every claim the accepted records
make about that filer, the period set with its count, and every line of at
least two periods in full, one instant and one duration. And `expected`, the
full rendering, recorded from the engine's output once every claim holds against
it, and from then on a regression guard. The harness checks both: `expected`
byte for byte, and every claim against `expected`. The claims are the proof and
`expected` is the guard; the engine never writes a claim. `expected` is
re-recorded only in a diff that names the record or ruling that changed the
rendering and shows the claims still hold. A re-recording with no such statement
is the gate being moved.

For a filer no accepted record pins a claim for — CIK 0002011954 and CIK
0002033615 today — the floor is the same: the period set with its count, two
periods in full, and the claims that are the fixture's reason to exist, which
for those two are the negative equity and the `NotApplicable` rows a bank's
kind excludes, each derived by hand from the facts.

**The benchmark baseline, for every benched stage.** When a task changes what
the measured stage does by design — a stub that copies its input becomes the
stage — it re-records, in the same diff, the workload and the baseline
together: the workload becomes what the stage now reads, which for normalize is
one filer's facts as the fetch boundary publishes them, taken from a merged
fetch fixture so that its size is a fixed thing in the tree; the baseline is
taken over it at the pass count the harness states, and the old and new
figures are stated in the pull request and the session entry. The threshold
file is left where it is. The threshold is the gate; re-recording a baseline for
work that changed is not moving it. Re-recording with no such change, or
loosening a multiple, is. The gate's proof of catch slows the first public
function the stage's `lib.rs` states on one line, and that stays the stage's
entry point: the wiring task keeps `normalize()` there, and where it cannot,
the proof is out of reach and the task stops and escalates rather than shipping
a gate with no proof.

## Alternatives

**Hand-write every line.** Not reviewable at that size, and the mechanical
lines — an `Unknown` listing each attempt's candidates — would be transcribed
from output in practice while claiming not to be.

**An independent second implementation as oracle.** A second normalization to
keep in step with the first, which is the unchecked duplication the
one-source-of-truth invariant bans, and twice the surface for the same wrong
answer.

**Record `expected` alone.** The emit record's objection, and correct: it proves
stability, not correctness.

## Consequences

**Easier.** Fixture tasks are sized: a claims file is a night's work, and the
rendering is recorded. The decider re-derives claims, not 6,765 lines.

**Harder.** The harness gains a check, which is an exclusive task with its own
record since it touches a gate. Until it lands, the first fixture task writes
both files and the decider re-derives the claims by hand.

**Expensive to reverse.** The convention, once several fixtures carry it.

## Enforcement

The harness, once the claims check lands. Until then: the fixture task states
its claims in the file, and the decider's re-derivation is the check.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: it adds a check to a
  gate and loosens none.
- **Checked:** the gates in AGENTS.md; the emit record's sentence on
  `expected` from output and its registry-version rendering; the one-source-of-
  truth invariant; the benchmark gate's proof of catch as M4-38's prose
  describes it.
- **Verdict and why:** accepted. A proof of the lines that matter beside a
  guard over all of them is what a golden fixture can honestly be at this size.
- **What would have changed it:** a way to derive 6,765 lines independently at
  a cost a run can pay. There is none.
