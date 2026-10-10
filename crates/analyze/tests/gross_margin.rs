//! Gross margin as a caller reaches it: a history in, and at each duration
//! period the figure or the reasons it has none.

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

fn prior_fiscal_year() -> Period {
    Period::Duration {
        start: "2023-10-01".into(),
        end: "2024-09-28".into(),
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

/// A row at `period` with the two inputs given and every other concept a
/// value the metric does not read.
fn row(period: Period, gross_profit: Resolution, revenue: Resolution) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| match Concept::ALL[at] {
            Concept::GrossProfit => gross_profit.clone(),
            Concept::Revenue => revenue.clone(),
            _ => value("7"),
        }),
    )
}

fn handed(rows: Vec<Row>) -> HandOver {
    analyze(History::of(CIK.into(), rows), None, &Settings::preset())
        .expect("no crossing is handed")
}

/// Gross margin's outcome at `period`, or none where the results there hold
/// no gross margin.
fn gross_margin_at(hand_over: &HandOver, period: &Period) -> Option<Outcome> {
    hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics()
        .iter()
        .find(|metric| &*metric.name == "gross_margin")
        .map(|metric| metric.outcome.clone())
}

/// Gross margin over one fiscal year with these two inputs.
fn gross_margin(gross_profit: Resolution, revenue: Resolution) -> Outcome {
    let hand_over = handed(vec![row(fiscal_year(), gross_profit, revenue)]);
    gross_margin_at(&hand_over, &fiscal_year()).expect("gross margin is stated at a duration")
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
fn gross_profit_over_revenue_is_the_figure() {
    assert_eq!(gross_margin(value("2"), value("5")), figure("0.4"));
}

#[test]
fn a_negative_gross_profit_gives_a_negative_figure_written_to_the_precision() {
    let third = format!("-0.{}", "3".repeat(34));
    assert_eq!(gross_margin(value("-1"), value("3")), figure(&third));
}

#[test]
fn a_not_applicable_gross_profit_is_absent_naming_it() {
    let at = row(
        fiscal_year(),
        not_applicable(Concept::GrossProfit),
        value("5"),
    );
    let reason = Reason::input_not_applicable(&at, Concept::GrossProfit)
        .expect("gross profit is NotApplicable at the row");
    assert_eq!(
        gross_margin(not_applicable(Concept::GrossProfit), value("5")),
        absent(reason, Vec::new())
    );
}

#[test]
fn an_unknown_revenue_is_absent_naming_it() {
    let at = row(fiscal_year(), value("2"), unknown());
    let reason =
        Reason::input_unknown(&at, Concept::Revenue).expect("revenue is Unknown at the row");
    assert_eq!(
        gross_margin(value("2"), unknown()),
        absent(reason, Vec::new())
    );
}

#[test]
fn both_inputs_absent_give_both_reasons_in_the_entrys_order() {
    let at = row(
        fiscal_year(),
        not_applicable(Concept::GrossProfit),
        unknown(),
    );
    let gross_profit = Reason::input_not_applicable(&at, Concept::GrossProfit)
        .expect("gross profit is NotApplicable at the row");
    let revenue =
        Reason::input_unknown(&at, Concept::Revenue).expect("revenue is Unknown at the row");
    assert_eq!(
        gross_margin(not_applicable(Concept::GrossProfit), unknown()),
        absent(gross_profit, vec![revenue])
    );
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    assert_eq!(
        gross_margin(value("1e9"), value("5")),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
    assert_eq!(
        gross_margin(value("2"), value("1,000")),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
    assert_eq!(
        gross_margin(value("1e9"), value("1e9")),
        absent(
            declined("input_not_a_decimal"),
            vec![declined("input_not_a_decimal")]
        )
    );
}

#[test]
fn an_unreadable_input_beside_an_absent_one_gives_both_in_order() {
    let at = row(fiscal_year(), value("1e9"), unknown());
    let revenue =
        Reason::input_unknown(&at, Concept::Revenue).expect("revenue is Unknown at the row");
    assert_eq!(
        gross_margin(value("1e9"), unknown()),
        absent(declined("input_not_a_decimal"), vec![revenue])
    );
}

#[test]
fn a_zero_revenue_is_declined_under_the_entrys_condition() {
    for zero in ["0", "-0", "0.000"] {
        assert_eq!(
            gross_margin(value("2"), value(zero)),
            absent(declined("zero_revenue"), Vec::new()),
            "{zero}"
        );
    }
}

#[test]
fn a_zero_revenue_beside_an_unread_gross_profit_is_not_its_condition() {
    assert_eq!(
        gross_margin(value("1e9"), value("0")),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
}

#[test]
fn an_instant_period_carries_no_gross_margin() {
    let hand_over = handed(vec![
        row(year_end(), value("2"), value("5")),
        row(fiscal_year(), value("2"), value("5")),
    ]);
    assert_eq!(gross_margin_at(&hand_over, &year_end()), None);
    assert_eq!(
        gross_margin_at(&hand_over, &fiscal_year()),
        Some(figure("0.4"))
    );
}

#[test]
fn each_duration_reads_its_own_row_alone() {
    let hand_over = handed(vec![
        row(fiscal_year(), value("2"), value("5")),
        row(prior_fiscal_year(), value("3"), unknown()),
    ]);
    assert_eq!(
        gross_margin_at(&hand_over, &fiscal_year()),
        Some(figure("0.4"))
    );
    assert!(matches!(
        gross_margin_at(&hand_over, &prior_fiscal_year()),
        Some(Outcome::Absent { .. })
    ));
}

#[test]
fn the_same_history_gives_the_same_hand_over() {
    let rows = || {
        vec![
            row(year_end(), value("2"), value("5")),
            row(fiscal_year(), value("-1"), value("3")),
            row(
                prior_fiscal_year(),
                not_applicable(Concept::GrossProfit),
                unknown(),
            ),
        ]
    };
    assert_eq!(handed(rows()), handed(rows()));
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![row(fiscal_year(), value("2"), value("5"))])
        .premises()
        .clone();
    assert_eq!(premises.method_version().get(), 1);
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
