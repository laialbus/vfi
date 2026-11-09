//! The discounted cash flow margin of safety: the value per share less the
//! unadjusted close at the valuation date, over that value. It is stated over
//! a year, built on [`DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE`], and is the
//! first metric to read a price. Every year's margin is that year's value
//! against the one close. It carries no threshold.
//!
//! The form, the value less the close over the value, is
//! `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s own decision,
//! which no source fixes: it is the fraction by which the value could be too
//! high before the price stops being below it. The term is from Graham, B.,
//! *The Intelligent Investor*, the chapter "Margin of Safety as the Central
//! Concept of Investment", not read from the workspace, cited for the concept
//! and not for the formula.
//!
//! The close is put beside the value only where the share-basis rule holds,
//! whose comparisons and condition are `docs/adr/share-basis-rule-dates.md`'s.
//!
//! The value and the price read are both made and neither stops the other,
//! so a filer with no price still has the value and the reason this is
//! missing. Where the value is absent its reasons are given as it gives them,
//! and the price read's one reason follows.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;

use super::discounted_cash_flow_value_per_share::{
    DISCOUNT_RATE_NOT_ABOVE_TERMINAL_GROWTH, DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE,
    NON_POSITIVE_DILUTED_SHARES, NON_POSITIVE_FREE_CASH_FLOW,
};
use super::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// The value per share is zero or below. Over a negative value the form reads
/// above one for any positive close, and a zero value is a zero divisor.
const NON_POSITIVE_VALUE_PER_SHARE: &str = "non_positive_value_per_share";

/// The close is zero or below. Over a zero or negative close the form gives
/// its most flattering figures.
const NON_POSITIVE_CLOSE: &str = "non_positive_close";

pub(super) const DISCOUNTED_CASH_FLOW_MARGIN_OF_SAFETY: Derivation = Derivation {
    entry: Entry {
        name: "discounted_cash_flow_margin_of_safety",
        meaning: "The discounted cash flow value per share, less the unadjusted close at the \
                  valuation date, over that value: the fraction by which the value could be too \
                  high before the price stops being below it. The value is free cash flow, \
                  operating cash flow less capital expenditure, over a year, grown for the \
                  forecast years at the forecast growth rate and then for ever at the terminal \
                  growth rate, discounted at the discount rate, less the average of net debt at \
                  the instants the year opens and closes at, per diluted share weighted over the \
                  year; the preferred claim and non-controlling interests are not subtracted. It \
                  records the four settings it is computed under: \
                  discounted_cash_flow_discount_rate, discounted_cash_flow_growth_rate, \
                  discounted_cash_flow_years and discounted_cash_flow_terminal_growth_rate.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE.entry.inputs,
        conditions: &[
            NON_POSITIVE_FREE_CASH_FLOW,
            DISCOUNT_RATE_NOT_ABOVE_TERMINAL_GROWTH,
            NON_POSITIVE_DILUTED_SHARES,
            NON_POSITIVE_VALUE_PER_SHARE,
            NON_POSITIVE_CLOSE,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE.readings,
    derived: discounted_cash_flow_margin_of_safety,
};

/// The year test, so that a year's price is the only one read, then the value
/// and the price read side by side, then the two conditions in order.
fn discounted_cash_flow_margin_of_safety(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    match (
        (DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE.derived)(computing),
        computing.close_at_valuation_date(),
    ) {
        (Ok(value), Ok(close)) => {
            if value.sign() != Ordering::Greater {
                return Err(declined(NON_POSITIVE_VALUE_PER_SHARE));
            }
            if close.sign() != Ordering::Greater {
                return Err(declined(NON_POSITIVE_CLOSE));
            }
            Ok(value
                .minus(&close)
                .divided(&value)
                .expect("the value is above zero"))
        }
        (value, close) => {
            let mut reasons = value
                .err()
                .into_iter()
                .flat_map(|reasons| reasons.each().cloned().collect::<Vec<_>>())
                .chain(close.err());
            let first = reasons
                .next()
                .expect("the value or the price read is absent");
            Err(Reasons::of(first, reasons.collect()))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DISCOUNTED_CASH_FLOW_MARGIN_OF_SAFETY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::discounted_cash_flow_value_per_share::DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DISCOUNTED_CASH_FLOW_MARGIN_OF_SAFETY.entry;
        assert_eq!(entry.name, "discounted_cash_flow_margin_of_safety");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for words in [
            "The discounted cash flow value per share, less the unadjusted close at the \
             valuation date, over that value: the fraction by which the value could be too high \
             before the price stops being below it.",
            "free cash flow, operating cash flow less capital expenditure, over a year",
            "the preferred claim and non-controlling interests are not subtracted",
            "discounted_cash_flow_discount_rate",
            "discounted_cash_flow_growth_rate",
            "discounted_cash_flow_years",
            "discounted_cash_flow_terminal_growth_rate",
        ] {
            assert!(entry.meaning.contains(words), "{words}");
        }
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_free_cash_flow",
                "discount_rate_not_above_terminal_growth",
                "non_positive_diluted_shares",
                "non_positive_value_per_share",
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
        let margin = &DISCOUNTED_CASH_FLOW_MARGIN_OF_SAFETY;
        let value = &DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE;
        assert_eq!(margin.entry.inputs, value.entry.inputs);
        assert_eq!(margin.readings, value.readings);
    }

    #[test]
    fn the_catalogue_keeps_the_two_entries_apart_where_the_record_does() {
        let margin = &DISCOUNTED_CASH_FLOW_MARGIN_OF_SAFETY.entry;
        let value = &DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE.entry;
        assert!(margin.conditions.contains(&SHARE_BASIS_DATE_NOT_A_DATE));
        assert!(!value.conditions.contains(&SHARE_BASIS_DATE_NOT_A_DATE));
        for entry in [margin, value] {
            assert!(!entry.name.contains("common"), "{}", entry.name);
            for read in [Concept::PreferredEquity, Concept::SharesOutstanding] {
                assert!(!entry.inputs.contains(&read), "{} {read:?}", entry.name);
            }
        }
    }
}
