//! The spent record as a file: one journal per source, appended to and never
//! rewritten.
//!
//! Where it sits is the caller's, always. The path arrives as a parameter, and
//! nothing here reads an environment, a config file or a constant to find a
//! journal. What the caller owes is that the path is outside the repository,
//! the metrics store and the filer ledger's journal, and that one journal is
//! open on it at a time; this end cannot see any of that, which is why the ADR
//! says so and no gate does.
//!
//! A journal is one source's. It is told its host when it is opened and refuses
//! a file holding an entry for another, so a record handed to the wrong
//! provider is caught at the open instead of quietly counting nothing.
//!
//! The layout holds three properties, as the filer ledger's journal does:
//!
//! - **An entry appends whole.** One entry is one `write_all` of one buffer, to
//!   a file opened for append, with no buffer above the file to lose.
//! - **The record reads back whole, in the order written.** The open reads the
//!   file through once and keeps every entry, so a journal reopened answers
//!   what the one that wrote it answered.
//! - **A kill during a write costs at most the entry being written.** An entry
//!   cut short is recognised by its own length (see [`written`]) and passed
//!   over; every entry that landed before it still reads back. The next append
//!   closes the cut-short line off rather than continuing it, so an entry cut
//!   short can stand inside the file as well as at its end — it is still the
//!   final entry of the session that was killed writing it.
//!
//! A line that landed whole and does not parse, an entry for another host, and
//! an I/O error part way through are not passed over: the open answers each
//! with [`Unkept::Unread`], naming the path and the byte the line it stopped at
//! begins at. An unreadable record read as nothing spent would send exactly the
//! requests it exists to hold back.
//!
//! An entry whose append returns [`Unkept`] still counts in this journal's own
//! view for the rest of the session, because its request is sent all the same.
//! The failure is kept for the journal's holder to read in [`Journal::unwritten`].
//! A later session opening the file does not see that entry, and under-counts
//! by every entry that did not land: one fewer request in the trailing day, a
//! ticker missing from the trailing month if it was that ticker's only entry,
//! and a pace seeded from an older last request than the one that left.
//!
//! What this does not buy is a machine losing power, which needs a `fsync` per
//! entry. An entry here reaches the operating system, not the platter.

mod written;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::SystemTime;

use super::{Entry, SpentRecord, Unkept};

/// A spent record that keeps what it is given in a file at a path it is handed.
pub struct Journal {
    host: Box<str>,
    sink: File,
    /// What the file held at the open, then every entry handed in since,
    /// whether or not its write landed.
    kept: Vec<Entry>,
    /// The failures of entries whose write did not land, in the order they
    /// were handed in.
    unwritten: Vec<io::Error>,
    /// Whether the file ends in a whole line. It does not when a write was cut
    /// short, and then the next entry is written after a terminator of its own
    /// so that the two never share a line.
    ends_whole: bool,
}

impl Journal {
    /// The journal of requests sent to `host`, at `path`, created there if
    /// there is none.
    ///
    /// The file is created and the directory holding it is not, so a mistyped
    /// path is the operating system's complaint rather than an empty record
    /// nobody can find. Opening reads the whole file back, so anything that is
    /// not this source's journal is answered here and never reaches a provider.
    pub fn at(path: impl AsRef<Path>, host: &str) -> Result<Self, Unkept> {
        let path = path.as_ref();
        let unread = |at, why| Unkept::Unread {
            path: path.to_path_buf(),
            at,
            why,
        };

        let sink = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|why| unread(0, why))?;
        let (kept, ends_whole) = read_back(path, host).map_err(|(at, why)| unread(at, why))?;

        Ok(Self {
            host: Box::from(host),
            sink,
            kept,
            unwritten: Vec::new(),
            ends_whole,
        })
    }

    /// Why each entry that did not land did not, oldest first. Each of them
    /// still counts in this journal's view, and none of them is in the file.
    pub fn unwritten(&self) -> &[io::Error] {
        &self.unwritten
    }

    fn append(&mut self, entry: &Entry) -> io::Result<()> {
        // Written, it would make the file one this journal refuses to open.
        if entry.host() != &*self.host {
            return Err(io::Error::other(format!(
                "an entry for {} is not one for {}, whose record this is",
                entry.host(),
                self.host
            )));
        }

        let line = written::line(entry).map_err(io::Error::other)?;

        // One buffer, so one entry is one call on the file. The terminator in
        // front closes off an entry an earlier write left cut short.
        let mut append = Vec::with_capacity(line.len() + 1);
        if !self.ends_whole {
            append.push(written::END_OF_LINE);
        }
        append.extend_from_slice(&line);

        self.ends_whole = false;
        self.sink.write_all(&append)?;
        self.ends_whole = true;

        Ok(())
    }
}

impl SpentRecord for Journal {
    fn record(&mut self, entry: Entry) -> Result<(), Unkept> {
        let landed = self.append(&entry);
        self.kept.push(entry);

        landed.map_err(|why| {
            self.unwritten
                .push(io::Error::new(why.kind(), why.to_string()));
            Unkept::Unwritten { why }
        })
    }

