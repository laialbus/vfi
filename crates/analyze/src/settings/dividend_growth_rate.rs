//! The dividend growth rate's one setting. Why the window is a setting and not
//! a constant, and the preset's reason, is
//! `docs/adr/which-duration-precedes-a-duration.md`'s, which
//! `docs/adr/dividend-growth-rate.md` quotes.

use std::num::NonZeroU32;

use super::{Refused, Settings, Written};
use crate::arithmetic::Number;

/// `dividend_growth_years`: how many years the dividend growth rate compounds
/// over, ending at the year it is computed at. The rate reads that year and
/// the year this many years before it.
///
/// Unit: years. Domain: [`DOMAIN`].
///
/// Preset: [`DividendGrowthYears::PRESET`], five. "Five annual steps hold
/// five dividend decisions", so one year's special dividend or one skipped
/// raise "moves the rate through one step of five rather than through the
/// whole rate". And "six fiscal years are inside the structured history of
/// any filer that has filed XBRL since the 2011 phase-in".
///
/// The value is the index of the root the rate takes, which is why its type
/// is the root's.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DividendGrowthYears(NonZeroU32);

/// The values `dividend_growth_years` admits.
const DOMAIN: &str = "the positive integers";

/// Every value, and one more, fits a `usize`, so a series of years that many
/// years back is taken without overflow at the largest value.
const _: () = assert!(
    u32::BITS < usize::BITS,
    "dividend_growth_years and one more fit a usize"
);

impl DividendGrowthYears {
    pub(super) const PRESET: DividendGrowthYears =
        DividendGrowthYears(NonZeroU32::new(5).expect("five is positive"));
}

/// The integer's decimal characters, which are in the reading grammar, read
/// and written by the one reader and writer. Every `u32` has fewer digits than
/// a figure is written to, so the value written is the value used.
impl Written for DividendGrowthYears {
    fn written(&self) -> Box<str> {
        Number::read(&self.0.get().to_string())
            .expect("an integer's decimal characters are in the reading grammar")
            .written()
    }
}

impl Settings {
    /// These settings with `dividend_growth_years` at `years`, or refused
    /// where `years` is outside its domain. It is never clamped.
    pub fn with_dividend_growth_years(mut self, years: u32) -> Result<Settings, Refused> {
        self.dividend_growth_years =
            DividendGrowthYears(NonZeroU32::new(years).ok_or_else(|| Refused::of(DOMAIN, years))?);
        Ok(self)
    }

    /// How many years the dividend growth rate compounds over.
    pub(crate) fn dividend_growth_years(&self) -> NonZeroU32 {
        self.dividend_growth_years.0
    }
}

#[cfg(test)]
mod tests {
    use super::DOMAIN;
    use crate::arithmetic::Number;
    use crate::settings::Settings;

    fn written(settings: &Settings) -> Box<str> {
        settings
            .used()
            .into_iter()
            .find(|setting| &*setting.name == "dividend_growth_years")
            .expect("the setting is recorded")
            .value
    }

    #[test]
    fn the_preset_is_five_years() {
        let preset = Settings::preset();
        assert_eq!(preset.dividend_growth_years().get(), 5);
        assert_eq!(&*written(&preset), "5");
    }

    #[test]
    fn zero_is_refused_and_never_clamped() {
        let refused = Settings::preset()
            .with_dividend_growth_years(0)
            .expect_err("zero is not a positive integer");
        assert_eq!(refused.domain(), DOMAIN);
        assert_eq!(refused.value(), "0");
        assert_eq!(DOMAIN, "the positive integers");
    }

    #[test]
    fn a_positive_integer_is_the_value_used_and_written() {
        for (years, value) in [(1, "1"), (2, "2"), (40, "40"), (u32::MAX, "4294967295")] {
            let settings = Settings::preset()
                .with_dividend_growth_years(years)
                .expect("a positive integer is admitted");
            assert_eq!(settings.dividend_growth_years().get(), years);
            let recorded = written(&settings);
            assert_eq!(&*recorded, value);
            assert!(
                Number::read(&recorded).is_some_and(|number| number.within_significant_digits())
            );
        }
    }
}
