//! How a metric reads its inputs: each at the row it computes at, except a
//! balance a duration metric reads, which is read at the instant the duration
//! closes at, opens at, or the two averaged, sought by its date and by no
//! other row. Why each, and how absence composes across the reads, is
//! `docs/adr/which-other-period-a-metric-reads.md`'s, under the method
//! version.
//!
//! This is the one place the derivation reaches the history. A metric is
//! handed the row it computes at, the history beside it and the settings it
//! is computed under as one [`Computing`], whose rows only this module reads,
//! so no metric finds a row itself.

use vfi_contracts::analyze_store::{Reason, Reasons};
use vfi_contracts::canonical_concepts::{Concept, Period, Resolution, Row};

use super::Derivation;
use crate::arithmetic::{INPUT_NOT_A_DECIMAL, Number};
use crate::date::{START_NOT_A_DATE, day_before};
use crate::settings::Settings;
use year::Year;

pub(super) mod year;

/// The `sought_by` of a read whose duration's opening instant the history
/// does not hold.
const OPENING_INSTANT: &str = "opening_instant";

/// The `sought_by` of a read whose duration's closing instant the history
/// does not hold.
const CLOSING_INSTANT: &str = "closing_instant";

/// The row a metric computes at, the history beside it, and the settings
/// it is computed under.
pub(super) struct Computing<'a> {
    row: &'a Row,
    history: &'a [Row],
    settings: &'a Settings,
}

impl<'a> Computing<'a> {
    /// At `row`, one of `history`'s periods, under `settings`.
    pub(super) fn at(row: &'a Row, history: &'a [Row], settings: &'a Settings) -> Self {
        Computing {
            row,
            history,
            settings,
        }
    }

    /// The settings handed to analyze, which are the only settings a metric
    /// reads.
    pub(super) fn settings(&self) -> &'a Settings {
        self.settings
    }

    /// The dates the duration computed at starts and ends at.
    fn duration(&self) -> (&'a str, &'a str) {
        match self.row.period() {
            Period::Duration { start, end } => (start, end),
            Period::Instant { .. } => {
                unreachable!("a reading is named only by a metric stated at a duration")
            }
        }
    }
}

/// The instant, or instants, a balance a duration metric reads is read at.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Reading {
    /// The balance at the instant the duration closes at.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no metric reads the closing balance alone yet")
    )]
    Closing,
    /// The balance at the instant the duration opens at.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "no metric reads the opening balance alone yet")
    )]
    Opening,
    /// The mean of the balances at the instants the duration opens and closes
    /// at.
    Averaged,
}

impl Reading {
    /// The instants the reading reads, in read order.
    fn seeks(self) -> &'static [Seek] {
        match self {
            Reading::Closing => &[Seek::Closing],
            Reading::Opening => &[Seek::Opening],
            Reading::Averaged => &[Seek::Opening, Seek::Closing],
        }
    }
}

/// One of the two instants a duration's balance is read at.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seek {
    Opening,
    Closing,
}

impl Seek {
    /// The name a `no_period` carries as `sought_by` where the history lacks
    /// the row this seeks.
    fn name(self) -> &'static str {
        match self {
            Seek::Opening => OPENING_INSTANT,
            Seek::Closing => CLOSING_INSTANT,
        }
    }

    /// The history's first row at the instant this seeks for the duration
    /// `computing` is at, or the one reason there is none.
    ///
    /// The closing instant is dated the duration's end, read as characters
    /// and not as a date. The opening instant is dated the day before the
    /// duration's start, which `START_NOT_A_DATE` declines where the start
    /// states none. Either is found by equality of characters alone: nothing
    /// is ordered, and no near row stands in for a missing one.
    fn row<'a>(self, computing: &Computing<'a>) -> Result<&'a Row, Reason> {
        let (start, end) = computing.duration();
        let date = match self {
            Seek::Opening => {
                day_before(start).ok_or_else(|| Reason::declined(START_NOT_A_DATE.into()))?
            }
            Seek::Closing => end.into(),
        };
        instant(computing.history, &date)
            .ok_or_else(|| Reason::no_period(computing.row.period().clone(), self.name().into()))
    }
}

