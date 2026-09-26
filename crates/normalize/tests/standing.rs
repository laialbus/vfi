//! What one concept comes to at one period over every filing that answers it.
//!
//! The hand-written cases each pin one thing `docs/adr/which-filing-sets-the-value.md`
//! or `docs/adr/silence-beside-a-read-figure.md` says the rule does, over facts
//! written for the case: no merged fixture holds a filing that reaches any of
//! the three silence-read concepts, a same-day tie, or an assertion. The
//! committed registry reads them all, under the filer the merged fixture
//! records, whose kind is `operating`.
//!
//! The fixture cases re-derive what those records and
//! `docs/adr/silence-zero-only-at-the-concepts-own-shape.md` claim about the
//! merged fetch fixtures, and pin it.

mod fixture;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Kind, Measure, Silence};
use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::answering::Admits;
use vfi_normalize::filings;
use vfi_normalize::periods::periods;
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

    // Each tied filing's attempt, carrying the three stand-ins it had no fact
    // for and not the exact reading that settled; the earlier filing, which
    // lost on `filed` alone, is not carried.
    let declined: Vec<(String, String)> = STAND_INS
        .iter()
        .map(|id| {
            (
                settling::named(registry.version(), id),
                "no fact answering the period asked for".to_owned(),
            )
        })
        .collect();
    assert_eq!(
        accessions(unsettled.attempted()),
        [FIRST_BY_ACCESSION, LATER]
    );
    for (accession, attempt) in unsettled.attempted() {
        assert_eq!(reasons(attempt), declined, "{accession}");
    }
}

/// The three entries that stand in for `short_term_investments`, as their ids.
const STAND_INS: [&str; 3] = [
    "short_term_investments|*|tag|element:us-gaap:AvailableForSaleSecuritiesDebtSecuritiesCurrent",
    "short_term_investments|*|tag|element:us-gaap:MarketableSecuritiesCurrent",
    "short_term_investments|*|tag|element:us-gaap:OtherShortTermInvestments",
];

