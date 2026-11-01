//! Piotroski's EQ_OFFER: one where the diluted weighted average share count
//! over a year is not above the same over the year before it, and zero
//! otherwise, standing for "did not issue common equity". The signal is
//! formed in `piotroski.rs`, which cites its source and names what the count
//! cannot see, and is stated over a year as
//! `docs/adr/piotroski-signals-and-score.md` fixes it. It reads no balance
//! and divides nothing.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{eq_offer, over_a_year_and_the_year_before};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{Computing, Derivation, Entry, Shape, Unit};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_EQ_OFFER: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_eq_offer",
        meaning: "One where the diluted weighted average share count over a year is not above \
                  the same over the year before it; zero otherwise. It stands for 'did not \
                  issue common equity' and cannot see an issue offset by a repurchase.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[Concept::DilutedSharesWeightedAverage],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
        ],
    },
    readings: &[None],
    derived: piotroski_eq_offer,
};

fn piotroski_eq_offer(computing: &Computing) -> Result<Number, Reasons> {
    let ([diluted_shares_weighted_average], [preceding_diluted_shares_weighted_average]) =
        over_a_year_and_the_year_before(computing, &PIOTROSKI_EQ_OFFER)?;
    Ok(eq_offer(
        &diluted_shares_weighted_average,
        &preceding_diluted_shares_weighted_average,
    ))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_EQ_OFFER;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_EQ_OFFER.entry;
        assert_eq!(entry.name, "piotroski_eq_offer");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "One where the diluted weighted average share count over a year is not above the \
             same over the year before it; zero otherwise. It stands for 'did not issue common \
             equity' and cannot see an issue offset by a repurchase."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::DilutedSharesWeightedAverage]);
        assert_eq!(PIOTROSKI_EQ_OFFER.readings, [None]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
