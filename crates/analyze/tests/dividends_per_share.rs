//! Dividend payout and dividend coverage in their per-share form as a caller
//! reaches them: a history in, and at each duration period each figure or the
//! reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
};

const CIK: &str = "0000320193";

const PAYOUT: &str = "dividend_payout_per_share";
const COVERAGE: &str = "dividend_coverage_per_share";

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

/// The figure `dividends_declared_per_share`'s conditional silence reading
/// sets where the period is silent on both members of its pair.
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

/// The two per-share inputs, as stated.
fn per_share(dividend: Resolution, earnings: Resolution) -> [(Concept, Resolution); 2] {
    [
        (Concept::DividendsDeclaredPerShare, dividend),
        (Concept::EarningsPerShareDiluted, earnings),
    ]
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
fn the_dividend_over_diluted_earnings_per_share_is_the_payout_and_its_inverse_the_coverage() {
    let given = per_share(value("1"), value("4"));
    assert_eq!(outcome(PAYOUT, &given), figure("0.25"));
    assert_eq!(outcome(COVERAGE, &given), figure("4"));
}

#[test]
fn a_payout_that_does_not_terminate_is_written_to_the_precision() {
    let third = format!("0.{}", "3".repeat(34));
    assert_eq!(
        outcome(PAYOUT, &per_share(value("1"), value("3"))),
        figure(&third)
    );
}

#[test]
fn a_loss_per_share_gives_a_negative_payout_and_coverage() {
    let given = per_share(value("0.5"), value("-2"));
    assert_eq!(outcome(PAYOUT, &given), figure("-0.25"));
    assert_eq!(outcome(COVERAGE, &given), figure("-4"));
}

#[test]
fn a_zero_diluted_earnings_per_share_declines_the_payout_and_not_the_coverage() {
    for zero in ["0", "-0", "0.000"] {
        let given = per_share(value("1"), value(zero));
        assert_eq!(
            outcome(PAYOUT, &given),
            absent(declined("zero_earnings_per_share_diluted"), Vec::new()),
            "{zero}"
        );
        assert_eq!(outcome(COVERAGE, &given), figure("0"), "{zero}");
    }
}

#[test]
fn a_dividend_a_silence_reading_set_to_zero_gives_a_payout_of_zero_and_declines_the_coverage() {
    let given = per_share(silent_zero(), value("4"));
    assert_eq!(outcome(PAYOUT, &given), figure("0"));
    assert_eq!(
        outcome(COVERAGE, &given),
        absent(declined("zero_dividends_declared_per_share"), Vec::new())
    );
}

#[test]
fn an_unknown_diluted_earnings_per_share_makes_each_absent_naming_it() {
    let given = per_share(value("1"), unknown());
    let at = row(fiscal_year(), &given);
    let reason = || {
        Reason::input_unknown(&at, Concept::EarningsPerShareDiluted)
            .expect("diluted earnings per share are Unknown at the row")
    };
    assert_eq!(outcome(PAYOUT, &given), absent(reason(), Vec::new()));
    assert_eq!(outcome(COVERAGE, &given), absent(reason(), Vec::new()));
}

#[test]
fn both_inputs_absent_give_two_reasons_in_each_entrys_order() {
    let given = per_share(unknown(), unknown());
    let at = row(fiscal_year(), &given);
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        outcome(PAYOUT, &given),
        absent(
            unknown(Concept::DividendsDeclaredPerShare),
            vec![unknown(Concept::EarningsPerShareDiluted)]
        )
    );
    assert_eq!(
        outcome(COVERAGE, &given),
        absent(
            unknown(Concept::EarningsPerShareDiluted),
            vec![unknown(Concept::DividendsDeclaredPerShare)]
        )
    );
}

#[test]
fn an_amount_outside_the_grammar_is_declined_once_per_such_input() {
    let not_a_decimal = || declined("input_not_a_decimal");
    let one = per_share(value("1"), value("1e9"));
    assert_eq!(outcome(PAYOUT, &one), absent(not_a_decimal(), Vec::new()));
    assert_eq!(outcome(COVERAGE, &one), absent(not_a_decimal(), Vec::new()));

    let both = per_share(value("0x1"), value("1,000"));
    for name in [PAYOUT, COVERAGE] {
        assert_eq!(
            outcome(name, &both),
            absent(not_a_decimal(), vec![not_a_decimal()]),
            "{name}"
        );
    }
}

#[test]
fn an_instant_period_carries_neither() {
    let given = per_share(value("1"), value("4"));
    let hand_over = handed(vec![row(year_end(), &given), row(fiscal_year(), &given)]);
    for name in [PAYOUT, COVERAGE] {
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
        (Concept::DividendsDeclaredPerShare, value("1")),
        (Concept::EarningsPerShareDiluted, value("4")),
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
        (fiscal_year(), PAYOUT, "0.25"),
        (fiscal_year(), COVERAGE, "4"),
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
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
