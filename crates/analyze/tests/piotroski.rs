//! Piotroski's F_ROA, F_CFO and F_ACCRUAL as a caller reaches them: a history
//! in, and at each year one or zero from that year and the instant it opens
//! at alone, or the reasons each has none. Each case is the record's worked
//! history, changed as the test says.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAMES: [&str; 3] = ["piotroski_f_roa", "piotroski_f_cfo", "piotroski_f_accrual"];

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

fn handed(history: &[Stated]) -> HandOver {
    analyze(
        History::of(CIK.into(), history.iter().map(row).collect()),
        None,
        &Settings::preset(),
    )
    .expect("no crossing is handed")
}

/// Each of the three's outcome at `period`, or none where the results there
/// hold no metric of that name.
fn outcomes_at(hand_over: &HandOver, period: &Period) -> [Option<Outcome>; 3] {
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

/// F_ROA, F_CFO and F_ACCRUAL at `period` in `history`.
fn at(history: Vec<Stated>, period: &Period) -> [Outcome; 3] {
    outcomes_at(&handed(&history), period).map(|outcome| outcome.expect("stated at a year"))
}

/// F_ROA, F_CFO and F_ACCRUAL at the year 2024 in `history`.
fn at_2024(history: Vec<Stated>) -> [Outcome; 3] {
    at(history, &calendar(2024))
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

fn no_opening_instant() -> Reason {
    Reason::no_period(calendar(2024), "opening_instant".into())
}

#[test]
fn the_worked_history_gives_each_of_the_three_one() {
    assert_eq!(at_2024(worked()), [figure("1"), figure("1"), figure("1")]);
}

#[test]
fn a_loss_gives_f_roa_zero_and_the_accrual_still_one() {
    let history = with(worked(), &calendar(2024), Concept::NetIncome, value("-30"));
    let history = with(history, &calendar(2023), Concept::NetIncome, value("-50"));
    assert_eq!(at_2024(history), [figure("0"), figure("1"), figure("1")]);
}

#[test]
fn a_negative_cash_flow_above_the_loss_gives_f_cfo_zero_and_the_accrual_one() {
    let history = with(worked(), &calendar(2024), Concept::NetIncome, value("-30"));
    let history = with(history, &calendar(2023), Concept::NetIncome, value("-50"));
    let history = with(
        history,
        &calendar(2024),
        Concept::OperatingCashFlow,
        value("-10"),
    );
    assert_eq!(at_2024(history), [figure("0"), figure("0"), figure("1")]);
}

#[test]
fn cash_flow_equal_to_net_income_gives_the_accrual_zero_and_f_cfo_one() {
    let history = with(
        worked(),
        &calendar(2024),
        Concept::OperatingCashFlow,
        value("60"),
    );
    assert_eq!(at_2024(history), [figure("1"), figure("1"), figure("0")]);
}

#[test]
fn net_income_of_zero_is_not_above_zero() {
    let history = with(worked(), &calendar(2024), Concept::NetIncome, value("0"));
    let history = with(history, &calendar(2023), Concept::NetIncome, value("-40"));
    assert_eq!(at_2024(history)[0], figure("0"));
}

#[test]
fn a_zero_opening_balance_declines_each_under_its_own_condition() {
    let history = with(
        worked(),
        &instant("2023-12-31"),
        Concept::TotalAssets,
        value("0"),
    );
    assert_eq!(
        at_2024(history),
        [(); 3].map(|()| declined("zero_opening_total_assets"))
    );
}

#[test]
fn each_reads_the_year_alone_and_nothing_of_the_year_before() {
    let history = without(worked(), &calendar(2023));
    assert_eq!(at_2024(history), [figure("1"), figure("1"), figure("1")]);
}

#[test]
fn a_missing_opening_instant_is_one_no_period_sought_by_the_opening_instant() {
    let history = without(worked(), &instant("2023-12-31"));
    assert_eq!(
        at_2024(history),
        [(); 3].map(|()| absent(no_opening_instant(), Vec::new()))
    );
}

#[test]
fn the_accrual_gives_every_reason_in_read_order() {
    let history = with(worked(), &calendar(2024), Concept::NetIncome, unknown());
    let history = without(history, &instant("2023-12-31"));
    let unknown_net_income = Reason::input_unknown(
        &row(history
            .iter()
            .find(|(period, _)| *period == calendar(2024))
            .expect("the history holds 2024")),
        Concept::NetIncome,
    )
    .expect("net income is unknown at 2024");
    assert_eq!(
        at_2024(history)[2],
        absent(unknown_net_income, vec![no_opening_instant()])
    );
}

#[test]
fn a_bank_gives_each_of_the_three_one() {
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
    assert_eq!(at_2024(bank), [figure("1"), figure("1"), figure("1")]);
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", "2024-12-31");
    let every_input_unknown = [
        Concept::NetIncome,
        Concept::OperatingCashFlow,
        Concept::TotalAssets,
    ]
    .into_iter()
    .fold(
        [
            (quarter.clone(), Vec::new()),
            (instant("2024-09-30"), Vec::new()),
        ]
        .into_iter()
        .chain(worked())
        .collect(),
        |history, concept| {
            let history = with(history, &quarter, concept, unknown());
            with(history, &instant("2024-09-30"), concept, unknown())
        },
    );
    assert_eq!(
        at(every_input_unknown, &quarter),
        [(); 3].map(|()| declined("not_a_year"))
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
        [(); 3].map(|()| declined("start_not_a_date"))
    );
}

#[test]
fn each_is_stated_at_every_year_and_at_no_instant() {
    let hand_over = handed(&worked());
    assert_eq!(
        outcomes_at(&hand_over, &calendar(2023)),
        [Some(figure("1")), Some(figure("1")), Some(figure("0"))]
    );
    for at in ["2022-12-31", "2023-12-31", "2024-12-31"] {
        assert_eq!(
            outcomes_at(&hand_over, &instant(at)),
            [None, None, None],
            "{at}"
        );
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
