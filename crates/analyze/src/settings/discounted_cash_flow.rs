//! The discounted cash flow's four settings. Each one's meaning, unit, domain,
//! preset and the preset's reason is
//! `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s.
//!
//! A rate is handed as characters and read by the one reader, so a value
//! outside the reading grammar is outside the domain, as is one that the one
//! writer could not write back as itself.
//!
//! That the discount rate is above the terminal growth rate is not a domain.
//! Each constructor sees one value, so the metric asks it, as the `declined`
//! condition `discount_rate_not_above_terminal_growth`.

use std::cmp::Ordering;
use std::num::NonZeroUsize;

use super::{Refused, Settings, Written};
use crate::arithmetic::Number;

/// `discounted_cash_flow_discount_rate`: the annual rate each year's flow and
/// the terminal value are discounted at, the return required of the capital
/// the flow serves.
///
/// Unit: a rate per year, a fraction of one. Domain: [`ABOVE_ZERO`]. At zero
/// or below, a later flow is worth at least as much as an earlier one, which
/// is no required return.
///
/// Preset: [`DiscountRate::PRESET`], `0.0772`, the cost of capital of the
/// United States market outside financial firms. A filer valued at the preset
/// is asked to earn what the average non-financial firm's capital costs.
///
/// Source: Damodaran, A., *Cost of Equity and Capital (US)*, Stern School of
/// Business, New York University,
/// <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/wacc.html>:
/// "Date of Analysis: Data used is as of January 2026". The row "Total Market
/// (without financials)" gives "Cost of Capital" 7.72%. The row without
/// financials is taken because the discounted cash flow's form is the one
/// Damodaran says is hard to apply to financial firms, in *Valuation
/// Approaches and Metrics: A Survey of the Theory and Evidence*, Stern School
/// of Business, New York University, November 2006, as the record reads it;
/// not read from the workspace. The source republishes each January, and
/// moving the preset to a later figure is a preset change, not a version bump,
/// since every result records the value it used.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiscountRate(Number);

/// `discounted_cash_flow_growth_rate`: the annual rate the base flow grows at
/// in each forecast year.
///
/// Unit: a rate per year, a fraction of one. Domain: [`ABOVE_MINUS_ONE`]. At
/// minus one every later flow is zero, and below it the flows alternate in
/// sign. Neither is a rate of growth.
///
/// Preset: [`GrowthRate::PRESET`], `0.02`, the terminal growth rate's preset.
/// No source fixes it, so this is the record's own decision: "No source fixes
/// a filer's excess growth, and the filings state a past, not a forecast". A
/// preset above the terminal rate would credit every filer with growth it has
/// not shown, so the preset assumes none, and the forecast grows at the rate
/// the perpetuity does. A user who expects more growth sets it, and the
/// result records it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GrowthRate(Number);

/// `discounted_cash_flow_years`: how many years the forecast runs before the
/// perpetuity.
///
/// Unit: years. Domain: [`POSITIVE_INTEGERS`]. Zero years would leave the
/// forecast rate unread.
///
/// Preset: [`ForecastYears::PRESET`], five. No source fixes it, so this is
/// the record's own decision: a forecast of five years looks as far ahead as
/// `dividend_growth_years` looks back at its preset, so the forecast span a
/// user starts from is the span over which the history shows growth at its
/// preset. At the growth rate's preset the years change nothing. They matter
/// only once a user sets an excess growth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ForecastYears(NonZeroUsize);

/// `discounted_cash_flow_terminal_growth_rate`: the annual rate the flow grows
/// at for ever after the forecast.
///
/// Unit: a rate per year, a fraction of one. Domain: [`ABOVE_MINUS_ONE`], for
/// the growth rate's reason.
///
/// Preset: [`TerminalGrowthRate::PRESET`], `0.02`, the inflation the Federal
/// Reserve takes as its goal. A perpetuity growing at expected inflation grows
/// at no real rate for ever, which is below the nominal growth of any economy
/// that grows in real terms. The discount rate's preset is nominal, so the
/// growth beside it is nominal too.
///
/// Source: Federal Open Market Committee, *Statement on Longer-Run Goals and
/// Monetary Policy Strategy*,
/// <https://www.federalreserve.gov/monetarypolicy/files/FOMC_LongerRunGoals.pdf>,
/// "Adopted effective January 24, 2012; as reaffirmed effective January 27,
/// 2026": "The Committee reaffirms its judgment that inflation at the
/// rate of 2 percent, as measured by the annual change in the price index for
/// personal consumption expenditures, is most consistent over the longer run
/// with the Federal Reserve’s statutory maximum employment and price stability
/// mandates."
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalGrowthRate(Number);

