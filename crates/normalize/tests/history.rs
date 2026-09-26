//! One filer's history, as `canonical-concepts` v2 publishes it.
//!
//! The fixture cases run the three merged fetch fixtures the milestone names
//! through the crate's surface against the committed registry, and pin what
//! `docs/adr/what-normalize-emits.md` and
//! `docs/adr/silence-zero-only-at-the-concepts-own-shape.md` claim of them.
//! The hand-written cases pin what no merged fixture reaches: a filer with no
//! kind, facts that admit no period, an assertion, a tie, and a `filed` that
//! does not read as a date.

mod fixture;

use vfi_contracts::canonical_concepts::{
    Attempted, Concept, Kind, Measure, Period, Resolution, SetBy, Silence, SourceTag,
};
use vfi_contracts::fetch_normalize::{self, Fact, Filer};
use vfi_normalize::history::{History, Row, history};
use vfi_normalize::registry::{Outcome, Registry};
use vfi_normalize::settling;

use fixture::{CASE, FILER, committed, instant, overriding, planted, read, recorded_in};

fn run(registry: &Registry, filer: &Filer) -> History {
    history(registry, filer).unwrap_or_else(|why| panic!("{why}"))
}

fn at(date: &str) -> Period {
    Period::Instant { at: date.into() }
}

fn between(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

fn row<'h>(history: &'h History, period: &Period) -> &'h Row {
    history
        .periods()
        .iter()
        .find(|row| row.period() == period)
        .unwrap_or_else(|| panic!("{period:?} is not a period of the history"))
}

fn own_shape(concept: Concept, period: &Period) -> bool {
    matches!(
        (concept.definition().measure, period),
        (Measure::Flow, Period::Duration { .. }) | (Measure::Balance, Period::Instant { .. })
    )
}

fn attempts(resolution: &Resolution) -> Vec<&Attempted> {
    match resolution {
        Resolution::Unknown { attempted } => attempted.in_each_filing().collect(),
        other => panic!("expected an Unknown, got {other:?}"),
    }
}

fn accessions(resolution: &Resolution) -> Vec<&str> {
    attempts(resolution)
        .iter()
        .map(|held| held.accession())
        .collect()
}

fn declined(attempted: &Attempted) -> Vec<(&str, &str)> {
    attempted
        .attempt()
        .declined()
        .iter()
        .map(|held| (&*held.candidate, &*held.rule))
        .collect()
}

fn tag(taxonomy: &str, tag: &str) -> SourceTag {
    SourceTag {
        taxonomy: taxonomy.into(),
        tag: tag.into(),
    }
}

/// The 10-Q filed 2025-05-14, which restated the half-year's diluted loss per
/// share.
const RESTATING: &str = "0001213900-25-042964";

/// The 10-Q filed 2025-08-20, quoting one comparative line of the quarter.
const QUOTING: &str = "0001213900-25-078735";

/// The 10-Q/A filed 2025-08-15, the later of the two filings stating
/// `PreferredStockValue` at 2024-12-31 on CIK 0001859199.
const AMENDED_QUARTER: &str = "0002008589-25-000033";

/// The counts the silence-shape record re-states for the restatement fixture,
/// with the 422 the 2026-09-24 stop re-derived: 32 periods, each with all
/// twenty-eight concepts, 51 silence values each at a period of the concept's
/// own shape, 576 `Unknown`s, 269 `read`, no assertion and no `NotApplicable`.
#[test]
fn the_restatement_filer_comes_to_the_counts_the_record_restates() {
    let registry = read(&committed());
    let history = run(&registry, &recorded_in(CASE));
    assert_eq!(history.filer(), FILER);
    assert_eq!(history.periods().len(), 32);

    let (mut read, mut asserted, mut silence, mut unknown, mut across, mut excluded) =
        (0, 0, 0, 0, 0, 0);
    for row in history.periods() {
        let concepts: Vec<Concept> = row.concepts().map(|(concept, _)| concept).collect();
        assert_eq!(concepts, Concept::ALL, "{:?}", row.period());

        for (concept, resolution) in row.concepts() {
            match resolution {
                Resolution::Value { set_by, .. } => match set_by {
                    SetBy::Read { .. } => read += 1,
                    SetBy::Asserted { .. } => asserted += 1,
                    SetBy::Silence { .. } => {
                        assert!(own_shape(concept, row.period()), "{concept:?} {row:?}");
                        silence += 1;
                    }
                },
                Resolution::Unknown { attempted } => {
                    unknown += 1;
                    if attempted.in_each_filing().count() > 1 {
                        across += 1;
                    }
                }
                Resolution::NotApplicable { .. } => excluded += 1,
            }
        }
    }
    assert_eq!(
        (read, asserted, silence, unknown, across, excluded),
        (269, 0, 51, 576, 422, 0)
    );
}

