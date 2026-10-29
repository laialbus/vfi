//! The dividend growth rate: the compound rate per year at which the dividend
//! declared per common share grew over `dividend_growth_years` years, ending
//! at the year it is computed at. Its formula, the two years it reads, its
//! conditions and how its absence composes are
//! `docs/adr/dividend-growth-rate.md`'s, and its window
//! `docs/adr/which-duration-precedes-a-duration.md`'s, under the method
//! version.
//!
//! It reads the computing year and the base year, the year
//! `dividend_growth_years` years before it, which is reached only through the
//! series of years before. Every year between is found as a year and none is
//! read, so an excess, a gap in a figure or a suspension there moves nothing.
//! An excess at either end moves the rate, up at the computing year and down
//! at the base year, and nothing marks it. A zero at the computing year is the
//! figure `-1`: a dividend that fell to nothing fell by all of itself.
//!
//! Source: Damodaran, A., *Investment Valuation*, 2nd ed., John Wiley and
//! Sons, 2002, chapter 11, "Estimating Growth", read from the author's own
//! copy of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch11.pdf>, sha256
//! `df01bdb8fc7706c1038c62b04aaebb7878f6b66acca85eeef3c8297020a44bf2`. Page 3:
//! "the geometric mean takes into account the compounding that occurs from
//! period to period". The geometric average that sentence introduces is set
//! as a fraction with a superscript: earnings now over earnings n years ago,
//! to the power one over n, less one. The same page: "The geometric average is
//! a much more accurate measure of true growth in past earnings, especially
//! when year-to-year growth has been erratic." Page 4: "The geometric mean
//! considers compounding but focuses on the first and the last earnings
//! observations in the series - it ignores the information in the
//! intermediate observations and any trend in growth rates that may have
//! developed over the period." Page 7: "If the EPS in the initial time period
//! is negative or zero, the geometric mean is not meaningful."
//!
//! The source states the form for earnings, and says on page 3 that it "also
//! applies to revenues". It does not state it for dividends. Applying it to
//! `dividends_declared_per_share` is the record's own decision: a dividend per
//! share is a per-share flow stated once a year, like the earnings per share
//! the source reads, and what page 4 counts as the form's cost, reading the
//! first and the last alone, is what keeps an excess between from moving it.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::dividend_streak::NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE;
use super::reading::first_and_last;
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// The base year declared no dividend per common share, and no rate of growth
/// from nothing is defined. It names a zero at the year
/// `dividend_growth_years` years before the year stated at, and not at that
/// year.
const ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE: &str =
    "zero_base_year_dividends_declared_per_share";

pub(super) const DIVIDEND_GROWTH_RATE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_growth_rate",
        meaning: "The compound rate per year at which the dividend declared per common share \
                  grew over dividend_growth_years years, ending at this year: this year's \
                  figure divided by the figure of the year dividend_growth_years years before \
                  it, to the root of dividend_growth_years, less one. It is stated over a \
                  year, and reads this year and that year, finding each year between and \
                  reading none of them.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::DividendsDeclaredPerShare],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
            ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE,
        ],
    },
    readings: &[None],
    derived: dividend_growth_rate,
};

/// The computing year's figure over the base year's, to the root of
/// `dividend_growth_years`, less one; or the year test's one reason, the
/// reasons of the two reads, or the first condition that holds of the two
/// figures read.
///
/// The conditions are asked before the root, so the root is never handed a
/// negative radicand, and the setting's domain gives it no zero index.
fn dividend_growth_rate(computing: &Computing) -> Result<Number, Reasons> {
    let year = year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let &[dividends_declared_per_share] = DIVIDEND_GROWTH_RATE.entry.inputs else {
        unreachable!("the rate reads one input")
    };
    let years = computing.settings().dividend_growth_years();
    let back = usize::try_from(years.get()).expect("the setting's every value fits a usize");
    let (computing_year, base_year) = first_and_last(&year, dividends_declared_per_share, back)?;
    if [&computing_year, &base_year]
        .into_iter()
        .any(|figure| figure.sign() == Ordering::Less)
    {
        return Err(declined(NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE));
    }
    if base_year.sign() == Ordering::Equal {
        return Err(declined(ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE));
    }
    let growth = computing_year
        .divided(&base_year)
        .expect("the base year's figure is above zero")
        .root(years.get())
        .expect("the quotient is not below zero and the index is positive");
    Ok(growth.minus(&Number::one()))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{DIVIDEND_GROWTH_RATE, ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_GROWTH_RATE.entry;
        assert_eq!(entry.name, "dividend_growth_rate");
        assert!(!entry.name.contains("common"));
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for words in [
            "compound rate per year",
            "dividend declared per common share",
            "over dividend_growth_years years, ending at this year",
            "this year's figure divided by the figure of the year dividend_growth_years \
             years before it",
            "to the root of dividend_growth_years, less one",
            "over a year",
            "reads this year and that year, finding each year between and reading none",
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
                "zero_base_year_dividends_declared_per_share",
                INPUT_NOT_A_DECIMAL
            ]
        );
        assert_eq!(
            ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE,
            "zero_base_year_dividends_declared_per_share"
        );
        assert_ne!(
            ZERO_BASE_YEAR_DIVIDENDS_DECLARED_PER_SHARE,
            "zero_dividends_declared_per_share"
        );
    }
}
