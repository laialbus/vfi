//! What one concept comes to at one period over every filing that answers it.
//!
//! The hand-written cases each pin one thing `docs/adr/which-filing-sets-the-value.md`
//! or `docs/adr/silence-beside-a-read-figure.md` says the rule does, over facts
//! written for the case: no merged fixture holds a filing that reaches any of
//! the three silence-read concepts, a same-day tie, or an assertion. The
//! committed registry reads them all, under the filer the merged fixture
//! records, whose kind is `operating`.
//!
//! The fixture cases re-derive what the two records claim about the merged
//! fetch fixtures, and pin it.

mod fixture;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Kind, Silence};
use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::answering::Admits;
use vfi_normalize::filings;
use vfi_normalize::registry::Registry;
use vfi_normalize::settling::{self, SetBy, Settled, Value};
use vfi_normalize::standing::{Stands, Undecided, stands};

use fixture::{
    FILER, committed, duration, instant, overriding, planted, read, reported, reported_in,
};

const KIND: Option<Kind> = Some(Kind::Operating);

/// A balance sheet date and a quarter no merged fixture reaches.
const AT: &str = "2030-06-30";
const QUARTER: (&str, &str) = ("2030-04-01", "2030-06-30");

const EARLIER: &str = "0000000001-30-000001";
const LATER: &str = "0000000001-31-000001";

/// Sorts ahead of both, so a case it wins is not won by the order accessions
/// arrive in.
const FIRST_BY_ACCESSION: &str = "0000000000-31-000001";

const EARLIER_FILED: &str = "2030-08-01";
const LATER_FILED: &str = "2031-08-01";

fn fact(tag: &str, unit: &str, period: &Period, value: &str, filing: (&str, &str, &str)) -> Fact {
    let (accession, form, filed) = filing;
    Fact {
        taxonomy: "us-gaap".into(),
        tag: tag.into(),
        unit: unit.into(),
        period: period.clone(),
        value: value.into(),
        accession: accession.into(),
        form: form.into(),
        filed: filed.into(),
        report_period_end: "".into(),
    }
}

fn earlier() -> (&'static str, &'static str, &'static str) {
    (EARLIER, "10-Q", EARLIER_FILED)
}

fn later() -> (&'static str, &'static str, &'static str) {
    (LATER, "10-Q", LATER_FILED)
}

/// Filed the day the later filing was, under a form that is not its with `/A`
/// appended, so Rule 3 can order the two by neither `filed` nor `form`.
fn same_day() -> (&'static str, &'static str, &'static str) {
    (FIRST_BY_ACCESSION, "10-K", LATER_FILED)
}

/// A fact under an element no rule of the three silence-read concepts names,
/// which makes its filing answer the period and reach none of them.
fn silent(period: &Period, filing: (&str, &str, &str)) -> Fact {
    let tag = match period {
        Period::Instant { .. } => "Assets",
        Period::Duration { .. } => "NetCashProvidedByUsedInFinancingActivities",
    };
    fact(tag, "USD", period, "900", filing)
}

fn standing<'r, 'f>(
    registry: &'r Registry,
    concept: Concept,
    period: &Period,
    facts: &'f [Fact],
) -> Stands<'r, 'f> {
    stands(registry, FILER, KIND, concept, period, facts).expect("a filing answers the period")
}

/// The value that stands, with the filing each fact it read was reported in.
fn value<'s, 'r, 'f>(stood: &'s Stands<'r, 'f>) -> &'s Value<'r, 'f> {
    match stood {
        Stands::Value(value) => value,
        other => panic!("expected a value, got {other:?}"),
    }
}

fn read_from<'f>(value: &Value<'_, 'f>) -> Vec<&'f str> {
    match value.set_by() {
        SetBy::Rule { facts, .. } => facts.iter().map(|fact| &*fact.accession).collect(),
        other => panic!("expected a value a rule read, got {other:?}"),
    }
}

fn is_silence(value: &Value) -> bool {
    matches!(value.set_by(), SetBy::Silence { .. })
}

/// What each answering filing came to by itself, as the per-filing procedure
/// answers: the case's premise, checked before its conclusion.
fn per_filing(
    registry: &Registry,
    concept: Concept,
    period: &Period,
    facts: &[Fact],
) -> Vec<(String, String)> {
    let answering = filings::answering(facts, period);
    filings::attempted(registry, FILER, KIND, concept, period, &answering)
        .iter()
        .map(|held| {
            let came_to = match held.settled() {
                Settled::Value(value) if is_silence(value) => "silence zero".to_owned(),
                Settled::Value(value) => value.amount().to_owned(),
                Settled::Unknown(_) => "unknown".to_owned(),
                Settled::NotApplicable(_) => "not applicable".to_owned(),
            };
            (held.accession().to_owned(), came_to)
        })
        .collect()
}

