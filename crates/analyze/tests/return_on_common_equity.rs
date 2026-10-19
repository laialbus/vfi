//! Return on common equity as a caller reaches it: a history in, and at each
//! duration period net income less preferred dividends over the average of
//! shareholders' equity less preferred equity at the instants the duration
//! opens and closes at, or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "return_on_common_equity";

const OPENING: &str = "2023-12-31";

const CLOSING: &str = "2024-12-31";

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
    duration("2024-01-01", CLOSING)
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

/// Net income and preferred dividends over `period`.
fn flows(period: Period, net_income: Resolution, preferred_dividends: Resolution) -> Row {
    row(
        period,
        &[
            (Concept::NetIncome, net_income),
            (Concept::PreferredDividends, preferred_dividends),
        ],
    )
}

/// Shareholders' equity and preferred equity at the instant `at`.
fn balances(at: &str, shareholders_equity: Resolution, preferred_equity: Resolution) -> Row {
    row(
        instant(at),
        &[
            (Concept::ShareholdersEquity, shareholders_equity),
            (Concept::PreferredEquity, preferred_equity),
        ],
    )
}

/// Net income and preferred dividends over the year, and shareholders' equity
/// then preferred equity at the instants it opens and closes at.
fn year_of(
    net_income: &str,
    preferred_dividends: &str,
    opening: [&str; 2],
    closing: [&str; 2],
) -> Vec<Row> {
    vec![
        flows(year(), value(net_income), value(preferred_dividends)),
        balances(OPENING, value(opening[0]), value(opening[1])),
        balances(CLOSING, value(closing[0]), value(closing[1])),
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

/// `name`'s outcome over the year, from a history of `rows`.
fn outcome_of(rows: Vec<Row>, name: &str) -> Outcome {
    outcome_at(&handed(rows), &year(), name).expect("the metric is stated at a duration")
}

/// The metric's outcome over the year, from a history of `rows`.
fn outcome(rows: Vec<Row>) -> Outcome {
    outcome_of(rows, NAME)
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

/// The `input_unknown` `concept` gives at `row`, named as the read it was
/// found at.
fn input_unknown(row: &Row, concept: Concept) -> Reason {
    let reason = Reason::input_unknown(row, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        reason.names(),
        &Names::InputUnknown {
            concept,
            period: row.period().clone(),
        }
    );
    reason
}

#[test]
fn income_available_to_common_over_the_average_common_equity_is_its_figure() {
    let rows = year_of("120", "20", ["1000", "200"], ["1000", "200"]);
    assert_eq!(outcome(rows.clone()), figure("0.125"));
    assert_eq!(outcome_of(rows, "return_on_equity"), figure("0.12"));
}

#[test]
fn a_negative_preferred_deduction_raises_the_income_above_net_income() {
    assert_eq!(
        outcome(year_of("120", "-20", ["1000", "200"], ["1000", "200"])),
        figure("0.175")
    );
}

#[test]
fn preferred_concepts_read_as_zero_give_the_total_forms_characters() {
    let rows = year_of("10", "0", ["90", "0"], ["110", "0"]);
    let total = outcome_of(rows.clone(), "return_on_equity");
    assert_eq!(total, figure("0.1"));
    assert_eq!(outcome(rows), total);
}

#[test]
fn shareholders_equity_equal_to_preferred_equity_declines_as_zero_common_equity() {
    assert_eq!(
        outcome(year_of("120", "20", ["200", "200"], ["300", "300"])),
        absent(declined("zero_common_equity"), Vec::new())
    );
}

#[test]
fn a_zero_average_of_common_equity_declines_as_zero_common_equity() {
    assert_eq!(
        outcome(year_of("120", "20", ["300", "200"], ["100", "200"])),
        absent(declined("zero_common_equity"), Vec::new())
    );
}

#[test]
fn a_negative_average_common_equity_is_a_value_that_gives_a_negative_figure() {
    assert_eq!(
        outcome(year_of("100", "0", ["100", "200"], ["100", "400"])),
        figure("-0.5")
    );
}

#[test]
fn an_unknown_preferred_equity_names_each_instant_and_leaves_the_total_form_standing() {
    let opening = balances(OPENING, value("1000"), unknown());
    let closing = balances(CLOSING, value("1000"), unknown());
    let reasons = absent(
        input_unknown(&opening, Concept::PreferredEquity),
        vec![input_unknown(&closing, Concept::PreferredEquity)],
    );
    let rows = vec![flows(year(), value("120"), value("20")), opening, closing];
    assert_eq!(outcome(rows.clone()), reasons);
    assert_eq!(outcome_of(rows, "return_on_equity"), figure("0.12"));
}

#[test]
fn an_unknown_preferred_dividend_names_the_duration() {
    let at = flows(year(), value("120"), unknown());
    let reason = input_unknown(&at, Concept::PreferredDividends);
    let rows = vec![
        at,
        balances(OPENING, value("1000"), value("200")),
        balances(CLOSING, value("1000"), value("200")),
    ];
    assert_eq!(outcome(rows), absent(reason, Vec::new()));
}

#[test]
fn both_preferred_concepts_unknown_give_the_dividend_first() {
    let at = flows(year(), value("120"), unknown());
    let closing = balances(CLOSING, value("1000"), unknown());
    let reasons = absent(
        input_unknown(&at, Concept::PreferredDividends),
        vec![input_unknown(&closing, Concept::PreferredEquity)],
    );
    let rows = vec![at, balances(OPENING, value("1000"), value("200")), closing];
    assert_eq!(outcome(rows), reasons);
}

#[test]
fn an_unknown_shareholders_equity_at_the_closing_instant_names_that_instant() {
    let closing = balances(CLOSING, unknown(), value("200"));
    let reason = input_unknown(&closing, Concept::ShareholdersEquity);
    let rows = vec![
        flows(year(), value("120"), value("20")),
        balances(OPENING, value("1000"), value("200")),
        closing,
    ];
    assert_eq!(outcome(rows), absent(reason, Vec::new()));
}

/// Each balance input is read at the missing row, so each gives its own
/// reason, and the two are not merged.
#[test]
fn a_history_lacking_the_closing_instant_gives_no_period_sought_by_the_closing() {
    let rows = vec![
        flows(year(), value("120"), value("20")),
        balances(OPENING, value("1000"), value("200")),
    ];
    assert_eq!(
        outcome(rows),
        absent(
            no_period("closing_instant"),
            vec![no_period("closing_instant")]
        )
    );
}

#[test]
fn a_history_lacking_the_opening_instant_gives_no_period_sought_by_the_opening() {
    let rows = vec![
        flows(year(), value("120"), value("20")),
        balances(CLOSING, value("1000"), value("200")),
    ];
    assert_eq!(
        outcome(rows),
        absent(
            no_period("opening_instant"),
            vec![no_period("opening_instant")]
        )
    );
}

#[test]
fn a_start_outside_the_grammar_declines_each_opening_read() {
    let odd = duration("2024-1-01", CLOSING);
    let hand_over = handed(vec![
        flows(odd.clone(), value("120"), value("20")),
        balances(OPENING, value("1000"), value("200")),
        balances(CLOSING, value("1000"), value("200")),
    ]);
    assert_eq!(
        outcome_at(&hand_over, &odd, NAME),
        Some(absent(
            declined("start_not_a_date"),
            vec![declined("start_not_a_date")]
        ))
    );
}

#[test]
fn an_instant_period_carries_no_return_on_common_equity() {
    let hand_over = handed(year_of("120", "20", ["1000", "200"], ["1000", "200"]));
    for at in [OPENING, CLOSING] {
        assert_eq!(outcome_at(&hand_over, &instant(at), NAME), None, "{at}");
    }
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(year_of("120", "20", ["1000", "200"], ["1000", "200"]))
        .premises()
        .clone();
    assert_eq!(premises.method_version().get(), 1);
    let recorded: Vec<(&str, &str)> = premises
        .settings()
        .iter()
        .map(|setting| (&*setting.name, &*setting.value))
        .collect();
    assert_eq!(recorded, [("dividend_streak_years", "10")]);
}
