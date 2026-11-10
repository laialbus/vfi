//! The dividend discount valuation's three settings. Each one's meaning, unit,
//! domain, preset and the preset's reason is
//! `docs/adr/dividend-discount-valuation-and-margin-of-safety.md`'s.
//!
//! That the required return is above the growth rate is not a domain: "each
//! setter sees one value, and a domain on the pair would make the order of the
//! two setters matter". The metric asks it, as the `declined` condition
//! `required_return_not_above_growth_rate`.

use std::num::NonZeroU16;

use super::{Refused, Settings, Written, read_above_minus_one, read_above_zero};
use crate::arithmetic::Number;

/// `dividend_discount_required_return`: the annual rate equity investors
/// require of the share, which the perpetuity is divided by after the growth
/// rate is taken from it.
///
/// Unit: a rate per year, a fraction of one. Domain: [`super::ABOVE_ZERO`].
/// At zero or below, a later dividend is worth at least an earlier one, which
/// is no required return.
///
/// Preset: [`RequiredReturn::PRESET`], `0.0802`, the cost of equity of the
/// United States market, financial firms included. It is the market's rate,
/// not the filer's: no filing carries the beta, riskless rate and risk premium
/// a filer's own would be built from.
///
/// Source: Damodaran, A., *Cost of Equity and Capital (US)*, Stern School of
/// Business, New York University,
/// <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/wacc.html>:
/// "Data used is as of January 2026" and "Last Updated in January 2026". The
/// row "Total Market" gives "Cost of Equity" 8.02%, beside the row "Total
/// Market (without financials)", whose 8.37% is not taken.
///
/// Why the column: the form discounts a flow to equity, so its rate is the
/// cost of equity, and not the cost of capital the discounted cash flow's
/// preset reads, which serves a flow to the firm.
///
/// Why the row: this metric is computed for every kind of filer, and the form
/// is the one Damodaran says is widely used for financial firms, so the row
/// holding them is the row of the filers valued. Damodaran, A., *Valuation
/// Approaches and Metrics: A Survey of the Theory and Evidence*, Stern School
/// of Business, New York University, November 2006,
/// <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
/// pages, sha256
/// `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`, page
/// 15: "There are two reasons why dividend discount model remain widely used
/// to value financial service companies. The first is that estimating capital
/// expenditures and working capital for a bank, an investment bank or an
/// insurance company is difficult to do."
///
/// The source republishes each January, and moving the preset to a later
/// figure is a preset change, not a version bump, since every result records
/// the value it used.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RequiredReturn(Number);

/// `dividend_discount_growth_rate`: the annual rate the dividend grows at for
/// ever, from the base.
///
/// Unit: a rate per year, a fraction of one. Domain:
/// [`super::ABOVE_MINUS_ONE`]. At minus one every later dividend is zero, and
/// below it they alternate in sign. Neither is growth.
///
/// Preset: [`PerpetualGrowthRate::PRESET`], `0.02`, the inflation the Federal
/// Reserve takes as its goal. A dividend growing at expected inflation holds
/// its purchasing power for ever and grows at no real rate, which is below the
/// nominal growth of any economy that grows in real terms. That is the limit
/// the survey above states of this rate on its page 11: "it cannot exceed the
/// growth rate of the economy in which the firm operates". The required
/// return's preset is nominal, so the growth beside it is too, and the preset
/// credits no filer with growth it has not shown.
///
/// The preset is this setting's own value, and it is not read from
/// `discounted_cash_flow_terminal_growth_rate`, which is the rate of a firm's
/// flow after a forecast. Moving one does not move the other.
///
/// Source: Federal Open Market Committee, *Statement on Longer-Run Goals and
/// Monetary Policy Strategy*,
/// <https://www.federalreserve.gov/monetarypolicy/files/FOMC_LongerRunGoals.pdf>,
/// 2 pages, "Adopted effective January 24, 2012; as reaffirmed effective
/// January 27, 2026": "The Committee reaffirms its judgment that inflation at
/// the rate of 2 percent, as measured by the annual change in the price index
/// for personal consumption expenditures, is most consistent over the longer
/// run with the Federal Reserve’s statutory maximum employment and price
/// stability mandates."
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PerpetualGrowthRate(Number);

