//! Dividend payout on earnings: dividends paid as a fraction of net income,
//! over one period, in the total form.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period.
//! `dividends_paid` is cash paid to every class of shareholder, so this is an
//! all-holder measure and reads no `preferred_dividends`. A `dividends_paid` a
//! silence reading set to zero is a value of zero, and the payout is then
//! zero. A net loss is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Net income is zero, and no fraction of zero is defined.
const ZERO_NET_INCOME: &str = "zero_net_income";

pub(super) const DIVIDEND_PAYOUT_ON_EARNINGS: Derivation = Derivation {
    entry: Entry {
        name: "dividend_payout_on_earnings",
        meaning: "Dividends paid over net income for the period: the fraction of the \
                  period's profit paid out in cash to shareholders of every class.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::DividendsPaid, Concept::NetIncome],
        conditions: &[ZERO_NET_INCOME],
    },
    derived: dividend_payout_on_earnings,
};

fn dividend_payout_on_earnings(row: &Row) -> Result<Number, Reasons> {
    let [dividends_paid, net_income] = read(row, DIVIDEND_PAYOUT_ON_EARNINGS.entry.inputs)?;
    dividends_paid
        .divided(&net_income)
        .ok_or_else(|| declined(ZERO_NET_INCOME))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_PAYOUT_ON_EARNINGS;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_PAYOUT_ON_EARNINGS.entry;
        assert_eq!(entry.name, "dividend_payout_on_earnings");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::DividendsPaid, Concept::NetIncome]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_net_income", INPUT_NOT_A_DECIMAL]
        );
    }
}
