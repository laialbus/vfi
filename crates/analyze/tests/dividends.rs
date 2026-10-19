//! Dividend payout on earnings, payout on free cash flow and dividend coverage
//! in their total form as a caller reaches them: a history in, and at each
//! duration period each figure or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
};

const CIK: &str = "0000320193";

const DIVIDENDS: [&str; 3] = [
    "dividend_payout_on_earnings",
    "dividend_payout_on_free_cash_flow",
    "dividend_coverage",
];

fn year_end() -> Period {
    Period::Instant {
        at: "2025-09-27".into(),
    }
}

fn fiscal_year() -> Period {
    Period::Duration {
        start: "2024-09-29".into(),
        end: "2025-09-27".into(),
    }
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

/// The figure `dividends_paid`'s conditional silence reading sets where the
/// period is silent on both its members.
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

/// `name`'s outcome over one fiscal year with `given` stated.
fn outcome(name: &str, given: &[(Concept, Resolution)]) -> Outcome {
    let hand_over = handed(vec![row(fiscal_year(), given)]);
    outcome_at(&hand_over, &fiscal_year(), name).expect("the metric is stated at a duration")
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

fn declined(condition: &str) -> Reason {
    Reason::declined(condition.into())
}

#[test]
fn dividends_paid_over_net_income_is_the_payout_on_earnings() {
    assert_eq!(
        outcome(
            "dividend_payout_on_earnings",
            &[
                (Concept::DividendsPaid, value("1")),
                (Concept::NetIncome, value("4")),
            ]
        ),
        figure("0.25")
    );
}

#[test]
fn a_payout_on_earnings_that_does_not_terminate_is_written_to_the_precision() {
    let third = format!("0.{}", "3".repeat(34));
    assert_eq!(
        outcome(
            "dividend_payout_on_earnings",
            &[
                (Concept::DividendsPaid, value("1")),
                (Concept::NetIncome, value("3")),
            ]
        ),
        figure(&third)
    );
}

#[test]
fn a_net_loss_gives_a_negative_payout_on_earnings() {
    assert_eq!(
        outcome(
            "dividend_payout_on_earnings",
            &[
                (Concept::DividendsPaid, value("2")),
                (Concept::NetIncome, value("-4")),
            ]
        ),
        figure("-0.5")
    );
}

#[test]
fn net_income_over_dividends_paid_is_the_coverage() {
    assert_eq!(
        outcome(
            "dividend_coverage",
            &[
                (Concept::NetIncome, value("9")),
                (Concept::DividendsPaid, value("4")),
            ]
        ),
        figure("2.25")
    );
}

#[test]
fn dividends_paid_over_operating_cash_flow_less_capital_expenditure_is_the_payout_on_free_cash_flow()
 {
    assert_eq!(
        outcome(
            "dividend_payout_on_free_cash_flow",
            &[
                (Concept::DividendsPaid, value("1")),
                (Concept::OperatingCashFlow, value("7")),
                (Concept::CapitalExpenditure, value("3")),
            ]
        ),
        figure("0.25")
    );
}

#[test]
fn a_negative_free_cash_flow_gives_a_negative_payout_on_it() {
    assert_eq!(
        outcome(
            "dividend_payout_on_free_cash_flow",
            &[
                (Concept::DividendsPaid, value("1")),
                (Concept::OperatingCashFlow, value("2")),
                (Concept::CapitalExpenditure, value("3")),
            ]
        ),
        figure("-1")
    );
}

#[test]
fn dividends_paid_a_silence_reading_set_to_zero_give_payouts_of_zero_and_decline_the_coverage() {
    let given = [
        (Concept::DividendsPaid, silent_zero()),
        (Concept::NetIncome, value("4")),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, value("3")),
    ];
    assert_eq!(outcome("dividend_payout_on_earnings", &given), figure("0"));
    assert_eq!(
        outcome("dividend_payout_on_free_cash_flow", &given),
        figure("0")
    );
    assert_eq!(
        outcome("dividend_coverage", &given),
        absent(declined("zero_dividends_paid"), Vec::new())
    );
}

#[test]
fn a_zero_net_income_declines_the_payout_on_earnings_and_not_the_coverage() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::DividendsPaid, value("2")),
            (Concept::NetIncome, value(zero)),
            (Concept::OperatingCashFlow, value("7")),
            (Concept::CapitalExpenditure, value("3")),
        ];
        assert_eq!(
            outcome("dividend_payout_on_earnings", &given),
            absent(declined("zero_net_income"), Vec::new()),
            "{zero}"
        );
        assert_eq!(outcome("dividend_coverage", &given), figure("0"), "{zero}");
        assert_eq!(
            outcome("dividend_payout_on_free_cash_flow", &given),
            figure("0.5"),
            "{zero}"
        );
    }
}

