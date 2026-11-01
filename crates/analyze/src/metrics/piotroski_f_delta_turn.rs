//! Piotroski's F_ΔTURN: one where the asset turnover over a year, revenue
//! scaled by total assets at the instant the year opens at, less the same
//! over the year before it, is above zero, and zero otherwise. The signal is
//! formed in `piotroski.rs`, which cites its source and names Table 1's
//! average beside it, and is stated over a year as
//! `docs/adr/piotroski-signals-and-score.md` fixes it. Each year's turnover
//! is scaled by the balance at the instant that year opens at.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{
    ZERO_OPENING_TOTAL_ASSETS, ZERO_PRECEDING_OPENING_TOTAL_ASSETS, f_delta_turn,
    over_a_year_and_the_year_before,
};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_DELTA_TURN: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_delta_turn",
        meaning: "One where revenue over a year divided by total assets at the instant it \
                  opens at, less the same over the year before it, is above zero; zero \
                  otherwise.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::Revenue, Concept::TotalAssets],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_OPENING_TOTAL_ASSETS,
            ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
        ],
    },
    readings: &[None, Some(Reading::Opening)],
    derived: piotroski_f_delta_turn,
};

fn piotroski_f_delta_turn(computing: &Computing) -> Result<Number, Reasons> {
    let ([revenue, opening_total_assets], [preceding_revenue, preceding_opening_total_assets]) =
        over_a_year_and_the_year_before(computing, &PIOTROSKI_F_DELTA_TURN)?;
    f_delta_turn(
        &revenue,
        &opening_total_assets,
        &preceding_revenue,
        &preceding_opening_total_assets,
    )
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_DELTA_TURN;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_DELTA_TURN.entry;
        assert_eq!(entry.name, "piotroski_f_delta_turn");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where revenue over a year divided by total assets at the instant it opens at, \
             less the same over the year before it, is above zero; zero otherwise."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::Revenue, Concept::TotalAssets]);
        assert_eq!(
            PIOTROSKI_F_DELTA_TURN.readings,
            [None, Some(Reading::Opening)]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_opening_total_assets",
                "zero_preceding_opening_total_assets",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
