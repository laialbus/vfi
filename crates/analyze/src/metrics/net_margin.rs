//! Net margin: net income as a fraction of revenue, over one period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period, and
//! is computed for a bank, an insurer and an investment company alike. A net
//! loss is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, ZERO_REVENUE, declined, read};
use crate::arithmetic::Number;

pub(super) const NET_MARGIN: Derivation = Derivation {
    entry: Entry {
        name: "net_margin",
        meaning: "Net income over revenue for the period: the fraction of each unit of \
                  revenue left as profit once every cost, interest and tax is met.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::NetIncome, Concept::Revenue],
        conditions: &[ZERO_REVENUE],
    },
    derived: net_margin,
};

fn net_margin(row: &Row) -> Result<Number, Reasons> {
    let [net_income, revenue] = read(row, NET_MARGIN.entry.inputs)?;
    net_income
        .divided(&revenue)
        .ok_or_else(|| declined(ZERO_REVENUE))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::NET_MARGIN;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &NET_MARGIN.entry;
        assert_eq!(entry.name, "net_margin");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::NetIncome, Concept::Revenue]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_revenue", INPUT_NOT_A_DECIMAL]
        );
    }
}
