//! Operating margin: operating income as a fraction of revenue, over one
//! period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration, so the metric
//! reads one row and no other period. A filer whose kind `operating_income`
//! does not apply to is absent with `input_not_applicable`, and an operating
//! loss is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, ZERO_REVENUE, declined, read};
use crate::arithmetic::Number;

pub(super) const OPERATING_MARGIN: Derivation = Derivation {
    entry: Entry {
        name: "operating_margin",
        meaning: "Operating income over revenue for the period: the fraction of each unit of \
                  revenue left once the costs of running the business are met, before \
                  interest and tax.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::OperatingIncome, Concept::Revenue],
        conditions: &[ZERO_REVENUE],
    },
    derived: operating_margin,
};

fn operating_margin(row: &Row) -> Result<Number, Reasons> {
    let [operating_income, revenue] = read(row, OPERATING_MARGIN.entry.inputs)?;
    operating_income
        .divided(&revenue)
        .ok_or_else(|| declined(ZERO_REVENUE))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::OPERATING_MARGIN;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &OPERATING_MARGIN.entry;
        assert_eq!(entry.name, "operating_margin");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::OperatingIncome, Concept::Revenue]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_revenue", INPUT_NOT_A_DECIMAL]
        );
    }
}
