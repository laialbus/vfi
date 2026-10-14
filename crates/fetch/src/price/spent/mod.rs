//! The price provider's spent record: every request sent to one source, and
//! when it left.
//!
//! A free tier's allowance is counted in three parts — requests an hour,
//! requests over any trailing day, distinct tickers over any trailing 31 days —
//! and all three are answered by one kind of [`Entry`]: the moment a request
//! left, the ticker that went into it, and the host it went to. A count is
//! derived from the entries and never kept beside them, and the newest entry
//! is when the last request left, so nothing here keeps that twice either.
//!
//! [`SpentRecord`] is the interface, and it carries the two operations
//! `docs/adr/price-allowance-between-sessions.md` gives it: record one entry,
//! and read back every entry stamped after a moment. The provider is handed a
//! record on every ask and keeps no field of it, so what was spent stays an
//! explicit input and output and the engine keeps no per-user state.
//!
//! It is fetch's own interface for the reason the filer ledger is: the stage
//! that spends may not call the stage that persists, and a record `vfi-store`
//! implemented would be that edge drawn the other way.
//!
//! Every moment here is handed in. Nothing in this module reads a clock: the
//! provider stamps an entry with the wall reading it is handed, and asks for
//! the entries after a moment it names.
//!
//! Two implementations, and which one a caller is handed is the caller's:
//! [`Journal`], which keeps entries in an append-only file at a path it is
//! given, and [`InMemory`], which keeps them for the length of a test. Both
//! answer a read from what they held when opened plus every entry handed to
//! them since, so a test that passes against the second is a test about the
//! interface.

mod journal;
mod memory;

use std::fmt;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

pub use journal::Journal;
pub use memory::InMemory;

/// Where a request's entry goes, and the only way back to one.
pub trait SpentRecord {
    /// Put `entry` on record, behind every entry already there.
    ///
    /// Nothing already kept is rewritten. An entry that cannot be kept is
    /// answered with [`Unkept`], and still counts in this record's own view
    /// from then on: the request it stands for is sent all the same, and a
    /// record that forgot it would under-count.
    fn record(&mut self, entry: Entry) -> Result<(), Unkept>;

    /// Every entry stamped after `moment`, in the order recorded.
    ///
    /// An entry stamped at `moment` exactly is not after it. This does not
    /// fail: it answers from what the record held when it was opened and every
    /// entry it has been handed since, whether or not the write landed.
    fn after(&self, moment: SystemTime) -> Vec<Entry>;
}

/// One request, as it went on record.
///
/// Exactly three things. There is no key, no price, no status and no count in
/// an entry: a key would put a secret on disk, and a status would tempt a
/// reader to leave a refused request uncounted, when whether the source counts
/// refusals is not published.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    left: SystemTime,
    ticker: Box<str>,
    host: Box<str>,
}

impl Entry {
    /// The request that left at `left`, wall-clock, for `ticker` as it went
    /// into the request path, to `host`.
    ///
    /// The ticker is kept as the characters sent rather than as a [`Ticker`]
    /// would read them. Two spellings of one symbol are then two tickers, which
    /// over-counts the month's symbols, and over-counting is the safe side.
    ///
    /// [`Ticker`]: crate::Ticker
    pub fn new(left: SystemTime, ticker: &str, host: &str) -> Self {
        Self {
            left,
            ticker: Box::from(ticker),
            host: Box::from(host),
        }
    }

    /// When the request left, by the wall clock the provider was handed.
    pub fn left(&self) -> SystemTime {
        self.left
    }

    /// The ticker, as the characters that went into the request path.
    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    /// The host the request went to.
    pub fn host(&self) -> &str {
        &self.host
    }
}

/// Why a record could not do what was asked of it.
///
/// Neither of these is an allowance spent or left. A caller that read an
/// unreadable record as nothing spent would be sending exactly the requests
/// the record is there to hold back, so an unreadable one is refused where it
/// is opened and never reaches the provider.
#[derive(Debug)]
pub enum Unkept {
    /// The entry counts in this record's view and did not land in what a later
    /// session reads.
    Unwritten {
        /// What stopped it.
        why: io::Error,
    },
    /// What is on record at `path` could not be read back, from byte `at` on.
    Unread {
        /// The file the record was opened at.
        path: PathBuf,
        /// Where the line it stopped at begins.
        at: u64,
        /// What stopped it.
        why: io::Error,
    },
}

impl fmt::Display for Unkept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unkept::Unwritten { why } => {
                write!(f, "the request's entry is not on record, because {why}")
            }
            Unkept::Unread { path, at, why } => write!(
                f,
                "what is on record at {} did not read back from byte {at}, because {why}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for Unkept {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Unkept::Unwritten { why } | Unkept::Unread { why, .. } => Some(why),
        }
    }
}

/// The entries in `kept` stamped after `moment`, in the order they were kept.
///
/// Written order and not stamp order: a clock set back between two requests
/// stamps the later one earlier, and a reader that sorted would hide that.
fn after(kept: &[Entry], moment: SystemTime) -> Vec<Entry> {
    kept.iter()
        .filter(|entry| entry.left > moment)
        .cloned()
        .collect()
}
