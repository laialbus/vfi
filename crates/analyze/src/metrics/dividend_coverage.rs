//! Dividend coverage: how many times net income covers dividends paid, over
//! one period, in the total form.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period.
//! `dividends_paid` is cash paid to every class of shareholder, so this is an
//! all-holder measure. A `dividends_paid` a silence reading set to zero is a
//! value of zero, a period in which nothing was paid, and nothing is a
//! multiple of it, so the coverage is then declined. A net loss is a value
//! that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Dividends paid are zero, and nothing is a multiple of zero.
const ZERO_DIVIDENDS_PAID: &str = "zero_dividends_paid";

pub(super) const DIVIDEND_COVERAGE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_coverage",
        meaning: "Net income over dividends paid for the period: the times the period's \
                  profit covers the cash paid to shareholders of every class.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[Concept::NetIncome, Concept::DividendsPaid],
        conditions: &[ZERO_DIVIDENDS_PAID],
    },
    readings: &[None, None],
    derived: dividend_coverage,
};

fn dividend_coverage(computing: &Computing) -> Result<Number, Reasons> {
    let [net_income, dividends_paid] = read(computing, &DIVIDEND_COVERAGE)?;
    net_income
        .divided(&dividends_paid)
        .ok_or_else(|| declined(ZERO_DIVIDENDS_PAID))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_COVERAGE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_COVERAGE.entry;
        assert_eq!(entry.name, "dividend_coverage");
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::NetIncome, Concept::DividendsPaid]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_dividends_paid", INPUT_NOT_A_DECIMAL]
        );
    }
}
