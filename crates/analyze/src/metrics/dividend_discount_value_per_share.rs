//! The dividend discount value per share: the mean of the dividends declared
//! per common share over the `dividend_discount_base_years` years ending at
//! the year it is computed at, grown once at `dividend_discount_growth_rate`,
//! over `dividend_discount_required_return` less that rate. Its four steps,
//! its settings, its conditions and how its absence composes are
//! `docs/adr/dividend-discount-valuation-and-margin-of-safety.md`'s, and its
//! window `docs/adr/which-duration-precedes-a-duration.md`'s, under the
//! method version.
//!
//! It is stated over a year, so at a duration that is not one it gives the
//! year test's one reason and makes no read. At a year it reads that year and
//! each year before it through the window's last, each reached only through
//! the series of years before, each read, and each counting once. A year the
//! series does not find is a reason, and so is every read beyond it: the
//! window is never shortened to the years found and never closed up across a
//! gap. A zero inside the window is a figure and counts in the mean. The
//! growth rate is the setting alone, so the computed `dividend_growth_rate`
//! is never read, and no price is read.
//!
//! Nothing in the history marks a special dividend, which the mean carries at
//! one part in `dividend_discount_base_years` for that many years, or a share
//! split inside the window, whose years then stand on different share bases.
//! The value reads no crossing and sees neither, and each raises it.
//!
//! The origins, not read from the workspace: Gordon, M.J., *The Investment,
//! Financing and Valuation of the Corporation*, Richard D. Irwin, 1962, for
//! the form; and Williams, J.B., *The Theory of Investment Value*, 1938, for
//! the present value of dividends.
//!
//! Source of the form: Damodaran, A., *Investment Valuation*, 2nd ed., John
//! Wiley and Sons, 2002, chapter 13, "Dividend Discount Models", read from
//! the author's own copy of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch13.pdf>, 37
//! pages whose printed numbers match their positions, sha256
//! `bf8fbef1797f91f8a8cc24aeb4de12ca0d7e746d7257c946e6e3a37189ef98c1`. Page 2:
//! "The Gordon growth model relates the value of a stock to its expected
//! dividends in the next time period, the cost of equity and the expected
//! growth rate in dividends." The equation that follows is set as an object
//! the page's text does not carry: the dividend expected one year on over the
//! required return less the growth rate, its terms defined beneath it as
//! "DPS1 = Expected Dividends one year from now (next period)", "ke= Required
//! rate of return for equity investors" and "g = Growth rate in dividends
//! forever". Page 3: "as the growth rate converges on the discount rate, the
//! value goes to infinity". Page 4: "If the growth rate exceeds the cost of
//! equity, the value per share becomes negative." The same page: "the Gordon
//! growth model is best suited for firms growing at a rate comparable to or
//! lower than the nominal growth in the economy and which have well
//! established dividend payout policies that they intend to continue into the
//! future."
//!
//! Page 5 values Consolidated Edison from "Dividends per share in 2000 =
//! $2.19", "Cost of Equity = 5.4% + 0.90*4% = 9%" and an expected growth rate
//! "= (1-0.6997)(0.1163) = 3.49%". Page 6, in an equation read from the
//! rendered page, forms the dividend expected one year on as that dividend
//! grown once, over 0.09 less 0.0349, and prints the value as "$41.15". That
//! figure is not the exact quotient of the printed inputs, which begins
//! 41.1330. The growth rate before rounding, 0.03492489, gives 41.1526, which
//! rounds to the page's figure. The tests pin both.
//!
//! Beside it: Damodaran, A., *Valuation Approaches and Metrics: A Survey of
//! the Theory and Evidence*, Stern School of Business, New York University,
//! November 2006,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
//! pages, sha256
//! `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page 9:
//! "the value of a stock is the present value of dividends through infinity".
//! Page 10 sets the stable-growth equation as "Expected Dividends next
//! period" over "(Cost of equity - Expected growth rate in perpetuity", and
//! its footnote 11 names the origin of the present value: "Williams, J.B.,
//! 1938, Theory of Investment Value, Fraser Publishing company (reprint)."
//! Page 11, footnote 14, running onto page 12: "Gordon, M.J., 1962, The
//! Investment, Financing and Valuation of the Corporation, Homewood,
//! Illinois: Richard D. Irwin, Inc." Page 15: "There are two reasons why
//! dividend discount model remain widely used to value financial service
//! companies."
//!
//! No source fixes the base. The chapter's illustration takes one year's
//! dividend, and neither text states a rule for choosing a base from a
//! history. The mean over the window, the window's length and a zero at the
//! computing year as a condition are each the record's own decision:
//!
//! - The mean, because `dividends_declared_per_share` is the year's total,
//!   regular and special together, and
//!   `docs/adr/canonical-concepts-open-questions.md`, answer 6, says the
//!   valuation "must not take one period's declared total as a perpetuity".
//!   The mean does not remove a year's excess. It carries one part of it for
//!   each year of the window, where the computing year alone would carry the
//!   whole.
//! - The window's length, a setting whose domain starts at two, because one
//!   year is the base that sentence forbids.
//! - A zero at the computing year, because the mean alone would value a
//!   dividend that is not being declared as though it went on for ever, where
//!   page 4 confines the form to payout policies "that they intend to
//!   continue into the future", and because a filer that pays nothing would
//!   be valued at nothing, which is no valuation.
//!
//! The nearest passage in the chapter is page 15, of buybacks, whose ratio
//! "for any one year can be skewed by the fact that stock buybacks, unlike
//! dividends, are not smoothed out". It is said of another flow, and is cited
//! for the reasoning behind the mean only.