/// The values `discounted_cash_flow_discount_rate` admits.
const ABOVE_ZERO: &str =
    "rates above zero, each a decimal of no more significant digits than a figure is written to";

/// The values `discounted_cash_flow_growth_rate` and
/// `discounted_cash_flow_terminal_growth_rate` admit.
const ABOVE_MINUS_ONE: &str = "rates above minus one, each a decimal of no more significant digits than a figure is written to";

/// The values `discounted_cash_flow_years` admits.
const POSITIVE_INTEGERS: &str = "the positive integers";

impl DiscountRate {
    const PRESET: &'static str = "0.0772";

    fn read(rate: &str) -> Result<DiscountRate, Refused> {
        read_rate(rate, ABOVE_ZERO, |rate| rate.sign() == Ordering::Greater).map(DiscountRate)
    }

    pub(super) fn preset() -> DiscountRate {
        DiscountRate::read(DiscountRate::PRESET).expect("the preset is above zero")
    }
}

impl GrowthRate {
    const PRESET: &'static str = "0.02";

    pub(super) fn preset() -> GrowthRate {
        GrowthRate(read_growth(GrowthRate::PRESET).expect("the preset is above minus one"))
    }
}

impl ForecastYears {
    pub(super) const PRESET: ForecastYears =
        ForecastYears(NonZeroUsize::new(5).expect("five is positive"));
}

impl TerminalGrowthRate {
    const PRESET: &'static str = "0.02";

    pub(super) fn preset() -> TerminalGrowthRate {
        TerminalGrowthRate(
            read_growth(TerminalGrowthRate::PRESET).expect("the preset is above minus one"),
        )
    }
}

/// The rate the characters state, or refused where they are outside the
/// reading grammar, have more significant digits than a figure is written to,
/// or state a rate that `admits` does not.
fn read_rate(
    rate: &str,
    domain: &'static str,
    admits: fn(&Number) -> bool,
) -> Result<Number, Refused> {
    Number::read(rate)
        .filter(|read| read.within_significant_digits() && admits(read))
        .ok_or_else(|| Refused::of(domain, rate))
}

fn read_growth(rate: &str) -> Result<Number, Refused> {
    read_rate(rate, ABOVE_MINUS_ONE, |rate| {
        Number::one().plus(rate).sign() == Ordering::Greater
    })
}

/// Every rate is within `SIGNIFICANT_DIGITS`, so the value written is the
/// value used.
impl Written for DiscountRate {
    fn written(&self) -> Box<str> {
        self.0.written()
    }
}

impl Written for GrowthRate {
    fn written(&self) -> Box<str> {
        self.0.written()
    }
}

impl Written for TerminalGrowthRate {
    fn written(&self) -> Box<str> {
        self.0.written()
    }
}

/// The integer's decimal characters, which are in the reading grammar, read
/// and written by the one reader and writer. Every `usize` has fewer digits
/// than a figure is written to, so the value written is the value used.
impl Written for ForecastYears {
    fn written(&self) -> Box<str> {
        Number::read(&self.0.get().to_string())
            .expect("an integer's decimal characters are in the reading grammar")
            .written()
    }
}

impl Settings {
    /// These settings with `discounted_cash_flow_discount_rate` at the rate
    /// `rate` states, or refused where it is outside its domain. It is never
    /// clamped.
    pub fn with_discounted_cash_flow_discount_rate(
        mut self,
        rate: &str,
    ) -> Result<Settings, Refused> {
        self.discounted_cash_flow_discount_rate = DiscountRate::read(rate)?;
        Ok(self)
    }

