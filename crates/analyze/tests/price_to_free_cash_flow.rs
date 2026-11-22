//! Price/free cash flow as a caller reaches it: a history and a crossing in,
//! and at each year market capitalisation over the averaged count over free
//! cash flow; or the reasons it has none.
//!
//! The figures are the proof of
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`: the year
//! 2024 with operating cash flow of 800 and capital expenditure of 300, and
//! the instants 2023-12-31 and 2024-12-31 with shares outstanding of 90 and
//! 110, beside `docs/adr/share-basis-rule-dates.md`'s preset crossing with a
//! close of 60, computed at the year, unless a test says otherwise.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const NAME: &str = "price_to_free_cash_flow";

const PAYOUT: &str = "dividend_payout_on_free_cash_flow";

const VALUE: &str = "discounted_cash_flow_value_per_share";

const COUNT: Concept = Concept::SharesOutstanding;

const OPERATING: Concept = Concept::OperatingCashFlow;

const CAPITAL: Concept = Concept::CapitalExpenditure;

const OPENING: &str = "2023-12-31";

const E: &str = "2024-12-31";

const T: &str = "2025-06-30";

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

/// A row at `period` stating the count, operating cash flow and capital
/// expenditure as given, and every other concept a value the metric does not
/// read.
fn row(period: Period, [count, operating, capital]: [Resolution; 3]) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| match Concept::ALL[at] {
            COUNT => count.clone(),
            OPERATING => operating.clone(),
            CAPITAL => capital.clone(),
            _ => value("7"),
        }),
    )
}

/// A row at `period` with every concept the metric reads as `state`.
fn stating(period: Period, state: Resolution) -> Row {
    row(period, [state.clone(), state.clone(), state])
}

/// The year, then the instants it opens and closes at, each holding its
/// state: operating cash flow and capital expenditure over the year, then
/// the count opening and closing.
fn history_in([operating, capital, opening, closing]: [Resolution; 4]) -> Vec<Row> {
    vec![
        row(year(), [value("7"), operating, capital]),
        stating(instant(OPENING), opening),
        stating(instant(E), closing),
    ]
}

/// The worked history with the count `opening` and `closing`, operating cash
/// flow of 800 and capital expenditure of `capital`.
fn history(opening: &str, closing: &str, capital: &str) -> Vec<Row> {
    history_in([value("800"), value(capital), value(opening), value(closing)])
}

/// The record's worked history.
fn worked() -> Vec<Row> {
    history("90", "110", "300")
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

fn split_by(date: &str, factor: &str) -> SplitFactor {
    SplitFactor {
        date: date.into(),
        factor: factor.into(),
    }
}

fn listed(dates: &[&str]) -> Splits {
    Splits::Listed {
        factors: dates.iter().map(|date| split_by(date, "4.0")).collect(),
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

/// The metric's outcome at `period`, or none where the results there do not
/// hold it.
fn outcome_at(hand_over: &HandOver, period: &Period) -> Option<Outcome> {
    outcome_of(hand_over, NAME, period)
}

/// The metric at `period`, from a history of `rows` beside `prices`, under
/// the preset.
fn multiple_at(rows: Vec<Row>, prices: Option<Crossing>, period: &Period) -> Outcome {
    outcome_at(&handed(rows, &Settings::preset(), prices), period)
        .expect("the metric is stated at the duration given")
}

/// The metric at the year.
fn multiple(rows: Vec<Row>, prices: Option<Crossing>) -> Outcome {
    multiple_at(rows, prices, &year())
}

/// The one reason the metric at the year of the worked history beside
/// `prices` is absent for.
fn the_one_reason(prices: Crossing) -> Reason {
    let Outcome::Absent { reasons } = multiple(worked(), Some(prices)) else {
        panic!("the metric is absent");
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

/// The history lacks the instant the year is `sought_by`.
fn no_period(sought_by: &str) -> Reason {
    Reason::no_period(year(), sought_by.into())
}

/// The split dated `date`, by its date and factor, stands on or after the
/// period end.
fn split_within(date: &str) -> Names {
    Names::BasisUnproven {
        basis_unproven: BasisUnproven::SplitWithin {
            split: split_by(date, "4.0"),
            period_end: E.into(),
            trading_date: T.into(),
        },
    }
}

/// 60 × 90 is 5400 and 60 × 110 is 6600, whose mean is 6000, over 800 less
/// 300, which is 500. The closing count alone would give 6600 over 500,
/// written `13.2`.
#[test]
fn the_average_of_the_two_terms_over_free_cash_flow_is_twelve_and_never_the_closing_counts_figure()
{
    let outcome = multiple(worked(), Some(preset()));
    assert_eq!(outcome, figure("12"));
    assert_ne!(outcome, figure("13.2"));
}

/// 60 × 45, the mean of 90 and 0, over 500 would be written `5.4`.
#[test]
fn either_count_at_zero_or_below_declines_and_is_never_averaged_in() {
    for (opening, closing) in [("90", "0"), ("0", "110"), ("90", "-110"), ("-90", "110")] {
        assert_eq!(
            multiple(history(opening, closing, "300"), Some(preset())),
            declined("non_positive_shares_outstanding"),
            "{opening} and {closing}"
        );
    }
}

#[test]
fn a_close_of_zero_or_below_declines() {
    for close in ["0", "-1"] {
        assert_eq!(
            multiple(worked(), Some(closing_at(close))),
            declined("non_positive_close"),
            "{close}"
        );
    }
}

#[test]
fn zero_free_cash_flow_declines_as_a_zero_divisor() {
    assert_eq!(
        multiple(history("90", "110", "800"), Some(preset())),
        declined("zero_free_cash_flow")
    );
}

/// 6000 over −100 would be written `-60`, below every real multiple.
#[test]
fn negative_free_cash_flow_declines_and_is_never_a_signed_multiple() {
    assert_eq!(
        multiple(history("90", "110", "900"), Some(preset())),
        declined("negative_free_cash_flow")
    );
}

/// The conditions are asked in formula order: the close, the count, then
/// free cash flow at zero and below it.
#[test]
fn the_first_condition_that_holds_is_the_one_reason() {
    for (close, [opening, closing, capital], condition) in [
        ("0", ["90", "0", "300"], "non_positive_close"),
        ("0", ["90", "110", "800"], "non_positive_close"),
        ("-60", ["90", "110", "900"], "non_positive_close"),
        ("60", ["90", "0", "800"], "non_positive_shares_outstanding"),
        ("60", ["0", "110", "800"], "non_positive_shares_outstanding"),
        (
            "60",
            ["-90", "110", "900"],
            "non_positive_shares_outstanding",
        ),
    ] {
        assert_eq!(
            multiple(history(opening, closing, capital), Some(closing_at(close))),
            declined(condition),
            "{close} beside {opening}, {closing} and {capital}"
        );
    }
}

#[test]
fn no_crossing_is_not_asked() {
    assert_eq!(multiple(worked(), None), one(not_asked()));
}

#[test]
fn the_valuation_date_answered_with_an_absence_is_answered_absent_carrying_the_date_asked() {
    let reason = the_one_reason(crossing(absence(T), vec![traded(E)], listed(&[])));
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
    assert_eq!(
        the_one_reason(alone).names(),
        &Names::BasisUnproven {
            basis_unproven: BasisUnproven::NotCovered {
                covered_range: CoveredRange {
                    from: T.into(),
                    through: T.into(),
                },
                period_end: E.into(),
                trading_date: T.into(),
            },
        }
    );
}

#[test]
fn the_split_list_absent_is_splits_absent() {
    let crossing = preset_with(no_list());
    assert_eq!(
        multiple(worked(), Some(crossing.clone())),
        one(Reason::splits_absent(&crossing).expect("the split list is an absence"))
    );
}

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
fn a_split_the_day_before_the_period_end_is_twelve() {
    assert_eq!(
        multiple(worked(), Some(preset_with(listed(&["2024-12-30"])))),
        figure("12")
    );
}

/// The rule is asked once, with E the year's end, so a 2-for-1 split dated
/// 2024-06-15 passes it while the opening count, 55, stands on the basis
/// before the split and the closing count, 110, on the basis after. 60 × 55
/// is 3300 and 60 × 110 is 6600, whose mean is 4950, over 500. On one basis
/// the counts are 110 and 110 and the figure `13.2`.
///
/// `docs/adr/market-capitalisation-and-the-equity-multiples.md` names this
/// exposure and leaves it open. A record that closes it changes this figure.
#[test]
fn a_split_before_the_period_end_that_the_rule_does_not_see_mixes_two_bases_in_the_average() {
    let splits = Splits::Listed {
        factors: vec![split_by("2024-06-15", "2.0")],
    };
    assert_eq!(
        multiple(history("55", "110", "300"), Some(preset_with(splits))),
        figure("9.9")
    );
}

#[test]
fn the_paired_closes_trading_date_outside_the_grammar_declines() {
    let paired = crossing(answered(T, "2025-6-30", "60"), vec![traded(E)], listed(&[]));
    assert_eq!(
        multiple(worked(), Some(paired)),
        declined("share_basis_date_not_a_date")
    );
}

#[test]
fn a_close_outside_the_decimal_grammar_declines() {
    assert_eq!(
        multiple(worked(), Some(closing_at("1e2"))),
        declined("input_not_a_decimal")
    );
}

/// 6000 over the 800 a capital expenditure read as zero would leave is
/// written `7.5`.
#[test]
fn a_missing_opening_instant_then_an_unknown_capital_expenditure_then_no_price_and_never_a_figure()
{
    let rows: Vec<Row> = history_in([value("800"), unknown(), value("90"), value("110")])
        .into_iter()
        .filter(|row| row.period() != &instant(OPENING))
        .collect();
    let silent = Reason::input_unknown(&rows[0], CAPITAL)
        .expect("capital expenditure is Unknown at the year");
    assert_eq!(
        silent.names(),
        &Names::InputUnknown {
            concept: CAPITAL,
            period: year(),
        }
    );
    assert_eq!(
        multiple(rows.clone(), None),
        absent(
            no_period("opening_instant"),
            vec![silent.clone(), not_asked()]
        )
    );
    assert_eq!(
        multiple(rows, Some(preset())),
        absent(no_period("opening_instant"), vec![silent])
    );
}

#[test]
fn an_unknown_capital_expenditure_is_never_read_as_zero() {
    let rows = history_in([value("800"), unknown(), value("90"), value("110")]);
    let silent = Reason::input_unknown(&rows[0], CAPITAL)
        .expect("capital expenditure is Unknown at the year");
    let outcome = multiple(rows, Some(preset()));
    assert_eq!(outcome, one(silent));
    assert_ne!(outcome, figure("7.5"));
}

#[test]
fn a_missing_closing_instant_is_named_as_the_instant_it_is() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .filter(|row| row.period() != &instant(E))
        .collect();
    assert_eq!(
        multiple(rows, Some(preset())),
        one(no_period("closing_instant"))
    );
}

#[test]
fn every_read_is_made_in_input_order_and_the_price_read_last() {
    let rows = history_in([unknown(), unknown(), unknown(), unknown()]);
    let unknown_at = |at: usize, concept| {
        Reason::input_unknown(&rows[at], concept).expect("the concept is Unknown at the row")
    };
    let [opening, closing, operating, capital] = [
        unknown_at(1, COUNT),
        unknown_at(2, COUNT),
        unknown_at(0, OPERATING),
        unknown_at(0, CAPITAL),
    ];
    assert_eq!(
        opening.names(),
        &Names::InputUnknown {
            concept: COUNT,
            period: instant(OPENING),
        }
    );
    assert_eq!(
        operating.names(),
        &Names::InputUnknown {
            concept: OPERATING,
            period: year(),
        }
    );
    let crossing = preset_with(no_list());
    assert_eq!(
        multiple(rows.clone(), Some(crossing.clone())),
        absent(
            opening.clone(),
            vec![
                closing.clone(),
                operating.clone(),
                capital.clone(),
                Reason::splits_absent(&crossing).expect("the split list is an absence")
            ]
        )
    );
    assert_eq!(
        multiple(rows.clone(), None),
        absent(opening, vec![closing, operating, capital, not_asked()])
    );
}

#[test]
fn a_count_not_applicable_at_an_instant_is_named_there() {
    let excluded = Concept::ShortTermInvestments
        .definition()
        .applies_to
        .excluding(Kind::Bank)
        .expect("short-term investments do not apply to a bank");
    let rows = history_in([
        value("800"),
        value("300"),
        value("90"),
        Resolution::NotApplicable { excluded },
    ]);
    let not_applicable = Reason::input_not_applicable(&rows[2], COUNT)
        .expect("the count is NotApplicable at the closing instant");
    assert_eq!(multiple(rows, Some(preset())), one(not_applicable));
}

#[test]
fn an_input_outside_the_decimal_grammar_does_not_stop_the_price_read() {
    let crossing = preset_with(no_list());
    for [operating, capital, opening, closing] in [
        ["800", "300", "9e1", "110"],
        ["8e2", "300", "90", "110"],
        ["800", "3e2", "90", "110"],
    ] {
        let rows = history_in([operating, capital, opening, closing].map(value));
        assert_eq!(
            multiple(rows, Some(crossing.clone())),
            absent(
                Reason::declined("input_not_a_decimal".into()),
                vec![Reason::splits_absent(&crossing).expect("the split list is an absence")]
            ),
            "{operating}, {capital}, {opening} and {closing}"
        );
    }
}

/// A count of zero or a free cash flow of zero or below beside no price says
/// only that no price was asked, and a capital expenditure that was not read
/// beside a close of zero says only that.
#[test]
fn no_condition_is_asked_unless_every_read_and_the_price_read_gave_a_number() {
    for [opening, closing, capital] in [
        ["90", "0", "300"],
        ["90", "110", "800"],
        ["90", "110", "900"],
    ] {
        assert_eq!(
            multiple(history(opening, closing, capital), None),
            one(not_asked()),
            "{opening}, {closing} and {capital}"
        );
    }

    let rows = history_in([value("800"), unknown(), value("0"), value("110")]);
    let silent = Reason::input_unknown(&rows[0], CAPITAL)
        .expect("capital expenditure is Unknown at the year");
    assert_eq!(multiple(rows, Some(closing_at("0"))), one(silent));
}

#[test]
fn a_filer_of_each_kind_has_the_same_multiple() {
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
        for (at, concept) in [(0, OPERATING), (0, CAPITAL), (1, COUNT), (2, COUNT)] {
            assert!(
                matches!(rows[at].of(concept), Resolution::Value { .. }),
                "{kind:?} {concept:?}"
            );
        }
        assert_eq!(multiple(rows, Some(preset())), figure("12"), "{kind:?}");
    }
}

#[test]
fn a_quarter_whose_every_input_is_unknown_gives_not_a_year_alone_and_no_read_is_made() {
    let quarter = duration("2024-10-01", E);
    let rows = vec![
        stating(quarter.clone(), unknown()),
        stating(instant("2024-09-30"), unknown()),
        stating(instant(E), unknown()),
    ];
    for prices in [None, Some(preset())] {
        assert_eq!(
            multiple_at(rows.clone(), prices, &quarter),
            declined("not_a_year")
        );
    }
}

#[test]
fn a_year_whose_start_is_outside_the_grammar_gives_start_not_a_date_alone() {
    let unreadable = duration("2024-1-01", E);
    let rows = vec![
        stating(unreadable.clone(), unknown()),
        stating(instant(OPENING), unknown()),
        stating(instant(E), unknown()),
    ];
    for prices in [None, Some(preset())] {
        assert_eq!(
            multiple_at(rows.clone(), prices, &unreadable),
            declined("start_not_a_date")
        );
    }
}

#[test]
fn it_is_stated_at_every_duration_and_at_no_instant() {
    let quarter = duration("2024-10-01", E);
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([
            row(quarter.clone(), [value("7"), value("200"), value("75")]),
            stating(instant("2024-09-30"), value("100")),
        ])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    for at in [OPENING, "2024-09-30", E] {
        assert_eq!(outcome_at(&hand_over, &instant(at)), None, "{at}");
    }
    assert_eq!(outcome_at(&hand_over, &year()), Some(figure("12")));
    assert_eq!(
        outcome_at(&hand_over, &quarter),
        Some(declined("not_a_year"))
    );
}

/// Every other concept states 7, so dividends paid of 7 over the one free
/// cash flow of 500 is `0.014`, over −100 is `-0.07`, and the payout's sign
/// is not this metric's condition. The discounted cash flow value declines
/// at zero and below under its own one condition.
#[test]
fn the_one_free_cash_flow_has_three_readers_and_each_keeps_its_own_conditions() {
    for (capital, multiple, payout, value_declines) in [
        ("300", figure("12"), figure("0.014"), false),
        (
            "800",
            declined("zero_free_cash_flow"),
            declined("zero_free_cash_flow"),
            true,
        ),
        (
            "900",
            declined("negative_free_cash_flow"),
            figure("-0.07"),
            true,
        ),
    ] {
        let hand_over = handed(
            history("90", "110", capital),
            &Settings::preset(),
            Some(preset()),
        );
        assert_eq!(outcome_at(&hand_over, &year()), Some(multiple), "{capital}");
        assert_eq!(
            outcome_of(&hand_over, PAYOUT, &year()),
            Some(payout),
            "{capital}"
        );
        let value = outcome_of(&hand_over, VALUE, &year()).expect("the value is stated at a year");
        assert_eq!(
            value == declined("non_positive_free_cash_flow"),
            value_declines,
            "{capital}: {value:?}"
        );
    }
}

#[test]
fn it_reads_no_setting() {
    let settings = Settings::preset()
        .with_dividend_discount_growth_rate("0.09")
        .expect("a rate above minus one");
    assert_eq!(
        outcome_at(&handed(worked(), &settings, Some(preset())), &year()),
        Some(figure("12"))
    );
}

#[test]
fn adding_the_multiple_adds_no_setting_and_keeps_the_version() {
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