    fn after(&self, moment: SystemTime) -> Vec<Entry> {
        super::after(&self.kept, moment)
    }
}

/// Every entry at `path`, in the order written, and whether the file ends in a
/// whole line — or the byte the line it stopped at begins at, and why.
fn read_back(path: &Path, host: &str) -> Result<(Vec<Entry>, bool), (u64, io::Error)> {
    let mut journal = BufReader::new(File::open(path).map_err(|why| (0, why))?);
    let mut kept = Vec::new();
    let mut line = Vec::new();
    let mut at = 0;
    let mut ends_whole = true;

    loop {
        line.clear();
        let read = journal
            .read_until(written::END_OF_LINE, &mut line)
            .map_err(|why| (at, why))?;
        if read == 0 {
            break;
        }

        ends_whole = line.last() == Some(&written::END_OF_LINE);
        if ends_whole {
            line.pop();
        }

        match written::read(&line) {
            Ok(Some(entry)) if entry.host() == host => kept.push(entry),
            Ok(Some(entry)) => {
                return Err((
                    at,
                    io::Error::other(format!(
                        "an entry for {}, in the record of requests sent to {host}",
                        entry.host()
                    )),
                ));
            }
            Ok(None) => {}
            Err(why) => return Err((at, io::Error::other(why))),
        }

        at += read as u64;
    }

    Ok((kept, ends_whole))
}

#[cfg(test)]
mod tests {
    use super::Journal;
    use crate::price::spent::{Entry, InMemory, SpentRecord, Unkept};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const TIINGO: &str = "api.tiingo.com";

    /// A directory of this test's own, outside the repository, made by the test
    /// and taken away with it however the test ends.
    struct Scratch(PathBuf);

    impl Scratch {
        fn named(test: &str) -> Self {
            let at = std::env::temp_dir().join(format!("vfi-spent-{}-{test}", std::process::id()));
            let _ = fs::remove_dir_all(&at);
            fs::create_dir_all(&at).expect("a directory of its own is a test's to make");

            Self(at)
        }

