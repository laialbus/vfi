//! How this crate spells a moment on disk: whole seconds from the Unix epoch,
//! then the nanoseconds after that second.
//!
//! One spelling for every file the stage keeps, because a record that says when
//! something happened is read back by a later process that compares it with its
//! own clock, and two spellings of one moment are two places to get it wrong.
//! The filer ledger's journal and the price provider's spent record both write
//! a moment through here.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// `moment` as whole seconds from the epoch and the nanoseconds after that
/// second, the seconds running negative before 1970 and the nanoseconds never.
///
/// The nanoseconds never run negative, so the two always add. A machine whose
/// clock disagrees with the calendar is not this crate's to correct: what a
/// record says is what the clock said.
pub(crate) fn since_epoch(moment: SystemTime) -> Result<(i64, u32), String> {
    let uncountable =
        |elapsed| format!("{elapsed:?} from the epoch is further than a record counts");

    match moment.duration_since(UNIX_EPOCH) {
        Ok(after) => Ok((
            i64::try_from(after.as_secs()).map_err(|_| uncountable(after))?,
            after.subsec_nanos(),
        )),
        Err(before) => {
            let before = before.duration();
            let seconds = i64::try_from(before.as_secs()).map_err(|_| uncountable(before))?;

            match before.subsec_nanos() {
                0 => Ok((-seconds, 0)),
                nanos => Ok((-seconds - 1, 1_000_000_000 - nanos)),
            }
        }
    }
}

/// The moment `seconds` and `nanos` from the epoch count to, or why they count
/// to none.
pub(crate) fn moment(seconds: i64, nanos: u32) -> Result<SystemTime, String> {
    let unreachable =
        || format!("{seconds}s and {nanos}ns from the epoch is not a moment this machine holds");

    if nanos >= 1_000_000_000 {
        return Err(unreachable());
    }

    let whole = Duration::from_secs(seconds.unsigned_abs());
    let counted = if seconds < 0 {
        UNIX_EPOCH.checked_sub(whole)
    } else {
        UNIX_EPOCH.checked_add(whole)
    };

    counted
        .and_then(|counted| counted.checked_add(Duration::from_nanos(nanos.into())))
        .ok_or_else(unreachable)
}

#[cfg(test)]
mod tests {
    use super::{moment, since_epoch};
    use std::time::{Duration, UNIX_EPOCH};

    /// A moment out and back, on both sides of the epoch and either side of a
    /// whole second. A record that quietly moved a moment by a second would be
    /// a file disagreeing with itself about when something happened.
    #[test]
    fn a_moment_counts_from_the_epoch_and_back_to_itself() {
        for judged_at in [
            UNIX_EPOCH,
            UNIX_EPOCH + Duration::new(1_754_000_000, 1),
            UNIX_EPOCH + Duration::new(1_754_000_000, 999_999_999),
            UNIX_EPOCH - Duration::new(1, 0),
            UNIX_EPOCH - Duration::new(0, 1),
            UNIX_EPOCH - Duration::new(86_400, 500_000_000),
        ] {
            let (seconds, nanos) = since_epoch(judged_at).expect("a moment a machine holds counts");

            assert!(nanos < 1_000_000_000);
            assert_eq!(moment(seconds, nanos), Ok(judged_at));
        }
    }

    /// A count no moment answers to is what a corrupt record looks like from
    /// here, and it is refused rather than folded into the nearest moment.
    #[test]
    fn a_count_that_is_no_moment_is_refused() {
        assert!(moment(0, 1_000_000_000).is_err());
    }
}
