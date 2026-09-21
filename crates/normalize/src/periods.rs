//! Which canonical periods a filer has.
//!
//! `docs/adr/period-alignment.md` is accepted and this is its Rule 1, whole —
//! the record superseding its Rule 3 leaves this one as it stands. "A canonical
//! period is a period a fact of this filer carries, as the contract publishes
//! it: one date for an instant, two for a duration. Nothing is constructed,
//! nothing is rounded to a calendar, nothing is synthesised." And: "Of those, a
//! period is the filer's when at least one concept its kind admits resolves to
//! a `Value` at it, through entries that answer the period asked for."
//!
//! **The candidates are the periods the facts carry,** compared as the
//! characters the boundary published, so an instant and a duration are never
//! one period. No quarter the filer never reported, no fourth quarter built
//! from a year less nine months, and nothing read off a length, an ordinal or
//! a fiscal label, none of which the boundary carries.
//!
//! **What admits a candidate is the value that stands over every filing
//! answering it,** which is Rule 3's answer in [`crate::standing`] and not one
//! filing's attempt. Nothing here orders filings, parses `filed` or reads a
//! form.
//!
//! **Only an entry that answers the period asked for is asked, and only a value
//! set through one counts** — read from a fact, or asserted. An entry that
//! answers the period of the filing it was reported in borrows a period named
//! elsewhere, so what it resolves is no evidence that this one exists: on the
//! merged fixture every one of the filer's ten cover dates is the `filed` of a
//! filing and no statement of it covers one. A `Value` the silence reading
//! supplied is set through no entry and makes no period exist either, which is
//! the owner's ruling on the 2026-09-18 M4-36 escalation, and what lets
//! `docs/adr/what-normalize-emits.md` count this filer's periods beside the
//! silence zeros rendered at them.
//!
//! **Nothing is dropped for being implausible.** A filer's own dates are its
//! periods where something resolves at them, and this ruleset has no evidence
//! with which to decide that one of them is wrong.
//!
//! The question is asked concept-first: the registry is never asked about a
//! tag.

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::answering::Admits;
use crate::applicability;
use crate::registry::{Answers, Registry};
use crate::settling::SetBy;
use crate::standing::{self, Stands};

/// Every canonical period this filer has, each once.
///
/// `facts` is every fact fetch handed over for the filer. The order the set is
/// handed over in carries no meaning.
///
/// A filer with no kind established has none: no concept is one its kind
/// admits, so nothing can make a period its, whatever [`applicability::ask`]
/// lets such a filer proceed on elsewhere.
pub fn periods<'f>(
    registry: &Registry,
    filer: &str,
    kind: Option<Kind>,
    facts: &'f [Fact],
) -> Vec<&'f Period> {
    let Some(kind) = kind else {
        return Vec::new();
    };

    let mut candidates: Vec<&'f Period> = Vec::new();
    for fact in facts {
        if !candidates.contains(&&fact.period) {
            candidates.push(&fact.period);
        }
    }

    candidates.retain(|period| {
        Concept::ALL
            .iter()
            .any(|concept| resolves(registry, filer, kind, *concept, period, facts))
    });
    candidates
}

/// Whether `concept`, which `kind` admits, stands at `period` as a `Value` set
/// through an entry that answers the period asked for.
fn resolves(
    registry: &Registry,
    filer: &str,
    kind: Kind,
    concept: Concept,
    period: &Period,
    facts: &[Fact],
) -> bool {
    if applicability::ask(concept, Some(kind)) != applicability::Answer::Proceeds {
        return false;
    }
    let Some(Stands::Value(value)) = standing::standing(
        registry,
        filer,
        Some(kind),
        concept,
        period,
        facts,
        Admits::OnlyThePeriodAskedFor,
    ) else {
        return false;
    };
    match value.set_by() {
        SetBy::Rule { rule, .. } => rule.answers() == Answers::PeriodAskedFor,
        SetBy::Assertion { .. } => true,
        SetBy::Silence { .. } => false,
    }
}
