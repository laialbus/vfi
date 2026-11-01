//! Piotroski's F_ROA: one where net income over a year, scaled by total
//! assets at the instant the year opens at, is above zero, and zero
//! otherwise. The signal is formed in `piotroski.rs`, which cites its source,
//! and is stated over a year as `docs/adr/piotroski-signals-and-score.md`
//! fixes it. It reads the year alone, and nothing of the year before.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{ZERO_OPENING_TOTAL_ASSETS, f_roa, over_a_year};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_ROA: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_roa",
        meaning: "One where net income over a year divided by total assets at the instant it \
                  opens at is above zero; zero otherwise.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::NetIncome, Concept::TotalAssets],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_OPENING_TOTAL_ASSETS,
        ],
    },
    readings: &[None, Some(Reading::Opening)],
    derived: piotroski_f_roa,
};

fn piotroski_f_roa(computing: &Computing) -> Result<Number, Reasons> {
    let [net_income, opening_total_assets] = over_a_year(computing, &PIOTROSKI_F_ROA)?;
    f_roa(&net_income, &opening_total_assets)
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_ROA;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_ROA.entry;
        assert_eq!(entry.name, "piotroski_f_roa");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where net income over a year divided by total assets at the instant it opens \
             at is above zero; zero otherwise."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::NetIncome, Concept::TotalAssets]);
        assert_eq!(PIOTROSKI_F_ROA.readings, [None, Some(Reading::Opening)]);
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
