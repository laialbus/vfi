//! The discounted cash flow margin of safety as a caller reaches it: a history
//! and a crossing in, and at each year the value per share less the close at
//! the valuation date, over that value; or the reasons it has none, with the
//! value beside them.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{CoveredRange, HandOver, Outcome, Prices, Reason, Reasons};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const VALUE: &str = "discounted_cash_flow_value_per_share";

const MARGIN: &str = "discounted_cash_flow_margin_of_safety";

const OPENING: &str = "2023-12-31";

const E: &str = "2024-12-31";

const T: &str = "2025-06-30";

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
    duration("2024-01-01", E)
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
            self.balances(E, &self.closing),
        ]
    }

    fn balances(&self, at: &str, stated: &[Resolution; 4]) -> Row {
        row(
            instant(at),
            &BALANCES.into_iter().zip(stated.clone()).collect::<Vec<_>>(),
        )
    }
}

/// Free cash flow 572, net debt 600 at both instants, 120 diluted shares: a
/// value of 80 under the presets.
fn preset_row() -> Stated {
    Stated {
        operating_cash_flow: value("800"),
        capital_expenditure: value("228"),
        opening: ["100", "900", "300", "100"].map(value),
        closing: ["100", "1100", "500", "100"].map(value),
        diluted_shares_weighted_average: value("120"),
    }
}

/// Free cash flow 100, net debt 200 at both instants, 10 diluted shares: a
/// value of 100 under [`off_preset`].
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

fn answered(asked: &str, traded: &str, close: &str) -> Answer {
    Answer::Price {
        date_asked: asked.into(),
        trading_date: traded.into(),
        close: close.into(),
        retrieved_from: "https://example.invalid/prices".into(),
    }
}

fn traded(date: &str) -> Answer {
    answered(date, date, "60")
}

fn absence(date: &str) -> Answer {
    Answer::Absence {
        date_asked: date.into(),
        reason: fetch_analyze::Reason::NoClose {},
    }
}

fn split(date: &str) -> SplitFactor {
    SplitFactor {
        date: date.into(),
        factor: "4.0".into(),
    }
}

fn listed(dates: &[&str]) -> Splits {
    Splits::Listed {
        factors: dates.iter().map(|date| split(date)).collect(),
    }
}

fn no_list() -> Splits {
    Splits::Absence {
        reason: fetch_analyze::Reason::NoKey {},
    }
}

fn crossing(valuation: Answer, earlier: Vec<Answer>, splits: Splits) -> Crossing {
    Crossing {
        cik: CIK.into(),
        ticker: "VFI".into(),
        at_valuation_date: valuation,
        at_earlier_dates: earlier,
        splits,
    }
}

/// The valuation date answered with `close`, and the earlier date with a
/// close traded on the period end.
fn closing_at(close: &str) -> Crossing {
    crossing(answered(T, T, close), vec![traded(E)], listed(&[]))
}

/// The share-basis record's preset, with the split list `splits`.
fn preset_with(splits: Splits) -> Crossing {
    crossing(traded(T), vec![traded(E)], splits)
}

fn preset() -> Crossing {
    preset_with(listed(&[]))
}

fn handed(rows: Vec<Row>, settings: &Settings, prices: Option<Crossing>) -> HandOver {
    analyze(History::of(CIK.into(), rows), prices, settings)
        .expect("every crossing here names the history's filer")
}

