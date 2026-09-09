//! Which of one filing's facts answer the period asked for.
//!
//! `docs/adr/candidate-choice.md` gives this its own section, "Which facts can
//! answer the period asked for", ahead of the four steps that use it. The
//! separation is the record's: step 1 asks whether every fact a rule needs
//! answers the period, and everything beneath that step is a question about
//! facts and dates alone, answerable with one rule in hand and no other. So
//! this is asked of one rule at a time, and it settles nothing between two of
//! them.
//!
//! Three things decide what answers, and nothing else does.
//!
//! **The measure.** The vocabulary marks every concept a flow or a balance. A
//! flow is answered only by a duration fact and a balance only by an instant
//! one, and neither is ever built from the other: two instants are not averaged
//! into a flow, and a duration is not read at its end date as a balance. A
//! filing carrying only the other shape has not reported the concept, and there
//! is no recovery here that would not be a number no filing states.
//!
//! **The dates, compared for equality as the boundary publishes them.** A
//! duration answers when its start and its end are the period's; an instant
//! answers when it is the period's end. There is no day count, no window and no
//! tolerance. The record argues that strictness from the fixture rather than
//! from taste: one filer carries durations of 24, 90, 91, 92, 182, 183, 273,
//! 274, 365 and 366 days, so anything looser than equality has a year-to-date
//! figure or a prior-year comparative sitting next to the right answer, under
//! the same tag, in the same filing. Its 10-K carries three durations of one
//! element at once and two of them carry the same number, so the dates are the
//! only thing telling those two apart. A tolerance would be a number with no
//! source, which anchor 5 bans outright.
//!
//! **The unit.** A fact answers only in the unit the vocabulary names for the
//! concept: a count in shares, an amount per share, a currency amount in a
//! currency. Which currency is not asked and cannot be, so where one entry
//! answers one period in two of them both are named and neither is chosen.
//!
//! One entry answers a period other than its own, and the registry is what says
//! which: an entry that answers the filing it was reported in is answered by
//! its instant fact in that filing rather than at the period's end. It exists
//! for the cover-page share count, stamped with the filing's own date and in
//! the fixture never falling at a period end. What makes that one fact rather
//! than a window is the boundary's five-field identity, so no search and no
//! nearest match is written here.
//!
//! The period asked for is compared whole, in the shape the boundary publishes
//! it: a balance is answered at the instant it is asked at, and a duration
//! period is not read at its end for one. The record's sentence — "a balance's
//! instant matches when it is the period's end" — admits the looser reading,
//! and this takes the strict half of it. The alignment ruleset makes every date
//! a fact carries a period in its own right, so a balance is reachable at its
//! own instant without the looser reading; with it, one figure would sit under
//! two period keys, one of them a period the figure does not describe. Where
//! the strict reading is the wrong one the concept is absent with its reason,
//! which is the direction to be wrong in.
//!
//! Two questions are not answered here. Whether a `difference`'s concept
//! operand resolved to a `Value` is only a resolved concept's to say, so a
//! difference reports its element term and its concept operand is not in the
//! answer at all. Which of several answering rules wins is the procedure above
//! this one. Nothing ranks, nothing is dropped for how its entry reads its
//! concept, and no fifth rule form appears — a duration the filing does not
//! carry is not built by subtracting one duration from another.

use vfi_contracts::canonical_concepts::{Concept, Measure, Unit};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::registry::{Answers, Operand, Rule};

/// The unit key a count of shares crosses under, and the denominator of a
/// per-share one.
const SHARES: &str = "shares";

/// What divides the two halves of a per-share unit key, as the boundary
/// publishes it: `USD/shares`.
const PER: char = '/';

/// The letters an ISO 4217 alphabetic currency code carries. A monetary fact
/// crosses under one of those codes, and this shape is the whole of what makes
/// a unit key a currency here — the codes themselves are ISO's list, and a copy
/// of it kept in this repository would be a second source of truth for
/// something published elsewhere.
const CURRENCY_CODE: usize = 3;

/// What one filing's facts answer for one rule at one period.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answer<'r, 'f> {
    operands: Vec<Answering<'r, 'f>>,
}

/// One element operand of a rule, and every fact of the filing that answers it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answering<'r, 'f> {
    operand: &'r Operand,
    facts: Vec<&'f Fact>,
}

impl<'r, 'f> Answer<'r, 'f> {
    /// One entry per element operand the rule reads, in the order the rule
    /// states them: one for a `tag`, one per element for a `sum`, and one for
    /// the element term of a `difference`.
    ///
    /// An operand nothing answered is here as the operand it is, carrying no
    /// facts. It is never read as zero and never dropped: reading silence as
    /// zero for one component of a composition is the reading the vocabulary's
    /// own silence test refused for the concepts it matters to.
    pub fn operands(&self) -> &[Answering<'r, 'f>] {
        &self.operands
    }

