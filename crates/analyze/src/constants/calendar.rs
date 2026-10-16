//! The numbers the reading of a date cannot do without: the width of each
//! field, the count of months, each month's length and the leap year. Why a
//! date is read at all is `docs/adr/which-other-period-a-metric-reads.md`'s.
//!
//! Source for every constant here: Klyne, G., and Newman, C., *Date and Time
//! on the Internet: Timestamps*, RFC 3339, IETF, 2002.

/// How many digits a year is written in.
///
/// Source: RFC 3339 §5.6, `date-fullyear = 4DIGIT`.
pub(crate) const YEAR_DIGITS: usize = 4;

/// How many digits a month is written in.
///
/// Source: RFC 3339 §5.6, `date-month = 2DIGIT ; 01-12`.
pub(crate) const MONTH_DIGITS: usize = 2;

/// How many digits a day of the month is written in.
///
/// Source: RFC 3339 §5.6, `date-mday = 2DIGIT ; 01-28, 01-29, 01-30, 01-31
/// based on month/year`.
pub(crate) const DAY_DIGITS: usize = 2;

/// How many months a year has, so that December, the last, is month `MONTHS`.
///
/// Source: RFC 3339 §5.6, where `date-month` runs 01 to 12, and §5.7, whose
/// table ends at 12, December.
pub(crate) const MONTHS: u32 = 12;

/// Each month's length in days, January first, as its length in a common
/// year and then in a leap year.
///
/// Source: RFC 3339 §5.7, the table of the maximum value of `date-mday`. It
/// gives February 28 days in a normal year and 29 in a leap year, and every
/// other month one length in both.
pub(crate) const MONTH_LENGTHS: [(u32, u32); MONTHS as usize] = [
    (31, 31),
    (28, 29),
    (31, 31),
    (30, 30),
    (31, 31),
    (30, 30),
    (31, 31),
    (31, 31),
    (30, 30),
    (31, 31),
    (30, 30),
    (31, 31),
];

/// A year this divides is a leap year, unless `CENTURY_DIVISOR` divides it
/// and `LEAP_CENTURY_DIVISOR` does not.
///
/// Source: RFC 3339 Appendix C, `year % 4 == 0 && (year % 100 != 0 || year %
/// 400 == 0)`.
pub(crate) const LEAP_DIVISOR: u32 = 4;

/// A year this divides is not a leap year, unless `LEAP_CENTURY_DIVISOR`
/// divides it.
///
/// Source: RFC 3339 Appendix C, as `LEAP_DIVISOR` cites it.
pub(crate) const CENTURY_DIVISOR: u32 = 100;

/// A year this divides is a leap year.
///
/// Source: RFC 3339 Appendix C, as `LEAP_DIVISOR` cites it.
pub(crate) const LEAP_CENTURY_DIVISOR: u32 = 400;
