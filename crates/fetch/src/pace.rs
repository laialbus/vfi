//! How fast requests may leave, and the clock that says when.
//!
//! [`Pace`] holds one turn queue. Every request takes a turn before it is sent
//! and turns are handed out [`crate::MINIMUM_SPACING`] apart, so the rate the
//! source sees is the rate it publishes however many callers there are — which
//! is the point, since the published limit is over the source and not over any
//! one of them. Callers share a `Pace` by cloning it: a clone is another handle
//! on the same queue, and a second `Pace` is a second allowance nobody granted.
//!
//! The clock is a parameter for the same reason storage is an interface: a
//! bound that can only be watched by waiting for it can only be proved by
//! spending the time, and a proof that costs real seconds is one somebody later
//! deletes rather than waits for.

use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Instant;

use crate::policy;

/// What time it is, to whatever this is a pace against.
///
/// Monotonic by contract, in both methods: [`Clock::now`] never goes backward,
/// and [`Clock::wait_until`] returns no sooner than the moment it is given. A
/// clock that broke either would hand out turns in the past, which is a rate
/// without a bound.
pub trait Clock: Send + Sync {
    /// What time it is now.
    fn now(&self) -> Instant;

    /// Return no sooner than `moment`, and at once if it has passed.
    fn wait_until(&self, moment: Instant);
}

/// The clock the machine has, and the one every pace uses that is not a test.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn wait_until(&self, moment: Instant) {
        let remaining = moment.saturating_duration_since(Instant::now());
        if !remaining.is_zero() {
            thread::sleep(remaining);
        }
    }
}

/// The rate limit, as one thing several callers hold at once.
///
/// Cloning shares it. Everything a clone knows lives behind the one `Arc`, so
/// two handles cannot each think they are first.
#[derive(Clone)]
pub struct Pace(Arc<Turns>);

struct Turns {
    clock: Box<dyn Clock>,
    /// The moment the next turn may be taken, or `None` while none has been.
    /// Nothing is owed before the first request: a pace built at startup and
    /// used an hour later does not hold that request back.
    next: Mutex<Option<Instant>>,
}

impl Pace {
    /// A pace against `clock`.
    pub fn new(clock: impl Clock + 'static) -> Self {
        Self(Arc::new(Turns {
            clock: Box::new(clock),
            next: Mutex::new(None),
        }))
    }

    /// A pace against the machine's clock.
    pub fn system() -> Self {
        Self::new(SystemClock)
    }

    /// Take the next turn, and hold the caller until it comes.
    ///
    /// The turn is taken under the lock and waited out from outside it. That
    /// order is what makes the bound hold under callers arriving at once: each
    /// one leaves with a turn nobody else has, in the order they arrived, and
    /// the waits then overlap instead of queueing behind each other.
    pub(crate) fn release(&self) {
        self.0.clock.wait_until(self.take_turn());
    }

    fn take_turn(&self) -> Instant {
        // A panic under this lock cannot leave half an instant behind, so a
        // poisoned lock is still a lock over a good value. Refusing here would
        // stop every later request over a panic somewhere else.
        let mut next = self.0.next.lock().unwrap_or_else(PoisonError::into_inner);

        let now = self.0.clock.now();
        let turn = match *next {
            Some(owed) if owed > now => owed,
            _ => now,
        };
        *next = Some(turn + policy::MINIMUM_SPACING);

        turn
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, Pace};
    use crate::policy::MINIMUM_SPACING;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    /// A clock that stands still until something waits on it. What a turn is
    /// worth is proved from outside the crate, over the seam a caller uses;
    /// this is here for the one thing that seam cannot show, which is what a
    /// pace does with time that passed while nobody was asking.
    #[derive(Clone)]
    struct Held(Arc<Mutex<Instant>>);

    impl Clock for Held {
        fn now(&self) -> Instant {
            *self.0.lock().expect("no case here panics under this lock")
        }

        fn wait_until(&self, moment: Instant) {
            let mut now = self.0.lock().expect("no case here panics under this lock");
            if moment > *now {
                *now = moment;
            }
        }
    }

    /// Time passing is not a turn saved up. A pace that went an hour unused
    /// owes the next request nothing, and owes the one after it the spacing
    /// from then rather than from an hour ago — otherwise an idle stretch buys
    /// a burst, and a burst is what the source is counting.
    #[test]
    fn a_turn_nobody_took_is_not_saved_up() {
        let held = Held(Arc::new(Mutex::new(Instant::now())));
        let pace = Pace::new(held.clone());

        pace.take_turn();
        let later = held.now() + MINIMUM_SPACING * 100;
        held.wait_until(later);

        assert_eq!(pace.take_turn(), later);
        assert_eq!(pace.take_turn(), later + MINIMUM_SPACING);
    }
}