    /// The element operands nothing answered, in that same order — what a
    /// composition is missing, named rather than counted.
    pub fn unanswered(&self) -> Vec<&'r Operand> {
        self.operands
            .iter()
            .filter(|answering| answering.facts.is_empty())
            .map(|answering| answering.operand)
            .collect()
    }
}

impl<'r, 'f> Answering<'r, 'f> {
    /// The element this is about, answered or not.
    pub fn operand(&self) -> &'r Operand {
        self.operand
    }

    /// Every fact under that element which answers the period, in the order the
    /// boundary handed them over.
    ///
    /// More than one is every one of them. Two facts under one entry answering
    /// one period is an outcome this reports and does not settle, because
    /// choosing between them is a choice a `Value` has no field to record.
    pub fn facts(&self) -> &[&'f Fact] {
        &self.facts
    }
}

/// Which of `filing`'s facts answer `period` for `rule`, with `concept` read as
/// the vocabulary defines it.
///
/// `filing` is the facts of one filing. Which filings answer a period, and
/// which of a filer's facts are one filing's, belong to the alignment ruleset:
/// a composition drawn across two filings is a number no filing states, and
/// nothing reaching this far could tell one from a number that is stated.
///
/// The concept arrives beside the rule because the two were asked together. The
/// registry was asked about this concept and answers about tags alone; what the
/// concept is measured in, and in what unit, is the vocabulary's to say.
pub fn ask<'r, 'f>(
    concept: Concept,
    rule: &'r Rule,
    period: &Period,
    filing: &'f [Fact],
) -> Answer<'r, 'f> {
    let read = concept.definition();
    let mut operands = Vec::with_capacity(rule.operands().len());

    for operand in rule.operands() {
        let Operand::Element { taxonomy, tag } = operand else {
            continue;
        };

        let mut facts = Vec::new();
        for fact in filing {
            if fact.taxonomy == *taxonomy
                && fact.tag == *tag
                && counts_in(&fact.unit, read.unit)
                && falls_at(&fact.period, read.measure, rule.answers(), period)
            {
                facts.push(fact);
            }
        }

        operands.push(Answering { operand, facts });
    }

    Answer { operands }
}

/// Whether a fact stated for `stated` answers `asked`, for a concept the
/// vocabulary measures as `measure`, under an entry that answers `answers`.
///
/// The dates are the characters the boundary published, compared as they are
/// written. Nothing here parses one: a parse is a reading, and two dates read
/// into a calendar could be compared by a distance, which is the thing this
/// must not have.
fn falls_at(stated: &Period, measure: Measure, answers: Answers, asked: &Period) -> bool {
    match (measure, stated) {
        (Measure::Flow, Period::Duration { .. }) => stated == asked,
        (Measure::Balance, Period::Instant { .. }) => match answers {
            Answers::PeriodAskedFor => stated == asked,
            // The cover-page shape: the entry declares that its fact answers the
            // period of the filing it was reported in, so the fact's own date is
            // not what is being asked about. The registry admits this only for a
            // concept the vocabulary measures as a balance, which is why it is
            // read inside this arm and nowhere else.
            Answers::FilingReportedIn => true,
        },
        // Neither shape is built from the other, so a fact of the wrong shape is
        // no answer however its dates fall.
        (Measure::Flow, Period::Instant { .. }) | (Measure::Balance, Period::Duration { .. }) => {
            false
        }
    }
}

/// Whether a fact published under `key` is a figure in the unit the vocabulary
/// names for the concept.
///
/// The three readings are the vocabulary's own words — "a count of shares", "an
/// amount in the filing's reporting currency at a scale of one, per common
/// share", "a single figure in the filing's reporting currency" — against the
/// keys the fetch surface publishes: `shares`, `USD/shares`, `USD`.
///
/// Which currency is not asked, and cannot be: the boundary publishes the unit
/// key and not the filing's reporting currency. What is asked is whether the key
/// is a currency at all, and that much has to be asked, because a filer states
/// units beside its money that are not money — a rate in `pure`, an area in
/// `sqm` — and a concept measured in currency must not take one of those.
fn counts_in(key: &str, unit: Unit) -> bool {
    match unit {
        Unit::Shares => key == SHARES,
        Unit::CurrencyPerShare => match key.split_once(PER) {
            Some((amount, per)) => currency(amount) && per == SHARES,
            None => false,
        },
        Unit::Currency => currency(key),
    }
}

fn currency(key: &str) -> bool {
    key.len() == CURRENCY_CODE && key.bytes().all(|letter| letter.is_ascii_uppercase())
}
