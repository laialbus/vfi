//! The dividend discount margin of safety as a caller reaches it: a history
//! and a crossing in, and at each year the dividend discount value per share
//! less the close at the valuation date, over that value; or the reasons it
//! has none, with the value beside them.
//!
//! The figures are the proof of
//! `docs/adr/dividend-discount-valuation-and-margin-of-safety.md`. Years are
//! calendar years, 2020 to 2024 and computed at 2024 unless a test says
//! otherwise, and figures are listed from 2024 back. The crossing is
//! `docs/adr/share-basis-rule-dates.md`'s preset unless a test says otherwise.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const VALUE: &str = "dividend_discount_value_per_share";

const MARGIN: &str = "dividend_discount_margin_of_safety";

/// The other metrics that read a price.
const OTHERS_PRICED: [&str; 3] = [
    "discounted_cash_flow_margin_of_safety",
    "price_to_earnings_per_share",
    "price_to_common_book",
];

const ZERO: &str = "zero_dividends_declared_per_share";

const NOT_ABOVE: &str = "required_return_not_above_growth_rate";

const E: &str = "2024-12-31";

const T: &str = "2025-06-30";

/// The close at which the preset row's value of 51 has a margin of a quarter.
const A_QUARTER_BELOW: &str = "38.25";

fn duration(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

/// The calendar year `year`, built by its dates.
fn calendar(year: u32) -> Period {
    duration(&format!("{year}-01-01"), &format!("{year}-12-31"))
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

/// A row at `period` with `dividends_declared_per_share` as `dividend`, and
/// every other concept a value the metric does not read.
fn row(period: Period, dividend: Resolution) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| {
            if Concept::ALL[at] == Concept::DividendsDeclaredPerShare {
                dividend.clone()
            } else {
                value("7")
            }
        }),
    )
}

/// The record's preset row, the calendar years 2020 to 2024: 15.05 over five
/// years, a base of 3.01 and a value of 51.
fn preset_row() -> Vec<Row> {
    (0..)
        .zip(["3.21", "3.11", "3.01", "2.91", "2.81"])
        .map(|(back, amount)| row(calendar(2024 - back), value(amount)))
        .collect()
}

/// `rows` with the row at the calendar year `year` replaced by one whose
/// dividend is `dividend`.
fn with(rows: Vec<Row>, year: u32, dividend: Resolution) -> Vec<Row> {
    rows.into_iter()
        .map(|stated| {
            if *stated.period() == calendar(year) {
                row(calendar(year), dividend.clone())
            } else {
                stated
            }
        })
        .collect()
}

