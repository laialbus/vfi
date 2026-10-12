//! Gross margin: gross profit as a fraction of revenue, over one period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration, so the metric
//! reads one row and no other period. A filer whose kind `gross_profit` does
//! not apply to is absent with `input_not_applicable`, and a negative gross
//! profit is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, ZERO_REVENUE, declined, read};
use crate::arithmetic::Number;

pub(super) const GROSS_MARGIN: Derivation = Derivation {
    entry: Entry {
        name: "gross_margin",
        meaning: "Gross profit over revenue for the period: the fraction of each unit of \
                  revenue left once the cost of producing what was sold is met.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::GrossProfit, Concept::Revenue],
        conditions: &[ZERO_REVENUE],
    },
    derived: gross_margin,
};

fn gross_margin(row: &Row) -> Result<Number, Reasons> {
    let [gross_profit, revenue] = read(row, GROSS_MARGIN.entry.inputs)?;
    gross_profit
        .divided(&revenue)
        .ok_or_else(|| declined(ZERO_REVENUE))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::GROSS_MARGIN;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &GROSS_MARGIN.entry;
        assert_eq!(entry.name, "gross_margin");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::GrossProfit, Concept::Revenue]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_revenue", INPUT_NOT_A_DECIMAL]
        );
    }
}
