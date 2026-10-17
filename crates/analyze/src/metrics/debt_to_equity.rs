//! Debt to equity: the filer's borrowings, short and long together, as a
//! ratio to its shareholders' equity, at one instant.
//!
//! All three are balances `canonical-concepts` v2 states at an instant and
//! applies to every kind, so the metric reads one row and no other period.
//! The denominator is `shareholders_equity` as v2 carries it, preferred claim
//! included: the open-questions record forms common equity only for the
//! common-shareholder forms of return on equity, price/book and sustainable
//! growth. Neither debt is a term that contributes nothing where it is
//! absent, and a negative equity is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Shareholders' equity is zero, and no ratio to zero is defined.
const ZERO_SHAREHOLDERS_EQUITY: &str = "zero_shareholders_equity";

pub(super) const DEBT_TO_EQUITY: Derivation = Derivation {
    entry: Entry {
        name: "debt_to_equity",
        meaning: "Short-term and long-term debt together over shareholders' equity at the \
                  period's end: the borrowing the business carries for each unit of its \
                  owners' claim.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Instant,
        inputs: &[
            Concept::ShortTermDebt,
            Concept::LongTermDebt,
            Concept::ShareholdersEquity,
        ],
        conditions: &[ZERO_SHAREHOLDERS_EQUITY],
    },
    readings: &[None, None, None],
    derived: debt_to_equity,
};

fn debt_to_equity(computing: &Computing) -> Result<Number, Reasons> {
    let [short_term_debt, long_term_debt, shareholders_equity] = read(computing, &DEBT_TO_EQUITY)?;
    short_term_debt
        .plus(&long_term_debt)
        .divided(&shareholders_equity)
        .ok_or_else(|| declined(ZERO_SHAREHOLDERS_EQUITY))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DEBT_TO_EQUITY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DEBT_TO_EQUITY.entry;
        assert_eq!(entry.name, "debt_to_equity");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [
                Concept::ShortTermDebt,
                Concept::LongTermDebt,
                Concept::ShareholdersEquity
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_shareholders_equity", INPUT_NOT_A_DECIMAL]
        );
    }
}
