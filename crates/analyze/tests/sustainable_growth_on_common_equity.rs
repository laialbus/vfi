//! Sustainable growth on common equity as a caller reaches it: a history in,
//! and at each duration period return on common equity times retention per
//! share at that duration, or the reasons either has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "sustainable_growth_on_common_equity";

const RETURN: &str = "return_on_common_equity";

const RETENTION: &str = "retention_per_share";

const OPENING: &str = "2023-12-31";

const CLOSING: &str = "2024-12-31";

fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
}

fn duration(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

fn year() -> Period {
    duration("2024-01-01", CLOSING)
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

fn unknown() -> Resolution {
    Resolution::Unknown {
        attempted: Attempts::in_filings(
            Attempted::in_filing("0000320193-25-000079".into(), Attempt::that_ran(Vec::new())),
            Vec::new(),
        ),
    }
}

/// A row at `period` with each of `given` as stated, and every other concept
/// a value no metric under test reads as one of these.
fn row(period: Period, given: &[(Concept, Resolution)]) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| {
            given
                .iter()
                .find(|(concept, _)| *concept == Concept::ALL[at])
                .map_or_else(|| value("7"), |(_, resolution)| resolution.clone())
        }),
    )
}

/// The four flows over `period`: net income, preferred dividends, dividends
/// declared per share and diluted earnings per share.
fn flows(period: Period, earnings_per_share_diluted: Resolution) -> Row {
    row(
        period,
        &[
            (Concept::NetIncome, value("120")),
            (Concept::PreferredDividends, value("20")),
            (Concept::DividendsDeclaredPerShare, value("0.5")),
            (Concept::EarningsPerShareDiluted, earnings_per_share_diluted),
        ],
    )
}

/// Shareholders' equity and preferred equity at the instant `at`.
fn balances(at: &str, shareholders_equity: Resolution, preferred_equity: Resolution) -> Row {
    row(
        instant(at),
        &[
            (Concept::ShareholdersEquity, shareholders_equity),
            (Concept::PreferredEquity, preferred_equity),
        ],
    )
}

/// The year's flows with `earnings_per_share_diluted`, and shareholders'
/// equity `1000` and preferred equity `200` at both instants.
fn year_with(earnings_per_share_diluted: &str) -> Vec<Row> {
    vec![
        flows(year(), value(earnings_per_share_diluted)),
        balances(OPENING, value("1000"), value("200")),
        balances(CLOSING, value("1000"), value("200")),
    ]
}

fn handed(rows: Vec<Row>) -> HandOver {
    analyze(History::of(CIK.into(), rows), None, &Settings::preset())
        .expect("no crossing is handed")
}

