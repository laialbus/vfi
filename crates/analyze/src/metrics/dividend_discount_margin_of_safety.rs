//! The dividend discount margin of safety: the dividend discount value per
//! share less the unadjusted close at the valuation date, over that value. It
//! is stated over a year, built on [`DIVIDEND_DISCOUNT_VALUE_PER_SHARE`], and
//! every year's margin is that year's value against the one close. It carries
//! no threshold.
//!
//! The form, the value less the close over the value, is
//! `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s decision, which
//! `docs/adr/dividend-discount-valuation-and-margin-of-safety.md` adopts by
//! name: over the value, the margin is the share of the estimate that can be
//! lost before the purchase is no bargain. It is written once, beside the
//! discounted cash flow margin, and reached from there. The term is from
//! Graham, B., *The Intelligent Investor*, the chapter "Margin of Safety as
//! the Central Concept of Investment", not read from the workspace, cited for
//! the concept and not for the formula.
//!
//! The close is put beside the value only where the share-basis rule holds,
//! whose comparisons and condition are `docs/adr/share-basis-rule-dates.md`'s.
//! The date the rule is asked at is the dividend discount record's: E is the
//! end of the year computed at. That record names the exposure this leaves
//! open, a split inside the window before that year's end, which sets the
//! window's years on different share bases and which the rule at that E does
//! not see. `tests/dividend_discount_margin_of_safety.rs` pins it without
//! closing it.
//!
//! The value and the price read are both made and neither stops the other,
//! so a filer with no price still has the value and the reason this is
//! missing. Where the value is absent its reasons are given as it gives them,
//! and the price read's one reason follows. Where the value is a figure it is
//! above zero, so no condition is asked of it.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;

use super::discounted_cash_flow_margin_of_safety::{NON_POSITIVE_CLOSE, margin_of_safety};
use super::dividend_coverage_per_share::ZERO_DIVIDENDS_DECLARED_PER_SHARE;
use super::dividend_discount_value_per_share::{
    DIVIDEND_DISCOUNT_VALUE_PER_SHARE, REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE,
};
use super::dividend_streak::NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE;
use super::reading::price::{SHARE_BASIS_DATE_NOT_A_DATE, beside_the_close};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY: Derivation = Derivation {
    entry: Entry {
        name: "dividend_discount_margin_of_safety",
        meaning: "The dividend discount value per share, less the unadjusted close at the \
                  valuation date, over that value: the fraction by which the value could be too \
                  high before the price stops being below it. The value is the mean of the \
                  dividends declared per common share, regular and special together, over the \
                  dividend_discount_base_years years ending at this year, grown once at \
                  dividend_discount_growth_rate, over dividend_discount_required_return less \
                  that rate.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: DIVIDEND_DISCOUNT_VALUE_PER_SHARE.entry.inputs,
        conditions: &[
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
            ZERO_DIVIDENDS_DECLARED_PER_SHARE,
            REQUIRED_RETURN_NOT_ABOVE_GROWTH_RATE,
            NON_POSITIVE_CLOSE,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: DIVIDEND_DISCOUNT_VALUE_PER_SHARE.readings,
    derived: dividend_discount_margin_of_safety,
};

/// The year test, so that a year's price is the only one read, then the value
/// and the price read side by side, then the form and its one condition.
fn dividend_discount_margin_of_safety(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let (value, close) = beside_the_close(
        (DIVIDEND_DISCOUNT_VALUE_PER_SHARE.derived)(computing),
        computing.close_at_valuation_date(),
    )?;
    if value.sign() != Ordering::Greater {
        unreachable!("the value is above zero wherever it is a figure");
    }
    margin_of_safety(&value, &close)
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::dividend_discount_value_per_share::DIVIDEND_DISCOUNT_VALUE_PER_SHARE;
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY.entry;
        assert_eq!(entry.name, "dividend_discount_margin_of_safety");
        assert!(!entry.name.contains("per_share"));
        assert_eq!(
            entry.meaning,
            "The dividend discount value per share, less the unadjusted close at the valuation \
             date, over that value: the fraction by which the value could be too high before the \
             price stops being below it. The value is the mean of the dividends declared per \
             common share, regular and special together, over the dividend_discount_base_years \
             years ending at this year, grown once at dividend_discount_growth_rate, over \
             dividend_discount_required_return less that rate."
        );
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "negative_dividends_declared_per_share",
                "zero_dividends_declared_per_share",
                "required_return_not_above_growth_rate",
                "non_positive_close",
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "share_basis_date_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }

    #[test]
    fn its_inputs_and_readings_are_the_values() {
        let margin = &DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY;
        let value = &DIVIDEND_DISCOUNT_VALUE_PER_SHARE;
        assert_eq!(margin.entry.inputs, [Concept::DividendsDeclaredPerShare]);
        assert_eq!(margin.entry.inputs, value.entry.inputs);
        assert_eq!(margin.readings, value.readings);
    }

    #[test]
    fn the_catalogue_keeps_the_two_entries_apart_where_the_record_does() {
        let margin = &DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY.entry;
        let value = &DIVIDEND_DISCOUNT_VALUE_PER_SHARE.entry;
        let lists = |conditions: &[&str], condition: &str| conditions.contains(&condition);
        for listed in ["share_basis_date_not_a_date", "non_positive_close"] {
            assert!(lists(margin.conditions, listed), "{listed}");
        }
        assert!(!lists(margin.conditions, "non_positive_value_per_share"));
        for unlisted in [
            "share_basis_date_not_a_date",
            "non_positive_close",
            "non_positive_value_per_share",
        ] {
            assert!(!lists(value.conditions, unlisted), "{unlisted}");
        }
        for entry in [margin, value] {
            assert!(!entry.name.contains("common"), "{}", entry.name);
            for unread in [Concept::DividendsPaid, Concept::SharesOutstanding] {
                assert!(!entry.inputs.contains(&unread), "{} {unread:?}", entry.name);
            }
        }
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| {
                derivation.entry.name == DIVIDEND_DISCOUNT_MARGIN_OF_SAFETY.entry.name
            })
            .count();
        assert_eq!(named, 1);
    }
}
