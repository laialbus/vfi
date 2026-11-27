//! The dividend yield as a caller reaches it: a history and a crossing in, and
//! at each year the dividends declared per common share over the close at the
//! valuation date; or the reasons it has none.
//!
//! The figures are the proof of `docs/adr/dividend-yield.md`: the year 2024
//! with dividends declared per share of 1.5, beside
//! `docs/adr/share-basis-rule-dates.md`'s preset crossing with a close of 60,
//! computed at the year, unless a test says otherwise. Every earlier date is
//! answered with a close of 50, so a figure a past period end's close entered
//! would not be the figure pinned.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const NAME: &str = "dividend_yield_per_share";

const DIVIDEND: Concept = Concept::DividendsDeclaredPerShare;

const NEGATIVE: &str = "negative_dividends_declared_per_share";

const E: &str = "2024-12-31";

const T: &str = "2025-06-30";

fn duration(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

fn year() -> Period {
    duration("2024-01-01", E)
}

fn year_before() -> Period {
    duration("2023-01-01", "2023-12-31")
}

fn quarter() -> Period {
    duration("2024-10-01", E)
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

/// A row at `period` with `dividends_declared_per_share` as `dividend`,
/// `dividends_paid` 300, and every other concept a value the metric does not
/// read.
fn row(period: Period, dividend: Resolution) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| match Concept::ALL[at] {
            DIVIDEND => dividend.clone(),
            Concept::DividendsPaid => value("300"),
            _ => value("7"),
        }),
    )
}

/// The year alone, declaring `amount` per share.
fn declaring(amount: &str) -> Vec<Row> {
    vec![row(year(), value(amount))]
}

/// The record's worked history: the year declaring 1.5 per share.
fn worked() -> Vec<Row> {
    declaring("1.5")
}

/// The worked history beside the year before it, declaring 1.2 per share.
fn two_years() -> Vec<Row> {
    vec![row(year_before(), value("1.2")), row(year(), value("1.5"))]
}

fn answered(asked: &str, traded: &str, close: &str) -> Answer {
    Answer::Price {
        date_asked: asked.into(),
        trading_date: traded.into(),
        close: close.into(),
        retrieved_from: "https://example.invalid/prices".into(),
    }
}

/// The valuation date answered with `close`, traded that day.
fn valued_at(close: &str) -> Answer {
    answered(T, T, close)
}

/// An earlier date answered with a close traded that day.
fn earlier(date: &str) -> Answer {
    answered(date, date, "50")
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
    crossing(valued_at(close), vec![earlier(E)], listed(&[]))
}

/// The preset crossing with the split list `splits`.
fn preset_with(splits: Splits) -> Crossing {
    crossing(valued_at("60"), vec![earlier(E)], splits)
}

fn preset() -> Crossing {
    preset_with(listed(&[]))
}

/// The preset crossing with the valuation date answered with an absence.
fn valuation_absent() -> Crossing {
    crossing(absence(T), vec![earlier(E)], listed(&[]))
}

fn handed(rows: Vec<Row>, settings: &Settings, prices: Option<Crossing>) -> HandOver {
    analyze(History::of(CIK.into(), rows), prices, settings)
        .expect("every crossing here names the history's filer")
}