use std::cmp::Ordering;
use std::num::NonZeroUsize;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::dividend_coverage_per_share::ZERO_DIVIDENDS_DECLARED_PER_SHARE;
use super::dividend_streak::NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE;
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::reading::{mean, over_years};
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// The required return less the growth rate is zero or below. At equality the
/// divisor is zero, and below it the chapter's page 4 says "the value per
/// share becomes negative".
const REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE: &str = "required_return_not_above_growth_rate";

pub(super) const DIVIDEND_DISCOUNT_VALUE_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_discount_value_per_share",
        meaning: "The value per common share: the mean of the dividends declared per common \
                  share, regular and special together, over the dividend_discount_base_years \
                  years ending at this year, grown once at dividend_discount_growth_rate, over \
                  dividend_discount_required_return less that rate.",
        unit: Unit::CurrencyPerShare,
        stated_at: Shape::Duration,
        inputs: &[Concept::DividendsDeclaredPerShare],
        conditions: &[
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
            ZERO_DIVIDENDS_DECLARED_PER_SHARE,
            REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
        ],
    },
    readings: &[None],
    derived: dividend_discount_value_per_share,
};

/// The year test, then every read of the window, then the three conditions in
/// their order, then the base dividend, the dividend expected one year on,
/// and the value.
///
/// Where the conditions have passed, every figure read is at least zero and
/// the computing year's is above it, one plus the growth rate is above zero
/// by the setting's domain and the divisor is above zero, so the value is
/// above zero.
fn dividend_discount_value_per_share(computing: &Computing) -> Result<Number, Reasons> {
    let year = year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let &[dividends_declared_per_share] = DIVIDEND_DISCOUNT_VALUE_PER_SHARE.entry.inputs else {
        unreachable!("the value reads one input")
    };
    let settings = computing.settings();
    let dividends = over_years(
        &year,
        dividends_declared_per_share,
        NonZeroUsize::from(settings.dividend_discount_base_years()),
    )?;

    if dividends
        .iter()
        .any(|dividend| dividend.sign() == Ordering::Less)
    {
        return Err(declined(NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE));
    }
    let computing_year = dividends.first().expect("a series begins at its own year");
    if computing_year.sign() == Ordering::Equal {
        return Err(declined(ZERO_DIVIDENDS_DECLARED_PER_SHARE));
    }
    let growth_rate = settings.dividend_discount_growth_rate();
    let required_return_less_growth = settings
        .dividend_discount_required_return()
        .minus(growth_rate);
    if required_return_less_growth.sign() != Ordering::Greater {
        return Err(declined(REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE));
    }

    let base_dividend = mean(dividends.into_iter());
    let expected_one_year_on = base_dividend.times(&Number::one().plus(growth_rate));
    Ok(expected_one_year_on
        .divided(&required_return_less_growth)
        .expect("the required return is above the growth rate"))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{DIVIDEND_DISCOUNT_VALUE_PER_SHARE, REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_DISCOUNT_VALUE_PER_SHARE.entry;
        assert_eq!(entry.name, "dividend_discount_value_per_share");
        assert!(entry.name.contains("per_share"));
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "The value per common share: the mean of the dividends declared per common share, \
             regular and special together, over the dividend_discount_base_years years ending \
             at this year, grown once at dividend_discount_growth_rate, over \
             dividend_discount_required_return less that rate."
        );
        assert_eq!(entry.unit, Unit::CurrencyPerShare);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::DividendsDeclaredPerShare]);
        for unread in [Concept::DividendsPaid, Concept::SharesOutstanding] {
            assert!(!entry.inputs.contains(&unread), "{unread:?}");
        }
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "negative_dividends_declared_per_share",
                "zero_dividends_declared_per_share",
                "required_return_not_above_growth_rate",
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                INPUT_NOT_A_DECIMAL
            ]
        );
        for unlisted in [
            "non_positive_value_per_share",
            "share_basis_date_not_a_date",
        ] {
            assert!(entry.declined().all(|condition| condition != unlisted));
        }
        assert_eq!(
            REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE,
            "required_return_not_above_growth_rate"
        );
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| {
                derivation.entry.name == DIVIDEND_DISCOUNT_VALUE_PER_SHARE.entry.name
            })
            .count();
        assert_eq!(named, 1);
    }
}
