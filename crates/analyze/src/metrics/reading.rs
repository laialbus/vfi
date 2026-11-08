//! How a metric reads its inputs: each at the row it computes at, except a
//! balance a duration metric reads, which is read at the instant the duration
//! closes at, opens at, or the two averaged, sought by its date and by no
//! other row. Why each, and how absence composes across the reads, is
//! `docs/adr/which-other-period-a-metric-reads.md`'s, under the method
//! version. A metric that compares a year with the year before reads each
//! input at both, the year before's rows found as `year.rs` finds them.
//!
//! This is the one place the derivation reaches the history. A metric is
//! handed the row it computes at, the history beside it and the settings it
//! is computed under as one [`Computing`], whose rows only this module reads,
//! so no metric finds a row itself.

use std::ops::ControlFlow;

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
    Closing,
    /// The balance at the instant the duration opens at.
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

    /// The number the input states at the year before, as
    /// [`read_averaged_with_the_year_before`] read it, `year` being what it
    /// states at the year after. An averaged balance closes the year before
    /// at the instant `year` opens at, whose read stands in the mean.
    fn mean_before(&self, year: &Reads) -> Number {
        match self.reading {
            Some(Reading::Averaged) => {
                mean(self.numbers.iter().chain(year.numbers.first()).cloned())
            }
            _ => self.mean(),
        }
    }

    /// The number the input states at the instant the duration opens at: the
    /// first read of a balance read there.
    pub(super) fn opening(&self) -> Number {
        assert!(
            matches!(self.reading, Some(Reading::Opening | Reading::Averaged)),
            "only a balance read at the opening instant states a number there"
        );
        self.numbers
            .first()
            .cloned()
            .expect("a reading reads at least one instant")
    }

    /// The reads of the year before as the year before's own reading would
    /// give them, `year` being the reads at the year after. An averaged
    /// balance closes the year before at the instant `year` opens at, whose
    /// read is added after the year before's own.
    fn closed_at(self, year: &Reads) -> Reads {
        match self.reading {
            Some(Reading::Averaged) => Reads {
                reading: self.reading,
                numbers: self
                    .numbers
                    .into_iter()
                    .chain(year.numbers.first().cloned())
                    .collect(),
            },
            _ => self,
        }
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
    reads_except(At::Computing(computing), derivation, None)
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
    reads_except(At::Computing(computing), derivation, Some(nothing))
}

/// The numbers each of `derivation`'s inputs states at `year` and at the
/// year before it, each in their order and each read as the derivation names;
/// or, where any read states none, a reason for each read that states none.
/// Every read at `year` is made before any at the year before, and the
/// reasons follow that order. Every read is made, none stops another, and
/// nothing is substituted for an input.
///
/// `N` is how many inputs the metric's entry lists.
pub(super) fn read_with_the_year_before<const N: usize>(
    year: &Year,
    derivation: &Derivation,
) -> Result<([Number; N], [Number; N]), Reasons> {
    let (this, before): ([Reads; N], [Reads; N]) = both_years(
        reads_except(At::Computing(year.computing()), derivation, None),
        reads_except(At::YearBefore(year), derivation, None),
    )?;
    Ok((
        this.map(|reads| reads.mean()),
        before.map(|reads| reads.mean()),
    ))
}

/// As [`read_with_the_year_before`], for a derivation that averages a balance
/// at each year. The instant the year before closes at is the instant `year`
/// opens at, one date and one row, so that balance is read once, at `year`,
/// and stands in both means: a missing row there gives one reason and not
/// two. The year before reads only the instant it opens at for that balance,
/// and every other input as [`read_with_the_year_before`] reads it.
pub(super) fn read_averaged_with_the_year_before<const N: usize>(
    year: &Year,
    derivation: &Derivation,
) -> Result<([Number; N], [Number; N]), Reasons> {
    let (this, before): ([Reads; N], [Reads; N]) = both_years(
        reads_except(At::Computing(year.computing()), derivation, None),
        reads_except(At::YearBeforeAveraged(year), derivation, None),
    )?;
    let before = std::array::from_fn(|at| before[at].mean_before(&this[at]));
    Ok((this.map(|reads| reads.mean()), before))
}

