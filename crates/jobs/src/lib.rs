//! Running a long job without holding up whoever started it.
//!
//! The engine's genuinely long operations arrive with M3 — a filing history is
//! minutes of network, not microseconds of arithmetic — and the interface that
//! has to stay live while one runs is not built either, since the toolkit is
//! ADR-001 and still open. What exists here is the seam between the two: the
//! job runs on its own thread, and its caller holds a handle it can read, stop,
//! and finally take the result from, none of which makes it wait.
//!
//! This is not a pipeline stage. No stage depends on it and it depends on none,
//! so the order anchor 2 fixes is untouched by it.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::{self, JoinHandle};

/// What the two threads say to each other while the job runs: the job publishes
/// how far it has got, and the caller asks it to stop.
///
/// Each is one value written by one side and read by the other, and neither
/// carries anything else along with it — the result travels through the join in
/// [`Running::wait`], which orders itself. So `Relaxed` is the whole of what
/// these need, and it is chosen rather than settled for: a caller that had to
/// take a lock to read progress would be the thing this crate exists to avoid.
#[derive(Default)]
struct Shared {
    steps: AtomicU64,
    stop: AtomicBool,
}

/// A job that is already running, held from the side that started it.
pub struct Running<T> {
    shared: Arc<Shared>,
    worker: JoinHandle<T>,
}

impl<T> Running<T> {
    /// How far the job says it has got. Reading it never waits, so a caller may
    /// ask as often as it has turns to spare.
    pub fn steps(&self) -> u64 {
        self.shared.steps.load(Ordering::Relaxed)
    }

    /// Ask the job to stop. It stops at its next step boundary, so this returns
    /// while the job is very likely still running; [`Running::wait`] is what
    /// waits for the end.
    pub fn stop(&self) {
        self.shared.stop.store(true, Ordering::Relaxed);
    }

    /// Whether the job has reached its end. A false answer does not promise the
    /// next one is also false, and a true answer does not hand over the result.
    pub fn is_finished(&self) -> bool {
        self.worker.is_finished()
    }

    /// Wait for the job and take what it returned. This is the one call here
    /// that blocks, and a caller with anything else to do makes it the last one.
    ///
    /// A job that panicked panics its caller here, in front of someone, rather
    /// than dying quietly on a thread nobody is watching.
    pub fn wait(self) -> T {
        match self.worker.join() {
            Ok(result) => result,
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }
}

/// The running job's side of the same conversation: what it reports through,
/// and what it hears the caller through.
pub struct Link {
    shared: Arc<Shared>,
}

impl Link {
    /// Whether the caller has asked the job to stop. A long job reads this
    /// between steps; one that never reads it is one nothing can stop.
    pub fn stopping(&self) -> bool {
        self.shared.stop.load(Ordering::Relaxed)
    }

    /// Publish how far the job has got, for the caller to read while it runs.
    pub fn reached(&self, steps: u64) {
        self.shared.steps.store(steps, Ordering::Relaxed);
    }
}

/// Start `body` on a thread of its own and hand back the handle immediately.
///
/// The caller keeps its thread. Everything it can then ask of the job — its
/// progress, whether it has ended, stopping it — answers without waiting, and
/// the one call that waits says so in its name.
pub fn start<T, F>(body: F) -> Running<T>
where
    F: FnOnce(Link) -> T + Send + 'static,
    T: Send + 'static,
{
    let shared = Arc::new(Shared::default());
    let link = Link {
        shared: Arc::clone(&shared),
    };

    Running {
        shared,
        worker: thread::spawn(move || body(link)),
    }
}

/// A job that is long-running by design: it has no end of its own and works
/// until it is stopped, returning the number of steps it got through.
///
/// It stands in for the operations M3 onward will bring, and it is a stand-in
/// only. It holds its thread for as long as the caller leaves it running, which
/// is the one property M2 asks to be shown, and it claims nothing else — it
/// computes no value anything downstream reads, and nothing but the test beside
/// it calls this.
pub fn slow_job() -> Running<u64> {
    start(|link| {
        let mut steps = 0;
        while !link.stopping() {
            steps += 1;
            link.reached(steps);
        }
        steps
    })
}
