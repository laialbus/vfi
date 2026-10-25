//! Dividend payout on free cash flow: dividends paid as a fraction of
//! operating cash flow less capital expenditure, over one period, in the total
//! form.
//!
//! All three are flows `canonical-concepts` v2 states at a duration and
//! applies to every kind of filer, so the metric reads one row and no other
//! period. Free cash flow is formed in [`super::free_cash_flow`], and the
//! payout is read at every duration, with no year test. An `Unknown` capital
//! expenditure makes the payout absent naming it. A negative free cash flow
//! is a value that gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::free_cash_flow::{ZERO_FREE_CASH_FLOW, free_cash_flow};
use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

pub(super) const DIVIDEND_PAYOUT_ON_FREE_CASH_FLOW: Derivation = Derivation {
    entry: Entry {
        name: "dividend_payout_on_free_cash_flow",
        meaning: "Dividends paid over operating cash flow less capital expenditure for \
                  the period: the fraction of the cash the business generated after \
                  investing in itself that was paid out to shareholders of every class.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::DividendsPaid,
            Concept::OperatingCashFlow,
            Concept::CapitalExpenditure,
        ],
        conditions: &[ZERO_FREE_CASH_FLOW],
    },
    readings: &[None, None, None],
    derived: dividend_payout_on_free_cash_flow,
};

fn dividend_payout_on_free_cash_flow(computing: &Computing) -> Result<Number, Reasons> {
    let [dividends_paid, operating_cash_flow, capital_expenditure] =
        read(computing, &DIVIDEND_PAYOUT_ON_FREE_CASH_FLOW)?;
    dividends_paid
        .divided(&free_cash_flow(&operating_cash_flow, &capital_expenditure))
        .ok_or_else(|| declined(ZERO_FREE_CASH_FLOW))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_PAYOUT_ON_FREE_CASH_FLOW;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_PAYOUT_ON_FREE_CASH_FLOW.entry;
        assert_eq!(entry.name, "dividend_payout_on_free_cash_flow");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::DividendsPaid,
                Concept::OperatingCashFlow,
                Concept::CapitalExpenditure
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_free_cash_flow", INPUT_NOT_A_DECIMAL]
        );
    }
}
