//! Which canonical periods a filer has.
//!
//! The fixture cases re-derive what `docs/adr/period-alignment.md` counts on
//! the merged fetch fixture, and pin it: 32 of the 50 periods the filer's facts
//! carry, and the 18 strays by name. The hand-written cases each pin one clause
//! of Rule 1 over facts written for the case, read against the committed
//! registry under the filer the merged fixture records, whose kind is
//! `operating`.

mod fixture;

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::periods::periods;
use vfi_normalize::registry::{Answers, Registry};
use vfi_normalize::settling::SetBy;
use vfi_normalize::standing::{Stands, stands};

use fixture::{FILER, committed, duration, instant, read, reported};

const KIND: Option<Kind> = Some(Kind::Operating);

/// A balance sheet date and a quarter no merged fixture reaches.
const AT: &str = "2030-06-30";
const QUARTER: (&str, &str) = ("2030-04-01", "2030-06-30");

const EARLIER: &str = "0000000001-30-000001";
const LATER: &str = "0000000001-31-000001";
const ELSEWHERE: &str = "0000000001-32-000001";

/// An element no rule of the committed registry names.
const UNMAPPED: &str = "AreaOfLand";

fn fact(taxonomy: &str, tag: &str, unit: &str, period: &Period, accession: &str) -> Fact {
    Fact {
        taxonomy: taxonomy.into(),
        tag: tag.into(),
        unit: unit.into(),
        period: period.clone(),
        value: "900".into(),
        accession: accession.into(),
        form: "10-Q".into(),
        filed: match accession {
            EARLIER => "2030-08-01",
            LATER => "2031-08-01",
            _ => "2032-08-01",
        }
        .into(),
        report_period_end: "".into(),
    }
}

fn unmapped(period: &Period, accession: &str) -> Fact {
    fact("us-gaap", UNMAPPED, "sqm", period, accession)
}

fn quarter() -> Period {
    duration(QUARTER.0, QUARTER.1)
}

/// What stands at `period` for each concept, keeping only the values.
fn values<'r, 'f>(
    registry: &'r Registry,
    period: &Period,
    facts: &'f [Fact],
) -> Vec<(Concept, SetBy<'r, 'f>)> {
    Concept::ALL
        .iter()
        .filter_map(
            |concept| match stands(registry, FILER, KIND, *concept, period, facts) {
                Some(Stands::Value(value)) => Some((*concept, value.set_by().clone())),
                _ => None,
            },
        )
        .collect()
}

fn instants(held: &[&Period]) -> usize {
    held.iter()
        .filter(|period| matches!(period, Period::Instant { .. }))
        .count()
}

#[test]
fn the_filer_has_the_32_periods_the_record_counts() {
    let registry = read(&committed());
    let facts = reported();

    let mut carried: Vec<&Period> = Vec::new();
    for fact in &facts {
        if !carried.contains(&&fact.period) {
            carried.push(&fact.period);
        }
    }
    assert_eq!(carried.len(), 50, "the premise: the facts carry 50 periods");

    let held = periods(&registry, FILER, KIND, &facts);
    assert_eq!(held.len(), 32);
    assert_eq!(instants(&held), 13);
    assert_eq!(held.len() - instants(&held), 19);
    for period in &held {
        assert!(
            carried.contains(period),
            "{period:?} is not a period a fact carries"
        );
    }
}

#[test]
fn the_18_left_out_are_the_strays_the_record_names() {
    let registry = read(&committed());
    let facts = reported();
    let held = periods(&registry, FILER, KIND, &facts);

    let mut left: Vec<&Period> = Vec::new();
    for fact in &facts {
        if !held.contains(&&fact.period) && !left.contains(&&fact.period) {
            left.push(&fact.period);
        }
    }

    let lease_terms = [
        duration("2023-09-01", "2024-11-30"),
        duration("2024-09-01", "2024-11-30"),
        duration("2024-12-01", "2025-11-30"),
    ];
    let tax_years = [
        duration("2024-01-01", "2024-09-30"),
        duration("2024-01-01", "2024-12-31"),
    ];
    let lease_instants = [
        instant("2023-09-01"),
        instant("2024-10-09"),
        instant("2025-12-01"),
    ];
    let mut filed: Vec<&str> = facts.iter().map(|fact| &*fact.filed).collect();
    filed.sort_unstable();
    filed.dedup();
    assert_eq!(filed.len(), 10, "the premise: ten filings");
    let cover_dates: Vec<Period> = filed.iter().map(|on| instant(on)).collect();

    let named: Vec<&Period> = lease_terms
        .iter()
        .chain(&tax_years)
        .chain(&lease_instants)
        .chain(&cover_dates)
        .collect();
    assert_eq!(left.len(), 18);
    assert_eq!(named.len(), 18);
    for period in &named {
        assert!(left.contains(period), "{period:?} is in the set");
    }
}

