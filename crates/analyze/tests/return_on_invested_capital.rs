//! Return on invested capital as a caller reaches it: a history in, and at
//! each duration period operating income times one less the effective tax
//! rate over the average of shareholders' equity plus net debt at the instants
//! the duration opens and closes at, or the reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "return_on_invested_capital";

const OPENING: &str = "2023-12-31";

const CLOSING: &str = "2024-12-31";

/// The three flows, in input order.
const FLOWS: [Concept; 3] = [
    Concept::OperatingIncome,
    Concept::IncomeTaxExpense,
    Concept::PretaxIncome,
];

/// The five balances, in input order.
const BALANCES: [Concept; 5] = [
    Concept::ShareholdersEquity,
    Concept::ShortTermDebt,
    Concept::LongTermDebt,
    Concept::CashAndEquivalents,
    Concept::ShortTermInvestments,
];

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

/// `concept` as a filer of `kind` states it where the kind is excluded from
/// it.
fn not_applicable(concept: Concept, kind: Kind) -> Resolution {
    Resolution::NotApplicable {
        excluded: concept
            .definition()
            .applies_to
            .excluding(kind)
            .expect("the kind is excluded from the concept"),
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

/// The year's row with operating income, income tax expense and pretax income
/// as stated.
fn flows(period: Period, stated: [Resolution; 3]) -> Row {
    row(period, &FLOWS.into_iter().zip(stated).collect::<Vec<_>>())
}

/// The instant `at` with the five balances as stated, in input order.
fn balances(at: &str, stated: [Resolution; 5]) -> Row {
    row(
        instant(at),
        &BALANCES.into_iter().zip(stated).collect::<Vec<_>>(),
    )
}

fn values<const N: usize>(amounts: [&str; N]) -> [Resolution; N] {
    amounts.map(value)
}

/// Operating income 120, income tax expense 25 and pretax income 100, so a
/// rate of 0.25 and NOPAT of 90.
fn worked_flows() -> [Resolution; 3] {
    values(["120", "25", "100"])
}

/// Invested capital 700 + 50 + 150 - 40 - 100 = 760.
fn worked_opening() -> [Resolution; 5] {
    values(["700", "50", "150", "40", "100"])
}

/// Invested capital 900 + 50 + 250 - 60 - 100 = 1040.
fn worked_closing() -> [Resolution; 5] {
    values(["900", "50", "250", "60", "100"])
}

fn year_of(
    flowing: [Resolution; 3],
    opening: [Resolution; 5],
    closing: [Resolution; 5],
) -> Vec<Row> {
    vec![
        flows(year(), flowing),
        balances(OPENING, opening),
        balances(CLOSING, closing),
    ]
}

/// The worked row with the flows as stated.
fn worked_with_flows(flowing: [&str; 3]) -> Vec<Row> {
    year_of(values(flowing), worked_opening(), worked_closing())
}

/// The worked row at a filer of `kind`: each concept either metric over
/// capital reads that the kind is excluded from is `NotApplicable` at every
/// row, and every other as the worked row states it.
fn worked_of_kind(kind: Kind) -> Vec<Row> {
    let excluded = |concept: Concept| concept.definition().applies_to.excluding(kind).is_some();
    let of_kind = |concepts: &[Concept], stated: &[Resolution]| -> Vec<(Concept, Resolution)> {
        concepts
            .iter()
            .zip(stated)
            .map(|(concept, resolution)| match excluded(*concept) {
                true => (*concept, not_applicable(*concept, kind)),
                false => (*concept, resolution.clone()),
            })
            .chain(excluded(Concept::CurrentLiabilities).then(|| {
                (
                    Concept::CurrentLiabilities,
                    not_applicable(Concept::CurrentLiabilities, kind),
                )
            }))
            .collect()
    };
    vec![
        row(year(), &of_kind(&FLOWS, &worked_flows())),
        row(instant(OPENING), &of_kind(&BALANCES, &worked_opening())),
        row(instant(CLOSING), &of_kind(&BALANCES, &worked_closing())),
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

fn declined(condition: &str) -> Outcome {
    absent(Reason::declined(condition.into()), Vec::new())
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

/// NOPAT 90 over invested capital 760 then 1040, whose average is 900.
#[test]
fn nopat_over_the_average_invested_capital_is_its_figure() {
    assert_eq!(
        outcome(year_of(worked_flows(), worked_opening(), worked_closing())),
        figure("0.1")
    );
}

#[test]
fn a_zero_pretax_income_declines_as_non_positive_pretax_income() {
    assert_eq!(
        outcome(worked_with_flows(["120", "25", "0"])),
        declined("non_positive_pretax_income")
    );
}

/// The quotient would be 0.25, the share of a loss recovered, which is no
/// charge on a profit.
#[test]
fn a_pretax_loss_declines_as_non_positive_pretax_income_though_the_quotient_is_in_bounds() {
    assert_eq!(
        outcome(worked_with_flows(["120", "-25", "-100"])),
        declined("non_positive_pretax_income")
    );
}

#[test]
fn a_tax_benefit_over_a_pretax_profit_declines_as_a_rate_below_zero() {
    assert_eq!(
        outcome(worked_with_flows(["120", "-10", "100"])),
        declined("tax_rate_outside_zero_to_one")
    );
}

#[test]
fn a_tax_above_pretax_income_declines_as_a_rate_above_one() {
    assert_eq!(
        outcome(worked_with_flows(["120", "110", "100"])),
        declined("tax_rate_outside_zero_to_one")
    );
}

#[test]
fn a_rate_of_one_is_inside_the_bounds_and_gives_zero() {
    assert_eq!(
        outcome(worked_with_flows(["120", "100", "100"])),
        figure("0")
    );
}

/// Invested capital is 100 + 0 + 50 - 100 - 50 = 0 at both instants.
fn zero_invested_capital() -> [Resolution; 5] {
    values(["100", "0", "50", "100", "50"])
}

#[test]
fn a_zero_average_invested_capital_declines_as_zero_invested_capital() {
    assert_eq!(
        outcome(year_of(
            worked_flows(),
            zero_invested_capital(),
            zero_invested_capital()
        )),
        declined("zero_invested_capital")
    );
}

/// The rate's conditions are asked before the divisor's, and the first that
/// holds is the one reason.
#[test]
fn a_zero_pretax_income_over_a_zero_invested_capital_gives_non_positive_pretax_income_alone() {
    assert_eq!(
        outcome(year_of(
            values(["120", "25", "0"]),
            zero_invested_capital(),
            zero_invested_capital()
        )),
        declined("non_positive_pretax_income")
    );
}

/// The worked balances with short-term investments as stated at each
/// instant.
fn worked_with_short_term_investments(
    opening: Resolution,
    closing: Resolution,
) -> [[Resolution; 5]; 2] {
    let [mut at_opening, mut at_closing] = [worked_opening(), worked_closing()];
    at_opening[4] = opening;
    at_closing[4] = closing;
    [at_opening, at_closing]
}

fn reit(concept: Concept) -> Resolution {
    not_applicable(concept, Kind::Reit)
}

/// Invested capital 860 then 1140, whose average is 1000.
#[test]
fn short_term_investments_not_applicable_at_both_instants_contribute_nothing() {
    let [opening, closing] = worked_with_short_term_investments(
        reit(Concept::ShortTermInvestments),
        reit(Concept::ShortTermInvestments),
    );
    assert_eq!(
        outcome(year_of(worked_flows(), opening.clone(), closing.clone())),
        figure("0.09")
    );
    assert_eq!(
        outcome(year_of(values(["120", "0", "100"]), opening, closing)),
        figure("0.12")
    );
}

/// Invested capital 860 then 940, whose average is 900. Counting only the
/// instant that held a value would give 940 alone, and 0.1125.
#[test]
fn an_instant_where_short_term_investments_contribute_nothing_is_still_counted() {
    let [opening, closing] =
        worked_with_short_term_investments(reit(Concept::ShortTermInvestments), value("200"));
    assert_eq!(
        outcome(year_of(worked_flows(), opening, closing)),
        figure("0.1")
    );
}

#[test]
fn an_unknown_short_term_investments_names_it_at_its_instant() {
    let [opening, closing] = worked_with_short_term_investments(value("100"), unknown());
    let closing = balances(CLOSING, closing);
    let reason = input_unknown(&closing, Concept::ShortTermInvestments);
    let rows = vec![
        flows(year(), worked_flows()),
        balances(OPENING, opening),
        closing,
    ];
    assert_eq!(outcome(rows), absent(reason, Vec::new()));
}

/// Every read is made and none stops another: the tax at the duration, then
/// each balance at the missing opening, in input order.
#[test]
fn an_unknown_tax_and_a_missing_opening_give_six_reasons_in_read_order() {
    let year_row = flows(year(), [value("120"), unknown(), value("100")]);
    let reason = input_unknown(&year_row, Concept::IncomeTaxExpense);
    let rows = vec![year_row, balances(CLOSING, worked_closing())];
    assert_eq!(
        outcome(rows),
        absent(
            reason,
            BALANCES.map(|_| no_period("opening_instant")).to_vec()
        )
    );
}

/// Short-term investments contribute nothing for a bank, and operating
/// income never does; current liabilities are read by capital employed alone.
#[test]
fn a_banks_row_gives_operating_income_alone_where_capital_employed_gives_three() {
    let rows = worked_of_kind(Kind::Bank);
    assert_eq!(
        outcome(rows.clone()),
        absent(
            input_not_applicable(&rows, &year(), Concept::OperatingIncome),
            Vec::new()
        )
    );
    assert_eq!(
        outcome_of(rows.clone(), "return_on_capital_employed"),
        absent(
            input_not_applicable(&rows, &year(), Concept::OperatingIncome),
            vec![
                input_not_applicable(&rows, &instant(OPENING), Concept::CurrentLiabilities),
                input_not_applicable(&rows, &instant(CLOSING), Concept::CurrentLiabilities),
            ]
        )
    );
}

#[test]
fn a_reits_row_gives_a_figure_where_capital_employed_gives_two() {
    let rows = worked_of_kind(Kind::Reit);
    assert_eq!(outcome(rows.clone()), figure("0.09"));
    assert_eq!(
        outcome_of(rows.clone(), "return_on_capital_employed"),
        absent(
            input_not_applicable(&rows, &instant(OPENING), Concept::CurrentLiabilities),
            vec![input_not_applicable(
                &rows,
                &instant(CLOSING),
                Concept::CurrentLiabilities
            )]
        )
    );
}

#[test]
fn a_start_outside_the_grammar_declines_each_opening_read() {
    let odd = duration("2024-1-01", CLOSING);
    let hand_over = handed(vec![
        flows(odd.clone(), worked_flows()),
        balances(OPENING, worked_opening()),
        balances(CLOSING, worked_closing()),
    ]);
    let start_not_a_date = || Reason::declined("start_not_a_date".into());
    assert_eq!(
        outcome_at(&hand_over, &odd, NAME),
        Some(absent(
            start_not_a_date(),
            BALANCES[1..].iter().map(|_| start_not_a_date()).collect()
        ))
    );
}

#[test]
fn an_instant_period_carries_no_return_on_invested_capital() {
    let hand_over = handed(year_of(worked_flows(), worked_opening(), worked_closing()));
    for at in [OPENING, CLOSING] {
        assert_eq!(outcome_at(&hand_over, &instant(at), NAME), None, "{at}");
    }
}

#[test]
fn adding_the_metric_adds_no_setting_and_keeps_the_version() {
    let premises = handed(year_of(worked_flows(), worked_opening(), worked_closing()))
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
            ("dividend_growth_years", "5"),
        ]
    );
}
