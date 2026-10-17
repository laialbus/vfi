//! Return on assets and asset turnover as a caller reaches them: a history
//! in, and at each duration period each figure over the average of total
//! assets at the instants the duration opens and closes at, or the reasons it
//! has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAMES: [&str; 2] = ["return_on_assets", "asset_turnover"];

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
    duration("2024-01-01", "2024-12-31")
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

/// The year's flows: net income 10 and revenue 50.
fn flows(period: Period) -> Row {
    row(
        period,
        &[
            (Concept::NetIncome, value("10")),
            (Concept::Revenue, value("50")),
        ],
    )
}

fn total_assets(at: &str, resolution: Resolution) -> Row {
    row(instant(at), &[(Concept::TotalAssets, resolution)])
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
fn outcome(rows: Vec<Row>, name: &str) -> Outcome {
    outcome_at(&handed(rows), &year(), name).expect("the metric is stated at a duration")
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

fn no_period(sought_by: &str) -> Reason {
    Reason::no_period(year(), sought_by.into())
}

fn declined(condition: &str) -> Reason {
    Reason::declined(condition.into())
}

#[test]
fn over_the_average_of_the_two_instants_each_is_its_figure() {
    let rows = || {
        vec![
            flows(year()),
            total_assets("2023-12-31", value("90")),
            total_assets("2024-12-31", value("110")),
        ]
    };
    assert_eq!(outcome(rows(), "return_on_assets"), figure("0.1"));
    assert_eq!(outcome(rows(), "asset_turnover"), figure("0.5"));
}

#[test]
fn a_history_lacking_the_closing_instant_gives_no_period_sought_by_the_closing() {
    let rows = || vec![flows(year()), total_assets("2023-12-31", value("90"))];
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(no_period("closing_instant"), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn a_history_lacking_the_opening_instant_gives_no_period_sought_by_the_opening() {
    let rows = || vec![flows(year()), total_assets("2024-12-31", value("110"))];
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(no_period("opening_instant"), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn a_history_lacking_both_instants_gives_both_opening_first() {
    for name in NAMES {
        assert_eq!(
            outcome(vec![flows(year())], name),
            absent(
                no_period("opening_instant"),
                vec![no_period("closing_instant")]
            ),
            "{name}"
        );
    }
}

#[test]
fn no_near_instant_stands_in_for_the_opening() {
    let rows = || {
        vec![
            flows(year()),
            total_assets("2023-09-30", value("90")),
            total_assets("2024-09-30", value("100")),
            total_assets("2024-12-31", value("110")),
        ]
    };
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(no_period("opening_instant"), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn a_duration_ending_at_the_date_does_not_stand_in_for_the_closing_instant() {
    let rows = || {
        vec![
            flows(year()),
            row(
                duration("2024-10-01", "2024-12-31"),
                &[(Concept::TotalAssets, value("110"))],
            ),
            total_assets("2023-12-31", value("90")),
        ]
    };
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(no_period("closing_instant"), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn an_unknown_balance_at_the_sought_instant_names_that_instant_and_not_the_duration() {
    let closing = total_assets("2024-12-31", unknown());
    let reason = Reason::input_unknown(&closing, Concept::TotalAssets)
        .expect("total assets are Unknown at the instant");
    assert_eq!(
        reason.names(),
        &Names::InputUnknown {
            concept: Concept::TotalAssets,
            period: instant("2024-12-31"),
        }
    );
    let rows = || {
        vec![
            flows(year()),
            total_assets("2023-12-31", value("90")),
            closing.clone(),
        ]
    };
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(reason.clone(), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn every_read_is_made_and_the_reasons_follow_the_read_order() {
    let computing = row(
        year(),
        &[
            (Concept::NetIncome, unknown()),
            (Concept::Revenue, unknown()),
        ],
    );
    let closing = total_assets("2024-12-31", unknown());
    let rows = vec![computing.clone(), closing.clone()];
    let hand_over = handed(rows);
    for (name, flow) in NAMES
        .into_iter()
        .zip([Concept::NetIncome, Concept::Revenue])
    {
        assert_eq!(
            outcome_at(&hand_over, &year(), name),
            Some(absent(
                Reason::input_unknown(&computing, flow).expect("the flow is Unknown"),
                vec![
                    no_period("opening_instant"),
                    Reason::input_unknown(&closing, Concept::TotalAssets)
                        .expect("total assets are Unknown at the closing instant"),
                ]
            )),
            "{name}"
        );
    }
}

#[test]
fn a_balance_outside_the_decimal_grammar_is_declined_at_its_read() {
    let rows = || {
        vec![
            flows(year()),
            total_assets("2023-12-31", value("9e1")),
            total_assets("2024-12-31", value("1,100")),
        ]
    };
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(
                declined("input_not_a_decimal"),
                vec![declined("input_not_a_decimal")]
            ),
            "{name}"
        );
    }
}

#[test]
fn a_start_outside_the_grammar_declines_the_opening_read() {
    let odd = duration("2024-1-01", "2024-12-31");
    let hand_over = handed(vec![
        flows(odd.clone()),
        total_assets("2023-12-31", value("90")),
        total_assets("2024-12-31", value("110")),
    ]);
    for name in NAMES {
        assert_eq!(
            outcome_at(&hand_over, &odd, name),
            Some(absent(declined("start_not_a_date"), Vec::new())),
            "{name}"
        );
    }
}

#[test]
fn a_transition_period_opens_at_the_day_before_it_starts() {
    let transition = duration("2023-05-01", "2023-12-31");
    let hand_over = handed(vec![
        flows(transition.clone()),
        total_assets("2023-04-30", value("90")),
        total_assets("2023-12-31", value("110")),
    ]);
    assert_eq!(
        outcome_at(&hand_over, &transition, "return_on_assets"),
        Some(figure("0.1"))
    );
    assert_eq!(
        outcome_at(&hand_over, &transition, "asset_turnover"),
        Some(figure("0.5"))
    );
}

#[test]
fn a_zero_average_declines_each_metric_under_its_own_condition() {
    let rows = || {
        vec![
            flows(year()),
            total_assets("2023-12-31", value("-10")),
            total_assets("2024-12-31", value("10")),
        ]
    };
    for name in NAMES {
        assert_eq!(
            outcome(rows(), name),
            absent(declined("zero_average_total_assets"), Vec::new()),
            "{name}"
        );
    }
}

#[test]
fn a_history_of_instants_alone_states_neither() {
    let hand_over = handed(vec![
        total_assets("2023-12-31", value("90")),
        total_assets("2024-12-31", value("110")),
    ]);
    for at in ["2023-12-31", "2024-12-31"] {
        for name in NAMES {
            assert_eq!(outcome_at(&hand_over, &instant(at), name), None, "{name}");
        }
    }
}

#[test]
fn a_banks_rows_compute_both() {
    let bank = |period: Period, given: &[(Concept, Resolution)]| {
        Row::at(
            period,
            std::array::from_fn(|at| {
                let concept = Concept::ALL[at];
                given
                    .iter()
                    .find(|(given, _)| *given == concept)
                    .map(|(_, resolution)| resolution.clone())
                    .or_else(|| {
                        concept
                            .definition()
                            .applies_to
                            .excluding(Kind::Bank)
                            .map(|excluded| Resolution::NotApplicable { excluded })
                    })
                    .unwrap_or_else(|| value("7"))
            }),
        )
    };
    let rows = || {
        vec![
            bank(
                year(),
                &[
                    (Concept::NetIncome, value("10")),
                    (Concept::Revenue, value("50")),
                ],
            ),
            bank(
                instant("2023-12-31"),
                &[(Concept::TotalAssets, value("90"))],
            ),
            bank(
                instant("2024-12-31"),
                &[(Concept::TotalAssets, value("110"))],
            ),
        ]
    };
    assert_eq!(outcome(rows(), "return_on_assets"), figure("0.1"));
    assert_eq!(outcome(rows(), "asset_turnover"), figure("0.5"));
}

#[test]
fn a_net_loss_gives_a_negative_return() {
    let rows = vec![
        row(year(), &[(Concept::NetIncome, value("-5"))]),
        total_assets("2023-12-31", value("90")),
        total_assets("2024-12-31", value("110")),
    ];
    assert_eq!(outcome(rows, "return_on_assets"), figure("-0.05"));
}

#[test]
fn adding_the_metrics_adds_no_setting_and_keeps_the_version() {
    let premises = handed(vec![flows(year())]).premises().clone();
    assert_eq!(premises.method_version().get(), 1);
    assert!(premises.settings().is_empty(), "{:?}", premises.settings());
}
