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
//! **The cover-page entry is refused.** The record admits an entry that answers
//! the period of the filing it was reported in on one condition — "only when
//! that filing's own period of report ends on the period asked for" — and then
//! says what this boundary does with it: "the condition cannot be stated, so it
//! is never met and such an entry is never a candidate." Nothing published says
//! which period a filing reports, so [`ADMITS`] is what every filing here is
//! asked with. What admitting it unchecked would cost is measured on the same
//! fixture: `shares_outstanding` at 2023-12-31 handed a count from a cover page
//! nineteen months later, 60,500,000 against the filer's own 60,000,000 either
//! side of it. The absence that stands in its place is one M5 shows with its
//! reason, and the two are not mistakable for each other.
//!
//! **Nothing here ranks and nothing here is dropped.** Which of several
//! answering filings sets the value is the record's Rule 3, and the record
//! states it twice in ways that do not agree — the rule takes the filing with
//! the greatest `filed`, while the restatement fixture the same record asks for
//! expects the values that did not change to go on naming the earlier filing.
//! That is a question for the decider, so nothing here orders two filings,
//! parses `filed`, or reads a form. Which periods a filer has is Rule 1, which
//! asks that settled answer and waits on the same ruling.

use std::collections::BTreeMap;

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::answering::Admits;
use crate::registry::Registry;
use crate::settling::{self, Settled};

/// What a filing is asked with at `fetch-normalize` v1: every entry the
/// registry states for the concept except one that answers the period of the
/// filing it was reported in.
///
/// The record's condition for admitting that one is a statement about the
/// filing's own period of report, which is the field this boundary withholds. A
/// condition that cannot be stated is never met, so the entry is refused here
/// rather than admitted on nothing.
const ADMITS: Admits = Admits::OnlyThePeriodAskedFor;

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
}

/// What a concept came to inside one answering filing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attempted<'r, 'f> {
    accession: &'f str,
    settled: Settled<'r, 'f>,
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
/// which is the whole of what turns a period and a filing into a value or an
/// absence. Nothing is compared between two of them here.
pub fn attempted<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    answering: &[Filing<'f>],
) -> Vec<Attempted<'r, 'f>> {
    let mut attempted = Vec::with_capacity(answering.len());
    for filing in answering {
        attempted.push(Attempted {
            accession: filing.accession,
            settled: settling::settle(
                registry,
                filer,
                kind,
                concept,
                period,
                filing.facts(),
                ADMITS,
            ),
        });
    }
    attempted
}
