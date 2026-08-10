//! That nothing leaves faster than the rate the source publishes.
//!
//! Every case here runs against a clock it owns. Nothing sleeps, so the whole
//! file costs no more than the requests it makes — which matters because the
//! alternative is a proof that costs a real second per ten requests, and a proof
//! that costs seconds is one somebody later deletes rather than waits for.
//!
//! What the cases read is when each request was cleared to leave. The clock is
//! asked to wait until that moment and keeps it, and a request is sent when its
//! wait returns, so those moments are when requests left.
//!
//! Nothing in this file reaches a real source: the transport answers from
//! memory, and the only host it is given is one off the crate's own list.

use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use vfi_fetch::{
    ALLOWED_HOSTS, Cleared, Clock, Declaration, Egress, MAX_REQUESTS_PER_SECOND, MINIMUM_SPACING,
    Pace, Response, Transport,
};

/// A clock that stands still until something waits on it, and keeps every
/// moment it was asked to wait until.
///
/// It reads the machine's clock once, to have somewhere to count from, and
/// never again. Time here passes only because a request asked it to.
#[derive(Clone)]
struct TestClock(Arc<Mutex<Kept>>);

struct Kept {
    now: Instant,
    granted: Vec<Instant>,
}

impl TestClock {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Kept {
            now: Instant::now(),
            granted: Vec::new(),
        })))
    }

    /// Every moment a request was cleared to leave, earliest first.
    ///
    /// Sorted, because callers arriving at once record their turns in whatever
    /// order they get back to this lock. What the source sees is when requests
    /// leave and not which caller was quickest to write it down.
    fn granted(&self) -> Vec<Instant> {
        let mut granted = self.kept().granted.clone();
        granted.sort_unstable();
        granted
    }

    fn kept(&self) -> std::sync::MutexGuard<'_, Kept> {
        self.0.lock().expect("no case here panics under this lock")
    }
}

impl Clock for TestClock {
    fn now(&self) -> Instant {
        self.kept().now
    }

    fn wait_until(&self, moment: Instant) {
        let mut kept = self.kept();
        kept.granted.push(moment);
        if moment > kept.now {
            kept.now = moment;
        }
    }
}

/// A transport with no wire under it, counting what it was handed.
#[derive(Default)]
struct Answering {
    sent: usize,
}

impl Transport for Answering {
    fn send(&mut self, _request: Cleared<'_>) -> io::Result<Response> {
        self.sent += 1;
        Ok(Response {
            status: 200,
            body: Vec::new(),
        })
    }
}

fn declaration() -> Declaration {
    Declaration::new("VFI test suite nobody@example.invalid").expect("this names somebody")
}

/// A URL on a host the list allows, read off the list rather than written here.
fn listed_url() -> String {
    let host = ALLOWED_HOSTS
        .first()
        .expect("the list allows no host, so there is nothing to fetch from");
    format!("https://{host}/cgi-bin/browse-edgar")
}

fn chokepoint(pace: Pace) -> Egress<Answering> {
    Egress::new(Answering::default(), declaration(), pace)
}

fn fetch_all(egress: &mut Egress<Answering>, requests: usize) {
    let url = listed_url();
    for _ in 0..requests {
        egress.fetch(&url).expect("a listed host is sent");
    }
}

#[test]
fn requests_leave_one_published_spacing_apart() {
    let clock = TestClock::new();
    let mut egress = chokepoint(Pace::new(clock.clone()));

    fetch_all(&mut egress, 5);

    let granted = clock.granted();
    assert_eq!(granted.len(), egress.transport().sent);
    let first = granted[0];
    for (nth, moment) in granted.iter().enumerate() {
        assert_eq!(
            *moment,
            first + MINIMUM_SPACING * nth as u32,
            "request {nth} left {:?} in, where {:?} is its turn",
            moment.duration_since(first),
            MINIMUM_SPACING * nth as u32
        );
    }
}

