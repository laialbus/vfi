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
fn with_no_setting_defined_the_premises_are_the_version_alone() {
    let premises = handed(None).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    assert!(
        premises.settings().is_empty(),
        "settings were recorded that Settings does not define: {:?}",
        premises.settings()
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
    assert!(
        names(0).is_empty(),
        "a metric was computed at an instant, and none is stated at one: {:?}",
        names(0)
    );
    assert!(names(1).contains(&"gross_margin"), "{:?}", names(1));
}

#[test]
fn a_period_the_history_repeats_is_still_one_period() {
    let repeated = History::of(CIK.into(), vec![row(year_end()), row(year_end())]);
    let hand_over = analyze(repeated, None, &Settings::preset())
        .expect("a repeated period is not the one error");
    assert_eq!(hand_over.results().len(), 1);
    assert_eq!(hand_over.results()[0].period(), &year_end());
}