fn pairs(held: &[(&str, &str)]) -> Vec<(String, String)> {
    held.iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

fn unknown<'s, 'f>(stood: &'s Stands<'_, 'f>) -> &'s vfi_normalize::standing::Unsettled<'f> {
    match stood {
        Stands::Unknown(unsettled) => unsettled,
        other => panic!("expected unknown, got {other:?}"),
    }
}

fn accessions<'f>(attempted: &[(&'f str, Attempt)]) -> Vec<&'f str> {
    attempted.iter().map(|(accession, _)| *accession).collect()
}

#[test]
fn an_earlier_read_figure_stands_beside_a_later_silence() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact("ShortTermInvestments", "USD", &at, "500", earlier()),
        silent(&at, later()),
    ];
    assert_eq!(
        per_filing(&registry, Concept::ShortTermInvestments, &at, &facts),
        pairs(&[(EARLIER, "500"), (LATER, "silence zero")]),
    );

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "500");
    assert_eq!(read_from(value), [EARLIER]);
}

#[test]
fn an_earlier_contest_that_did_not_settle_beside_a_later_silence_is_unknown_and_not_the_zero() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact("MarketableSecuritiesCurrent", "USD", &at, "300", earlier()),
        fact("OtherShortTermInvestments", "USD", &at, "200", earlier()),
        silent(&at, later()),
    ];
    assert_eq!(
        per_filing(&registry, Concept::ShortTermInvestments, &at, &facts),
        pairs(&[(EARLIER, "unknown"), (LATER, "silence zero")]),
    );

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let unsettled = unknown(&stood);
    assert_eq!(accessions(unsettled.attempted()), [EARLIER, LATER]);
    assert_eq!(unsettled.undecided(), None);

    // The withheld zero carries what its four steps found: every eligible rule,
    // none of them a candidate.
    assert_eq!(unsettled.attempted()[1].1.declined().len(), 4);
}

#[test]
fn an_earlier_silence_beside_a_later_read_figure_is_the_read_figure() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        silent(&at, earlier()),
        fact("ShortTermInvestments", "USD", &at, "500", later()),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "500");
    assert_eq!(read_from(value), [LATER]);
}

#[test]
fn every_answering_filing_silent_is_the_zero_naming_no_filing() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [silent(&at, earlier()), silent(&at, later())];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "0");
    assert!(is_silence(value));
    assert_eq!(value.set_by().rule(), None);
}

#[test]
fn a_period_one_filing_answers_is_what_that_filing_settles_to() {
    let registry = read(&committed());
    let at = instant(AT);
    for facts in [
        vec![fact("ShortTermInvestments", "USD", &at, "500", earlier())],
        vec![silent(&at, earlier())],
    ] {
        let settled = settling::settle(
            &registry,
            FILER,
            KIND,
            Concept::ShortTermInvestments,
            &at,
            &facts.iter().collect::<Vec<_>>(),
            Admits::OnlyThePeriodAskedFor,
        );
        let Settled::Value(settled) = settled else {
            panic!("the filing settles to a value");
        };
        assert_eq!(
            standing(&registry, Concept::ShortTermInvestments, &at, &facts),
            Stands::Value(settled)
        );
    }
}

#[test]
fn an_assertion_is_total_over_the_period_and_no_silence_reading_is_reached_beside_it() {
    let root = planted("standing-assertion");
    overriding(
        &root,
        &format!(
            "\n[[assert]]\nconcept = \"short_term_investments\"\n\
             period = {{ instant = \"{AT}\" }}\n\
             value = \"750\"\n\
             source = {{ accession = \"{EARLIER}\", line = \"12\" }}\n"
        ),
    );
    let registry = read(&root);
    let at = instant(AT);
    let facts = [silent(&at, earlier()), silent(&at, later())];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "750");
    assert!(matches!(value.set_by(), SetBy::Assertion { .. }));
}

