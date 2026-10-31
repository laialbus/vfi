//! Piotroski's signals, each formed once, here. Every metric that states a
//! signal reaches it through this module, and no metric forms one itself.
//! Each signal's form, benchmark and comparison, the instant each term reads,
//! its conditions and how its absence composes are
//! `docs/adr/piotroski-signals-and-score.md`'s, the year each is stated over
//! `docs/adr/which-duration-precedes-a-duration.md`'s, and the opening
//! balance `docs/adr/which-other-period-a-metric-reads.md`'s, under the method
//! version.
//!
//! Each signal is a function from its numbers to one or zero, or to its one
//! condition. A term two signals compare, such as ROA, is formed by one
//! function both call. Every benchmark is zero, and every figure is
//! `Number::one` or `Number::zero`, so the derivation needs no other literal.
//! A negative figure at any term is a value that gives its comparison, never
//! a condition, and nothing is substituted for any input.
//!
//! Origin, named as `docs/adr/which-other-period-a-metric-reads.md` names it
//! and not read from the workspace: Piotroski, J. D., "Value Investing: The
//! Use of Historical Financial Statement Information to Separate Winners from
//! Losers", *Journal of Accounting Research* 38, Supplement, 2000, pp. 1–41.
//!
//! The open copy beside it: *Selected Paper 84*, The University of Chicago
//! Graduate School of Business, 2002, read from
//! <https://www.chicagobooth.edu/~/media/FE874EE65F624AAEBD0166B1974FD74D.pdf>.
//! The section numbers below are this copy's, and its printed page numbers.
//! §2.3, p. 6: "I chose nine fundamental signals to measure three areas of the
//! firm's financial condition: profitability, financial leverage/liquidity,
//! and operating efficiency." and "An indicator variable for the signal is
//! equal to one (zero) if the signal's realization is good (bad)." §2.3.1,
//! p. 7: "I define ROA and CFO as net income before extraordinary items and
//! cash flow from operations, respectively, scaled by beginning of the year
//! total assets. If the firm's ROA (CFO) is positive, I define the indicator
//! variable F_ROA (F_CFO) equal to one, zero otherwise." and "I define the
//! variable ACCRUAL as current year's net income before extraordinary items
//! less cash flow from operations, scaled by beginning of the year total
//! assets. The indicator variable F_ACCRUAL equals one if CFO > ROA, zero
//! otherwise."
//!
//! Beside the zero benchmark, footnote 3, p. 7: "The benchmarks of zero
//! profits and zero cash flow from operations were chosen for two reasons.
//! First, a substantial portion of high BM firms (41.6%) experience a loss in
//! the prior two fiscal years; therefore, positive earnings realizations are
//! nontrivial events for these firms. Second, this is an easy benchmark to
//! implement since it does not rely on industry, market-level, or
//! time-specific comparisons."

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;

use super::reading::year::year;
use super::{Computing, Derivation, declined, read};
use crate::arithmetic::Number;

/// Total assets at the instant the year opens at are zero, and no term scaled
/// by them is defined. It names that instant, and not the year's close.
pub(super) const ZERO_OPENING_TOTAL_ASSETS: &str = "zero_opening_total_assets";

/// The numbers `derivation`'s inputs state at the year `computing` is at, each
/// read as the derivation names; or the year test's one reason, and no read
/// made, where the duration is not a year; or a reason for each read that
/// states none, in read order.
pub(super) fn over_a_year<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
) -> Result<[Number; N], Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    read(computing, derivation)
}

/// F_ROA: one where ROA is above zero, and zero otherwise.
pub(super) fn f_roa(net_income: &Number, opening_total_assets: &Number) -> Result<Number, Reasons> {
    Ok(indicator(
        roa(net_income, opening_total_assets)?.sign() == Ordering::Greater,
    ))
}

/// F_CFO: one where CFO is above zero, and zero otherwise.
pub(super) fn f_cfo(
    operating_cash_flow: &Number,
    opening_total_assets: &Number,
) -> Result<Number, Reasons> {
    Ok(indicator(
        cfo(operating_cash_flow, opening_total_assets)?.sign() == Ordering::Greater,
    ))
}

