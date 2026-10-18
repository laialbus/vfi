//! Sustainable growth on common equity: return on common equity times
//! retention per share, at one duration, which
//! `docs/adr/common-shareholder-forms.md` fixes on the common claim.
//!
//! It is built on the two metrics and forms no term itself: each number is
//! the one that metric derives at the same row, through its own derivation,
//! and the product is taken exactly, before anything is written. Both are
//! derived and neither stops the other, so where either is absent its reasons
//! are given as that metric gives them, `return_on_common_equity`'s first. It
//! asks no condition of its own and has no total form.
//!
//! Source: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
//! *International Financial Statement Analysis*, 3rd ed., CFA Institute
//! Investment Series, Wiley, 2015, the chapter "Financial Analysis
//! Techniques", not read from the workspace; and Pinto, J. E., Henry, E.,
//! Robinson, T. R., and Stowe, J. D., *Equity Asset Valuation*, 3rd ed., CFA
//! Institute Investment Series, Wiley, 2015, the chapter "Discounted Dividend
//! Valuation", not read from the workspace. Beside them, the CFA Institute's
//! *CFA Program Level II Financial Ratio List*,
//! <https://www.cfainstitute.org/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
//! item 37, "Sustainable growth rate = Retention rate × Return on equity".
//! The list does not say which claim the return is on; the record takes the
//! common one, the claim the retention is kept for.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::common_shareholder::ZERO_COMMON_EQUITY;
use super::dividend_payout_per_share::ZERO_EARNINGS_PER_SHARE_DILUTED;
use super::retention_per_share::RETENTION_PER_SHARE;
use super::return_on_common_equity::RETURN_ON_COMMON_EQUITY;
use super::{Computing, Derivation, Entry, Reading, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const SUSTAINABLE_GROWTH_ON_COMMON_EQUITY: Derivation = Derivation {
    entry: Entry {
        name: "sustainable_growth_on_common_equity",
        meaning: "Return on common equity, income available to common, net income less \
                  preferred dividends, over the duration divided by the average of common \
                  equity, shareholders' equity less preferred equity, at the instants it opens \
                  and closes at, times retention per share, one less dividends declared per \
                  share over diluted earnings per share for the duration: the rate the common \
                  owners' claim can grow at from the part of each common share's earnings kept, \
                  at the return earned on that claim.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::NetIncome,
            Concept::PreferredDividends,
            Concept::ShareholdersEquity,
            Concept::PreferredEquity,
            Concept::DividendsDeclaredPerShare,
            Concept::EarningsPerShareDiluted,
        ],
        conditions: &[
            ZERO_COMMON_EQUITY,
            START_NOT_A_DATE,
            ZERO_EARNINGS_PER_SHARE_DILUTED,
        ],
    },
    readings: &[
        None,
        None,
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        None,
        None,
    ],
    derived: sustainable_growth_on_common_equity,
};

fn sustainable_growth_on_common_equity(computing: &Computing) -> Result<Number, Reasons> {
    match (
        (RETURN_ON_COMMON_EQUITY.derived)(computing),
        (RETENTION_PER_SHARE.derived)(computing),
    ) {
        (Ok(return_on_common_equity), Ok(retention_per_share)) => {
            Ok(return_on_common_equity.times(&retention_per_share))
        }
        (return_on_common_equity, retention_per_share) => {
            let mut reasons = [return_on_common_equity.err(), retention_per_share.err()]
                .into_iter()
                .flatten()
                .flat_map(|reasons| reasons.each().cloned().collect::<Vec<_>>());
            let first = reasons.next().expect("one of the two metrics is absent");
            Err(Reasons::of(first, reasons.collect()))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::SUSTAINABLE_GROWTH_ON_COMMON_EQUITY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::retention_per_share::RETENTION_PER_SHARE;
    use crate::metrics::return_on_common_equity::RETURN_ON_COMMON_EQUITY;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &SUSTAINABLE_GROWTH_ON_COMMON_EQUITY.entry;
        assert_eq!(entry.name, "sustainable_growth_on_common_equity");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for words in [
            "Return on common equity",
            "retention per share",
            "net income less preferred dividends",
            "shareholders' equity less preferred equity",
            "each common share's",
        ] {
            assert!(entry.meaning.contains(words), "{words}");
        }
        assert_eq!(
            entry.inputs,
            [
                Concept::NetIncome,
                Concept::PreferredDividends,
                Concept::ShareholdersEquity,
                Concept::PreferredEquity,
                Concept::DividendsDeclaredPerShare,
                Concept::EarningsPerShareDiluted
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "zero_common_equity",
                "start_not_a_date",
                "zero_earnings_per_share_diluted",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }

    #[test]
    fn its_inputs_readings_and_conditions_are_the_two_metrics_in_order() {
        let built_on = [RETURN_ON_COMMON_EQUITY, RETENTION_PER_SHARE];
        let derivation = &SUSTAINABLE_GROWTH_ON_COMMON_EQUITY;
        assert_eq!(
            derivation.entry.inputs,
            built_on
                .iter()
                .flat_map(|metric| metric.entry.inputs)
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            derivation.readings,
            built_on
                .iter()
                .flat_map(|metric| metric.readings)
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            derivation.entry.conditions,
            built_on
                .iter()
                .flat_map(|metric| metric.entry.conditions)
                .copied()
                .collect::<Vec<_>>()
        );
    }
}