#[test]
fn a_same_day_tie_the_amendment_breaks() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact(
            "ShortTermInvestments",
            "USD",
            &at,
            "500",
            (LATER, "10-Q", LATER_FILED),
        ),
        fact(
            "ShortTermInvestments",
            "USD",
            &at,
            "600",
            (FIRST_BY_ACCESSION, "10-Q/A", LATER_FILED),
        ),
        fact("ShortTermInvestments", "USD", &at, "700", earlier()),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "600");
    assert_eq!(read_from(value), [FIRST_BY_ACCESSION]);
}

#[test]
fn a_same_day_tie_no_form_breaks_is_unknown_carrying_both_accessions_whatever_the_figures_say() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact(
            "ShortTermInvestments",
            "USD",
            &at,
            "500",
            (LATER, "10-Q", LATER_FILED),
        ),
        fact(
            "ShortTermInvestments",
            "USD",
            &at,
            "500",
            (FIRST_BY_ACCESSION, "10-K", LATER_FILED),
        ),
        fact("ShortTermInvestments", "USD", &at, "500", earlier()),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let unsettled = unknown(&stood);
    assert_eq!(
        unsettled.undecided(),
        Some(&Undecided::Tie(vec![FIRST_BY_ACCESSION, LATER]))
    );
    assert!(unsettled.attempted().is_empty());
}

#[test]
fn a_conditional_reading_whose_partner_one_filing_reached() {
    let registry = read(&committed());
    let quarter = duration(QUARTER.0, QUARTER.1);
    let facts = [
        fact("PaymentsOfDividends", "USD", &quarter, "1000", earlier()),
        silent(&quarter, later()),
    ];
    assert_eq!(
        per_filing(
            &registry,
            Concept::DividendsDeclaredPerShare,
            &quarter,
            &facts
        ),
        pairs(&[(EARLIER, "unknown"), (LATER, "silence zero")]),
    );
    assert_eq!(
        per_filing(&registry, Concept::DividendsPaid, &quarter, &facts),
        pairs(&[(EARLIER, "1000"), (LATER, "silence zero")]),
    );

    let declared = standing(
        &registry,
        Concept::DividendsDeclaredPerShare,
        &quarter,
        &facts,
    );
    assert_eq!(accessions(unknown(&declared).attempted()), [EARLIER, LATER]);

    let paid = standing(&registry, Concept::DividendsPaid, &quarter, &facts);
    let value = value(&paid);
    assert_eq!(value.amount(), "1000");
    assert_eq!(read_from(value), [EARLIER]);
}

/// The reading and the registry version a silence zero carries, and the two
/// things it is asked for beyond its amount.
fn supplied(value: &Value) -> (Silence, String) {
    match value.set_by() {
        SetBy::Silence { reading, version } => (*reading, version.rendered()),
        other => panic!("expected a silence zero, got {other:?}"),
    }
}

#[test]
fn filings_sharing_the_greatest_filed_and_all_silent_come_to_the_zero_and_not_to_a_tie() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        silent(&at, earlier()),
        silent(&at, later()),
        silent(&at, same_day()),
    ];
    assert_eq!(
        per_filing(&registry, Concept::ShortTermInvestments, &at, &facts),
        pairs(&[
            (FIRST_BY_ACCESSION, "silence zero"),
            (EARLIER, "silence zero"),
            (LATER, "silence zero"),
        ]),
    );

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "0");
    assert_eq!(value.set_by().rule(), None);
    assert_eq!(
        supplied(value),
        (Silence::Zero, registry.version().rendered())
    );
}

#[test]
fn a_filing_reaching_the_concept_stands_beside_one_filed_the_same_day_and_silent() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact("ShortTermInvestments", "USD", &at, "500", same_day()),
        silent(&at, later()),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let value = value(&stood);
    assert_eq!(value.amount(), "500");
    assert_eq!(read_from(value), [FIRST_BY_ACCESSION]);
}

#[test]
fn two_filings_reaching_the_concept_on_the_same_day_are_the_tie_as_before() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact("ShortTermInvestments", "USD", &at, "500", same_day()),
        fact("ShortTermInvestments", "USD", &at, "600", later()),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    let unsettled = unknown(&stood);
    assert_eq!(
        unsettled.undecided(),
        Some(&Undecided::Tie(vec![FIRST_BY_ACCESSION, LATER]))
    );
}