/// `history`'s first row at the instant dated `date`, character for
/// character, or none.
fn instant<'a>(history: &'a [Row], date: &str) -> Option<&'a Row> {
    history
        .iter()
        .find(|row| matches!(row.period(), Period::Instant { at } if **at == *date))
}

/// What one input's reads give, in read order: one number for an input read
/// at the row, and one for each instant its reading reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Reads {
    reading: Option<Reading>,
    numbers: Vec<Number>,
}

impl Reads {
    /// The number the input states as its derivation reads it: the mean of
    /// its reads, which at one read is that read.
    pub(super) fn mean(&self) -> Number {
        mean(self.numbers.iter().cloned())
    }

    /// `operation` taken read by read over `self` and `other`, which were read
    /// at the same rows: a term two inputs form at each row, before any mean.
    pub(super) fn with(&self, other: &Reads, operation: fn(&Number, &Number) -> Number) -> Reads {
        assert_eq!(
            self.reading, other.reading,
            "a term is formed of inputs read at the same rows"
        );
        Reads {
            reading: self.reading,
            numbers: self
                .numbers
                .iter()
                .zip(&other.numbers)
                .map(|(one, another)| operation(one, another))
                .collect(),
        }
    }
}

/// The number each of `derivation`'s inputs states, in their order, each read
/// as the derivation names; or, where any read states none, a reason for each
/// read that states none, in read order.
///
/// `N` is how many numbers the metric takes, which is how many inputs its
/// entry lists.
pub(super) fn read<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
) -> Result<[Number; N], Reasons> {
    Ok(reads(computing, derivation)?.map(|reads| reads.mean()))
}

/// What each of `derivation`'s inputs' reads give, in their order, each read
/// as the derivation names; or, where any read states none, a reason for each
/// read that states none, in read order. Every read is made, none stops
/// another, and nothing is substituted for an input.
///
/// `N` is how many inputs the metric's entry lists.
pub(super) fn reads<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
) -> Result<[Reads; N], Reasons> {
    reads_except(computing, derivation, None)
}

/// As [`reads`], except that `nothing`, at a row where it is `NotApplicable`,
/// is read as zero: summed as nothing there, with the row still counted.
/// Every other state of it, and every other concept, is read as [`reads`]
/// reads it.
pub(super) fn reads_with_nothing<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
    nothing: Concept,
) -> Result<[Reads; N], Reasons> {
    reads_except(computing, derivation, Some(nothing))
}

fn reads_except<const N: usize>(
    computing: &Computing,
    derivation: &Derivation,
    nothing: Option<Concept>,
) -> Result<[Reads; N], Reasons> {
    let (stated, absent): (Vec<_>, Vec<_>) = derivation
        .entry
        .inputs
        .iter()
        .zip(derivation.readings)
        .map(|(concept, reading)| {
            match reading {
                None => input_or_nothing(computing.row, *concept, nothing)
                    .map(|number| vec![number])
                    .map_err(|reason| vec![reason]),
                Some(reading) => balance(computing, *concept, *reading, nothing),
            }
            .map(|numbers| Reads {
                reading: *reading,
                numbers,
            })
        })
        .partition(Result::is_ok);
    let mut reasons = absent.into_iter().filter_map(Result::err).flatten();
    if let Some(first) = reasons.next() {
        return Err(Reasons::of(first, reasons.collect()));
    }
    let stated: Vec<Reads> = stated.into_iter().filter_map(Result::ok).collect();
    Ok(stated
        .try_into()
        .expect("a metric takes a number for each input its entry lists"))
}

