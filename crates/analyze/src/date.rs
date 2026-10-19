//! The one reader of a date in the derivation. It reads a date, steps it back
//! one day and writes it in the grammar it was read in. Why a metric reads a
//! date, and only so, is `docs/adr/which-other-period-a-metric-reads.md`'s,
//! under the method version. Whether a date ends its month, whether two dates
//! are a year apart in one month, and whether one date is on or before
//! another are `docs/adr/which-duration-precedes-a-duration.md`'s, for the
//! test of whether a duration is a year and for nothing else.

use crate::constants::calendar::{
    CENTURY_DIVISOR, DAY_DIGITS, LEAP_CENTURY_DIVISOR, LEAP_DIVISOR, MONTH_DIGITS, MONTH_LENGTHS,
    MONTHS, YEAR_DIGITS,
};
use crate::constants::figure::RADIX;

/// The `declined` condition of a read whose duration's start states no day
/// with one before it: a start outside the grammar, or `0000-01-01`, the one
/// start in it with no day before it.
pub(crate) const START_NOT_A_DATE: &str = "start_not_a_date";

/// What stands between the fields, RFC 3339 §5.6's `full-date`.
const SEPARATOR: &str = "-";

/// The day before the day `start` states, written as `start` is, or none
/// where `start` states no day with one before it, which is
/// `START_NOT_A_DATE`.
///
/// This is the instant a duration starting at `start` opens at. Source:
/// Engel, P., Hamscher, W., Shuetrim, G., vun Kannon, D., and Wallis, H.,
/// *Extensible Business Reporting Language (XBRL) 2.1*, XBRL International,
/// Recommendation 2003-12-31 with errata corrections to 2013-02-20, §4.7.2:
/// "A date, with no time part, in the content of an startDate element is
/// defined to be equivalent to specifying a dateTime of the same date, and
/// T00:00:00 (midnight at the start of the day). A date, with no time part,
/// in the endDate or instant element is defined to be equivalent to
/// specifying a dateTime of the same date plus P1D and with a time part of
/// T00:00:00. This represents midnight at the end of the day." So an instant
/// dated the day before a start is the moment that start begins at.
pub(crate) fn day_before(start: &str) -> Option<Box<str>> {
    Date::read(start)?.previous().map(Date::written)
}

/// A day on the proleptic Gregorian calendar, as its three calendar fields.
/// They are not amounts, and no figure is made of them.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    /// The date the characters state in RFC 3339's `full-date`, or none where
    /// they are outside it: the year, month and day each in exactly their
    /// field's digits, joined by `SEPARATOR`, with a month from the first to
    /// the `MONTHS`th and a day from the first to that month's length in
    /// that year.
    pub(crate) fn read(characters: &str) -> Option<Date> {
        let mut fields = characters.split(SEPARATOR);
        let year = field(fields.next()?, YEAR_DIGITS)?;
        let month = field(fields.next()?, MONTH_DIGITS)?;
        let day = field(fields.next()?, DAY_DIGITS)?;
        let in_grammar = fields.next().is_none()
            && (1..=MONTHS).contains(&month)
            && (1..=days_in(year, month)).contains(&day);
        in_grammar.then_some(Date { year, month, day })
    }

    /// The day before, or none before the first day of year zero. Across a
    /// month it is the previous month's last day, and across a year
    /// December's.
    pub(crate) fn previous(self) -> Option<Date> {
        if self.day > 1 {
            Some(Date {
                day: self.day - 1,
                ..self
            })
        } else if self.month > 1 {
            let month = self.month - 1;
            Some(Date {
                month,
                day: days_in(self.year, month),
                ..self
            })
        } else {
            let year = self.year.checked_sub(1)?;
            Some(Date {
                year,
                month: MONTHS,
                day: days_in(year, MONTHS),
            })
        }
    }

    /// The characters of the date in RFC 3339's `full-date`.
    pub(crate) fn written(self) -> Box<str> {
        [
            written_field(self.year, YEAR_DIGITS),
            written_field(self.month, MONTH_DIGITS),
            written_field(self.day, DAY_DIGITS),
        ]
        .join(SEPARATOR)
        .into()
    }
}

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "no metric tests for a year yet")
)]
impl Date {
    /// Whether this is the last day of its month, by that month's length in
    /// its year.
    pub(crate) fn ends_its_month(self) -> bool {
        self.day == days_in(self.year, self.month)
    }

    /// Whether `later` is in this date's month, in the year after this
    /// date's year, read from the calendar fields alone.
    pub(crate) fn in_its_month_a_year_before(self, later: Date) -> bool {
        self.month == later.month && self.year.checked_add(1) == Some(later.year)
    }

    /// Whether this date is on or before `other`: an earlier year, or the
    /// same year and an earlier month, or the same month and an earlier or
    /// the same day.
    ///
    /// The one ordering of dates in the derivation. It is for whether two
    /// durations share a day and for nothing else: it never seeks a row,
    /// never sorts and never picks the latest.
    pub(crate) fn on_or_before(self, other: Date) -> bool {
        (self.year, self.month, self.day) <= (other.year, other.month, other.day)
    }
}

/// The value of a field written in exactly `width` ASCII digits, or none.
/// Each digit is read as the amount reader reads it.
fn field(characters: &str, width: usize) -> Option<u32> {
    if characters.len() != width {
        return None;
    }
    characters.chars().try_fold(0, |value, character| {
        Some(value * RADIX + character.to_digit(RADIX)?)
    })
}