#[test]
fn a_conditional_reading_whose_partner_one_same_day_filing_reached_is_unknown() {
    let registry = read(&committed());
    let quarter = duration(QUARTER.0, QUARTER.1);
    let facts = [
        fact("PaymentsOfDividends", "USD", &quarter, "1000", same_day()),
        silent(&quarter, later()),
    ];
    assert_eq!(
        per_filing(
            &registry,
            Concept::DividendsDeclaredPerShare,
            &quarter,
            &facts
        ),
        pairs(&[(FIRST_BY_ACCESSION, "unknown"), (LATER, "silence zero")]),
    );

    let declared = standing(
        &registry,
        Concept::DividendsDeclaredPerShare,
        &quarter,
        &facts,
    );
    let unsettled = unknown(&declared);
    assert_eq!(
        accessions(unsettled.attempted()),
        [FIRST_BY_ACCESSION, LATER]
    );
    assert_eq!(unsettled.undecided(), None);
}

#[test]
fn two_same_day_filings_silent_on_both_members_of_the_pair_come_to_the_zero() {
    let registry = read(&committed());
    let quarter = duration(QUARTER.0, QUARTER.1);
    let facts = [silent(&quarter, later()), silent(&quarter, same_day())];

    for concept in [Concept::DividendsDeclaredPerShare, Concept::DividendsPaid] {
        let stood = standing(&registry, concept, &quarter, &facts);
        let value = value(&stood);
        assert_eq!(value.amount(), "0", "{concept:?}");
        assert_eq!(
            supplied(value),
            (Silence::Conditional, registry.version().rendered()),
            "{concept:?}"
        );
    }
}

#[test]
fn a_concept_the_kind_excludes_is_not_applicable_with_no_filing_consulted() {
    let registry = read(&committed());
    let at = instant(AT);
    let stood = stands(
        &registry,
        FILER,
        Some(Kind::Bank),
        Concept::ShortTermInvestments,
        &at,
        &[],
    );
    assert!(matches!(stood, Some(Stands::NotApplicable(_))));

    // With no kind established the same concept is attempted, and a period no
    // filing answers has no attempt to show.
    assert_eq!(
        stands(
            &registry,
            FILER,
            None,
            Concept::ShortTermInvestments,
            &at,
            &[]
        ),
        None
    );
    let facts = [silent(&at, earlier())];
    assert!(matches!(
        stands(&registry, FILER, None, Concept::GrossProfit, &at, &facts),
        Some(Stands::Unknown(_))
    ));
}

#[test]
fn a_filing_whose_filed_does_not_read_as_a_date_orders_nothing() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = [
        fact("ShortTermInvestments", "USD", &at, "500", earlier()),
        fact(
            "ShortTermInvestments",
            "USD",
            &at,
            "600",
            (LATER, "10-Q", "2031-13-01"),
        ),
    ];

    let stood = standing(&registry, Concept::ShortTermInvestments, &at, &facts);
    assert_eq!(
        unknown(&stood).undecided(),
        Some(&Undecided::Undated(vec![LATER]))
    );
}

/// The two tables `docs/adr/which-filing-sets-the-value.md` states for the
/// restatement fixture: each concept, the amount that stands, and the filing it
/// was read from, or none for the silence rows.
const HALF_YEAR: [(Concept, &str, Option<&str>); 15] = [
    (Concept::Revenue, "264956", Some(RESTATING)),
    (Concept::GrossProfit, "103206", Some(RESTATING)),
    (Concept::OperatingIncome, "-76928", Some(RESTATING)),
    (Concept::PretaxIncome, "-77035", Some(RESTATING)),
    (Concept::IncomeTaxExpense, "367", Some(RESTATING)),
    (Concept::NetIncome, "-77402", Some(RESTATING)),
    (Concept::InterestExpense, "536", Some(RESTATING)),
    (Concept::DepreciationAndAmortization, "401", Some(RESTATING)),
    (Concept::OperatingCashFlow, "-127584", Some(RESTATING)),
    (Concept::CapitalExpenditure, "3729", Some(RESTATING)),
    (
        Concept::DilutedSharesWeightedAverage,
        "50301639",
        Some(RESTATING),
    ),
    (Concept::EarningsPerShareDiluted, "-0.0015", Some(RESTATING)),
    (Concept::ShortTermInvestments, "0", None),
    (Concept::DividendsDeclaredPerShare, "0", None),
    (Concept::DividendsPaid, "0", None),
];

