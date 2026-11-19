//! Price/earnings as a caller reaches it: a history and a crossing in, and at
//! each year the close at the valuation date over diluted earnings per share;
//! or the reasons it has none.
//!
//! The figures are the proof of
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`: the year
//! 2024 with diluted earnings per share of 4, beside
//! `docs/adr/share-basis-rule-dates.md`'s preset crossing with a close of 60,
//! computed at the year, unless a test says otherwise.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const NAME: &str = "price_to_earnings_per_share";

const EARNINGS: Concept = Concept::EarningsPerShareDiluted;

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

/// A row at `period` with `earnings_per_share_diluted` as `earnings`, and
/// every other concept a value the metric does not read.
fn row(period: Period, earnings: Resolution) -> Row {
    Row::at(
        period,
        std::array::from_fn(|at| {
            if Concept::ALL[at] == EARNINGS {
                earnings.clone()
            } else {
                value("7")
            }
        }),
    )
}

/// The year alone, with diluted earnings per share of `amount`.
fn earning(amount: &str) -> Vec<Row> {
    vec![row(year(), value(amount))]
}

/// The record's worked history: the year with diluted earnings per share of 4.
fn worked() -> Vec<Row> {
    earning("4")
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

/// 60 ÷ 4.
#[test]
fn the_close_over_the_years_diluted_earnings_per_share_is_fifteen() {
    assert_eq!(multiple(worked(), Some(preset())), figure("15"));
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
fn zero_earnings_per_share_decline_as_a_zero_divisor() {
    assert_eq!(
        multiple(earning("0"), Some(preset())),
        declined("zero_earnings_per_share_diluted")
    );
}

/// 60 ÷ −4 would be written `-15`, below every real multiple.
#[test]
fn negative_earnings_per_share_decline_and_are_never_a_signed_multiple() {
    assert_eq!(
        multiple(earning("-4"), Some(preset())),
        declined("negative_earnings_per_share_diluted")
    );
}

/// The close is asked first, so a close of zero or below is the one reason
/// whatever the figure's sign: −60 ÷ −4 is never written as `15`.
#[test]
fn the_first_condition_that_holds_is_the_one_reason() {
    for (close, earnings) in [("0", "0"), ("0", "-4"), ("-60", "-4")] {
        assert_eq!(
            multiple(earning(earnings), Some(closing_at(close))),
            declined("non_positive_close"),
            "{close} over {earnings}"
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
fn a_split_the_day_before_the_period_end_is_fifteen() {
    assert_eq!(
        multiple(worked(), Some(preset_with(listed(&["2024-12-30"])))),
        figure("15")
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

#[test]
fn the_reads_reason_comes_first_and_the_price_reads_follows() {
    let rows = vec![row(year(), unknown())];
    let unknown_at_the_year =
        Reason::input_unknown(&rows[0], EARNINGS).expect("the figure is Unknown at the year");
    assert_eq!(
        multiple(rows, None),
        absent(unknown_at_the_year, vec![not_asked()])
    );
}

#[test]
fn a_figure_outside_the_decimal_grammar_does_not_stop_the_price_read() {
    let crossing = preset_with(no_list());
    assert_eq!(
        multiple(earning("4e0"), Some(crossing.clone())),
        absent(
            Reason::declined("input_not_a_decimal".into()),
            vec![Reason::splits_absent(&crossing).expect("the split list is an absence")]
        )
    );
}

/// A figure of zero beside no price says only that no price was asked, and a
/// figure that was not read beside a close of zero says only that.
#[test]
fn no_condition_is_asked_unless_both_reads_gave_a_number() {
    assert_eq!(multiple(earning("0"), None), one(not_asked()));
    assert_eq!(multiple(earning("-4"), None), one(not_asked()));

    let rows = vec![row(year(), unknown())];
    let unknown_at_the_year =
        Reason::input_unknown(&rows[0], EARNINGS).expect("the figure is Unknown at the year");
    assert_eq!(
        multiple(rows, Some(closing_at("0"))),
        one(unknown_at_the_year)
    );
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
        assert!(
            matches!(rows[0].of(EARNINGS), Resolution::Value { .. }),
            "{kind:?}"
        );
        assert_eq!(multiple(rows, Some(preset())), figure("15"), "{kind:?}");
    }
}

#[test]
fn a_quarter_whose_figure_is_unknown_gives_not_a_year_alone_and_no_read_is_made() {
    let quarter = duration("2024-10-01", E);
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([row(quarter.clone(), unknown())])
        .collect();
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
    for prices in [None, Some(preset())] {
        assert_eq!(
            multiple_at(
                vec![row(unreadable.clone(), unknown())],
                prices,
                &unreadable
            ),
            declined("start_not_a_date")
        );
    }
}

#[test]
fn it_is_stated_at_every_duration_and_at_no_instant() {
    let instant = Period::Instant { at: E.into() };
    let quarter = duration("2024-10-01", E);
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([
            row(quarter.clone(), value("1")),
            row(instant.clone(), value("4")),
        ])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    assert_eq!(outcome_at(&hand_over, &instant), None);
    assert_eq!(outcome_at(&hand_over, &year()), Some(figure("15")));
    assert_eq!(
        outcome_at(&hand_over, &quarter),
        Some(declined("not_a_year"))
    );
}

#[test]
fn it_reads_no_setting() {
    let settings = Settings::preset()
        .with_dividend_discount_growth_rate("0.09")
        .expect("a rate above minus one");
    assert_eq!(
        outcome_at(&handed(worked(), &settings, Some(preset())), &year()),
        Some(figure("15"))
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
