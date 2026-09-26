//! Which of a filer's filings answer a period, and what a concept comes to
//! inside each of them.
//!
//! `docs/adr/period-alignment.md` is accepted and this is its Rule 2, whole: "A
//! filing answers a period when it carries at least one fact whose own period
//! is exactly that period", and resolution "runs once per period, per concept,
//! per answering filing, over that filing's facts and no other's."
//!
//! **What makes a filing an answer.** One fact of it stated for the period
//! asked for, as the boundary publishes the dates. Nothing else is read: not
//! the form, not `filed`, not how many facts the filing carries at the period,
//! and not whether the period is the one the filing is about — which the eight
//! published fields do not say in any case. A filing quoting one comparative
//! line of a quarter answers that quarter exactly as the quarter's own report
//! does, and nothing here tells the two apart.
//!
//! **One filing at a time.** A `sum` or a `difference` composed across two
//! filings is a number no filing states, which is the objection
//! `docs/adr/candidate-choice.md` already makes to a composition mixing a
//! quarter with a year to date. So a filing's facts are the facts carrying its
//! accession, and [`crate::settling`] is asked once over those and no others.
//!
//! **Every answering filing, not the newest.** The newest is usually the
//! thinnest. On the merged fetch fixture the last filing to mention the quarter
//! 2024-10-01 to 2024-12-31 carries two facts at it, so a rule that read that
//! filing alone would resolve `net_income` for the quarter and report revenue,
//! gross profit, tax and cash flow absent for a quarter the filer published in
//! full.
//!
//! **The cover-page entry is admitted on one condition and no other.** The
//! record admits an entry that answers the period of the filing it was reported
//! in "only when that filing's own period of report ends on the period asked
//! for", and `fetch-normalize` v2 carries that period's end on every fact as
//! `report_period_end`. So a filing is asked with every entry where the period
//! asked for is an instant and each of the filing's facts publishes that
//! instant's date as its period of report's end, character for character. What
//! admitting it unchecked would cost is measured on the same fixture:
//! `shares_outstanding` at 2023-12-31 handed a count from a cover page nineteen
//! months later, 60,500,000 against the filer's own 60,000,000 either side of
//! it. No filing's period of report ends on that day, so none is asked with the
//! entry there.
//!
//! The period asked for has to be an instant. The condition compares a date
//! with it, and the concept the entry reaches is a balance, which
//! `docs/adr/candidate-choice.md` answers only by an instant: a duration ending
//! on the report date admits the entry in no filing. An empty
//! `report_period_end` names no date, so it meets the condition nowhere, and
//! the concept is left where the silence reading puts it.
//!
//! **Nothing here ranks and nothing here is dropped.** Which of several
//! answering filings sets the value is Rule 3, and it is [`crate::standing`]'s:
//! nothing here orders two filings, parses `filed`, or reads a form. Which
//! periods a filer has is Rule 1, [`crate::periods`], which asks that settled
//! answer.

use std::collections::BTreeMap;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Declined, Kind};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::answering::Admits;
use crate::registry::Registry;
use crate::settling::{self, Settled};

/// One filing that answers a period, and the facts of it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Filing<'f> {
    accession: &'f str,
    facts: Vec<&'f Fact>,
}

impl<'f> Filing<'f> {
    /// The accession of the filing, which is what makes its facts its own.
    pub fn accession(&self) -> &'f str {
        self.accession
    }

    /// Every fact carrying that accession, in the order the boundary handed
    /// them over — all of them, not only the ones stated for the period.
    ///
    /// A composition reads each of its operands at the period asked for, and an
    /// entry that answers a period other than its own reads a fact stated for
    /// another, so what a filing answers with is the filing.
    pub fn facts(&self) -> &[&'f Fact] {
        &self.facts
    }

    /// Which entries this filing is asked with at `period`.
    ///
    /// Every fact is read rather than the first, because the field repeats per
    /// fact and nothing on the type holds the copies to one date. A filing whose
    /// facts disagree on where its period of report ends has not said where it
    /// ends, and a condition not stated is not met.
    fn admits(&self, period: &Period) -> Admits {
        let Period::Instant { at } = period else {
            return Admits::OnlyThePeriodAskedFor;
        };
        if !at.is_empty() && self.facts.iter().all(|fact| fact.report_period_end == *at) {
            Admits::EveryEntry
        } else {
            Admits::OnlyThePeriodAskedFor
        }
    }
}

