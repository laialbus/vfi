//! What one concept comes to at one period, over every filing that answers it.
//!
//! [`crate::filings`] runs the attempt inside each answering filing and stops.
//! This takes those attempts and says which of them stands. Two accepted
//! records fix it, and each clause below is theirs, transcribed rather than
//! decided here.
//!
//! **Applicability first.** A concept the filer's kind excludes comes to the
//! correct absence [`crate::applicability`] answers, with no filing consulted
//! and no attempt weighed. `contracts/canonical-concepts/v2.toml` makes the
//! state "a function of the kind", and a filer with no kind established
//! reaches it nowhere, as that module has it.
//!
//! **The silence reading is read over the period.**
//! `docs/adr/silence-beside-a-read-figure.md`: the vocabulary's zero — outright
//! for `short_term_investments`, on its pair condition for the two dividend
//! concepts — "is supplied only where no filing answering the period reached
//! the concept: where in none of them did an assertion settle it or candidate
//! choice's first step find a candidate for it, and, for a conditional reading,
//! where none reached the concept its condition names either. Where one did,
//! each answering filing whose attempt found nothing comes to `Unknown`,
//! carrying what it attempted, and Rule 3 as accepted takes it from there."
//! What crosses between filings is that one bit per attempt,
//! [`Attempted::reached`]: never the figure, `filed`, `form`, the exactness of
//! a rule, how many facts a filing carries, or whether the attempt settled. So
//! every answering filing's first step has run before any of them keeps a
//! zero, whatever order the filings arrive in.
//!
//! **The zero is reached only at a period of the concept's own shape.**
//! `docs/adr/silence-zero-only-at-the-concepts-own-shape.md`: an instant for a
//! balance, a duration for a flow, "and at the other shape the concept is
//! `Unknown`, carrying each answering filing's attempt as any other `Unknown`
//! does". The condition is one more on when the vocabulary supplies its zero,
//! made where the other two are made, so the reading over the period and the
//! reading inside one filing cannot reach a zero the other refuses.
//!
//! **The zero is supplied once, and never enters the contest.**
//! `docs/adr/silence-zero-supplied-once-per-period.md`: where the reading above
//! supplies it, the vocabulary supplies it once for the period. It carries its
//! reading and the registry version, names no filing, and has no `filed` and no
//! `form`, so there is nothing for Rule 3 to order and Rule 3 does not run. Two
//! answering filings both silent come to that zero exactly as one silent filing
//! does. Where the reading supplies no zero, each filing's own is withheld and
//! Rule 3 runs over the values an attempt read or an assertion set, which is
//! also the only company a tie is reached among.
//!
//! **Rule 3.** `docs/adr/which-filing-sets-the-value.md`, which supersedes Rule
//! 3 of `docs/adr/period-alignment.md` and nothing else of it: among the
//! answering filings whose attempt produced a `Value`, "the value that stands is
//! the one produced by the attempt in the filing with the greatest `filed`. The
//! `Value` is that attempt's, and it carries what that attempt gave it. Whether
//! an earlier filing's attempt produced the same figure is not read". So the
//! rule chooses an attempt and stamps nothing onto it. An attempt that produced
//! an `Unknown` displaces nothing, whatever left it unknown. An amendment is a
//! filing, and wins for what it states because it states it later.
//!
//! **`form` is read once, to break a tie on `filed`.** Where two answering
//! filings share the greatest `filed`, "a filing whose form is another's with
//! `/A` appended supersedes it", and "any tie that survives is `Unknown`,
//! carrying both accessions" — whatever the two figures say, since a value
//! records the rule that set it, singular.
//!
//! Nothing else enters: no window, no day count, no quorum, no preference for
//! an annual report, an original or a filing's own period, and no comparison of
//! two figures. `filed` is compared as a date, which is where the fetch record
//! puts the parse. Which periods a filer has is Rule 1, [`crate::periods`],
//! which asks this.

use vfi_contracts::canonical_concepts::{Attempt, Concept, Kind, Resolution};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::answering::Admits;
use crate::applicability;
use crate::filings::{self, Attempted};
use crate::registry::Registry;
use crate::settling::{self, Settled, Value};

/// What separates a form from the one it amends.
const AMENDS: &str = "/A";

/// What a concept comes to at a period over every filing answering it: one of
/// the three states the vocabulary publishes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stands<'r, 'f> {
    /// The filer's kind is excluded from the concept, as
    /// [`crate::applicability`] built it.
    NotApplicable(Resolution),
    /// The value Rule 3 chose, as the attempt that produced it built it, or the
    /// zero the vocabulary supplied once for the period.
    Value(Value<'r, 'f>),
    /// No attempt's value stands.
    Unknown(Unsettled<'f>),
}

/// What an `Unknown` over several filings carries: what each attempt that came
/// to nothing attempted, under its filing's accession, and where Rule 3 left
/// values undecided, the filings it could not choose between.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unsettled<'f> {
    attempted: Vec<(&'f str, Attempt)>,
    undecided: Option<Undecided<'f>>,
}

impl<'f> Unsettled<'f> {
    /// Every answering filing whose attempt came to `Unknown`, with what it
    /// attempted, in the order the filings arrived in. A withheld silence zero
    /// is here, carrying what its four steps found.
    pub fn attempted(&self) -> &[(&'f str, Attempt)] {
        &self.attempted
    }

    /// Why values some filings produced did not stand, where they did not.
    pub fn undecided(&self) -> Option<&Undecided<'f>> {
        self.undecided.as_ref()
    }
}