const QUARTER_OF_IT: [(Concept, &str, Option<&str>); 11] = [
    (Concept::Revenue, "140986", Some(RESTATING)),
    (Concept::GrossProfit, "55075", Some(RESTATING)),
    (Concept::OperatingIncome, "-24626", Some(RESTATING)),
    (Concept::PretaxIncome, "-24626", Some(RESTATING)),
    (Concept::IncomeTaxExpense, "282", Some(RESTATING)),
    (
        Concept::DilutedSharesWeightedAverage,
        "60000000",
        Some(RESTATING),
    ),
    (Concept::EarningsPerShareDiluted, "-0.0004", Some(RESTATING)),
    (Concept::NetIncome, "-24908", Some(QUOTING)),
    (Concept::ShortTermInvestments, "0", None),
    (Concept::DividendsDeclaredPerShare, "0", None),
    (Concept::DividendsPaid, "0", None),
];

/// The 10-Q filed 2025-05-14, which restated the half-year's diluted loss per
/// share.
const RESTATING: &str = "0001213900-25-042964";

/// The 10-Q filed 2025-08-20, quoting one comparative line of the quarter.
const QUOTING: &str = "0001213900-25-078735";

fn pins(facts: &[Fact], period: &Period, table: &[(Concept, &str, Option<&str>)]) {
    let registry = read(&committed());
    for (concept, amount, filing) in table {
        let stood = standing(&registry, *concept, period, facts);
        let value = value(&stood);
        assert_eq!(value.amount(), *amount, "{concept:?}");
        match filing {
            Some(accession) => {
                let read = read_from(value);
                assert!(!read.is_empty(), "{concept:?}");
                assert!(
                    read.iter().all(|held| held == accession),
                    "{concept:?}: {read:?}"
                );
            }
            None => assert!(is_silence(value), "{concept:?}"),
        }
    }
}

#[test]
fn the_restatement_half_year_stands_as_the_record_states_it() {
    let facts = reported();
    let period = duration("2023-10-01", "2024-03-31");
    let answering: Vec<&str> = filings::answering(&facts, &period)
        .iter()
        .map(|filing| filing.accession())
        .collect();
    assert_eq!(answering, ["0001213900-24-040632", RESTATING]);
    pins(&facts, &period, &HALF_YEAR);
}

#[test]
fn the_restatement_quarter_stands_as_the_record_states_it() {
    let facts = reported();
    let period = duration("2024-01-01", "2024-03-31");
    let answering: Vec<&str> = filings::answering(&facts, &period)
        .iter()
        .map(|filing| filing.accession())
        .collect();
    assert_eq!(
        answering,
        [
            "0001213900-24-040632",
            "0001213900-24-067900",
            RESTATING,
            QUOTING
        ]
    );
    pins(&facts, &period, &QUARTER_OF_IT);
}

/// The 10-Q/A filed 2025-08-15, the later of the two filings stating
/// `PreferredStockValue` at 2024-12-31.
const AMENDED_QUARTER: &str = "0002008589-25-000033";

#[test]
fn preferred_equity_stands_as_the_read_zero_the_four_later_silences_do_not_displace() {
    let facts = reported_in("a-filer-that-changed-its-fiscal-year");
    let registry = read(&committed());
    let filer = "0001859199";
    let at = instant("2024-12-31");

    let answering = filings::answering(&facts, &at);
    assert_eq!(answering.len(), 9);
    let stating: Vec<&str> = answering
        .iter()
        .filter(|filing| {
            filing
                .facts()
                .iter()
                .any(|fact| &*fact.tag == "PreferredStockValue" && fact.period == at)
        })
        .map(|filing| filing.accession())
        .collect();
    assert_eq!(stating, ["0002008589-25-000020", AMENDED_QUARTER]);

    let attempted = filings::attempted(
        &registry,
        filer,
        KIND,
        Concept::PreferredEquity,
        &at,
        &answering,
    );
    let after: Vec<_> = answering
        .iter()
        .zip(&attempted)
        .filter(|(filing, _)| &*filing.facts()[0].filed > "2025-08-15")
        .map(|(_, held)| held)
        .collect();
    assert_eq!(after.len(), 4);
    for held in after {
        assert!(!held.reached(), "{}", held.accession());
        assert!(
            matches!(held.settled(), Settled::Unknown(_)),
            "{}",
            held.accession()
        );
    }

    let stood = stands(
        &registry,
        filer,
        KIND,
        Concept::PreferredEquity,
        &at,
        &facts,
    )
    .expect("nine filings answer the instant");
    let value = value(&stood);
    assert_eq!(value.amount(), "0");
    assert_eq!(read_from(value), [AMENDED_QUARTER]);
    let SetBy::Rule { facts: read, .. } = value.set_by() else {
        unreachable!()
    };
    assert_eq!(&*read[0].tag, "PreferredStockValue");
}
