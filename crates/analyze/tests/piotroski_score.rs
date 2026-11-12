//! Piotroski's F_SCORE as a caller reaches it: a history in, and at each year
//! the count of the nine signals that are one, or the reasons it has none.
//! Each case is the record's worked history, changed as the test says.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "piotroski_f_score";

/// The nine signals the score counts, in §2.3.4's order.
const SIGNALS: [&str; 9] = [
    "piotroski_f_roa",
    "piotroski_f_delta_roa",
    "piotroski_f_cfo",
    "piotroski_f_accrual",
    "piotroski_f_delta_margin",
    "piotroski_f_delta_turn",
    "piotroski_f_delta_lever",
    "piotroski_f_delta_liquid",
    "piotroski_eq_offer",
];

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

/// The outcome of the metric `name` at `period`, or none where the results
/// there hold no metric of that name.
fn outcome_of(hand_over: &HandOver, period: &Period, name: &str) -> Option<Outcome> {
    hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics()
        .iter()
        .find(|metric| &*metric.name == name)
        .map(|metric| metric.outcome.clone())
}

/// F_SCORE at `period` in `history`.
fn at(history: Vec<Stated>, period: &Period) -> Outcome {
    outcome_of(&handed(&history), period, NAME).expect("stated at a year")
}

/// F_SCORE at the year 2024 in `history`.
fn at_2024(history: Vec<Stated>) -> Outcome {
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

/// A `no_period` at the year 2024, sought by `sought_by`.
fn no_period(sought_by: &str) -> Reason {
    Reason::no_period(calendar(2024), sought_by.into())
}

/// `count` reasons, each a `no_period` at the year 2024 sought by
/// `sought_by`.
fn no_periods(sought_by: &str, count: usize) -> Outcome {
    let reason = no_period(sought_by);
    absent(reason.clone(), vec![reason; count - 1])
}

/// The reason `concept` does not apply at `period` in `history`.
fn not_applicable_at(history: &[Stated], period: &Period, concept: Concept) -> Reason {
    Reason::input_not_applicable(&row_at(history, period), concept)
        .expect("the concept does not apply there")
}

/// The worked history with `concept` over the year `year` stated as `amount`.
fn flow(history: Vec<Stated>, year: u32, concept: Concept, amount: &str) -> Vec<Stated> {
    with_value(history, &calendar(year), concept, amount)
}

/// The worked history with `concept` at the instant `at` stated as `amount`.
fn balance(history: Vec<Stated>, at: &str, concept: Concept, amount: &str) -> Vec<Stated> {
    with_value(history, &instant(at), concept, amount)
}

/// The F_ROA row of the record's flip table: a loss in both years.
fn losses() -> Vec<Stated> {
    let history = flow(worked(), 2024, Concept::NetIncome, "-30");
    flow(history, 2023, Concept::NetIncome, "-50")
}

#[test]
fn the_worked_history_gives_nine() {
    assert_eq!(at_2024(worked()), figure("9"));
}

#[test]
fn each_signal_flipped_alone_gives_eight() {
    let flips = [
        losses(),
        flow(worked(), 2023, Concept::NetIncome, "48"),
        flow(worked(), 2024, Concept::OperatingCashFlow, "60"),
        flow(worked(), 2024, Concept::GrossProfit, "330"),
        flow(
            flow(worked(), 2024, Concept::Revenue, "1000"),
            2024,
            Concept::GrossProfit,
            "320",
        ),
        balance(worked(), "2024-12-31", Concept::LongTermDebt, "330"),
        balance(worked(), "2024-12-31", Concept::CurrentAssets, "300"),
        flow(worked(), 2024, Concept::DilutedSharesWeightedAverage, "101"),
    ];
    for (at, history) in flips.into_iter().enumerate() {
        assert_eq!(at_2024(history), figure("8"), "flip {at}");
    }
}

#[test]
fn f_cfo_flipped_beside_f_roa_gives_seven() {
    let history = flow(losses(), 2024, Concept::OperatingCashFlow, "-10");
    assert_eq!(at_2024(history), figure("7"));
}

#[test]
fn a_figure_at_zero_is_not_above_zero() {
    let history = flow(worked(), 2024, Concept::NetIncome, "0");
    let history = flow(history, 2023, Concept::NetIncome, "-40");
    assert_eq!(at_2024(history), figure("8"));
    let history = balance(worked(), "2023-12-31", Concept::LongTermDebt, "0");
    let history = balance(history, "2024-12-31", Concept::LongTermDebt, "0");
    assert_eq!(at_2024(history), figure("8"));
}

#[test]
fn the_score_is_the_count_of_the_signals_beside_it() {
    let histories = [
        worked(),
        losses(),
        flow(losses(), 2024, Concept::OperatingCashFlow, "-10"),
        flow(worked(), 2023, Concept::NetIncome, "48"),
        balance(worked(), "2024-12-31", Concept::LongTermDebt, "330"),
        flow(worked(), 2024, Concept::DilutedSharesWeightedAverage, "101"),
    ];
    for (at, history) in histories.into_iter().enumerate() {
        let hand_over = handed(&history);
        let year = calendar(2024);
        let ones = SIGNALS
            .iter()
            .filter(|name| outcome_of(&hand_over, &year, name) == Some(figure("1")))
            .count();
        assert!(
            SIGNALS.iter().all(|name| matches!(
                outcome_of(&hand_over, &year, name),
                Some(Outcome::Value { .. })
            )),
            "history {at}"
        );
        assert_eq!(
            outcome_of(&hand_over, &year, NAME),
            Some(figure(&ones.to_string())),
            "history {at}"
        );
    }
}

#[test]
fn each_condition_gives_the_score_its_one_reason() {
    let cases = [
        (
            balance(worked(), "2023-12-31", Concept::TotalAssets, "0"),
            "zero_opening_total_assets",
        ),
        (
            balance(worked(), "2022-12-31", Concept::TotalAssets, "0"),
            "zero_preceding_opening_total_assets",
        ),
        (
            flow(
                flow(worked(), 2024, Concept::Revenue, "0"),
                2024,
                Concept::GrossProfit,
                "0",
            ),
            "zero_revenue",
        ),
        (
            flow(
                flow(worked(), 2023, Concept::Revenue, "0"),
                2023,
                Concept::GrossProfit,
                "0",
            ),
            "zero_preceding_revenue",
        ),
        (
            balance(worked(), "2024-12-31", Concept::TotalAssets, "-1000"),
            "zero_average_total_assets",
        ),
        (
            balance(worked(), "2022-12-31", Concept::TotalAssets, "-1000"),
            "zero_preceding_average_total_assets",
        ),
        (
            balance(worked(), "2024-12-31", Concept::CurrentLiabilities, "0"),
            "zero_current_liabilities",
        ),
        (
            balance(worked(), "2023-12-31", Concept::CurrentLiabilities, "0"),
            "zero_preceding_current_liabilities",
        ),
    ];
    for (history, condition) in cases {
        assert_eq!(at_2024(history), declined(condition), "{condition}");
    }
}

#[test]
fn the_conditions_are_asked_in_formula_order() {
    let history = flow(worked(), 2024, Concept::Revenue, "0");
    let history = balance(history, "2024-12-31", Concept::CurrentLiabilities, "0");
    assert_eq!(at_2024(history), declined("zero_revenue"));
    let history = balance(worked(), "2023-12-31", Concept::TotalAssets, "0");
    let history = flow(history, 2024, Concept::Revenue, "0");
    assert_eq!(at_2024(history), declined("zero_opening_total_assets"));
}

#[test]
fn the_year_before_missing_gives_preceding_year_for_each_of_the_eight_reads_at_it() {
    let history = without(worked(), &calendar(2023));
    assert_eq!(at_2024(history), no_periods("preceding_year", 8));
}

#[test]
fn the_opening_of_the_year_before_missing_is_one_preceding_opening_instant() {
    let history = without(worked(), &instant("2022-12-31"));
    assert_eq!(at_2024(history), no_periods("preceding_opening_instant", 1));
}

#[test]
fn the_opening_of_the_year_missing_gives_one_reason_for_each_of_its_four_reads() {
    let history = without(worked(), &instant("2023-12-31"));
    assert_eq!(at_2024(history), no_periods("opening_instant", 4));
}

#[test]
fn total_assets_at_the_years_opening_are_read_once() {
    let opening = instant("2023-12-31");
    let history = with(worked(), &opening, Concept::TotalAssets, unknown());
    let reason = Reason::input_unknown(&row_at(&history, &opening), Concept::TotalAssets)
        .expect("total assets are unknown there");
    assert_eq!(at_2024(history), absent(reason, Vec::new()));
}

#[test]
fn cash_flow_from_operations_is_not_read_at_the_year_before() {
    let history = with(
        worked(),
        &calendar(2023),
        Concept::OperatingCashFlow,
        unknown(),
    );
    assert_eq!(at_2024(history), figure("9"));
}

#[test]
fn the_years_reasons_come_before_the_year_befores() {
    let history = with(worked(), &calendar(2024), Concept::NetIncome, unknown());
    let history = without(history, &calendar(2023));
    let unknown_net_income =
        Reason::input_unknown(&row_at(&history, &calendar(2024)), Concept::NetIncome)
            .expect("net income is unknown at 2024");
    let preceding_year = no_period("preceding_year");
    assert_eq!(
        at_2024(history),
        absent(unknown_net_income, vec![preceding_year; 8])
    );
}

#[test]
fn across_a_fiscal_year_change_the_first_new_year_has_no_score() {
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
        no_periods("preceding_year", 8)
    );
    assert_eq!(at(history, &calendar(2025)), figure("3"));
}