/// `value` in exactly `width` digits, led by zeros. Every field read or
/// stepped back fits its width.
fn written_field(value: u32, width: usize) -> String {
    let digits: Vec<char> = std::iter::successors(Some(value), |rest| Some(rest / RADIX))
        .take(width)
        .map(|rest| char::from_digit(rest % RADIX, RADIX).expect("a remainder is below the radix"))
        .collect();
    digits.into_iter().rev().collect()
}

/// How many days `month`, from the first to the `MONTHS`th, has in `year`.
fn days_in(year: u32, month: u32) -> u32 {
    let at = usize::try_from(month - 1).expect("a month is an index");
    let (common, leap) = MONTH_LENGTHS[at];
    if is_leap(year) { leap } else { common }
}

/// Whether `year` is a leap year, as RFC 3339 Appendix C reads it.
fn is_leap(year: u32) -> bool {
    year.is_multiple_of(LEAP_DIVISOR)
        && (!year.is_multiple_of(CENTURY_DIVISOR) || year.is_multiple_of(LEAP_CENTURY_DIVISOR))
}

#[cfg(test)]
mod tests {
    use super::{Date, START_NOT_A_DATE, day_before};

    const STEPS: [(&str, &str); 7] = [
        ("2024-01-15", "2024-01-14"),
        ("2023-05-01", "2023-04-30"),
        ("2024-01-01", "2023-12-31"),
        ("2024-03-01", "2024-02-29"),
        ("2023-03-01", "2023-02-28"),
        ("2000-03-01", "2000-02-29"),
        ("2100-03-01", "2100-02-28"),
    ];

    const NO_DAY_BEFORE: [&str; 6] = [
        "2024-1-01",
        "20240101",
        "2024-13-01",
        "2023-02-29",
        "0000-01-01",
        "",
    ];

    #[test]
    fn each_start_gives_the_day_before() {
        for (start, opening) in STEPS {
            assert_eq!(day_before(start).as_deref(), Some(opening), "{start}");
        }
    }

    #[test]
    fn a_start_that_states_no_day_before_gives_none() {
        for start in NO_DAY_BEFORE {
            assert_eq!(day_before(start), None, "{start:?}");
        }
    }

    #[test]
    fn the_first_day_of_year_zero_is_in_the_grammar_and_has_no_day_before() {
        let first = Date::read("0000-01-01").expect("0000-01-01 is a full-date");
        assert!(first.previous().is_none());
    }

    fn date(characters: &str) -> Date {
        Date::read(characters).expect(characters)
    }

    #[test]
    fn a_month_ends_on_its_length_in_its_year() {
        for end in [
            "2024-12-31",
            "2024-09-30",
            "2024-02-29",
            "2023-02-28",
            "2100-02-28",
        ] {
            assert!(date(end).ends_its_month(), "{end}");
        }
        for not_an_end in ["2024-12-30", "2024-02-28", "2024-07-14", "2024-09-01"] {
            assert!(!date(not_an_end).ends_its_month(), "{not_an_end}");
        }
    }

    #[test]
    fn a_year_before_in_its_month_is_the_same_month_of_the_next_year() {
        let in_month = [
            ("2023-12-31", "2024-12-31"),
            ("2023-02-28", "2024-02-29"),
            ("2023-09-01", "2024-09-30"),
        ];
        for (earlier, later) in in_month {
            assert!(
                date(earlier).in_its_month_a_year_before(date(later)),
                "{earlier} {later}"
            );
        }
        let not_in_month = [
            ("2023-12-31", "2023-12-31"),
            ("2023-12-31", "2025-12-31"),
            ("2023-08-31", "2024-09-30"),
            ("2024-12-31", "2023-12-31"),
            ("9999-12-31", "9999-12-31"),
        ];
        for (earlier, later) in not_in_month {
            assert!(
                !date(earlier).in_its_month_a_year_before(date(later)),
                "{earlier} {later}"
            );
        }
    }

    #[test]
    fn a_date_is_on_or_before_itself_and_every_later_date() {
        let ordered = [
            ("2023-12-31", "2023-12-31"),
            ("2023-12-31", "2024-01-01"),
            ("2023-04-30", "2023-05-01"),
            ("2023-01-31", "2023-02-01"),
            ("2022-12-31", "2023-01-01"),
        ];
        for (earlier, later) in ordered {
            assert!(date(earlier).on_or_before(date(later)), "{earlier} {later}");
        }
        for (earlier, later) in ordered.iter().filter(|(one, other)| one != other) {
            assert!(
                !date(later).on_or_before(date(earlier)),
                "{later} {earlier}"
            );
        }
    }

    #[test]
    fn a_date_written_back_from_a_read_is_its_characters() {
        let dates = STEPS
            .iter()
            .flat_map(|(start, opening)| [*start, *opening])
            .chain(["0000-01-01", "9999-12-31"]);
        for date in dates {
            let read = Date::read(date).expect(date);
            assert_eq!(&*read.written(), date);
        }
    }

    #[test]
    fn the_condition_is_named_as_the_record_names_it() {
        assert_eq!(START_NOT_A_DATE, "start_not_a_date");
    }
}
