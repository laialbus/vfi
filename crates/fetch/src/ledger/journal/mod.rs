//! The ledger as a file: one journal, appended to and never rewritten.
//!
//! Where it sits is the caller's, always. The path arrives as a parameter — the
//! engine holds no per-user state, so nothing here reads an environment, a
//! config file, or a constant to find a journal, and there is no default for a
//! caller to fall through to. What the caller owes is that the path is neither
//! inside the repository nor the metrics store the analyze → store edge writes;
//! this end of it cannot see either, which is why the ADR says so and this says
//! so and no gate does.
//!
//! Append-only, in the sense the ADR fixes rather than as a manner of speaking.
//! Nothing already on record is rewritten or moved: a later verdict for a filer
//! and step goes behind the earlier one, so a rejection reversed next month
//! reads as the two facts it is, and current state is the last record for that
//! filer and step. A journal only ever grows.
//!
//! Three properties are what the layout beneath has to hold to, and each is
//! bought here rather than argued:
//!
//! - **A record appends whole.** One record is one `write_all` of one buffer, to
//!   a file opened for append. Nothing sits in a buffer above the file, because a
//!   buffer is exactly what a killed process loses — and it loses it having told
//!   the caller the verdict was on record.
//! - **The journal reads back whole, in the order written.** Records are read
//!   out of the file rather than out of anything remembered beside it, so a
//!   journal reopened answers what the one that wrote it answers.
//! - **A process killed mid-write costs at most the record it was writing.** A
//!   record cut short is recognised by its own length (see [`written`]) and
//!   passed over; every record that landed before it still reads back. The next
//!   append closes the cut-short line off rather than continuing it, so the
//!   wreck cannot swallow the record written after it either.
//!
//! What this does not buy is a machine losing power, which is a different
//! failure with a different answer — a `fsync` per record, at a cost per record
//! this has no reason to pay yet. A record here reaches the operating system,
//! not the platter.
//!
//! One journal per file, which is the arrangement the ADR describes and the one
//! this holds to: a second writer on one path would leave a record cut short
//! that the first knows nothing about, and would append onto the end of it. The
//! interface is what keeps that to a review point rather than a race — nothing
//! outside here opens the file at all.
//!
//! One thing is remembered beside the file: where each filer's records begin.
//! That index is built by reading the journal once when it is opened and grows
//! by one number per record after that, and it is what keeps "what happened to
//! this filer" from being a scan of every verdict ever recorded. It holds
//! offsets and no records, so what it can be wrong about is where to look, and
//! the file is what answers.

mod written;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use super::{FilerKey, FilerLedger, Record, Unkept};

/// A ledger that keeps what it is given in a file at a path it is handed.
pub struct Journal {
    path: PathBuf,
    sink: File,
    /// Where each filer's records begin, in the order they were written.
    kept: BTreeMap<FilerKey, Vec<u64>>,
    /// Whether the file ends in a whole line. It does not when a write was cut
    /// short, and then the next record is written after a terminator of its own
    /// so that the two never share a line.
    ends_whole: bool,
}

impl Journal {
    /// The journal at `path`, created there if there is none.
    ///
    /// The file is created and the directory holding it is not. A journal that
    /// made its own path would turn a mistyped one into a tree of empty
    /// directories and a ledger nobody can find, which is the case where the
    /// operating system's own complaint is worth more than the convenience.
    ///
    /// Opening reads what is already on record, so a path holding something
    /// other than a journal is answered here rather than at the first lookup.
    pub fn at(path: impl AsRef<Path>) -> Result<Self, Unkept> {
        let path = path.as_ref().to_path_buf();
        let sink = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|why| Unkept::Unwritten { why })?;

        let (kept, ends_whole) = read_back(&path)?;

        Ok(Self {
            path,
            sink,
            kept,
            ends_whole,
        })
    }
}

