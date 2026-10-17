//! Cash ratio: cash and equivalents as a ratio to current liabilities, at one
//! instant.
//!
//! Both are balances `canonical-concepts` v2 states at an instant, so the
//! metric reads one row and no other period. It reads the narrow figure alone:
//! the open-questions record leaves `short_term_investments` to net debt.
//! `cash_and_equivalents` applies to every kind and `current_liabilities` to
//! an operating filer alone, so any other kind carries the metric absent
//! naming `current_liabilities` only.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, ZERO_CURRENT_LIABILITIES, declined, read};
use crate::arithmetic::Number;

pub(super) const CASH_RATIO: Derivation = Derivation {
    entry: Entry {
        name: "cash_ratio",
        meaning: "Cash and cash equivalents over current liabilities at the period's end: \
                  the cash on hand for each unit the business owes within a year.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Instant,
        inputs: &[Concept::CashAndEquivalents, Concept::CurrentLiabilities],
        conditions: &[ZERO_CURRENT_LIABILITIES],
    },
    readings: &[None, None],
    derived: cash_ratio,
};

fn cash_ratio(computing: &Computing) -> Result<Number, Reasons> {
    let [cash_and_equivalents, current_liabilities] = read(computing, &CASH_RATIO)?;
    cash_and_equivalents
        .divided(&current_liabilities)
        .ok_or_else(|| declined(ZERO_CURRENT_LIABILITIES))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::CASH_RATIO;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &CASH_RATIO.entry;
        assert_eq!(entry.name, "cash_ratio");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [Concept::CashAndEquivalents, Concept::CurrentLiabilities]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_current_liabilities", INPUT_NOT_A_DECIMAL]
        );
    }
}
