//! The entry point as a caller reaches it: three inputs in, one filer's
//! hand-over or the one error out.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Prices};
use vfi_contracts::canonical_concepts::{History, Period, Resolution, Row, SetBy, Silence};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

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

fn row(period: Period) -> Row {
    Row::at(
        period,
        std::array::from_fn(|_| Resolution::Value {
            amount: "0".into(),
            set_by: SetBy::Silence {
                reading: Silence::Zero,
                registry: "1".into(),
            },
        }),
    )
}

fn history() -> History {
    History::of(CIK.into(), vec![row(year_end()), row(fiscal_year())])
}

fn crossing(cik: &str) -> Crossing {
    Crossing {
        cik: cik.into(),
        ticker: "AAPL".into(),
        at_valuation_date: Answer::Price {
            date_asked: "2026-10-01".into(),
            trading_date: "2026-09-30".into(),
            close: "254.63".into(),
            retrieved_from: "https://example.invalid/prices".into(),
        },
        at_earlier_dates: vec![Answer::Absence {
            date_asked: "2025-09-27".into(),
            reason: fetch_analyze::Reason::NoClose {},
        }],
        splits: Splits::Listed {
            factors: vec![SplitFactor {
                date: "2020-08-31".into(),
                factor: "4:1".into(),
            }],
        },
    }
}

fn handed(prices: Option<Crossing>) -> HandOver {
    analyze(history(), prices, &Settings::preset()).expect("the crossing is the history's filer's")
}

#[test]
fn the_same_three_inputs_give_the_same_hand_over() {
    assert_eq!(handed(Some(crossing(CIK))), handed(Some(crossing(CIK))));
    assert_eq!(handed(None), handed(None));
}

#[test]
fn a_crossing_for_another_filer_is_the_one_error() {
    let other = "0000789019";
    let refused = analyze(history(), Some(crossing(other)), &Settings::preset())
        .expect_err("a crossing for another filer was handed over");

    let expected: (&str, &str) = (CIK, other);
    assert_eq!((refused.history(), refused.crossing()), expected);
}

#[test]
fn the_hand_over_carries_the_history_and_crossing_as_handed() {
    let asked = handed(Some(crossing(CIK)));
    assert_eq!(asked.history(), &history());
    assert_eq!(
        asked.prices(),
        &Prices::Asked {
            crossing: crossing(CIK)
        }
    );
}

#[test]
fn no_crossing_is_handed_over_as_none_asked() {
    assert_eq!(handed(None).prices(), &Prices::NotAsked {});
}

#[test]
fn the_premises_are_the_version_and_each_setting_by_name_with_the_value_handed() {
    let recorded = |settings: &Settings| {
        let premises = analyze(history(), None, settings)
            .expect("no crossing is handed")
            .premises()
            .clone();
        assert_eq!(premises.method_version().get(), 1);
        premises
            .settings()
            .iter()
            .map(|setting| (setting.name.to_string(), setting.value.to_string()))
            .collect::<Vec<_>>()
    };
    let named = |values: [&str; 6]| {
        [
            "dividend_streak_years",
            "discounted_cash_flow_discount_rate",
            "discounted_cash_flow_growth_rate",
            "discounted_cash_flow_years",
            "discounted_cash_flow_terminal_growth_rate",
            "dividend_growth_years",
        ]
        .into_iter()
        .zip(values)
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect::<Vec<_>>()
    };

    assert_eq!(
        recorded(&Settings::preset()),
        named(["10", "0.0772", "0.02", "5", "0.02", "5"])
    );
    let handed = Settings::preset()
        .with_dividend_streak_years(2)
        .and_then(|settings| settings.with_discounted_cash_flow_discount_rate("0.1"))
        .and_then(|settings| settings.with_discounted_cash_flow_growth_rate("-0.05"))
        .and_then(|settings| settings.with_discounted_cash_flow_years(12))
        .and_then(|settings| settings.with_discounted_cash_flow_terminal_growth_rate("0.015"))
        .and_then(|settings| settings.with_dividend_growth_years(3))
        .expect("each value is inside its domain");
    assert_eq!(
        recorded(&handed),
        named(["2", "0.1", "-0.05", "12", "0.015", "3"])
    );
}