/// The outcome of the metric named `name` at `period`, or none where the
/// results there hold no metric of that name.
fn outcome_at(hand_over: &HandOver, name: &str, period: &Period) -> Option<Outcome> {
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

/// The value and the margin at `period`, from a history of `rows` beside
/// `prices`, after showing the value is what it is where no price was asked.
fn outcomes_at(
    rows: Vec<Row>,
    settings: &Settings,
    prices: Option<Crossing>,
    period: &Period,
) -> (Outcome, Outcome) {
    let priced = handed(rows.clone(), settings, prices);
    let value =
        outcome_at(&priced, VALUE, period).expect("the value is stated at the duration given");
    assert_eq!(
        Some(&value),
        outcome_at(&handed(rows, settings, None), VALUE, period).as_ref(),
        "the value reads no price"
    );
    (
        value,
        outcome_at(&priced, MARGIN, period).expect("the margin is stated at the duration given"),
    )
}

/// The value and the margin over the year.
fn outcomes(rows: Vec<Row>, settings: &Settings, prices: Option<Crossing>) -> (Outcome, Outcome) {
    outcomes_at(rows, settings, prices, &year())
}

/// The margin over the preset row beside `prices`, whose value is always 80.
fn margin(prices: Option<Crossing>) -> Outcome {
    let (value, margin) = outcomes(preset_row().rows(), &Settings::preset(), prices);
    assert_eq!(value, figure("80"));
    margin
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

fn one(reason: Reason) -> Outcome {
    absent(reason, Vec::new())
}

fn declined(condition: &str) -> Outcome {
    one(Reason::declined(condition.into()))
}

fn not_asked() -> Reason {
    Reason::not_asked(&Prices::NotAsked {}).expect("no price was asked")
}

fn not_covered(from: &str, through: &str, traded: &str) -> Outcome {
    one(Reason::not_covered(
        CoveredRange {
            from: from.into(),
            through: through.into(),
        },
        E.into(),
        traded.into(),
    ))
}

fn split_within(crossing: &Crossing, date: &str, traded: &str) -> Outcome {
    one(
        Reason::split_within(crossing, &split(date), E.into(), traded.into())
            .expect("the split is one the list holds"),
    )
}

fn splits_absent(crossing: &Crossing) -> Outcome {
    one(Reason::splits_absent(crossing).expect("the split list is an absence"))
}

/// (80 − 60) ÷ 80.
#[test]
fn the_preset_row_beside_the_preset_crossing_is_a_quarter() {
    assert_eq!(margin(Some(preset())), figure("0.25"));
}

/// (80 − 100) ÷ 80: a price above the value is a negative margin.
#[test]
fn a_close_above_the_value_is_negative() {
    assert_eq!(margin(Some(closing_at("100"))), figure("-0.25"));
}

#[test]
fn a_close_of_zero_declines() {
    assert_eq!(
        margin(Some(closing_at("0"))),
        declined("non_positive_close")
    );
}

/// Net debt 1200 equals the value of operations, then 1300 exceeds it by 100.
#[test]
fn a_value_of_zero_or_below_is_a_figure_and_the_margin_declines() {
    for (long_term_debt, value_per_share) in [("1300", "0"), ("1400", "-10")] {
        let mut stated = off_preset_row();
        stated.opening[1] = value(long_term_debt);
        stated.closing[1] = value(long_term_debt);
        assert_eq!(
            outcomes(stated.rows(), &off_preset(), Some(preset())),
            (
                figure(value_per_share),
                declined("non_positive_value_per_share")
            ),
            "{long_term_debt}"
        );
    }
}

#[test]
fn a_value_of_zero_or_below_is_asked_before_the_close() {
    let mut stated = off_preset_row();
    stated.opening[1] = value("1400");
    stated.closing[1] = value("1400");
    assert_eq!(
        outcomes(stated.rows(), &off_preset(), Some(closing_at("0"))).1,
        declined("non_positive_value_per_share")
    );
}

/// The margin's own conditions are asked only where the price read gave a
/// number, so a negative value with no crossing says only that none was asked.
#[test]
fn the_value_is_not_tested_where_the_price_read_gave_no_number() {
    let mut stated = off_preset_row();
    stated.opening[1] = value("1400");
    stated.closing[1] = value("1400");
    assert_eq!(
        outcomes(stated.rows(), &off_preset(), None),
        (figure("-10"), one(not_asked()))
    );
}

#[test]
fn no_crossing_is_not_asked_with_the_value_beside_it() {
    assert_eq!(margin(None), one(not_asked()));
}

#[test]
fn the_valuation_date_answered_with_an_absence_is_answered_absent() {
    let crossing = crossing(absence(T), vec![traded(E)], listed(&[]));
    assert_eq!(
        margin(Some(crossing.clone())),
        one(Reason::answered_absent(&crossing, T).expect("the answer is an absence"))
    );
}

#[test]
fn the_valuation_date_alone_is_not_covered_and_never_date_not_asked() {
    assert_eq!(
        margin(Some(crossing(traded(T), vec![], listed(&[])))),
        not_covered(T, T, T)
    );
}

#[test]
fn the_split_list_absent_is_splits_absent() {
    let crossing = preset_with(no_list());
    assert_eq!(margin(Some(crossing.clone())), splits_absent(&crossing));
}

#[test]
fn a_range_that_starts_after_the_period_end_is_not_covered() {
    assert_eq!(
        margin(Some(crossing(
            traded(T),
            vec![traded("2025-03-31")],
            listed(&[])
        ))),
        not_covered("2025-03-31", T, T)
    );
}

#[test]
fn a_split_inside_the_span_is_split_within() {
    let crossing = preset_with(listed(&["2025-03-14"]));
    assert_eq!(
        margin(Some(crossing.clone())),
        split_within(&crossing, "2025-03-14", T)
    );
}

#[test]
fn a_split_dated_the_period_end_is_split_within() {
    let crossing = preset_with(listed(&[E]));
    assert_eq!(
        margin(Some(crossing.clone())),
        split_within(&crossing, E, T)
    );
}

#[test]
fn a_split_the_day_before_the_period_end_is_a_quarter() {
    assert_eq!(
        margin(Some(preset_with(listed(&["2024-12-30"])))),
        figure("0.25")
    );
}

#[test]
fn one_price_on_the_period_end_is_a_quarter_until_a_split_is_dated_that_day() {
    assert_eq!(
        margin(Some(crossing(traded(E), vec![], listed(&[])))),
        figure("0.25")
    );

    let split_that_day = crossing(traded(E), vec![], listed(&[E]));
    assert_eq!(
        margin(Some(split_that_day.clone())),
        split_within(&split_that_day, E, E)
    );
}

#[test]
fn a_year_ending_after_every_price_is_not_covered() {
    assert_eq!(
        margin(Some(crossing(
            traded("2024-06-28"),
            vec![traded("2023-12-29")],
            listed(&[])
        ))),
        not_covered("2023-12-29", "2024-06-28", "2024-06-28")
    );
}

#[test]
fn each_date_the_rule_compares_outside_the_grammar_declines() {
    for crossing in [
        crossing(answered(T, "2025-6-30", "60"), vec![traded(E)], listed(&[])),
        crossing(traded(T), vec![answered(E, "20241231", "60")], listed(&[])),
        preset_with(listed(&["2025-02-30"])),
    ] {
        assert_eq!(
            margin(Some(crossing)),
            declined("share_basis_date_not_a_date")
        );
    }
}

#[test]
fn the_split_list_absent_is_read_before_any_date() {
    let crossing = crossing(answered(T, "2025-6-30", "60"), vec![traded(E)], no_list());
    assert_eq!(margin(Some(crossing.clone())), splits_absent(&crossing));
}

#[test]
fn no_date_asked_is_read() {
    assert_eq!(
        margin(Some(crossing(
            answered("2025-6-30", T, "60"),
            vec![traded(E)],
            listed(&[])
        ))),
        figure("0.25")
    );
    assert_eq!(
        margin(Some(crossing(
            traded(T),
            vec![traded(E), absence("20241231")],
            listed(&[])
        ))),
        figure("0.25")
    );
}

#[test]
fn the_earlier_answers_in_either_order_give_the_same_outcome() {
    for earlier in [
        vec![traded(E), traded("2025-03-31")],
        vec![traded("2025-03-31"), traded(E)],
    ] {
        assert_eq!(
            margin(Some(crossing(traded(T), earlier, listed(&[])))),
            figure("0.25")
        );
    }
}

#[test]
fn the_first_split_the_list_holds_is_named_in_each_order() {
    for dates in [["2025-01-15", "2025-03-14"], ["2025-03-14", "2025-01-15"]] {
        let crossing = preset_with(listed(&dates));
        assert_eq!(
            margin(Some(crossing.clone())),
            split_within(&crossing, dates[0], T)
        );
    }
}

#[test]
fn a_close_outside_the_decimal_grammar_declines() {
    assert_eq!(
        margin(Some(closing_at("1e2"))),
        declined("input_not_a_decimal")
    );
}

/// The value's six reasons in read order, then the price read's one.
#[test]
fn the_values_reasons_come_first_and_the_price_reads_follows() {
    let stated = Stated {
        capital_expenditure: unknown(),
        diluted_shares_weighted_average: unknown(),
        ..preset_row()
    };
    let rows = stated.year_and_closing();
    let no_opening = || Reason::no_period(year(), "opening_instant".into());
    let capital_expenditure = Reason::input_unknown(&rows[0], Concept::CapitalExpenditure)
        .expect("capital expenditure is Unknown at the year");
    let diluted_shares = Reason::input_unknown(&rows[0], Concept::DilutedSharesWeightedAverage)
        .expect("the diluted shares are Unknown at the year");
    let of_the_value = vec![
        no_opening(),
        no_opening(),
        no_opening(),
        no_opening(),
        diluted_shares,
    ];
    let mut of_the_margin = of_the_value.clone();
    of_the_margin.push(not_asked());
    assert_eq!(
        outcomes(rows, &Settings::preset(), None),
        (
            absent(capital_expenditure.clone(), of_the_value),
            absent(capital_expenditure, of_the_margin)
        )
    );
}

#[test]
fn a_declined_value_is_carried_into_the_margin_before_the_price_read() {
    let stated = Stated {
        capital_expenditure: value("800"),
        ..preset_row()
    };
    let non_positive = Reason::declined("non_positive_free_cash_flow".into());
    assert_eq!(
        outcomes(stated.rows(), &Settings::preset(), None),
        (
            one(non_positive.clone()),
            absent(non_positive, vec![not_asked()])
        )
    );
}

#[test]
fn a_quarter_is_not_a_year_and_no_read_is_made() {
    let quarter = duration("2024-10-01", E);
    let every_input_unknown = |period| {
        row(
            period,
            &Concept::ALL
                .iter()
                .map(|concept| (*concept, unknown()))
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(
        outcomes_at(
            vec![
                every_input_unknown(quarter.clone()),
                every_input_unknown(instant("2024-09-30")),
                every_input_unknown(instant(E)),
            ],
            &Settings::preset(),
            None,
            &quarter,
        ),
        (declined("not_a_year"), declined("not_a_year"))
    );
}

/// (95 − 57) ÷ 95, the value's net debt carrying no short-term investments.
#[test]
fn a_banks_row_is_a_margin_at_a_price() {
    let bank = short_term_investments_not_applicable(Kind::Bank);
    let mut stated = off_preset_row();
    stated.opening[3] = bank.clone();
    stated.closing[3] = bank;
    assert_eq!(
        outcomes(stated.rows(), &off_preset(), Some(closing_at("57"))),
        (figure("95"), figure("0.4"))
    );
}

#[test]
fn a_start_outside_the_grammar_gives_start_not_a_date_alone() {
    let unreadable = duration("2024-1-01", E);
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
        outcomes_at(rows, &Settings::preset(), Some(preset()), &unreadable),
        (declined("start_not_a_date"), declined("start_not_a_date"))
    );
}

#[test]
fn an_instant_carries_no_margin() {
    let hand_over = handed(preset_row().rows(), &Settings::preset(), Some(preset()));
    for at in [OPENING, E] {
        assert_eq!(outcome_at(&hand_over, MARGIN, &instant(at)), None, "{at}");
    }
}

#[test]
fn the_crossing_moves_no_other_metric_and_is_handed_on_as_it_came() {
    let others = |hand_over: &HandOver| {
        hand_over
            .results()
            .iter()
            .flat_map(|results| results.metrics())
            .filter(|metric| &*metric.name != MARGIN)
            .map(|metric| (metric.name.clone(), metric.outcome.clone()))
            .collect::<Vec<_>>()
    };
    let priced = handed(preset_row().rows(), &Settings::preset(), Some(preset()));
    let unpriced = handed(preset_row().rows(), &Settings::preset(), None);
    assert_eq!(others(&priced), others(&unpriced));
    assert_eq!(priced.prices(), &Prices::Asked { crossing: preset() });
    assert_eq!(unpriced.prices(), &Prices::NotAsked {});
}

#[test]
fn the_premises_are_the_six_settings_under_method_version_one() {
    let premises = handed(off_preset_row().rows(), &off_preset(), Some(preset()))
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
            ("dividend_discount_required_return", "0.0802"),
            ("dividend_discount_growth_rate", "0.02"),
            ("dividend_discount_base_years", "5"),
        ]
    );
}