fn reasons(attempt: &Attempt) -> Vec<(String, String)> {
    attempt
        .declined()
        .iter()
        .map(|held| (held.candidate.to_string(), held.rule.to_string()))
        .collect()
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

/// Whether `period` is of `concept`'s own shape, read off the published
/// measure here rather than through the crate, so the rule the crate applies is
/// checked against the vocabulary and not against itself.
fn own_shape(concept: Concept, period: &Period) -> bool {
    matches!(
        (concept.definition().measure, period),
        (Measure::Flow, Period::Duration { .. }) | (Measure::Balance, Period::Instant { .. })
    )
}

/// `docs/adr/silence-zero-only-at-the-concepts-own-shape.md`: the zero is
/// reached at an instant for a balance and at a duration for a flow, and at the
/// other shape the concept is `Unknown`, carrying each answering filing's
/// attempt as any other `Unknown` does.
///
/// Each filing's own reading is pinned beside the reading made over the period,
/// because the two are made in one place and a zero one of them refused is one
/// the other must refuse too.
#[test]
fn a_silence_zero_is_reached_only_at_a_period_of_the_concepts_own_shape() {
    let registry = read(&committed());
    let at = instant(AT);
    let quarter = duration(QUARTER.0, QUARTER.1);

    for (concept, own, other) in [
        (Concept::ShortTermInvestments, &at, &quarter),
        (Concept::DividendsPaid, &quarter, &at),
    ] {
        assert!(own_shape(concept, own) && !own_shape(concept, other));

        let facts = [silent(own, earlier())];
        let stood = standing(&registry, concept, own, &facts);
        assert_eq!(value(&stood).amount(), "0", "{concept:?}");
        assert!(is_silence(value(&stood)), "{concept:?}");

        let facts = [silent(other, earlier()), silent(other, later())];
        assert_eq!(
            per_filing(&registry, concept, other, &facts),
            pairs(&[(EARLIER, "unknown"), (LATER, "unknown")]),
            "{concept:?}"
        );
        let stood = standing(&registry, concept, other, &facts);
        assert_eq!(
            accessions(unknown(&stood).attempted()),
            [EARLIER, LATER],
            "{concept:?}"
        );
    }
}

/// The conditional pair at an instant, which is neither member's shape: both
/// are `Unknown`, and stay so where an assertion makes one of them reached, the
/// condition being unasked where there is no zero for it to hold up.
#[test]
fn the_conditional_pair_at_an_instant_is_unknown_whatever_its_partner() {
    let at = instant(AT);
    let facts = [silent(&at, earlier())];

    let registry = read(&committed());
    for concept in [Concept::DividendsDeclaredPerShare, Concept::DividendsPaid] {
        let stood = standing(&registry, concept, &at, &facts);
        assert_eq!(
            accessions(unknown(&stood).attempted()),
            [EARLIER],
            "{concept:?}"
        );
    }

    let root = planted("the-conditional-pair-at-an-instant");
    overriding(
        &root,
        &format!(
            "\n[[assert]]\nconcept = \"dividends_paid\"\n\
             period = {{ instant = \"{AT}\" }}\n\
             value = \"1000\"\n\
             source = {{ accession = \"0001213900-24-101777\", line = \"58\" }}\n"
        ),
    );
    let asserting = read(&root);

    let paid = standing(&asserting, Concept::DividendsPaid, &at, &facts);
    assert_eq!(value(&paid).amount(), "1000");
    let declared = standing(&asserting, Concept::DividendsDeclaredPerShare, &at, &facts);
    assert_eq!(accessions(unknown(&declared).attempted()), [EARLIER]);
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

/// What a row of the two tables below expects.
///
/// `Unknown` carries no accessions of its own: at a period of the wrong shape
/// every answering filing attempted the concept, so the row states that the
/// accessions are those filings' and `pins` reads them off the period.
enum Stood {
    /// The amount that stands, and the filing every fact behind it was reported
    /// in.
    Read(&'static str, &'static str),
    /// The zero the vocabulary's silence reading supplied for the period.
    Silence,
    Unknown,
}

/// The two tables `docs/adr/which-filing-sets-the-value.md` states for the
/// restatement fixture, with the `short_term_investments` row of each as
/// `docs/adr/silence-zero-only-at-the-concepts-own-shape.md` supersedes it: a
/// balance at a half-year and at a quarter is `Unknown`, where the tables said
/// zero. The two dividend rows stand as the tables state them, both concepts
/// being flows read at a duration.
const HALF_YEAR: [(Concept, Stood); 15] = [
    (Concept::Revenue, Stood::Read("264956", RESTATING)),
    (Concept::GrossProfit, Stood::Read("103206", RESTATING)),
    (Concept::OperatingIncome, Stood::Read("-76928", RESTATING)),
    (Concept::PretaxIncome, Stood::Read("-77035", RESTATING)),
    (Concept::IncomeTaxExpense, Stood::Read("367", RESTATING)),
    (Concept::NetIncome, Stood::Read("-77402", RESTATING)),
    (Concept::InterestExpense, Stood::Read("536", RESTATING)),
    (
        Concept::DepreciationAndAmortization,
        Stood::Read("401", RESTATING),
    ),
    (
        Concept::OperatingCashFlow,
        Stood::Read("-127584", RESTATING),
    ),
    (Concept::CapitalExpenditure, Stood::Read("3729", RESTATING)),
    (
        Concept::DilutedSharesWeightedAverage,
        Stood::Read("50301639", RESTATING),
    ),
    (
        Concept::EarningsPerShareDiluted,
        Stood::Read("-0.0015", RESTATING),
    ),
    (Concept::ShortTermInvestments, Stood::Unknown),
    (Concept::DividendsDeclaredPerShare, Stood::Silence),
    (Concept::DividendsPaid, Stood::Silence),
];

const QUARTER_OF_IT: [(Concept, Stood); 11] = [
    (Concept::Revenue, Stood::Read("140986", RESTATING)),
    (Concept::GrossProfit, Stood::Read("55075", RESTATING)),
    (Concept::OperatingIncome, Stood::Read("-24626", RESTATING)),
    (Concept::PretaxIncome, Stood::Read("-24626", RESTATING)),
    (Concept::IncomeTaxExpense, Stood::Read("282", RESTATING)),
    (
        Concept::DilutedSharesWeightedAverage,
        Stood::Read("60000000", RESTATING),
    ),
    (
        Concept::EarningsPerShareDiluted,
        Stood::Read("-0.0004", RESTATING),
    ),
    (Concept::NetIncome, Stood::Read("-24908", QUOTING)),
    (Concept::ShortTermInvestments, Stood::Unknown),
    (Concept::DividendsDeclaredPerShare, Stood::Silence),
    (Concept::DividendsPaid, Stood::Silence),
];

/// The 10-Q filed 2025-05-14, which restated the half-year's diluted loss per
/// share.
const RESTATING: &str = "0001213900-25-042964";

/// The 10-Q filed 2025-08-20, quoting one comparative line of the quarter.
const QUOTING: &str = "0001213900-25-078735";

fn pins(facts: &[Fact], period: &Period, table: &[(Concept, Stood)]) {
    let registry = read(&committed());
    let answering: Vec<&str> = filings::answering(facts, period)
        .iter()
        .map(|filing| filing.accession())
        .collect();

    for (concept, expected) in table {
        let stood = standing(&registry, *concept, period, facts);
        match expected {
            Stood::Read(amount, accession) => {
                let value = value(&stood);
                assert_eq!(value.amount(), *amount, "{concept:?}");
                let read = read_from(value);
                assert!(!read.is_empty(), "{concept:?}");
                assert!(
                    read.iter().all(|held| held == accession),
                    "{concept:?}: {read:?}"
                );
            }
            Stood::Silence => {
                let value = value(&stood);
                assert_eq!(value.amount(), "0", "{concept:?}");
                assert!(is_silence(value), "{concept:?}");
            }
            Stood::Unknown => {
                assert_eq!(
                    accessions(unknown(&stood).attempted()),
                    answering,
                    "{concept:?}"
                );
            }
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

/// The counts `docs/adr/silence-zero-only-at-the-concepts-own-shape.md` states
/// for the merged fixture, over every period Rule 1 admits and every concept
/// the vocabulary publishes: 51 silence values, every one at a period of the
/// concept's own shape, and 576 `Unknown`s.
///
/// These are the emit record's 96 and 531 with its 45 wrong-shape zeros moved
/// across, and the record that moved them says a count that does not re-derive
/// is an escalation rather than a number to adjust.
#[test]
fn the_merged_fixture_comes_to_the_counts_the_record_supersedes_the_emit_records_with() {
    let registry = read(&committed());
    let facts = reported();
    let held = periods(&registry, FILER, KIND, &facts);
    assert_eq!(held.len(), 32);

    let mut silence = 0;
    let mut unknown = 0;
    for period in &held {
        for concept in Concept::ALL {
            let stood = stands(&registry, FILER, KIND, *concept, period, &facts)
                .expect("a filing answers every period Rule 1 admits");
            match &stood {
                Stands::Value(value) if is_silence(value) => {
                    assert!(own_shape(*concept, period), "{concept:?} {period:?}");
                    silence += 1;
                }
                Stands::Unknown(_) => unknown += 1,
                _ => {}
            }
        }
    }
    assert_eq!((silence, unknown), (51, 576));
}