/// The balances `concept` states at each instant `reading` reads, in read
/// order; or a reason for each of those reads that states none, in read
/// order.
fn balance(
    computing: &Computing,
    concept: Concept,
    reading: Reading,
    nothing: Option<Concept>,
) -> Result<Vec<Number>, Vec<Reason>> {
    let (balances, absent): (Vec<_>, Vec<_>) = reading
        .seeks()
        .iter()
        .map(|seek| {
            seek.row(computing)
                .and_then(|row| input_or_nothing(row, concept, nothing))
        })
        .partition(Result::is_ok);
    let reasons: Vec<Reason> = absent.into_iter().filter_map(Result::err).collect();
    if !reasons.is_empty() {
        return Err(reasons);
    }
    Ok(balances.into_iter().filter_map(Result::ok).collect())
}

/// What `concept` states at each year of `year`'s series `back` years back,
/// in series order: a number, or the one reason the read gives. A read is
/// made only when its year is asked for, so a metric whose reads are defined
/// by the figures read makes none past where it stops.
pub(super) fn yearly<'a>(
    year: &Year<'_, 'a>,
    concept: Concept,
    back: usize,
) -> impl Iterator<Item = Result<Number, Reason>> + use<'a> {
    year.series(back)
        .map(move |found| found.and_then(|row| input(row, concept)))
}

/// The sum of `numbers` divided by how many were summed, the count starting
/// at zero and stepping by one.
fn mean(numbers: impl Iterator<Item = Number>) -> Number {
    let (sum, count) = numbers.fold((Number::zero(), Number::zero()), |(sum, count), number| {
        (sum.plus(&number), count.plus(&Number::one()))
    });
    sum.divided(&count).expect("an input is read at least once")
}

/// The number `concept` states at `row`, as [`input`] reads it, except that
/// where `concept` is `nothing` and `NotApplicable` there it is zero.
fn input_or_nothing(
    row: &Row,
    concept: Concept,
    nothing: Option<Concept>,
) -> Result<Number, Reason> {
    match row.of(concept) {
        Resolution::NotApplicable { .. } if nothing == Some(concept) => Ok(Number::zero()),
        _ => input(row, concept),
    }
}