/// `rows` without the calendar year `year`.
fn without(rows: Vec<Row>, year: u32) -> Vec<Row> {
    rows.into_iter()
        .filter(|row| *row.period() != calendar(year))
        .collect()
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
    answered(date, date, A_QUARTER_BELOW)
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

/// A crossing, which cannot be built without `at_valuation_date`.
fn crossing(valuation: Answer, earlier: Vec<Answer>, splits: Splits) -> Crossing {
    Crossing {
        cik: CIK.into(),
        ticker: "VFI".into(),
        at_valuation_date: valuation,
        at_earlier_dates: earlier,
        splits,
    }
}

/// The preset crossing with the valuation date answered with `close`.
fn closing_at(close: &str) -> Crossing {
    crossing(answered(T, T, close), vec![traded(E)], listed(&[]))
}

/// The preset crossing with the split list `splits`.
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

/// The value and the margin at 2024, under the preset.
fn outcomes(rows: Vec<Row>, prices: Option<Crossing>) -> (Outcome, Outcome) {
    outcomes_at(rows, &Settings::preset(), prices, &calendar(2024))
}

/// The margin at 2024 over the preset row beside `prices`, whose value is
/// always 51.
fn margin(prices: Option<Crossing>) -> Outcome {
    let (value, margin) = outcomes(preset_row(), prices);
    assert_eq!(value, figure("51"));
    margin
}

/// The one reason the margin at 2024 over the preset row beside `prices` is
/// absent for.
fn the_one_reason(prices: Option<Crossing>) -> Reason {
    let Outcome::Absent { reasons } = margin(prices) else {
        panic!("the margin is absent");
    };
    let mut each = reasons.each();
    let first = each.next().expect("an absence has a reason").clone();
    assert_eq!(each.next(), None);
    first
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

/// The covered range `from` through `through` does not hold both the period
/// end and the valuation date's trading date.
fn not_covered(from: &str, through: &str) -> Names {
    Names::BasisUnproven {
        basis_unproven: BasisUnproven::NotCovered {
            covered_range: CoveredRange {
                from: from.into(),
                through: through.into(),
            },
            period_end: E.into(),
            trading_date: T.into(),
        },
    }
}

/// The split dated `date`, by its date and factor, stands on or after the
/// period end.
fn split_within(date: &str) -> Names {
    Names::BasisUnproven {
        basis_unproven: BasisUnproven::SplitWithin {
            split: SplitFactor {
                date: date.into(),
                factor: "4.0".into(),
            },
            period_end: E.into(),
            trading_date: T.into(),
        },
    }
}

/// `dividends_declared_per_share` is `Unknown` at `row`.
fn input_unknown(row: &Row) -> Reason {
    Reason::input_unknown(row, Concept::DividendsDeclaredPerShare)
        .expect("the dividend is Unknown at the row")
}

/// The reason of a read at a year the series from 2024 did not find, or at
/// one beyond it.
fn no_year_before_2024() -> Reason {
    Reason::no_period(calendar(2024), "preceding_year".into())
}

/// (51 − 38.25) ÷ 51.
#[test]
fn a_close_a_quarter_below_the_value_is_a_quarter() {
    assert_eq!(margin(Some(preset())), figure("0.25"));
}

/// (51 − 51) ÷ 51.
#[test]
fn a_close_at_the_value_is_zero() {
    assert_eq!(margin(Some(closing_at("51"))), figure("0"));
}

/// (51 − 63.75) ÷ 51: a price above the value is a negative margin.
#[test]
fn a_close_above_the_value_is_negative() {
    assert_eq!(margin(Some(closing_at("63.75"))), figure("-0.25"));
}

#[test]
fn a_close_of_zero_or_below_declines() {
    for close in ["0", "-1"] {
        assert_eq!(
            margin(Some(closing_at(close))),
            declined("non_positive_close"),
            "{close}"
        );
    }
}

#[test]
fn no_crossing_is_not_asked_with_the_value_beside_it() {
    assert_eq!(margin(None), one(not_asked()));
}

#[test]
fn the_valuation_date_answered_with_an_absence_is_answered_absent_carrying_the_date_asked() {
    let reason = the_one_reason(Some(crossing(absence(T), vec![traded(E)], listed(&[]))));
    assert_eq!(
        reason.names(),
        &Names::NoPrice {
            no_price: NoPrice::AnsweredAbsent {
                date_asked: T.into()
            }
        }
    );
}

#[test]
fn the_valuation_date_alone_is_not_covered_and_never_date_not_asked() {
    let alone = crossing(traded(T), vec![], listed(&[]));
    assert_eq!(Reason::date_not_asked(&alone, T), None);
    assert_eq!(the_one_reason(Some(alone)).names(), &not_covered(T, T));
}

#[test]
fn the_split_list_absent_is_splits_absent() {
    let crossing = preset_with(no_list());
    assert_eq!(
        margin(Some(crossing.clone())),
        one(Reason::splits_absent(&crossing).expect("the split list is an absence"))
    );
}

#[test]
fn a_range_that_starts_after_the_period_end_is_not_covered() {
    let late = crossing(traded(T), vec![traded("2025-03-31")], listed(&[]));
    assert_eq!(
        the_one_reason(Some(late)).names(),
        &not_covered("2025-03-31", T)
    );
}

#[test]
fn a_split_inside_the_span_is_split_within_naming_the_split() {
    let reason = the_one_reason(Some(preset_with(listed(&["2025-03-14"]))));
    assert_eq!(reason.names(), &split_within("2025-03-14"));
}

#[test]
fn a_split_dated_the_period_end_is_split_within() {
    let reason = the_one_reason(Some(preset_with(listed(&[E]))));
    assert_eq!(reason.names(), &split_within(E));
}

#[test]
fn a_split_the_day_before_the_period_end_is_a_quarter() {
    assert_eq!(
        margin(Some(preset_with(listed(&["2024-12-30"])))),
        figure("0.25")
    );
}

#[test]
fn each_date_the_rule_compares_outside_the_grammar_declines() {
    for crossing in [
        crossing(
            answered(T, "2025-6-30", A_QUARTER_BELOW),
            vec![traded(E)],
            listed(&[]),
        ),
        crossing(
            traded(T),
            vec![answered(E, "20241231", A_QUARTER_BELOW)],
            listed(&[]),
        ),
        preset_with(listed(&["2025-02-30"])),
    ] {
        assert_eq!(
            margin(Some(crossing)),
            declined("share_basis_date_not_a_date")
        );
    }
}

/// The dividend discount record's open exposure, pinned and not closed. The
/// split restates the years the later filings still quote and leaves the
/// older ones on the old basis, and the rule, asked at the end of 2024, looks
/// only at splits on or after that day. A record that closes it changes this
/// figure knowingly.
#[test]
fn a_split_inside_the_window_is_the_exposure_the_rule_at_e_the_end_of_y_does_not_see() {
    let split_in_the_window = crossing(
        traded(T),
        vec![traded(E), traded("2022-12-30")],
        listed(&["2023-06-15"]),
    );
    assert_eq!(margin(Some(split_in_the_window)), figure("0.25"));
}

#[test]
fn a_close_outside_the_decimal_grammar_declines() {
    assert_eq!(
        margin(Some(closing_at("1e2"))),
        declined("input_not_a_decimal")
    );
}

/// The value's four reasons in series order, then the price read's one.
#[test]
fn the_values_reasons_come_first_and_the_price_reads_follows() {
    let rows = with(
        with(without(preset_row(), 2021), 2024, unknown()),
        2022,
        unknown(),
    );
    assert_eq!(rows[0].period(), &calendar(2024));
    assert_eq!(rows[2].period(), &calendar(2022));
    let unknown_2024 = input_unknown(&rows[0]);
    let of_the_value = vec![
        input_unknown(&rows[2]),
        no_year_before_2024(),
        no_year_before_2024(),
    ];
    let mut of_the_margin = of_the_value.clone();
    of_the_margin.push(not_asked());
    assert_eq!(
        outcomes(rows, None),
        (
            absent(unknown_2024.clone(), of_the_value),
            absent(unknown_2024, of_the_margin)
        )
    );
}

#[test]
fn a_zero_at_the_computing_year_is_carried_into_the_margin_before_the_price_read() {
    let zero = Reason::declined(ZERO.into());
    assert_eq!(
        outcomes(with(preset_row(), 2024, value("0")), None),
        (one(zero.clone()), absent(zero, vec![not_asked()]))
    );
}

#[test]
fn a_required_return_not_above_the_growth_rate_is_carried_into_the_margin_before_the_price_read() {
    let settings = Settings::preset()
        .with_dividend_discount_growth_rate("0.09")
        .expect("a rate above minus one");
    let not_above = Reason::declined(NOT_ABOVE.into());
    assert_eq!(
        outcomes_at(preset_row(), &settings, None, &calendar(2024)),
        (one(not_above.clone()), absent(not_above, vec![not_asked()]))
    );
}

/// The margin's own condition is asked only where the value is a figure, so a
/// declined value beside a close of zero says only what the value says.
#[test]
fn the_close_is_not_tested_where_the_value_is_absent() {
    for prices in [preset(), closing_at("0")] {
        assert_eq!(
            outcomes(with(preset_row(), 2024, value("0")), Some(prices)),
            (declined(ZERO), declined(ZERO))
        );
    }
}

#[test]
fn a_quarter_whose_figure_is_unknown_gives_not_a_year_alone_and_no_read_is_made() {
    let quarter = duration("2024-10-01", E);
    let rows: Vec<Row> = preset_row()
        .into_iter()
        .chain([row(quarter.clone(), unknown())])
        .collect();
    for prices in [None, Some(preset())] {
        assert_eq!(
            outcomes_at(rows.clone(), &Settings::preset(), prices, &quarter),
            (declined("not_a_year"), declined("not_a_year"))
        );
    }
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let unreadable = duration("2024-1-01", E);
    for prices in [None, Some(preset())] {
        assert_eq!(
            outcomes_at(
                vec![row(unreadable.clone(), unknown())],
                &Settings::preset(),
                prices,
                &unreadable
            ),
            (declined("start_not_a_date"), declined("start_not_a_date"))
        );
    }
}

#[test]
fn the_margin_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant { at: E.into() };
    let rows: Vec<Row> = preset_row()
        .into_iter()
        .chain([row(instant.clone(), value("0.25"))])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    assert_eq!(outcome_at(&hand_over, MARGIN, &instant), None);
    for year in 2020..=2024 {
        assert!(
            outcome_at(&hand_over, MARGIN, &calendar(year)).is_some(),
            "{year}"
        );
    }
}

#[test]
fn the_crossing_moves_no_metric_but_those_that_read_a_price_and_is_handed_on_as_it_came() {
    let others = |hand_over: &HandOver| {
        hand_over
            .results()
            .iter()
            .flat_map(|results| results.metrics())
            .filter(|metric| &*metric.name != MARGIN && !OTHERS_PRICED.contains(&&*metric.name))
            .map(|metric| (metric.name.clone(), metric.outcome.clone()))
            .collect::<Vec<_>>()
    };
    let priced = handed(preset_row(), &Settings::preset(), Some(preset()));
    let unpriced = handed(preset_row(), &Settings::preset(), None);
    assert_eq!(others(&priced), others(&unpriced));
    assert_eq!(priced.prices(), &Prices::Asked { crossing: preset() });
    assert_eq!(unpriced.prices(), &Prices::NotAsked {});
}

#[test]
fn adding_the_margin_adds_no_setting_and_keeps_the_version() {
    let premises = handed(preset_row(), &Settings::preset(), Some(preset()))
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
            ("dividend_discount_required_return", "0.0802"),
            ("dividend_discount_growth_rate", "0.02"),
            ("dividend_discount_base_years", "5"),
        ]
    );
}
