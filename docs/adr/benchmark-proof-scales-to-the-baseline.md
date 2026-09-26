# The benchmark proof of catch allocates a tenth of the committed baseline per call, so it goes red whatever the stage's size

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-26, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-26, by Albus Lai (human), by merging the pull request
  that carries this record, and by the `human-approved` label on the task's
  pull request, since it edits `scripts/gates.sh`
- **Touches:** the benchmark gate's proof of catch in the gate runner, a
  protected path; the baseline file's fields; and one sentence of the accepted
  `golden-claims-and-baseline-re-recording.md`, which keeps its target and
  gains a magnitude. No threshold moves and no gate is weakened: the gate
  itself is untouched, and its proof becomes one that can fail.

## Context

M4-44 stopped before writing anything. Its acceptance says the benchmark proof
of catch must still go red once `normalize()` is the stage, and the worker
measured that it cannot. The proof, `violate_benchmark`, inserts one line into
the first one-line `pub fn` of the benched stage's `lib.rs`:
`std::hint::black_box(String::from("a deliberate slowdown"))`, one allocation
of 21 bytes per call. Against today's stub, one allocation per measurement, that
doubles the count and `work 1.05` catches it. Against the stage over the
restatement filer it is one allocation beside 283,306 and 21 bytes beside
megabytes: a move of one part in three hundred thousand, invisible to a
five-percent threshold, and to the cost line too. The proof was written for a
stub, and it stops proving anything at twenty allocations per call.

## Decision

The slowdown scales to the baseline it is meant to breach. `violate_benchmark`
reads the benched stage's committed `baseline` — the `bytes` it records and
the `passes` it was taken at — and inserts one allocation per call of
`ceil(bytes / passes / 10)` bytes, one at least:
`std::hint::black_box(vec![0u8; N])`. Over the measurement that adds a tenth to
the bytes the gate counts, against a threshold of a twentieth, so the proof
goes red at any stage size, deterministically, on the counted measurement the
thresholds file calls exact. The target is unchanged: the first public function
`lib.rs` states on one line, found and not named, exactly as the runner does
today.

For the runner to read `passes`, the baseline file carries it: one line,
`passes 512`, beside `allocations`, `bytes` and `cost`, and the harness reads
the count from there instead of from a constant, so the number the baseline was
taken at is stated once, where the baseline is. The harness header already says
changing the count restates the baseline; now the file says at what count.

`benchmarks/thresholds` is untouched, and so is its rationale: the gate still
asks whether the stage does a different amount of work than its baseline. Only
the deliberate violation grows with the thing it violates.

## Alternatives

**Compare counted work as an absolute difference.** It would see one
allocation, but it changes what the gate asks and the reason the thresholds
file gives for headroom, for the sake of a proof.

**Let the proof target something other than the entry point.** The
golden-claims record fixed the target so the proof cannot drift onto a function
the benchmark does not run; a scaled slowdown keeps that and needs no exception.

**A time-based slowdown, for the cost line.** The cost ratio is the noisy
measurement, with a tenfold threshold because of it; a proof that leans on it
would flake where the counted one cannot.

**Leave the proof as it is.** A gate with a proof that cannot fail is the
"gate nobody has watched refuse something" that M0 says is indistinguishable
from no gate.

## Consequences

**Easier.** M4-44 proceeds, and every later stage that grows keeps a proof that
catches. The baseline file says its own pass count.

**Harder.** The runner parses two numbers from a file it already reads. The
gate runner is protected, so the task lands under a human-approved label.

**Expensive to reverse.** Nothing.

## Enforcement

The proof is its own enforcement: `scripts/gates.sh` runs it on every full
sweep and exits 3 where it does not catch. Task M4-50 makes the change and
shows the proof red on the stub before M4-44 makes `normalize()` the stage.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging and on the label.

- **Authority:** Structural, and within the owner's reach: it edits a proof of
  catch to make it catch, weakens no gate and moves no threshold. Weakening or
  removing a gate would be Constitutional; this is the opposite.
- **Checked:** `violate_benchmark` and the proof loop in the gate runner; the
  harness's counting allocator, `PASSES`, `exceeds` and its comparator cases;
  the thresholds file's two rationales; the worker's measurements of 283,306
  and 81,068 allocations per call; the golden-claims record's sentence on the
  proof's target.
- **Verdict and why:** accepted. The gate's own threshold is a ratio, so the
  violation that proves it has to be a ratio too.
- **What would have changed it:** a benched stage whose baseline cannot state
  its bytes. Every baseline does.