impl FilerLedger for Journal {
    fn record(&mut self, verdict: Record) -> Result<(), Unkept> {
        let line = written::line(&verdict).map_err(unwritable)?;

        // One buffer, so one record is one call on the file. The terminator in
        // front closes off a record an earlier write left cut short: the wreck
        // stays where it is, unread, and this record starts a line of its own.
        let mut append = Vec::with_capacity(line.len() + 1);
        if !self.ends_whole {
            append.push(written::END_OF_LINE);
        }
        let at = self.sink.metadata().map_err(unwritten)?.len() + append.len() as u64;
        append.extend_from_slice(&line);

        // Not whole until the write says it is. A write that returns short of
        // the whole buffer has left a record cut short, and the next one heals
        // it rather than appending onto the end of it.
        self.ends_whole = false;
        self.sink.write_all(&append).map_err(unwritten)?;
        self.ends_whole = true;

        self.kept
            .entry(verdict.filer().key().clone())
            .or_default()
            .push(at);

        Ok(())
    }

    fn verdicts(&self, filer: &FilerKey) -> Result<Vec<Record>, Unkept> {
        let Some(offsets) = self.kept.get(filer) else {
            return Ok(Vec::new());
        };

        let mut journal = BufReader::new(File::open(&self.path).map_err(unread)?);
        let mut kept = Vec::with_capacity(offsets.len());
        let mut line = Vec::new();

        for at in offsets {
            journal.seek(SeekFrom::Start(*at)).map_err(unread)?;
            line.clear();
            journal
                .read_until(written::END_OF_LINE, &mut line)
                .map_err(unread)?;
            if line.last() == Some(&written::END_OF_LINE) {
                line.pop();
            }

            match written::read(&line) {
                Ok(Some(record)) => kept.push(record),
                // The index names whole records only, so neither of these is
                // the journal this ledger wrote: the file changed underneath it.
                Ok(None) => {
                    return Err(corrupt(format!(
                        "the record at byte {at} is no longer a whole one"
                    )));
                }
                Err(why) => return Err(corrupt(why)),
            }
        }

        Ok(kept)
    }
}

/// What is already on record at `path`: where each filer's records begin, and
/// whether the file ends in a whole line.
///
/// One pass, in the order the records were written, so the offsets a filer
/// collects are in that order too. A record cut short is passed over and the
/// pass carries on: the run that was killed part way through a journal cost
/// that record and no other.
fn read_back(path: &Path) -> Result<(BTreeMap<FilerKey, Vec<u64>>, bool), Unkept> {
    let mut journal = BufReader::new(File::open(path).map_err(unread)?);
    let mut kept: BTreeMap<FilerKey, Vec<u64>> = BTreeMap::new();
    let mut line = Vec::new();
    let mut at = 0;
    let mut ends_whole = true;

    loop {
        line.clear();
        let read = journal
            .read_until(written::END_OF_LINE, &mut line)
            .map_err(unread)?;
        if read == 0 {
            break;
        }

        // A line with no terminator is the end of the file, and the write that
        // was to have terminated it never returned. Whether the record itself
        // landed whole is the length's to say, not the terminator's.
        ends_whole = line.last() == Some(&written::END_OF_LINE);
        if ends_whole {
            line.pop();
        }

        match written::read(&line) {
            Ok(Some(record)) => kept
                .entry(record.filer().key().clone())
                .or_default()
                .push(at),
            Ok(None) => {}
            Err(why) => return Err(corrupt(format!("{why}, at byte {at}"))),
        }

        at += read as u64;
    }

    Ok((kept, ends_whole))
}

fn unwritten(why: io::Error) -> Unkept {
    Unkept::Unwritten { why }
}

fn unread(why: io::Error) -> Unkept {
    Unkept::Unread { why }
}

/// A verdict this cannot write down, which is a verdict no journal can hold.
fn unwritable(why: String) -> Unkept {
    Unkept::Unwritten {
        why: io::Error::other(why),
    }
}

/// A journal that does not read as one. Every implementation of the interface
/// is behind [`Unkept`], and what is behind this one is a file, so what a
/// caller is told is that what is on record did not read back.
fn corrupt(why: String) -> Unkept {
    Unkept::Unread {
        why: io::Error::other(why),
    }
}