enum Stood {
    Read(&'static str, &'static str),
    Silence,
    Unknown,
}

/// The rows of the Rule 3 record's two tables, the `short_term_investments` row
/// of each as the silence-shape record supersedes it.
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

fn pins(period: &Period, table: &[(Concept, Stood)], reads: usize) {
    let registry = read(&committed());
    let version = registry.version().rendered();
    let history = run(&registry, &recorded_in(CASE));
    let row = row(&history, period);

    for (concept, expected) in table {
        let resolution = row.of(*concept);
        match (expected, resolution) {
            (
                Stood::Read(amount, accession),
                Resolution::Value {
                    amount: held,
                    set_by:
                        SetBy::Read {
                            source_tags,
                            filing,
                            rule,
                        },
                },
            ) => {
                assert_eq!(&**held, *amount, "{concept:?}");
                assert_eq!(&**filing, *accession, "{concept:?}");
                assert!(!source_tags.is_empty(), "{concept:?}");
                assert_eq!(*rule.registry, *version, "{concept:?}");
            }
            (
                Stood::Silence,
                Resolution::Value {
                    amount,
                    set_by: SetBy::Silence { reading, registry },
                },
            ) => {
                assert_eq!(&**amount, "0", "{concept:?}");
                assert_eq!(*reading, Silence::Conditional, "{concept:?}");
                assert_eq!(**registry, *version, "{concept:?}");
            }
            (Stood::Unknown, Resolution::Unknown { .. }) => {}
            (_, other) => panic!("{concept:?}: {other:?}"),
        }
    }

    let read = row
        .concepts()
        .filter(|(_, resolution)| {
            matches!(
                resolution,
                Resolution::Value {
                    set_by: SetBy::Read { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!(read, reads);
}

#[test]
fn the_restatement_half_year_crosses_as_the_record_states_it() {
    pins(&between("2023-10-01", "2024-03-31"), &HALF_YEAR, 12);
}

#[test]
fn the_restatement_quarter_crosses_as_the_record_states_it() {
    pins(&between("2024-01-01", "2024-03-31"), &QUARTER_OF_IT, 8);
}

#[test]
fn a_read_zero_and_a_silence_zero_cross_as_two_ways_at_one_instant() {
    let registry = read(&committed());
    let version = registry.version().rendered();
    let history = run(
        &registry,
        &recorded_in("a-filer-that-changed-its-fiscal-year"),
    );
    assert_eq!(history.filer(), "0001859199");
    let row = row(&history, &at("2024-12-31"));

    let Resolution::Value {
        amount,
        set_by:
            SetBy::Read {
                source_tags,
                filing,
                rule,
            },
    } = row.of(Concept::PreferredEquity)
    else {
        panic!("{:?}", row.of(Concept::PreferredEquity));
    };
    assert_eq!(&**amount, "0");
    assert_eq!(source_tags, &[tag("us-gaap", "PreferredStockValue")]);
    assert_eq!(&**filing, AMENDED_QUARTER);
    assert_eq!(*rule.registry, *version);

    assert_eq!(
        row.of(Concept::ShortTermInvestments),
        &Resolution::Value {
            amount: "0".into(),
            set_by: SetBy::Silence {
                reading: Silence::Zero,
                registry: version.into(),
            },
        }
    );
}

#[test]
fn a_concept_the_registrys_kind_excludes_is_not_applicable_at_every_period() {
    let registry = read(&committed());
    let history = run(
        &registry,
        &recorded_in("a-filer-that-suspended-its-dividend"),
    );
    assert_eq!(history.filer(), "0001778784");
    assert_eq!(registry.kind_of("0001778784"), Some(Kind::Bank));
    assert!(!history.periods().is_empty());

    for row in history.periods() {
        let Resolution::NotApplicable { excluded } = row.of(Concept::ShortTermInvestments) else {
            panic!("{row:?}");
        };
        assert_eq!(excluded.kind(), Kind::Bank);
        assert_eq!(
            excluded.clause(),
            Concept::ShortTermInvestments.definition().applies_to
        );
    }
}

/// A balance sheet date no merged fixture reaches.
const AT: &str = "2030-06-30";

const EARLIER: &str = "0000000001-30-000001";
const LATER: &str = "0000000001-31-000001";
const SAME_DAY: &str = "0000000000-31-000001";

const EARLIER_FILED: &str = "2030-08-01";
const LATER_FILED: &str = "2031-08-01";

/// No file in the registry names this filer, so it holds no kind for it.
const UNASSIGNED: &str = "0000000001";

fn fact(tag: &str, value: &str, filing: (&str, &str, &str)) -> Fact {
    let (accession, form, filed) = filing;
    Fact {
        taxonomy: "us-gaap".into(),
        tag: tag.into(),
        unit: "USD".into(),
        period: instant(AT),
        value: value.into(),
        accession: accession.into(),
        form: form.into(),
        filed: filed.into(),
        report_period_end: "".into(),
    }
}

fn filer(cik: &str, facts: Vec<Fact>) -> Filer {
    Filer {
        cik: cik.into(),
        retrieved_from: "written for the case".into(),
        facts,
    }
}

fn earlier() -> (&'static str, &'static str, &'static str) {
    (EARLIER, "10-Q", EARLIER_FILED)
}

fn later() -> (&'static str, &'static str, &'static str) {
    (LATER, "10-Q", LATER_FILED)
}

/// Filed the day the later filing was, under a form that is not its with `/A`
/// appended.
fn same_day() -> (&'static str, &'static str, &'static str) {
    (SAME_DAY, "10-K", LATER_FILED)
}

#[test]
fn a_filer_the_registry_holds_no_kind_for_is_run_with_none_and_has_no_period() {
    let registry = read(&committed());
    let facts = vec![fact("Assets", "900", earlier())];
    assert_eq!(registry.kind_of(UNASSIGNED), None);

    let unassigned = run(&registry, &filer(UNASSIGNED, facts.clone()));
    assert_eq!(unassigned.filer(), UNASSIGNED);
    assert!(unassigned.periods().is_empty());

    let operating = run(&registry, &filer(FILER, facts));
    assert_eq!(operating.periods().len(), 1);
}

#[test]
fn a_filer_whose_facts_admit_no_period_has_none() {
    let registry = read(&committed());
    let history = run(
        &registry,
        &filer(FILER, vec![fact("ElementNoRuleReads", "900", earlier())]),
    );
    assert_eq!(history.filer(), FILER);
    assert!(history.periods().is_empty());
}

#[test]
fn an_asserted_value_carries_its_rule_pair_and_cited_filing_and_no_tag() {
    let root = planted("history-assertion");
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
    let Outcome::Asserted(assertion) = registry
        .answer(
            FILER,
            Some(Kind::Operating),
            Concept::ShortTermInvestments,
            &instant(AT),
        )
        .outcome
    else {
        panic!("the registry asserts nothing");
    };

    let history = run(
        &registry,
        &filer(FILER, vec![fact("Assets", "900", later())]),
    );
    assert_eq!(
        row(&history, &at(AT)).of(Concept::ShortTermInvestments),
        &Resolution::Value {
            amount: "750".into(),
            set_by: SetBy::Asserted {
                rule: vfi_contracts::canonical_concepts::Rule {
                    registry: registry.version().rendered().into(),
                    id: assertion.rule().into(),
                },
                filing: EARLIER.into(),
            },
        }
    );
}

#[test]
fn a_tie_crosses_as_each_tied_filings_attempt_with_what_it_declined_beside_the_tie() {
    let registry = read(&committed());
    let named = |id: &str| {
        settling::named(
            registry.version(),
            &format!("short_term_investments|*|tag|element:us-gaap:{id}"),
        )
    };
    let history = run(
        &registry,
        &filer(
            FILER,
            vec![
                fact("ShortTermInvestments", "500", same_day()),
                fact("MarketableSecuritiesCurrent", "200", same_day()),
                fact("ShortTermInvestments", "600", later()),
                fact("Assets", "900", later()),
                fact("ShortTermInvestments", "700", earlier()),
            ],
        ),
    );
    let resolution = row(&history, &at(AT)).of(Concept::ShortTermInvestments);

    let Resolution::Unknown { attempted } = resolution else {
        panic!("{resolution:?}");
    };
    let tied: Vec<&str> = attempted
        .tie()
        .expect("the two same-day filings tie")
        .accessions()
        .iter()
        .map(|held| &**held)
        .collect();
    assert_eq!(tied, [SAME_DAY, LATER]);
    assert_eq!(accessions(resolution), [SAME_DAY, LATER]);

    let no_fact = "no fact answering the period asked for";
    let (afs, marketable, other) = (
        named("AvailableForSaleSecuritiesDebtSecuritiesCurrent"),
        named("MarketableSecuritiesCurrent"),
        named("OtherShortTermInvestments"),
    );
    let each = attempts(resolution);
    assert_eq!(
        declined(each[0]),
        [
            (&*afs, no_fact),
            (&*other, no_fact),
            (&*marketable, "a stand-in dropped behind an exact reading"),
        ]
    );
    assert_eq!(
        declined(each[1]),
        [
            (&*afs, no_fact),
            (&*marketable, no_fact),
            (&*other, no_fact)
        ]
    );
}

#[test]
fn a_filed_that_does_not_read_as_a_date_is_a_failure_naming_the_filing() {
    let registry = read(&committed());
    let undated = (LATER, "10-Q", "2031-13-01");

    // Alone at the instant, so Rule 1 admits no period and the run meets the
    // filing only through Rule 1's own question.
    let unadmitted = history(
        &registry,
        &filer(
            FILER,
            vec![
                fact("ShortTermInvestments", "500", earlier()),
                fact("ShortTermInvestments", "600", undated),
            ],
        ),
    )
    .expect_err("an undated filing is a stop");
    assert_eq!(unadmitted.accessions(), [Box::<str>::from(LATER)]);

    // Beside a figure that admits the instant.
    let admitted = history(
        &registry,
        &filer(
            FILER,
            vec![
                fact("ShortTermInvestments", "500", earlier()),
                fact("Assets", "900", earlier()),
                fact("ShortTermInvestments", "600", undated),
            ],
        ),
    )
    .expect_err("an undated filing is a stop");
    assert_eq!(admitted.accessions(), [Box::<str>::from(LATER)]);
}

/// The periods cross as the characters the facts carried them.
#[test]
fn a_period_crosses_as_its_dates() {
    let registry = read(&committed());
    let history = run(&registry, &recorded_in(CASE));
    let carried: Vec<Period> = recorded_in(CASE)
        .facts
        .iter()
        .map(|fact| match &fact.period {
            fetch_normalize::Period::Instant { at: date } => at(date),
            fetch_normalize::Period::Duration { start, end } => between(start, end),
        })
        .collect();
    for row in history.periods() {
        assert!(carried.contains(row.period()), "{:?}", row.period());
    }
}