/// F_ACCRUAL: one where CFO is above ROA, and zero otherwise. The two scaled
/// terms are compared, as the copy states it, so no branch on the divisor's
/// sign is written: where the divisor is below zero the comparison of the
/// unscaled flows reverses, and the scaled terms carry that.
pub(super) fn f_accrual(
    net_income: &Number,
    operating_cash_flow: &Number,
    opening_total_assets: &Number,
) -> Result<Number, Reasons> {
    let roa = roa(net_income, opening_total_assets)?;
    let cfo = cfo(operating_cash_flow, opening_total_assets)?;
    Ok(indicator(cfo > roa))
}

/// ROA: net income over a year, divided by total assets at the instant it
/// opens at.
fn roa(net_income: &Number, opening_total_assets: &Number) -> Result<Number, Reasons> {
    scaled(net_income, opening_total_assets)
}

/// CFO: cash flow from operations over a year, divided by total assets at the
/// instant it opens at.
fn cfo(operating_cash_flow: &Number, opening_total_assets: &Number) -> Result<Number, Reasons> {
    scaled(operating_cash_flow, opening_total_assets)
}

/// A flow over a year scaled by total assets at the instant it opens at, or
/// `ZERO_OPENING_TOTAL_ASSETS` where they are zero.
fn scaled(flow: &Number, opening_total_assets: &Number) -> Result<Number, Reasons> {
    flow.divided(opening_total_assets)
        .ok_or_else(|| declined(ZERO_OPENING_TOTAL_ASSETS))
}

/// One where the signal's realization is good, and zero where it is not.
fn indicator(good: bool) -> Number {
    if good { Number::one() } else { Number::zero() }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};

    use super::{ZERO_OPENING_TOTAL_ASSETS, f_accrual, f_cfo, f_roa};
    use crate::arithmetic::Number;

    fn number(characters: &str) -> Number {
        Number::read(characters).expect("a decimal")
    }

    fn signal(characters: &str) -> Result<Number, Reasons> {
        Ok(number(characters))
    }

    fn zero_opening() -> Result<Number, Reasons> {
        Err(Reasons::of(
            Reason::declined(ZERO_OPENING_TOTAL_ASSETS.into()),
            Vec::new(),
        ))
    }

    #[test]
    fn the_condition_is_spelled_as_the_record_spells_it() {
        assert_eq!(ZERO_OPENING_TOTAL_ASSETS, "zero_opening_total_assets");
        assert_ne!(ZERO_OPENING_TOTAL_ASSETS, "zero_total_assets");
    }

    #[test]
    fn each_signal_is_one_only_on_a_strict_good_realization() {
        let assets = number("1000");
        assert_eq!(f_roa(&number("60"), &assets), signal("1"));
        assert_eq!(f_roa(&number("0"), &assets), signal("0"));
        assert_eq!(f_roa(&number("-30"), &assets), signal("0"));
        assert_eq!(f_cfo(&number("90"), &assets), signal("1"));
        assert_eq!(f_cfo(&number("0"), &assets), signal("0"));
        assert_eq!(f_cfo(&number("-10"), &assets), signal("0"));
        assert_eq!(
            f_accrual(&number("60"), &number("90"), &assets),
            signal("1")
        );
        assert_eq!(
            f_accrual(&number("60"), &number("60"), &assets),
            signal("0")
        );
        assert_eq!(
            f_accrual(&number("-30"), &number("-10"), &assets),
            signal("1")
        );
    }

    #[test]
    fn a_negative_divisor_reverses_the_accrual_through_the_scaled_terms() {
        let assets = number("-1000");
        assert_eq!(f_roa(&number("60"), &assets), signal("0"));
        assert_eq!(f_cfo(&number("-90"), &assets), signal("1"));
        assert_eq!(
            f_accrual(&number("60"), &number("90"), &assets),
            signal("0")
        );
        assert_eq!(
            f_accrual(&number("90"), &number("60"), &assets),
            signal("1")
        );
    }

    #[test]
    fn a_zero_opening_balance_declines_each_signal() {
        let zero = Number::zero();
        assert_eq!(f_roa(&number("60"), &zero), zero_opening());
        assert_eq!(f_cfo(&number("90"), &zero), zero_opening());
        assert_eq!(
            f_accrual(&number("60"), &number("90"), &zero),
            zero_opening()
        );
    }
}
