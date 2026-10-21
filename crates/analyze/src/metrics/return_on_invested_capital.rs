//! Return on invested capital: net operating profit after tax over a duration
//! as a fraction of the average of invested capital at the instants the
//! duration opens and closes at.
//!
//! Operating income, income tax expense and pretax income are flows
//! `canonical-concepts` v2 states at a duration, and the five balances of
//! invested capital it states at an instant. So the metric is stated at the
//! duration, reads the three flows at its row, and reads each balance
//! averaged, which `docs/adr/which-other-period-a-metric-reads.md` fixes for
//! return on invested capital in its table. Invested capital is formed in
//! [`super::capital`] at each instant before the average, and the effective
//! tax rate and net operating profit after tax there over the duration.
//!
//! Short-term investments, read for net debt alone, are the one term that
//! contributes nothing where they are `NotApplicable`, so a REIT has a
//! figure. Every other input is absent where it is absent, and a negative
//! average invested capital is a value that gives a figure with its sign.
//!
//! The formula is `docs/adr/capital-employed-invested-capital-and-nopat.md`'s,
//! whose sources are cited beside the terms.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::capital::{
    NON_POSITIVE_PRETAX_INCOME, NOTHING_WHERE_NOT_APPLICABLE, TAX_RATE_OUTSIDE_ZERO_TO_ONE,
    ZERO_INVESTED_CAPITAL, effective_tax_rate, invested_capital, net_operating_profit_after_tax,
};
use super::reading::reads_with_nothing;
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const RETURN_ON_INVESTED_CAPITAL: Derivation = Derivation {
    entry: Entry {
        name: "return_on_invested_capital",
        meaning: "Operating income over the duration times one less the effective tax rate, \
                  income tax expense over pretax income, divided by the average of \
                  shareholders' equity plus short-term and long-term debt less cash and \
                  short-term investments at the instants it opens and closes at: the profit \
                  the business earned after tax for each unit of the capital its owners and \
                  lenders invested in it, the preferred claim included and cash and \
                  short-term investments netted.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::OperatingIncome,
            Concept::IncomeTaxExpense,
            Concept::PretaxIncome,
            Concept::ShareholdersEquity,
            Concept::ShortTermDebt,
            Concept::LongTermDebt,
            Concept::CashAndEquivalents,
            Concept::ShortTermInvestments,
        ],
        conditions: &[
            NON_POSITIVE_PRETAX_INCOME,
            TAX_RATE_OUTSIDE_ZERO_TO_ONE,
            ZERO_INVESTED_CAPITAL,
            START_NOT_A_DATE,
        ],
    },
    readings: &[
        None,
        None,
        None,
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
    ],
    derived: return_on_invested_capital,
};

fn return_on_invested_capital(computing: &Computing) -> Result<Number, Reasons> {
    let [
        operating_income,
        income_tax_expense,
        pretax_income,
        shareholders_equity,
        short_term_debt,
        long_term_debt,
        cash_and_equivalents,
        short_term_investments,
    ] = reads_with_nothing(
        computing,
        &RETURN_ON_INVESTED_CAPITAL,
        NOTHING_WHERE_NOT_APPLICABLE,
    )?;
    let rate = effective_tax_rate(&income_tax_expense.mean(), &pretax_income.mean())?;
    net_operating_profit_after_tax(&operating_income.mean(), &rate)
        .divided(
            &invested_capital(
                &shareholders_equity,
                &short_term_debt,
                &long_term_debt,
                &cash_and_equivalents,
                &short_term_investments,
            )
            .mean(),
        )
        .ok_or_else(|| declined(ZERO_INVESTED_CAPITAL))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::RETURN_ON_INVESTED_CAPITAL;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &RETURN_ON_INVESTED_CAPITAL.entry;
        assert_eq!(entry.name, "return_on_invested_capital");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for term in [
            "Operating income over the duration times one less the effective tax rate",
            "income tax expense over pretax income",
            "shareholders' equity plus short-term and long-term debt less cash and \
             short-term investments",
            "the preferred claim included",
            "cash and short-term investments netted",
        ] {
            assert!(entry.meaning.contains(term), "{term}");
        }
        assert_eq!(
            entry.inputs,
            [
                Concept::OperatingIncome,
                Concept::IncomeTaxExpense,
                Concept::PretaxIncome,
                Concept::ShareholdersEquity,
                Concept::ShortTermDebt,
                Concept::LongTermDebt,
                Concept::CashAndEquivalents,
                Concept::ShortTermInvestments
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_pretax_income",
                "tax_rate_outside_zero_to_one",
                "zero_invested_capital",
                "start_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
