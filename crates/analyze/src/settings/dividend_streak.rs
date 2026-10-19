//! The dividend streak's one setting. Why the window is a setting and not a
//! constant, and the preset's reason, is
//! `docs/adr/which-duration-precedes-a-duration.md`'s.

use std::num::NonZeroUsize;

use super::{Refused, Settings, Written};
use crate::arithmetic::Number;

/// `dividend_streak_years`: how many consecutive paying years, ending at the
/// year the streak is computed at, the dividend streak counts to at most. A
/// run longer than this reads as this.
///
/// Unit: years. Domain: [`DOMAIN`].
///
/// Preset: [`DividendStreakYears::PRESET`], ten. Ten years spans a business
/// cycle with its downturn, which is when a dividend is cut. And ten fiscal
/// years are inside the structured history of any filer that has filed XBRL
/// since 2011, so most filers are measured rather than absent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DividendStreakYears(NonZeroUsize);

/// The values `dividend_streak_years` admits.
const DOMAIN: &str = "the positive integers";

impl DividendStreakYears {
    pub(super) const PRESET: DividendStreakYears =
        DividendStreakYears(NonZeroUsize::new(10).expect("ten is positive"));

    /// How many years the streak counts to at most.
    pub(crate) fn years(self) -> NonZeroUsize {
        self.0
    }
}

/// The integer's decimal characters, which are in the reading grammar, read
/// and written by the one reader and writer. Every `usize` has fewer digits
/// than a figure is written to, so the value written is the value used.
impl Written for DividendStreakYears {
    fn written(&self) -> Box<str> {
        Number::read(&self.0.get().to_string())
            .expect("an integer's decimal characters are in the reading grammar")
            .written()
    }
}

impl Settings {
    /// These settings with `dividend_streak_years` at `years`, or refused
    /// where `years` is outside its domain. It is never clamped.
    pub fn with_dividend_streak_years(mut self, years: usize) -> Result<Settings, Refused> {
        self.dividend_streak_years = DividendStreakYears(
            NonZeroUsize::new(years).ok_or_else(|| Refused::of(DOMAIN, years))?,
        );
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::DOMAIN;
    use crate::arithmetic::Number;
    use crate::settings::Settings;

    #[test]
    fn the_preset_is_ten_years() {
        let preset = Settings::preset();
        assert_eq!(preset.dividend_streak_years.years().get(), 10);
        assert_eq!(&*preset.used()[0].value, "10");
    }

    #[test]
    fn zero_is_refused_and_never_clamped() {
        let refused = Settings::preset()
            .with_dividend_streak_years(0)
            .expect_err("zero is not a positive integer");
        assert_eq!(refused.domain(), DOMAIN);
        assert_eq!(refused.value(), "0");
        assert_eq!(DOMAIN, "the positive integers");
    }

    #[test]
    fn a_positive_integer_is_the_value_used_and_written() {
        for (years, written) in [(1, "1"), (2, "2"), (40, "40"), (1000, "1000")] {
            let settings = Settings::preset()
                .with_dividend_streak_years(years)
                .expect("a positive integer is admitted");
            assert_eq!(settings.dividend_streak_years.years().get(), years);
            let value = &settings.used()[0].value;
            assert_eq!(&**value, written);
            assert!(Number::read(value).is_some_and(|number| number.within_significant_digits()));
        }
    }
}