/// As [`read_averaged_with_the_year_before`], for a derivation that forms
/// more than one term of an input at each year: each input's reads are given
/// read by read, with an averaged balance's reads at the year before closed at
/// the read made at the instant `year` opens at, so that its mean is the year
/// before's mean and its opening the year before's own. `this_year_alone` is
/// read at `year` and not at the year before, where no term reads it, so the
/// year before gives `M` reads, one fewer than the `N` the entry lists.
pub(super) fn reads_once_with_the_year_before<const N: usize, const M: usize>(
    year: &Year,
    derivation: &Derivation,
    this_year_alone: Concept,
) -> Result<([Reads; N], [Reads; M]), Reasons> {
    let read_before = |(concept, _): &(Concept, Option<Reading>)| *concept != this_year_alone;
    let (this, before) = both_years(
        reads_of(At::Computing(year.computing()), inputs(derivation), None),
        reads_of(
            At::YearBeforeAveraged(year),
            inputs(derivation).filter(read_before),
            None,
        ),
    )?;
    let before: Vec<Reads> = inputs(derivation)
        .zip(&this)
        .filter(|(input, _)| read_before(input))
        .zip(before)
        .map(|((_, year), before)| before.closed_at(year))
        .collect();
    Ok((
        this.try_into()
            .expect("a metric takes the reads of each input its entry lists"),
        before
            .try_into()
            .expect("the year before reads every input but the one it does not"),
    ))
}

/// What each year's reads give, or every reason either gives, the year's
/// before the year before's.
fn both_years<T>(this: Result<T, Reasons>, before: Result<T, Reasons>) -> Result<(T, T), Reasons> {
    match (this, before) {
        (Ok(this), Ok(before)) => Ok((this, before)),
        (Err(reasons), Ok(_)) | (Ok(_), Err(reasons)) => Err(reasons),
        (Err(this), Err(before)) => {
            let mut reasons = this.each().chain(before.each()).cloned();
            let first = reasons.next().expect("reasons hold at least one");
            Err(Reasons::of(first, reasons.collect()))
        }
    }
}

/// Where an input is read: at the period computed at, or at the year before
/// the year computed at, with or without the closing instant of an averaged
/// balance, which [`read_averaged_with_the_year_before`] reads at the year.
#[derive(Clone, Copy)]
enum At<'y, 'c, 'a> {
    Computing(&'c Computing<'a>),
    YearBefore(&'y Year<'c, 'a>),
    YearBeforeAveraged(&'y Year<'c, 'a>),
}

impl<'a> At<'_, '_, 'a> {
    /// The row a flow is read at: the period itself, or the year before,
    /// which is `no_period` with `PRECEDING_YEAR` where the chain finds none.
    fn row(self) -> Result<&'a Row, Reason> {
        match self {
            At::Computing(computing) => Ok(computing.row),
            At::YearBefore(year) | At::YearBeforeAveraged(year) => year.before(),
        }
    }

    /// The instants `reading` reads here, in read order.
    fn seeks(self, reading: Reading) -> &'static [Seek] {
        match (self, reading) {
            (At::YearBeforeAveraged(_), Reading::Averaged) => Reading::Opening.seeks(),
            _ => reading.seeks(),
        }
    }

    /// The row `seek` finds for a balance read at this period: the period's
    /// own instant, or the year before's, which is `no_period` with
    /// `PRECEDING_YEAR` where the chain finds no year before.
    fn instant(self, seek: Seek) -> Result<&'a Row, Reason> {
        match (self, seek) {
            (At::Computing(computing), seek) => seek.row(computing),
            (At::YearBefore(year) | At::YearBeforeAveraged(year), Seek::Opening) => {
                year.opening_of_before()
            }
            (At::YearBefore(year) | At::YearBeforeAveraged(year), Seek::Closing) => {
                year.closing_of_before()
            }
        }
    }
}

fn reads_except<const N: usize>(
    at: At,
    derivation: &Derivation,
    nothing: Option<Concept>,
) -> Result<[Reads; N], Reasons> {
    Ok(reads_of(at, inputs(derivation), nothing)?
        .try_into()
        .expect("a metric takes a number for each input its entry lists"))
}

/// `derivation`'s inputs, each with its reading, in the entry's order.
fn inputs(derivation: &Derivation) -> impl Iterator<Item = (Concept, Option<Reading>)> + '_ {
    derivation
        .entry
        .inputs
        .iter()
        .copied()
        .zip(derivation.readings.iter().copied())
}

/// What each of `inputs` reads give here, in their order; or a reason for
/// each read that states none, in read order.
fn reads_of(
    at: At,
    inputs: impl Iterator<Item = (Concept, Option<Reading>)>,
    nothing: Option<Concept>,
) -> Result<Vec<Reads>, Reasons> {
    let (stated, absent): (Vec<_>, Vec<_>) = inputs
        .map(|(concept, reading)| {
            match reading {
                None => at
                    .row()
                    .and_then(|row| input_or_nothing(row, concept, nothing))
                    .map(|number| vec![number])
                    .map_err(|reason| vec![reason]),
                Some(reading) => balance(at, concept, reading, nothing),
            }
            .map(|numbers| Reads { reading, numbers })
        })
        .partition(Result::is_ok);
    let mut reasons = absent.into_iter().filter_map(Result::err).flatten();
    if let Some(first) = reasons.next() {
        return Err(Reasons::of(first, reasons.collect()));
    }
    Ok(stated.into_iter().filter_map(Result::ok).collect())
}