/// The published limit stated as the source states it, rather than as the
/// spacing it was turned into: however the two are related, no second may carry
/// more than the rate. Read over every window rather than the first, since a
/// limiter that catches up after falling behind passes the first and fails a
/// later one.
#[test]
fn no_second_carries_more_than_the_published_rate() {
    let rate = MAX_REQUESTS_PER_SECOND as usize;
    let clock = TestClock::new();
    let mut egress = chokepoint(Pace::new(clock.clone()));

    fetch_all(&mut egress, rate * 2 + rate / 2);

    let granted = clock.granted();
    assert_eq!(granted.len(), egress.transport().sent);
    assert!(
        granted.len() > rate,
        "fewer requests than a second holds, so no window below is read at all"
    );
    for (nth, moment) in granted.iter().enumerate() {
        let Some(rate_later) = granted.get(nth + rate) else {
            break;
        };
        assert!(
            rate_later.duration_since(*moment) >= Duration::from_secs(1),
            "requests {nth} to {} left inside {:?}, and {rate} a second is the published rate",
            nth + rate,
            rate_later.duration_since(*moment)
        );
    }
}

/// The bound the source publishes is over the source, "regardless of the number
/// of machines used to submit requests". So it is proved over callers running at
/// once and sharing one pace, which is the arrangement a limit counted per
/// caller passes and a source refuses: four callers each keeping to the rate
/// would be four times it.
#[test]
fn the_bound_holds_across_callers_and_not_only_within_one() {
    let callers = 4;
    let each = 3;
    let clock = TestClock::new();
    let pace = Pace::new(clock.clone());

    thread::scope(|scope| {
        for _ in 0..callers {
            let pace = pace.clone();
            scope.spawn(move || fetch_all(&mut chokepoint(pace), each));
        }
    });

    let granted = clock.granted();
    assert_eq!(granted.len(), callers * each);
    for pair in granted.windows(2) {
        assert!(
            pair[1].duration_since(pair[0]) >= MINIMUM_SPACING,
            "two requests left {:?} apart, and {MINIMUM_SPACING:?} is the closest two may be",
            pair[1].duration_since(pair[0])
        );
    }
    assert!(
        granted[granted.len() - 1].duration_since(granted[0])
            >= MINIMUM_SPACING * (callers * each - 1) as u32,
        "the callers between them went faster than one of them could have"
    );
}

/// A withheld host reaches nothing, so it is not something the source is owed a
/// wait for. The two requests that did leave are one spacing apart and not two:
/// a refusal that spent a turn would slow every later request by the ones that
/// never happened.
#[test]
fn a_host_that_is_never_reached_costs_no_turn() {
    let clock = TestClock::new();
    let mut egress = chokepoint(Pace::new(clock.clone()));
    let url = listed_url();

    egress.fetch(&url).expect("a listed host is sent");
    egress
        .fetch("https://filings.example.invalid/latest")
        .expect_err("an unlisted host is not fetched");
    egress.fetch(&url).expect("a listed host is sent");

    let granted = clock.granted();
    assert_eq!(granted.len(), 2);
    assert_eq!(granted[1], granted[0] + MINIMUM_SPACING);
}

/// A request the transport failed on may have reached the source anyway — the
/// wire breaking says nothing about which end had already read the request — so
/// it costs its turn like any other. Counting only what came back would let a
/// source that is timing out be hammered.
#[test]
fn a_request_that_failed_on_the_wire_still_cost_its_turn() {
    struct Broken;

    impl Transport for Broken {
        fn send(&mut self, _request: Cleared<'_>) -> io::Result<Response> {
            Err(io::Error::other("the stand-in transport has no wire"))
        }
    }

    let clock = TestClock::new();
    let mut egress = Egress::new(Broken, declaration(), Pace::new(clock.clone()));
    let url = listed_url();

    egress.fetch(&url).expect_err("this transport has no wire");
    egress.fetch(&url).expect_err("this transport has no wire");

    let granted = clock.granted();
    assert_eq!(granted.len(), 2);
    assert_eq!(granted[1], granted[0] + MINIMUM_SPACING);
}
