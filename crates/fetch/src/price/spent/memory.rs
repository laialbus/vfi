//! A spent record kept in memory, for tests.
//!
//! It touches no disk and outlives nothing. What it is for is every case about
//! the provider rather than about the journal: a test that drives an ask and
//! then reads what was spent should fail over the entries, not over a file it
//! had to create and remove.

use std::time::SystemTime;

use super::{Entry, SpentRecord, Unkept};

/// A spent record in memory: what it was given, in the order it was given it.
#[derive(Debug, Default)]
pub struct InMemory {
    kept: Vec<Entry>,
}

impl InMemory {
    /// A record with nothing spent on it.
    pub fn new() -> Self {
        Self::default()
    }
}

impl SpentRecord for InMemory {
    fn record(&mut self, entry: Entry) -> Result<(), Unkept> {
        self.kept.push(entry);

        Ok(())
    }

    fn after(&self, moment: SystemTime) -> Vec<Entry> {
        super::after(&self.kept, moment)
    }
}

#[cfg(test)]
mod tests {
    use super::InMemory;
    use crate::price::spent::{Entry, SpentRecord};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const TIINGO: &str = "api.tiingo.com";

    fn at(seconds: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn sent(seconds: u64, ticker: &str) -> Entry {
        Entry::new(at(seconds), ticker, TIINGO)
    }

    fn record(spent: &mut InMemory, entry: Entry) {
        spent
            .record(entry)
            .expect("a record in memory takes an entry");
    }

    /// After the moment asked, and only after it: an entry stamped at the
    /// moment itself is the boundary of the window and not inside it.
    #[test]
    fn entries_stamped_after_the_moment_read_back_and_none_at_or_before_it() {
        let mut spent = InMemory::new();
        let before = sent(1_754_000_000, "AAPL");
        let on = sent(1_754_000_072, "MSFT");
        let after = sent(1_754_000_144, "KO");

        for entry in [before, on, after.clone()] {
            record(&mut spent, entry);
        }

        assert_eq!(spent.after(at(1_754_000_072)), [after]);
    }

    /// The order handed in, not the order stamped. A wall clock set back
    /// between two requests stamps the later one earlier, and both still count.
    #[test]
    fn entries_read_back_in_the_order_they_were_recorded() {
        let mut spent = InMemory::new();
        let first = sent(1_754_000_144, "AAPL");
        let second = sent(1_754_000_072, "AAPL");

        record(&mut spent, first.clone());
        record(&mut spent, second.clone());

        assert_eq!(spent.after(at(1_754_000_000)), [first, second]);
    }

    /// A record nothing was spent on reads back as nothing spent, which is an
    /// answer and not a failure.
    #[test]
    fn a_record_with_nothing_spent_reads_back_as_nothing() {
        assert!(InMemory::new().after(UNIX_EPOCH).is_empty());
    }
}
