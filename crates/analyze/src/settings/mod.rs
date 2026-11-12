//! The settings: every free parameter of the derivation, each a field of the
//! one [`Settings`] a caller hands analyze on every call.
//!
//! What a setting is, and what each defines beside it — its meaning, unit,
//! domain, preset and the preset's reason — is
//! `docs/adr/what-analyze-takes-and-returns.md`'s. A method that has settings
//! adds its file here and its fields to the one declaration below.

use std::cmp::Ordering;
use std::fmt;

use vfi_contracts::analyze_store::Setting;

use crate::arithmetic::Number;
use discounted_cash_flow::{DiscountRate, ForecastYears, GrowthRate, TerminalGrowthRate};
use dividend_discount::{BaseYears, PerpetualGrowthRate, RequiredReturn};
use dividend_growth_rate::DividendGrowthYears;
use dividend_streak::DividendStreakYears;

mod discounted_cash_flow;
mod dividend_discount;
mod dividend_growth_rate;
mod dividend_streak;

/// A setting's value as the characters analyze writes it, which is how the
/// premises carry it. How a value is written is under the method version, so
/// a setting's type implements this where that writing is defined.
pub trait Written {
    fn written(&self) -> Box<str>;
}

/// Declares [`Settings`] and, from the same fields, every setting a result
/// records: each field by its name, with its value written.
///
/// The names are read off the declaration, so a setting added is recorded by
/// adding its field, and there is no second list to leave it out of.
macro_rules! settings {
    (
        $(#[$type_doc:meta])*
        pub struct Settings {
            $( $(#[$field_doc:meta])* $field:ident : $field_type:ty, )*
        }
    ) => {
        $(#[$type_doc])*
        pub struct Settings {
            $( $(#[$field_doc])* $field: $field_type, )*
        }

        impl Settings {
            /// Every setting by name, with the value used.
            pub(crate) fn used(&self) -> Vec<Setting> {
                vec![$(
                    Setting {
                        name: stringify!($field).into(),
                        value: Written::written(&self.$field),
                    },
                )*]
            }
        }
    };
}

settings! {
    /// Every setting the derivation reads.
    ///
    /// `non_exhaustive` keeps the struct literal out of other crates, so a
    /// caller has one only from a constructor here.
    #[non_exhaustive]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Settings {
        dividend_streak_years: DividendStreakYears,
        discounted_cash_flow_discount_rate: DiscountRate,
        discounted_cash_flow_growth_rate: GrowthRate,
        discounted_cash_flow_years: ForecastYears,
        discounted_cash_flow_terminal_growth_rate: TerminalGrowthRate,
        dividend_growth_years: DividendGrowthYears,
        dividend_discount_required_return: RequiredReturn,
        dividend_discount_growth_rate: PerpetualGrowthRate,
        dividend_discount_base_years: BaseYears,
    }
}

impl Settings {
    /// Every setting at its preset value. The caller calls this, and the
    /// derivation never does.
    pub fn preset() -> Self {
        Settings {
            dividend_streak_years: DividendStreakYears::PRESET,
            discounted_cash_flow_discount_rate: DiscountRate::preset(),
            discounted_cash_flow_growth_rate: GrowthRate::preset(),
            discounted_cash_flow_years: ForecastYears::PRESET,
            discounted_cash_flow_terminal_growth_rate: TerminalGrowthRate::preset(),
            dividend_growth_years: DividendGrowthYears::PRESET,
            dividend_discount_required_return: RequiredReturn::preset(),
            dividend_discount_growth_rate: PerpetualGrowthRate::preset(),
            dividend_discount_base_years: BaseYears::PRESET,
        }
    }

    pub(crate) fn dividend_streak_years(&self) -> DividendStreakYears {
        self.dividend_streak_years
    }
}

/// The values a rate that must be above zero admits.
const ABOVE_ZERO: &str =
    "rates above zero, each a decimal of no more significant digits than a figure is written to";

/// The values a rate that must be above minus one admits.
const ABOVE_MINUS_ONE: &str = "rates above minus one, each a decimal of no more significant digits than a figure is written to";

/// The rate the characters state, or refused where they are outside the
/// reading grammar, have more significant digits than a figure is written to,
/// or state a rate that `admits` does not.
///
/// A rate is handed as characters and read by the one reader, so a value
/// outside the reading grammar is outside the domain, as is one that the one
/// writer could not write back as itself.
fn read_rate(
    rate: &str,
    domain: &'static str,
    admits: fn(&Number) -> bool,
) -> Result<Number, Refused> {
    Number::read(rate)
        .filter(|read| read.within_significant_digits() && admits(read))
        .ok_or_else(|| Refused::of(domain, rate))
}

fn read_above_zero(rate: &str) -> Result<Number, Refused> {
    read_rate(rate, ABOVE_ZERO, |rate| rate.sign() == Ordering::Greater)
}

fn read_above_minus_one(rate: &str) -> Result<Number, Refused> {
    read_rate(rate, ABOVE_MINUS_ONE, |rate| {
        Number::one().plus(rate).sign() == Ordering::Greater
    })
}

/// A value outside a setting's domain, which the setting's constructor
/// refuses and never clamps. It is the caller's error, raised before analyze
/// runs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refused {
    domain: &'static str,
    value: Box<str>,
}

impl Refused {
    fn of(domain: &'static str, value: impl fmt::Display) -> Self {
        Refused {
            domain,
            value: value.to_string().into(),
        }
    }

    /// The values the setting admits, in words.
    pub fn domain(&self) -> &str {
        self.domain
    }

    /// The value refused, as the caller handed it.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Refused {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            out,
            "the setting admits {}, and {} is not among them",
            self.domain, self.value
        )
    }
}

impl std::error::Error for Refused {}
