//! Return on equity as a caller reaches it: a history in, and at each duration
//! period net income over the average of shareholders' equity at the instants
//! the duration opens and closes at, or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "return_on_equity";

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
/// a value the metric does not read as one of these.
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

fn net_income(period: Period, amount: &str) -> Row {
    row(period, &[(Concept::NetIncome, value(amount))])
}

fn shareholders_equity(at: &str, resolution: Resolution) -> Row {
    row(instant(at), &[(Concept::ShareholdersEquity, resolution)])
}

/// Net income `income` over the year, and shareholders' equity `opening` and
/// `closing` at the instants it opens and closes at.
fn year_of(income: &str, opening: &str, closing: &str) -> Vec<Row> {
    vec![
        net_income(year(), income),
        shareholders_equity("2023-12-31", value(opening)),
        shareholders_equity("2024-12-31", value(closing)),
    ]
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

/// The metric's outcome over the year, from a history of `rows`.
fn outcome(rows: Vec<Row>) -> Outcome {
    outcome_at(&handed(rows), &year(), NAME).expect("the metric is stated at a duration")
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
fn over_the_average_of_the_two_instants_it_is_its_figure() {
    assert_eq!(outcome(year_of("10", "90", "110")), figure("0.1"));
}

#[test]
fn a_negative_average_equity_is_a_value_that_gives_a_negative_figure() {
    assert_eq!(outcome(year_of("10", "-40", "-60")), figure("-0.2"));
}

#[test]
fn a_net_loss_gives_a_negative_return() {
    assert_eq!(outcome(year_of("-5", "90", "110")), figure("-0.05"));
}

#[test]
fn a_history_lacking_the_closing_instant_gives_no_period_sought_by_the_closing() {
    let rows = vec![
        net_income(year(), "10"),
        shareholders_equity("2023-12-31", value("90")),
    ];
    assert_eq!(
        outcome(rows),
        absent(no_period("closing_instant"), Vec::new())
    );
}

#[test]
fn a_history_lacking_the_opening_instant_gives_no_period_sought_by_the_opening() {
    let rows = vec![
        net_income(year(), "10"),
        shareholders_equity("2024-12-31", value("110")),
    ];
    assert_eq!(
        outcome(rows),
        absent(no_period("opening_instant"), Vec::new())
    );
}

#[test]
fn an_unknown_equity_at_the_closing_instant_names_that_instant_and_not_the_duration() {
    let closing = shareholders_equity("2024-12-31", unknown());
    let reason = Reason::input_unknown(&closing, Concept::ShareholdersEquity)
        .expect("shareholders' equity is Unknown at the instant");
    assert_eq!(
        reason.names(),
        &Names::InputUnknown {
            concept: Concept::ShareholdersEquity,
            period: instant("2024-12-31"),
        }
    );
    let rows = vec![
        net_income(year(), "10"),
        shareholders_equity("2023-12-31", value("90")),
        closing,
    ];
    assert_eq!(outcome(rows), absent(reason, Vec::new()));
}

#[test]
fn a_zero_average_declines_under_the_entrys_own_condition() {
    assert_eq!(
        outcome(year_of("10", "10", "-10")),
        absent(declined("zero_average_shareholders_equity"), Vec::new())
    );
}

#[test]
fn the_preferred_terms_are_not_read() {
    let preferred_unknown = |period: Period, given: (Concept, Resolution)| {
        row(
            period,
            &[
                given,
                (Concept::PreferredEquity, unknown()),
                (Concept::PreferredDividends, unknown()),
            ],
        )
    };
    let rows = vec![
        preferred_unknown(year(), (Concept::NetIncome, value("10"))),
        preferred_unknown(
            instant("2023-12-31"),
            (Concept::ShareholdersEquity, value("90")),
        ),
        preferred_unknown(
            instant("2024-12-31"),
            (Concept::ShareholdersEquity, value("110")),
        ),
    ];
    assert_eq!(outcome(rows), figure("0.1"));
}

#[test]
fn a_start_outside_the_grammar_declines_the_opening_read() {
    let odd = duration("2024-1-01", "2024-12-31");
    let hand_over = handed(vec![
        net_income(odd.clone(), "10"),
        shareholders_equity("2023-12-31", value("90")),
        shareholders_equity("2024-12-31", value("110")),
    ]);
    assert_eq!(
        outcome_at(&hand_over, &odd, NAME),
        Some(absent(declined("start_not_a_date"), Vec::new()))
    );
}

#[test]
fn an_instant_period_carries_no_return_on_equity() {
    let hand_over = handed(year_of("10", "90", "110"));
    for at in ["2023-12-31", "2024-12-31"] {
        assert_eq!(outcome_at(&hand_over, &instant(at), NAME), None, "{at}");
    }
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(year_of("10", "90", "110")).premises().clone();
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
