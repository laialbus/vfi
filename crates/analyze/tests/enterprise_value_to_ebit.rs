//! EV/EBIT as a caller reaches it: a history and a crossing in, and at each
//! year the average of enterprise value at the instants the year opens and
//! closes at over operating income; or the reasons it has none.
//!
//! The figures are the proof of
//! `docs/adr/enterprise-value-and-its-multiples.md`: the year 2024 with
//! operating income of 800, and the instants 2023-12-31 and 2024-12-31 with
//! shares outstanding of 90 and 110, short-term debt of 100 and 200,
//! long-term debt of 900 and 1300, cash and equivalents of 300 and 400 and
//! short-term investments of 100 and 100, beside
//! `docs/adr/share-basis-rule-dates.md`'s preset crossing with a close of 60,
//! computed at the year, unless a test says otherwise. Market capitalisation
//! is 5400 and 6600, net debt 600 and 1000, enterprise value 6000 and 7600,
//! and their average 6800.

use vfi_analyze::{Settings, analyze};
use vfi_contracts::analyze_store::{
    BasisUnproven, CoveredRange, HandOver, Names, NoPrice, Outcome, Prices, Reason, Reasons,
};
use vfi_contracts::canonical_concepts::{
    Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, Rule, SetBy,
};
use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

const CIK: &str = "0000320193";

const NAME: &str = "enterprise_value_to_ebit";

const COUNT: Concept = Concept::SharesOutstanding;

const SHORT_TERM_DEBT: Concept = Concept::ShortTermDebt;

const LONG_TERM_DEBT: Concept = Concept::LongTermDebt;

const CASH: Concept = Concept::CashAndEquivalents;

const INVESTMENTS: Concept = Concept::ShortTermInvestments;

const INCOME: Concept = Concept::OperatingIncome;

/// The five balances, in input order.
const BALANCES: [Concept; 5] = [COUNT, SHORT_TERM_DEBT, LONG_TERM_DEBT, CASH, INVESTMENTS];

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

/// `NotApplicable`, by the witness short-term investments give for a `reit`.
fn not_applicable() -> Resolution {
    Resolution::NotApplicable {
        excluded: INVESTMENTS
            .definition()
            .applies_to
            .excluding(Kind::Reit)
            .expect("short-term investments do not apply to a reit"),
    }
}

/// A row at `period` where each concept is what `stated` gives for it.
fn row(period: Period, stated: impl Fn(Concept) -> Resolution) -> Row {
    Row::at(period, std::array::from_fn(|at| stated(Concept::ALL[at])))
}

/// A row at `period` with every concept as `state`.
fn stating(period: Period, state: Resolution) -> Row {
    row(period, |_| state.clone())
}

/// A row at the instant `at` stating the five balances as given, in input
/// order, and every other concept a value the metric does not read.
fn balances(at: &str, amounts: [&str; 5]) -> Row {
    row(instant(at), |concept| {
        value(
            BALANCES
                .iter()
                .position(|balance| *balance == concept)
                .map_or("7", |balance| amounts[balance]),
        )
    })
}

/// The record's worked history: the year, then the instants it opens and
/// closes at.
fn worked() -> Vec<Row> {
    vec![
        row(year(), |concept| {
            value(if concept == INCOME { "800" } else { "7" })
        }),
        balances(OPENING, ["90", "100", "900", "300", "100"]),
        balances(E, ["110", "200", "1300", "400", "100"]),
    ]
}

/// `rows` with `concept` as `state` at `period`, and nothing else changed.
fn restated(rows: Vec<Row>, period: &Period, concept: Concept, state: Resolution) -> Vec<Row> {
    rows.into_iter()
        .map(|stated| {
            if stated.period() == period {
                row(period.clone(), |other| {
                    if other == concept {
                        state.clone()
                    } else {
                        stated.of(other).clone()
                    }
                })
            } else {
                stated
            }
        })
        .collect()
}

/// `rows` with `concept` as `opening` and `closing` at the year's two
/// instants.
fn at_both(rows: Vec<Row>, concept: Concept, opening: Resolution, closing: Resolution) -> Vec<Row> {
    restated(
        restated(rows, &instant(OPENING), concept, opening),
        &instant(E),
        concept,
        closing,
    )
}

/// The worked history with `concept` as `opening` and `closing` at the
/// year's two instants.
fn worked_with(concept: Concept, opening: &str, closing: &str) -> Vec<Row> {
    at_both(worked(), concept, value(opening), value(closing))
}

