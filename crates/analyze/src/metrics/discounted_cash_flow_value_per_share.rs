//! The discounted cash flow value per share: a year's free cash flow grown
//! over the forecast years and then for ever, discounted to the year's close,
//! less the average of net debt at the instants the year opens and closes at,
//! per diluted share. Its eight steps, its settings, its conditions and how
//! its absence composes are
//! `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s, under the method
//! version.
//!
//! It is stated over a year, so at a duration that is not one it gives the
//! year test's one reason and makes no read. The base is the computing year's
//! flow alone, and no other year is read. Free cash flow is formed in
//! [`super::free_cash_flow`] and net debt in [`super::capital`], with
//! short-term investments the one term that contributes nothing where they
//! are `NotApplicable`. Neither the preferred claim nor non-controlling
//! interests is subtracted. A negative equity value is a figure with its
//! sign.
//!
//! Source: Damodaran, A., *Valuation Approaches and Metrics: A Survey of the
//! Theory and Evidence*, Stern School of Business, New York University,
//! November 2006,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>.
//! Page 28: "The value of the firm, in the most general case, can be written
//! as the present value of expected free cashflows to the firm." The equation
//! that follows is set as an object the page's text does not carry: the sum
//! over the years of each year's flow over one plus the discount rate to the
//! power of the year. The same page: "If the firm reaches steady state after
//! n years and starts growing at a stable growth rate gn after that, the value
//! of the firm can be written as:", and the equation, carried the same way,
//! is the sum of each of the n years' flows discounted so, plus year n + 1's
//! flow over the discount rate less the stable growth rate, discounted by one
//! plus the discount rate to the power n. Year n + 1's flow is year n's grown
//! once at the terminal growth rate. Page 27, of the stable growth rate: "the
//! growth rate used in the model has to be less than or equal to the growth
//! rate in the economy". Page 29, the bridge: "To get from the value of
//! operating assets to the value of equity, we have to first incorporate the
//! value of non-operating assets that are owned by the firm and then subtract
//! out all non-equity claims that may be outstanding against the firm." and
//! "The most common of the non-operating assets is cash and marketable
//! securities".
//!
//! The flow is free cash flow, and not the flow to the firm, which page 29
//! calls "a pre-debt cashflow". Operating cash flow is after interest where
//! the filer classifies it in operations, as United States GAAP does
//! (Financial Accounting Standards Board, ASC Topic 230, *Statement of Cash
//! Flows*, not read from the workspace), and the bridge then subtracts the
//! net debt that bears it, so net interest is counted twice. For a filer that
//! pays more interest than it receives the value is understated by the
//! present value of its net interest; for one whose cash earns more than its
//! debt costs it is overstated by the same.
//!
//! For a `bank`, an `insurer` or an `investment_company` the figure is
//! computed and is not a valuation the source would make, whose page 15 says
//! "estimating capital expenditures and working capital for a bank, an
//! investment bank or an insurance company is difficult to do."

use std::cmp::Ordering;
use std::iter;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::capital::{NOTHING_WHERE_NOT_APPLICABLE, net_debt};
use super::free_cash_flow::free_cash_flow;
use super::reading::reads_with_nothing;
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// Free cash flow over the year is zero or below. Grown at a positive rate an
/// outflow is a larger outflow for ever, and a zero flow is worth nothing, so
/// neither is a valuation.
const NON_POSITIVE_FREE_CASH_FLOW: &str = "non_positive_free_cash_flow";

/// The discount rate less the terminal growth rate is zero or below. At zero
/// the perpetuity divides by zero, and below it the perpetuity has no finite
/// sum.
const DISCOUNT_RATE_NOT_ABOVE_TERMINAL_GROWTH: &str = "discount_rate_not_above_terminal_growth";

/// The diluted shares weighted over the year are zero or below. Zero is a
/// zero divisor, and a negative count is no count and would turn the value's
/// sign.
const NON_POSITIVE_DILUTED_SHARES: &str = "non_positive_diluted_shares";

pub(super) const DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "discounted_cash_flow_value_per_share",
        meaning: "The equity value per diluted share: free cash flow, operating cash flow less \
                  capital expenditure, over a year, grown for the forecast years at the forecast \
                  growth rate and then for ever at the terminal growth rate, discounted at the \
                  discount rate, less the average of net debt at the instants the year opens \
                  and closes at, per diluted share weighted over the year; the preferred claim \
                  and non-controlling interests are not subtracted. It records the four \
                  settings it is computed under: discounted_cash_flow_discount_rate, \
                  discounted_cash_flow_growth_rate, discounted_cash_flow_years and \
                  discounted_cash_flow_terminal_growth_rate.",
        unit: Unit::CurrencyPerShare,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::OperatingCashFlow,
            Concept::CapitalExpenditure,
            Concept::ShortTermDebt,
            Concept::LongTermDebt,
            Concept::CashAndEquivalents,
            Concept::ShortTermInvestments,
            Concept::DilutedSharesWeightedAverage,
        ],
        conditions: &[
            NON_POSITIVE_FREE_CASH_FLOW,
            DISCOUNT_RATE_NOT_ABOVE_TERMINAL_GROWTH,
            NON_POSITIVE_DILUTED_SHARES,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
        ],
    },
    readings: &[
        None,
        None,
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        None,
    ],
    derived: discounted_cash_flow_value_per_share,
};

