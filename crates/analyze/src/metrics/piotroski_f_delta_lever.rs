//! Piotroski's F_ΔLEVER: one where long-term debt at the instant a year closes
//! at, scaled by the average of total assets at the instants it opens and
//! closes at, fell from the same over the year before it, and zero otherwise.
//! The signal is formed in `piotroski.rs`, which cites its source and argues
//! that an unchanged ratio gives zero, and is stated over a year as
//! `docs/adr/piotroski-signals-and-score.md` fixes it. Total assets at the
//! instant the year opens at are also the year before's at its close, and are
//! read once.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{
    ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS, f_delta_lever, over_a_year_and_the_year_before_averaged,
};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, ZERO_AVERAGE_TOTAL_ASSETS};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_DELTA_LEVER: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_delta_lever",
        meaning: "One where long-term debt, without its current portion, at the instant a year \
                  closes at, divided by the average of total assets at the instants it opens \
                  and closes at, fell from the same over the year before it; zero otherwise, \
                  and an unchanged ratio gives zero.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::LongTermDebt, Concept::TotalAssets],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_AVERAGE_TOTAL_ASSETS,
            ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
        ],
    },
    readings: &[Some(Reading::Closing), Some(Reading::Averaged)],
    derived: piotroski_f_delta_lever,
};

fn piotroski_f_delta_lever(computing: &Computing) -> Result<Number, Reasons> {
    let (
        [long_term_debt, average_total_assets],
        [preceding_long_term_debt, preceding_average_total_assets],
    ) = over_a_year_and_the_year_before_averaged(computing, &PIOTROSKI_F_DELTA_LEVER)?;
    f_delta_lever(
        &long_term_debt,
        &average_total_assets,
        &preceding_long_term_debt,
        &preceding_average_total_assets,
    )
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_DELTA_LEVER;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_DELTA_LEVER.entry;
        assert_eq!(entry.name, "piotroski_f_delta_lever");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where long-term debt, without its current portion, at the instant a year closes \
             at, divided by the average of total assets at the instants it opens and closes at, \
             fell from the same over the year before it; zero otherwise, and an unchanged ratio \
             gives zero."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::LongTermDebt, Concept::TotalAssets]);
        assert_eq!(
            PIOTROSKI_F_DELTA_LEVER.readings,
            [Some(Reading::Closing), Some(Reading::Averaged)]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_average_total_assets",
                "zero_preceding_average_total_assets",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