/// The balances `concept` states at each instant `reading` reads, in read
/// order; or a reason for each of those reads that states none, in read
/// order.
fn balance(
    at: At,
    concept: Concept,
    reading: Reading,
    nothing: Option<Concept>,
) -> Result<Vec<Number>, Vec<Reason>> {
    let (balances, absent): (Vec<_>, Vec<_>) = at
        .seeks(reading)
        .iter()
        .map(|seek| {
            at.instant(*seek)
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

/// What `concept` states at `year` and at the last year of its series `back`
/// years back, in that order; or a reason for each of the two reads that
/// states none, in that order. Every year between is found by the series and
/// none is read. The series is walked no further than the first year it does
/// not find, whose `no_period` is then the last year's one reason, however
/// many years beyond it are missing.
pub(super) fn first_and_last(
    year: &Year<'_, '_>,
    concept: Concept,
    back: usize,
) -> Result<(Number, Number), Reasons> {
    let mut series = year.series(back);
    let first = series.next().expect("a series begins at its own year");
    let last = match series.try_fold(first.clone(), |_, found| match found {
        Ok(_) => ControlFlow::Continue(found),
        Err(_) => ControlFlow::Break(found),
    }) {
        ControlFlow::Continue(found) | ControlFlow::Break(found) => found,
    };
    match [first, last].map(|found| found.and_then(|row| input(row, concept))) {
        [Ok(first), Ok(last)] => Ok((first, last)),
        [Err(reason), Ok(_)] | [Ok(_), Err(reason)] => Err(Reasons::of(reason, Vec::new())),
        [Err(first), Err(last)] => Err(Reasons::of(first, vec![last])),
    }
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

    use super::year::{
        END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, PRECEDING_OPENING_INSTANT,
        PRECEDING_YEAR, year,
    };
    use super::{
        CLOSING_INSTANT, Computing, OPENING_INSTANT, Reading, Reads, Seek, read,
        read_averaged_with_the_year_before, read_with_the_year_before,
        reads_once_with_the_year_before,
    };
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

    /// What `reading` reads at the year 2024 and at the year before it, in
    /// `history`, which holds 2024 first.
    fn read_with_the_year_before_under(
        reading: Reading,
        history: &[Row],
    ) -> Result<([Number; 2], [Number; 2]), Reasons> {
        let settings = Settings::preset();
        let computing = Computing::at(&history[0], history, &settings);
        let year = year(&computing).expect("2024 is a year");
        read_with_the_year_before(&year, &over_total_assets(reading))
    }

    #[test]
    fn the_year_before_is_read_at_its_own_row_and_instants_after_the_year() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        for (reading, this, before) in [
            (Reading::Opening, "100", "80"),
            (Reading::Closing, "120", "100"),
            (Reading::Averaged, "110", "90"),
        ] {
            assert_eq!(
                read_with_the_year_before_under(reading, &history),
                Ok((numbers(["10", this]), numbers(["20", before]))),
                "{reading:?}"
            );
        }
    }

    #[test]
    fn every_read_at_a_missing_year_before_is_preceding_year_after_the_years_reasons() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2024-12-31"), "120"),
        ];
        let at = || history[0].period().clone();
        let preceding_year = || Reason::no_period(at(), PRECEDING_YEAR.into());
        assert_eq!(
            read_with_the_year_before_under(Reading::Averaged, &history),
            Err(Reasons::of(
                Reason::no_period(at(), OPENING_INSTANT.into()),
                vec![preceding_year(), preceding_year(), preceding_year()]
            ))
        );
        assert_eq!(
            read_with_the_year_before_under(Reading::Closing, &history),
            Err(Reasons::of(preceding_year(), vec![preceding_year()]))
        );
    }

    #[test]
    fn a_missing_opening_of_the_year_before_is_preceding_opening_instant() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2023-12-31"), "100"),
        ];
        assert_eq!(
            read_with_the_year_before_under(Reading::Opening, &history),
            Err(Reasons::of(
                Reason::no_period(
                    history[0].period().clone(),
                    PRECEDING_OPENING_INSTANT.into()
                ),
                Vec::new()
            ))
        );
    }

    /// What `reading` reads at the year 2024 and at the year before it, with
    /// an averaged balance read once at the instant 2024 opens at, in
    /// `history`, which holds 2024 first.
    fn read_averaged_under(
        reading: Reading,
        history: &[Row],
    ) -> Result<([Number; 2], [Number; 2]), Reasons> {
        let settings = Settings::preset();
        let computing = Computing::at(&history[0], history, &settings);
        let year = year(&computing).expect("2024 is a year");
        read_averaged_with_the_year_before(&year, &over_total_assets(reading))
    }

    #[test]
    fn the_read_at_the_years_opening_stands_in_both_means() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        assert_eq!(
            read_averaged_under(Reading::Averaged, &history),
            Ok((numbers(["10", "110"]), numbers(["20", "90"])))
        );
    }

    #[test]
    fn only_an_averaged_balance_shares_the_read() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        for (reading, this, before) in [
            (Reading::Opening, "100", "80"),
            (Reading::Closing, "120", "100"),
        ] {
            assert_eq!(
                read_averaged_under(reading, &history),
                Ok((numbers(["10", this]), numbers(["20", before]))),
                "{reading:?}"
            );
        }
    }

    #[test]
    fn a_missing_opening_of_the_year_gives_one_reason_and_not_two() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2024-12-31"), "120"),
        ];
        assert_eq!(
            read_averaged_under(Reading::Averaged, &history),
            Err(Reasons::of(
                Reason::no_period(history[0].period().clone(), OPENING_INSTANT.into()),
                Vec::new()
            ))
        );
    }

    #[test]
    fn a_missing_year_before_gives_preceding_year_for_the_reads_made_at_it() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        let preceding_year =
            || Reason::no_period(history[0].period().clone(), PRECEDING_YEAR.into());
        assert_eq!(
            read_averaged_under(Reading::Averaged, &history),
            Err(Reasons::of(preceding_year(), vec![preceding_year()]))
        );
    }

    /// The reads at the year 2024 and at the year before it, each made once,
    /// with net income read at the year alone, in `history`, which holds 2024
    /// first.
    fn reads_once_under(history: &[Row]) -> Result<([Reads; 2], [Reads; 1]), Reasons> {
        let settings = Settings::preset();
        let computing = Computing::at(&history[0], history, &settings);
        let year = year(&computing).expect("2024 is a year");
        reads_once_with_the_year_before(
            &year,
            &over_total_assets(Reading::Averaged),
            Concept::NetIncome,
        )
    }

    #[test]
    fn each_read_is_given_alone_and_the_year_befores_average_closes_at_the_years_opening() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        let ([net_income, total_assets], [preceding_total_assets]) =
            reads_once_under(&history).expect("every read states a number");
        let [ten, eighty, ninety, hundred, hundred_and_ten] =
            ["10", "80", "90", "100", "110"].map(|amount| Number::read(amount).expect("a decimal"));
        assert_eq!(net_income.mean(), ten);
        assert_eq!(total_assets.opening(), hundred);
        assert_eq!(total_assets.mean(), hundred_and_ten);
        assert_eq!(preceding_total_assets.opening(), eighty);
        assert_eq!(preceding_total_assets.mean(), ninety);
    }

    #[test]
    fn an_input_read_at_the_year_alone_gives_no_reason_at_the_year_before() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(instant("2022-12-31"), "80"),
            row(instant("2023-12-31"), "100"),
            row(instant("2024-12-31"), "120"),
        ];
        assert_eq!(
            reads_once_under(&history).map(|_| ()),
            Err(Reasons::of(
                Reason::no_period(history[0].period().clone(), PRECEDING_YEAR.into()),
                Vec::new()
            ))
        );
    }

    #[test]
    fn a_missing_opening_of_the_year_is_read_once_for_both_years() {
        let history = [
            row(duration("2024-01-01", "2024-12-31"), "10"),
            row(duration("2023-01-01", "2023-12-31"), "20"),
            row(instant("2022-12-31"), "80"),
            row(instant("2024-12-31"), "120"),
        ];
        assert_eq!(
            reads_once_under(&history).map(|_| ()),
            Err(Reasons::of(
                Reason::no_period(history[0].period().clone(), OPENING_INSTANT.into()),
                Vec::new()
            ))
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
                "dividend_streak",
                "discounted_cash_flow_value_per_share",
                "dividend_growth_rate",
                "piotroski_f_roa",
                "piotroski_f_cfo",
                "piotroski_f_accrual",
                "piotroski_f_delta_roa",
                "piotroski_f_delta_margin",
                "piotroski_f_delta_turn",
                "piotroski_eq_offer",
                "piotroski_f_delta_lever",
                "piotroski_f_delta_liquid",
                "piotroski_f_score"
            ]
        );
    }
}
