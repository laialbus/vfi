//! Piotroski's F_ACCRUAL: one where cash flow from operations over a year is
//! above net income over it, each scaled by total assets at the instant the
//! year opens at, and zero otherwise. The signal is formed in `piotroski.rs`,
//! which cites its source, and is stated over a year as
//! `docs/adr/piotroski-signals-and-score.md` fixes it. It reads the year
//! alone, and nothing of the year before.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{ZERO_OPENING_TOTAL_ASSETS, f_accrual, over_a_year};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_ACCRUAL: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_accrual",
        meaning: "One where cash flow from operations over a year divided by total assets at \
                  the instant it opens at is above net income over the year divided by the \
                  same; zero otherwise. It compares cash flow from operations and net income \
                  each scaled by total assets at the instant the year opens at.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::NetIncome,
            Concept::OperatingCashFlow,
            Concept::TotalAssets,
        ],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_OPENING_TOTAL_ASSETS,
        ],
    },
    readings: &[None, None, Some(Reading::Opening)],
    derived: piotroski_f_accrual,
};

fn piotroski_f_accrual(computing: &Computing) -> Result<Number, Reasons> {
    let [net_income, operating_cash_flow, opening_total_assets] =
        over_a_year(computing, &PIOTROSKI_F_ACCRUAL)?;
    f_accrual(&net_income, &operating_cash_flow, &opening_total_assets)
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_ACCRUAL;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_ACCRUAL.entry;
        assert_eq!(entry.name, "piotroski_f_accrual");
        assert!(!entry.name.contains("common"));
        for words in [
            "One where cash flow from operations over a year divided by total assets at the \
             instant it opens at is above net income over the year divided by the same; zero \
             otherwise.",
            "compares cash flow from operations and net income each scaled by total assets at \
             the instant the year opens at",
        ] {
            assert!(entry.meaning.contains(words), "{words}");
        }
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::NetIncome,
                Concept::OperatingCashFlow,
                Concept::TotalAssets
            ]
        );
        assert_eq!(
            PIOTROSKI_F_ACCRUAL.readings,
            [None, None, Some(Reading::Opening)]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_opening_total_assets",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
