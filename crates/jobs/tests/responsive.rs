//! That a slow job leaves its caller free.
//!
//! M2's last criterion is a deliberately slow job running while the interface
//! stays responsive. There is no interface yet, so what is checked here is the
//! half that exists: the caller keeps its thread, does its own work while the
//! job runs beside it, and still gets the result back at the end.
//!
//! This is an ordinary test target, so `cargo test --workspace` selects it and
//! the test gate covers it. It is deliberately not a fixture or a benchmark —
//! there is no expected output to pin and no cost to measure, only a property
//! that holds or does not.

/// How many turns of its own work the caller takes before it gives up waiting
/// for the job to show progress.
///
/// This is not a timeout on correct behaviour: a job whose thread has started
/// passes in a handful of turns. It is what turns the one failure that could
/// otherwise hang — a job that never runs at all — into a red test, and it is
/// far above any number of turns a live job could hide behind.
const CALLER_TURNS: u64 = 100_000_000;

#[test]
fn the_caller_works_while_the_job_runs() {
    let job = vfi_jobs::slow_job();

    // Stopping the job is the only thing that ends it, and nothing has stopped
    // it, so a start that waited on the result could not have reached this line.
    assert!(
        !job.is_finished(),
        "the job was already over by the time its caller held the handle"
    );

    // The caller's own work, on the caller's thread, while the job runs on
    // another. It keeps taking turns until it has watched the job advance —
    // that is what makes this the two running at once rather than one after the
    // other, and it is the part a blocking start could not produce.
    let mut turns: u64 = 0;
    let mut watched = 0;
    while watched == 0 {
        turns += 1;
        assert!(
            turns <= CALLER_TURNS,
            "the job published no progress across {CALLER_TURNS} turns of the caller's own work"
        );
        watched = job.steps();
    }

    assert!(
        !job.is_finished(),
        "the job ended on its own after {turns} turns; only stopping it ends it"
    );

    job.stop();
    let steps = job.wait();

    assert!(
        steps >= watched,
        "the job returned {steps} steps, behind the {watched} its caller watched it reach"
    );
}

#[test]
fn a_job_that_ends_by_itself_still_hands_back_its_result() {
    let job = vfi_jobs::start(|link| {
        link.reached(1);
        "what the job returns"
    });

    assert_eq!(job.wait(), "what the job returns");
}
