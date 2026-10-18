//! Retention per share as a caller reaches it: a history in, and at each
//! duration period one less the dividend declared per share over diluted
//! earnings per share, or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy, Silence,
};

const CIK: &str = "0000320193";

const NAME: &str = "retention_per_share";

const PAYOUT: &str = "dividend_payout_per_share";

const OPENING: &str = "2023-12-31";

const CLOSING: &str = "2024-12-31";

fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
}

fn year() -> Period {
    Period::Duration {
        start: "2024-01-01".into(),
        end: CLOSING.into(),
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

/// `name`'s outcome over the year with `given` stated.
fn outcome_of(name: &str, given: &[(Concept, Resolution)]) -> Outcome {
    let hand_over = handed(vec![row(year(), given)]);
    outcome_at(&hand_over, &year(), name).expect("the metric is stated at a duration")
}

fn outcome(given: &[(Concept, Resolution)]) -> Outcome {
    outcome_of(NAME, given)
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
fn retention_is_one_less_the_payout_at_the_same_row() {
    let given = per_share(value("0.5"), value("2"));
    assert_eq!(outcome(&given), figure("0.75"));
    assert_eq!(outcome_of(PAYOUT, &given), figure("0.25"));
}

#[test]
fn a_zero_diluted_earnings_per_share_declines_retention_as_it_declines_the_payout() {
    for zero in ["0", "-0", "0.000"] {
        let given = per_share(value("0.5"), value(zero));
        let zero_earnings = absent(declined("zero_earnings_per_share_diluted"), Vec::new());
        assert_eq!(outcome(&given), zero_earnings, "{zero}");
        assert_eq!(outcome_of(PAYOUT, &given), zero_earnings, "{zero}");
    }
}

#[test]
fn a_loss_per_share_gives_a_retention_above_one_and_no_decline() {
    let given = per_share(value("0.5"), value("-2"));
    assert_eq!(outcome(&given), figure("1.25"));
    assert_eq!(outcome_of(PAYOUT, &given), figure("-0.25"));
}

#[test]
fn a_dividend_of_zero_retains_all_of_the_earnings() {
    for dividend in [value("0"), silent_zero()] {
        assert_eq!(outcome(&per_share(dividend, value("2"))), figure("1"));
    }
}

#[test]
fn an_unknown_diluted_earnings_per_share_is_named_at_the_duration() {
    let given = per_share(value("0.5"), unknown());
    let at = row(year(), &given);
    let reason = Reason::input_unknown(&at, Concept::EarningsPerShareDiluted)
        .expect("diluted earnings per share are Unknown at the row");
    assert_eq!(outcome(&given), absent(reason, Vec::new()));
}

#[test]
fn both_inputs_unknown_give_two_reasons_the_dividend_first() {
    let given = per_share(unknown(), unknown());
    let at = row(year(), &given);
    let unknown =
        |concept| Reason::input_unknown(&at, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        outcome(&given),
        absent(
            unknown(Concept::DividendsDeclaredPerShare),
            vec![unknown(Concept::EarningsPerShareDiluted)]
        )
    );
}

#[test]
fn unknown_preferred_concepts_at_every_row_leave_the_figure_as_it_is() {
    let [dividend, earnings] = per_share(value("0.5"), value("2"));
    let given = [
        dividend,
        earnings,
        (Concept::PreferredEquity, unknown()),
        (Concept::PreferredDividends, unknown()),
    ];
    let hand_over = handed(vec![
        row(instant(OPENING), &given),
        row(instant(CLOSING), &given),
        row(year(), &given),
    ]);
    assert_eq!(outcome_at(&hand_over, &year(), NAME), Some(figure("0.75")));
}

#[test]
fn an_instant_period_carries_no_retention_per_share() {
    let given = per_share(value("0.5"), value("2"));
    let hand_over = handed(vec![row(instant(CLOSING), &given), row(year(), &given)]);
    assert_eq!(outcome_at(&hand_over, &instant(CLOSING), NAME), None);
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
        (year(), PAYOUT, "0.25"),
        (year(), "dividend_coverage_per_share", "4"),
        (year(), "return_on_assets", "-0.25"),
        (year(), "asset_turnover", "2"),
        (year(), "return_on_equity", "-0.25"),
        (year(), "return_on_common_equity", "-1"),
        (year(), NAME, "0.75"),
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
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
