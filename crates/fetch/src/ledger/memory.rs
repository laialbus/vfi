//! A ledger that keeps what it is given in memory, for tests.
//!
//! It touches no disk, opens nothing, and outlives nothing. What it is for is
//! every case about the funnel rather than about the journal: a test that drives
//! a pass and then asks what was recorded should fail over the verdicts, not
//! over a file it had to create and remove.
//!
//! Append-only in the same sense the journal is. A later verdict for one filer
//! and step is kept behind the earlier one rather than in place of it, so a test
//! passing against this is a test about the shape both implementations hold to,
//! and not about the one that forgets less.

use std::collections::BTreeMap;

use super::{FilerKey, FilerLedger, Record, Unkept};

/// A ledger in memory: what it was given, in the order it was given it.
#[derive(Debug, Default)]
pub struct InMemory {
    kept: BTreeMap<FilerKey, Vec<Record>>,
}

impl InMemory {
    /// A ledger with nothing on it.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FilerLedger for InMemory {
    fn record(&mut self, verdict: Record) -> Result<(), Unkept> {
        self.kept
            .entry(verdict.filer().key().clone())
            .or_default()
            .push(verdict);

        Ok(())
    }

    fn verdicts(&self, filer: &FilerKey) -> Result<Vec<Record>, Unkept> {
        Ok(self.kept.get(filer).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::InMemory;
    use crate::company::{Cik, Ticker};
    use crate::ledger::{
        Filer, FilerKey, FilerLedger, Pass, Reason, Record, Ruleset, Step, Verdict, When,
    };
    use crate::source::Source;
    use std::time::{Duration, SystemTime};

    const RULES: Ruleset = Ruleset::version(1);
    const PASS: Pass = Pass::new(7);

    const MAITONG_SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0002003750.json";
    const APPLE_SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0000320193.json";

    /// A moment written down rather than read off the machine, which is the
    /// whole of what a pinned time costs here.
    fn at(seconds: u64) -> When {
        When::new(
            SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
            PASS,
            RULES,
        )
    }

    fn maitong() -> Filer {
        Filer::keyed(Cik::new(2003750))
            .seen_as(&Ticker::new("MGSD"))
            .named("Maitong Sunshine Cultural Development Co., Ltd")
    }

    fn apple() -> Filer {
        Filer::keyed(Cik::new(320193))
            .seen_as(&Ticker::new("AAPL"))
            .named("Apple Inc.")
    }

    fn dropped(filer: Filer, submissions: &str, when: When) -> Record {
        Record::new(
            filer,
            Step::Metadata,
            Verdict::Rejected(Reason::new(
                "filed as a foreign private issuer",
                "form",
                "20-F",
            )),
            Source::new(submissions),
            when,
        )
    }

    fn admitted(filer: Filer, submissions: &str, when: When) -> Record {
        Record::new(
            filer,
            Step::Metadata,
            Verdict::Admitted(Reason::new("files the annual report", "form", "10-K")),
            Source::new(submissions),
            when,
        )
    }

    /// Every field, out the way it went in, moment included. The assertion is
    /// over the whole record rather than a field of it, because what this
    /// implementation is trusted for downstream is that reading back changes
    /// nothing.
    #[test]
    fn what_was_recorded_is_what_reads_back() {
        let verdict = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));
        let mut ledger = InMemory::new();

        ledger
            .record(verdict.clone())
            .expect("a ledger in memory refuses nothing");

        assert_eq!(
            ledger
                .verdicts(maitong().key())
                .expect("a ledger in memory refuses nothing"),
            [verdict]
        );
    }

    /// The moment is the record's, and it is the one it was given. A test that
    /// had to wait for a clock to prove this is a test somebody later deletes.
    #[test]
    fn a_moment_reads_back_as_the_one_it_was_recorded_at() {
        let mut ledger = InMemory::new();

        ledger
            .record(dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000)))
            .expect("a ledger in memory refuses nothing");

        let kept = ledger
            .verdicts(maitong().key())
            .expect("a ledger in memory refuses nothing");

        assert_eq!(
            kept[0].when().moment(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_000_000)
        );
        assert_eq!(kept[0].when().pass(), PASS);
        assert_eq!(kept[0].when().ruleset(), RULES);
    }

    /// Nothing is overwritten. A filer rejected in August and admitted in
    /// December is two judgements that were both made, and a ledger that kept
    /// the second in place of the first would answer "was this ever rejected"
    /// with no.
    #[test]
    fn a_later_verdict_is_kept_behind_the_earlier_one() {
        let mut ledger = InMemory::new();
        let earlier = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));
        let later = admitted(maitong(), MAITONG_SUBMISSIONS, at(1_764_000_000));

        for verdict in [earlier.clone(), later.clone()] {
            ledger
                .record(verdict)
                .expect("a ledger in memory refuses nothing");
        }

        assert_eq!(
            ledger
                .verdicts(maitong().key())
                .expect("a ledger in memory refuses nothing"),
            [earlier, later]
        );
    }

    /// One filer's verdicts and not another's, and a filer nothing was recorded
    /// for reads back as nothing recorded. That last one is an answer: it is
    /// what a filer no step has reached yet looks like, and the funnel deriving
    /// its survivors from these needs it to be told from a rejection.
    #[test]
    fn verdicts_come_back_for_the_filer_they_were_recorded_for() {
        let mut ledger = InMemory::new();
        let verdict = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));

        ledger
            .record(verdict.clone())
            .expect("a ledger in memory refuses nothing");
        ledger
            .record(admitted(apple(), APPLE_SUBMISSIONS, at(1_754_000_001)))
            .expect("a ledger in memory refuses nothing");

        assert_eq!(
            ledger
                .verdicts(maitong().key())
                .expect("a ledger in memory refuses nothing"),
            [verdict]
        );
        assert!(
            ledger
                .verdicts(&FilerKey::Unresolved(Box::from("ZZZZ")))
                .expect("a ledger in memory refuses nothing")
                .is_empty()
        );
    }
}
