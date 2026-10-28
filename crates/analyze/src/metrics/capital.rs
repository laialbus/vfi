//! The capital terms: capital employed, total debt, net debt, invested
//! capital, the effective tax rate and net operating profit after tax, each
//! formed once, here. Every metric that reads one reads it from this module,
//! and none writes any part of one itself. What each term is, is
//! `docs/adr/capital-employed-invested-capital-and-nopat.md`'s, under the
//! method version.
//!
//! Each term is formed at each row its inputs were read at, before any mean,
//! so an averaged term is the average of the term.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::declined;
use super::reading::Reads;
use crate::arithmetic::Number;

/// The average capital employed a metric divides by is zero, and no ratio to
/// zero is defined. Every metric over capital employed is undefined under
/// this one condition.
pub(super) const ZERO_CAPITAL_EMPLOYED: &str = "zero_capital_employed";

/// The average invested capital a metric divides by is zero, and no ratio to
/// zero is defined. Every metric over invested capital is undefined under
/// this one condition.
pub(super) const ZERO_INVESTED_CAPITAL: &str = "zero_invested_capital";

/// Pretax income is zero or negative: there is no profit for the effective
/// tax rate to be a share of.
pub(super) const NON_POSITIVE_PRETAX_INCOME: &str = "non_positive_pretax_income";

/// Pretax income is positive and the effective tax rate is below zero or
/// above one, so the period's charge is not a share of its profit.
pub(super) const TAX_RATE_OUTSIDE_ZERO_TO_ONE: &str = "tax_rate_outside_zero_to_one";

/// The one concept a `NotApplicable` lets contribute nothing, and only inside
/// net debt: short-term investments, which apply to `operating` filers alone.
/// A metric reads it so only where it reads it for net debt. At an instant
/// where it is `NotApplicable` it is summed as nothing, and the instant is
/// still counted. The exception is
/// `docs/adr/canonical-concepts-open-questions.md`'s.
pub(super) const NOTHING_WHERE_NOT_APPLICABLE: Concept = Concept::ShortTermInvestments;

/// Capital employed: total assets less current liabilities, at each instant
/// both were read at. It is the long-term funding of the business, total
/// equity and every non-current liability.
///
/// Source: Atrill, P., and McLaney, E., *Accounting and Finance for
/// Non-Specialists*, 9th ed., Pearson, 2015, the chapter "Analysing and
/// interpreting financial statements", not read from the workspace, which
/// the record reads as stating return on capital employed as operating profit
/// over share capital, reserves and non-current liabilities, a sum equal to
/// total assets less current liabilities.
pub(super) fn capital_employed(total_assets: &Reads, current_liabilities: &Reads) -> Reads {
    total_assets.with(current_liabilities, Number::minus)
}

/// Total debt: short-term debt plus long-term debt, at each instant both were
/// read at.
pub(super) fn total_debt(short_term_debt: &Reads, long_term_debt: &Reads) -> Reads {
    short_term_debt.with(long_term_debt, Number::plus)
}

/// Net debt: total debt less cash and equivalents and less short-term
/// investments, at each instant all four were read at.
///
/// Source: `docs/adr/canonical-concepts-open-questions.md`, which forms net
/// debt in analysis from both cash concepts beside the debt. Damodaran,
/// cited at [`invested_capital`], nets cash and names no marketable
/// securities beside it, so the second netting is that record's.
pub(super) fn net_debt(
    short_term_debt: &Reads,
    long_term_debt: &Reads,
    cash_and_equivalents: &Reads,
    short_term_investments: &Reads,
) -> Reads {
    total_debt(short_term_debt, long_term_debt)
        .with(cash_and_equivalents, Number::minus)
        .with(short_term_investments, Number::minus)
}

/// Invested capital: shareholders' equity plus net debt, at each instant all
/// five were read at. The preferred claim is inside, as shareholders' equity
/// carries it, and the cash whose income operating income excludes is
/// netted.
///
/// Source: Damodaran, A., "Return on Capital (ROC), Return on Invested
/// Capital (ROIC) and Return on Equity (ROE): Measurement and Implications",
/// Stern School of Business, New York University, 2007,
/// <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/returnmeasures.pdf>,
/// under "II. Invested Capital": "The reason we net out cash is to be
/// consistent with the use of operating income as our measure of earnings.
/// The interest income from cash is not part of operating income."; and Table
/// 3, the row for Return on Invested Capital, "Measured as" "EBIT (1-t)/ (BV
/// of Debt + BV of Equity – Cash)".
pub(super) fn invested_capital(
    shareholders_equity: &Reads,
    short_term_debt: &Reads,
    long_term_debt: &Reads,
    cash_and_equivalents: &Reads,
    short_term_investments: &Reads,
) -> Reads {
    shareholders_equity.with(
        &net_debt(
            short_term_debt,
            long_term_debt,
            cash_and_equivalents,
            short_term_investments,
        ),
        Number::plus,
    )
}

/// The effective tax rate: income tax expense over pretax income, over one
/// duration; or the one condition under which it is no share of a profit.
/// Pretax income at or below zero declines under
/// `NON_POSITIVE_PRETAX_INCOME`, and otherwise a rate below zero or above one,
/// the bounds inclusive, under `TAX_RATE_OUTSIDE_ZERO_TO_ONE`. A rate outside
/// the bounds is never clamped and no statutory rate stands in for it.
///
/// Sources: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
/// *International Financial Statement Analysis*, 3rd ed., CFA Institute
/// Investment Series, Wiley, 2015, the chapter "Income Taxes", not read from
/// the workspace; and Financial Accounting Standards Board, ASC 740-10-50-12,
/// not read from the workspace. Beside them, Securities and Exchange
/// Commission, Regulation S-X, Rule 4-08(h)(2), 17 CFR 210.4-08(h)(2),
/// <https://www.law.cornell.edu/cfr/text/17/210.4-08>, whose first sentence
/// reads: "In the reconciliation between the amount of reported total income
/// tax expense (benefit) and the amount computed by multiplying the income
/// (loss) before tax by the applicable statutory Federal income tax rate, if
/// no individual reconciling item amounts to more than five percent of the
/// amount computed by multiplying the income before tax by the applicable
/// statutory Federal income tax rate, and the total difference to be
/// reconciled is less than five percent of such computed amount, no
/// reconciliation need be provided unless it would be significant in
/// appraising the trend of earnings." The rate is the filer's own two
/// figures, and no third number enters.
pub(super) fn effective_tax_rate(
    income_tax_expense: &Number,
    pretax_income: &Number,
) -> Result<Number, Reasons> {
    if pretax_income.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_PRETAX_INCOME));
    }
    let rate = income_tax_expense
        .divided(pretax_income)
        .expect("pretax income is positive");
    if rate.sign() == Ordering::Less || Number::one().minus(&rate).sign() == Ordering::Less {
        return Err(declined(TAX_RATE_OUTSIDE_ZERO_TO_ONE));
    }
    Ok(rate)
}

/// Net operating profit after tax: operating income times one less the
/// effective tax rate, over one duration.
///
/// Source: Damodaran 2007, cited at [`invested_capital`], under "a. Return on
/// Invested Capital", introduced by "In practice, it is usually defined as
/// follows", whose formula, its layout flattened, reads "Return on Capital
/// (ROIC) = Operating Income t (1 - tax rate) Book Value of Invested Capital
/// t - 1", the last term its divisor.
pub(super) fn net_operating_profit_after_tax(
    operating_income: &Number,
    effective_tax_rate: &Number,
) -> Number {
    operating_income.times(&Number::one().minus(effective_tax_rate))
}
