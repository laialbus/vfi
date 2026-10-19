//! The dividend streak as a caller reaches it: a history in, and at each year
//! the consecutive paying years ending there, counted to at most
//! `dividend_streak_years`, or the one reason it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
};

const CIK: &str = "0000320193";

const NAME: &str = "dividend_streak";

fn duration(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

/// The calendar year `year`, built by its dates.
fn calendar(year: u32) -> Period {
    duration(&format!("{year}-01-01"), &format!("{year}-12-31"))
}

fn value(amount: &str) -> Resolution {
    Resolution::Value {
        amount: amount.into(),
        set_by: SetBy::Asserted {
            rule: Rule {
                registry: "1".into(),
                id: "asserted".into(),
            },
            filing: "0000320193-25-000079".into(),
        },
    }
}

/// The figure `dividends_declared_per_share`'s conditional silence reading
/// sets where the period is silent on both members of its pair.
fn silent_zero() -> Resolution {
    Resolution::Value {
        amount: "0".into(),
        set_by: SetBy::Silence {
            reading: Silence::Conditional,
            registry: "1".into(),
        },
    }
}

fn unknown() -> Resolution {
    Resolution::Unknown {
        attempted: Attempts::in_filings(
            Attempted::in_filing("0000320193-25-000079".into(), Attempt::that_ran(Vec::new())),
            Vec::new(),
        ),
    }
}

/// A row at `period` with `dividends_declared_per_share` as `dividend`, and
/// every other concept a value the streak does not read.
fn row(period: Period, dividend: Resolution) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| {
            if Concept::ALL[at] == Concept::DividendsDeclaredPerShare {
                dividend.clone()
            } else {
                value("7")
            }
        }),
    )
}

/// The suspension fixture's calendar years 2018 to 2024, each with the
/// dividend declared per share the fixture's history holds for it.
fn suspension() -> Vec<Row> {
    [
        (2018, value("0")),
        (2019, value("0")),
        (2020, value("0.09")),
        (2021, value("0.15")),
        (2022, value("0.12")),
        (2023, unknown()),
        (2024, silent_zero()),
    ]
    .into_iter()
    .map(|(year, dividend)| row(calendar(year), dividend))
    .collect()
}

fn handed(rows: Vec<Row>, settings: &Settings) -> HandOver {
    analyze(History::of(CIK.into(), rows), None, settings).expect("no crossing is handed")
}

/// The streak's outcome at `period`, or none where the results there hold no
/// streak.
fn outcome_at(hand_over: &HandOver, period: &Period) -> Option<Outcome> {
    hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics()
        .iter()
        .find(|metric| &*metric.name == NAME)
        .map(|metric| metric.outcome.clone())
}

/// The streak's outcome at `period` among `rows`, under `settings`.
fn streak(rows: Vec<Row>, period: &Period, settings: &Settings) -> Outcome {
    outcome_at(&handed(rows, settings), period).expect("the streak is stated at a duration")
}

fn figure(characters: &str) -> Outcome {
    Outcome::Value {
        figure: characters.into(),
    }
}

fn absent(first: Reason) -> Outcome {
    Outcome::Absent {
        reasons: Reasons::of(first, Vec::new()),
    }
}

fn declined(condition: &str) -> Outcome {
    absent(Reason::declined(condition.into()))
}

#[test]
fn the_suspension_fixtures_years_give_the_records_streaks() {
    let preset = Settings::preset();
    let hand_over = handed(suspension(), &preset);
    for (year, characters) in [(2022, "3"), (2021, "2"), (2020, "1"), (2019, "0")] {
        assert_eq!(
            outcome_at(&hand_over, &calendar(year)),
            Some(figure(characters)),
            "{year}"
        );
    }
}

#[test]
fn a_zero_set_by_silence_at_the_computing_year_did_not_pay_and_nothing_before_it_is_read() {
    let rows = suspension();
    let at_2023 = rows[5].clone();
    let at_2024 = &rows[6];
    assert!(matches!(
        at_2024.of(Concept::DividendsDeclaredPerShare),
        Resolution::Value {
            set_by: SetBy::Silence { .. },
            ..
        }
    ));
    assert!(matches!(
        at_2023.of(Concept::DividendsDeclaredPerShare),
        Resolution::Unknown { .. }
    ));
    assert_eq!(
        streak(rows, &calendar(2024), &Settings::preset()),
        figure("0")
    );
}

