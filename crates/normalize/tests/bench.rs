//! The benchmark harness for the normalize stage.
//!
//! The workload and the numbers it must stay inside live under
//! `benchmarks/normalize/`, as data: `input` is what the stage runs over and
//! `baseline` is what it cost when the baseline was taken. The thresholds are
//! `benchmarks/thresholds`, shared by every stage. This file is the code that
//! measures; it writes none of those, for the reason the golden harness beside
//! it gives — a number the subject produced proves only that the subject agrees
//! with itself.
//!
//! ## Why this measures what it measures
//!
//! The baseline is committed, so it is compared on machines it was not recorded
//! on: a laptop writes it, a shared CI runner checks it. That rules out wall
//! clock as the measurement. A time recorded here says nothing about a runner
//! whose cores, memory, and neighbours are all different, and a gate built on
//! one either flakes or is set so loose it catches nothing. So neither number
//! below is a time.
//!
//! What is measured instead is the work, in two kinds:
//!
//! - **Counted work** — how many allocations the stage makes and how many bytes
//!   it asks for, over a fixed workload. Same code, same input, same counts, on
//!   every machine: there is no noise in this number at all, so a green run is
//!   repeatable by construction rather than by a threshold wide enough to
//!   swallow the variance. It is also the thing the volume stages are asked to
//!   keep low (`.claude/rules/rust.md`: few allocations, reused buffers), so a
//!   regression in it is a regression in the stated terms.
//!
//! - **Cost** — the stage's time divided by the time of the plainest copy of
//!   the same bytes, measured in the same run on the same machine. The division
//!   is what makes the number portable: the machine's speed is in both halves
//!   and cancels. What is left is how much the stage costs above moving its
//!   input, which is a unit that stays meaningful as the stage grows real work
//!   to do.
//!
//! Counted work is the sharp one — it catches a per-call allocation or an extra
//! pass exactly, at a few percent. Cost is the blunt one, and it is here for
//! what counted work cannot see: a slowdown that allocates nothing. A sleep, a
//! spin, a copy that became a scan. Its threshold is wide because a ratio of two
//! timings is not free of noise, so it catches the gross case and does not
//! pretend to catch a ten percent one.
//!
//! Between them there is still a gap: a change that is slower by a little and
//! allocates nothing passes both. Closing it needs a baseline per machine, which
//! is not a thing that can be committed, and that limit is the reason the gate
//! is shaped this way rather than an oversight in it.
//!
//! ## How it runs
//!
//! `scripts/gates.sh` runs this target by name, in release, as its own gate.
//! `cargo test` does not select it (`test = false` in the manifest), for the
//! reason the golden harness states: two gates in AGENTS.md have to be able to
//! go red apart, and a benchmark run under the test gate would report that gate
//! failing and leave this one a name that can never go red on its own.
//!
//! Release is part of the measurement, not a detail of it. The baseline
//! describes optimized code, and the same numbers taken from a debug build are
//! a different measurement of a different thing.
//!
//! There is one test rather than several because the measurement has to be the
//! only thing running while it happens. A second test would be a second thread
//! competing for the machine during the timing.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// `benchmarks/<stage>/` is this harness's half of `benchmarks/`, and it is what
/// scripts/gates.sh reads to decide which harnesses to run. A stage directory
/// with no harness to match is a workload nobody runs, so the gate goes red
/// there rather than here.
const STAGE: &str = "normalize";

/// How many times one measurement runs the stage over the workload. Enough that
/// the timing sits far above the clock's resolution, and enough that an
/// allocation made once per call reads as five hundred times the baseline rather
/// than a rounding difference. The baseline is taken at this number, so changing
/// it restates what the baseline describes.
const PASSES: usize = 512;

/// A timing is the smallest of this many runs, on both halves of the ratio.
/// Noise only ever adds time — a descheduled thread, a neighbouring job, a cold
/// cache — so the minimum is the closest thing to what the machine can do, and
/// one unlucky repetition is discarded rather than averaged in.
const REPETITIONS: usize = 9;

/// What one measurement of the workload cost. `allocations` and `bytes` are
/// counted and identical everywhere; `cost` is the ratio described in the header
/// and is the only number here a machine can move.
#[derive(Clone, Copy)]
struct Measured {
    allocations: u64,
    bytes: u64,
    cost: f64,
}

// ---------------------------------------------------------------------------
// Counting what the stage allocates
// ---------------------------------------------------------------------------

