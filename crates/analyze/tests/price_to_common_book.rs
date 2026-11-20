//! Price/book as a caller reaches it: a history and a crossing in, and at
//! each instant market capitalisation over common equity; or the reasons it
//! has none.
//!
//! The figures are the proof of
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`: the instants
//! 2023-12-31 and 2024-12-31 with shares outstanding of 90 and 110,
//! shareholders' equity of 2000 and 2400 and preferred equity of 200 and 200,
//! beside `docs/adr/share-basis-rule-dates.md`'s preset crossing with a close
//! of 60, computed at 2024-12-31, unless a test says otherwise.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const NAME: &str = "price_to_common_book";

const COUNT: Concept = Concept::SharesOutstanding;

const EQUITY: Concept = Concept::ShareholdersEquity;

const PREFERRED: Concept = Concept::PreferredEquity;

const OPENING: &str = "2023-12-31";

const E: &str = "2024-12-31";

const QUARTER_END: &str = "2024-09-30";

const T: &str = "2025-06-30";

fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
}

fn year() -> Period {
    Period::Duration {
        start: "2024-01-01".into(),
        end: E.into(),
    }
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

/// A row at `period` with the three balances the metric reads as given, in
/// input order, and every other concept a value it does not read.
fn row(period: Period, [count, equity, preferred]: [Resolution; 3]) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| match Concept::ALL[at] {
            COUNT => count.clone(),
            EQUITY => equity.clone(),
            PREFERRED => preferred.clone(),
            _ => value("7"),
        }),
    )
}

/// The instant dated `at` with shares outstanding, shareholders' equity and
/// preferred equity as given.
fn balances(at: &str, amounts: [&str; 3]) -> Row {
    row(instant(at), amounts.map(value))
}

fn opening() -> Row {
    balances(OPENING, ["90", "2000", "200"])
}

/// The worked history with the instant 2024-12-31 holding `amounts`.
fn closing_with(amounts: [&str; 3]) -> Vec<Row> {
    vec![opening(), balances(E, amounts)]
}

/// The worked history with the instant 2024-12-31 holding `states`.
fn closing_in(states: [Resolution; 3]) -> Vec<Row> {
    vec![opening(), row(instant(E), states)]
}

/// The record's worked history, opening then closing.
fn worked() -> Vec<Row> {
    closing_with(["110", "2400", "200"])
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

/// The metric's outcome at `period`, or none where the results there do not
/// hold it.
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

/// The metric at the instant dated `at`, from a history of `rows` beside
/// `prices`, under the preset.
fn multiple_at(rows: Vec<Row>, prices: Option<Crossing>, at: &str) -> Outcome {
    outcome_at(&handed(rows, &Settings::preset(), prices), &instant(at))
        .expect("the metric is stated at the instant given")
}

/// The metric at 2024-12-31.
fn multiple(rows: Vec<Row>, prices: Option<Crossing>) -> Outcome {
    multiple_at(rows, prices, E)
}

/// The one reason the metric at the instant dated `at` is absent for.
fn the_one_reason_at(rows: Vec<Row>, prices: Crossing, at: &str) -> Reason {
    let Outcome::Absent { reasons } = multiple_at(rows, Some(prices), at) else {
        panic!("the metric is absent");
    };
    let mut each = reasons.each();
    let first = each.next().expect("an absence has a reason").clone();
    assert_eq!(each.next(), None);
    first
}

/// The one reason the metric at 2024-12-31 of the worked history beside
/// `prices` is absent for.
fn the_one_reason(prices: Crossing) -> Reason {
    the_one_reason_at(worked(), prices, E)
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

/// The prices answered trade `from` through `through`, which does not reach
/// both `period_end` and the valuation date's close.
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

/// The split dated `date`, by its date and factor, stands on or after
/// 2024-12-31.
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

/// 60 × 110 is 6600, and 2400 less 200 is 2200.
#[test]
fn market_capitalisation_over_common_equity_is_three() {
    assert_eq!(multiple(worked(), Some(preset())), figure("3"));
}

/// 6600 over 2400. A preferred equity of zero is a number that was read.
#[test]
fn a_zero_preferred_equity_leaves_the_whole_of_shareholders_equity() {
    assert_eq!(
        multiple(closing_with(["110", "2400", "0"]), Some(preset())),
        figure("2.75")
    );
}

/// The caller asked no date at or before 2023-12-31, so its E is outside the
/// range the prices cover: 60 × 90 over 1800 is never written as `3`.
#[test]
fn the_earlier_instant_is_not_covered_and_never_a_figure() {
    assert_eq!(
        the_one_reason_at(worked(), preset(), OPENING).names(),
        &not_covered(E, T, OPENING)
    );
}

#[test]
fn a_quarter_end_instant_is_computed_at_once_a_price_at_or_before_it_is_answered() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([balances(QUARTER_END, ["110", "2400", "200"])])
        .collect();
    assert_eq!(
        the_one_reason_at(rows.clone(), preset(), QUARTER_END).names(),
        &not_covered(E, T, QUARTER_END)
    );

    let reaching = crossing(traded(T), vec![traded(E), traded(QUARTER_END)], listed(&[]));
    assert_eq!(multiple_at(rows, Some(reaching), QUARTER_END), figure("3"));
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

/// 0 over 2200 would be written `0`, the multiple of a filer that is free.
#[test]
fn a_count_of_zero_or_below_declines_and_is_never_a_multiple_of_zero() {
    for count in ["0", "-110"] {
        assert_eq!(
            multiple(closing_with([count, "2400", "200"]), Some(preset())),
            declined("non_positive_shares_outstanding"),
            "{count}"
        );
    }
}

#[test]
fn only_the_instants_own_row_is_read() {
    let zero_before = vec![
        balances(OPENING, ["0", "2000", "200"]),
        balances(E, ["110", "2400", "200"]),
    ];
    assert_eq!(multiple(zero_before, Some(preset())), figure("3"));

    let silent_before = vec![
        row(instant(OPENING), [unknown(), unknown(), unknown()]),
        balances(E, ["110", "2400", "200"]),
    ];
    assert_eq!(multiple(silent_before, Some(preset())), figure("3"));
}

#[test]
fn a_zero_common_equity_declines_as_a_zero_divisor() {
    assert_eq!(
        multiple(closing_with(["110", "200", "200"]), Some(preset())),
        declined("zero_common_equity")
    );
}

/// 6600 over −100 would be written `-66`, below every real multiple.
#[test]
fn a_negative_common_equity_declines_and_is_never_a_signed_multiple() {
    assert_eq!(
        multiple(closing_with(["110", "100", "200"]), Some(preset())),
        declined("negative_common_equity")
    );
}

/// The conditions are asked in formula order: the close, the count, then the
/// common equity at zero and below it.
#[test]
fn the_first_condition_that_holds_is_the_one_reason() {
    for (close, amounts, condition) in [
        ("0", ["0", "2400", "200"], "non_positive_close"),
        ("0", ["110", "200", "200"], "non_positive_close"),
        ("-60", ["110", "100", "200"], "non_positive_close"),
        ("60", ["0", "200", "200"], "non_positive_shares_outstanding"),
        (
            "60",
            ["-110", "100", "200"],
            "non_positive_shares_outstanding",
        ),
    ] {
        assert_eq!(
            multiple(closing_with(amounts), Some(closing_at(close))),
            declined(condition),
            "{close} beside {amounts:?}"
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
    assert_eq!(the_one_reason(alone).names(), &not_covered(T, T, E));
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
fn a_split_dated_the_instant_is_split_within() {
    let reason = the_one_reason(preset_with(listed(&[E])));
    assert_eq!(reason.names(), &split_within(E));
}

#[test]
fn a_split_the_day_before_the_instant_is_three() {
    assert_eq!(
        multiple(worked(), Some(preset_with(listed(&["2024-12-30"])))),
        figure("3")
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

/// E is the instant's own date, read through the one grammar.
#[test]
fn an_instant_dated_outside_the_grammar_declines() {
    let unreadable = "2024-12-1";
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([balances(unreadable, ["110", "2400", "200"])])
        .collect();
    assert_eq!(
        multiple_at(rows, Some(preset()), unreadable),
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

#[test]
fn a_filer_silent_on_preferred_equity_has_no_multiple_and_the_price_reads_reason_follows() {
    let rows = closing_in([value("110"), value("2400"), unknown()]);
    let silent = Reason::input_unknown(&rows[1], PREFERRED)
        .expect("preferred equity is Unknown at the instant");
    assert_eq!(
        silent.names(),
        &Names::InputUnknown {
            concept: PREFERRED,
            period: instant(E),
        }
    );
    assert_eq!(multiple(rows.clone(), Some(preset())), one(silent.clone()));
    assert_eq!(multiple(rows, None), absent(silent, vec![not_asked()]));
}

#[test]
fn every_read_is_made_in_input_order_and_the_price_read_last() {
    let rows = closing_in([unknown(), unknown(), unknown()]);
    let [count, equity, preferred] = [COUNT, EQUITY, PREFERRED].map(|concept| {
        Reason::input_unknown(&rows[1], concept).expect("the concept is Unknown at the instant")
    });
    let crossing = preset_with(no_list());
    assert_eq!(
        multiple(rows.clone(), Some(crossing.clone())),
        absent(
            count.clone(),
            vec![
                equity.clone(),
                preferred.clone(),
                Reason::splits_absent(&crossing).expect("the split list is an absence")
            ]
        )
    );
    assert_eq!(
        multiple(rows, None),
        absent(count, vec![equity, preferred, not_asked()])
    );
}

#[test]
fn a_concept_not_applicable_at_the_instant_is_named_there() {
    let excluded = Concept::ShortTermInvestments
        .definition()
        .applies_to
        .excluding(Kind::Bank)
        .expect("short-term investments do not apply to a bank");
    let rows = closing_in([
        Resolution::NotApplicable { excluded },
        value("2400"),
        value("200"),
    ]);
    let not_applicable = Reason::input_not_applicable(&rows[1], COUNT)
        .expect("the count is NotApplicable at the instant");
    assert_eq!(multiple(rows, Some(preset())), one(not_applicable));
}

#[test]
fn a_balance_outside_the_decimal_grammar_does_not_stop_the_price_read() {
    let crossing = preset_with(no_list());
    assert_eq!(
        multiple(closing_with(["110", "24e2", "200"]), Some(crossing.clone())),
        absent(
            Reason::declined("input_not_a_decimal".into()),
            vec![Reason::splits_absent(&crossing).expect("the split list is an absence")]
        )
    );
}

/// A count of zero beside no price says only that no price was asked, and a
/// balance that was not read beside a close of zero says only that.
#[test]
fn no_condition_is_asked_unless_every_read_and_the_price_read_gave_a_number() {
    for amounts in [
        ["0", "2400", "200"],
        ["110", "200", "200"],
        ["110", "100", "200"],
    ] {
        assert_eq!(
            multiple(closing_with(amounts), None),
            one(not_asked()),
            "{amounts:?}"
        );
    }

    let rows = closing_in([value("0"), value("200"), unknown()]);
    let silent = Reason::input_unknown(&rows[1], PREFERRED)
        .expect("preferred equity is Unknown at the instant");
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
        for concept in [COUNT, EQUITY, PREFERRED] {
            assert!(
                matches!(rows[1].of(concept), Resolution::Value { .. }),
                "{kind:?} {concept:?}"
            );
        }
        assert_eq!(multiple(rows, Some(preset())), figure("3"), "{kind:?}");
    }
}

#[test]
fn it_is_stated_at_every_instant_and_at_no_duration() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([
            balances(QUARTER_END, ["110", "2400", "200"]),
            row(year(), [value("110"), value("2400"), value("200")]),
        ])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    assert_eq!(outcome_at(&hand_over, &year()), None);
    assert_eq!(outcome_at(&hand_over, &instant(E)), Some(figure("3")));
    for at in [OPENING, QUARTER_END] {
        assert!(
            matches!(
                outcome_at(&hand_over, &instant(at)),
                Some(Outcome::Absent { .. })
            ),
            "{at}"
        );
    }
}

#[test]
fn it_reads_no_setting() {
    let settings = Settings::preset()
        .with_dividend_discount_growth_rate("0.09")
        .expect("a rate above minus one");
    assert_eq!(
        outcome_at(&handed(worked(), &settings, Some(preset())), &instant(E)),
        Some(figure("3"))
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
