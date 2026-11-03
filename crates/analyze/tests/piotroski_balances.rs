//! Piotroski's F_ΔLEVER and F_ΔLIQUID as a caller reaches them: a history in,
//! and at each year one or zero from that year's closing balances and the year
//! before's, or the reasons each has none. Each case is the record's worked
//! history, changed as the test says.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAMES: [&str; 2] = ["piotroski_f_delta_lever", "piotroski_f_delta_liquid"];

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

fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
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

fn not_applicable(concept: Concept) -> Resolution {
    Resolution::NotApplicable {
        excluded: concept
            .definition()
            .applies_to
            .excluding(Kind::Bank)
            .expect("the concept does not apply to a bank"),
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

/// A period and what it states of the concepts it names. Every concept it
/// does not name is a figure.
type Stated = (Period, Vec<(Concept, Resolution)>);

/// The record's worked history: the years 2023 and 2024 and the instants
/// 2022-12-31, 2023-12-31 and 2024-12-31.
fn worked() -> Vec<Stated> {
    let values = |given: &[(Concept, &str)]| {
        given
            .iter()
            .map(|(concept, amount)| (*concept, value(amount)))
            .collect()
    };
    vec![
        (
            calendar(2023),
            values(&[
                (Concept::NetIncome, "40"),
                (Concept::GrossProfit, "240"),
                (Concept::Revenue, "800"),
                (Concept::DilutedSharesWeightedAverage, "100"),
            ]),
        ),
        (
            calendar(2024),
            values(&[
                (Concept::NetIncome, "60"),
                (Concept::OperatingCashFlow, "90"),
                (Concept::GrossProfit, "352"),
                (Concept::Revenue, "1100"),
                (Concept::DilutedSharesWeightedAverage, "100"),
            ]),
        ),
        (
            instant("2022-12-31"),
            values(&[(Concept::TotalAssets, "800")]),
        ),
        (
            instant("2023-12-31"),
            values(&[
                (Concept::TotalAssets, "1000"),
                (Concept::LongTermDebt, "270"),
                (Concept::CurrentAssets, "300"),
                (Concept::CurrentLiabilities, "200"),
            ]),
        ),
        (
            instant("2024-12-31"),
            values(&[
                (Concept::TotalAssets, "1200"),
                (Concept::LongTermDebt, "220"),
                (Concept::CurrentAssets, "360"),
                (Concept::CurrentLiabilities, "200"),
            ]),
        ),
    ]
}

/// `history` with `concept` at `period` stated as `resolution`.
fn with(
    history: Vec<Stated>,
    period: &Period,
    concept: Concept,
    resolution: Resolution,
) -> Vec<Stated> {
    history
        .into_iter()
        .map(|(at, mut given)| {
            if at == *period {
                given.retain(|(named, _)| *named != concept);
                given.push((concept, resolution.clone()));
            }
            (at, given)
        })
        .collect()
}

/// `history` with `concept` at `period` stated as the figure `amount`.
fn with_value(
    history: Vec<Stated>,
    period: &Period,
    concept: Concept,
    amount: &str,
) -> Vec<Stated> {
    with(history, period, concept, value(amount))
}

/// `history` without the period `period`.
fn without(history: Vec<Stated>, period: &Period) -> Vec<Stated> {
    history.into_iter().filter(|(at, _)| at != period).collect()
}

fn row((period, given): &Stated) -> Row {
    Row::at(
        period.clone(),
        std::array::from_fn(|at| {
            given
                .iter()
                .find(|(concept, _)| *concept == Concept::ALL[at])
                .map_or_else(|| value("7"), |(_, resolution)| resolution.clone())
        }),
    )
}

/// The row `history` holds at `period`.
fn row_at(history: &[Stated], period: &Period) -> Row {
    row(history
        .iter()
        .find(|(at, _)| at == period)
        .expect("the history holds the period"))
}

fn handed(history: &[Stated]) -> HandOver {
    analyze(
        History::of(CIK.into(), history.iter().map(row).collect()),
        None,
        &Settings::preset(),
    )
    .expect("no crossing is handed")
}

/// Each of the two's outcome at `period`, or none where the results there
/// hold no metric of that name.
fn outcomes_at(hand_over: &HandOver, period: &Period) -> [Option<Outcome>; 2] {
    let metrics = hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics();
    NAMES.map(|name| {
        metrics
            .iter()
            .find(|metric| &*metric.name == name)
            .map(|metric| metric.outcome.clone())
    })
}

/// F_ΔLEVER and F_ΔLIQUID at `period` in `history`.
fn at(history: Vec<Stated>, period: &Period) -> [Outcome; 2] {
    outcomes_at(&handed(&history), period).map(|outcome| outcome.expect("stated at a year"))
}

/// F_ΔLEVER and F_ΔLIQUID at the year 2024 in `history`.
fn at_2024(history: Vec<Stated>) -> [Outcome; 2] {
    at(history, &calendar(2024))
}

fn figure(characters: &str) -> Outcome {
    Outcome::Value {
        figure: characters.into(),
    }
}

fn one() -> Outcome {
    figure("1")
}

fn zero() -> Outcome {
    figure("0")
}

fn absent(first: Reason, rest: Vec<Reason>) -> Outcome {
    Outcome::Absent {
        reasons: Reasons::of(first, rest),
    }
}

fn declined(condition: &str) -> Outcome {
    absent(Reason::declined(condition.into()), Vec::new())
}

/// A `no_period` at the year 2024, sought by `sought_by`.
fn no_period(sought_by: &str) -> Reason {
    Reason::no_period(calendar(2024), sought_by.into())
}

/// `count` reasons, each a `no_period` at the year 2024 sought by
/// `preceding_year`.
fn preceding_years(count: usize) -> Outcome {
    let reason = no_period("preceding_year");
    absent(reason.clone(), vec![reason; count - 1])
}

/// The reason `concept` is `Unknown` at `period` in `history`.
fn unknown_at(history: &[Stated], period: &Period, concept: Concept) -> Reason {
    Reason::input_unknown(&row_at(history, period), concept).expect("the concept is unknown there")
}

/// The reason `concept` does not apply at `period` in `history`.
fn not_applicable_at(history: &[Stated], period: &Period, concept: Concept) -> Reason {
    Reason::input_not_applicable(&row_at(history, period), concept)
        .expect("the concept does not apply there")
}

#[test]
fn the_worked_history_gives_each_of_the_two_one() {
    assert_eq!(at_2024(worked()), [one(), one()]);
}

#[test]
fn an_unchanged_leverage_gives_f_delta_lever_zero() {
    let history = with_value(
        worked(),
        &instant("2024-12-31"),
        Concept::LongTermDebt,
        "330",
    );
    assert_eq!(at_2024(history), [zero(), one()]);
}

#[test]
fn a_debt_free_filer_gives_f_delta_lever_zero() {
    let history = with_value(worked(), &instant("2023-12-31"), Concept::LongTermDebt, "0");
    let history = with_value(history, &instant("2024-12-31"), Concept::LongTermDebt, "0");
    assert_eq!(at_2024(history), [zero(), one()]);
}

#[test]
fn an_unchanged_current_ratio_gives_f_delta_liquid_zero() {
    let history = with_value(
        worked(),
        &instant("2024-12-31"),
        Concept::CurrentAssets,
        "300",
    );
    assert_eq!(at_2024(history), [one(), zero()]);
}

#[test]
fn short_term_debt_never_stands_in_for_long_term_debt() {
    let history = with_value(
        worked(),
        &instant("2024-12-31"),
        Concept::ShortTermDebt,
        "99999",
    );
    let history = with(
        history,
        &instant("2023-12-31"),
        Concept::ShortTermDebt,
        unknown(),
    );
    assert_eq!(at_2024(history), [one(), one()]);
}

#[test]
fn a_zero_balance_at_the_years_opening_stands_in_both_averages() {
    let history = with_value(worked(), &instant("2023-12-31"), Concept::TotalAssets, "0");
    assert_eq!(at_2024(history)[0], one());
}

#[test]
fn a_zero_average_declines_f_delta_lever_the_years_condition_first() {
    let closing = || {
        with_value(
            worked(),
            &instant("2024-12-31"),
            Concept::TotalAssets,
            "-1000",
        )
    };
    let preceding = |history| {
        with_value(
            history,
            &instant("2022-12-31"),
            Concept::TotalAssets,
            "-1000",
        )
    };
    let this = declined("zero_average_total_assets");
    let before = declined("zero_preceding_average_total_assets");
    assert_eq!(at_2024(closing()), [this.clone(), one()]);
    assert_eq!(at_2024(preceding(worked())), [before, one()]);
    assert_eq!(at_2024(preceding(closing())), [this, one()]);
}

#[test]
fn zero_current_liabilities_decline_f_delta_liquid_the_years_condition_first() {
    let zeroed = |history, at| with_value(history, &instant(at), Concept::CurrentLiabilities, "0");
    let this = declined("zero_current_liabilities");
    let before = declined("zero_preceding_current_liabilities");
    assert_eq!(
        at_2024(zeroed(worked(), "2024-12-31")),
        [one(), this.clone()]
    );
    assert_eq!(at_2024(zeroed(worked(), "2023-12-31")), [one(), before]);
    assert_eq!(
        at_2024(zeroed(zeroed(worked(), "2024-12-31"), "2023-12-31")),
        [one(), this]
    );
}

#[test]
fn the_year_before_missing_gives_preceding_year_for_each_read_at_it() {
    let history = without(worked(), &calendar(2023));
    assert_eq!(at_2024(history), [preceding_years(2), preceding_years(2)]);
}

#[test]
fn the_opening_of_the_year_before_missing_is_one_preceding_opening_instant() {
    let history = without(worked(), &instant("2022-12-31"));
    assert_eq!(
        at_2024(history),
        [
            absent(no_period("preceding_opening_instant"), Vec::new()),
            one()
        ]
    );
}

#[test]
fn the_opening_of_the_year_missing_gives_a_reason_for_each_read_there() {
    let history = without(worked(), &instant("2023-12-31"));
    let opening = || no_period("opening_instant");
    assert_eq!(
        at_2024(history),
        [
            absent(opening(), vec![opening()]),
            absent(opening(), vec![opening()])
        ]
    );
}

#[test]
fn total_assets_at_the_years_opening_are_read_once() {
    let opening = instant("2023-12-31");
    let history = with(worked(), &opening, Concept::TotalAssets, unknown());
    let reason = unknown_at(&history, &opening, Concept::TotalAssets);
    assert_eq!(at_2024(history)[0], absent(reason, Vec::new()));
}

#[test]
fn the_years_reasons_come_before_the_year_befores() {
    let closing = instant("2024-12-31");
    let history = with(worked(), &closing, Concept::LongTermDebt, unknown());
    let history = without(history, &instant("2022-12-31"));
    let reason = unknown_at(&history, &closing, Concept::LongTermDebt);
    assert_eq!(
        at_2024(history)[0],
        absent(reason, vec![no_period("preceding_opening_instant")])
    );
}

#[test]
fn across_a_fiscal_year_change_the_first_new_year_has_no_year_before() {
    let history: Vec<Stated> = [
        duration("2022-05-01", "2023-04-30"),
        calendar(2023),
        duration("2023-05-01", "2023-12-31"),
        calendar(2024),
        calendar(2025),
        instant("2023-12-31"),
        instant("2024-12-31"),
        instant("2025-12-31"),
    ]
    .into_iter()
    .map(|period| (period, Vec::new()))
    .collect();
    assert_eq!(
        at(history.clone(), &calendar(2024)),
        [preceding_years(2), preceding_years(2)]
    );
    assert_eq!(at(history, &calendar(2025)), [zero(), zero()]);
}

#[test]
fn a_bank_has_no_f_delta_liquid_and_has_f_delta_lever() {
    let bank = [
        Concept::GrossProfit,
        Concept::CurrentAssets,
        Concept::CurrentLiabilities,
    ]
    .into_iter()
    .fold(worked(), |history, concept| {
        let periods: Vec<Period> = history.iter().map(|(period, _)| period.clone()).collect();
        periods.iter().fold(history, |history, period| {
            with(history, period, concept, not_applicable(concept))
        })
    });
    let [closing, opening] = [instant("2024-12-31"), instant("2023-12-31")];
    assert_eq!(
        at_2024(bank.clone()),
        [
            one(),
            absent(
                not_applicable_at(&bank, &closing, Concept::CurrentAssets),
                vec![
                    not_applicable_at(&bank, &closing, Concept::CurrentLiabilities),
                    not_applicable_at(&bank, &opening, Concept::CurrentAssets),
                    not_applicable_at(&bank, &opening, Concept::CurrentLiabilities),
                ]
            )
        ]
    );
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", "2024-12-31");
    let opening = instant("2024-09-30");
    let closing = instant("2024-12-31");
    let every_input_unknown = [
        Concept::LongTermDebt,
        Concept::TotalAssets,
        Concept::CurrentAssets,
        Concept::CurrentLiabilities,
    ]
    .into_iter()
    .fold(
        [(quarter.clone(), Vec::new()), (opening.clone(), Vec::new())]
            .into_iter()
            .chain(worked())
            .collect(),
        |history, concept| {
            let history = with(history, &quarter, concept, unknown());
            let history = with(history, &opening, concept, unknown());
            with(history, &closing, concept, unknown())
        },
    );
    assert_eq!(
        at(every_input_unknown, &quarter),
        [(); 2].map(|()| declined("not_a_year"))
    );
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let history = vec![
        (duration("2024-1-01", "2024-12-31"), Vec::new()),
        (instant("2023-12-31"), Vec::new()),
        (instant("2024-12-31"), Vec::new()),
    ];
    assert_eq!(
        at(history, &duration("2024-1-01", "2024-12-31")),
        [(); 2].map(|()| declined("start_not_a_date"))
    );
}

#[test]
fn each_is_stated_at_every_year_and_at_no_instant() {
    let hand_over = handed(&worked());
    let at_2023 = || {
        let reason = Reason::no_period(calendar(2023), "preceding_year".into());
        Some(absent(reason.clone(), vec![reason]))
    };
    assert_eq!(
        outcomes_at(&hand_over, &calendar(2023)),
        [at_2023(), at_2023()]
    );
    for at in ["2022-12-31", "2023-12-31", "2024-12-31"] {
        assert_eq!(outcomes_at(&hand_over, &instant(at)), [None, None], "{at}");
    }
}

#[test]
fn adding_the_signals_adds_no_setting_and_keeps_the_version() {
    let premises = handed(&worked()).premises().clone();
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
        ]
    );
}