#[test]
fn a_bank_has_no_score_and_has_the_seven_signals_that_read_none_of_the_three() {
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
    let [year, year_before] = [calendar(2024), calendar(2023)];
    let [closing, opening] = [instant("2024-12-31"), instant("2023-12-31")];
    let hand_over = handed(&bank);
    assert_eq!(
        outcome_of(&hand_over, &year, NAME),
        Some(absent(
            not_applicable_at(&bank, &year, Concept::GrossProfit),
            vec![
                not_applicable_at(&bank, &closing, Concept::CurrentAssets),
                not_applicable_at(&bank, &closing, Concept::CurrentLiabilities),
                not_applicable_at(&bank, &year_before, Concept::GrossProfit),
                not_applicable_at(&bank, &opening, Concept::CurrentAssets),
                not_applicable_at(&bank, &opening, Concept::CurrentLiabilities),
            ]
        ))
    );
    for name in SIGNALS
        .iter()
        .filter(|name| !["piotroski_f_delta_margin", "piotroski_f_delta_liquid"].contains(name))
    {
        assert_eq!(
            outcome_of(&hand_over, &year, name),
            Some(figure("1")),
            "{name}"
        );
    }
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", "2024-12-31");
    let opening = instant("2024-09-30");
    let closing = instant("2024-12-31");
    let every_input_unknown = [
        Concept::NetIncome,
        Concept::TotalAssets,
        Concept::OperatingCashFlow,
        Concept::GrossProfit,
        Concept::Revenue,
        Concept::LongTermDebt,
        Concept::CurrentAssets,
        Concept::CurrentLiabilities,
        Concept::DilutedSharesWeightedAverage,
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
    assert_eq!(at(every_input_unknown, &quarter), declined("not_a_year"));
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
        declined("start_not_a_date")
    );
}

#[test]
fn it_is_stated_at_every_year_and_at_no_instant() {
    let hand_over = handed(&worked());
    let reason = Reason::no_period(calendar(2023), "preceding_year".into());
    assert_eq!(
        outcome_of(&hand_over, &calendar(2023), NAME),
        Some(absent(reason.clone(), vec![reason; 7]))
    );
    for at in ["2022-12-31", "2023-12-31", "2024-12-31"] {
        assert_eq!(outcome_of(&hand_over, &instant(at), NAME), None, "{at}");
    }
}

#[test]
fn adding_the_score_adds_no_setting_and_keeps_the_version() {
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
            ("dividend_discount_required_return", "0.0802"),
            ("dividend_discount_growth_rate", "0.02"),
            ("dividend_discount_base_years", "5"),
        ]
    );
}
