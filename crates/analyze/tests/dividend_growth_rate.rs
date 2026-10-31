//! The dividend growth rate as a caller reaches it: a history in, and at each
//! year the compound rate per year over `dividend_growth_years` years ending
//! there, read at that year and the base year alone, or the reasons it has
//! none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
};

const CIK: &str = "0000320193";

const NAME: &str = "dividend_growth_rate";

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
/// every other concept a value the rate does not read.
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

/// The calendar years 2019 to 2024 of a dividend growing at a tenth a year
/// from one.
fn growing() -> Vec<Row> {
    [
        (2019, "1"),
        (2020, "1.1"),
        (2021, "1.21"),
        (2022, "1.331"),
        (2023, "1.4641"),
        (2024, "1.61051"),
    ]
    .into_iter()
    .map(|(year, amount)| row(calendar(year), value(amount)))
    .collect()
}

/// `rows` with the row at the calendar year `year` replaced by one whose
/// dividend is `dividend`.
fn with(rows: Vec<Row>, year: u32, dividend: Resolution) -> Vec<Row> {
    rows.into_iter()
        .map(|stated| {
            if *stated.period() == calendar(year) {
                row(calendar(year), dividend.clone())
            } else {
                stated
            }
        })
        .collect()
}

/// `rows` without the calendar year `year`.
fn without(rows: Vec<Row>, year: u32) -> Vec<Row> {
    rows.into_iter()
        .filter(|row| *row.period() != calendar(year))
        .collect()
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

/// The rate's outcome at `period`, or none where the results there hold no
/// rate.
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

/// The rate's outcome at `period` among `rows`, under `settings`.
fn rate(rows: Vec<Row>, period: &Period, settings: &Settings) -> Outcome {
    outcome_at(&handed(rows, settings), period).expect("the rate is stated at a duration")
}

/// The rate's outcome at 2024 among `rows`, under the preset.
fn at_2024(rows: Vec<Row>) -> Outcome {
    rate(rows, &calendar(2024), &Settings::preset())
}

fn years(years: u32) -> Settings {
    Settings::preset()
        .with_dividend_growth_years(years)
        .expect("a positive integer")
}

fn figure(characters: &str) -> Outcome {
    Outcome::Value {
        figure: characters.into(),
    }
}

fn absent(first: Reason, rest: Vec<Reason>) -> Outcome {
    Outcome::Absent {
        reasons: Reasons::of(first, rest),
    }
}

fn declined(condition: &str) -> Outcome {
    absent(Reason::declined(condition.into()), Vec::new())
}

fn no_year_before(year: u32) -> Reason {
    Reason::no_period(calendar(year), "preceding_year".into())
}

/// `dividends_declared_per_share` is `Unknown` at `row`.
fn input_unknown(row: &Row) -> Reason {
    Reason::input_unknown(row, Concept::DividendsDeclaredPerShare)
        .expect("the dividend is Unknown at the row")
}

const NEGATIVE: &str = "negative_dividends_declared_per_share";

const ZERO_BASE_YEAR: &str = "zero_base_year_dividends_declared_per_share";

#[test]
fn a_tenth_a_year_over_the_presets_five_years_is_a_tenth() {
    assert_eq!(at_2024(growing()), figure("0.1"));
}

#[test]
fn a_year_between_is_found_and_never_read() {
    assert_eq!(
        at_2024(with(growing(), 2022, value("2.662"))),
        figure("0.1")
    );
    assert_eq!(at_2024(with(growing(), 2022, unknown())), figure("0.1"));
    assert_eq!(
        at_2024(with(growing(), 2022, value("0.2.5"))),
        figure("0.1")
    );
}

#[test]
fn an_excess_at_the_computing_year_reads_as_growth_and_at_the_base_year_as_none() {
    assert_eq!(
        at_2024(with(growing(), 2024, value("2.48832"))),
        figure("0.2")
    );
    assert_eq!(
        at_2024(with(growing(), 2019, value("1.61051"))),
        figure("0")
    );
}

#[test]
fn a_negative_figure_at_either_year_read_is_the_streaks_condition() {
    for year in [2024, 2019] {
        assert_eq!(
            at_2024(with(growing(), year, value("-0.01"))),
            declined(NEGATIVE),
            "{year}"
        );
    }
}

#[test]
fn a_zero_at_the_base_year_is_its_own_condition() {
    let zero_2019 = with(growing(), 2019, value("0"));
    assert_eq!(at_2024(zero_2019.clone()), declined(ZERO_BASE_YEAR));
    assert_eq!(
        at_2024(with(zero_2019.clone(), 2024, value("0"))),
        declined(ZERO_BASE_YEAR)
    );
    assert_eq!(
        at_2024(with(zero_2019, 2024, value("-0.01"))),
        declined(NEGATIVE)
    );
}

#[test]
fn a_zero_at_the_computing_year_is_minus_one() {
    assert_eq!(at_2024(with(growing(), 2024, value("0"))), figure("-1"));
}

#[test]
fn characters_outside_the_grammar_at_both_years_read_are_two_reasons_in_read_order() {
    let unreadable = with(with(growing(), 2024, value("0.2.5")), 2019, value("1e0"));
    let not_a_decimal = || Reason::declined("input_not_a_decimal".into());
    assert_eq!(
        at_2024(unreadable),
        absent(not_a_decimal(), vec![not_a_decimal()])
    );
}

#[test]
fn a_missing_year_between_is_no_period_once_and_never_a_shorter_window() {
    let without_2021 = without(growing(), 2021);
    assert_eq!(
        at_2024(without_2021.clone()),
        absent(no_year_before(2024), Vec::new())
    );
    let unknown_2024 = with(without_2021, 2024, unknown());
    let reason = input_unknown(&unknown_2024[4]);
    assert_eq!(
        at_2024(unknown_2024),
        absent(reason, vec![no_year_before(2024)])
    );
}

#[test]
fn a_quarter_in_a_years_place_is_a_gap_and_is_not_a_year() {
    let quarter = duration("2021-10-01", "2021-12-31");
    let rows: Vec<Row> = without(growing(), 2021)
        .into_iter()
        .chain([row(quarter.clone(), value("1.21"))])
        .collect();
    let hand_over = handed(rows, &Settings::preset());
    assert_eq!(
        outcome_at(&hand_over, &calendar(2024)),
        Some(absent(no_year_before(2024), Vec::new()))
    );
    assert_eq!(
        outcome_at(&hand_over, &quarter),
        Some(declined("not_a_year"))
    );
}

#[test]
fn the_suspension_fixtures_years_give_the_records_rates_over_two_years() {
    let rows = suspension();
    let unknown_2023 = input_unknown(&rows[5]);
    let hand_over = handed(rows, &years(2));
    for (year, outcome) in [
        (2018, absent(no_year_before(2018), Vec::new())),
        (2019, absent(no_year_before(2019), Vec::new())),
        (2020, declined(ZERO_BASE_YEAR)),
        (2021, declined(ZERO_BASE_YEAR)),
        (2022, figure("0.154700538379251529018297561003915")),
        (2023, absent(unknown_2023, Vec::new())),
        (2024, figure("-1")),
    ] {
        assert_eq!(
            outcome_at(&hand_over, &calendar(year)),
            Some(outcome),
            "{year}"
        );
    }
}

#[test]
fn the_suspension_fixtures_years_give_the_records_rates_over_one_year_and_the_preset() {
    let hand_over = handed(suspension(), &years(1));
    assert_eq!(
        outcome_at(&hand_over, &calendar(2022)),
        Some(figure("-0.2"))
    );
    assert_eq!(
        outcome_at(&hand_over, &calendar(2021)),
        Some(figure("0.666666666666666666666666666666667"))
    );
    assert_eq!(at_2024(suspension()), declined(ZERO_BASE_YEAR));
}

#[test]
fn a_window_far_past_the_history_is_one_no_period_and_the_call_returns() {
    assert_eq!(
        rate(suspension(), &calendar(2024), &years(u32::MAX)),
        absent(no_year_before(2024), Vec::new())
    );
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let period = duration("2024-1-01", "2024-12-31");
    assert_eq!(
        rate(
            vec![row(period.clone(), unknown())],
            &period,
            &Settings::preset()
        ),
        declined("start_not_a_date")
    );
}

#[test]
fn the_rate_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant {
        at: "2024-12-31".into(),
    };
    let rows: Vec<Row> = growing()
        .into_iter()
        .chain([row(instant.clone(), value("0.25"))])
        .collect();
    let hand_over = handed(rows, &Settings::preset());
    assert_eq!(outcome_at(&hand_over, &instant), None);
    for year in 2019..=2024 {
        assert!(outcome_at(&hand_over, &calendar(year)).is_some(), "{year}");
    }
}

#[test]
fn the_setting_arrives_last_among_the_premises_and_the_version_stays() {
    let premises = handed(growing(), &years(3)).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    let recorded: Vec<(&str, &str)> = premises
        .settings()
        .iter()
        .map(|setting| (&*setting.name, &*setting.value))
        .collect();
    assert_eq!(
        recorded,
        [
            ("dividend_streak_years", "10"),
            ("discounted_cash_flow_discount_rate", "0.0772"),
            ("discounted_cash_flow_growth_rate", "0.02"),
            ("discounted_cash_flow_years", "5"),
            ("discounted_cash_flow_terminal_growth_rate", "0.02"),
            ("dividend_growth_years", "3"),
        ]
    );
}

#[test]
fn zero_years_are_refused_naming_the_domain_and_the_value() {
    let refused = Settings::preset()
        .with_dividend_growth_years(0)
        .expect_err("zero is not a positive integer");
    assert_eq!(
        (refused.domain(), refused.value()),
        ("the positive integers", "0")
    );
}
