//! Current ratio: current assets as a ratio to current liabilities, at one
//! instant.
//!
//! Both are balances `canonical-concepts` v2 states at an instant and applies
//! to an operating filer alone, so the metric reads one row and no other
//! period, and any other kind carries it absent with `input_not_applicable`.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, ZERO_CURRENT_LIABILITIES, declined, read};
use crate::arithmetic::Number;

pub(super) const CURRENT_RATIO: Derivation = Derivation {
    entry: Entry {
        name: "current_ratio",
        meaning: "Current assets over current liabilities at the period's end: what the \
                  business expects to turn into cash within a year for each unit it owes \
                  within one.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Instant,
        inputs: &[Concept::CurrentAssets, Concept::CurrentLiabilities],
        conditions: &[ZERO_CURRENT_LIABILITIES],
    },
    readings: &[None, None],
    derived: current_ratio,
};

fn current_ratio(computing: &Computing) -> Result<Number, Reasons> {
    let [current_assets, current_liabilities] = read(computing, &CURRENT_RATIO)?;
    current_assets
        .divided(&current_liabilities)
        .ok_or_else(|| declined(ZERO_CURRENT_LIABILITIES))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::CURRENT_RATIO;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &CURRENT_RATIO.entry;
        assert_eq!(entry.name, "current_ratio");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [Concept::CurrentAssets, Concept::CurrentLiabilities]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_current_liabilities", INPUT_NOT_A_DECIMAL]
        );
    }
}
