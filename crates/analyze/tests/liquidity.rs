//! The current, quick and cash ratios as a caller reaches them: a history in,
//! and at each instant period each figure or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
    Silence,
};

const CIK: &str = "0000320193";

const LIQUIDITY: [&str; 3] = ["current_ratio", "quick_ratio", "cash_ratio"];

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

fn silent_zero() -> Resolution {
    Resolution::Value {
        amount: "0".into(),
        set_by: SetBy::Silence {
            reading: Silence::Zero,
            registry: "1".into(),
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

/// `name`'s outcome at one year end with `given` stated.
fn outcome(name: &str, given: &[(Concept, Resolution)]) -> Outcome {
    let hand_over = handed(vec![row(year_end(), given)]);
    outcome_at(&hand_over, &year_end(), name).expect("the metric is stated at an instant")
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
fn current_assets_over_current_liabilities_is_the_current_ratio() {
    assert_eq!(
        outcome(
            "current_ratio",
            &[
                (Concept::CurrentAssets, value("9")),
                (Concept::CurrentLiabilities, value("4")),
            ]
        ),
        figure("2.25")
    );
}

#[test]
fn current_assets_less_inventory_over_current_liabilities_is_the_quick_ratio() {
    assert_eq!(
        outcome(
            "quick_ratio",
            &[
                (Concept::CurrentAssets, value("9")),
                (Concept::Inventory, value("1")),
                (Concept::CurrentLiabilities, value("4")),
            ]
        ),
        figure("2")
    );
}

#[test]
fn an_inventory_a_silence_reading_set_to_zero_gives_the_current_ratio() {
    let given = [
        (Concept::CurrentAssets, value("9")),
        (Concept::Inventory, silent_zero()),
        (Concept::CurrentLiabilities, value("4")),
    ];
    assert_eq!(outcome("quick_ratio", &given), figure("2.25"));
    assert_eq!(outcome("current_ratio", &given), figure("2.25"));
}

#[test]
fn cash_and_equivalents_over_current_liabilities_is_the_cash_ratio() {
    assert_eq!(
        outcome(
            "cash_ratio",
            &[
                (Concept::CashAndEquivalents, value("1")),
                (Concept::CurrentLiabilities, value("4")),
            ]
        ),
        figure("0.25")
    );
}

#[test]
fn the_cash_ratio_reads_no_short_term_investments() {
    for short_term_investments in [value("100"), unknown(), value("1e9")] {
        assert_eq!(
            outcome(
                "cash_ratio",
                &[
                    (Concept::CashAndEquivalents, value("1")),
                    (Concept::ShortTermInvestments, short_term_investments),
                    (Concept::CurrentLiabilities, value("4")),
                ]
            ),
            figure("0.25")
        );
    }
}

#[test]
fn a_quick_ratio_that_does_not_terminate_is_written_to_the_precision() {
    let third = format!("0.{}", "3".repeat(34));
    assert_eq!(
        outcome(
            "quick_ratio",
            &[
                (Concept::CurrentAssets, value("1")),
                (Concept::Inventory, value("0")),
                (Concept::CurrentLiabilities, value("3")),
            ]
        ),
        figure(&third)
    );
}

#[test]
fn a_negative_difference_gives_a_negative_quick_ratio() {
    assert_eq!(
        outcome(
            "quick_ratio",
            &[
                (Concept::CurrentAssets, value("1")),
                (Concept::Inventory, value("2")),
                (Concept::CurrentLiabilities, value("4")),
            ]
        ),
        figure("-0.25")
    );
}

#[test]
fn a_zero_current_liabilities_declines_each_under_its_own_condition() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::CurrentAssets, value("9")),
            (Concept::Inventory, value("1")),
            (Concept::CashAndEquivalents, value("1")),
            (Concept::CurrentLiabilities, value(zero)),
        ];
        for name in LIQUIDITY {
            assert_eq!(
                outcome(name, &given),
                absent(declined("zero_current_liabilities"), Vec::new()),
                "{name} at {zero}"
            );
        }
    }
}

#[test]
fn a_bank_carries_each_absent_naming_each_input_not_applicable_in_order() {
    let given = [
        (
            Concept::CurrentAssets,
            not_applicable(Concept::CurrentAssets),
        ),
        (Concept::Inventory, not_applicable(Concept::Inventory)),
        (
            Concept::CurrentLiabilities,
            not_applicable(Concept::CurrentLiabilities),
        ),
        (Concept::CashAndEquivalents, value("1")),
    ];
    let at = row(year_end(), &given);
    let not_applicable = |concept| {
        Reason::input_not_applicable(&at, concept).expect("the concept is NotApplicable at the row")
    };
    assert_eq!(
        outcome("current_ratio", &given),
        absent(
            not_applicable(Concept::CurrentAssets),
            vec![not_applicable(Concept::CurrentLiabilities)]
        )
    );
    assert_eq!(
        outcome("quick_ratio", &given),
        absent(
            not_applicable(Concept::CurrentAssets),
            vec![
                not_applicable(Concept::Inventory),
                not_applicable(Concept::CurrentLiabilities)
            ]
        )
    );
    assert_eq!(
        outcome("cash_ratio", &given),
        absent(not_applicable(Concept::CurrentLiabilities), Vec::new())
    );
}

#[test]
fn an_unknown_inventory_makes_the_quick_ratio_alone_absent_naming_it() {
    let given = [
        (Concept::CurrentAssets, value("9")),
        (Concept::Inventory, unknown()),
        (Concept::CashAndEquivalents, value("1")),
        (Concept::CurrentLiabilities, value("4")),
    ];
    let at = row(year_end(), &given);
    let reason =
        Reason::input_unknown(&at, Concept::Inventory).expect("inventory is Unknown at the row");
    assert_eq!(outcome("quick_ratio", &given), absent(reason, Vec::new()));
    assert_eq!(outcome("current_ratio", &given), figure("2.25"));
    assert_eq!(outcome("cash_ratio", &given), figure("0.25"));
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    let given = [
        (Concept::CurrentAssets, value("1e9")),
        (Concept::Inventory, value("1,000")),
        (Concept::CashAndEquivalents, value("1")),
        (Concept::CurrentLiabilities, value("+4")),
    ];
    let not_a_decimal = || declined("input_not_a_decimal");
    assert_eq!(
        outcome("current_ratio", &given),
        absent(not_a_decimal(), vec![not_a_decimal()])
    );
    assert_eq!(
        outcome("quick_ratio", &given),
        absent(not_a_decimal(), vec![not_a_decimal(), not_a_decimal()])
    );
    assert_eq!(
        outcome("cash_ratio", &given),
        absent(not_a_decimal(), Vec::new())
    );
}

#[test]
fn a_duration_period_carries_none_of_the_three() {
    let given = [
        (Concept::CurrentAssets, value("9")),
        (Concept::CurrentLiabilities, value("4")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for name in LIQUIDITY {
        assert_eq!(outcome_at(&hand_over, &fiscal_year(), name), None, "{name}");
        assert!(
            outcome_at(&hand_over, &year_end(), name).is_some(),
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
        (Concept::CurrentLiabilities, value("4")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for (period, name, characters) in [
        (fiscal_year(), "gross_margin", "0.25"),
        (fiscal_year(), "operating_margin", "0.375"),
        (fiscal_year(), "net_margin", "-0.125"),
        (fiscal_year(), "interest_coverage", "0.75"),
        (year_end(), "debt_to_equity", "1.25"),
        (year_end(), "liabilities_to_assets", "0.75"),
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
    let premises = handed(vec![row(year_end(), &[])]).premises().clone();
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
