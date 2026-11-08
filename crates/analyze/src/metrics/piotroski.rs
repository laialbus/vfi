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
//! otherwise." and "I define ΔROA as the current year's ROA less the prior
//! year's ROA. If ΔROA > 0, the indicator variable F_ΔROA equals one, zero
//! otherwise." §2.3.2, pp. 7–8: "I assume that an increase in leverage, a
//! deterioration of liquidity, or the use of external financing is a bad
//! signal about financial risk.", "I measure ΔLEVER as the historical change
//! in the ratio of total long-term debt to average total assets, and view an
//! increase (decrease) in financial leverage as a negative (positive)
//! signal.", "I define the indicator variable F_ΔLEVER to equal one (zero) if
//! the firm's leverage ratio fell (rose) in the year preceding portfolio
//! formation.", "where I define the current ratio as the ratio of current
//! assets to current liabilities at fiscal year-end. I assume that an
//! improvement in liquidity (i.e., ΔLIQUID > 0) is a good signal", "The
//! indicator variable F_ΔLIQUID equals one if the firm's liquidity improved,
//! zero otherwise." and "I define the indicator variable EQ_OFFER to equal one if the firm did not issue common equity in the year
//! preceding portfolio formation, zero otherwise." §2.3.3, p. 8: "I define
//! ΔMARGIN as the firm's current gross margin ratio (gross margin scaled by
//! total sales) less the prior year's gross margin ratio.", "The indicator
//! variable F_ΔMARGIN equals one if ΔMARGIN is positive, zero otherwise.",
//! "I define ΔTURN as the firm's current year asset turnover ratio (total
//! sales scaled by beginning of the year total assets) less the prior year's
//! asset turnover ratio." and "The indicator variable F_ΔTURN equals one if
//! ΔTURN is positive, zero otherwise."
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
use vfi_contracts::canonical_concepts::Concept;

use super::reading::year::year;
use super::reading::{
    Reads, read_averaged_with_the_year_before, read_with_the_year_before,
    reads_once_with_the_year_before,
};
use super::{
    Computing, Derivation, ZERO_AVERAGE_TOTAL_ASSETS, ZERO_CURRENT_LIABILITIES, ZERO_REVENUE,
    declined, read,
};
use crate::arithmetic::Number;
use crate::constants::piotroski::PIOTROSKI_SIGNALS;

/// Total assets at the instant the year opens at are zero, and no term scaled
/// by them is defined. It names that instant, and not the year's close.
pub(super) const ZERO_OPENING_TOTAL_ASSETS: &str = "zero_opening_total_assets";

/// Total assets at the instant the year before opens at are zero, and no term
/// of the year before scaled by them is defined.
pub(super) const ZERO_PRECEDING_OPENING_TOTAL_ASSETS: &str = "zero_preceding_opening_total_assets";

/// Revenue over the year before is zero, and no fraction of it is defined.
pub(super) const ZERO_PRECEDING_REVENUE: &str = "zero_preceding_revenue";

/// The average of total assets at the instants the year before opens and
/// closes at is zero, and the year before's leverage is not defined.
pub(super) const ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS: &str = "zero_preceding_average_total_assets";

/// Current liabilities at the instant the year before closes at are zero, and
/// the year before's current ratio is not defined.
pub(super) const ZERO_PRECEDING_CURRENT_LIABILITIES: &str = "zero_preceding_current_liabilities";

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

/// The numbers `derivation`'s inputs state at the year `computing` is at,
/// then at the year before it, each read as the derivation names; or the
/// year test's one reason, and no read made, where the duration is not a
/// year; or a reason for each read that states none, the year's reads before
/// the year before's.
pub(super) fn over_a_year_and_the_year_before<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
) -> Result<([Number; N], [Number; N]), Reasons> {
    let year = year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    read_with_the_year_before(&year, derivation)
}

/// As [`over_a_year_and_the_year_before`], for a derivation that averages a
/// balance at each year: the balance at the instant the year opens at is read
/// once, and stands in the year's mean and in the year before's.
pub(super) fn over_a_year_and_the_year_before_averaged<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
) -> Result<([Number; N], [Number; N]), Reasons> {
    let year = year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    read_averaged_with_the_year_before(&year, derivation)
}

