//! The funnel: which filers this stage considers, and why each one goes on or
//! stops.
//!
//! Three steps, and [`run`] is all of them in one pass. [`seed`] reads the
//! filers there are out of what EDGAR publishes. [`gate`] judges each of them on
//! what EDGAR publishes about it, before anything asks what it has filed.
//! [`history`] retrieves what the survivors have filed, and nothing at all for
//! anyone else — which is the arithmetic the whole shape is for.
//!
//! Every filer any step reaches leaves a verdict behind, through
//! [`crate::ledger`] and nowhere else. That is not bookkeeping: most of the
//! corpus stops before the last step, and a filer that stops leaves no filings,
//! no metrics and no row anyone can ask about, so the verdict is the whole of
//! the trace that it was ever considered.
//!
//! What a step is handed is filers to consider, never which of them anything
//! kept. Whether a filer goes on is [`admitted`]'s answer, read back out of the
//! ledger, so a filer reaching the step after without a record is not something
//! a caller has to remember to prevent — there is no path that produces one.

mod gate;
mod history;
mod seed;

use std::fmt;
use std::time::SystemTime;

pub use gate::gate;
pub use history::history;
pub use seed::seed;

use crate::edgar::Unretrieved;
use crate::egress::{Egress, Transport};
use crate::filing::History;
use crate::ledger::{FilerKey, FilerLedger, Pass, Record, Ruleset, Step, Unkept, Verdict, When};

/// What the date is, to a record that has to say when it was made.
///
/// A parameter for the reason [`crate::Clock`] is one: a record that stamped
/// itself could only be checked by waiting for the time it stamped. It is not
/// that clock, and the two are apart on purpose. Pacing is an interval between
/// two requests and wants a reading that never goes backward; a verdict is a
/// date somebody reads off a journal in December, and the machine's idea of it
/// is what they will be reading.
pub trait Calendar {
    /// What the date is now.
    fn now(&self) -> SystemTime;
}

/// The machine's calendar, and the one every pass uses that is not a test.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemCalendar;

impl Calendar for SystemCalendar {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// One pass of the funnel: which sweep its verdicts belong to, and what stamps
/// them.
///
/// A pass over the seed set is long enough to be interrupted part way, and the
/// number is what tells the verdicts of an interrupted sweep from those of the
/// last complete one without anybody reconstructing it from clocks.
pub struct Sweep {
    pass: Pass,
    calendar: Box<dyn Calendar>,
}

impl Sweep {
    /// The pass numbered `pass`, dated by `calendar`.
    pub fn new(pass: Pass, calendar: impl Calendar + 'static) -> Self {
        Self {
            pass,
            calendar: Box::new(calendar),
        }
    }

    /// Which sweep this is.
    pub fn pass(&self) -> Pass {
        self.pass
    }

    /// A judgement made now, under `rules`.
    fn now(&self, rules: Ruleset) -> When {
        When::new(self.calendar.now(), self.pass, rules)
    }
}

/// Why a step of the funnel did not run to the end.
///
/// Neither of these is a verdict about anybody. A pass that stopped has decided
/// nothing about the filers it never reached, and what is on record for them is
/// what was on record before it started.
#[derive(Debug)]
pub enum Unswept {
    /// The document a step reads its filers out of did not arrive, so there was
    /// no set for it to judge.
    Unpublished {
        /// What stopped it.
        why: Unretrieved,
    },
    /// A verdict did not go on record, or what is on record did not read back.
    ///
    /// The pass stops here rather than carrying on unrecorded. Every step after
    /// this one is derived from what the ledger holds, so a step that went on
    /// past a verdict it could not keep would be handing the next one a filer
    /// nothing was ever written down about — which is the one outcome this
    /// funnel is built not to have.
    Unkept {
        /// What stopped it.
        why: Unkept,
    },
}

impl fmt::Display for Unswept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unswept::Unpublished { why } => {
                write!(f, "the pass has no filers to judge, because {why}")
            }
            Unswept::Unkept { why } => write!(f, "the pass stopped, because {why}"),
        }
    }
}

impl std::error::Error for Unswept {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Unswept::Unpublished { why } => Some(why),
            Unswept::Unkept { why } => Some(why),
        }
    }
}

/// One pass of the funnel, end to end: the filers EDGAR publishes, then the
/// ones its metadata does not rule out, then a history for those and for nobody
/// else.
///
/// Every step is handed the same thing — the filers the pass considered — and
/// each reads out of the ledger which of them its own step is about. Nothing
/// travels between the steps here, so there is no list for a step to be given
/// that the record does not agree with.
///
/// What comes back is that same set, in the order EDGAR published them. What the
/// pass decided about each is the ledger's to answer — [`admitted`] over
/// [`Step::History`] is who came out the far end — and the histories themselves
/// went to `retrieved` as they arrived. A pass hands back the question it asked,
/// because the answer is on record and a second copy of it would eventually
/// disagree.
///
/// A pass that stops stops where it stopped. The verdicts it completed are on
/// record and stay there; the filers it never reached have whatever they had
/// before it started, which for most of them is nothing, and that is what an
/// interrupted sweep is supposed to look like. Which sweep any of it belongs to
/// is `sweep`, on every verdict, so the record of a pass that died part way is
/// told from the last complete one by reading it rather than by dating it.
pub fn run<T: Transport, L: FilerLedger>(
    edgar: &mut Egress<T>,
    ledger: &mut L,
    sweep: &Sweep,
    retrieved: impl FnMut(History),
) -> Result<Vec<FilerKey>, Unswept> {
    let considered = seed(edgar, ledger, sweep)?;
    gate(edgar, ledger, sweep, &considered)?;
    history(edgar, ledger, sweep, &considered, retrieved)?;

    Ok(considered)
}

/// The filers `step` admitted, out of the verdicts `ledger` holds for
/// `considered`.
///
/// This is what a step hands the next one, and it is derived rather than
/// collected. A step records what it decided and then asks the ledger who is
/// left; nothing is carried alongside the records. So a verdict that did not
/// land takes its filer out of the answer, and a filer with no record at all is
/// simply not in it — which is what makes reaching the next step without a
/// record impossible rather than merely discouraged.
///
/// The last verdict on record for a step is the one that counts. A filer
/// rejected in August and admitted in December is two judgements that were both
/// made, and the ledger keeps both; where that filer stands now is the second.
pub fn admitted<L: FilerLedger>(
    ledger: &L,
    step: Step,
    considered: &[FilerKey],
) -> Result<Vec<Record>, Unswept> {
    let mut admitted = Vec::new();

    for key in considered {
        let verdicts = ledger
            .verdicts(key)
            .map_err(|why| Unswept::Unkept { why })?;
        let latest = verdicts
            .into_iter()
            .rev()
            .find(|record| record.step() == step);

        if let Some(record) = latest
            && matches!(record.verdict(), Verdict::Admitted(_))
        {
            admitted.push(record);
        }
    }

    Ok(admitted)
}
