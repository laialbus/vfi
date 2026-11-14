//! The dividend discount value per share as a caller reaches it: a history
//! in, and at each year the mean of the window's declared dividends grown
//! once at the growth setting, over the required return less that rate, or
//! the reasons it has none.
//!
//! The figures are the proof of
//! `docs/adr/dividend-discount-valuation-and-margin-of-safety.md`. Years are
//! calendar years, 2020 to 2024 and computed at 2024 unless a test says
//! otherwise, and figures are listed from 2024 back.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
    Silence,
};

const CIK: &str = "0000320193";

const NAME: &str = "dividend_discount_value_per_share";

const NEGATIVE: &str = "negative_dividends_declared_per_share";

const ZERO: &str = "zero_dividends_declared_per_share";

const NOT_ABOVE: &str = "required_return_not_above_growth_rate";

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
/// every other concept a value the metric does not read.
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

/// The calendar years ending at `last`, each with its figure of `figures`,
/// which lists them from `last` back.
fn years_to<const N: usize>(last: u32, figures: [&str; N]) -> Vec<Row> {
    (0..)
        .zip(figures)
        .map(|(back, amount)| row(calendar(last - back), value(amount)))
        .collect()
}

/// The calendar years 2020 to 2024, with `figures` from 2024 back.
fn years(figures: [&str; 5]) -> Vec<Row> {
    years_to(2024, figures)
}

