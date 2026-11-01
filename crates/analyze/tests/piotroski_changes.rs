//! Piotroski's F_ΔROA, F_ΔMARGIN, F_ΔTURN and EQ_OFFER as a caller reaches
//! them: a history in, and at each year one or zero from that year and the
//! year before it, or the reasons each has none. Each case is the record's
//! worked history, changed as the test says.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAMES: [&str; 4] = [
    "piotroski_f_delta_roa",
    "piotroski_f_delta_margin",
    "piotroski_f_delta_turn",
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

/// Each of the four's outcome at `period`, or none where the results there
/// hold no metric of that name.
fn outcomes_at(hand_over: &HandOver, period: &Period) -> [Option<Outcome>; 4] {
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

/// F_ΔROA, F_ΔMARGIN, F_ΔTURN and EQ_OFFER at `period` in `history`.
fn at(history: Vec<Stated>, period: &Period) -> [Outcome; 4] {
    outcomes_at(&handed(&history), period).map(|outcome| outcome.expect("stated at a year"))
}

/// F_ΔROA, F_ΔMARGIN, F_ΔTURN and EQ_OFFER at the year 2024 in `history`.
fn at_2024(history: Vec<Stated>) -> [Outcome; 4] {
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

#[test]
fn the_worked_history_gives_each_of_the_four_one() {
    assert_eq!(at_2024(worked()), [one(), one(), one(), one()]);
}

#[test]
fn an_unchanged_roa_gives_f_delta_roa_zero() {
    let history = with_value(worked(), &calendar(2023), Concept::NetIncome, "48");
    assert_eq!(at_2024(history), [zero(), one(), one(), one()]);
}

#[test]
fn a_smaller_loss_is_a_rise_in_roa() {
    let history = with_value(worked(), &calendar(2024), Concept::NetIncome, "-30");
    let history = with_value(history, &calendar(2023), Concept::NetIncome, "-50");
    assert_eq!(at_2024(history)[0], one());
}

#[test]
fn net_income_of_zero_after_a_loss_is_a_rise_in_roa() {
    let history = with_value(worked(), &calendar(2024), Concept::NetIncome, "0");
    let history = with_value(history, &calendar(2023), Concept::NetIncome, "-40");
    assert_eq!(at_2024(history)[0], one());
}

#[test]
fn an_unchanged_margin_gives_f_delta_margin_zero() {
    let history = with_value(worked(), &calendar(2024), Concept::GrossProfit, "330");
    assert_eq!(at_2024(history), [one(), zero(), one(), one()]);
}

#[test]
fn an_unchanged_turnover_gives_f_delta_turn_zero_and_the_margin_still_rises() {
    let history = with_value(worked(), &calendar(2024), Concept::Revenue, "1000");
    let history = with_value(history, &calendar(2024), Concept::GrossProfit, "320");
    assert_eq!(at_2024(history), [one(), one(), zero(), one()]);
}

#[test]
fn a_count_above_the_year_befores_gives_eq_offer_zero_and_one_below_it_one() {
    let history = |count| {
        with_value(
            worked(),
            &calendar(2024),
            Concept::DilutedSharesWeightedAverage,
            count,
        )
    };
    assert_eq!(at_2024(history("101")), [one(), one(), one(), zero()]);
    assert_eq!(at_2024(history("95")), [one(), one(), one(), one()]);
}

#[test]
fn a_zero_opening_balance_declines_the_two_scaled_by_it_the_years_condition_first() {
    let opening = || with_value(worked(), &instant("2023-12-31"), Concept::TotalAssets, "0");
    let preceding_opening =
        |history| with_value(history, &instant("2022-12-31"), Concept::TotalAssets, "0");
    let this = declined("zero_opening_total_assets");
    let before = declined("zero_preceding_opening_total_assets");
    assert_eq!(
        at_2024(opening()),
        [this.clone(), one(), this.clone(), one()]
    );
    assert_eq!(
        at_2024(preceding_opening(worked())),
        [before.clone(), one(), before, one()]
    );
    assert_eq!(
        at_2024(preceding_opening(opening())),
        [this.clone(), one(), this, one()]
    );
}

#[test]
fn zero_revenue_declines_the_margin_and_not_the_turnover() {
    let zeroed = |year| {
        let history = with_value(worked(), &calendar(year), Concept::Revenue, "0");
        with_value(history, &calendar(year), Concept::GrossProfit, "0")
    };
    assert_eq!(
        at_2024(zeroed(2024)),
        [one(), declined("zero_revenue"), zero(), one()]
    );
    assert_eq!(
        at_2024(zeroed(2023)),
        [one(), declined("zero_preceding_revenue"), one(), one()]
    );
}

#[test]
fn a_negative_opening_balance_of_the_year_before_gives_its_comparison() {
    let history = with_value(
        worked(),
        &instant("2022-12-31"),
        Concept::TotalAssets,
        "-1000",
    );
    assert_eq!(at_2024(history), [one(), one(), one(), one()]);
}

#[test]
fn the_year_before_missing_gives_preceding_year_for_each_read_at_it() {
    let history = without(worked(), &calendar(2023));
    assert_eq!(
        at_2024(history),
        [
            preceding_years(2),
            preceding_years(2),
            preceding_years(2),
            preceding_years(1)
        ]
    );
}

#[test]
fn the_opening_of_the_year_before_missing_is_one_preceding_opening_instant() {
    let history = without(worked(), &instant("2022-12-31"));
    let missing = || absent(no_period("preceding_opening_instant"), Vec::new());
    assert_eq!(at_2024(history), [missing(), one(), missing(), one()]);
}

#[test]
fn the_opening_of_the_year_missing_is_one_opening_instant() {
    let history = without(worked(), &instant("2023-12-31"));
    let missing = || absent(no_period("opening_instant"), Vec::new());
    assert_eq!(at_2024(history), [missing(), one(), missing(), one()]);
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
        at_2024(history)[0],
        absent(
            unknown_net_income,
            vec![preceding_year.clone(), preceding_year]
        )
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
        [
            preceding_years(2),
            preceding_years(2),
            preceding_years(2),
            preceding_years(1)
        ]
    );
    assert_eq!(
        at(history, &calendar(2025)),
        [zero(), zero(), zero(), one()]
    );
}

#[test]
fn a_bank_has_no_margin_and_the_other_three() {
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
    let not_applicable_at = |year| {
        Reason::input_not_applicable(&row_at(&bank, &calendar(year)), Concept::GrossProfit)
            .expect("gross profit does not apply to a bank")
    };
    assert_eq!(
        at_2024(bank.clone()),
        [
            one(),
            absent(not_applicable_at(2024), vec![not_applicable_at(2023)]),
            one(),
            one()
        ]
    );
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", "2024-12-31");
    let every_input_unknown = [
        Concept::NetIncome,
        Concept::TotalAssets,
        Concept::GrossProfit,
        Concept::Revenue,
        Concept::DilutedSharesWeightedAverage,
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
        [(); 4].map(|()| declined("not_a_year"))
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
        [(); 4].map(|()| declined("start_not_a_date"))
    );
}

#[test]
fn each_is_stated_at_every_year_and_at_no_instant() {
    let hand_over = handed(&worked());
    let at_2023 = |sought_by: &str, count: usize| {
        let reason = Reason::no_period(calendar(2023), sought_by.into());
        Some(absent(reason.clone(), vec![reason; count - 1]))
    };
    assert_eq!(
        outcomes_at(&hand_over, &calendar(2023)),
        [
            at_2023("preceding_year", 2),
            at_2023("preceding_year", 2),
            at_2023("preceding_year", 2),
            at_2023("preceding_year", 1)
        ]
    );
    for at in ["2022-12-31", "2023-12-31", "2024-12-31"] {
        assert_eq!(
            outcomes_at(&hand_over, &instant(at)),
            [None, None, None, None],
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