#[test]
fn a_setting_outside_its_domain_is_refused_before_analyze_runs() {
    let refused = Settings::preset()
        .with_dividend_streak_years(0)
        .expect_err("zero is not a positive integer");
    assert_eq!(refused.value(), "0");
    assert_eq!(refused.domain(), "the positive integers");
}

#[test]
fn a_discounted_cash_flow_setting_outside_its_domain_is_refused_naming_it_and_the_value() {
    let above_zero = "rates above zero, each a decimal of no more significant digits than a figure is written to";
    let above_minus_one = "rates above minus one, each a decimal of no more significant digits than a figure is written to";
    let thirty_five_digits = "0.12345678901234567890123456789012345";
    let preset = Settings::preset;

    let refusals = [
        (
            preset().with_discounted_cash_flow_discount_rate("0"),
            above_zero,
            "0",
        ),
        (
            preset().with_discounted_cash_flow_discount_rate("-0.01"),
            above_zero,
            "-0.01",
        ),
        (
            preset().with_discounted_cash_flow_discount_rate(thirty_five_digits),
            above_zero,
            thirty_five_digits,
        ),
        (
            preset().with_discounted_cash_flow_discount_rate("7.72%"),
            above_zero,
            "7.72%",
        ),
        (
            preset().with_discounted_cash_flow_growth_rate("-1"),
            above_minus_one,
            "-1",
        ),
        (
            preset().with_discounted_cash_flow_growth_rate(thirty_five_digits),
            above_minus_one,
            thirty_five_digits,
        ),
        (
            preset().with_discounted_cash_flow_years(0),
            "the positive integers",
            "0",
        ),
        (
            preset().with_discounted_cash_flow_terminal_growth_rate("-1"),
            above_minus_one,
            "-1",
        ),
        (
            preset().with_discounted_cash_flow_terminal_growth_rate(thirty_five_digits),
            above_minus_one,
            thirty_five_digits,
        ),
    ];
    for (outcome, domain, value) in refusals {
        let refused = outcome.expect_err("the value is outside the domain");
        assert_eq!((refused.domain(), refused.value()), (domain, value));
    }
}

#[test]
fn a_discount_rate_not_above_the_terminal_growth_rate_is_admitted_by_each_constructor() {
    let settings = Settings::preset()
        .with_discounted_cash_flow_terminal_growth_rate("0.08")
        .expect("a rate above minus one");
    assert!(
        settings
            .with_discounted_cash_flow_discount_rate("0.03")
            .is_ok()
    );
}

#[test]
fn each_period_holds_its_results_and_the_metrics_stated_at_it() {
    let hand_over = handed(None);
    let periods: Vec<&Period> = hand_over.results().iter().map(|at| at.period()).collect();
    assert_eq!(periods, [&year_end(), &fiscal_year()]);

    let names = |at: usize| -> Vec<&str> {
        hand_over.results()[at]
            .metrics()
            .iter()
            .map(|metric| &*metric.name)
            .collect()
    };
    assert_eq!(
        names(0),
        [
            "debt_to_equity",
            "liabilities_to_assets",
            "current_ratio",
            "quick_ratio",
            "cash_ratio"
        ]
    );
    assert!(names(1).contains(&"gross_margin"), "{:?}", names(1));
    assert!(!names(1).contains(&"debt_to_equity"), "{:?}", names(1));
}

#[test]
fn a_period_the_history_repeats_is_still_one_period() {
    let repeated = History::of(CIK.into(), vec![row(year_end()), row(year_end())]);
    let hand_over = analyze(repeated, None, &Settings::preset())
        .expect("a repeated period is not the one error");
    assert_eq!(hand_over.results().len(), 1);
    assert_eq!(hand_over.results()[0].period(), &year_end());
}
