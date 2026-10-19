//! Whether a duration is a year, the year before a year, and a series of
//! years, each the year before the last. Why each, and how absence composes
//! across the reads made at them, is
//! `docs/adr/which-duration-precedes-a-duration.md`'s, under the method
//! version.
//!
//! A year is read from the dates of the history's durations alone, through
//! `date.rs`, and is computed afresh wherever it is asked: nothing in the
//! history is labelled a year. The year before is found by equality of
//! characters, and nothing is ordered to find it.

#![cfg_attr(
    not(test),
    expect(dead_code, reason = "no metric reads the year before's instants yet")
)]

use std::iter;

use vfi_contracts::analyze_store::Reason;
use vfi_contracts::canonical_concepts::{Period, Row};

use super::{Computing, Seek, instant};
use crate::constants::annual_period::{DAYS_IN_A_WEEK, FEWEST_WEEKS, MOST_WEEKS};
use crate::date::{Date, START_NOT_A_DATE, day_before};

/// The `sought_by` of a read at a year the series did not find, or at a year
/// beyond one it did not find.
pub(in crate::metrics) const PRECEDING_YEAR: &str = "preceding_year";

/// The `sought_by` of a read at the opening instant of the year before, where
/// the history does not hold that instant.
pub(in crate::metrics) const PRECEDING_OPENING_INSTANT: &str = "preceding_opening_instant";

/// The `declined` condition of a metric stated over a year, at a duration
/// whose end is outside the grammar.
pub(in crate::metrics) const END_NOT_A_DATE: &str = "end_not_a_date";

/// The `declined` condition of a metric stated over a year, at a duration
/// whose own length is neither twelve months to a month's end nor fifty-two
/// or fifty-three weeks.
pub(in crate::metrics) const NOT_A_YEAR: &str = "not_a_year";

/// The `declined` condition of a metric stated over a year, at a duration
/// whose own length is a year and which shares a day with another duration of
/// the history whose own length is a year.
pub(in crate::metrics) const OVERLAPS_ANOTHER_YEAR: &str = "overlaps_another_year";

/// The duration a metric computes at, which the test found a year.
pub(in crate::metrics) struct Year<'c, 'a> {
    computing: &'c Computing<'a>,
}

/// The duration `computing` is at as a year, or the one `declined` reason it
/// is not.
pub(in crate::metrics) fn year<'c, 'a>(
    computing: &'c Computing<'a>,
) -> Result<Year<'c, 'a>, Reason> {
    let (start, end) = computing.duration();
    test(start, end, computing.history)
        .map(|()| Year { computing })
        .map_err(|condition| Reason::declined(condition.into()))
}

impl<'a> Year<'_, 'a> {
    /// This year and the `back` years before it, in series order, each next
    /// year the year before the last one found. Where the chain finds no
    /// year, that year and every one beyond it give `no_period` with
    /// `PRECEDING_YEAR`, and `computing_at` this year: the series never steps
    /// around a gap and never closes up.
    pub(in crate::metrics) fn series(
        &self,
        back: usize,
    ) -> impl Iterator<Item = Result<&'a Row, Reason>> + use<'a> {
        let history = self.computing.history;
        let computing_at = self.computing.row.period().clone();
        iter::successors(Some(self.computing.row), move |found| {
            before(found, history)
        })
        .map(Ok)
        .chain(iter::repeat_with(move || {
            Err(Reason::no_period(
                computing_at.clone(),
                PRECEDING_YEAR.into(),
            ))
        }))
        .take(back + 1)
    }

    /// The year before this one, or `no_period` with `PRECEDING_YEAR`.
    pub(in crate::metrics) fn before(&self) -> Result<&'a Row, Reason> {
        self.series(1)
            .nth(1)
            .expect("a series one year back holds two years")
    }

    /// The instant the year before closes at, which is the instant this year
    /// opens at, read by the accepted `opening_instant` seek once the year
    /// before is found.
    pub(in crate::metrics) fn closing_of_before(&self) -> Result<&'a Row, Reason> {
        self.before()?;
        Seek::Opening.row(self.computing)
    }

    /// The instant the year before opens at, the day before its own start, or
    /// `no_period` with `PRECEDING_OPENING_INSTANT` where the history does not
    /// hold it.
    pub(in crate::metrics) fn opening_of_before(&self) -> Result<&'a Row, Reason> {
        let (start, _) = dates(self.before()?.period()).expect("a year is a duration");
        let date = day_before(start).expect("a year's start has a day before it");
        instant(self.computing.history, &date).ok_or_else(|| {
            Reason::no_period(
                self.computing.row.period().clone(),
                PRECEDING_OPENING_INSTANT.into(),
            )
        })
    }
}