/// The outcome of the metric named `name` at `period`, or none where the
/// results there do not hold it.
fn outcome_of(hand_over: &HandOver, name: &str, period: &Period) -> Option<Outcome> {
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

/// The yield at `period`, from a history of `rows` beside `prices`, under the
/// preset.
fn yield_at(rows: Vec<Row>, prices: Option<Crossing>, period: &Period) -> Outcome {
    outcome_of(&handed(rows, &Settings::preset(), prices), NAME, period)
        .expect("the metric is stated at the duration given")
}

/// The yield at the year.
fn yielded(rows: Vec<Row>, prices: Option<Crossing>) -> Outcome {
    yield_at(rows, prices, &year())
}

/// The one reason the yield at `period` of `rows` beside `prices` is absent
/// for.
fn the_one_reason_at(rows: Vec<Row>, prices: Crossing, period: &Period) -> Reason {
    let Outcome::Absent { reasons } = yield_at(rows, Some(prices), period) else {
        panic!("the metric is absent");
    };
    let mut each = reasons.each();
    let first = each.next().expect("an absence has a reason").clone();
    assert_eq!(each.next(), None);
    first
}

/// The one reason the yield at the year of the worked history beside `prices`
/// is absent for.
fn the_one_reason(prices: Crossing) -> Reason {
    the_one_reason_at(worked(), prices, &year())
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

fn answered_absent() -> Names {
    Names::NoPrice {
        no_price: NoPrice::AnsweredAbsent {
            date_asked: T.into(),
        },
    }
}

/// The prices carried trade `from` through `through`, which does not reach
/// from `period_end` to the valuation date's close.
fn not_covered(from: &str, through: &str, period_end: &str) -> Names {
    Names::BasisUnproven {
        basis_unproven: BasisUnproven::NotCovered {
            covered_range: CoveredRange {
                from: from.into(),
                through: through.into(),
            },
            period_end: period_end.into(),
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

/// 1.5 ÷ 60.
#[test]
fn the_years_dividend_over_the_close_is_two_and_a_half_hundredths() {
    assert_eq!(yielded(worked(), Some(preset())), figure("0.025"));
}

/// 2 ÷ 60 is one thirtieth, rounded once where it is written.
#[test]
fn a_quotient_that_does_not_terminate_is_written_to_the_one_precision() {
    let thirtieth = format!("0.0{}", "3".repeat(34));
    assert_eq!(yielded(declaring("2"), Some(preset())), figure(&thirtieth));
}

/// 1.2 ÷ 60 and 1.5 ÷ 60. Over each year's own close, 1.2 ÷ 40 and 1.5 ÷ 50
/// would both be written `0.03`.
#[test]
fn a_past_years_dividend_is_set_over_the_valuation_dates_close_and_no_other() {
    for close_then in ["40", "4000"] {
        let prices = crossing(
            valued_at("60"),
            vec![earlier(E), answered("2023-12-31", "2023-12-29", close_then)],
            listed(&[]),
        );
        let hand_over = handed(two_years(), &Settings::preset(), Some(prices));
        assert_eq!(
            outcome_of(&hand_over, NAME, &year_before()),
            Some(figure("0.02")),
            "{close_then}"
        );
        assert_eq!(
            outcome_of(&hand_over, NAME, &year()),
            Some(figure("0.025")),
            "{close_then}"
        );
    }
}

#[test]
fn an_earlier_year_no_price_was_asked_at_is_not_covered() {
    let reason = the_one_reason_at(two_years(), preset(), &year_before());
    assert_eq!(reason.names(), &not_covered(E, T, "2023-12-31"));
}

#[test]
fn a_year_declaring_nothing_yields_zero_and_meets_no_condition() {
    assert_eq!(yielded(declaring("0"), Some(preset())), figure("0"));
}

/// −1.5 ÷ 60 would be written `-0.025`.
#[test]
fn a_negative_dividend_declines_and_is_never_a_signed_yield() {
    assert_eq!(
        yielded(declaring("-1.5"), Some(preset())),
        declined(NEGATIVE)
    );
}

#[test]
fn a_close_of_zero_or_below_declines() {
    for close in ["0", "-1"] {
        assert_eq!(
            yielded(worked(), Some(closing_at(close))),
            declined("non_positive_close"),
            "{close}"
        );
    }
}

/// The dividend is asked first, so −1.5 ÷ −60 is never written as `0.025`.
#[test]
fn a_negative_dividend_beside_a_close_of_zero_or_below_gives_the_dividends_condition_alone() {
    for close in ["0", "-60"] {
        assert_eq!(
            yielded(declaring("-1.5"), Some(closing_at(close))),
            declined(NEGATIVE),
            "{close}"
        );
    }
}

#[test]
fn a_dividend_of_zero_beside_a_close_of_zero_declines_and_is_never_zero() {
    assert_eq!(
        yielded(declaring("0"), Some(closing_at("0"))),
        declined("non_positive_close")
    );
}

#[test]
fn no_crossing_is_not_asked() {
    assert_eq!(yielded(worked(), None), one(not_asked()));
}

/// A negative dividend beside no price says only that no price was asked,
/// and a dividend that was not read beside a close of zero says only that.
#[test]
fn no_condition_is_asked_unless_both_reads_gave_a_number() {
    assert_eq!(yielded(declaring("-1.5"), None), one(not_asked()));

    let rows = vec![row(year(), unknown())];
    let unknown_at_the_year =
        Reason::input_unknown(&rows[0], DIVIDEND).expect("the dividend is Unknown at the year");
    assert_eq!(
        yielded(rows, Some(closing_at("0"))),
        one(unknown_at_the_year)
    );
}

#[test]
fn the_valuation_date_answered_with_an_absence_is_answered_absent_carrying_the_date_asked() {
    assert_eq!(
        the_one_reason(valuation_absent()).names(),
        &answered_absent()
    );
}

#[test]
fn the_split_list_absent_is_splits_absent() {
    let crossing = preset_with(no_list());
    assert_eq!(
        yielded(worked(), Some(crossing.clone())),
        one(Reason::splits_absent(&crossing).expect("the split list is an absence"))
    );
}

#[test]
fn the_valuation_date_alone_is_not_covered_and_never_date_not_asked() {
    let alone = crossing(valued_at("60"), vec![], listed(&[]));
    assert_eq!(Reason::date_not_asked(&alone, T), None);
    assert_eq!(the_one_reason(alone).names(), &not_covered(T, T, E));
}

/// A dividend on one share basis is never set over a close on another.
#[test]
fn a_split_inside_the_span_is_split_within_naming_the_split() {
    let reason = the_one_reason(preset_with(listed(&["2025-03-14"])));
    assert_eq!(reason.names(), &split_within("2025-03-14"));
}

#[test]
fn a_split_dated_the_period_end_is_split_within() {
    let reason = the_one_reason(preset_with(listed(&[E])));
    assert_eq!(reason.names(), &split_within(E));
}

#[test]
fn a_split_the_day_before_the_period_end_leaves_the_figure() {
    assert_eq!(
        yielded(worked(), Some(preset_with(listed(&["2024-12-30"])))),
        figure("0.025")
    );
}

#[test]
fn the_paired_closes_trading_date_outside_the_grammar_declines() {
    let paired = crossing(
        answered(T, "2025-6-30", "60"),
        vec![earlier(E)],
        listed(&[]),
    );
    assert_eq!(
        yielded(worked(), Some(paired)),
        declined("share_basis_date_not_a_date")
    );
}

/// The year test asks first, so the end never reaches the share-basis rule.
#[test]
fn a_year_whose_end_is_outside_the_grammar_gives_end_not_a_date_and_never_the_rules_condition() {
    let unreadable = duration("2024-01-01", "2024-12-1");
    for prices in [None, Some(preset())] {
        assert_eq!(
            yield_at(
                vec![row(unreadable.clone(), value("1.5"))],
                prices,
                &unreadable
            ),
            declined("end_not_a_date")
        );
    }
}

#[test]
fn a_close_outside_the_decimal_grammar_declines() {
    assert_eq!(
        yielded(worked(), Some(closing_at("1e2"))),
        declined("input_not_a_decimal")
    );
}

#[test]
fn a_dividend_outside_the_decimal_grammar_declines_from_the_read() {
    assert_eq!(
        yielded(declaring("1e2"), Some(preset())),
        declined("input_not_a_decimal")
    );
}

/// 300 of dividends paid stands in the same row, and over the 7 shares beside
/// it would give a figure.
#[test]
fn an_unknown_dividend_is_input_unknown_alone_and_is_never_formed_from_dividends_paid() {
    let rows = vec![row(year(), unknown())];
    assert_eq!(rows[0].of(Concept::DividendsPaid), &value("300"));
    let unknown_at_the_year =
        Reason::input_unknown(&rows[0], DIVIDEND).expect("the dividend is Unknown at the year");
    assert_eq!(yielded(rows, Some(preset())), one(unknown_at_the_year));
}

#[test]
fn a_filer_of_each_kind_has_the_same_yield() {
    assert_eq!(Kind::ALL.len(), 5);
    for kind in Kind::ALL.iter().copied() {
        let rows: Vec<Row> = worked()
            .into_iter()
            .map(|stated| {
                Row::at(
                    stated.period().clone(),
                    std::array::from_fn(|at| {
                        let concept = Concept::ALL[at];
                        match concept.definition().applies_to.excluding(kind) {
                            Some(excluded) => Resolution::NotApplicable { excluded },
                            None => stated.of(concept).clone(),
                        }
                    }),
                )
            })
            .collect();
        assert!(
            matches!(rows[0].of(DIVIDEND), Resolution::Value { .. }),
            "{kind:?}"
        );
        assert_eq!(yielded(rows, Some(preset())), figure("0.025"), "{kind:?}");
    }
}

#[test]
fn a_quarter_whose_dividend_is_unknown_gives_not_a_year_alone_and_no_read_is_made() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([row(quarter(), unknown())])
        .collect();
    for prices in [None, Some(preset())] {
        assert_eq!(
            yield_at(rows.clone(), prices, &quarter()),
            declined("not_a_year")
        );
    }
}

/// 0.375 ÷ 60 would be written `0.00625`, about a quarter of the year's yield
/// with nothing to mark it.
#[test]
fn a_quarters_dividend_is_never_set_over_the_close() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([row(quarter(), value("0.375"))])
        .collect();
    assert_eq!(
        yield_at(rows, Some(preset()), &quarter()),
        declined("not_a_year")
    );
}

#[test]
fn the_reads_reason_comes_first_and_the_price_reads_follows() {
    let rows = vec![row(year(), unknown())];
    let unknown_at_the_year =
        Reason::input_unknown(&rows[0], DIVIDEND).expect("the dividend is Unknown at the year");
    assert_eq!(
        yielded(rows, None),
        absent(unknown_at_the_year, vec![not_asked()])
    );
}

#[test]
fn a_dividend_outside_the_decimal_grammar_does_not_stop_the_price_read() {
    let Outcome::Absent { reasons } = yielded(declaring("1e2"), Some(valuation_absent())) else {
        panic!("the metric is absent");
    };
    let mut each = reasons.each();
    assert_eq!(
        each.next(),
        Some(&Reason::declined("input_not_a_decimal".into()))
    );
    assert_eq!(each.next().map(Reason::names), Some(&answered_absent()));
    assert_eq!(each.next(), None);
}

#[test]
fn it_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant { at: E.into() };
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([
            row(quarter(), value("0.375")),
            row(instant.clone(), value("1.5")),
        ])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    assert_eq!(outcome_of(&hand_over, NAME, &instant), None);
    assert_eq!(outcome_of(&hand_over, NAME, &year()), Some(figure("0.025")));
    assert_eq!(
        outcome_of(&hand_over, NAME, &quarter()),
        Some(declined("not_a_year"))
    );
}

#[test]
fn it_reads_no_setting() {
    let settings = Settings::preset()
        .with_dividend_discount_growth_rate("0.09")
        .expect("a rate above minus one");
    assert_eq!(
        outcome_of(&handed(worked(), &settings, Some(preset())), NAME, &year()),
        Some(figure("0.025"))
    );
}

#[test]
fn adding_the_yield_adds_no_setting_and_keeps_the_version() {
    let premises = handed(worked(), &Settings::preset(), Some(preset()))
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
