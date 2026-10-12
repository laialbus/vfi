//! Interest coverage: how many times operating income covers interest
//! expense, over one period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration, so the metric
//! reads one row and no other period. A filer whose kind `operating_income`
//! does not apply to is absent with `input_not_applicable`. `interest_expense`
//! is a magnitude, positive when it is a cost; one a statement nets and no one
//! decomposed is `Unknown`, and the coverage is absent with that reason rather
//! than computed over a gross figure nobody stated.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Interest expense is zero, and nothing is a multiple of zero.
const ZERO_INTEREST_EXPENSE: &str = "zero_interest_expense";

pub(super) const INTEREST_COVERAGE: Derivation = Derivation {
    entry: Entry {
        name: "interest_coverage",
        meaning: "Operating income over interest expense for the period: the times the \
                  income from running the business covers the interest owed on its debt.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[Concept::OperatingIncome, Concept::InterestExpense],
        conditions: &[ZERO_INTEREST_EXPENSE],
    },
    derived: interest_coverage,
};

fn interest_coverage(row: &Row) -> Result<Number, Reasons> {
    let [operating_income, interest_expense] = read(row, INTEREST_COVERAGE.entry.inputs)?;
    operating_income
        .divided(&interest_expense)
        .ok_or_else(|| declined(ZERO_INTEREST_EXPENSE))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::INTEREST_COVERAGE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &INTEREST_COVERAGE.entry;
        assert_eq!(entry.name, "interest_coverage");
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [Concept::OperatingIncome, Concept::InterestExpense]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_interest_expense", INPUT_NOT_A_DECIMAL]
        );
    }
}
