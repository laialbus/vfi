//! Return on capital employed as a caller reaches it: a history in, and at
//! each duration period operating income over the average of total assets
//! less current liabilities at the instants the duration opens and closes at,
//! or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "return_on_capital_employed";

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

/// Operating income over `period`.
fn flows(period: Period, operating_income: Resolution) -> Row {
    row(period, &[(Concept::OperatingIncome, operating_income)])
}

/// Total assets and current liabilities at the instant `at`.
fn balances(at: &str, total_assets: Resolution, current_liabilities: Resolution) -> Row {
    row(
        instant(at),
        &[
            (Concept::TotalAssets, total_assets),
            (Concept::CurrentLiabilities, current_liabilities),
        ],
    )
}

/// Operating income over the year, and total assets then current liabilities
/// at the instants it opens and closes at.
fn year_of(operating_income: &str, opening: [&str; 2], closing: [&str; 2]) -> Vec<Row> {
    vec![
        flows(year(), value(operating_income)),
        balances(OPENING, value(opening[0]), value(opening[1])),
        balances(CLOSING, value(closing[0]), value(closing[1])),
    ]
}

/// The year at a filer of `kind`, where each of the metric's inputs the kind
/// is excluded from is `NotApplicable` at every row, and every other concept
/// a value.
fn year_of_kind(kind: Kind) -> Vec<Row> {
    let given: Vec<(Concept, Resolution)> = [
        Concept::OperatingIncome,
        Concept::TotalAssets,
        Concept::CurrentLiabilities,
    ]
    .into_iter()
    .filter_map(|concept| {
        let excluded = concept.definition().applies_to.excluding(kind)?;
        Some((concept, Resolution::NotApplicable { excluded }))
    })
    .collect();
    [year(), instant(OPENING), instant(CLOSING)]
        .into_iter()
        .map(|period| row(period, &given))
        .collect()
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

/// The `input_not_applicable` `concept` gives at `period`, one of `rows`'.
fn input_not_applicable(rows: &[Row], period: &Period, concept: Concept) -> Reason {
    let row = rows
        .iter()
        .find(|row| row.period() == period)
        .expect("the history holds the period");
    let reason = Reason::input_not_applicable(row, concept)
        .expect("the concept is NotApplicable at the row");
    assert_eq!(
        reason.names(),
        &Names::InputNotApplicable {
            concept,
            period: period.clone(),
        }
    );
    reason
}

/// Capital employed is 800 then 1000, whose average 900 is also the average
/// total assets 1200 less the average current liabilities 300.
#[test]
fn operating_income_over_the_average_capital_employed_is_its_figure() {
    assert_eq!(
        outcome(year_of("135", ["1000", "200"], ["1400", "400"])),
        figure("0.15")
    );
}

#[test]
fn a_negative_average_capital_employed_is_a_value_that_gives_a_negative_figure() {
    assert_eq!(
        outcome(year_of("135", ["300", "400"], ["300", "500"])),
        figure("-0.9")
    );
}

#[test]
fn total_assets_equal_to_current_liabilities_declines_as_zero_capital_employed() {
    assert_eq!(
        outcome(year_of("135", ["400", "400"], ["900", "900"])),
        absent(declined("zero_capital_employed"), Vec::new())
    );
}

/// Capital employed is 100 then -100: the condition is on the average.
#[test]
fn a_zero_average_of_capital_employed_declines_as_zero_capital_employed() {
    assert_eq!(
        outcome(year_of("135", ["600", "500"], ["400", "500"])),
        absent(declined("zero_capital_employed"), Vec::new())
    );
}

/// Each balance input is read at the missing row, so each gives its own
/// reason, and the two are not merged.
#[test]
fn a_history_lacking_the_closing_instant_gives_no_period_sought_by_the_closing() {
    let rows = vec![
        flows(year(), value("135")),
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
        flows(year(), value("135")),
        balances(CLOSING, value("1400"), value("400")),
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
fn an_unknown_total_assets_at_the_closing_instant_names_that_instant() {
    let closing = balances(CLOSING, unknown(), value("400"));
    let reason = input_unknown(&closing, Concept::TotalAssets);
    let rows = vec![
        flows(year(), value("135")),
        balances(OPENING, value("1000"), value("200")),
        closing,
    ];
    assert_eq!(outcome(rows), absent(reason, Vec::new()));
}

/// Neither operating income nor current liabilities contributes nothing: a
/// bank's capital employed is never its total assets.
#[test]
fn a_banks_row_gives_operating_income_then_current_liabilities_at_each_instant() {
    let rows = year_of_kind(Kind::Bank);
    let reasons = absent(
        input_not_applicable(&rows, &year(), Concept::OperatingIncome),
        vec![
            input_not_applicable(&rows, &instant(OPENING), Concept::CurrentLiabilities),
            input_not_applicable(&rows, &instant(CLOSING), Concept::CurrentLiabilities),
        ],
    );
    assert_eq!(outcome(rows), reasons);
}

#[test]
fn a_reits_row_gives_current_liabilities_at_each_instant_alone() {
    let rows = year_of_kind(Kind::Reit);
    let reasons = absent(
        input_not_applicable(&rows, &instant(OPENING), Concept::CurrentLiabilities),
        vec![input_not_applicable(
            &rows,
            &instant(CLOSING),
            Concept::CurrentLiabilities,
        )],
    );
    assert_eq!(outcome(rows), reasons);
}

#[test]
fn a_start_outside_the_grammar_declines_each_opening_read() {
    let odd = duration("2024-1-01", CLOSING);
    let hand_over = handed(vec![
        flows(odd.clone(), value("135")),
        balances(OPENING, value("1000"), value("200")),
        balances(CLOSING, value("1400"), value("400")),
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
fn an_instant_period_carries_no_return_on_capital_employed() {
    let hand_over = handed(year_of("135", ["1000", "200"], ["1400", "400"]));
    for at in [OPENING, CLOSING] {
        assert_eq!(outcome_at(&hand_over, &instant(at), NAME), None, "{at}");
    }
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(year_of("135", ["1000", "200"], ["1400", "400"]))
        .premises()
        .clone();
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
        ]
    );
}