/// `dividend_discount_base_years`: how many years the base dividend is the
/// mean of, ending at the year computed at.
///
/// Unit: years. Domain: [`FROM_TWO`]. One year is the base the record
/// forbids, one period's declared total taken as a perpetuity, and refusing it
/// holds that against the settings as well as the preset.
///
/// The type is the ceiling, and no literal stands for it. Past a gap each year
/// read gives a reason, so a window's length is the length of an absence's
/// list of reasons, and a value that list could not hold cannot be handed.
///
/// Preset: [`BaseYears::PRESET`], five. No source fixes it, so this is the
/// record's own decision, in the series record's words: "Five annual steps
/// hold five dividend decisions", and "six fiscal years are inside the
/// structured history of any filer that has filed XBRL since the 2011
/// phase-in", so five are. One year's special dividend then moves the base by
/// a fifth of itself. It is a setting of its own and not
/// `dividend_growth_years`, "which counts n steps across n + 1 years and reads
/// two of them".
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BaseYears(NonZeroU16);

/// The values `dividend_discount_base_years` admits.
const FROM_TWO: &str = "the integers from two up";

/// The fewest years `dividend_discount_base_years` admits.
const FEWEST_BASE_YEARS: u16 = 2;

impl RequiredReturn {
    const PRESET: &'static str = "0.0802";

    pub(super) fn preset() -> RequiredReturn {
        RequiredReturn(read_above_zero(RequiredReturn::PRESET).expect("the preset is above zero"))
    }
}

impl PerpetualGrowthRate {
    const PRESET: &'static str = "0.02";

    pub(super) fn preset() -> PerpetualGrowthRate {
        PerpetualGrowthRate(
            read_above_minus_one(PerpetualGrowthRate::PRESET)
                .expect("the preset is above minus one"),
        )
    }
}

impl BaseYears {
    pub(super) const PRESET: BaseYears = BaseYears(NonZeroU16::new(5).expect("five is positive"));
}

/// Every rate is within `SIGNIFICANT_DIGITS`, so the value written is the
/// value used.
impl Written for RequiredReturn {
    fn written(&self) -> Box<str> {
        self.0.written()
    }
}

impl Written for PerpetualGrowthRate {
    fn written(&self) -> Box<str> {
        self.0.written()
    }
}

/// The integer's decimal characters, which are in the reading grammar, read
/// and written by the one reader and writer. Every `u16` has fewer digits than
/// a figure is written to, so the value written is the value used.
impl Written for BaseYears {
    fn written(&self) -> Box<str> {
        Number::read(&self.0.get().to_string())
            .expect("an integer's decimal characters are in the reading grammar")
            .written()
    }
}

impl Settings {
    /// These settings with `dividend_discount_required_return` at the rate
    /// `rate` states, or refused where it is outside its domain. It is never
    /// clamped.
    pub fn with_dividend_discount_required_return(
        mut self,
        rate: &str,
    ) -> Result<Settings, Refused> {
        self.dividend_discount_required_return = RequiredReturn(read_above_zero(rate)?);
        Ok(self)
    }

    /// These settings with `dividend_discount_growth_rate` at the rate `rate`
    /// states, or refused where it is outside its domain. It is never clamped.
    pub fn with_dividend_discount_growth_rate(mut self, rate: &str) -> Result<Settings, Refused> {
        self.dividend_discount_growth_rate = PerpetualGrowthRate(read_above_minus_one(rate)?);
        Ok(self)
    }

    /// These settings with `dividend_discount_base_years` at `years`, or
    /// refused where `years` is outside its domain. It is never clamped.
    pub fn with_dividend_discount_base_years(mut self, years: u16) -> Result<Settings, Refused> {
        self.dividend_discount_base_years = BaseYears(
            NonZeroU16::new(years)
                .filter(|years| years.get() >= FEWEST_BASE_YEARS)
                .ok_or_else(|| Refused::of(FROM_TWO, years))?,
        );
        Ok(self)
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no metric reads the required return yet")
    )]
    pub(crate) fn dividend_discount_required_return(&self) -> &Number {
        &self.dividend_discount_required_return.0
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no metric reads the dividend's growth rate yet")
    )]
    pub(crate) fn dividend_discount_growth_rate(&self) -> &Number {
        &self.dividend_discount_growth_rate.0
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no metric reads the base years yet")
    )]
    pub(crate) fn dividend_discount_base_years(&self) -> NonZeroU16 {
        self.dividend_discount_base_years.0
    }
}

