//! Liabilities to assets: total liabilities as a fraction of total assets, at
//! one instant.
//!
//! Both are balances `canonical-concepts` v2 states at an instant and applies
//! to every kind, so the metric reads one row and no other period.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Total assets are zero, and no fraction of zero is defined.
const ZERO_TOTAL_ASSETS: &str = "zero_total_assets";

pub(super) const LIABILITIES_TO_ASSETS: Derivation = Derivation {
    entry: Entry {
        name: "liabilities_to_assets",
        meaning: "Total liabilities over total assets at the period's end: the fraction of \
                  what the business holds that is owed to others.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Instant,
        inputs: &[Concept::TotalLiabilities, Concept::TotalAssets],
        conditions: &[ZERO_TOTAL_ASSETS],
    },
    derived: liabilities_to_assets,
};

fn liabilities_to_assets(row: &Row) -> Result<Number, Reasons> {
    let [total_liabilities, total_assets] = read(row, LIABILITIES_TO_ASSETS.entry.inputs)?;
    total_liabilities
        .divided(&total_assets)
        .ok_or_else(|| declined(ZERO_TOTAL_ASSETS))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::LIABILITIES_TO_ASSETS;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &LIABILITIES_TO_ASSETS.entry;
        assert_eq!(entry.name, "liabilities_to_assets");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [Concept::TotalLiabilities, Concept::TotalAssets]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_total_assets", INPUT_NOT_A_DECIMAL]
        );
    }
}