/// The year before `year`: the duration of `history` that ends on `year`'s
/// opening date, character for character, and is a year by the test, at its
/// first row. There is one or none, since two years ending on one day share
/// it and neither is alone.
fn before<'a>(year: &Row, history: &'a [Row]) -> Option<&'a Row> {
    let (start, _) = dates(year.period()).expect("a year is a duration");
    let opening = day_before(start).expect("a year's start has a day before it");
    history.iter().find(|row| {
        dates(row.period())
            .is_some_and(|(start, end)| *end == *opening && test(start, end, history).is_ok())
    })
}

/// Whether the duration `start` to `end` is a year among `history`'s
/// durations, or the first condition under which it is not: its own length
/// is a year, and no other duration whose own length is a year shares a day
/// with it. Two durations share a day when each starts on or before the
/// other ends.
fn test(start: &str, end: &str, history: &[Row]) -> Result<(), &'static str> {
    let (first, last) = own_length(start, end)?;
    let overlapped = history
        .iter()
        .filter_map(|row| dates(row.period()))
        .filter(|other| *other != (start, end))
        .filter_map(|(start, end)| own_length(start, end).ok())
        .any(|(other_first, other_last)| {
            first.on_or_before(other_last) && other_first.on_or_before(last)
        });
    if overlapped {
        Err(OVERLAPS_ANOTHER_YEAR)
    } else {
        Ok(())
    }
}

/// The days `start` and `end` state, where the duration's own length is a
/// year by either clause; or the first condition under which it is not:
/// `START_NOT_A_DATE` where the start states no day with one before it,
/// `END_NOT_A_DATE` where the end states no day, and `NOT_A_YEAR` where
/// neither clause holds. Which clause holds makes no difference.
fn own_length(start: &str, end: &str) -> Result<(Date, Date), &'static str> {
    let first = Date::read(start).ok_or(START_NOT_A_DATE)?;
    let opening = first.previous().ok_or(START_NOT_A_DATE)?;
    let last = Date::read(end).ok_or(END_NOT_A_DATE)?;
    if twelve_months(opening, last) || whole_weeks(opening, last) {
        Ok((first, last))
    } else {
        Err(NOT_A_YEAR)
    }
}

/// Twelve months to a month's end: the opening date and the end each end
/// their month, in one month, the end's year one after the opening date's.
///
/// Source: 26 U.S.C. §441(d), "a period of 12 months ending on December 31",
/// and (e), "a period of 12 months ending on the last day of any month other
/// than December", as `constants/annual_period.rs` quotes them.
fn twelve_months(opening: Date, last: Date) -> bool {
    opening.ends_its_month() && last.ends_its_month() && opening.in_its_month_a_year_before(last)
}

/// Fifty-two or fifty-three weeks: the end stepped back one day for each day
/// of `FEWEST_WEEKS` weeks, or of `MOST_WEEKS` weeks, is the opening date,
/// compared character for character.
///
/// Source: 26 U.S.C. §441(f)(1), "an annual period which varies from 52 to 53
/// weeks", as `constants/annual_period.rs` quotes it.
fn whole_weeks(opening: Date, last: Date) -> bool {
    let opening = opening.written();
    [FEWEST_WEEKS, MOST_WEEKS].iter().any(|weeks| {
        iter::successors(Some(last), |day| day.previous())
            .nth(weeks * DAYS_IN_A_WEEK)
            .is_some_and(|day| day.written() == opening)
    })
}