/// The rule declining to correct the filer: equity a year before it was
/// incorporated, and a net loss it also reports against its inception stub.
#[test]
fn a_period_is_not_dropped_for_being_implausible() {
    let registry = read(&committed());
    let facts = reported();
    let held = periods(&registry, FILER, KIND, &facts);

    assert!(held.contains(&&instant("2022-09-30")));
    assert!(held.contains(&&duration("2022-10-01", "2023-09-30")));
}

#[test]
fn a_filer_with_no_kind_has_no_periods() {
    let registry = read(&committed());
    let facts = reported();

    assert!(periods(&registry, FILER, None, &facts).is_empty());
}

#[test]
fn the_set_holds_each_period_once_and_tells_an_instant_from_a_duration() {
    let registry = read(&committed());
    let facts = vec![
        fact("us-gaap", "Revenues", "USD", &quarter(), EARLIER),
        fact("us-gaap", "Revenues", "USD", &quarter(), LATER),
        fact("us-gaap", "Assets", "USD", &instant(QUARTER.1), EARLIER),
    ];

    let held = periods(&registry, FILER, KIND, &facts);
    assert_eq!(held.len(), 2);
    assert!(held.contains(&&quarter()));
    assert!(held.contains(&&instant(QUARTER.1)));
}

/// Revenue is reached and does not settle, one entry answering in two
/// currencies, and nothing else is reached. The dividend pair's silence zero
/// still stands at any duration a filing answers, so it is the next case's.
#[test]
fn a_period_at_which_no_concept_resolves_is_not_the_filers() {
    let registry = read(&committed());
    let facts = vec![
        fact("us-gaap", "Revenues", "USD", &quarter(), EARLIER),
        fact("us-gaap", "Revenues", "EUR", &quarter(), EARLIER),
    ];

    assert!(
        matches!(
            stands(&registry, FILER, KIND, Concept::Revenue, &quarter(), &facts),
            Some(Stands::Unknown(_))
        ),
        "the premise: revenue is unknown at the quarter"
    );
    let stood = values(&registry, &quarter(), &facts);
    assert!(
        stood
            .iter()
            .all(|(_, set_by)| matches!(set_by, SetBy::Silence { .. })),
        "the premise: no value set through an entry stands at the quarter: {stood:?}"
    );
    assert!(periods(&registry, FILER, KIND, &facts).is_empty());
}

#[test]
fn a_period_only_a_silence_reading_supplies_is_not_the_filers() {
    let registry = read(&committed());
    let at = instant(AT);
    let facts = vec![unmapped(&at, EARLIER)];

    let stood = values(&registry, &at, &facts);
    assert!(
        !stood.is_empty()
            && stood
                .iter()
                .all(|(_, set_by)| matches!(set_by, SetBy::Silence { .. })),
        "the premise: a silence zero stands at the instant, and nothing else does: {stood:?}"
    );
    assert!(periods(&registry, FILER, KIND, &facts).is_empty());
}

#[test]
fn a_period_only_the_cover_page_entry_reaches_is_not_the_filers() {
    let registry = read(&committed());
    let at = instant(AT);
    let mut cover = fact(
        "dei",
        "EntityCommonStockSharesOutstanding",
        "shares",
        &at,
        EARLIER,
    );
    cover.report_period_end = AT.into();
    let facts = vec![cover];

    let stood = values(&registry, &at, &facts);
    assert!(
        stood.iter().any(|(concept, set_by)| *concept == Concept::SharesOutstanding
            && matches!(set_by, SetBy::Rule { rule, .. } if rule.answers() == Answers::FilingReportedIn)),
        "the premise: the cover-page entry sets shares outstanding at the instant: {stood:?}"
    );
    assert!(
        stood.iter().all(|(concept, set_by)| match set_by {
            SetBy::Rule { rule, .. } => rule.answers() == Answers::FilingReportedIn,
            SetBy::Assertion { .. } => false,
            SetBy::Silence { .. } => *concept != Concept::SharesOutstanding,
        }),
        "the premise: nothing else but a silence reading stands there: {stood:?}"
    );
    assert!(periods(&registry, FILER, KIND, &facts).is_empty());
}

/// Resolved in the earlier filing; answered and unresolved in the later one,
/// whose `Unknown` displaces nothing; and not answered at all in a third.
#[test]
fn a_period_resolved_in_one_filing_and_in_no_other_is_the_filers() {
    let registry = read(&committed());
    let facts = vec![
        fact("us-gaap", "Revenues", "USD", &quarter(), EARLIER),
        unmapped(&quarter(), LATER),
        unmapped(&instant(AT), ELSEWHERE),
    ];

    let held = periods(&registry, FILER, KIND, &facts);
    assert_eq!(held, vec![&quarter()]);
}