/// The worked history with operating income of `amount` over the year.
fn earning(amount: &str) -> Vec<Row> {
    restated(worked(), &year(), INCOME, value(amount))
}

/// `rows` without the instant the year opens at.
fn without_the_opening(rows: Vec<Row>) -> Vec<Row> {
    rows.into_iter()
        .filter(|row| row.period() != &instant(OPENING))
        .collect()
}

/// The worked history as a filer of `kind` states it: every concept v2 does
/// not apply to that kind `NotApplicable`.
fn worked_of(kind: Kind) -> Vec<Row> {
    worked()
        .into_iter()
        .map(|stated| {
            row(stated.period().clone(), |concept| {
                match concept.definition().applies_to.excluding(kind) {
                    Some(excluded) => Resolution::NotApplicable { excluded },
                    None => stated.of(concept).clone(),
                }
            })
        })
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

/// The names of the metrics the results at `period` hold, in their order.
fn names_at(hand_over: &HandOver, period: &Period) -> Vec<Box<str>> {
    hand_over
        .results()
        .iter()
        .find(|results| results.period() == period)
        .expect("each period the history holds has its results")
        .metrics()
        .iter()
        .map(|metric| metric.name.clone())
        .collect()
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

/// The metric at the year beside the preset crossing.
fn priced(rows: Vec<Row>) -> Outcome {
    multiple(rows, Some(preset()))
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

/// `concept` is `Unknown` at the row of `rows` at `period`.
fn silent(rows: &[Row], period: &Period, concept: Concept) -> Reason {
    let row = rows
        .iter()
        .find(|row| row.period() == period)
        .expect("the history holds the period");
    let reason = Reason::input_unknown(row, concept).expect("the concept is Unknown at the row");
    assert_eq!(
        reason.names(),
        &Names::InputUnknown {
            concept,
            period: period.clone(),
        }
    );
    reason
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

/// 6800 over 800. The closing balances alone would give 7600 over 800,
/// `9.5`; the closing count beside the averaged net debt 7400, `9.25`; and
/// the averaged count beside the closing net debt 7000, `8.75`.
#[test]
fn the_average_of_enterprise_value_over_operating_income_and_no_other_reading_of_the_instants() {
    let outcome = priced(worked());
    assert_eq!(outcome, figure("8.5"));
    for other in ["9.5", "9.25", "8.75"] {
        assert_ne!(outcome, figure(other), "{other}");
    }
}

/// Net debt is 700 and 1100, and enterprise value 6100 and 7700, whose
/// average is 6900.
#[test]
fn short_term_investments_not_applicable_at_both_instants_contribute_nothing() {
    let rows = at_both(worked(), INVESTMENTS, not_applicable(), not_applicable());
    assert_eq!(priced(rows), figure("8.625"));
}

/// Net debt is 700 and 1000, and enterprise value 6100 and 7600, whose
/// average is 6850. Counting the one read that gave a value would average
/// short-term investments at 100 and give `8.5`.
#[test]
fn an_instant_where_short_term_investments_contribute_nothing_is_still_counted() {
    let rows = at_both(worked(), INVESTMENTS, not_applicable(), value("100"));
    let outcome = priced(rows);
    assert_eq!(outcome, figure("8.5625"));
    assert_ne!(outcome, figure("8.5"));
}

/// The exception is one concept's. Any other balance `NotApplicable` at an
/// instant is named there and contributes no zero.
#[test]
fn no_other_balance_contributes_nothing_where_it_is_not_applicable() {
    for concept in [COUNT, SHORT_TERM_DEBT, LONG_TERM_DEBT, CASH] {
        let rows = restated(worked(), &instant(E), concept, not_applicable());
        let closing = &rows[2];
        let reason = Reason::input_not_applicable(closing, concept)
            .expect("the concept is NotApplicable at the closing instant");
        assert_eq!(priced(rows.clone()), one(reason), "{concept:?}");
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

/// A zero count averaged in would leave 60 × 45 or 60 × 55 beside the
/// averaged net debt of 800, a figure that looks real.
#[test]
fn either_count_at_zero_or_below_declines_and_is_never_averaged_in() {
    for (opening, closing) in [("90", "0"), ("0", "110"), ("90", "-110"), ("-90", "110")] {
        assert_eq!(
            priced(worked_with(COUNT, opening, closing)),
            declined("non_positive_shares_outstanding"),
            "{opening} and {closing}"
        );
    }
}

/// Cash of 7900 and 6400 gives net debt of −7000 and −5000 and enterprise
/// value of −1600 and 1600, whose average is 0. Cash of 7900 and 6600 gives
/// −1600 and 1400, whose average is −100, and over 800 that would be written
/// `-0.125`.
#[test]
fn an_averaged_enterprise_value_of_zero_or_below_declines_and_is_never_a_signed_multiple() {
    for closing in ["6400", "6600"] {
        let outcome = priced(worked_with(CASH, "7900", closing));
        assert_eq!(
            outcome,
            declined("non_positive_enterprise_value"),
            "{closing}"
        );
        assert_ne!(outcome, figure("-0.125"), "{closing}");
        assert_ne!(outcome, figure("0"), "{closing}");
    }
}

/// Cash of 7900 and 6200 gives enterprise value of −1600 and 1800, whose
/// average is 100.
#[test]
fn the_condition_is_on_the_average_so_one_instant_below_zero_still_gives_a_figure() {
    assert_eq!(priced(worked_with(CASH, "7900", "6200")), figure("0.125"));
}

#[test]
fn zero_operating_income_declines_as_a_zero_divisor() {
    assert_eq!(priced(earning("0")), declined("zero_operating_income"));
}

/// 6800 over −800 would be written `-8.5`, below every real multiple.
#[test]
fn negative_operating_income_declines_and_is_never_a_signed_multiple() {
    for income in ["-100", "-800"] {
        let outcome = priced(earning(income));
        assert_eq!(outcome, declined("negative_operating_income"), "{income}");
        assert_ne!(outcome, figure("-8.5"), "{income}");
    }
}

/// The conditions are asked in formula order: the close, the count, the
/// averaged enterprise value, then operating income at zero and below it.
#[test]
fn the_first_condition_that_holds_is_the_one_reason() {
    let zero_count = || worked_with(COUNT, "90", "0");
    let no_enterprise_value = || worked_with(CASH, "7900", "6400");
    let earning_nothing = |rows| restated(rows, &year(), INCOME, value("0"));
    let losing = |rows| restated(rows, &year(), INCOME, value("-100"));
    for (close, rows, condition) in [
        ("0", earning_nothing(worked()), "non_positive_close"),
        ("0", zero_count(), "non_positive_close"),
        ("-1", losing(no_enterprise_value()), "non_positive_close"),
        (
            "60",
            at_both(zero_count(), CASH, value("7900"), value("6400")),
            "non_positive_shares_outstanding",
        ),
        (
            "60",
            earning_nothing(zero_count()),
            "non_positive_shares_outstanding",
        ),
        (
            "60",
            earning_nothing(no_enterprise_value()),
            "non_positive_enterprise_value",
        ),
        (
            "60",
            losing(no_enterprise_value()),
            "non_positive_enterprise_value",
        ),
    ] {
        assert_eq!(
            multiple(rows, Some(closing_at(close))),
            declined(condition),
            "{close} and {condition}"
        );
    }
}

#[test]
fn no_crossing_is_not_asked() {
    let outcome = multiple(worked(), None);
    assert_eq!(outcome, one(not_asked()));
    assert_eq!(
        not_asked().names(),
        &Names::NoPrice {
            no_price: NoPrice::NotAsked {}
        }
    );
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
fn a_split_the_day_before_the_period_end_gives_the_figure() {
    assert_eq!(
        multiple(worked(), Some(preset_with(listed(&["2024-12-30"])))),
        figure("8.5")
    );
}

/// The rule is asked once, with E the year's end, so a 2-for-1 split dated
/// 2024-06-15 passes it while the opening count, 55, stands on the basis
/// before the split and the closing count, 110, on the basis after.
/// Enterprise value is 60 × 55 + 600, 3900, and 7600, whose average is 5750,
/// over 800. On one basis the counts are 110 and 110, enterprise value 7200
/// and 7600, and the figure `9.25`.
///
/// `docs/adr/enterprise-value-and-its-multiples.md` names this exposure,
/// which reaches it through the market capitalisation term, and leaves it
/// open. A record that closes it changes this figure.
#[test]
fn a_split_before_the_period_end_that_the_rule_does_not_see_mixes_two_bases_in_the_average() {
    let splits = || Splits::Listed {
        factors: vec![split_by("2024-06-15", "2.0")],
    };
    assert_eq!(
        multiple(worked_with(COUNT, "55", "110"), Some(preset_with(splits()))),
        figure("7.1875")
    );
    assert_eq!(
        multiple(
            worked_with(COUNT, "110", "110"),
            Some(preset_with(splits()))
        ),
        figure("9.25")
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

/// A debt read as zero at the closing instant would leave net debt of 800
/// or −300 there, and cash read as zero 1400, each a figure that looks real.
#[test]
fn a_filer_silent_on_a_debt_or_on_cash_has_no_figure_and_the_silence_is_never_read_as_zero() {
    for (concept, as_zero) in [
        (SHORT_TERM_DEBT, "8.375"),
        (LONG_TERM_DEBT, "7.6875"),
        (CASH, "8.75"),
    ] {
        let rows = restated(worked(), &instant(E), concept, unknown());
        let outcome = priced(rows.clone());
        assert_eq!(
            outcome,
            one(silent(&rows, &instant(E), concept)),
            "{concept:?}"
        );
        assert_ne!(outcome, figure(as_zero), "{concept:?}");
    }
}

/// A filer silent on short-term investments has a zero there, supplied
/// before analysis: net debt 700 and 1100. An `Unknown` is not that zero.
#[test]
fn short_term_investments_of_zero_net_cash_alone_and_an_unknown_names_its_instant() {
    assert_eq!(priced(worked_with(INVESTMENTS, "0", "0")), figure("8.625"));

    let rows = restated(worked(), &instant(E), INVESTMENTS, unknown());
    assert_eq!(
        priced(rows.clone()),
        one(silent(&rows, &instant(E), INVESTMENTS))
    );
}

#[test]
fn the_preferred_claim_is_not_read() {
    let preferred = Concept::PreferredEquity;
    assert_eq!(
        priced(at_both(worked(), preferred, unknown(), unknown())),
        figure("8.5")
    );
    assert_eq!(
        priced(at_both(worked(), preferred, value("500"), value("500"))),
        figure("8.5")
    );
}

#[test]
fn an_operating_filer_has_the_figure_and_a_reit_the_figure_without_short_term_investments() {
    assert_eq!(priced(worked_of(Kind::Operating)), figure("8.5"));

    let reit = worked_of(Kind::Reit);
    for at in [1, 2] {
        assert!(matches!(
            reit[at].of(INVESTMENTS),
            Resolution::NotApplicable { .. }
        ));
    }
    assert_eq!(priced(reit), figure("8.625"));
}

#[test]
fn a_bank_an_insurer_and_an_investment_company_are_absent_for_operating_income_alone() {
    assert_eq!(Kind::ALL.len(), 5);
    for kind in [Kind::Bank, Kind::Insurer, Kind::InvestmentCompany] {
        let rows = worked_of(kind);
        for at in [1, 2] {
            assert!(
                matches!(rows[at].of(INVESTMENTS), Resolution::NotApplicable { .. }),
                "{kind:?}"
            );
        }
        let reason = Reason::input_not_applicable(&rows[0], INCOME)
            .expect("operating income is NotApplicable at the year");
        assert_eq!(
            reason.names(),
            &Names::InputNotApplicable {
                concept: INCOME,
                period: year(),
            }
        );
        assert_eq!(priced(rows), one(reason), "{kind:?}");
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
fn a_missing_opening_instant_is_named_once_for_each_balance_and_the_price_read_comes_last() {
    let rows = without_the_opening(worked());
    assert_eq!(
        multiple(rows, None),
        absent(
            no_period("opening_instant"),
            BALANCES[1..]
                .iter()
                .map(|_| no_period("opening_instant"))
                .chain([not_asked()])
                .collect()
        )
    );
}

#[test]
fn an_unknown_operating_income_stands_between_the_balances_reasons_and_the_price_reads() {
    let rows = without_the_opening(restated(worked(), &year(), INCOME, unknown()));
    let income = silent(&rows, &year(), INCOME);
    assert_eq!(
        multiple(rows.clone(), None),
        absent(
            no_period("opening_instant"),
            BALANCES[1..]
                .iter()
                .map(|_| no_period("opening_instant"))
                .chain([income.clone(), not_asked()])
                .collect()
        )
    );
    assert_eq!(
        priced(rows),
        absent(
            no_period("opening_instant"),
            BALANCES[1..]
                .iter()
                .map(|_| no_period("opening_instant"))
                .chain([income])
                .collect()
        )
    );
}

#[test]
fn a_missing_closing_instant_is_named_as_the_instant_it_is() {
    let rows: Vec<Row> = worked()
        .into_iter()
        .filter(|row| row.period() != &instant(E))
        .collect();
    assert_eq!(
        priced(rows),
        absent(
            no_period("closing_instant"),
            BALANCES[1..]
                .iter()
                .map(|_| no_period("closing_instant"))
                .collect()
        )
    );
}

/// Eleven reads: each balance at the opening instant and then the closing,
/// in input order, then operating income at the year.
#[test]
fn every_read_is_made_in_input_order_and_the_price_read_last() {
    let rows = vec![
        stating(year(), unknown()),
        stating(instant(OPENING), unknown()),
        stating(instant(E), unknown()),
    ];
    let mut reads: Vec<Reason> = BALANCES
        .iter()
        .flat_map(|balance| [OPENING, E].map(|at| silent(&rows, &instant(at), *balance)))
        .chain([silent(&rows, &year(), INCOME)])
        .collect();
    assert_eq!(reads.len(), 11);
    let first = reads.remove(0);

    let crossing = preset_with(no_list());
    let unproven = Reason::splits_absent(&crossing).expect("the split list is an absence");
    assert_eq!(
        multiple(rows.clone(), Some(crossing)),
        absent(
            first.clone(),
            reads.iter().cloned().chain([unproven]).collect()
        )
    );
    assert_eq!(
        multiple(rows, None),
        absent(first, reads.into_iter().chain([not_asked()]).collect())
    );
}

#[test]
fn an_input_outside_the_decimal_grammar_does_not_stop_the_price_read() {
    let crossing = preset_with(no_list());
    let unproven = || Reason::splits_absent(&crossing).expect("the split list is an absence");
    let not_a_decimal = || Reason::declined("input_not_a_decimal".into());
    for concept in BALANCES {
        let rows = restated(worked(), &instant(OPENING), concept, value("1e2"));
        assert_eq!(
            multiple(rows, Some(crossing.clone())),
            absent(not_a_decimal(), vec![unproven()]),
            "{concept:?}"
        );
    }
    assert_eq!(
        multiple(earning("8e2"), Some(crossing.clone())),
        absent(not_a_decimal(), vec![unproven()])
    );
}

/// A count of zero, an enterprise value of zero or an operating income of
/// zero or below beside no price says only that no price was asked, and a
/// debt that was not read beside a close of zero says only that.
#[test]
fn no_condition_is_asked_unless_every_read_and_the_price_read_gave_a_number() {
    for rows in [
        worked_with(COUNT, "90", "0"),
        worked_with(CASH, "7900", "6400"),
        earning("0"),
        earning("-800"),
    ] {
        assert_eq!(multiple(rows, None), one(not_asked()));
    }

    let rows = restated(earning("0"), &instant(E), SHORT_TERM_DEBT, unknown());
    assert_eq!(
        multiple(rows.clone(), Some(closing_at("0"))),
        one(silent(&rows, &instant(E), SHORT_TERM_DEBT))
    );
}

#[test]
fn it_is_stated_at_every_duration_and_at_no_instant() {
    let quarter = duration("2024-10-01", E);
    let rows: Vec<Row> = worked()
        .into_iter()
        .chain([
            stating(quarter.clone(), value("200")),
            stating(instant("2024-09-30"), value("100")),
        ])
        .collect();
    let hand_over = handed(rows, &Settings::preset(), Some(preset()));
    for at in [OPENING, "2024-09-30", E] {
        assert_eq!(outcome_at(&hand_over, &instant(at)), None, "{at}");
    }
    assert_eq!(outcome_at(&hand_over, &year()), Some(figure("8.5")));
    assert_eq!(
        outcome_at(&hand_over, &quarter),
        Some(declined("not_a_year"))
    );
}

#[test]
fn enterprise_value_is_handed_over_under_no_name_of_its_own() {
    let hand_over = handed(worked(), &Settings::preset(), Some(preset()));
    for period in [year(), instant(OPENING), instant(E)] {
        let holding: Vec<Box<str>> = names_at(&hand_over, &period)
            .into_iter()
            .filter(|name| name.contains("enterprise_value"))
            .collect();
        let expected: &[&str] = if period == year() { &[NAME] } else { &[] };
        assert_eq!(
            holding.iter().map(|name| &**name).collect::<Vec<_>>(),
            expected,
            "{period:?}"
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
        Some(figure("8.5"))
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
