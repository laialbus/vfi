//! The filer decision ledger: what this stage decided about a filer, and the
//! one way to reach it.
//!
//! The funnel evaluates far more filers than it keeps, and a rejected one
//! leaves nothing else behind — no filings, no metrics, no row anybody can ask
//! about. So the verdict is the whole of the trace, and it has to answer in
//! December what was decided in August: which filer, at which step, on what
//! evidence, under which rules.
//!
//! [`FilerLedger`] is the interface that holds, and it carries the two
//! operations `docs/adr/filer-decision-ledger.md` gives it: record one verdict,
//! and read back what was recorded for one filer. Everything else — the file,
//! the format, the ordering — is behind it. The funnel is handed an
//! implementation rather than building one, so nothing about where records go
//! is compiled into the stage and the engine keeps no per-user state.
//!
//! It is fetch's own interface, and that is anchor 2 rather than a preference:
//! the stage that decides these things may not call the stage that persists
//! them, and a sink `vfi-store` implemented would be the same edge drawn the
//! other way. The ADR argues that out, and the day this has a second consumer
//! is an ADR of its own.
//!
//! Half of what a record has to be is types rather than review. A verdict
//! cannot be recorded without the [`Step`] that made it, a [`Verdict::Rejected`]
//! cannot be recorded without its [`Reason`], and a reason is a rule named once
//! beside the step that issues it together with the values it was judged on —
//! never prose composed where it is recorded. None of those is a check that
//! runs; each is a record that does not compile.
//!
//! The half that is review: that nothing reaches a recorded verdict except
//! through here, and that no second thing opens what is behind it.

mod memory;
mod record;

use std::fmt;
use std::io;

pub use memory::InMemory;
pub use record::{Filer, FilerKey, Judged, Pass, Reason, Record, Ruleset, Step, Verdict, When};

/// Where a verdict goes, and the only way back to one.
pub trait FilerLedger {
    /// Put `verdict` on record.
    ///
    /// Nothing already recorded is replaced. A filer rejected under one pass and
    /// admitted under the next is two verdicts that were both reached, and a
    /// ledger that kept the second in place of the first could not say the first
    /// had ever happened.
    fn record(&mut self, verdict: Record) -> Result<(), Unkept>;

    /// Every verdict on record for `filer`, oldest first, so the last of them
    /// for a step is where that step now stands.
    ///
    /// A filer nothing was recorded for reads back as nothing recorded. That is
    /// an answer and not a failure: it is what a filer no step has reached looks
    /// like, and it is the answer a caller deriving one step's set from the step
    /// before it is asking for.
    fn verdicts(&self, filer: &FilerKey) -> Result<Vec<Record>, Unkept>;
}

/// Why the ledger could not do what was asked of it.
///
/// Neither of these is a verdict, and neither becomes one. A filer whose verdict
/// did not land has not been judged as anything, and a caller that read an
/// unkept record as a rejection would be making exactly the substitution the
/// three verdicts are kept apart to prevent.
///
/// The cause is an [`io::Error`] because what is behind this interface is a
/// file, and because an implementation that fails for some other reason still
/// has [`io::Error::other`] to say so in.
#[derive(Debug)]
pub enum Unkept {
    /// The verdict was reached and the record of it did not land.
    Unwritten {
        /// What stopped it.
        why: io::Error,
    },
    /// What is on record could not be read back.
    Unread {
        /// What stopped it.
        why: io::Error,
    },
}

impl fmt::Display for Unkept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unkept::Unwritten { why } => {
                write!(f, "the verdict is not on record, because {why}")
            }
            Unkept::Unread { why } => {
                write!(f, "what is on record did not read back, because {why}")
            }
        }
    }
}

impl std::error::Error for Unkept {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Unkept::Unwritten { why } | Unkept::Unread { why } => Some(why),
        }
    }
}