/// What a concept came to inside one answering filing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attempted<'r, 'f> {
    accession: &'f str,
    settled: Settled<'r, 'f>,
    reached: bool,
    withheld: Option<Attempt>,
    declined: Vec<Declined>,
}

impl<'r, 'f> Attempted<'r, 'f> {
    /// The filing the attempt ran inside.
    pub fn accession(&self) -> &'f str {
        self.accession
    }

    /// What the concept settled to there: one of the three states the
    /// vocabulary publishes, out of this filing's facts alone.
    pub fn settled(&self) -> &Settled<'r, 'f> {
        &self.settled
    }

    /// Whether the attempt reached the concept: an assertion settled it, or
    /// candidate choice's first step found a candidate for it here. A contest
    /// that did not settle reached it; a silence, whatever it reads as, did not.
    pub fn reached(&self) -> bool {
        self.reached
    }

    /// This filing's silence zero withheld, where the reading made over the
    /// period supplies none: the filing comes to `Unknown`, carrying what its
    /// four steps found. Anything else is left as it is.
    pub(crate) fn withhold_silence(&mut self) {
        if let Some(attempted) = self.withheld.take() {
            self.settled = Settled::Unknown(attempted);
        }
    }

    /// What the attempt settled to, and what candidate choice declined on the
    /// way where a rule settled it to a value.
    pub(crate) fn into_settled(self) -> (Settled<'r, 'f>, Vec<Declined>) {
        (self.settled, self.declined)
    }
}

/// Which of the filings `facts` were reported in answer `period`.
///
/// `facts` is every fact fetch handed over for one filer. The filings come back
/// in the order their accessions sort in, which is the one order derived from
/// what identifies a filing and from nothing else: `filed`, `form` and the
/// number of facts a filing carries are unread here, and no position in this
/// list says which filing sets the value.
pub fn answering<'f>(facts: &'f [Fact], period: &Period) -> Vec<Filing<'f>> {
    let mut reported: BTreeMap<&'f str, Vec<&'f Fact>> = BTreeMap::new();
    for fact in facts {
        reported.entry(&fact.accession).or_default().push(fact);
    }

    let mut answering = Vec::new();
    for (accession, facts) in reported {
        if facts.iter().any(|fact| fact.period == *period) {
            answering.push(Filing { accession, facts });
        }
    }
    answering
}

/// What `concept` comes to at `period` inside each filing of `answering`, one
/// attempt per filing and in the order they arrived in.
///
/// Every attempt is the candidate-choice procedure over one filing's facts,
/// asked with the entries that filing's own period of report admits, which is
/// the whole of what turns a period and a filing into a value or an absence.
/// Nothing is compared between two of them here.
pub fn attempted<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    answering: &[Filing<'f>],
) -> Vec<Attempted<'r, 'f>> {
    attempted_within(
        registry,
        filer,
        kind,
        concept,
        period,
        answering,
        Admits::EveryEntry,
    )
}

/// [`attempted`], with no filing asked with more entries than `within` admits.
///
/// Rule 1 asks through the entries that answer the period asked for and no
/// other, so it narrows every filing to those; nothing else narrows.
pub(crate) fn attempted_within<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    answering: &[Filing<'f>],
    within: Admits,
) -> Vec<Attempted<'r, 'f>> {
    let mut attempted = Vec::with_capacity(answering.len());
    for filing in answering {
        let ran = settling::reaching(
            registry,
            filer,
            kind,
            concept,
            period,
            filing.facts(),
            match within {
                Admits::EveryEntry => filing.admits(period),
                Admits::OnlyThePeriodAskedFor => Admits::OnlyThePeriodAskedFor,
            },
        );
        attempted.push(Attempted {
            accession: filing.accession,
            settled: ran.settled,
            reached: ran.reached,
            withheld: ran.withheld,
            declined: ran.declined,
        });
    }
    attempted
}