/// The year test, then every read, then the three conditions in formula
/// order, then the eight steps.
fn discounted_cash_flow_value_per_share(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let [
        operating_cash_flow,
        capital_expenditure,
        short_term_debt,
        long_term_debt,
        cash_and_equivalents,
        short_term_investments,
        diluted_shares_weighted_average,
    ] = reads_with_nothing(
        computing,
        &DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE,
        NOTHING_WHERE_NOT_APPLICABLE,
    )?;
    let settings = computing.settings();
    let discount_rate = settings.discounted_cash_flow_discount_rate();
    let terminal_growth_rate = settings.discounted_cash_flow_terminal_growth_rate();

    let base = free_cash_flow(&operating_cash_flow.mean(), &capital_expenditure.mean());
    if base.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_FREE_CASH_FLOW));
    }
    let spread = discount_rate.minus(terminal_growth_rate);
    if spread.sign() != Ordering::Greater {
        return Err(declined(DISCOUNT_RATE_NOT_ABOVE_TERMINAL_GROWTH));
    }
    let diluted_shares = diluted_shares_weighted_average.mean();
    if diluted_shares.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_DILUTED_SHARES));
    }

    let growth = Number::one().plus(settings.discounted_cash_flow_growth_rate());
    let discount = Number::one().plus(discount_rate);
    let present = |flow: &Number, discounting: &Number| {
        flow.divided(discounting)
            .expect("one plus a discount rate above zero is above one")
    };

    // Each pass is year t, from one to n: (1 + g)ᵗ and (1 + r)ᵗ as the running
    // product of the year before's, and the sum of Fₜ ÷ (1 + r)ᵗ so far.
    let (grown, discounting, forecast) =
        iter::repeat_n((), settings.discounted_cash_flow_years().get()).fold(
            (Number::one(), Number::one(), Number::zero()),
            |(grown, discounting, forecast), ()| {
                let grown = grown.times(&growth);
                let discounting = discounting.times(&discount);
                let forecast = forecast.plus(&present(&base.times(&grown), &discounting));
                (grown, discounting, forecast)
            },
        );
    let terminal_value = base
        .times(&grown)
        .times(&Number::one().plus(terminal_growth_rate))
        .divided(&spread)
        .expect("the discount rate is above the terminal growth rate");
    let value_of_operations = forecast.plus(&present(&terminal_value, &discounting));

    let equity_value = value_of_operations.minus(
        &net_debt(
            &short_term_debt,
            &long_term_debt,
            &cash_and_equivalents,
            &short_term_investments,
        )
        .mean(),
    );
    Ok(equity_value
        .divided(&diluted_shares)
        .expect("the diluted shares are above zero"))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DISCOUNTED_CASH_FLOW_VALUE_PER_SHARE.entry;
        assert_eq!(entry.name, "discounted_cash_flow_value_per_share");
        assert!(!entry.name.contains("common"));
        assert_eq!(entry.unit, Unit::CurrencyPerShare);
        assert_eq!(entry.stated_at, Shape::Duration);
        for term in [
            "free cash flow, operating cash flow less capital expenditure, over a year, grown \
             for the forecast years at the forecast growth rate and then for ever at the \
             terminal growth rate, discounted at the discount rate, less the average of net \
             debt at the instants the year opens and closes at, per diluted share weighted \
             over the year; the preferred claim and non-controlling interests are not \
             subtracted",
            "discounted_cash_flow_discount_rate",
            "discounted_cash_flow_growth_rate",
            "discounted_cash_flow_years",
            "discounted_cash_flow_terminal_growth_rate",
        ] {
            assert!(entry.meaning.contains(term), "{term}");
        }
        assert_eq!(
            entry.inputs,
            [
                Concept::OperatingCashFlow,
                Concept::CapitalExpenditure,
                Concept::ShortTermDebt,
                Concept::LongTermDebt,
                Concept::CashAndEquivalents,
                Concept::ShortTermInvestments,
                Concept::DilutedSharesWeightedAverage
            ]
        );
        for read in [Concept::PreferredEquity, Concept::SharesOutstanding] {
            assert!(!entry.inputs.contains(&read), "{read:?}");
        }
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_free_cash_flow",
                "discount_rate_not_above_terminal_growth",
                "non_positive_diluted_shares",
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
