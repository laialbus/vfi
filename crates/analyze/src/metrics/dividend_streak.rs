//! The dividend streak: how many consecutive years, ending at the year it is
//! computed at, declared a dividend per common share above zero, counted to
//! at most `dividend_streak_years` years. What it reads, where it stops and
//! how its absence composes is
//! `docs/adr/which-duration-precedes-a-duration.md`'s.
//!
//! It counts paying years, not raising ones. A `Value` of zero at a year read
//! did not pay, however the zero was set, and how it was set is not read. A
//! wrong zero shortens a streak and never lengthens one.

use std::cmp::Ordering;
use std::ops::ControlFlow;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::reading::yearly;
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// A year read declared a negative dividend per common share, and no streak
/// of paying years is defined across it, nor any rate of growth from or to
/// it. The dividend growth rate reaches this one definition.
pub(super) const NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE: &str =
    "negative_dividends_declared_per_share";

pub(super) const DIVIDEND_STREAK: Derivation = Derivation {
    entry: Entry {
        name: "dividend_streak",
        meaning: "The number of consecutive years, ending at this year, whose dividend \
                  declared per common share is above zero, counted to at most \
                  dividend_streak_years years. It is stated over a year, and reads this year, \
                  then each year before it in turn, until a year that is not above zero or \
                  the last of those years.",
        unit: Unit::Years,
        stated_at: Shape::Duration,
        inputs: &[Concept::DividendsDeclaredPerShare],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
        ],
    },
    readings: &[None],
    derived: dividend_streak,
};

/// The years counted from the computing year back while each is above zero,
/// stopping at a year that is not, at the last year the setting admits, or at
/// the first read that gives a reason, which is then the one reason.
fn dividend_streak(computing: &Computing) -> Result<Number, Reasons> {
    let alone = |reason| Reasons::of(reason, Vec::new());
    let year = year(computing).map_err(alone)?;
    let &[dividends_declared_per_share] = DIVIDEND_STREAK.entry.inputs else {
        unreachable!("the streak reads one input")
    };
    let back = computing.settings().dividend_streak_years().years().get() - 1;
    let counted = yearly(&year, dividends_declared_per_share, back).try_fold(
        Number::zero(),
        |paying, read| match read.map(|figure| figure.sign()) {
            Err(reason) => ControlFlow::Break(Err(alone(reason))),
            Ok(Ordering::Greater) => ControlFlow::Continue(paying.plus(&Number::one())),
            Ok(Ordering::Equal) => ControlFlow::Break(Ok(paying)),
            Ok(Ordering::Less) => {
                ControlFlow::Break(Err(declined(NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE)))
            }
        },
    );
    match counted {
        ControlFlow::Continue(paying) => Ok(paying),
        ControlFlow::Break(stopped) => stopped,
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{DIVIDEND_STREAK, NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_STREAK.entry;
        assert_eq!(entry.name, "dividend_streak");
        assert_eq!(entry.unit, Unit::Years);
        assert_eq!(entry.stated_at, Shape::Duration);
        for words in [
            "over a year",
            "this year, then each year before it",
            "dividend declared per common share is above zero",
            "counted to at most dividend_streak_years years",
        ] {
            assert!(entry.meaning.contains(words), "{words}");
        }
        assert_eq!(entry.inputs, [Concept::DividendsDeclaredPerShare]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "negative_dividends_declared_per_share",
                INPUT_NOT_A_DECIMAL
            ]
        );
        assert_eq!(
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
            "negative_dividends_declared_per_share"
        );
    }
}