#[test]
fn a_zero_free_cash_flow_declines_the_payout_on_it_under_its_own_condition() {
    for (operating_cash_flow, capital_expenditure) in [("3", "3"), ("3.0", "3.00"), ("0", "0")] {
        let given = [
            (Concept::DividendsPaid, value("1")),
            (Concept::NetIncome, value("4")),
            (Concept::OperatingCashFlow, value(operating_cash_flow)),
            (Concept::CapitalExpenditure, value(capital_expenditure)),
        ];
        assert_eq!(
            outcome("dividend_payout_on_free_cash_flow", &given),
            absent(declined("zero_free_cash_flow"), Vec::new()),
            "{operating_cash_flow} less {capital_expenditure}"
        );
        assert_eq!(
            outcome("dividend_payout_on_earnings", &given),
            figure("0.25")
        );
    }
}

#[test]
fn an_unknown_capital_expenditure_makes_the_payout_on_free_cash_flow_alone_absent_naming_it() {
    let given = [
        (Concept::DividendsPaid, value("1")),
        (Concept::NetIncome, value("4")),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, unknown()),
    ];
    let at = row(fiscal_year(), &given);
    let reason = Reason::input_unknown(&at, Concept::CapitalExpenditure)
        .expect("capital expenditure is Unknown at the row");
    assert_eq!(
        outcome("dividend_payout_on_free_cash_flow", &given),
        absent(reason, Vec::new())
    );
    assert_eq!(
        outcome("dividend_payout_on_earnings", &given),
        figure("0.25")
    );
    assert_eq!(outcome("dividend_coverage", &given), figure("4"));
}

#[test]
fn an_unknown_net_income_makes_both_its_metrics_absent_naming_it() {
    let given = [
        (Concept::DividendsPaid, value("1")),
        (Concept::NetIncome, unknown()),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, value("3")),
    ];
    let at = row(fiscal_year(), &given);
    let reason = || {
        Reason::input_unknown(&at, Concept::NetIncome).expect("net income is Unknown at the row")
    };
    assert_eq!(
        outcome("dividend_payout_on_earnings", &given),
        absent(reason(), Vec::new())
    );
    assert_eq!(
        outcome("dividend_coverage", &given),
        absent(reason(), Vec::new())
    );
    assert_eq!(
        outcome("dividend_payout_on_free_cash_flow", &given),
        figure("0.25")
    );
}