/// What each of `derivation`'s inputs' reads give at the year `computing` is
/// at, then at the year before it, each read once and given read by read as
/// [`reads_once_with_the_year_before`] gives them, `this_year_alone` read at
/// the year alone; or the year test's one reason, and no read made, where the
/// duration is not a year; or a reason for each read that states none, the
/// year's reads before the year before's.
pub(super) fn over_a_year_and_the_year_before_read_once<const N: usize, const M: usize>(
    computing: &Computing,
    derivation: &Derivation,
    this_year_alone: Concept,
) -> Result<([Reads; N], [Reads; M]), Reasons> {
    let year = year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    reads_once_with_the_year_before(&year, derivation, this_year_alone)
}

/// F_ROA: one where ROA is above zero, and zero otherwise.
pub(super) fn f_roa(net_income: &Number, opening_total_assets: &Number) -> Result<Number, Reasons> {
    let roa = defined(
        roa(net_income, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    Ok(indicator(roa.sign() == Ordering::Greater))
}

/// F_ΔROA: one where ΔROA, the year's ROA less the year before's, is above
/// zero, and zero otherwise.
pub(super) fn f_delta_roa(
    net_income: &Number,
    opening_total_assets: &Number,
    preceding_net_income: &Number,
    preceding_opening_total_assets: &Number,
) -> Result<Number, Reasons> {
    let roa_now = defined(
        roa(net_income, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    let roa_before = defined(
        roa(preceding_net_income, preceding_opening_total_assets),
        ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
    )?;
    Ok(rose(&roa_now, &roa_before))
}

/// F_CFO: one where CFO is above zero, and zero otherwise.
pub(super) fn f_cfo(
    operating_cash_flow: &Number,
    opening_total_assets: &Number,
) -> Result<Number, Reasons> {
    let cfo = defined(
        cfo(operating_cash_flow, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    Ok(indicator(cfo.sign() == Ordering::Greater))
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
    let roa = defined(
        roa(net_income, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    let cfo = defined(
        cfo(operating_cash_flow, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    Ok(indicator(cfo > roa))
}

/// F_ΔMARGIN: one where ΔMARGIN, the year's gross margin less the year
/// before's, is above zero, and zero otherwise.
pub(super) fn f_delta_margin(
    gross_profit: &Number,
    revenue: &Number,
    preceding_gross_profit: &Number,
    preceding_revenue: &Number,
) -> Result<Number, Reasons> {
    let margin_now = defined(margin(gross_profit, revenue), ZERO_REVENUE)?;
    let margin_before = defined(
        margin(preceding_gross_profit, preceding_revenue),
        ZERO_PRECEDING_REVENUE,
    )?;
    Ok(rose(&margin_now, &margin_before))
}

/// F_ΔTURN: one where ΔTURN, the year's asset turnover less the year
/// before's, is above zero, and zero otherwise.
///
/// Each year's turnover is scaled by total assets at the instant that year
/// opens at, as §2.3.3 states it, and not by the average of Table 1
/// (continued), "Variable definitions", p. 13: "The asset turnover ratio is
/// defined as net sales scaled by average total assets for the year." The
/// record records that statement and follows the body against it.
pub(super) fn f_delta_turn(
    revenue: &Number,
    opening_total_assets: &Number,
    preceding_revenue: &Number,
    preceding_opening_total_assets: &Number,
) -> Result<Number, Reasons> {
    let turnover_now = defined(
        turnover(revenue, opening_total_assets),
        ZERO_OPENING_TOTAL_ASSETS,
    )?;
    let turnover_before = defined(
        turnover(preceding_revenue, preceding_opening_total_assets),
        ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
    )?;
    Ok(rose(&turnover_now, &turnover_before))
}

/// F_ΔLEVER: one where ΔLEVER, the year's leverage less the year before's, is
/// below zero, and zero otherwise.
///
/// An unchanged ratio gives zero. §2.3.2 says "fell (rose)" and names no third
/// case, and the record decides it for three reasons. Every other signal gives
/// its one only on a strict move in the good direction. §2.3, p. 6, "An
/// indicator variable for the signal is equal to one (zero) if the signal's
/// realization is good (bad).", and an unchanged ratio is not an improvement.
/// And it never awards a point the copy does not state.
///
/// Table 1 (continued), "Variable definitions", p. 13, states a term
/// `canonical-concepts` v2 cannot carry: "The debt-to-asset ratio is defined
/// as the firm's total long-term debt (including the portion of long-term debt
/// classified as current) scaled by average total assets." `long_term_debt`
/// excludes the current portion, so reading it alone flatters a filer that
/// reclassifies debt to current, whose leverage falls where the copy's term is
/// unchanged. `short_term_debt` cannot stand in: it holds the current portion
/// together with notes payable, commercial paper and current finance lease
/// obligations, so adding it would move the ratio on short-term funding the
/// copy's term excludes.
pub(super) fn f_delta_lever(
    long_term_debt: &Number,
    average_total_assets: &Number,
    preceding_long_term_debt: &Number,
    preceding_average_total_assets: &Number,
) -> Result<Number, Reasons> {
    let leverage_now = defined(
        leverage(long_term_debt, average_total_assets),
        ZERO_AVERAGE_TOTAL_ASSETS,
    )?;
    let leverage_before = defined(
        leverage(preceding_long_term_debt, preceding_average_total_assets),
        ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
    )?;
    Ok(fell(&leverage_now, &leverage_before))
}

/// F_ΔLIQUID: one where ΔLIQUID, the current ratio at the year's close less
/// the same at the year before's, is above zero, and zero otherwise.
pub(super) fn f_delta_liquid(
    current_assets: &Number,
    current_liabilities: &Number,
    preceding_current_assets: &Number,
    preceding_current_liabilities: &Number,
) -> Result<Number, Reasons> {
    let ratio_now = defined(
        current_ratio(current_assets, current_liabilities),
        ZERO_CURRENT_LIABILITIES,
    )?;
    let ratio_before = defined(
        current_ratio(preceding_current_assets, preceding_current_liabilities),
        ZERO_PRECEDING_CURRENT_LIABILITIES,
    )?;
    Ok(rose(&ratio_now, &ratio_before))
}

/// EQ_OFFER: one where the diluted weighted average share count over the
/// year is not above the same over the year before, and zero where it is
/// above. It divides nothing and has no condition.
///
/// Reading "did not issue common equity" through the weighted count is the
/// record's own decision, pending a concept for common equity issued. The
/// count cannot see an issue offset by a repurchase, or a fall from a loss
/// year, and each flatters. It reads an issue where none was made where
/// dilutive securities enter the count, where an issue late in the year
/// before is carried fully into the year, and where a split is not restated
/// in the year before's count.
pub(super) fn eq_offer(
    diluted_shares_weighted_average: &Number,
    preceding_diluted_shares_weighted_average: &Number,
) -> Number {
    indicator(diluted_shares_weighted_average <= preceding_diluted_shares_weighted_average)
}

/// F_SCORE: the count of `signals` that are one, the nine in §2.3.4's order,
/// from zero to `PIOTROSKI_SIGNALS`; or, where a signal is undefined, the one
/// condition the first such signal gives. The count starts at zero and steps
/// by one, and is never a fraction.
pub(super) fn f_score(
    signals: [Result<Number, Reasons>; PIOTROSKI_SIGNALS],
) -> Result<Number, Reasons> {
    if let Some(reasons) = signals.iter().find_map(|signal| signal.as_ref().err()) {
        return Err(reasons.clone());
    }
    let held: [bool; PIOTROSKI_SIGNALS] = signals.map(|signal| signal == Ok(Number::one()));
    Ok(held
        .iter()
        .filter(|held| **held)
        .fold(Number::zero(), |count, _| count.plus(&Number::one())))
}

/// ROA: net income over a year, divided by total assets at the instant it
/// opens at, or none where they are zero.
fn roa(net_income: &Number, opening_total_assets: &Number) -> Option<Number> {
    net_income.divided(opening_total_assets)
}

/// CFO: cash flow from operations over a year, divided by total assets at the
/// instant it opens at, or none where they are zero.
fn cfo(operating_cash_flow: &Number, opening_total_assets: &Number) -> Option<Number> {
    operating_cash_flow.divided(opening_total_assets)
}

/// Asset turnover: revenue over a year, divided by total assets at the
/// instant it opens at, or none where they are zero.
fn turnover(revenue: &Number, opening_total_assets: &Number) -> Option<Number> {
    revenue.divided(opening_total_assets)
}

/// Gross margin: gross profit over a year as a fraction of revenue over it,
/// or none where that revenue is zero.
fn margin(gross_profit: &Number, revenue: &Number) -> Option<Number> {
    gross_profit.divided(revenue)
}

/// Leverage: long-term debt at the instant a year closes at, divided by the
/// average of total assets at the instants it opens and closes at, or none
/// where that average is zero.
fn leverage(long_term_debt: &Number, average_total_assets: &Number) -> Option<Number> {
    long_term_debt.divided(average_total_assets)
}

/// The current ratio: current assets divided by current liabilities, each at
/// the instant a year closes at, or none where those liabilities are zero.
fn current_ratio(current_assets: &Number, current_liabilities: &Number) -> Option<Number> {
    current_assets.divided(current_liabilities)
}

/// `term`, or the one reason `condition` where it is undefined. A term is
/// formed alike at either year, and the condition names whose divisor is
/// zero.
fn defined(term: Option<Number>, condition: &'static str) -> Result<Number, Reasons> {
    term.ok_or_else(|| declined(condition))
}

/// One where `now` less `before` is above zero, and zero otherwise, so an
/// unchanged term gives zero.
fn rose(now: &Number, before: &Number) -> Number {
    indicator(now.minus(before).sign() == Ordering::Greater)
}

/// One where `now` less `before` is below zero, and zero otherwise, so an
/// unchanged term gives zero.
fn fell(now: &Number, before: &Number) -> Number {
    indicator(now.minus(before).sign() == Ordering::Less)
}

/// One where the signal's realization is good, and zero where it is not.
fn indicator(good: bool) -> Number {
    if good { Number::one() } else { Number::zero() }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};
    use vfi_contracts::canonical_concepts::{Concept, Period, Resolution, Row, Rule, SetBy};

    use super::{
        ZERO_OPENING_TOTAL_ASSETS, ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
        ZERO_PRECEDING_CURRENT_LIABILITIES, ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
        ZERO_PRECEDING_REVENUE, eq_offer, f_accrual, f_cfo, f_delta_lever, f_delta_liquid,
        f_delta_margin, f_delta_roa, f_delta_turn, f_roa, f_score,
    };
    use crate::arithmetic::Number;
    use crate::constants::piotroski::PIOTROSKI_SIGNALS;
    use crate::metrics::reading::Computing;
    use crate::metrics::{
        DERIVATIONS, Shape, Unit, ZERO_AVERAGE_TOTAL_ASSETS, ZERO_CURRENT_LIABILITIES, ZERO_REVENUE,
    };
    use crate::settings::Settings;

    fn number(characters: &str) -> Number {
        Number::read(characters).expect("a decimal")
    }

    fn signal(characters: &str) -> Result<Number, Reasons> {
        Ok(number(characters))
    }

    fn declined(condition: &str) -> Result<Number, Reasons> {
        Err(Reasons::of(Reason::declined(condition.into()), Vec::new()))
    }

    fn zero_opening() -> Result<Number, Reasons> {
        declined(ZERO_OPENING_TOTAL_ASSETS)
    }

    #[test]
    fn the_condition_is_spelled_as_the_record_spells_it() {
        assert_eq!(ZERO_OPENING_TOTAL_ASSETS, "zero_opening_total_assets");
        assert_ne!(ZERO_OPENING_TOTAL_ASSETS, "zero_total_assets");
    }

    #[test]
    fn the_year_befores_conditions_are_spelled_as_the_record_spells_them() {
        assert_eq!(
            ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
            "zero_preceding_opening_total_assets"
        );
        assert_eq!(ZERO_PRECEDING_REVENUE, "zero_preceding_revenue");
        assert_eq!(
            ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
            "zero_preceding_average_total_assets"
        );
        assert_eq!(
            ZERO_PRECEDING_CURRENT_LIABILITIES,
            "zero_preceding_current_liabilities"
        );
    }

    #[test]
    fn leverage_gives_one_only_where_the_ratio_fell() {
        let [debt_before, average_before, average] = ["270", "900", "1100"].map(number);
        assert_eq!(
            f_delta_lever(&number("220"), &average, &debt_before, &average_before),
            signal("1")
        );
        assert_eq!(
            f_delta_lever(&number("330"), &average, &debt_before, &average_before),
            signal("0")
        );
        assert_eq!(
            f_delta_lever(&number("340"), &average, &debt_before, &average_before),
            signal("0")
        );
        let zero = Number::zero();
        assert_eq!(
            f_delta_lever(&zero, &average, &zero, &average_before),
            signal("0")
        );
    }

    #[test]
    fn liquidity_gives_one_only_where_the_ratio_rose() {
        let [liabilities, assets_before] = ["200", "300"].map(number);
        assert_eq!(
            f_delta_liquid(&number("360"), &liabilities, &assets_before, &liabilities),
            signal("1")
        );
        assert_eq!(
            f_delta_liquid(&assets_before, &liabilities, &assets_before, &liabilities),
            signal("0")
        );
        assert_eq!(
            f_delta_liquid(&number("280"), &liabilities, &assets_before, &liabilities),
            signal("0")
        );
    }

    #[test]
    fn a_negative_figure_gives_its_balance_comparison() {
        let [average, average_before] = ["1100", "900"].map(number);
        assert_eq!(
            f_delta_lever(&number("-220"), &average, &number("270"), &average_before),
            signal("1")
        );
        assert_eq!(
            f_delta_lever(
                &number("220"),
                &number("-1100"),
                &number("270"),
                &average_before
            ),
            signal("1")
        );
        assert_eq!(
            f_delta_liquid(&number("-1"), &number("200"), &number("-2"), &number("200")),
            signal("1")
        );
    }

    #[test]
    fn each_balance_signal_asks_the_years_condition_first() {
        let zero = Number::zero();
        let [debt, average] = ["220", "1100"].map(number);
        assert_eq!(
            f_delta_lever(&debt, &zero, &debt, &average),
            declined(ZERO_AVERAGE_TOTAL_ASSETS)
        );
        assert_eq!(
            f_delta_lever(&debt, &average, &debt, &zero),
            declined(ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS)
        );
        assert_eq!(
            f_delta_lever(&debt, &zero, &debt, &zero),
            declined(ZERO_AVERAGE_TOTAL_ASSETS)
        );
        let [assets, liabilities] = ["360", "200"].map(number);
        assert_eq!(
            f_delta_liquid(&assets, &zero, &assets, &liabilities),
            declined(ZERO_CURRENT_LIABILITIES)
        );
        assert_eq!(
            f_delta_liquid(&assets, &liabilities, &assets, &zero),
            declined(ZERO_PRECEDING_CURRENT_LIABILITIES)
        );
        assert_eq!(
            f_delta_liquid(&assets, &zero, &assets, &zero),
            declined(ZERO_CURRENT_LIABILITIES)
        );
    }

    #[test]
    fn each_change_is_one_only_where_it_is_above_zero() {
        let [hundred, thousand, eight_hundred] = ["100", "1000", "800"].map(number);
        assert_eq!(
            f_delta_roa(&number("60"), &thousand, &number("40"), &eight_hundred),
            signal("1")
        );
        assert_eq!(
            f_delta_roa(&number("48"), &thousand, &number("40"), &eight_hundred),
            signal("0")
        );
        assert_eq!(
            f_delta_roa(&number("40"), &thousand, &number("40"), &eight_hundred),
            signal("0")
        );
        assert_eq!(
            f_delta_margin(
                &number("352"),
                &number("1100"),
                &number("240"),
                &eight_hundred
            ),
            signal("1")
        );
        assert_eq!(
            f_delta_margin(
                &number("330"),
                &number("1100"),
                &number("240"),
                &eight_hundred
            ),
            signal("0")
        );
        assert_eq!(
            f_delta_turn(&number("1100"), &thousand, &eight_hundred, &eight_hundred),
            signal("1")
        );
        assert_eq!(
            f_delta_turn(&thousand, &thousand, &eight_hundred, &eight_hundred),
            signal("0")
        );
        assert_eq!(
            f_delta_turn(&number("900"), &thousand, &eight_hundred, &eight_hundred),
            signal("0")
        );
        assert_eq!(eq_offer(&hundred, &hundred), Number::one());
        assert_eq!(eq_offer(&number("95"), &hundred), Number::one());
        assert_eq!(eq_offer(&number("101"), &hundred), Number::zero());
    }

    #[test]
    fn a_negative_figure_gives_its_comparison() {
        let [thousand, eight_hundred] = ["1000", "800"].map(number);
        assert_eq!(
            f_delta_roa(&number("-30"), &thousand, &number("-50"), &eight_hundred),
            signal("1")
        );
        assert_eq!(
            f_delta_roa(&number("60"), &thousand, &number("40"), &number("-1000")),
            signal("1")
        );
        assert_eq!(
            f_delta_turn(&number("1100"), &thousand, &eight_hundred, &number("-1000")),
            signal("1")
        );
        assert_eq!(
            f_delta_margin(
                &number("-10"),
                &number("100"),
                &number("-20"),
                &number("100")
            ),
            signal("1")
        );
    }

    #[test]
    fn the_years_condition_is_asked_before_the_year_befores() {
        let zero = Number::zero();
        let [thousand, eight_hundred] = ["1000", "800"].map(number);
        for f in [f_delta_roa, f_delta_turn] {
            assert_eq!(
                f(&number("60"), &zero, &number("40"), &eight_hundred),
                zero_opening()
            );
            assert_eq!(
                f(&number("60"), &thousand, &number("40"), &zero),
                declined(ZERO_PRECEDING_OPENING_TOTAL_ASSETS)
            );
            assert_eq!(
                f(&number("60"), &zero, &number("40"), &zero),
                zero_opening()
            );
        }
        assert_eq!(
            f_delta_margin(&zero, &zero, &number("240"), &eight_hundred),
            declined(ZERO_REVENUE)
        );
        assert_eq!(
            f_delta_margin(&number("352"), &number("1100"), &zero, &zero),
            declined(ZERO_PRECEDING_REVENUE)
        );
        assert_eq!(
            f_delta_margin(&zero, &zero, &zero, &zero),
            declined(ZERO_REVENUE)
        );
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

    #[test]
    fn the_score_counts_the_signals_that_are_one() {
        let one = || signal("1");
        let zero = || signal("0");
        assert_eq!(
            f_score([(); PIOTROSKI_SIGNALS].map(|()| one())),
            signal("9")
        );
        assert_eq!(
            f_score([(); PIOTROSKI_SIGNALS].map(|()| zero())),
            signal("0")
        );
        assert_eq!(
            f_score([
                one(),
                zero(),
                one(),
                one(),
                zero(),
                one(),
                one(),
                zero(),
                one()
            ]),
            signal("6")
        );
    }

    #[test]
    fn the_score_gives_the_first_condition_its_signals_give() {
        let one = || signal("1");
        assert_eq!(
            f_score([
                one(),
                one(),
                one(),
                one(),
                declined(ZERO_REVENUE),
                one(),
                one(),
                declined(ZERO_CURRENT_LIABILITIES),
                one()
            ]),
            declined(ZERO_REVENUE)
        );
    }

    #[test]
    fn the_count_is_nine_and_stands_in_constants() {
        assert_eq!(PIOTROSKI_SIGNALS, 9);
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

    fn calendar(year: &str) -> Period {
        Period::Duration {
            start: format!("{year}-01-01").into(),
            end: format!("{year}-12-31").into(),
        }
    }

    fn instant(at: &str) -> Period {
        Period::Instant { at: at.into() }
    }

    /// The record's worked history, the year 2024 first, with `change`'s
    /// concept at its period stated as its amount. Every concept the record
    /// does not name is `7`.
    fn worked(change: (Period, Concept, &str)) -> Vec<Row> {
        let stated: [(Period, &[(Concept, &str)]); 5] = [
            (
                calendar("2024"),
                &[
                    (Concept::NetIncome, "60"),
                    (Concept::OperatingCashFlow, "90"),
                    (Concept::GrossProfit, "352"),
                    (Concept::Revenue, "1100"),
                    (Concept::DilutedSharesWeightedAverage, "100"),
                ],
            ),
            (
                calendar("2023"),
                &[
                    (Concept::NetIncome, "40"),
                    (Concept::GrossProfit, "240"),
                    (Concept::Revenue, "800"),
                    (Concept::DilutedSharesWeightedAverage, "100"),
                ],
            ),
            (instant("2022-12-31"), &[(Concept::TotalAssets, "800")]),
            (
                instant("2023-12-31"),
                &[
                    (Concept::TotalAssets, "1000"),
                    (Concept::LongTermDebt, "270"),
                    (Concept::CurrentAssets, "300"),
                    (Concept::CurrentLiabilities, "200"),
                ],
            ),
            (
                instant("2024-12-31"),
                &[
                    (Concept::TotalAssets, "1200"),
                    (Concept::LongTermDebt, "220"),
                    (Concept::CurrentAssets, "360"),
                    (Concept::CurrentLiabilities, "200"),
                ],
            ),
        ];
        let (changed, changed_concept, changed_amount) = change;
        stated
            .into_iter()
            .map(|(period, given)| {
                let resolutions = std::array::from_fn(|at| {
                    let concept = Concept::ALL[at];
                    let amount = if period == changed && concept == changed_concept {
                        changed_amount
                    } else {
                        given
                            .iter()
                            .find(|(named, _)| *named == concept)
                            .map_or("7", |(_, amount)| amount)
                    };
                    value(amount)
                });
                Row::at(period, resolutions)
            })
            .collect()
    }

    #[test]
    fn every_piotroski_entry_is_a_count_of_signals() {
        let piotroski: Vec<&str> = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.name.starts_with("piotroski_"))
            .map(|derivation| {
                assert_eq!(
                    derivation.entry.unit,
                    Unit::Signals,
                    "{}",
                    derivation.entry.name
                );
                derivation.entry.name
            })
            .collect();
        assert_eq!(piotroski.len(), PIOTROSKI_SIGNALS + 1);
        assert!(piotroski.contains(&"piotroski_f_score"));
    }

    #[test]
    fn every_derivation_that_gives_one_of_the_eight_conditions_lists_it() {
        let cases = [
            (
                (instant("2023-12-31"), Concept::TotalAssets, "0"),
                ZERO_OPENING_TOTAL_ASSETS,
            ),
            (
                (instant("2022-12-31"), Concept::TotalAssets, "0"),
                ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
            ),
            ((calendar("2024"), Concept::Revenue, "0"), ZERO_REVENUE),
            (
                (calendar("2023"), Concept::Revenue, "0"),
                ZERO_PRECEDING_REVENUE,
            ),
            (
                (instant("2024-12-31"), Concept::TotalAssets, "-1000"),
                ZERO_AVERAGE_TOTAL_ASSETS,
            ),
            (
                (instant("2022-12-31"), Concept::TotalAssets, "-1000"),
                ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
            ),
            (
                (instant("2024-12-31"), Concept::CurrentLiabilities, "0"),
                ZERO_CURRENT_LIABILITIES,
            ),
            (
                (instant("2023-12-31"), Concept::CurrentLiabilities, "0"),
                ZERO_PRECEDING_CURRENT_LIABILITIES,
            ),
        ];
        let settings = Settings::preset();
        for (change, condition) in cases {
            let history = worked(change);
            let computing = Computing::at(&history[0], &history, &settings);
            let declined = Reason::declined(condition.into());
            let gave: Vec<&str> = DERIVATIONS
                .iter()
                .filter(|derivation| derivation.entry.stated_at == Shape::Duration)
                .filter(|derivation| {
                    (derivation.derived)(&computing)
                        .is_err_and(|reasons| reasons.each().any(|reason| *reason == declined))
                })
                .map(|derivation| {
                    assert!(
                        derivation.entry.conditions.contains(&condition),
                        "{} gives {condition} and does not list it",
                        derivation.entry.name
                    );
                    derivation.entry.name
                })
                .collect();
            assert!(gave.contains(&"piotroski_f_score"), "{condition}");
        }
    }
}