/// `name`'s outcome at `period`, or none where the results there hold no
/// metric of that name.
fn outcome_at(hand_over: &HandOver, period: &Period, name: &str) -> Option<Outcome> {
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

/// `name`'s outcome over the year, from a history of `rows`.
fn outcome_of(rows: Vec<Row>, name: &str) -> Outcome {
    outcome_at(&handed(rows), &year(), name).expect("the metric is stated at a duration")
}

fn outcome(rows: Vec<Row>) -> Outcome {
    outcome_of(rows, NAME)
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

fn reasons(outcome: Outcome) -> Vec<Reason> {
    match outcome {
        Outcome::Absent { reasons } => reasons.each().cloned().collect(),
        Outcome::Value { figure } => panic!("a figure, {figure}, where reasons were expected"),
    }
}

fn no_period(sought_by: &str) -> Reason {
    Reason::no_period(year(), sought_by.into())
}

fn declined(condition: &str) -> Reason {
    Reason::declined(condition.into())
}

#[test]
fn return_on_common_equity_times_retention_per_share_is_its_figure() {
    let rows = year_with("2");
    assert_eq!(outcome(rows.clone()), figure("0.09375"));
    assert_eq!(outcome_of(rows.clone(), RETURN), figure("0.125"));
    assert_eq!(outcome_of(rows, RETENTION), figure("0.75"));
}

#[test]
fn a_loss_per_share_gives_a_figure_and_no_decline() {
    assert_eq!(outcome(year_with("-2")), figure("0.15625"));
}

#[test]
fn an_unknown_preferred_equity_and_diluted_earnings_give_both_metrics_reasons_in_order() {
    let rows = vec![
        flows(year(), unknown()),
        balances(OPENING, value("1000"), unknown()),
        balances(CLOSING, value("1000"), unknown()),
    ];
    let given = reasons(outcome(rows.clone()));
    let return_on_common_equity = reasons(outcome_of(rows.clone(), RETURN));
    let retention_per_share = reasons(outcome_of(rows.clone(), RETENTION));
    assert_eq!(
        given,
        [return_on_common_equity.clone(), retention_per_share.clone()].concat()
    );
    let unknown = |row: &Row, concept| {
        Reason::input_unknown(row, concept).expect("the concept is Unknown at the row")
    };
    assert_eq!(
        return_on_common_equity,
        [
            unknown(&rows[1], Concept::PreferredEquity),
            unknown(&rows[2], Concept::PreferredEquity)
        ]
    );
    assert_eq!(
        retention_per_share,
        [unknown(&rows[0], Concept::EarningsPerShareDiluted)]
    );
}

#[test]
fn a_history_lacking_the_closing_instant_gives_no_period_sought_by_the_closing_first() {
    let rows = vec![
        flows(year(), value("2")),
        balances(OPENING, value("1000"), value("200")),
    ];
    assert_eq!(
        outcome(rows),
        absent(
            no_period("closing_instant"),
            vec![no_period("closing_instant")]
        )
    );
}

#[test]
fn shareholders_equity_equal_to_preferred_equity_declines_as_zero_common_equity() {
    let rows = vec![
        flows(year(), value("2")),
        balances(OPENING, value("200"), value("200")),
        balances(CLOSING, value("200"), value("200")),
    ];
    assert_eq!(
        outcome(rows),
        absent(declined("zero_common_equity"), Vec::new())
    );
}

#[test]
fn a_zero_diluted_earnings_per_share_declines_as_it_declines_retention() {
    assert_eq!(
        outcome(year_with("0")),
        absent(declined("zero_earnings_per_share_diluted"), Vec::new())
    );
}

#[test]
fn both_conditions_met_at_one_row_give_both_declines_in_order() {
    let rows = vec![
        flows(year(), value("0")),
        balances(OPENING, value("200"), value("200")),
        balances(CLOSING, value("200"), value("200")),
    ];
    assert_eq!(
        outcome(rows),
        absent(
            declined("zero_common_equity"),
            vec![declined("zero_earnings_per_share_diluted")]
        )
    );
}

#[test]
fn a_start_outside_the_grammar_declines_as_start_not_a_date() {
    let odd = duration("2024-1-01", CLOSING);
    let hand_over = handed(vec![
        flows(odd.clone(), value("2")),
        balances(OPENING, value("1000"), value("200")),
        balances(CLOSING, value("1000"), value("200")),
    ]);
    assert_eq!(
        outcome_at(&hand_over, &odd, NAME),
        Some(absent(
            declined("start_not_a_date"),
            vec![declined("start_not_a_date")]
        ))
    );
}

#[test]
fn an_instant_period_carries_no_sustainable_growth_on_common_equity() {
    let hand_over = handed(year_with("2"));
    for at in [OPENING, CLOSING] {
        assert_eq!(outcome_at(&hand_over, &instant(at), NAME), None, "{at}");
    }
    assert!(outcome_at(&hand_over, &year(), NAME).is_some());
}

#[test]
fn every_earlier_metric_is_computed_as_before_beside_it() {
    let given = [
        (Concept::GrossProfit, value("2")),
        (Concept::OperatingIncome, value("3")),
        (Concept::NetIncome, value("-1")),
        (Concept::Revenue, value("8")),
        (Concept::InterestExpense, value("4")),
        (Concept::ShortTermDebt, value("2")),
        (Concept::LongTermDebt, value("3")),
        (Concept::ShareholdersEquity, value("4")),
        (Concept::PreferredEquity, value("2")),
        (Concept::TotalLiabilities, value("3")),
        (Concept::TotalAssets, value("4")),
        (Concept::CurrentAssets, value("9")),
        (Concept::Inventory, value("1")),
        (Concept::CashAndEquivalents, value("2")),
        (Concept::CurrentLiabilities, value("4")),
        (Concept::DividendsPaid, value("1")),
        (Concept::PreferredDividends, value("1")),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, value("3")),
        (Concept::DividendsDeclaredPerShare, value("1")),
        (Concept::EarningsPerShareDiluted, value("4")),
    ];
    let hand_over = handed(vec![
        row(instant(OPENING), &given),
        row(instant(CLOSING), &given),
        row(year(), &given),
    ]);
    for (period, name, characters) in [
        (year(), "gross_margin", "0.25"),
        (year(), "operating_margin", "0.375"),
        (year(), "net_margin", "-0.125"),
        (year(), "interest_coverage", "0.75"),
        (instant(CLOSING), "debt_to_equity", "1.25"),
        (instant(CLOSING), "liabilities_to_assets", "0.75"),
        (instant(CLOSING), "current_ratio", "2.25"),
        (instant(CLOSING), "quick_ratio", "2"),
        (instant(CLOSING), "cash_ratio", "0.5"),
        (year(), "dividend_payout_on_earnings", "-1"),
        (year(), "dividend_payout_on_free_cash_flow", "0.25"),
        (year(), "dividend_coverage", "-1"),
        (year(), "dividend_payout_per_share", "0.25"),
        (year(), "dividend_coverage_per_share", "4"),
        (year(), "return_on_assets", "-0.25"),
        (year(), "asset_turnover", "2"),
        (year(), "return_on_equity", "-0.25"),
        (year(), RETURN, "-1"),
        (year(), RETENTION, "0.75"),
        (year(), NAME, "-0.75"),
    ] {
        assert_eq!(
            outcome_at(&hand_over, &period, name),
            Some(figure(characters)),
            "{name}"
        );
    }
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![row(year(), &[])]).premises().clone();
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
