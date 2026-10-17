//! Quick ratio: current assets less inventory as a ratio to current
//! liabilities, at one instant.
//!
//! All three are balances `canonical-concepts` v2 states at an instant and
//! applies to an operating filer alone, so the metric reads one row and no
//! other period. Inventory is not a term that contributes nothing where it is
//! absent: an `inventory` a silence reading set to zero is a value of zero,
//! and the ratio then equals the current ratio. A negative difference is a
//! value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, ZERO_CURRENT_LIABILITIES, declined, read};
use crate::arithmetic::Number;

pub(super) const QUICK_RATIO: Derivation = Derivation {
    entry: Entry {
        name: "quick_ratio",
        meaning: "Current assets less inventory over current liabilities at the period's \
                  end: what the business could turn into cash within a year without \
                  selling its stock, for each unit it owes within one.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Instant,
        inputs: &[
            Concept::CurrentAssets,
            Concept::Inventory,
            Concept::CurrentLiabilities,
        ],
        conditions: &[ZERO_CURRENT_LIABILITIES],
    },
    readings: &[None, None, None],
    derived: quick_ratio,
};

fn quick_ratio(computing: &Computing) -> Result<Number, Reasons> {
    let [current_assets, inventory, current_liabilities] = read(computing, &QUICK_RATIO)?;
    current_assets
        .minus(&inventory)
        .divided(&current_liabilities)
        .ok_or_else(|| declined(ZERO_CURRENT_LIABILITIES))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::QUICK_RATIO;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &QUICK_RATIO.entry;
        assert_eq!(entry.name, "quick_ratio");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [
                Concept::CurrentAssets,
                Concept::Inventory,
                Concept::CurrentLiabilities
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_current_liabilities", INPUT_NOT_A_DECIMAL]
        );
    }
}