/// The dates a duration starts and ends at, or none at an instant.
fn dates(period: &Period) -> Option<(&str, &str)> {
    match period {
        Period::Duration { start, end } => Some((start, end)),
        Period::Instant { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::Reason;
    use vfi_contracts::canonical_concepts::{History, Period, Resolution, Row, Rule, SetBy};

    use super::super::{CLOSING_INSTANT, Computing, OPENING_INSTANT};
    use super::{
        END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, PRECEDING_OPENING_INSTANT,
        PRECEDING_YEAR, year,
    };
    use crate::date::START_NOT_A_DATE;
    use crate::settings::Settings;

    fn duration(start: &str, end: &str) -> Period {
        Period::Duration {
            start: start.into(),
            end: end.into(),
        }
    }

    fn instant(at: &str) -> Period {
        Period::Instant { at: at.into() }
    }

    /// A row at `period` where every concept states one.
    fn row(period: Period) -> Row {
        Row::at(
            period,
            std::array::from_fn(|_| Resolution::Value {
                amount: "1".into(),
                set_by: SetBy::Asserted {
                    rule: Rule {
                        registry: "1".into(),
                        id: "asserted".into(),
                    },
                    filing: "0000320193-25-000079".into(),
                },
            }),
        )
    }

    /// The history of one filer holding a row at each of `periods`.
    fn history(periods: impl IntoIterator<Item = Period>) -> History {
        History::of("0000320193".into(), periods.into_iter().map(row).collect())
    }

    /// The history of one filer holding a row at each duration `dates` names.
    fn durations(dates: &[(&str, &str)]) -> History {
        history(dates.iter().map(|(start, end)| duration(start, end)))
    }

    /// What the rule says of a duration: a year, with the period of the year
    /// before it or the reason none is found, or the one condition it is not
    /// a year under.
    #[derive(Debug, PartialEq)]
    enum Answer {
        Year(Result<Period, Reason>),
        Not(Reason),
    }

    /// What the rule says of the duration `start` to `end` among `history`'s
    /// periods.
    fn answer(history: &History, start: &str, end: &str) -> Answer {
        let rows = history.periods();
        let at = rows
            .iter()
            .find(|row| *row.period() == duration(start, end))
            .expect("the duration is in the history");
        let settings = Settings::preset();
        let computing = Computing::at(at, rows, &settings);
        match year(&computing) {
            Ok(year) => Answer::Year(year.before().map(|row| row.period().clone())),
            Err(reason) => Answer::Not(reason),
        }
    }

    fn declined(condition: &str) -> Answer {
        Answer::Not(Reason::declined(condition.into()))
    }

    fn a_year_before(start: &str, end: &str) -> Answer {
        Answer::Year(Ok(duration(start, end)))
    }

    /// A year, `start` to `end`, whose year before is not found.
    fn no_year_before(start: &str, end: &str) -> Answer {
        Answer::Year(Err(Reason::no_period(
            duration(start, end),
            PRECEDING_YEAR.into(),
        )))
    }

    const YEARS: [(&str, &str); 7] = [
        ("2024-01-01", "2024-12-31"),
        ("2023-10-01", "2024-09-30"),
        ("2023-03-01", "2024-02-29"),
        ("2024-03-01", "2025-02-28"),
        ("2023-10-01", "2024-09-28"),
        ("2023-01-01", "2023-12-30"),
        ("2022-09-25", "2023-09-30"),
    ];

    const NOT_YEARS: [(&str, &str); 3] = [
        ("2023-05-01", "2023-12-31"),
        ("2024-07-15", "2025-07-14"),
        ("2023-09-01", "2024-11-30"),
    ];

    #[test]
    fn the_names_and_conditions_are_spelled_as_the_record_spells_them() {
        assert_eq!(PRECEDING_YEAR, "preceding_year");
        assert_eq!(PRECEDING_OPENING_INSTANT, "preceding_opening_instant");
        assert_eq!(END_NOT_A_DATE, "end_not_a_date");
        assert_eq!(NOT_A_YEAR, "not_a_year");
        assert_eq!(OVERLAPS_ANOTHER_YEAR, "overlaps_another_year");
    }

    #[test]
    fn each_length_clause_makes_a_year() {
        for (start, end) in YEARS {
            let alone = durations(&[(start, end)]);
            assert_eq!(
                answer(&alone, start, end),
                no_year_before(start, end),
                "{start} {end}"
            );
        }
    }

    #[test]
    fn a_length_neither_clause_holds_is_not_a_year() {
        for (start, end) in NOT_YEARS {
            let alone = durations(&[(start, end)]);
            assert_eq!(
                answer(&alone, start, end),
                declined(NOT_A_YEAR),
                "{start} {end}"
            );
        }
    }

    #[test]
    fn a_date_outside_the_grammar_gives_its_condition_start_first() {
        let cases = [
            ("2024-1-01", "2024-12-31", START_NOT_A_DATE),
            ("0000-01-01", "0000-12-31", START_NOT_A_DATE),
            ("2024-1-01", "2024-12-1", START_NOT_A_DATE),
            ("2024-01-01", "2024-12-1", END_NOT_A_DATE),
            ("2024-01-01", "2024-02-30", END_NOT_A_DATE),
        ];
        for (start, end, condition) in cases {
            let alone = durations(&[(start, end)]);
            assert_eq!(
                answer(&alone, start, end),
                declined(condition),
                "{start} {end}"
            );
        }
    }

    #[test]
    fn a_length_that_is_not_a_year_gives_not_a_year_though_it_overlaps_a_year() {
        let history = durations(&[
            ("2024-01-01", "2024-12-31"),
            ("2024-04-01", "2024-06-30"),
            ("2024-07-15", "2025-07-14"),
        ]);
        assert_eq!(
            answer(&history, "2024-07-15", "2025-07-14"),
            declined(NOT_A_YEAR)
        );
        assert_eq!(
            answer(&history, "2024-04-01", "2024-06-30"),
            declined(NOT_A_YEAR)
        );
        assert_eq!(
            answer(&history, "2024-01-01", "2024-12-31"),
            no_year_before("2024-01-01", "2024-12-31")
        );
    }

    #[test]
    fn a_duration_outside_the_grammar_never_counts_against_a_year() {
        let history = durations(&[
            ("2024-01-01", "2024-12-31"),
            ("2024-1-01", "2024-12-31"),
            ("2024-01-01", "2024-12-1"),
        ]);
        assert_eq!(
            answer(&history, "2024-01-01", "2024-12-31"),
            no_year_before("2024-01-01", "2024-12-31")
        );
    }

    #[test]
    fn two_years_sharing_one_day_are_neither_alone() {
        let history = durations(&[("2023-01-01", "2023-12-31"), ("2023-12-31", "2024-12-28")]);
        for (start, end) in [("2023-01-01", "2023-12-31"), ("2023-12-31", "2024-12-28")] {
            assert_eq!(
                answer(&history, start, end),
                declined(OVERLAPS_ANOTHER_YEAR),
                "{start} {end}"
            );
        }
    }

    #[test]
    fn two_consecutive_years_find_the_first_before_the_second() {
        let history = durations(&[("2024-01-01", "2024-12-31"), ("2023-01-01", "2023-12-31")]);
        assert_eq!(
            answer(&history, "2024-01-01", "2024-12-31"),
            a_year_before("2023-01-01", "2023-12-31")
        );
    }

    #[test]
    fn the_year_before_missing_is_no_period_at_the_computing_year() {
        let history = durations(&[("2024-01-01", "2024-12-31")]);
        let rows = history.periods();
        let settings = Settings::preset();
        let computing = Computing::at(&rows[0], rows, &settings);
        let year = year(&computing).expect("2024 is a year");
        assert_eq!(
            year.before(),
            Err(Reason::no_period(
                duration("2024-01-01", "2024-12-31"),
                PRECEDING_YEAR.into()
            ))
        );
    }

    #[test]
    fn a_duration_that_is_not_a_year_is_never_the_year_before() {
        for (start, end) in [
            ("2023-10-01", "2023-12-31"),
            ("2023-07-01", "2023-12-31"),
            ("2023-05-01", "2023-12-31"),
        ] {
            let history = durations(&[("2024-01-01", "2024-12-31"), (start, end)]);
            assert_eq!(
                answer(&history, "2024-01-01", "2024-12-31"),
                no_year_before("2024-01-01", "2024-12-31"),
                "{start} {end}"
            );
        }
    }

    #[test]
    fn no_near_year_is_taken() {
        let history = durations(&[("2024-01-01", "2024-12-31"), ("2022-01-01", "2022-12-31")]);
        assert_eq!(
            answer(&history, "2024-01-01", "2024-12-31"),
            no_year_before("2024-01-01", "2024-12-31")
        );
    }

    #[test]
    fn two_overlapping_years_are_neither_taken() {
        let history = durations(&[
            ("2022-05-01", "2023-04-30"),
            ("2023-01-01", "2023-12-31"),
            ("2024-01-01", "2024-12-31"),
        ]);
        assert_eq!(
            answer(&history, "2023-01-01", "2023-12-31"),
            declined(OVERLAPS_ANOTHER_YEAR)
        );
        assert_eq!(
            answer(&history, "2022-05-01", "2023-04-30"),
            declined(OVERLAPS_ANOTHER_YEAR)
        );
        assert_eq!(
            answer(&history, "2024-01-01", "2024-12-31"),
            no_year_before("2024-01-01", "2024-12-31")
        );
    }

    /// The 28 durations of `fixtures/normalize/a-filer-that-changed-its-fiscal-year`,
    /// each with what the record's table says of it.
    fn a_filer_that_changed_its_fiscal_year() -> [((&'static str, &'static str), Answer); 28] {
        [
            (
                ("2021-05-01", "2022-04-30"),
                no_year_before("2021-05-01", "2022-04-30"),
            ),
            (("2022-05-01", "2022-10-31"), declined(NOT_A_YEAR)),
            (
                ("2022-05-01", "2023-04-30"),
                declined(OVERLAPS_ANOTHER_YEAR),
            ),
            (("2022-08-01", "2022-10-31"), declined(NOT_A_YEAR)),
            (("2023-01-01", "2023-03-31"), declined(NOT_A_YEAR)),
            (("2023-01-01", "2023-06-30"), declined(NOT_A_YEAR)),
            (("2023-01-01", "2023-09-30"), declined(NOT_A_YEAR)),
            (
                ("2023-01-01", "2023-12-31"),
                declined(OVERLAPS_ANOTHER_YEAR),
            ),
            (("2023-04-01", "2023-06-30"), declined(NOT_A_YEAR)),
            (("2023-05-01", "2023-10-31"), declined(NOT_A_YEAR)),
            (("2023-05-01", "2023-12-31"), declined(NOT_A_YEAR)),
            (("2023-07-01", "2023-09-30"), declined(NOT_A_YEAR)),
            (("2023-08-01", "2023-10-31"), declined(NOT_A_YEAR)),
            (("2024-01-01", "2024-03-31"), declined(NOT_A_YEAR)),
            (("2024-01-01", "2024-06-30"), declined(NOT_A_YEAR)),
            (("2024-01-01", "2024-09-30"), declined(NOT_A_YEAR)),
            (
                ("2024-01-01", "2024-12-31"),
                no_year_before("2024-01-01", "2024-12-31"),
            ),
            (("2024-04-01", "2024-06-30"), declined(NOT_A_YEAR)),
            (("2024-07-01", "2024-09-30"), declined(NOT_A_YEAR)),
            (("2025-01-01", "2025-03-31"), declined(NOT_A_YEAR)),
            (("2025-01-01", "2025-06-30"), declined(NOT_A_YEAR)),
            (("2025-01-01", "2025-09-30"), declined(NOT_A_YEAR)),
            (
                ("2025-01-01", "2025-12-31"),
                a_year_before("2024-01-01", "2024-12-31"),
            ),
            (("2025-04-01", "2025-06-30"), declined(NOT_A_YEAR)),
            (("2025-07-01", "2025-09-30"), declined(NOT_A_YEAR)),
            (("2026-01-01", "2026-03-31"), declined(NOT_A_YEAR)),
            (("2026-01-01", "2026-06-30"), declined(NOT_A_YEAR)),
            (("2026-04-01", "2026-06-30"), declined(NOT_A_YEAR)),
        ]
    }

    /// The 19 durations of `fixtures/normalize/every-fact-a-filer-reported`,
    /// each with what the record's table says of it.
    fn every_fact_a_filer_reported() -> [((&'static str, &'static str), Answer); 19] {
        [
            (
                ("2022-10-01", "2023-09-30"),
                no_year_before("2022-10-01", "2023-09-30"),
            ),
            (("2023-09-07", "2023-09-30"), declined(NOT_A_YEAR)),
            (("2023-10-01", "2023-12-31"), declined(NOT_A_YEAR)),
            (("2023-10-01", "2024-03-31"), declined(NOT_A_YEAR)),
            (("2023-10-01", "2024-06-30"), declined(NOT_A_YEAR)),
            (
                ("2023-10-01", "2024-09-30"),
                a_year_before("2022-10-01", "2023-09-30"),
            ),
            (("2024-01-01", "2024-03-31"), declined(NOT_A_YEAR)),
            (("2024-04-01", "2024-06-30"), declined(NOT_A_YEAR)),
            (("2024-10-01", "2024-12-31"), declined(NOT_A_YEAR)),
            (("2024-10-01", "2025-03-31"), declined(NOT_A_YEAR)),
            (("2024-10-01", "2025-06-30"), declined(NOT_A_YEAR)),
            (
                ("2024-10-01", "2025-09-30"),
                a_year_before("2023-10-01", "2024-09-30"),
            ),
            (("2025-01-01", "2025-03-31"), declined(NOT_A_YEAR)),
            (("2025-04-01", "2025-06-30"), declined(NOT_A_YEAR)),
            (("2025-10-01", "2025-12-31"), declined(NOT_A_YEAR)),
            (("2025-10-01", "2026-03-31"), declined(NOT_A_YEAR)),
            (("2025-10-01", "2026-06-30"), declined(NOT_A_YEAR)),
            (("2026-01-01", "2026-03-31"), declined(NOT_A_YEAR)),
            (("2026-04-01", "2026-06-30"), declined(NOT_A_YEAR)),
        ]
    }

    /// Each duration of `table`, in a history holding them all in either
    /// order, gives what the table says of it.
    fn gives_its_table(table: &[((&str, &str), Answer)]) {
        let dates: Vec<(&str, &str)> = table.iter().map(|(dates, _)| *dates).collect();
        let reversed: Vec<(&str, &str)> = dates.iter().rev().copied().collect();
        for history in [durations(&dates), durations(&reversed)] {
            for ((start, end), expected) in table {
                assert_eq!(answer(&history, start, end), *expected, "{start} {end}");
            }
        }
    }

    #[test]
    fn the_fiscal_year_change_gives_the_records_table() {
        gives_its_table(&a_filer_that_changed_its_fiscal_year());
    }

    #[test]
    fn every_fact_a_filer_reported_gives_the_records_table() {
        gives_its_table(&every_fact_a_filer_reported());
    }

    const YEAR_2023: (&str, &str) = ("2023-01-01", "2023-12-31");
    const YEAR_2024: (&str, &str) = ("2024-01-01", "2024-12-31");

    /// What the year 2024 reads at the closing and the opening instant of the
    /// year before, in a history holding the years 2023 and 2024 and
    /// `instants`.
    fn instants_of_the_year_before(instants: &[&str]) -> [Result<Period, Reason>; 2] {
        let history = history(
            [YEAR_2024, YEAR_2023]
                .map(|(start, end)| duration(start, end))
                .into_iter()
                .chain(instants.iter().map(|at| instant(at))),
        );
        let rows = history.periods();
        let settings = Settings::preset();
        let computing = Computing::at(&rows[0], rows, &settings);
        let year = year(&computing).expect("2024 is a year");
        [year.closing_of_before(), year.opening_of_before()]
            .map(|read| read.map(|row| row.period().clone()))
    }

    #[test]
    fn the_instants_of_the_year_before_are_its_close_and_the_day_before_its_start() {
        assert_eq!(
            instants_of_the_year_before(&["2022-12-31", "2023-12-31"]),
            [Ok(instant("2023-12-31")), Ok(instant("2022-12-31"))]
        );
    }

    #[test]
    fn a_missing_instant_of_the_year_before_names_which_seek_missed_it() {
        let at = || duration(YEAR_2024.0, YEAR_2024.1);
        assert_eq!(
            instants_of_the_year_before(&["2023-12-31"]),
            [
                Ok(instant("2023-12-31")),
                Err(Reason::no_period(at(), PRECEDING_OPENING_INSTANT.into()))
            ]
        );
        assert_eq!(
            instants_of_the_year_before(&["2022-12-31"]),
            [
                Err(Reason::no_period(at(), OPENING_INSTANT.into())),
                Ok(instant("2022-12-31"))
            ]
        );
        assert_ne!(OPENING_INSTANT, PRECEDING_OPENING_INSTANT);
        assert_ne!(CLOSING_INSTANT, PRECEDING_OPENING_INSTANT);
    }

    #[test]
    fn no_instant_of_a_missing_year_before_is_read() {
        let history = history([
            duration(YEAR_2024.0, YEAR_2024.1),
            duration("2023-10-01", "2023-12-31"),
            instant("2023-12-31"),
            instant("2023-09-30"),
        ]);
        let rows = history.periods();
        let settings = Settings::preset();
        let computing = Computing::at(&rows[0], rows, &settings);
        let year = year(&computing).expect("2024 is a year");
        let missing = || {
            Err(Reason::no_period(
                rows[0].period().clone(),
                PRECEDING_YEAR.into(),
            ))
        };
        assert_eq!(year.closing_of_before().map(Row::period), missing());
        assert_eq!(year.opening_of_before().map(Row::period), missing());
    }

    /// The periods of the series from the year 2022 two years back, in a
    /// history holding the calendar years `years`.
    fn series_from_2022(years: &[(&str, &str)]) -> Vec<Result<Period, Reason>> {
        let history = durations(years);
        let rows = history.periods();
        let settings = Settings::preset();
        let computing = Computing::at(&rows[0], rows, &settings);
        let year = year(&computing).expect("2022 is a year");
        year.series(2)
            .map(|read| read.map(|row| row.period().clone()))
            .collect()
    }

    #[test]
    fn a_series_reaches_each_year_through_the_one_after_it() {
        assert_eq!(
            series_from_2022(&[
                ("2022-01-01", "2022-12-31"),
                ("2020-01-01", "2020-12-31"),
                ("2021-01-01", "2021-12-31"),
                ("2019-01-01", "2019-12-31"),
            ]),
            [
                Ok(duration("2022-01-01", "2022-12-31")),
                Ok(duration("2021-01-01", "2021-12-31")),
                Ok(duration("2020-01-01", "2020-12-31")),
            ]
        );
    }

    #[test]
    fn a_series_never_steps_around_a_gap() {
        let gap = || {
            Err(Reason::no_period(
                duration("2022-01-01", "2022-12-31"),
                PRECEDING_YEAR.into(),
            ))
        };
        assert_eq!(
            series_from_2022(&[
                ("2022-01-01", "2022-12-31"),
                ("2020-01-01", "2020-12-31"),
                ("2019-01-01", "2019-12-31"),
            ]),
            [Ok(duration("2022-01-01", "2022-12-31")), gap(), gap()]
        );
    }

    #[test]
    fn a_series_never_takes_a_duration_that_is_not_a_year_in_a_years_place() {
        let gap = || {
            Err(Reason::no_period(
                duration("2022-01-01", "2022-12-31"),
                PRECEDING_YEAR.into(),
            ))
        };
        assert_eq!(
            series_from_2022(&[
                ("2022-01-01", "2022-12-31"),
                ("2021-10-01", "2021-12-31"),
                ("2021-07-01", "2021-09-30"),
                ("2020-01-01", "2020-12-31"),
            ]),
            [Ok(duration("2022-01-01", "2022-12-31")), gap(), gap()]
        );
    }

    #[test]
    fn a_repeated_year_before_is_taken_at_its_first_row() {
        let history = durations(&[YEAR_2024, YEAR_2023, YEAR_2023]);
        let rows = history.periods();
        let settings = Settings::preset();
        let computing = Computing::at(&rows[0], rows, &settings);
        let year = year(&computing).expect("2024 is a year");
        assert!(std::ptr::eq(
            year.before().expect("2023 is the year before"),
            &rows[1]
        ));
    }
}
