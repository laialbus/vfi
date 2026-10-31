//! The discounted cash flow value per share as a caller reaches it: a history
//! in, and at each year free cash flow grown over the forecast and then for
//! ever, discounted, less the average of net debt, per diluted share; or the
//! reasons it has none.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{HandOver, Names, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};

const CIK: &str = "0000320193";

const NAME: &str = "discounted_cash_flow_value_per_share";

const OPENING: &str = "2023-12-31";

const CLOSING: &str = "2024-12-31";

/// The four balances of net debt, in input order.
const BALANCES: [Concept; 4] = [
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

/// Short-term investments as a filer of `kind` states them, which the kind is
/// excluded from.
fn short_term_investments_not_applicable(kind: Kind) -> Resolution {
    Resolution::NotApplicable {
        excluded: Concept::ShortTermInvestments
            .definition()
            .applies_to
            .excluding(kind)
            .expect("the kind is excluded from short-term investments"),
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

/// What a year's history states: the three flows at the year, and the four
/// balances at its opening and its closing instant, each in input order.
#[derive(Clone)]
struct Stated {
    operating_cash_flow: Resolution,
    capital_expenditure: Resolution,
    opening: [Resolution; 4],
    closing: [Resolution; 4],
    diluted_shares_weighted_average: Resolution,
}

impl Stated {
    /// The year's row and both instants' rows.
    fn rows(&self) -> Vec<Row> {
        let mut rows = self.year_and_closing();
        rows.push(self.balances(OPENING, &self.opening));
        rows
    }

    /// The year's row and the closing instant's, with no opening instant.
    fn year_and_closing(&self) -> Vec<Row> {
        vec![
            row(
                year(),
                &[
                    (Concept::OperatingCashFlow, self.operating_cash_flow.clone()),
                    (
                        Concept::CapitalExpenditure,
                        self.capital_expenditure.clone(),
                    ),
                    (
                        Concept::DilutedSharesWeightedAverage,
                        self.diluted_shares_weighted_average.clone(),
                    ),
                ],
            ),
            self.balances(CLOSING, &self.closing),
        ]
    }

    fn balances(&self, at: &str, stated: &[Resolution; 4]) -> Row {
        row(
            instant(at),
            &BALANCES.into_iter().zip(stated.clone()).collect::<Vec<_>>(),
        )
    }
}

/// Free cash flow 572, net debt 600 at both instants, 120 diluted shares.
fn preset_row() -> Stated {
    Stated {
        operating_cash_flow: value("800"),
        capital_expenditure: value("228"),
        opening: ["100", "900", "300", "100"].map(value),
        closing: ["100", "1100", "500", "100"].map(value),
        diluted_shares_weighted_average: value("120"),
    }
}

/// Free cash flow 100, net debt 200 at both instants, 10 diluted shares.
fn off_preset_row() -> Stated {
    Stated {
        operating_cash_flow: value("150"),
        capital_expenditure: value("50"),
        opening: ["50", "300", "100", "50"].map(value),
        closing: ["50", "300", "100", "50"].map(value),
        diluted_shares_weighted_average: value("10"),
    }
}

/// A discount rate of 0.1, a growth rate of 0.1, two years and a terminal
/// growth rate of 0.
fn off_preset() -> Settings {
    Settings::preset()
        .with_discounted_cash_flow_discount_rate("0.1")
        .and_then(|settings| settings.with_discounted_cash_flow_growth_rate("0.1"))
        .and_then(|settings| settings.with_discounted_cash_flow_years(2))
        .and_then(|settings| settings.with_discounted_cash_flow_terminal_growth_rate("0"))
        .expect("each is inside its domain")
}

fn handed(rows: Vec<Row>, settings: &Settings) -> HandOver {
    analyze(History::of(CIK.into(), rows), None, settings).expect("no crossing is handed")
}

/// The metric's outcome at `period`, or none where the results there hold no
/// metric of its name.
fn outcome_at(hand_over: &HandOver, period: &Period) -> Option<Outcome> {
    hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics()
        .iter()
        .find(|metric| &*metric.name == NAME)
        .map(|metric| metric.outcome.clone())
}

/// The metric's outcome over the year, from a history of `rows`, under
/// `settings`.
fn outcome(rows: Vec<Row>, settings: &Settings) -> Outcome {
    outcome_at(&handed(rows, settings), &year()).expect("the metric is stated at a duration")
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

/// 572 × 1.02 ÷ 0.0572 = 10200, less 600, over 120.
#[test]
fn the_preset_row_under_the_presets_is_eighty_at_any_forecast_years() {
    assert_eq!(
        outcome(preset_row().rows(), &Settings::preset()),
        figure("80")
    );
    for years in [1, 10] {
        let settings = Settings::preset()
            .with_discounted_cash_flow_years(years)
            .expect("a positive integer");
        assert_eq!(
            outcome(preset_row().rows(), &settings),
            figure("80"),
            "{years}"
        );
    }
}

/// 110 and 121 each discount to 100; 121 ÷ 0.1 = 1210 discounts to 1000; 1200
/// less 200, over 10.
#[test]
fn the_off_preset_row_is_worked_by_hand_to_one_hundred() {
    assert_eq!(
        outcome(off_preset_row().rows(), &off_preset()),
        figure("100")
    );
}

#[test]
fn short_term_investments_not_applicable_at_both_instants_contribute_nothing() {
    let reit = short_term_investments_not_applicable(Kind::Reit);
    let mut stated = off_preset_row();
    stated.opening[3] = reit.clone();
    stated.closing[3] = reit;
    assert_eq!(outcome(stated.rows(), &off_preset()), figure("95"));
}

/// Net debt 250 at the opening and 200 at the closing: the instant where
/// short-term investments contribute nothing is still counted.
#[test]
fn short_term_investments_not_applicable_at_one_instant_is_averaged_over_two() {
    let mut stated = off_preset_row();
    stated.opening[3] = short_term_investments_not_applicable(Kind::Reit);
    let outcome = outcome(stated.rows(), &off_preset());
    assert_eq!(outcome, figure("97.5"));
    assert_ne!(outcome, figure("100"));
}

#[test]
fn short_term_investments_unknown_at_the_closing_is_absent_naming_it_there() {
    let mut stated = off_preset_row();
    stated.closing[3] = unknown();
    let rows = stated.rows();
    let closing = rows
        .iter()
        .find(|row| *row.period() == instant(CLOSING))
        .expect("the closing instant");
    let reason = input_unknown(closing, Concept::ShortTermInvestments);
    assert_eq!(
        outcome(rows.clone(), &off_preset()),
        absent(reason, Vec::new())
    );
}

/// Net debt 1200 equals the value of operations, then 1300 exceeds it by 100.
#[test]
fn an_equity_value_of_zero_or_below_is_a_figure_with_its_sign() {
    for (long_term_debt, value_per_share) in [("1300", "0"), ("1400", "-10")] {
        let mut stated = off_preset_row();
        stated.opening[1] = value(long_term_debt);
        stated.closing[1] = value(long_term_debt);
        assert_eq!(
            outcome(stated.rows(), &off_preset()),
            figure(value_per_share),
            "{long_term_debt}"
        );
    }
}

#[test]
fn a_free_cash_flow_of_zero_or_below_declines() {
    for capital_expenditure in ["800", "900"] {
        let stated = Stated {
            capital_expenditure: value(capital_expenditure),
            ..preset_row()
        };
        assert_eq!(
            outcome(stated.rows(), &Settings::preset()),
            declined("non_positive_free_cash_flow"),
            "{capital_expenditure}"
        );
    }
}

#[test]
fn a_terminal_growth_rate_at_or_above_the_discount_rate_declines() {
    for terminal_growth_rate in ["0.0772", "0.08"] {
        let settings = Settings::preset()
            .with_discounted_cash_flow_terminal_growth_rate(terminal_growth_rate)
            .expect("a rate above minus one");
        assert_eq!(
            outcome(preset_row().rows(), &settings),
            declined("discount_rate_not_above_terminal_growth"),
            "{terminal_growth_rate}"
        );
    }
}

#[test]
fn zero_diluted_shares_decline() {
    let stated = Stated {
        diluted_shares_weighted_average: value("0"),
        ..preset_row()
    };
    assert_eq!(
        outcome(stated.rows(), &Settings::preset()),
        declined("non_positive_diluted_shares")
    );
}

/// Every read is made and none stops another: the flows, then each balance
/// of net debt at the opening it cannot find, then the share count.
#[test]
fn every_read_gives_its_reason_in_read_order() {
    let stated = Stated {
        capital_expenditure: unknown(),
        diluted_shares_weighted_average: unknown(),
        ..preset_row()
    };
    let rows = stated.year_and_closing();
    let no_opening = || Reason::no_period(year(), "opening_instant".into());
    assert_eq!(
        outcome(rows.clone(), &Settings::preset()),
        absent(
            input_unknown(&rows[0], Concept::CapitalExpenditure),
            vec![
                no_opening(),
                no_opening(),
                no_opening(),
                no_opening(),
                input_unknown(&rows[0], Concept::DilutedSharesWeightedAverage),
            ]
        )
    );
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", CLOSING);
    let every_input_unknown = |period| {
        row(
            period,
            &Concept::ALL
                .iter()
                .map(|concept| (*concept, unknown()))
                .collect::<Vec<_>>(),
        )
    };
    let hand_over = handed(
        vec![
            every_input_unknown(quarter.clone()),
            every_input_unknown(instant("2024-09-30")),
            every_input_unknown(instant(CLOSING)),
        ],
        &Settings::preset(),
    );
    assert_eq!(
        outcome_at(&hand_over, &quarter),
        Some(declined("not_a_year"))
    );
}

#[test]
fn a_banks_row_is_computed_with_short_term_investments_contributing_nothing() {
    let bank = short_term_investments_not_applicable(Kind::Bank);
    let mut stated = off_preset_row();
    stated.opening[3] = bank.clone();
    stated.closing[3] = bank;
    assert_eq!(outcome(stated.rows(), &off_preset()), figure("95"));
}

#[test]
fn a_start_outside_the_grammar_gives_start_not_a_date_alone() {
    let unreadable = duration("2024-1-01", CLOSING);
    let mut rows = preset_row().rows();
    rows[0] = row(
        unreadable.clone(),
        &[
            (Concept::OperatingCashFlow, value("800")),
            (Concept::CapitalExpenditure, value("228")),
            (Concept::DilutedSharesWeightedAverage, value("120")),
        ],
    );
    assert_eq!(
        outcome_at(&handed(rows, &Settings::preset()), &unreadable),
        Some(declined("start_not_a_date"))
    );
}

#[test]
fn an_instant_carries_no_value_per_share() {
    let hand_over = handed(preset_row().rows(), &Settings::preset());
    for at in [OPENING, CLOSING] {
        assert_eq!(outcome_at(&hand_over, &instant(at)), None, "{at}");
    }
}

#[test]
fn the_premises_record_the_settings_the_value_was_computed_under() {
    let premises = handed(off_preset_row().rows(), &off_preset())
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
            ("discounted_cash_flow_discount_rate", "0.1"),
            ("discounted_cash_flow_growth_rate", "0.1"),
            ("discounted_cash_flow_years", "2"),
            ("discounted_cash_flow_terminal_growth_rate", "0"),
            ("dividend_growth_years", "5"),
        ]
    );
}