// Counting is armed per thread, not globally: the counter must see the stage's
// allocations and nothing else, and a global counter would also see whatever the
// test harness happens to do on another thread while the measurement runs. The
// cell holds a `Copy` type and is const-initialized, so reaching it allocates
// nothing — a counter that allocated would count itself.
thread_local! {
    static COUNTED: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
}

struct Counting;

impl Counting {
    /// A realloc counts as an allocation of its new size. It double-counts the
    /// bytes a growing buffer has already asked for, which is the intent:
    /// growing a buffer in steps is the pattern this number exists to make
    /// visible, and it should not read the same as asking for the size once.
    fn record(size: usize) {
        let _ = COUNTED.try_with(|counted| {
            if let Some((allocations, bytes)) = counted.get() {
                counted.set(Some((allocations + 1, bytes + size as u64)));
            }
        });
    }
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::record(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::record(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        Self::record(new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn counting(body: impl FnOnce()) -> (u64, u64) {
    COUNTED.with(|counted| counted.set(Some((0, 0))));
    body();
    COUNTED.with(|counted| counted.replace(None)).unwrap_or((0, 0))
}

// ---------------------------------------------------------------------------
// The workload and what it is measured against
// ---------------------------------------------------------------------------

/// One measurement's worth of the stage. The output buffer is reused across
/// passes and cleared rather than replaced, which is how the stage is meant to
/// be driven; the allocation count is a statement about that use, and it is the
/// use that matters.
///
/// `black_box` on both ends of the call is what stops the optimizer from
/// noticing that every pass computes the same thing and keeping one of them.
fn subject(input: &str, out: &mut String) {
    for _ in 0..PASSES {
        out.clear();
        vfi_normalize::normalize(black_box(input), black_box(&mut *out));
    }
}

/// The same bytes moved by the plainest thing that could move them. This is the
/// denominator of `cost`, and it is deliberately the floor rather than a
/// synthetic workload: what the ratio then says is how much the stage costs
/// above the copy it cannot avoid. Today the stage is that copy and the ratio is
/// about one; when the stage does real work the ratio rises and the baseline
/// restates it.
fn reference(input: &str, out: &mut String) {
    for _ in 0..PASSES {
        out.clear();
        out.push_str(black_box(input));
        black_box(&mut *out);
    }
}

fn measure(input: &str) -> Measured {
    let mut out = String::new();
    let (allocations, bytes) = counting(|| subject(input, &mut out));
    black_box(&out);

    Measured {
        allocations,
        bytes,
        cost: measure_cost(input),
    }
}

fn measure_cost(input: &str) -> f64 {
    let mut out = String::new();
    let mut copied = String::new();

    // Both buffers reach their full capacity before the clock starts, so the
    // repetition that pays for the allocation is not one of the timed ones.
    subject(input, &mut out);
    reference(input, &mut copied);

    let mut stage = Duration::MAX;
    let mut copy = Duration::MAX;
    for _ in 0..REPETITIONS {
        // Interleaved rather than run in two blocks: whatever the machine is
        // doing to one of them for a moment, it is doing to the other.
        let start = Instant::now();
        subject(input, &mut out);
        stage = stage.min(start.elapsed());

        let start = Instant::now();
        reference(input, &mut copied);
        copy = copy.min(start.elapsed());
    }

    assert!(
        !copy.is_zero(),
        "the reference copy measured no time at all, so there is nothing to \
         divide by; the workload is too small for this machine's clock"
    );
    stage.as_secs_f64() / copy.as_secs_f64()
}

// ---------------------------------------------------------------------------
// The comparison, and the cases that hold it to its own rule
// ---------------------------------------------------------------------------

/// A measurement against the baseline it must stay inside. The threshold is a
/// multiple, and the multiple itself is allowed — a measurement exactly at the
/// line is what the threshold says may happen, not what it forbids.
fn exceeds(measured: f64, baseline: f64, threshold: f64) -> bool {
    measured > baseline * threshold
}

/// One case apiece for what the comparison must say, either side of the line it
/// draws. These run in-band, before the measurement they decide the verdict on,
/// for the reason the contract checker's cases do: a comparison nobody has
/// watched refuse anything reads exactly like one that is holding, and the whole
/// of this gate is the comparison.
fn comparator_cases() -> [(&'static str, f64, f64, f64, bool); 9] {
    [
        // name                    measured  baseline  threshold  caught
        ("unchanged", 1.0, 1.0, 1.05, false),
        ("cheaper-than-baseline", 0.5, 1.0, 1.05, false),
        ("inside-the-threshold", 1.04, 1.0, 1.05, false),
        ("exactly-at-the-threshold", 1.05, 1.0, 1.05, false),
        ("just-past-the-threshold", 1.06, 1.0, 1.05, true),
        ("one-allocation-more", 2.0, 1.0, 1.05, true),
        ("an-allocation-per-pass", 512.0, 1.0, 1.05, true),
        ("nothing-where-nothing-is-allowed", 0.0, 0.0, 1.05, false),
        ("something-where-nothing-is-allowed", 1.0, 0.0, 1.05, true),
    ]
}

fn check_comparator() {
    let mut failures = String::new();
    for (name, measured, baseline, threshold, caught) in comparator_cases() {
        if exceeds(measured, baseline, threshold) != caught {
            let verdict = if caught { "lets it through" } else { "catches it" };
            let _ = writeln!(failures, "    {name}: the comparison {verdict}");
        }
    }
    assert!(
        failures.is_empty(),
        "the comparison does not hold to its own rule:\n{failures}"
    );
}

// ---------------------------------------------------------------------------
// Reading the committed numbers
// ---------------------------------------------------------------------------

fn benchmarks() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../benchmarks")
}

/// The numbers a committed file records, in the order asked for. Every way the
/// file can be wrong is a panic naming the file and the line: a key this does
/// not know, a key recorded twice, a key missing, a value that is not a finite
/// number, a line carrying more than the two fields. A gate whose numbers
/// arrived from a typo is not a gate, and the quiet version of that failure is a
/// default silently standing in for what the file meant to say.
fn read_numbers<const N: usize>(path: &Path, wanted: [&str; N]) -> [f64; N] {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: the benchmark reads this file ({e})", path.display()));

    let mut found = [None; N];
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }

        let mut fields = line.split_whitespace();
        let key = fields.next().unwrap_or_default();
        let Some(value) = fields.next() else {
            panic!("{}: line {number} records {key} without a number", path.display());
        };
        if fields.next().is_some() {
            panic!(
                "{}: line {number} carries more than a name and a number",
                path.display()
            );
        }

        let Some(slot) = wanted.iter().position(|name| *name == key) else {
            panic!("{}: line {number} names {key}, and this file records {wanted:?}", path.display());
        };
        let value: f64 = value
            .parse()
            .unwrap_or_else(|e| panic!("{}: line {number} records no number for {key} ({e})", path.display()));
        if !value.is_finite() {
            panic!("{}: line {number} records {key} as a number nothing can be compared against", path.display());
        }
        if found[slot].is_some() {
            panic!("{}: records {key} twice, and one of the two is not the one in force", path.display());
        }
        found[slot] = Some(value);
    }

    let mut numbers = [0.0; N];
    for (slot, value) in found.iter().enumerate() {
        numbers[slot] = value.unwrap_or_else(|| {
            panic!("{}: records no {}, so there is nothing to check against", path.display(), wanted[slot])
        });
    }
    numbers
}

// ---------------------------------------------------------------------------

#[test]
fn the_stage_stays_inside_its_committed_baseline() {
    // The comparison before what it decides.
    check_comparator();

    let root = benchmarks();
    let [work, cost] = read_numbers(&root.join("thresholds"), ["work", "cost"]);

    let stage = root.join(STAGE);
    let [allocations, bytes, baseline_cost] =
        read_numbers(&stage.join("baseline"), ["allocations", "bytes", "cost"]);

    let input = fs::read_to_string(stage.join("input")).unwrap_or_else(|e| {
        panic!("{}: the stage runs over this workload ({e})", stage.join("input").display())
    });
    assert!(
        !input.is_empty(),
        "{}: is empty, so the measurement is of nothing",
        stage.join("input").display()
    );

    let measured = measure(&input);

    let mut failures = String::new();
    let mut check = |name: &str, measured: f64, baseline: f64, threshold: f64| {
        if exceeds(measured, baseline, threshold) {
            let _ = writeln!(
                failures,
                "    {name}: {measured}, against a baseline of {baseline} and a \
                 threshold of {threshold}, which allows up to {}",
                baseline * threshold
            );
        }
    };

    check("allocations", measured.allocations as f64, allocations, work);
    check("bytes", measured.bytes as f64, bytes, work);
    check("cost", measured.cost, baseline_cost, cost);

    assert!(
        failures.is_empty(),
        "benchmarks/{STAGE} costs more than its committed baseline allows:\n{failures}\n  \
         this run measured allocations {}, bytes {}, cost {}.\n  \
         whether that is a regression or a baseline that no longer describes the \
         stage is the reader's call, and the baseline changes in the diff that \
         changed what it measures.",
        measured.allocations,
        measured.bytes,
        measured.cost
    );
}