    /// These settings with `discounted_cash_flow_growth_rate` at the rate
    /// `rate` states, or refused where it is outside its domain. It is never
    /// clamped.
    pub fn with_discounted_cash_flow_growth_rate(
        mut self,
        rate: &str,
    ) -> Result<Settings, Refused> {
        self.discounted_cash_flow_growth_rate = GrowthRate(read_growth(rate)?);
        Ok(self)
    }

    /// These settings with `discounted_cash_flow_years` at `years`, or refused
    /// where `years` is outside its domain. It is never clamped.
    pub fn with_discounted_cash_flow_years(mut self, years: usize) -> Result<Settings, Refused> {
        self.discounted_cash_flow_years = ForecastYears(
            NonZeroUsize::new(years).ok_or_else(|| Refused::of(POSITIVE_INTEGERS, years))?,
        );
        Ok(self)
    }

    /// These settings with `discounted_cash_flow_terminal_growth_rate` at the
    /// rate `rate` states, or refused where it is outside its domain. It is
    /// never clamped.
    pub fn with_discounted_cash_flow_terminal_growth_rate(
        mut self,
        rate: &str,
    ) -> Result<Settings, Refused> {
        self.discounted_cash_flow_terminal_growth_rate = TerminalGrowthRate(read_growth(rate)?);
        Ok(self)
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the discounted cash flow is not computed yet")
    )]
    pub(crate) fn discounted_cash_flow_discount_rate(&self) -> &Number {
        &self.discounted_cash_flow_discount_rate.0
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the discounted cash flow is not computed yet")
    )]
    pub(crate) fn discounted_cash_flow_growth_rate(&self) -> &Number {
        &self.discounted_cash_flow_growth_rate.0
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the discounted cash flow is not computed yet")
    )]
    pub(crate) fn discounted_cash_flow_years(&self) -> NonZeroUsize {
        self.discounted_cash_flow_years.0
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the discounted cash flow is not computed yet")
    )]
    pub(crate) fn discounted_cash_flow_terminal_growth_rate(&self) -> &Number {
        &self.discounted_cash_flow_terminal_growth_rate.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ABOVE_MINUS_ONE, ABOVE_ZERO, POSITIVE_INTEGERS};
    use crate::arithmetic::Number;
    use crate::settings::{Refused, Settings};

    const THIRTY_FIVE_DIGITS: &str = "0.12345678901234567890123456789012345";

    fn number(characters: &str) -> Number {
        Number::read(characters).expect("a decimal")
    }

    fn written(settings: &Settings, name: &str) -> Box<str> {
        settings
            .used()
            .into_iter()
            .find(|setting| &*setting.name == name)
            .expect("the setting is recorded")
            .value
    }

    fn refused(refused: Refused, domain: &str, value: &str) {
        assert_eq!((refused.domain(), refused.value()), (domain, value));
    }

    #[test]
    fn the_presets_are_the_records() {
        let preset = Settings::preset();
        assert_eq!(
            preset.discounted_cash_flow_discount_rate(),
            &number("0.0772")
        );
        assert_eq!(preset.discounted_cash_flow_growth_rate(), &number("0.02"));
        assert_eq!(preset.discounted_cash_flow_years().get(), 5);
        assert_eq!(
            preset.discounted_cash_flow_terminal_growth_rate(),
            &number("0.02")
        );
        for (name, value) in [
            ("discounted_cash_flow_discount_rate", "0.0772"),
            ("discounted_cash_flow_growth_rate", "0.02"),
            ("discounted_cash_flow_years", "5"),
            ("discounted_cash_flow_terminal_growth_rate", "0.02"),
        ] {
            assert_eq!(&*written(&preset, name), value);
        }
    }

    #[test]
    fn a_discount_rate_not_above_zero_is_refused_and_never_clamped() {
        for rate in ["0", "-0.01", "-1", "-0"] {
            let refusal = Settings::preset()
                .with_discounted_cash_flow_discount_rate(rate)
                .expect_err("a rate not above zero");
            refused(refusal, ABOVE_ZERO, rate);
        }
    }

    #[test]
    fn a_growth_rate_not_above_minus_one_is_refused_and_never_clamped() {
        for rate in ["-1", "-1.0", "-2"] {
            let growth = Settings::preset()
                .with_discounted_cash_flow_growth_rate(rate)
                .expect_err("a rate not above minus one");
            refused(growth, ABOVE_MINUS_ONE, rate);
            let terminal = Settings::preset()
                .with_discounted_cash_flow_terminal_growth_rate(rate)
                .expect_err("a rate not above minus one");
            refused(terminal, ABOVE_MINUS_ONE, rate);
        }
    }

    #[test]
    fn zero_years_are_refused_and_never_clamped() {
        let refusal = Settings::preset()
            .with_discounted_cash_flow_years(0)
            .expect_err("zero is not a positive integer");
        refused(refusal, POSITIVE_INTEGERS, "0");
    }

    #[test]
    fn a_rate_with_more_digits_than_a_figure_or_outside_the_grammar_is_refused() {
        for rate in [
            THIRTY_FIVE_DIGITS,
            "1e-2",
            "0.05%",
            ".05",
            "0.",
            "",
            " 0.05",
            "+0.05",
        ] {
            let preset = Settings::preset;
            refused(
                preset()
                    .with_discounted_cash_flow_discount_rate(rate)
                    .expect_err("unreadable"),
                ABOVE_ZERO,
                rate,
            );
            refused(
                preset()
                    .with_discounted_cash_flow_growth_rate(rate)
                    .expect_err("unreadable"),
                ABOVE_MINUS_ONE,
                rate,
            );
            refused(
                preset()
                    .with_discounted_cash_flow_terminal_growth_rate(rate)
                    .expect_err("unreadable"),
                ABOVE_MINUS_ONE,
                rate,
            );
        }
    }

    #[test]
    fn a_rate_inside_its_domain_is_the_value_used_and_written() {
        for rate in ["0.0001", "0.1", "1", "0.1234567890123456789012345678901234"] {
            let settings = Settings::preset()
                .with_discounted_cash_flow_discount_rate(rate)
                .expect("a rate above zero");
            assert_eq!(settings.discounted_cash_flow_discount_rate(), &number(rate));
            assert_eq!(
                &*written(&settings, "discounted_cash_flow_discount_rate"),
                rate
            );
        }
        for rate in ["-0.9999", "-0.5", "0", "0.1", "3"] {
            let settings = Settings::preset()
                .with_discounted_cash_flow_growth_rate(rate)
                .and_then(|settings| settings.with_discounted_cash_flow_terminal_growth_rate(rate))
                .expect("a rate above minus one");
            assert_eq!(settings.discounted_cash_flow_growth_rate(), &number(rate));
            assert_eq!(
                settings.discounted_cash_flow_terminal_growth_rate(),
                &number(rate)
            );
            for name in [
                "discounted_cash_flow_growth_rate",
                "discounted_cash_flow_terminal_growth_rate",
            ] {
                assert_eq!(&*written(&settings, name), rate);
            }
        }
    }

    #[test]
    fn positive_years_are_the_value_used_and_written() {
        for (years, value) in [(1, "1"), (10, "10"), (1000, "1000")] {
            let settings = Settings::preset()
                .with_discounted_cash_flow_years(years)
                .expect("a positive integer");
            assert_eq!(settings.discounted_cash_flow_years().get(), years);
            assert_eq!(&*written(&settings, "discounted_cash_flow_years"), value);
        }
    }

    #[test]
    fn a_discount_rate_not_above_the_terminal_growth_rate_is_no_constructors_to_refuse() {
        for (discount, terminal) in [("0.03", "0.01"), ("0.01", "0.03"), ("0.0772", "0.0772")] {
            let one_way = Settings::preset()
                .with_discounted_cash_flow_discount_rate(discount)
                .and_then(|settings| {
                    settings.with_discounted_cash_flow_terminal_growth_rate(terminal)
                });
            let other_way = Settings::preset()
                .with_discounted_cash_flow_terminal_growth_rate(terminal)
                .and_then(|settings| settings.with_discounted_cash_flow_discount_rate(discount));
            assert_eq!(one_way, other_way);
            assert!(one_way.is_ok());
        }
    }
}