#[cfg(test)]
mod tests {
    use super::Journal;
    use crate::company::{Cik, Ticker};
    use crate::ledger::{
        Filer, FilerKey, FilerLedger, Pass, Reason, Record, Ruleset, Step, Unkept, Verdict, When,
    };
    use crate::source::Source;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime};

    const RULES: Ruleset = Ruleset::version(1);
    const PASS: Pass = Pass::new(7);

    const MAITONG_SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0002003750.json";
    const APPLE_SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0000320193.json";

    /// A directory of this test's own, outside the repository, made by the test
    /// and taken away with it however the test ends. Named for the test and the
    /// process, so two tests at once — or two runs at once — never share one.
    struct Scratch(PathBuf);

    impl Scratch {
        fn named(test: &str) -> Self {
            let at =
                std::env::temp_dir().join(format!("vfi-journal-{}-{test}", std::process::id()));
            let _ = fs::remove_dir_all(&at);
            fs::create_dir_all(&at).expect("a directory of its own is a test's to make");

            Self(at)
        }

        fn journal(&self) -> PathBuf {
            self.0.join("verdicts")
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

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

    fn journal(path: &Path) -> Journal {
        Journal::at(path).expect("a journal opens where a test put one")
    }

    fn verdicts(journal: &Journal, filer: &FilerKey) -> Vec<Record> {
        journal.verdicts(filer).expect("a journal reads back")
    }

    fn record(journal: &mut Journal, verdict: Record) {
        journal.record(verdict).expect("a journal takes a record");
    }

    /// What was recorded, out of the file rather than out of anything the
    /// writing journal remembered: the reader here is a second journal over the
    /// same path, which is what the next run is.
    #[test]
    fn what_was_recorded_reads_back_out_of_the_file() {
        let scratch = Scratch::named("what-was-recorded");
        let verdict = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));

        record(&mut journal(&scratch.journal()), verdict.clone());

        assert_eq!(
            verdicts(&journal(&scratch.journal()), maitong().key()),
            [verdict]
        );
    }

    /// In the order written, which for one filer is the order its verdicts were
    /// reached — and interleaved with another filer's, because a pass records as
    /// it goes rather than a filer at a time.
    #[test]
    fn records_read_back_in_the_order_they_were_written() {
        let scratch = Scratch::named("in-order");
        let mut ledger = journal(&scratch.journal());

        let seed = Record::new(
            maitong(),
            Step::Seed,
            Verdict::Admitted(Reason::new("named in the ticker map", "ticker", "MGSD")),
            Source::new("https://www.sec.gov/files/company_tickers.json"),
            at(1_754_000_000),
        );
        let metadata = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_002));

        record(&mut ledger, seed.clone());
        record(
            &mut ledger,
            admitted(apple(), APPLE_SUBMISSIONS, at(1_754_000_001)),
        );
        record(&mut ledger, metadata.clone());

        assert_eq!(
            verdicts(&journal(&scratch.journal()), maitong().key()),
            [seed, metadata]
        );
    }

    /// Nothing is overwritten. Both verdicts read back, and the later of them is
    /// where that step now stands.
    #[test]
    fn a_later_verdict_is_kept_behind_the_earlier_one() {
        let scratch = Scratch::named("nothing-overwritten");
        let earlier = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));
        let later = admitted(maitong(), MAITONG_SUBMISSIONS, at(1_764_000_000));

        let mut ledger = journal(&scratch.journal());
        record(&mut ledger, earlier.clone());
        record(&mut ledger, later.clone());

        let kept = verdicts(&journal(&scratch.journal()), maitong().key());

        assert_eq!(kept, [earlier, later.clone()]);
        assert_eq!(kept.last(), Some(&later));
    }

    /// A filer nothing was recorded for reads back as nothing recorded, out of a
    /// journal that has records in it and out of one that has none. Both are an
    /// answer rather than a failure.
    #[test]
    fn a_filer_with_nothing_on_record_reads_back_as_nothing() {
        let scratch = Scratch::named("nothing-recorded");
        let unresolved = FilerKey::Unresolved(Box::from("ZZZZ"));

        assert!(verdicts(&journal(&scratch.journal()), &unresolved).is_empty());

        record(
            &mut journal(&scratch.journal()),
            dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000)),
        );

        assert!(verdicts(&journal(&scratch.journal()), &unresolved).is_empty());
    }

    /// The criterion a green suite would not notice missing, proved against a
    /// journal with a torn tail rather than argued: a pass records three
    /// verdicts, the process dies part way through the third, and what landed
    /// before it still reads back — from every place inside the record in flight
    /// that the process could have been killed.
    ///
    /// The record in flight rides on whether its own bytes all landed, which is
    /// its length's to say and not its terminator's. There is one cut where they
    /// did — the byte before the end — and the verdict there is a whole one that
    /// nothing but the line ending is missing from.
    #[test]
    fn a_process_killed_mid_write_costs_at_most_the_record_in_flight() {
        let scratch = Scratch::named("torn-tail");
        let path = scratch.journal();

        let first = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));
        let second = admitted(apple(), APPLE_SUBMISSIONS, at(1_754_000_001));
        let third = admitted(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_002));

        let mut ledger = journal(&path);
        record(&mut ledger, first.clone());
        record(&mut ledger, second.clone());
        let landed = read(&path).len();
        record(&mut ledger, third.clone());
        let whole = read(&path);
        drop(ledger);

        for killed_at in landed..whole.len() {
            holds(&path, &whole[..killed_at]);

            let mut kept = vec![first.clone()];
            if killed_at == whole.len() - 1 {
                kept.push(third.clone());
            }

            let ledger = journal(&path);
            assert_eq!(
                verdicts(&ledger, maitong().key()),
                kept,
                "a journal cut at byte {killed_at} does not hold what landed before the cut"
            );
            assert_eq!(
                verdicts(&ledger, apple().key()),
                std::slice::from_ref(&second)
            );
        }
    }

    /// And the run after that one. The record cut short is not continued — the
    /// verdict written next is its own line and reads back — and what landed
    /// before the tear is still there.
    #[test]
    fn a_verdict_recorded_after_a_torn_write_reads_back() {
        let scratch = Scratch::named("after-a-tear");
        let path = scratch.journal();

        let landed = dropped(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_000));
        let torn = admitted(maitong(), MAITONG_SUBMISSIONS, at(1_754_000_001));
        let after = admitted(apple(), APPLE_SUBMISSIONS, at(1_754_000_002));

        let mut ledger = journal(&path);
        record(&mut ledger, landed.clone());
        let whole = read(&path).len();
        record(&mut ledger, torn);
        let cut_short = read(&path);
        drop(ledger);

        holds(&path, &cut_short[..whole + 4]);

        let mut ledger = journal(&path);
        record(&mut ledger, after.clone());
        drop(ledger);

        let ledger = journal(&path);
        assert_eq!(verdicts(&ledger, maitong().key()), [landed]);
        assert_eq!(verdicts(&ledger, apple().key()), [after]);
    }

    /// A journal is a file this ledger wrote, so a path holding something else
    /// is answered rather than read past. The line planted here is whole — it is
    /// not a record cut short — and reading a record out of it would be reading
    /// one out of a file no ledger wrote.
    #[test]
    fn a_file_that_is_not_a_journal_is_not_read_as_one() {
        let scratch = Scratch::named("not-a-journal");
        let path = scratch.journal();

        holds(&path, b"a line no ledger wrote\n");

        assert!(matches!(Journal::at(&path), Err(Unkept::Unread { .. })));
    }

    /// The path is the caller's, and so is the directory holding it. A journal
    /// that made the directory would answer a mistyped path with an empty ledger
    /// nobody can find, in place of the mistake.
    #[test]
    fn a_journal_makes_its_file_and_not_the_directory_holding_it() {
        let scratch = Scratch::named("no-directory");

        assert!(matches!(
            Journal::at(scratch.0.join("nothing-made-this").join("verdicts")),
            Err(Unkept::Unwritten { .. })
        ));

        let path = scratch.journal();
        assert!(Journal::at(&path).is_ok());
        assert!(path.exists());
    }

    /// What the journal file holds, byte for byte. A test that cuts one short
    /// has to see the bytes; nothing else outside this module does.
    fn read(path: &Path) -> Vec<u8> {
        fs::read(path).expect("a journal a test wrote is there")
    }

    /// The journal file holding exactly `bytes` — which, for a prefix of what it
    /// held, is what a process killed part way through a write leaves behind.
    fn holds(path: &Path, bytes: &[u8]) {
        fs::write(path, bytes).expect("a test writes its own scratch file");
    }
}
