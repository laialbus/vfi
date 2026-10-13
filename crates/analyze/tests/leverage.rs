//! Debt to equity and liabilities to assets as a caller reaches them: a
//! history in, and at each instant period each figure or the reasons it has
//! none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
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

fn silent_zero() -> Resolution {
    Resolution::Value {
        amount: "0".into(),
        set_by: SetBy::Silence {
            reading: Silence::Zero,
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
fn total_liabilities_over_total_assets_is_liabilities_to_assets() {
    assert_eq!(
        outcome(
            "liabilities_to_assets",
            &[
                (Concept::TotalLiabilities, value("3")),
                (Concept::TotalAssets, value("4")),
            ]
        ),
        figure("0.75")
    );
}

#[test]
fn both_debts_over_shareholders_equity_is_debt_to_equity() {
    assert_eq!(
        outcome(
            "debt_to_equity",
            &[
                (Concept::ShortTermDebt, value("2")),
                (Concept::LongTermDebt, value("3")),
                (Concept::ShareholdersEquity, value("4")),
            ]
        ),
        figure("1.25")
    );
}

#[test]
fn a_negative_shareholders_equity_gives_a_negative_debt_to_equity() {
    assert_eq!(
        outcome(
            "debt_to_equity",
            &[
                (Concept::ShortTermDebt, value("2")),
                (Concept::LongTermDebt, value("3")),
                (Concept::ShareholdersEquity, value("-4")),
            ]
        ),
        figure("-1.25")
    );
}

#[test]
fn a_debt_a_silence_reading_set_to_zero_sums_as_zero() {
    let third = format!("0.{}", "3".repeat(34));
    assert_eq!(
        outcome(
            "debt_to_equity",
            &[
                (Concept::ShortTermDebt, silent_zero()),
                (Concept::LongTermDebt, value("1")),
                (Concept::ShareholdersEquity, value("3")),
            ]
        ),
        figure(&third)
    );
}

#[test]
fn a_zero_shareholders_equity_declines_debt_to_equity_under_its_own_condition() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::ShortTermDebt, value("2")),
            (Concept::LongTermDebt, value("3")),
            (Concept::ShareholdersEquity, value(zero)),
            (Concept::TotalLiabilities, value("3")),
            (Concept::TotalAssets, value("4")),
        ];
        assert_eq!(
            outcome("debt_to_equity", &given),
            absent(declined("zero_shareholders_equity"), Vec::new()),
            "{zero}"
        );
        assert_eq!(outcome("liabilities_to_assets", &given), figure("0.75"));
    }
}

#[test]
fn a_zero_total_assets_declines_liabilities_to_assets_under_its_own_condition() {
    for zero in ["0", "-0", "0.000"] {
        let given = [
            (Concept::TotalLiabilities, value("3")),
            (Concept::TotalAssets, value(zero)),
            (Concept::ShortTermDebt, value("2")),
            (Concept::LongTermDebt, value("3")),
            (Concept::ShareholdersEquity, value("4")),
        ];
        assert_eq!(
            outcome("liabilities_to_assets", &given),
            absent(declined("zero_total_assets"), Vec::new()),
            "{zero}"
        );
        assert_eq!(outcome("debt_to_equity", &given), figure("1.25"));
    }
}

#[test]
fn an_unknown_long_term_debt_makes_debt_to_equity_absent_naming_it() {
    let given = [
        (Concept::ShortTermDebt, value("2")),
        (Concept::LongTermDebt, unknown()),
        (Concept::ShareholdersEquity, value("4")),
    ];
    let at = row(year_end(), &given);
    let reason = Reason::input_unknown(&at, Concept::LongTermDebt)
        .expect("long-term debt is Unknown at the row");
    assert_eq!(
        outcome("debt_to_equity", &given),
        absent(reason, Vec::new())
    );
}

#[test]
fn all_three_inputs_absent_give_three_reasons_in_the_entrys_order() {
    let given = [
        (Concept::ShareholdersEquity, unknown()),
        (Concept::LongTermDebt, unknown()),
        (Concept::ShortTermDebt, unknown()),
    ];
    let at = row(year_end(), &given);
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        outcome("debt_to_equity", &given),
        absent(
            unknown(Concept::ShortTermDebt),
            vec![
                unknown(Concept::LongTermDebt),
                unknown(Concept::ShareholdersEquity)
            ]
        )
    );
}

#[test]
fn both_inputs_absent_give_both_reasons_in_the_entrys_order() {
    let given = [
        (Concept::TotalAssets, unknown()),
        (Concept::TotalLiabilities, unknown()),
    ];
    let at = row(year_end(), &given);
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        outcome("liabilities_to_assets", &given),
        absent(
            unknown(Concept::TotalLiabilities),
            vec![unknown(Concept::TotalAssets)]
        )
    );
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    let given = [
        (Concept::ShortTermDebt, value("1e9")),
        (Concept::LongTermDebt, value("3")),
        (Concept::ShareholdersEquity, value("1,000")),
        (Concept::TotalLiabilities, value("+3")),
        (Concept::TotalAssets, value("4")),
    ];
    assert_eq!(
        outcome("debt_to_equity", &given),
        absent(
            declined("input_not_a_decimal"),
            vec![declined("input_not_a_decimal")]
        )
    );
    assert_eq!(
        outcome("liabilities_to_assets", &given),
        absent(declined("input_not_a_decimal"), Vec::new())
    );
}

#[test]
fn a_duration_period_carries_neither() {
    let given = [
        (Concept::TotalLiabilities, value("3")),
        (Concept::TotalAssets, value("4")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for name in ["debt_to_equity", "liabilities_to_assets"] {
        assert_eq!(outcome_at(&hand_over, &fiscal_year(), name), None, "{name}");
        assert!(
            outcome_at(&hand_over, &year_end(), name).is_some(),
            "{name}"
        );
    }
}

#[test]
fn the_metrics_at_a_duration_are_computed_as_before_beside_them() {
    let given = [
        (Concept::GrossProfit, value("2")),
        (Concept::OperatingIncome, value("3")),
        (Concept::NetIncome, value("-1")),
        (Concept::Revenue, value("8")),
        (Concept::InterestExpense, value("4")),
    ];
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for (name, characters) in [
        ("gross_margin", "0.25"),
        ("operating_margin", "0.375"),
        ("net_margin", "-0.125"),
        ("interest_coverage", "0.75"),
    ] {
        assert_eq!(
            outcome_at(&hand_over, &fiscal_year(), name),
            Some(figure(characters)),
            "{name}"
        );
        assert_eq!(outcome_at(&hand_over, &year_end(), name), None, "{name}");
    }
}

#[test]
fn an_instant_carries_each_metric_by_its_entrys_name_in_order() {
    let hand_over = handed(vec![row(year_end(), &[])]);
    let names: Vec<&str> = hand_over.results()[0]
        .metrics()
        .iter()
        .map(|metric| &*metric.name)
        .collect();
    assert_eq!(
        names,
        [
            "debt_to_equity",
            "liabilities_to_assets",
            "current_ratio",
            "quick_ratio",
            "cash_ratio"
        ]
    );
}

#[test]
fn adding_the_metrics_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![row(year_end(), &[])]).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
