//! Operating margin, net margin and interest coverage as a caller reaches
//! them: a history in, and at each duration period each figure or the reasons
//! it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

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
fn operating_income_over_revenue_is_the_operating_margin() {
    assert_eq!(
        outcome(
            "operating_margin",
            &[
                (Concept::OperatingIncome, value("3")),
                (Concept::Revenue, value("8")),
            ]
        ),
        figure("0.375")
    );
}

#[test]
fn a_net_loss_gives_a_negative_net_margin() {
    assert_eq!(
        outcome(
            "net_margin",
            &[
                (Concept::NetIncome, value("-1")),
                (Concept::Revenue, value("4")),
            ]
        ),
        figure("-0.25")
    );
}

#[test]
fn a_net_margin_that_does_not_terminate_is_written_to_the_precision() {
    let third = format!("0.{}", "3".repeat(34));
    assert_eq!(
        outcome(
            "net_margin",
            &[
                (Concept::NetIncome, value("1")),
                (Concept::Revenue, value("3")),
            ]
        ),
        figure(&third)
    );
}

#[test]
fn operating_income_over_interest_expense_is_the_coverage() {
    assert_eq!(
        outcome(
            "interest_coverage",
            &[
                (Concept::OperatingIncome, value("17")),
                (Concept::InterestExpense, value("4")),
            ]
        ),
        figure("4.25")
    );
}

#[test]
fn a_not_applicable_operating_income_makes_both_its_metrics_absent_and_not_net_margin() {
    let given = [
        (
            Concept::OperatingIncome,
            not_applicable(Concept::OperatingIncome),
        ),
        (Concept::Revenue, value("4")),
        (Concept::InterestExpense, value("2")),
        (Concept::NetIncome, value("1")),
    ];
    let at = row(fiscal_year(), &given);
    let reason = || {
        Reason::input_not_applicable(&at, Concept::OperatingIncome)
            .expect("operating income is NotApplicable at the row")
    };
    assert_eq!(
        outcome("operating_margin", &given),
        absent(reason(), Vec::new())
    );
    assert_eq!(
        outcome("interest_coverage", &given),
        absent(reason(), Vec::new())
    );
    assert_eq!(outcome("net_margin", &given), figure("0.25"));
}

#[test]
fn an_unknown_interest_expense_makes_the_coverage_absent_naming_it() {
    let given = [
        (Concept::OperatingIncome, value("17")),
        (Concept::InterestExpense, unknown()),
    ];
    let at = row(fiscal_year(), &given);
    let reason = Reason::input_unknown(&at, Concept::InterestExpense)
        .expect("interest expense is Unknown at the row");
    assert_eq!(
        outcome("interest_coverage", &given),
        absent(reason, Vec::new())
    );
}

#[test]
fn both_inputs_absent_give_both_reasons_in_each_entrys_order() {
    let given = [
        (
            Concept::OperatingIncome,
            not_applicable(Concept::OperatingIncome),
        ),
        (Concept::Revenue, unknown()),
        (Concept::NetIncome, unknown()),
        (Concept::InterestExpense, unknown()),
    ];
    let at = row(fiscal_year(), &given);
    let not_applicable = |concept| {
        Reason::input_not_applicable(&at, concept).expect("the concept is NotApplicable at the row")
    };
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");

    assert_eq!(
        outcome("operating_margin", &given),
        absent(
            not_applicable(Concept::OperatingIncome),
            vec![unknown(Concept::Revenue)]
        )
    );
    assert_eq!(
        outcome("net_margin", &given),
        absent(unknown(Concept::NetIncome), vec![unknown(Concept::Revenue)])
    );
    assert_eq!(
        outcome("interest_coverage", &given),
        absent(
            not_applicable(Concept::OperatingIncome),
            vec![unknown(Concept::InterestExpense)]
        )
    );
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    let given = [
        (Concept::OperatingIncome, value("1e9")),
        (Concept::Revenue, value("1,000")),
        (Concept::NetIncome, value("2")),
        (Concept::InterestExpense, value("4")),
    ];
    assert_eq!(
        outcome("operating_margin", &given),
        absent(
            declined("input_not_a_decimal"),
            vec![declined("input_not_a_decimal")]
        )
    );
    assert_eq!(
        outcome("net_margin", &given),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
    assert_eq!(
        outcome("interest_coverage", &given),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
}

#[test]
fn a_zero_revenue_declines_each_margin_under_its_own_condition() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::OperatingIncome, value("3")),
            (Concept::NetIncome, value("1")),
            (Concept::Revenue, value(zero)),
            (Concept::InterestExpense, value("4")),
        ];
        for margin in ["gross_margin", "operating_margin", "net_margin"] {
            assert_eq!(
                outcome(margin, &given),
                absent(declined("zero_revenue"), Vec::new()),
                "{margin} at {zero}"
            );
        }
        assert_eq!(outcome("interest_coverage", &given), figure("0.75"));
    }
}

#[test]
fn a_zero_interest_expense_declines_the_coverage_under_its_own_condition() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::OperatingIncome, value("17")),
            (Concept::Revenue, value("8")),
            (Concept::InterestExpense, value(zero)),
        ];
        assert_eq!(
            outcome("interest_coverage", &given),
            absent(declined("zero_interest_expense"), Vec::new()),
            "{zero}"
        );
        assert_eq!(outcome("operating_margin", &given), figure("2.125"));
    }
}

#[test]
fn an_instant_period_carries_none_of_the_three() {
    let given = [
        (Concept::OperatingIncome, value("3")),
        (Concept::Revenue, value("8")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for name in ["operating_margin", "net_margin", "interest_coverage"] {
        assert_eq!(outcome_at(&hand_over, &year_end(), name), None, "{name}");
        assert!(
            outcome_at(&hand_over, &fiscal_year(), name).is_some(),
            "{name}"
        );
    }
}

#[test]
fn gross_margin_is_still_computed_beside_the_three() {
    let given = [
        (Concept::GrossProfit, value("2")),
        (Concept::Revenue, value("5")),
    ];
    assert_eq!(outcome("gross_margin", &given), figure("0.4"));

    let third = format!("-0.{}", "3".repeat(34));
    let given = [
        (Concept::GrossProfit, value("-1")),
        (Concept::Revenue, value("3")),
    ];
    assert_eq!(outcome("gross_margin", &given), figure(&third));
}

#[test]
fn the_hand_over_carries_each_metric_by_its_entrys_name_in_order() {
    let hand_over = handed(vec![row(fiscal_year(), &[])]);
    let names: Vec<&str> = hand_over.results()[0]
        .metrics()
        .iter()
        .map(|metric| &*metric.name)
        .collect();
    assert_eq!(
        names,
        [
            "gross_margin",
            "operating_margin",
            "net_margin",
            "interest_coverage"
        ]
    );
}

#[test]
fn adding_the_metrics_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![row(fiscal_year(), &[])]).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
