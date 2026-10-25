//! Free cash flow, formed once, here. Every metric that reads it reads it
//! from this module, and none writes the subtraction itself. What it is, and
//! which metrics read it, is
//! `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s, under the method
//! version.
//!
//! It is no metric of the method and is handed over nowhere. It is formed at
//! every duration a reader computes at, and no year test is made here.

use crate::arithmetic::Number;

/// The free cash flow a metric divides by is zero, and no fraction of zero is
/// defined. Every metric over free cash flow is undefined under this one
/// condition.
pub(super) const ZERO_FREE_CASH_FLOW: &str = "zero_free_cash_flow";

/// Free cash flow: operating cash flow less capital expenditure, over one
/// duration. Capital expenditure is a magnitude, positive when cash was
/// spent, so the term is the difference and not a sum. Its sign is not
/// branched on: a negative free cash flow is a value. Neither input
/// contributes nothing where it is absent, so an `Unknown` capital
/// expenditure leaves no free cash flow to form, and is never read as zero.
///
/// Source: Securities and Exchange Commission, Division of Corporation
/// Finance, *Compliance and Disclosure Interpretations: Non-GAAP Financial
/// Measures*, Section 102, Question 102.07, May 17, 2016,
/// <https://www.sec.gov/corpfin/non-gaap-financial-measures>: "Some companies
/// present a measure of "free cash flow," which is typically calculated as
/// cash flows from operating activities as presented in the statement of
/// cash flows under GAAP, less capital expenditures."
pub(super) fn free_cash_flow(operating_cash_flow: &Number, capital_expenditure: &Number) -> Number {
    operating_cash_flow.minus(capital_expenditure)
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};
    use vfi_contracts::canonical_concepts::{
        Attempt, Attempted, Attempts, Concept, Period, Resolution, Row, Rule, SetBy,
    };

    use super::free_cash_flow;
    use crate::metrics::reading::{Computing, read};
    use crate::metrics::{Derivation, Entry, Shape, Unit};
    use crate::settings::Settings;

    /// Free cash flow as a reader forms it at one duration, its two inputs in
    /// the record's order.
    const FREE_CASH_FLOW: Derivation = Derivation {
        entry: Entry {
            name: "free_cash_flow",
            meaning: "Operating cash flow less capital expenditure for the period.",
            unit: Unit::Multiple,
            stated_at: Shape::Duration,
            inputs: &[Concept::OperatingCashFlow, Concept::CapitalExpenditure],
            conditions: &[],
        },
        readings: &[None, None],
        derived: |computing| {
            let [operating_cash_flow, capital_expenditure] = read(computing, &FREE_CASH_FLOW)?;
            Ok(free_cash_flow(&operating_cash_flow, &capital_expenditure))
        },
    };

    fn fiscal_year() -> Period {
        Period::Duration {
            start: "2024-09-29".into(),
            end: "2025-09-27".into(),
        }
    }

    fn value(amount: &str) -> Resolution {
        Resolution::Value {
            amount: amount.into(),
            set_by: SetBy::Asserted {
                rule: Rule {
                    registry: "1".into(),
                    id: "asserted".into(),
                },
                filing: "0000320193-25-000079".into(),
            },
        }
    }

    /// A row over the fiscal year where operating cash flow and capital
    /// expenditure state what is given, and every other concept states `1`.
    fn row(operating_cash_flow: Resolution, capital_expenditure: Resolution) -> Row {
        Row::at(
            fiscal_year(),
            std::array::from_fn(|at| match Concept::ALL[at] {
                Concept::OperatingCashFlow => operating_cash_flow.clone(),
                Concept::CapitalExpenditure => capital_expenditure.clone(),
                _ => value("1"),
            }),
        )
    }

    fn formed(row: &Row) -> Result<Box<str>, Reasons> {
        let history = std::slice::from_ref(row);
        (FREE_CASH_FLOW.derived)(&Computing::at(row, history, &Settings::preset()))
            .map(|number| number.written())
    }

    #[test]
    fn free_cash_flow_is_operating_cash_flow_less_capital_expenditure() {
        for (operating_cash_flow, capital_expenditure, free) in [
            ("800", "228", "572"),
            ("150", "50", "100"),
            ("800", "800", "0"),
            ("800", "900", "-100"),
        ] {
            assert_eq!(
                formed(&row(value(operating_cash_flow), value(capital_expenditure))),
                Ok(free.into()),
                "{operating_cash_flow} less {capital_expenditure}"
            );
        }
    }

    #[test]
    fn an_unknown_capital_expenditure_is_never_read_as_zero() {
        let unknown = Resolution::Unknown {
            attempted: Attempts::in_filings(
                Attempted::in_filing("0000320193-25-000079".into(), Attempt::that_ran(Vec::new())),
                Vec::new(),
            ),
        };
        let at = row(value("800"), unknown);
        let reason = Reason::input_unknown(&at, Concept::CapitalExpenditure)
            .expect("capital expenditure is Unknown at the row");
        assert_eq!(formed(&at), Err(Reasons::of(reason, Vec::new())));
    }
}