#[test]
fn every_input_absent_gives_a_reason_for_each_in_each_entrys_order() {
    let given = [
        (Concept::DividendsPaid, unknown()),
        (Concept::NetIncome, unknown()),
        (Concept::OperatingCashFlow, unknown()),
        (Concept::CapitalExpenditure, unknown()),
    ];
    let at = row(fiscal_year(), &given);
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");

    assert_eq!(
        outcome("dividend_payout_on_free_cash_flow", &given),
        absent(
            unknown(Concept::DividendsPaid),
            vec![
                unknown(Concept::OperatingCashFlow),
                unknown(Concept::CapitalExpenditure)
            ]
        )
    );
    assert_eq!(
        outcome("dividend_payout_on_earnings", &given),
        absent(
            unknown(Concept::DividendsPaid),
            vec![unknown(Concept::NetIncome)]
        )
    );
    assert_eq!(
        outcome("dividend_coverage", &given),
        absent(
            unknown(Concept::NetIncome),
            vec![unknown(Concept::DividendsPaid)]
        )
    );
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    let given = [
        (Concept::DividendsPaid, value("1")),
        (Concept::NetIncome, value("1e9")),
        (Concept::OperatingCashFlow, value("1,000")),
        (Concept::CapitalExpenditure, value("0x3")),
    ];
    let not_a_decimal = || declined("input_not_a_decimal");
    assert_eq!(
        outcome("dividend_payout_on_free_cash_flow", &given),
        absent(not_a_decimal(), vec![not_a_decimal()])
    );
    assert_eq!(
        outcome("dividend_payout_on_earnings", &given),
        absent(not_a_decimal(), Vec::new())
    );
    assert_eq!(
        outcome("dividend_coverage", &given),
        absent(not_a_decimal(), Vec::new())
    );
}

#[test]
fn an_instant_period_carries_none_of_the_three() {
    let given = [
        (Concept::DividendsPaid, value("1")),
        (Concept::NetIncome, value("4")),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, value("3")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for name in DIVIDENDS {
        assert_eq!(outcome_at(&hand_over, &year_end(), name), None, "{name}");
        assert!(
            outcome_at(&hand_over, &fiscal_year(), name).is_some(),
            "{name}"
        );
    }
}

#[test]
fn every_earlier_metric_is_computed_as_before_beside_them() {
    let given = [
        (Concept::GrossProfit, value("2")),
        (Concept::OperatingIncome, value("3")),
        (Concept::NetIncome, value("-1")),
        (Concept::Revenue, value("8")),
        (Concept::InterestExpense, value("4")),
        (Concept::ShortTermDebt, value("2")),
        (Concept::LongTermDebt, value("3")),
        (Concept::ShareholdersEquity, value("4")),
        (Concept::TotalLiabilities, value("3")),
        (Concept::TotalAssets, value("4")),
        (Concept::CurrentAssets, value("9")),
        (Concept::Inventory, value("1")),
        (Concept::CashAndEquivalents, value("2")),
        (Concept::CurrentLiabilities, value("4")),
        (Concept::DividendsPaid, value("1")),
        (Concept::OperatingCashFlow, value("7")),
        (Concept::CapitalExpenditure, value("3")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for (period, name, characters) in [
        (fiscal_year(), "gross_margin", "0.25"),
        (fiscal_year(), "operating_margin", "0.375"),
        (fiscal_year(), "net_margin", "-0.125"),
        (fiscal_year(), "interest_coverage", "0.75"),
        (year_end(), "debt_to_equity", "1.25"),
        (year_end(), "liabilities_to_assets", "0.75"),
        (year_end(), "current_ratio", "2.25"),
        (year_end(), "quick_ratio", "2"),
        (year_end(), "cash_ratio", "0.5"),
        (fiscal_year(), "dividend_payout_on_earnings", "-1"),
        (fiscal_year(), "dividend_payout_on_free_cash_flow", "0.25"),
        (fiscal_year(), "dividend_coverage", "-1"),
    ] {
        assert_eq!(
            outcome_at(&hand_over, &period, name),
            Some(figure(characters)),
            "{name}"
        );
    }
}

#[test]
fn adding_the_metrics_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![row(fiscal_year(), &[])]).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    let recorded: Vec<(&str, &str)> = premises
        .settings()
        .iter()
        .map(|setting| (&*setting.name, &*setting.value))
        .collect();
    assert_eq!(recorded, [("dividend_streak_years", "10")]);
}
