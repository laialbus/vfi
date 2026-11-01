//! Piotroski's F_ΔMARGIN: one where the gross margin over a year, gross
//! profit as a fraction of revenue, less the same over the year before it,
//! is above zero, and zero otherwise. The signal is formed in
//! `piotroski.rs`, which cites its source, and is stated over a year as
//! `docs/adr/piotroski-signals-and-score.md` fixes it. It reads no balance.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{ZERO_PRECEDING_REVENUE, f_delta_margin, over_a_year_and_the_year_before};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Shape, Unit, ZERO_REVENUE};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_DELTA_MARGIN: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_delta_margin",
        meaning: "One where gross profit over a year divided by revenue over it, less the same \
                  over the year before it, is above zero; zero otherwise.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::GrossProfit, Concept::Revenue],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_REVENUE,
            ZERO_PRECEDING_REVENUE,
        ],
    },
    readings: &[None, None],
    derived: piotroski_f_delta_margin,
};

fn piotroski_f_delta_margin(computing: &Computing) -> Result<Number, Reasons> {
    let ([gross_profit, revenue], [preceding_gross_profit, preceding_revenue]) =
        over_a_year_and_the_year_before(computing, &PIOTROSKI_F_DELTA_MARGIN)?;
    f_delta_margin(
        &gross_profit,
        &revenue,
        &preceding_gross_profit,
        &preceding_revenue,
    )
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_DELTA_MARGIN;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_DELTA_MARGIN.entry;
        assert_eq!(entry.name, "piotroski_f_delta_margin");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where gross profit over a year divided by revenue over it, less the same over \
             the year before it, is above zero; zero otherwise."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::GrossProfit, Concept::Revenue]);
        assert_eq!(PIOTROSKI_F_DELTA_MARGIN.readings, [None, None]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_revenue",
                "zero_preceding_revenue",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
