//! Every metric of the method: each one's catalogue entry, defined beside it in
//! its own file, and how its number at a row is derived.
//!
//! A metric is added by adding its file here and its derivation to
//! [`DERIVATIONS`]. The name the hand-over carries it by is read from its
//! entry, and from no second list. How an absent input makes a metric absent
//! is `docs/adr/what-analyze-takes-and-returns.md`'s, and every metric reads
//! and writes through the arithmetic that
//! `docs/adr/how-analyze-computes-and-writes-a-figure.md` fixes.

use vfi_contracts::analyze_store::{Metric, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{Concept, Period, Resolution, Row};

use crate::arithmetic::{INPUT_NOT_A_DECIMAL, Number};

mod gross_margin;

/// Every metric the method computes, each once.
const DERIVATIONS: &[Derivation] = &[gross_margin::GROSS_MARGIN];

/// A metric: its entry, and its number at a row of the shape the entry is
/// stated at, or the reasons it has none.
struct Derivation {
    entry: Entry,
    derived: fn(&Row) -> Result<Number, Reasons>,
}

/// What a metric states about itself, which the catalogue the shell reads is
/// generated from. No entry states a precision: every figure is written to
/// the one `SIGNIFICANT_DIGITS`.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the catalogue that reads the rest is not built yet"
    )
)]
struct Entry {
    /// The name the hand-over carries the metric by.
    name: &'static str,
    /// What the figure is, in words.
    meaning: &'static str,
    unit: Unit,
    /// The shape of period the metric is stated at. It is computed at every
    /// period of that shape the history holds, and at no other.
    stated_at: Shape,
    /// The concepts the metric reads at the row, in the order its reasons
    /// follow.
    inputs: &'static [Concept],
    /// Each condition under which the metric's own definition is undefined at
    /// inputs that were all read.
    conditions: &'static [&'static str],
}

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "the catalogue that reads it is not built yet")
)]
impl Entry {
    /// Every `declined` condition the metric can be absent under: its own,
    /// then `INPUT_NOT_A_DECIMAL`, which every metric meets because every
    /// metric reads an amount or a close.
    fn declined(&self) -> impl Iterator<Item = &'static str> {
        self.conditions
            .iter()
            .copied()
            .chain(std::iter::once(INPUT_NOT_A_DECIMAL))
    }
}

/// The unit a figure is written in, as a scale, so that nothing presenting it
/// reads a fraction as a percentage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Unit {
    /// A ratio: `0.25` is a quarter, and never `25`.
    FractionOfOne,
}

/// One of the two shapes a period takes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    #[expect(dead_code, reason = "no metric is stated at an instant yet")]
    Instant,
    Duration,
}

impl Shape {
    fn holds(self, period: &Period) -> bool {
        matches!(
            (self, period),
            (Shape::Instant, Period::Instant { .. }) | (Shape::Duration, Period::Duration { .. })
        )
    }
}

/// Every metric stated at `row`'s period, each once, by its entry's name, with
/// its outcome there: its number written once, or the reasons it has none.
pub(crate) fn at(row: &Row) -> Vec<Metric> {
    DERIVATIONS
        .iter()
        .filter(|derivation| derivation.entry.stated_at.holds(row.period()))
        .map(|derivation| Metric {
            name: derivation.entry.name.into(),
            outcome: match (derivation.derived)(row) {
                Ok(number) => Outcome::Value {
                    figure: number.written(),
                },
                Err(reasons) => Outcome::Absent { reasons },
            },
        })
        .collect()
}

/// The number each of `inputs` states at `row`, in their order; or, where any
/// states none, a reason for each that states none, in the same order.
/// Nothing is substituted for an input.
///
/// `N` is how many numbers the metric takes, which is how many inputs its
/// entry lists.
fn read<const N: usize>(row: &Row, inputs: &[Concept]) -> Result<[Number; N], Reasons> {
    let (numbers, reasons): (Vec<_>, Vec<_>) = inputs
        .iter()
        .map(|concept| input(row, *concept))
        .partition(Result::is_ok);
    let mut reasons = reasons.into_iter().filter_map(Result::err);
    if let Some(first) = reasons.next() {
        return Err(Reasons::of(first, reasons.collect()));
    }
    let numbers: Vec<Number> = numbers.into_iter().filter_map(Result::ok).collect();
    Ok(numbers
        .try_into()
        .expect("a metric takes a number for each input its entry lists"))
}

/// The number `concept` states at `row`, or why it states none: the state it
/// is in where that is no `Value`, and `INPUT_NOT_A_DECIMAL` where its
/// characters are outside the reader's grammar.
fn input(row: &Row, concept: Concept) -> Result<Number, Reason> {
    match row.of(concept) {
        Resolution::Value { amount, .. } => {
            Number::read(amount).ok_or_else(|| Reason::declined(INPUT_NOT_A_DECIMAL.into()))
        }
        Resolution::NotApplicable { .. } => Err(Reason::input_not_applicable(row, concept)
            .expect("the concept is NotApplicable at the row")),
        Resolution::Unknown { .. } => {
            Err(Reason::input_unknown(row, concept).expect("the concept is Unknown at the row"))
        }
    }
}

/// The one reason a metric is absent where every input was read and its
/// definition is undefined at them, under `condition`, which its entry names.
fn declined(condition: &'static str) -> Reasons {
    Reasons::of(Reason::declined(condition.into()), Vec::new())
}

#[cfg(test)]
mod tests {
    use super::DERIVATIONS;

    #[test]
    fn no_two_entries_share_a_name() {
        for (at, derivation) in DERIVATIONS.iter().enumerate() {
            assert!(
                DERIVATIONS[..at]
                    .iter()
                    .all(|earlier| earlier.entry.name != derivation.entry.name),
                "{} is named twice",
                derivation.entry.name
            );
        }
    }

    #[test]
    fn every_entry_lists_an_input_and_a_meaning() {
        for derivation in DERIVATIONS {
            assert!(
                !derivation.entry.inputs.is_empty(),
                "{}",
                derivation.entry.name
            );
            assert!(
                !derivation.entry.meaning.is_empty(),
                "{}",
                derivation.entry.name
            );
        }
    }
}
