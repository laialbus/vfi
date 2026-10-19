//! The capital terms: capital employed and total debt, each formed once, here.
//! Every metric that reads one reads it from this module, and none writes
//! either subtraction or sum itself. What each term is, and the terms left to
//! return on invested capital, is
//! `docs/adr/capital-employed-invested-capital-and-nopat.md`'s, under the
//! method version.
//!
//! Each term is formed at each row its inputs were read at, before any mean,
//! so an averaged term is the average of the term.

use super::reading::Reads;
use crate::arithmetic::Number;

/// The average capital employed a metric divides by is zero, and no ratio to
/// zero is defined. Every metric over capital employed is undefined under
/// this one condition.
pub(super) const ZERO_CAPITAL_EMPLOYED: &str = "zero_capital_employed";

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