#[test]
fn an_unknown_dividend_at_the_computing_year_is_the_one_reason() {
    let rows = suspension();
    let reason = Reason::input_unknown(&rows[5], Concept::DividendsDeclaredPerShare)
        .expect("the dividend is Unknown in 2023");
    assert_eq!(
        reason.names(),
        &Names::InputUnknown {
            concept: Concept::DividendsDeclaredPerShare,
            period: calendar(2023),
        }
    );
    assert_eq!(
        streak(rows, &calendar(2023), &Settings::preset()),
        absent(reason)
    );
}

#[test]
fn a_zero_at_the_earliest_year_is_a_figure_and_no_year_before_it_is_sought() {
    assert_eq!(
        streak(suspension(), &calendar(2018), &Settings::preset()),
        figure("0")
    );
}

#[test]
fn the_setting_caps_the_count() {
    let two = Settings::preset()
        .with_dividend_streak_years(2)
        .expect("two is a positive integer");
    assert_eq!(streak(suspension(), &calendar(2022), &two), figure("2"));
}

#[test]
fn a_missing_year_is_no_period_once_and_never_a_suspension() {
    let without_2021: Vec<Row> = suspension()
        .into_iter()
        .filter(|row| *row.period() != calendar(2021))
        .collect();
    assert_eq!(
        streak(without_2021, &calendar(2022), &Settings::preset()),
        absent(Reason::no_period(calendar(2022), "preceding_year".into()))
    );
}

#[test]
fn a_run_longer_than_the_setting_reads_as_the_setting() {
    let eleven: Vec<Row> = (2014..=2024)
        .map(|year| row(calendar(year), value("0.25")))
        .collect();
    assert_eq!(
        streak(eleven.clone(), &calendar(2024), &Settings::preset()),
        figure("10")
    );
    let twelve = Settings::preset()
        .with_dividend_streak_years(12)
        .expect("twelve is a positive integer");
    assert_eq!(
        streak(eleven, &calendar(2024), &twelve),
        absent(Reason::no_period(calendar(2024), "preceding_year".into()))
    );
}

#[test]
fn a_quarter_is_not_a_year_and_nothing_at_it_is_read() {
    let quarter = duration("2021-10-01", "2021-12-31");
    for dividend in [silent_zero(), unknown()] {
        let rows: Vec<Row> = suspension()
            .into_iter()
            .chain([row(quarter.clone(), dividend)])
            .collect();
        assert_eq!(
            streak(rows, &quarter, &Settings::preset()),
            declined("not_a_year")
        );
    }
}

#[test]
fn a_negative_dividend_at_a_year_read_is_the_streaks_own_condition() {
    let negative = declined("negative_dividends_declared_per_share");
    for year in [2022, 2020] {
        let rows: Vec<Row> = suspension()
            .into_iter()
            .map(|stated| {
                if *stated.period() == calendar(year) {
                    row(calendar(year), value("-0.01"))
                } else {
                    stated
                }
            })
            .collect();
        assert_eq!(
            streak(rows, &calendar(2022), &Settings::preset()),
            negative,
            "{year}"
        );
    }
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let period = duration("2024-1-01", "2024-12-31");
    assert_eq!(
        streak(
            vec![row(period.clone(), value("0.25"))],
            &period,
            &Settings::preset()
        ),
        declined("start_not_a_date")
    );
}

#[test]
fn a_dividend_outside_the_grammar_is_input_not_a_decimal() {
    assert_eq!(
        streak(
            vec![row(calendar(2024), value("0.2.5"))],
            &calendar(2024),
            &Settings::preset()
        ),
        declined("input_not_a_decimal")
    );
}

#[test]
fn the_streak_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant {
        at: "2024-12-31".into(),
    };
    let rows: Vec<Row> = suspension()
        .into_iter()
        .chain([row(instant.clone(), value("0.25"))])
        .collect();
    let hand_over = handed(rows, &Settings::preset());
    assert_eq!(outcome_at(&hand_over, &instant), None);
    for year in 2018..=2024 {
        assert!(outcome_at(&hand_over, &calendar(year)).is_some(), "{year}");
    }
}