        fn journal(&self) -> PathBuf {
            self.0.join("spent")
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn at(seconds: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn sent(seconds: u64, ticker: &str) -> Entry {
        Entry::new(at(seconds), ticker, TIINGO)
    }

    fn journal(path: &Path) -> Journal {
        Journal::at(path, TIINGO).expect("a journal opens where a test put one")
    }

    fn record(spent: &mut impl SpentRecord, entry: Entry) {
        spent.record(entry).expect("a record takes an entry");
    }

    /// The same entries, and the same question, put to a journal and to the
    /// record in memory: both answer after the moment asked and not at or
    /// before it.
    #[test]
    fn entries_stamped_after_the_moment_read_back_and_none_at_or_before_it() {
        let scratch = Scratch::named("after-the-moment");
        let mut on_disk = journal(&scratch.journal());
        let mut in_memory = InMemory::new();

        let before = sent(1_754_000_000, "AAPL");
        let on = sent(1_754_000_072, "MSFT");
        let after = sent(1_754_000_144, "KO");
        for entry in [before, on, after.clone()] {
            record(&mut on_disk, entry.clone());
            record(&mut in_memory, entry);
        }

        assert_eq!(
            on_disk.after(at(1_754_000_072)),
            std::slice::from_ref(&after)
        );
        assert_eq!(in_memory.after(at(1_754_000_072)), [after]);
    }

    /// Out of the file rather than out of anything the writing journal kept:
    /// the reader here is a second journal over the same path, which is what
    /// the next session is.
    #[test]
    fn a_journal_reopened_answers_what_the_one_that_wrote_it_answered() {
        let scratch = Scratch::named("reopened");
        let mut writer = journal(&scratch.journal());

        for entry in [
            sent(1_754_000_144, "AAPL"),
            sent(1_754_000_000, "MSFT"),
            Entry::new(at(1_754_000_216) + Duration::from_nanos(7), "KO", TIINGO),
        ] {
            record(&mut writer, entry);
        }

        for moment in [UNIX_EPOCH, at(1_754_000_000), at(1_754_000_144)] {
            assert_eq!(
                journal(&scratch.journal()).after(moment),
                writer.after(moment)
            );
        }
        assert_eq!(writer.after(UNIX_EPOCH).len(), 3);
    }

    #[test]
    fn an_entry_appended_after_the_open_reads_back_from_the_same_record() {
        let scratch = Scratch::named("appended-after-open");
        let earlier = sent(1_754_000_000, "AAPL");
        record(&mut journal(&scratch.journal()), earlier.clone());

        let mut spent = journal(&scratch.journal());
        let later = sent(1_754_000_072, "MSFT");
        record(&mut spent, later.clone());

        assert_eq!(spent.after(UNIX_EPOCH), [earlier, later]);
    }

    /// A record handed to the wrong provider is caught where it is opened,
    /// naming the file and the line it stopped at.
    #[test]
    fn an_entry_for_another_host_fails_the_open() {
        let scratch = Scratch::named("another-host");
        let path = scratch.journal();

        record(
            &mut Journal::at(&path, "api.example.com").expect("a journal opens"),
            Entry::new(at(1_754_000_000), "AAPL", "api.example.com"),
        );

        assert_unread_at(&path, 0);
    }

    /// A line that landed whole and does not parse is not read past, wherever
    /// it stands: the open names the byte its line begins at.
    #[test]
    fn a_whole_entry_that_does_not_parse_fails_the_open() {
        let scratch = Scratch::named("does-not-parse");
        let path = scratch.journal();

        record(&mut journal(&path), sent(1_754_000_000, "AAPL"));
        let landed = read(&path).len();
        let mut planted = read(&path);
        planted.extend_from_slice(b"7 {\"a\":1}\n");
        holds(&path, &planted);

        assert_unread_at(&path, landed as u64);
    }

    /// A process killed part way through the third of three entries, from
    /// every place inside it that the kill could land: what landed before it
    /// still reads back, and the entry in flight reads back only where its
    /// own bytes all landed and nothing but the line ending is missing.
    #[test]
    fn a_final_entry_cut_short_is_passed_over_and_every_entry_before_it_reads_back() {
        let scratch = Scratch::named("cut-short");
        let path = scratch.journal();

        let first = sent(1_754_000_000, "AAPL");
        let second = sent(1_754_000_072, "MSFT");
        let third = sent(1_754_000_144, "KO");

        let mut spent = journal(&path);
        record(&mut spent, first.clone());
        record(&mut spent, second.clone());
        let landed = read(&path).len();
        record(&mut spent, third.clone());
        let whole = read(&path);
        drop(spent);

        for killed_at in landed..whole.len() {
            holds(&path, &whole[..killed_at]);

            let mut kept = vec![first.clone(), second.clone()];
            if killed_at == whole.len() - 1 {
                kept.push(third.clone());
            }

            assert_eq!(
                journal(&path).after(UNIX_EPOCH),
                kept,
                "a journal cut at byte {killed_at} does not hold what landed before the cut"
            );
        }
    }

    /// And the session after that one. The entry cut short is not continued:
    /// the next entry is a line of its own, and the file still opens with the
    /// wreck inside it rather than at its end.
    #[test]
    fn an_entry_recorded_after_one_cut_short_reads_back() {
        let scratch = Scratch::named("after-a-cut");
        let path = scratch.journal();

        let landed = sent(1_754_000_000, "AAPL");
        let after = sent(1_754_000_144, "KO");

        let mut spent = journal(&path);
        record(&mut spent, landed.clone());
        let whole = read(&path).len();
        record(&mut spent, sent(1_754_000_072, "MSFT"));
        let cut_short = read(&path);
        drop(spent);
        holds(&path, &cut_short[..whole + 4]);

        record(&mut journal(&path), after.clone());

        assert_eq!(journal(&path).after(UNIX_EPOCH), [landed, after]);
    }

    /// An entry that cannot be kept still counts in the record's own view, and
    /// the failure is there for the holder to read. Here it is an entry for a
    /// host the journal is not, which no write would have made readable.
    #[test]
    fn an_entry_that_did_not_land_still_counts_for_the_session() {
        let scratch = Scratch::named("did-not-land");
        let mut spent = journal(&scratch.journal());
        let elsewhere = Entry::new(at(1_754_000_000), "AAPL", "api.example.com");

        assert!(matches!(
            spent.record(elsewhere.clone()),
            Err(Unkept::Unwritten { .. })
        ));
        assert_eq!(spent.after(UNIX_EPOCH), [elsewhere]);
        assert_eq!(spent.unwritten().len(), 1);
        assert!(journal(&scratch.journal()).after(UNIX_EPOCH).is_empty());
    }

    /// The path is the caller's, and so is the directory holding it.
    #[test]
    fn a_journal_makes_its_file_and_not_the_directory_holding_it() {
        let scratch = Scratch::named("no-directory");
        let missing = scratch.0.join("nothing-made-this").join("spent");

        assert!(matches!(
            Journal::at(&missing, TIINGO),
            Err(Unkept::Unread { path, at: 0, .. }) if path == missing
        ));

        let path = scratch.journal();
        assert!(journal(&path).after(UNIX_EPOCH).is_empty());
        assert!(path.exists());
    }

    fn assert_unread_at(path: &Path, byte: u64) {
        match Journal::at(path, TIINGO) {
            Err(Unkept::Unread {
                path: named, at, ..
            }) => {
                assert_eq!(named, path);
                assert_eq!(at, byte);
            }
            Err(unkept) => panic!("the open failed as {unkept} and not as unread"),
            Ok(_) => panic!("{} opened as a journal for {TIINGO}", path.display()),
        }
    }

    /// What the journal file holds, byte for byte. A test that cuts one short
    /// has to see the bytes; nothing else outside this module does.
    fn read(path: &Path) -> Vec<u8> {
        fs::read(path).expect("a journal a test wrote is there")
    }

    /// The journal file holding exactly `bytes`, which for a prefix of what it
    /// held is what a process killed part way through a write leaves behind.
    fn holds(path: &Path, bytes: &[u8]) {
        fs::write(path, bytes).expect("a test writes its own scratch file");
    }
}