#[cfg(test)]
mod tests {
    use super::FROM_TWO;
    use crate::arithmetic::Number;
    use crate::settings::{ABOVE_MINUS_ONE, ABOVE_ZERO, Refused, Settings};

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
            preset.dividend_discount_required_return(),
            &number("0.0802")
        );
        assert_eq!(preset.dividend_discount_growth_rate(), &number("0.02"));
        assert_eq!(preset.dividend_discount_base_years().get(), 5);
        for (name, value) in [
            ("dividend_discount_required_return", "0.0802"),
            ("dividend_discount_growth_rate", "0.02"),
            ("dividend_discount_base_years", "5"),
        ] {
            assert_eq!(&*written(&preset, name), value);
        }
    }

    #[test]
    fn a_required_return_not_above_zero_is_refused_and_never_clamped() {
        for rate in ["0", "-0.01", "-1", "-0"] {
            let refusal = Settings::preset()
                .with_dividend_discount_required_return(rate)
                .expect_err("a rate not above zero");
            refused(refusal, ABOVE_ZERO, rate);
        }
    }

    #[test]
    fn a_growth_rate_not_above_minus_one_is_refused_and_never_clamped() {
        for rate in ["-1", "-1.0", "-2"] {
            let refusal = Settings::preset()
                .with_dividend_discount_growth_rate(rate)
                .expect_err("a rate not above minus one");
            refused(refusal, ABOVE_MINUS_ONE, rate);
        }
    }

    #[test]
    fn base_years_below_two_are_refused_and_never_clamped() {
        for (years, value) in [(0, "0"), (1, "1")] {
            let refusal = Settings::preset()
                .with_dividend_discount_base_years(years)
                .expect_err("below two");
            refused(refusal, FROM_TWO, value);
        }
        assert_eq!(FROM_TWO, "the integers from two up");
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
            refused(
                Settings::preset()
                    .with_dividend_discount_required_return(rate)
                    .expect_err("unreadable"),
                ABOVE_ZERO,
                rate,
            );
            refused(
                Settings::preset()
                    .with_dividend_discount_growth_rate(rate)
                    .expect_err("unreadable"),
                ABOVE_MINUS_ONE,
                rate,
            );
        }
    }

    #[test]
    fn a_rate_inside_its_domain_is_the_value_used_and_written() {
        for rate in [
            "0.0001",
            "0.09",
            "1",
            "0.1234567890123456789012345678901234",
        ] {
            let settings = Settings::preset()
                .with_dividend_discount_required_return(rate)
                .expect("a rate above zero");
            assert_eq!(settings.dividend_discount_required_return(), &number(rate));
            assert_eq!(
                &*written(&settings, "dividend_discount_required_return"),
                rate
            );
        }
        for rate in ["-0.9999", "-0.5", "0", "0.0802", "3"] {
            let settings = Settings::preset()
                .with_dividend_discount_growth_rate(rate)
                .expect("a rate above minus one");
            assert_eq!(settings.dividend_discount_growth_rate(), &number(rate));
            assert_eq!(&*written(&settings, "dividend_discount_growth_rate"), rate);
        }
    }

    #[test]
    fn base_years_from_two_are_the_value_used_and_written() {
        for (years, value) in [(2, "2"), (5, "5"), (40, "40"), (u16::MAX, "65535")] {
            let settings = Settings::preset()
                .with_dividend_discount_base_years(years)
                .expect("two or more");
            assert_eq!(settings.dividend_discount_base_years().get(), years);
            let recorded = written(&settings, "dividend_discount_base_years");
            assert_eq!(&*recorded, value);
            assert!(
                Number::read(&recorded).is_some_and(|number| number.within_significant_digits())
            );
        }
    }

    #[test]
    fn the_preset_growth_rate_is_its_own_and_not_the_terminal_growth_rates() {
        let settings = Settings::preset()
            .with_discounted_cash_flow_terminal_growth_rate("0.03")
            .expect("a rate above minus one");
        assert_eq!(settings.dividend_discount_growth_rate(), &number("0.02"));
    }

    #[test]
    fn a_required_return_not_above_the_growth_rate_is_no_constructors_to_refuse() {
        for (required, growth) in [("0.0802", "0.0802"), ("0.0802", "0.09")] {
            let one_way = Settings::preset()
                .with_dividend_discount_required_return(required)
                .and_then(|settings| settings.with_dividend_discount_growth_rate(growth));
            let other_way = Settings::preset()
                .with_dividend_discount_growth_rate(growth)
                .and_then(|settings| settings.with_dividend_discount_required_return(required));
            assert_eq!(one_way, other_way);
            assert!(one_way.is_ok());
        }
    }
}
