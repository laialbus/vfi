//! Return on common equity: income available to common over a duration as a
//! fraction of the average of common equity at the instants the duration
//! opens and closes at.
//!
//! Net income and preferred dividends are flows `canonical-concepts` v2
//! states at a duration, and shareholders' equity and preferred equity
//! balances it states at an instant; all four apply to every kind of filer.
//! So the metric is stated at the duration, reads the flows at its row, and
//! reads the balances averaged, which
//! `docs/adr/which-other-period-a-metric-reads.md` fixes for return on common
//! equity in its table. Both terms are formed in [`super::common_shareholder`],
//! common equity at each instant before the two are averaged. A preferred
//! concept that is `Unknown` is an absence and is never read as zero, and
//! `return_on_equity` stands beside this as the total form. A negative
//! average common equity is a value that gives a negative figure.
//!
//! Source: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
//! *International Financial Statement Analysis*, 3rd ed., CFA Institute
//! Investment Series, Wiley, 2015, the chapter "Financial Analysis
//! Techniques", not read from the workspace. Beside it, the CFA Institute's
//! own definition, which the list's preface makes the one to use where
//! definitions differ: *CFA Program Level II Financial Ratio List*,
//! <https://www.cfainstitute.org/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
//! item 24, "Return on common equity = (Net income – Preferred dividends) ÷
//! Average common shareholders’ equity".

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::common_shareholder::{ZERO_COMMON_EQUITY, common_equity, income_available_to_common};
use super::reading::reads;
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const RETURN_ON_COMMON_EQUITY: Derivation = Derivation {
    entry: Entry {
        name: "return_on_common_equity",
        meaning: "Income available to common, net income less preferred dividends, over the \
                  duration, divided by the average of common equity, shareholders' equity \
                  less preferred equity, at the instants it opens and closes at: the profit \
                  the business earned for its common owners for each unit of their claim \
                  while earning it.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::NetIncome,
            Concept::PreferredDividends,
            Concept::ShareholdersEquity,
            Concept::PreferredEquity,
        ],
        conditions: &[ZERO_COMMON_EQUITY, START_NOT_A_DATE],
    },
    readings: &[None, None, Some(Reading::Averaged), Some(Reading::Averaged)],
    derived: return_on_common_equity,
};

fn return_on_common_equity(computing: &Computing) -> Result<Number, Reasons> {
    let [
        net_income,
        preferred_dividends,
        shareholders_equity,
        preferred_equity,
    ] = reads(computing, &RETURN_ON_COMMON_EQUITY)?;
    income_available_to_common(&net_income, &preferred_dividends)
        .mean()
        .divided(&common_equity(&shareholders_equity, &preferred_equity).mean())
        .ok_or_else(|| declined(ZERO_COMMON_EQUITY))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::RETURN_ON_COMMON_EQUITY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &RETURN_ON_COMMON_EQUITY.entry;
        assert_eq!(entry.name, "return_on_common_equity");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for term in [
            "net income less preferred dividends",
            "shareholders' equity less preferred equity",
        ] {
            assert!(entry.meaning.contains(term), "{term}");
        }
        assert_eq!(
            entry.inputs,
            [
                Concept::NetIncome,
                Concept::PreferredDividends,
                Concept::ShareholdersEquity,
                Concept::PreferredEquity
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "zero_common_equity",
                "start_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