/// The record's preset row: 15.05 over five years, a base of 3.01.
fn preset_row() -> Vec<Row> {
    years(["3.21", "3.11", "3.01", "2.91", "2.81"])
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
/// dividend declared per share the fixture's history holds for it, as
/// `tests/dividend_streak.rs` builds them.
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

/// The value's outcome at `period`, or none where the results there hold no
/// value.
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

/// The value's outcome at `period` among `rows`, under `settings`.
fn valued(rows: Vec<Row>, period: &Period, settings: &Settings) -> Outcome {
    outcome_at(&handed(rows, settings), period).expect("the value is stated at a duration")
}

/// The value's outcome at 2024 among `rows`, under the preset.
fn at_2024(rows: Vec<Row>) -> Outcome {
    valued(rows, &calendar(2024), &Settings::preset())
}

fn base_years(years: u16) -> Settings {
    Settings::preset()
        .with_dividend_discount_base_years(years)
        .expect("an integer from two up")
}

fn growth_rate(settings: Settings, rate: &str) -> Settings {
    settings
        .with_dividend_discount_growth_rate(rate)
        .expect("a rate above minus one")
}

/// Two base years, with the required return and the growth rate as stated.
fn over_two_years(required_return: &str, growth: &str) -> Settings {
    growth_rate(
        base_years(2)
            .with_dividend_discount_required_return(required_return)
            .expect("a rate above zero"),
        growth,
    )
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

/// The reason of a read at a year the series from `year` did not find, or
/// at one beyond it.
fn no_year_before(year: u32) -> Reason {
    Reason::no_period(calendar(year), "preceding_year".into())
}

/// `dividends_declared_per_share` is `Unknown` at `row`.
fn input_unknown(row: &Row) -> Reason {
    Reason::input_unknown(row, Concept::DividendsDeclaredPerShare)
        .expect("the dividend is Unknown at the row")
}

#[test]
fn the_preset_row_under_the_presets_is_fifty_one() {
    assert_eq!(at_2024(preset_row()), figure("51"));
}

#[test]
fn two_years_at_a_tenth_less_a_twentieth_give_the_records_figure() {
    let rows = with(with(preset_row(), 2024, value("2.2")), 2023, value("2"));
    assert_eq!(
        valued(rows, &calendar(2024), &over_two_years("0.1", "0.05")),
        figure("44.1")
    );
}

#[test]
fn the_sources_illustration_is_pinned_at_its_printed_rate_and_at_the_rate_before_rounding() {
    let rows = || with(with(preset_row(), 2024, value("2.19")), 2023, value("2.19"));
    assert_eq!(
        valued(rows(), &calendar(2024), &over_two_years("0.09", "0.0349")),
        figure("41.13304900181488203266787658802178")
    );
    assert_eq!(
        valued(
            rows(),
            &calendar(2024),
            &over_two_years("0.09", "0.03492489")
        ),
        figure("41.15262791304456768220708047609891")
    );
}

#[test]
fn a_years_excess_counts_once_wherever_in_the_window_it_stands() {
    assert_eq!(
        at_2024(years(["5.05", "2.5", "2.5", "2.5", "2.5"])),
        figure("51")
    );
    assert_eq!(
        at_2024(years(["2.5", "2.5", "2.5", "5.05", "2.5"])),
        figure("51")
    );
    assert_eq!(
        valued(
            years_to(2025, ["2.5", "5.05", "2.5", "2.5", "2.5"]),
            &calendar(2025),
            &Settings::preset()
        ),
        figure("51")
    );
}

#[test]
fn a_zero_at_the_computing_year_is_a_condition_and_never_a_value() {
    assert_eq!(at_2024(years(["0", "1", "1", "1", "1"])), declined(ZERO));
    assert_eq!(at_2024(years(["0", "0", "0", "0", "0"])), declined(ZERO));
}

#[test]
fn a_zero_inside_the_window_counts_in_the_mean() {
    assert_eq!(
        at_2024(years(["3.7625", "3.7625", "0", "3.7625", "3.7625"])),
        figure("51")
    );
}

#[test]
fn the_suspension_fixtures_years_give_the_records_outcomes_over_two_years() {
    let rows = suspension();
    let unknown_2023 = input_unknown(&rows[5]);
    assert_eq!(
        unknown_2023.names(),
        &Names::InputUnknown {
            concept: Concept::DividendsDeclaredPerShare,
            period: calendar(2023),
        }
    );
    let hand_over = handed(rows, &base_years(2));
    for (year, outcome) in [
        (2018, absent(no_year_before(2018), Vec::new())),
        (2019, declined(ZERO)),
        (2020, figure("0.7624584717607973421926910299003322")),
        (2021, figure("2.033222591362126245847176079734219")),
        (2022, figure("2.287375415282392026578073089700997")),
        (2023, absent(unknown_2023.clone(), Vec::new())),
        (2024, absent(unknown_2023.clone(), Vec::new())),
    ] {
        assert_eq!(
            outcome_at(&hand_over, &calendar(year)),
            Some(outcome),
            "{year}"
        );
    }
}

#[test]
fn the_suspension_fixtures_2022_under_the_preset_counts_its_two_zeros() {
    assert_eq!(
        valued(suspension(), &calendar(2022), &Settings::preset()),
        figure("1.219933554817275747508305647840532")
    );
}

#[test]
fn a_negative_figure_at_any_year_read_is_the_streaks_condition_and_is_asked_first() {
    for year in [2024, 2021] {
        assert_eq!(
            at_2024(with(preset_row(), year, value("-0.01"))),
            declined(NEGATIVE),
            "{year}"
        );
    }
    assert_eq!(
        at_2024(with(
            with(preset_row(), 2021, value("-0.01")),
            2024,
            value("0")
        )),
        declined(NEGATIVE)
    );
}

#[test]
fn a_required_return_not_above_the_growth_rate_is_asked_after_a_zero_at_the_computing_year() {
    for rate in ["0.0802", "0.09"] {
        let settings = growth_rate(Settings::preset(), rate);
        assert_eq!(
            valued(preset_row(), &calendar(2024), &settings),
            declined(NOT_ABOVE),
            "{rate}"
        );
        assert_eq!(
            valued(
                with(preset_row(), 2024, value("0")),
                &calendar(2024),
                &settings
            ),
            declined(ZERO),
            "{rate}"
        );
    }
}

#[test]
fn characters_outside_the_grammar_at_two_years_are_two_reasons_in_read_order() {
    let unreadable = with(with(preset_row(), 2024, value("0.2.5")), 2022, value("1e0"));
    let not_a_decimal = || Reason::declined("input_not_a_decimal".into());
    assert_eq!(
        at_2024(unreadable),
        absent(not_a_decimal(), vec![not_a_decimal()])
    );
}

#[test]
fn every_read_is_made_past_a_gap_and_the_window_is_never_shortened() {
    let rows = with(
        with(without(preset_row(), 2021), 2024, unknown()),
        2022,
        unknown(),
    );
    let unknown_2024 = input_unknown(&rows[0]);
    let unknown_2022 = input_unknown(&rows[2]);
    assert_eq!(rows[0].period(), &calendar(2024));
    assert_eq!(rows[2].period(), &calendar(2022));
    assert_eq!(
        at_2024(rows),
        absent(
            unknown_2024,
            vec![unknown_2022, no_year_before(2024), no_year_before(2024)]
        )
    );
}

/// The preset row with the calendar year `year` replaced by its last
/// quarter, and that quarter.
fn with_a_quarter_in_place_of(year: u32) -> (Vec<Row>, Period) {
    let quarter = duration(&format!("{year}-10-01"), &format!("{year}-12-31"));
    let rows = without(preset_row(), year)
        .into_iter()
        .chain([row(quarter.clone(), value("0.75"))])
        .collect();
    (rows, quarter)
}

/// One `no_period` at 2024 for each of `reads` reads at or beyond a year the
/// series did not find.
fn no_year_at_each_of(reads: usize) -> Outcome {
    absent(no_year_before(2024), vec![no_year_before(2024); reads - 1])
}

#[test]
fn a_quarter_never_enters_the_series_and_is_not_a_year() {
    let (rows, quarter) = with_a_quarter_in_place_of(2021);
    let preset = handed(rows.clone(), &Settings::preset());
    assert_eq!(
        outcome_at(&preset, &calendar(2024)),
        Some(no_year_at_each_of(2)),
        "the reads of 2021 and 2020"
    );
    assert_eq!(outcome_at(&preset, &quarter), Some(declined("not_a_year")));

    let six = handed(rows, &base_years(6));
    assert_eq!(
        outcome_at(&six, &calendar(2024)),
        Some(no_year_at_each_of(3)),
        "the reads of 2021, 2020 and 2019"
    );
    assert_eq!(outcome_at(&six, &quarter), Some(declined("not_a_year")));

    let (rows, quarter) = with_a_quarter_in_place_of(2022);
    let preset = handed(rows, &Settings::preset());
    assert_eq!(
        outcome_at(&preset, &calendar(2024)),
        Some(no_year_at_each_of(3)),
        "the reads of 2022, 2021 and 2020"
    );
    assert_eq!(outcome_at(&preset, &quarter), Some(declined("not_a_year")));
}

#[test]
fn a_quarter_whose_figure_is_unknown_gives_not_a_year_alone() {
    let quarter = duration("2024-10-01", "2024-12-31");
    let rows: Vec<Row> = preset_row()
        .into_iter()
        .chain([row(quarter.clone(), unknown())])
        .collect();
    assert_eq!(
        valued(rows, &quarter, &Settings::preset()),
        declined("not_a_year")
    );
}

#[test]
fn the_widest_window_gives_one_no_period_for_each_read_beyond_the_chain() {
    let rows = preset_row();
    let found = rows.len();
    let Outcome::Absent { reasons } = valued(rows, &calendar(2024), &base_years(u16::MAX)) else {
        panic!("a window past the history has no value");
    };
    assert_eq!(reasons.each().count(), usize::from(u16::MAX) - found);
    let beyond = no_year_before(2024);
    assert!(reasons.each().all(|reason| *reason == beyond));
}

#[test]
fn a_filer_of_each_kind_is_valued_from_the_same_row() {
    for kind in [
        Kind::Bank,
        Kind::Insurer,
        Kind::Reit,
        Kind::InvestmentCompany,
    ] {
        assert!(
            Concept::ShortTermInvestments
                .definition()
                .applies_to
                .excluding(kind)
                .is_some(),
            "{kind:?}"
        );
        let rows: Vec<Row> = preset_row()
            .into_iter()
            .map(|stated| {
                Row::at(
                    stated.period().clone(),
                    std::array::from_fn(|at| {
                        let concept = Concept::ALL[at];
                        match concept.definition().applies_to.excluding(kind) {
                            Some(excluded) => Resolution::NotApplicable { excluded },
                            None => stated.of(concept).clone(),
                        }
                    }),
                )
            })
            .collect();
        assert_eq!(at_2024(rows), figure("51"), "{kind:?}");
    }
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let period = duration("2024-1-01", "2024-12-31");
    assert_eq!(
        valued(
            vec![row(period.clone(), unknown())],
            &period,
            &Settings::preset()
        ),
        declined("start_not_a_date")
    );
}

#[test]
fn the_value_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant {
        at: "2024-12-31".into(),
    };
    let rows: Vec<Row> = preset_row()
        .into_iter()
        .chain([row(instant.clone(), value("0.25"))])
        .collect();
    let hand_over = handed(rows, &Settings::preset());
    assert_eq!(outcome_at(&hand_over, &instant), None);
    for year in 2020..=2024 {
        assert!(outcome_at(&hand_over, &calendar(year)).is_some(), "{year}");
    }
}

#[test]
fn adding_the_value_adds_no_setting_and_keeps_the_version() {
    let premises = handed(preset_row(), &Settings::preset()).premises().clone();
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
            ("dividend_growth_years", "5"),
            ("dividend_discount_required_return", "0.0802"),
            ("dividend_discount_growth_rate", "0.02"),
            ("dividend_discount_base_years", "5"),
        ]
    );
}