/// The number `concept` states at `row`, or why it states none: the state it
/// is in where that is no `Value`, and `INPUT_NOT_A_DECIMAL` where its
/// characters are outside the reader's grammar.
fn input(row: &Row, concept: Concept) -> Result<Number, Reason> {
    match row.of(concept) {
        Resolution::Value { amount, .. } => {
            Number::read(amount).ok_or_else(|| Reason::declined(INPUT_NOT_A_DECIMAL.into()))
        }
        Resolution::NotApplicable { .. } => Err(Reason::input_not_applicable(row, concept)
            .expect("the concept is NotApplicable at the row")),
        Resolution::Unknown { .. } => {
            Err(Reason::input_unknown(row, concept).expect("the concept is Unknown at the row"))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};
    use vfi_contracts::canonical_concepts::{
        Concept, Measure, Period, Resolution, Row, Rule, SetBy,
    };

    use super::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use super::{CLOSING_INSTANT, Computing, OPENING_INSTANT, Reading, Seek, read};
    use crate::arithmetic::Number;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::{DERIVATIONS, Derivation, Entry, Shape, Unit};
    use crate::settings::Settings;

    fn instant(at: &str) -> Period {
        Period::Instant { at: at.into() }
    }

    fn duration(start: &str, end: &str) -> Period {
        Period::Duration {
            start: start.into(),
            end: end.into(),
        }
    }

    /// A row at `period` where every concept states `amount`.
    fn row(period: Period, amount: &str) -> Row {
        Row::at(
            period,
            std::array::from_fn(|_| Resolution::Value {
                amount: amount.into(),
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

    /// Net income over total assets, read under `reading`.
    const fn over_total_assets(reading: Reading) -> Derivation {
        Derivation {
            entry: Entry {
                name: "over_total_assets",
                meaning: "Net income over total assets as the reading reads them.",
                unit: Unit::FractionOfOne,
                stated_at: Shape::Duration,
                inputs: &[Concept::NetIncome, Concept::TotalAssets],
                conditions: &[START_NOT_A_DATE],
            },
            readings: match reading {
                Reading::Closing => &[None, Some(Reading::Closing)],
                Reading::Opening => &[None, Some(Reading::Opening)],
                Reading::Averaged => &[None, Some(Reading::Averaged)],
            },
            derived: |_| unreachable!("the reads are what is under test"),
        }
    }

    /// The two numbers `reading` reads at the duration `history` holds first.
    fn read_under(reading: Reading, history: &[Row]) -> Result<[Number; 2], Reasons> {
        read(
            &Computing::at(&history[0], history, &Settings::preset()),
            &over_total_assets(reading),
        )
    }

    fn numbers(amounts: [&str; 2]) -> [Number; 2] {
        amounts.map(|amount| Number::read(amount).expect("a decimal"))
    }

    #[test]
    fn the_two_seeks_are_named_as_the_record_names_them() {
        assert_eq!(OPENING_INSTANT, "opening_instant");
        assert_eq!(CLOSING_INSTANT, "closing_instant");
        assert_eq!(Seek::Opening.name(), OPENING_INSTANT);
        assert_eq!(Seek::Closing.name(), CLOSING_INSTANT);
    }

    #[test]
    fn each_reading_reads_its_instants_in_read_order() {
        assert_eq!(Reading::Closing.seeks(), [Seek::Closing]);
        assert_eq!(Reading::Opening.seeks(), [Seek::Opening]);
        assert_eq!(Reading::Averaged.seeks(), [Seek::Opening, Seek::Closing]);
    }

    #[test]
    fn a_closing_reading_reads_the_balance_at_the_end_alone() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2024-12-31"), "110"),
        ];
        assert_eq!(
            read_under(Reading::Closing, &history),
            Ok(numbers(["10", "110"]))
        );
        let at = history[0].period().clone();
        assert_eq!(
            read_under(Reading::Opening, &history),
            Err(Reasons::of(
                Reason::no_period(at.clone(), OPENING_INSTANT.into()),
                Vec::new()
            ))
        );
        assert_eq!(
            read_under(Reading::Averaged, &history),
            Err(Reasons::of(
                Reason::no_period(at, OPENING_INSTANT.into()),
                Vec::new()
            ))
        );
    }

    #[test]
    fn an_opening_reading_reads_the_balance_at_the_day_before_the_start_alone() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2023-12-31"), "90"),
        ];
        assert_eq!(
            read_under(Reading::Opening, &history),
            Ok(numbers(["10", "90"]))
        );
        let at = history[0].period().clone();
        assert_eq!(
            read_under(Reading::Closing, &history),
            Err(Reasons::of(
                Reason::no_period(at.clone(), CLOSING_INSTANT.into()),
                Vec::new()
            ))
        );
        assert_eq!(
            read_under(Reading::Averaged, &history),
            Err(Reasons::of(
                Reason::no_period(at, CLOSING_INSTANT.into()),
                Vec::new()
            ))
        );
    }

    #[test]
    fn an_averaged_reading_reads_the_mean_of_the_two() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2024-12-31"), "110"),
            row(instant("2023-12-31"), "90"),
        ];
        assert_eq!(
            read_under(Reading::Averaged, &history),
            Ok(numbers(["10", "100"]))
        );
    }

    #[test]
    fn a_start_outside_the_grammar_declines_the_opening_and_still_matches_the_closing() {
        let history = [
            row(duration("2024-1-01", "2024-12-31"), "10"),
            row(instant("2024-12-31"), "110"),
            row(instant("2023-12-31"), "90"),
        ];
        let declined = || Reason::declined(START_NOT_A_DATE.into());
        assert_eq!(
            read_under(Reading::Opening, &history),
            Err(Reasons::of(declined(), Vec::new()))
        );
        assert_eq!(
            read_under(Reading::Averaged, &history),
            Err(Reasons::of(declined(), Vec::new()))
        );
        assert_eq!(
            read_under(Reading::Closing, &history),
            Ok(numbers(["10", "110"]))
        );
    }

    #[test]
    fn an_end_outside_the_grammar_still_matches_an_instant_written_the_same_way() {
        let history = [
            row(duration("2024-01-01", "2024-12-1"), "10"),
            row(instant("2024-12-1"), "110"),
        ];
        assert_eq!(
            read_under(Reading::Closing, &history),
            Ok(numbers(["10", "110"]))
        );
    }

    #[test]
    fn a_repeated_instant_is_read_at_its_first_row() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2024-12-31"), "110"),
            row(instant("2024-12-31"), "120"),
        ];
        assert_eq!(
            read_under(Reading::Closing, &history),
            Ok(numbers(["10", "110"]))
        );
    }

    #[test]
    fn every_balance_a_duration_metric_reads_has_a_reading_and_no_other_input_has_one() {
        for derivation in DERIVATIONS {
            let entry = &derivation.entry;
            assert_eq!(
                derivation.readings.len(),
                entry.inputs.len(),
                "{}",
                entry.name
            );
            for (concept, reading) in entry.inputs.iter().zip(derivation.readings) {
                let balance = concept.definition().measure == Measure::Balance;
                let named = match entry.stated_at {
                    Shape::Duration => balance,
                    Shape::Instant => false,
                };
                assert_eq!(reading.is_some(), named, "{} reads {concept:?}", entry.name);
            }
        }
    }

    #[test]
    fn a_metric_lists_start_not_a_date_exactly_where_it_reads_an_opening_or_tests_for_a_year() {
        for derivation in DERIVATIONS {
            let reads_an_opening = derivation
                .readings
                .iter()
                .flatten()
                .any(|reading| reading.seeks().contains(&Seek::Opening));
            let conditions = derivation.entry.conditions;
            let tests_for_a_year = conditions.contains(&NOT_A_YEAR);
            assert_eq!(
                conditions.contains(&START_NOT_A_DATE),
                reads_an_opening || tests_for_a_year,
                "{}",
                derivation.entry.name
            );
            for condition in [END_NOT_A_DATE, OVERLAPS_ANOTHER_YEAR] {
                assert_eq!(
                    conditions.contains(&condition),
                    tests_for_a_year,
                    "{} {condition}",
                    derivation.entry.name
                );
            }
        }
    }

    #[test]
    fn every_metric_that_gives_start_not_a_date_lists_it() {
        let history = [
            row(duration("2024-1-01", "2024-12-31"), "7"),
            row(instant("2024-12-31"), "7"),
            row(instant("2023-12-31"), "7"),
        ];
        let settings = Settings::preset();
        let computing = Computing::at(&history[0], &history, &settings);
        let declined = Reason::declined(START_NOT_A_DATE.into());
        let gave: Vec<&str> = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.stated_at == Shape::Duration)
            .filter(|derivation| {
                (derivation.derived)(&computing)
                    .is_err_and(|reasons| reasons.each().any(|reason| *reason == declined))
            })
            .map(|derivation| {
                assert!(
                    derivation.entry.conditions.contains(&START_NOT_A_DATE),
                    "{}",
                    derivation.entry.name
                );
                derivation.entry.name
            })
            .collect();
        assert_eq!(
            gave,
            [
                "return_on_assets",
                "asset_turnover",
                "return_on_equity",
                "return_on_common_equity",
                "return_on_capital_employed",
                "return_on_invested_capital",
                "sustainable_growth_on_common_equity",
                "dividend_streak"
            ]
        );
    }
}
