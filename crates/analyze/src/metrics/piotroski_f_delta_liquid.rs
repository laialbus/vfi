//! Piotroski's F_ΔLIQUID: one where the current ratio at the instant a year
//! closes at, current assets divided by current liabilities, is above the same
//! at the instant the year before it closes at, and zero otherwise. The signal
//! is formed in `piotroski.rs`, which cites its source, and is stated over a
//! year as `docs/adr/piotroski-signals-and-score.md` fixes it.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{
    ZERO_PRECEDING_CURRENT_LIABILITIES, f_delta_liquid, over_a_year_and_the_year_before,
};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, ZERO_CURRENT_LIABILITIES};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_DELTA_LIQUID: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_delta_liquid",
        meaning: "One where current assets divided by current liabilities, each at the instant \
                  a year closes at, is above the same at the instant the year before it closes \
                  at; zero otherwise.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::CurrentAssets, Concept::CurrentLiabilities],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_CURRENT_LIABILITIES,
            ZERO_PRECEDING_CURRENT_LIABILITIES,
        ],
    },
    readings: &[Some(Reading::Closing), Some(Reading::Closing)],
    derived: piotroski_f_delta_liquid,
};

fn piotroski_f_delta_liquid(computing: &Computing) -> Result<Number, Reasons> {
    let (
        [current_assets, current_liabilities],
        [preceding_current_assets, preceding_current_liabilities],
    ) = over_a_year_and_the_year_before(computing, &PIOTROSKI_F_DELTA_LIQUID)?;
    f_delta_liquid(
        &current_assets,
        &current_liabilities,
        &preceding_current_assets,
        &preceding_current_liabilities,
    )
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_DELTA_LIQUID;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_DELTA_LIQUID.entry;
        assert_eq!(entry.name, "piotroski_f_delta_liquid");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where current assets divided by current liabilities, each at the instant a year \
             closes at, is above the same at the instant the year before it closes at; zero \
             otherwise."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [Concept::CurrentAssets, Concept::CurrentLiabilities]
        );
        assert_eq!(
            PIOTROSKI_F_DELTA_LIQUID.readings,
            [Some(Reading::Closing), Some(Reading::Closing)]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_current_liabilities",
                "zero_preceding_current_liabilities",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