/// The filings Rule 3 produced values in and could not choose between.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Undecided<'f> {
    /// They share the greatest `filed`, and no one of them is left by `form`.
    Tie(Vec<&'f str>),
    /// Their facts do not state one `filed` that reads as a date, so the
    /// greatest `filed` is not known. The boundary publishes no such filing;
    /// where one arrives this is the direction the crate takes for a broken
    /// boundary reached at run time — an absence, never an order invented for
    /// it.
    Undated(Vec<&'f str>),
}

/// What `concept` comes to at `period` for this filer, over every filing of
/// `facts` that answers the period.
///
/// `facts` is every fact fetch handed over for the filer. `None` where the
/// concept proceeds and no filing answers the period: no attempt ran, and an
/// `Unknown` is built from one that did.
pub fn stands<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    facts: &'f [Fact],
) -> Option<Stands<'r, 'f>> {
    standing(
        registry,
        filer,
        kind,
        concept,
        period,
        facts,
        Admits::EveryEntry,
    )
}

/// [`stands`], with no filing asked with more entries than `within` admits:
/// the question [`crate::periods`] asks, through the entries that answer the
/// period asked for.
pub(crate) fn standing<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    facts: &'f [Fact],
    within: Admits,
) -> Option<Stands<'r, 'f>> {
    if let applicability::Answer::Excluded(state) = applicability::ask(concept, kind) {
        return Some(Stands::NotApplicable(state));
    }

    let answering = filings::answering(facts, period);
    if answering.is_empty() {
        return None;
    }

    let mut attempted =
        filings::attempted_within(registry, filer, kind, concept, period, &answering, within);

    if !attempted.iter().any(Attempted::reached) {
        let supplied = settling::supplied(registry.version(), kind, concept, period, |other| {
            filings::attempted_within(registry, filer, kind, other, period, &answering, within)
                .iter()
                .any(Attempted::reached)
        });
        if let Some(zero) = supplied {
            return Some(Stands::Value(zero));
        }
    }

    for held in &mut attempted {
        held.withhold_silence();
    }
    Some(latest(&answering, attempted))
}

/// Rule 3 over the attempts, the silence reading already applied.
fn latest<'r, 'f>(
    answering: &[filings::Filing<'f>],
    attempted: Vec<Attempted<'r, 'f>>,
) -> Stands<'r, 'f> {
    let mut unknown = Vec::new();
    let mut valued = Vec::new();
    let mut undated = Vec::new();
    for (filing, held) in answering.iter().zip(attempted) {
        let accession = held.accession();
        match held.into_settled() {
            Settled::Value(value) => match filed(filing.facts()) {
                Some(on) => valued.push((on, form(filing.facts()), accession, value)),
                None => undated.push(accession),
            },
            Settled::Unknown(attempt) => unknown.push((accession, attempt)),
            Settled::NotApplicable(state) => return Stands::NotApplicable(state),
        }
    }

    if !undated.is_empty() {
        return Stands::Unknown(Unsettled {
            attempted: unknown,
            undecided: Some(Undecided::Undated(undated)),
        });
    }

    let Some(greatest) = valued.iter().map(|(on, ..)| *on).max() else {
        return Stands::Unknown(Unsettled {
            attempted: unknown,
            undecided: None,
        });
    };
    valued.retain(|(on, ..)| *on == greatest);

    let amended: Vec<bool> = valued
        .iter()
        .map(|(_, form, ..)| {
            form.is_some_and(|form| {
                valued
                    .iter()
                    .any(|(_, other, ..)| other.is_some_and(|other| amends(other, form)))
            })
        })
        .collect();
    let mut at = 0;
    valued.retain(|_| {
        at += 1;
        !amended[at - 1]
    });

    if valued.len() == 1 {
        let (.., value) = valued.remove(0);
        return Stands::Value(value);
    }
    Stands::Unknown(Unsettled {
        attempted: unknown,
        undecided: Some(Undecided::Tie(
            valued
                .into_iter()
                .map(|(_, _, accession, _)| accession)
                .collect(),
        )),
    })
}

/// Whether `form` is `amended` with `/A` appended.
fn amends(form: &str, amended: &str) -> bool {
    form.strip_suffix(AMENDS) == Some(amended)
}

/// The one `filed` every fact of a filing states, read as a date.
///
/// Every fact is read rather than the first, because the field repeats per fact
/// and nothing on the type holds the copies to one date.
fn filed(facts: &[&Fact]) -> Option<Date> {
    let (first, rest) = facts.split_first()?;
    if rest.iter().any(|fact| fact.filed != first.filed) {
        return None;
    }
    Date::read(&first.filed)
}

/// The one `form` every fact of a filing states. A filing whose facts disagree
/// on it has not said what it is, and a condition not stated is not met.
fn form<'f>(facts: &[&'f Fact]) -> Option<&'f str> {
    let (first, rest) = facts.split_first()?;
    if rest.iter().any(|fact| fact.form != first.form) {
        return None;
    }
    Some(&first.form)
}

/// A calendar date, compared as one: year, then month, then day.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    /// `YYYY-MM-DD`, the shape the boundary publishes a date in, and nothing
    /// else.
    fn read(held: &str) -> Option<Date> {
        let mut parts = held.split('-');
        let (Some(year), Some(month), Some(day), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return None;
        };
        let digits = |part: &str, count: usize| {
            part.len() == count && part.chars().all(|held| held.is_ascii_digit())
        };
        if !digits(year, 4) || !digits(month, 2) || !digits(day, 2) {
            return None;
        }
        let date = Date {
            year: year.parse().ok()?,
            month: month.parse().ok()?,
            day: day.parse().ok()?,
        };
        ((1..=12).contains(&date.month) && (1..=31).contains(&date.day)).then_some(date)
    }
}
