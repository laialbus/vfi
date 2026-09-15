//! Which of a filer's filings answer a period, and what a concept comes to
//! inside each of them, over the facts one filer actually reported.
//!
//! Every case reads the merged fetch fixture, and the committed registry unless
//! it says otherwise. The three shapes `docs/adr/period-alignment.md` argues
//! Rule 2 from are all in that one filer: a quarter it published in full and
//! then quoted for two years in filings that carry two facts of it; a quarter it
//! never filed a report of its own for, reported in full by a filing a year
//! later; and the instant where the only entry that could reach
//! `shares_outstanding` is the cover-page one, which no filing answering that
//! instant admits.
//!
//! Where the cover-page entry is admitted, this filer also tagged the
//! period-end count, which wins. So the cases that show the admission itself
//! exclude the period-end entry in a planted copy of the registry, leaving the
//! cover-page count as the only way to the concept.
//!
//! Nothing here asks which of several answering filings wins, because nothing
//! under test decides that. What the cases pin is that every filing carrying a
//! fact stated for the period is asked, that each is asked over its own facts,
//! and that no attempt reads a fact of another filing.

mod fixture;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Kind};
use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::answering::Admits;
use vfi_normalize::filings::{self, Attempted, Filing};
use vfi_normalize::registry::{Reading, Registry};
use vfi_normalize::settling::{self, SetBy, Settled};

use fixture::{FILER, committed, duration, instant, overriding, planted, read, reported};

/// The kind the filer's own registry file assigns it.
const KIND: Option<Kind> = Some(Kind::Operating);

/// The quarter the filer published in full and went on quoting: six filings
/// carry a fact stated for it, and the last four carry two apiece.
const QUARTER: (&str, &str) = ("2024-10-01", "2024-12-31");

/// The quarter no filing of this filer reports as its own. Its first periodic
/// report is the 10-Q filed 2024-05-08, a quarter later than the earliest
/// period its facts describe, so what reports this one in full is a comparative
/// column filed a year after it closed.
const QUOTED: (&str, &str) = ("2023-10-01", "2023-12-31");

/// The instant no filing of this filer tags a period-end share count at, and
/// the two either side of it that it does.
const NO_COUNT: &str = "2023-12-31";
const COUNTED: &str = "2024-03-31";

/// The 10-Q filed 2024-05-08, whose own period of report ends on [`COUNTED`].
const REPORTS_COUNTED: &str = "0001213900-24-040632";

/// The 10-Q filed 2026-08-13, which the filer's submissions history publishes
/// no report date for.
const NO_REPORT_DATE: &str = "0001213900-26-088707";

/// The 10-Q filed 2025-08-20, the latest of the five filings that answer
/// [`NO_COUNT`] and the one `docs/adr/period-alignment.md` prices the cover-page
/// admission on. Named by its accession, because nothing under test orders two
/// filings by anything.
const NINETEEN_MONTHS_LATER: &str = "0001213900-25-078735";

const NET_INCOME: &str = "net_income|*|tag|element:us-gaap:NetIncomeLoss";
const REVENUE: &str = "revenue|operating|tag|element:us-gaap:Revenues";
const COVER_COUNT: &str = "shares_outstanding|*|tag|element:dei:EntityCommonStockSharesOutstanding";
const PERIOD_END_COUNT: &str =
    "shares_outstanding|*|tag|element:us-gaap:CommonStockSharesOutstanding";

/// A filing as a case names it, with how many of its facts are stated for the
/// period it answers — which is what tells the report of a quarter from a
/// comparative column quoting it, and is read by nothing under test.
fn carrying<'f>(answering: &[Filing<'f>], period: &Period) -> Vec<(&'f str, usize)> {
    answering
        .iter()
        .map(|filing| {
            let at = filing
                .facts()
                .iter()
                .filter(|fact| fact.period == *period)
                .count();
            (filing.accession(), at)
        })
        .collect()
}

/// What a settlement came to, as a case reads it back: the state, and for a
/// value the amount together with the rule that set it.
fn came_to(settled: &Settled) -> String {
    match settled {
        Settled::NotApplicable(_) => "not applicable".to_owned(),
        Settled::Unknown(_) => "unknown".to_owned(),
        Settled::Value(value) => match value.set_by().rule() {
            Some(rule) => format!("{} by {rule}", value.amount()),
            None => format!("{} by the published silence reading", value.amount()),
        },
    }
}

/// What each answering filing came to, in the order they arrived in.
fn each<'f>(attempted: &[Attempted<'_, 'f>]) -> Vec<(&'f str, String)> {
    attempted
        .iter()
        .map(|held| (held.accession(), came_to(held.settled())))
        .collect()
}

/// A value of `amount`, set by the rule `id` under this registry's version.
fn by(registry: &Registry, amount: &str, id: &str) -> String {
    format!("{amount} by {}", settling::named(registry.version(), id))
}

fn read_attempt(attempt: &Attempt) -> Vec<(String, String)> {
    attempt
        .declined()
        .iter()
        .map(|held| (held.candidate.to_string(), held.rule.to_string()))
        .collect()
}

/// A filing answers a period by carrying one fact stated for it, and by nothing
/// else.
///
/// Six filings answer this quarter. Two report it — 40 facts and 37 — and four
/// quote two lines of it apiece, `NetIncomeLoss` and an other-comprehensive-
/// income line that the equity roll-forward and the cash flow statement carry
/// for a period they are not about. Two facts and forty make a filing an answer
/// alike, and the four thin ones are here rather than filtered out because a
/// count of facts is not something Rule 2 reads.
#[test]
fn a_filing_answers_a_period_by_carrying_one_fact_stated_for_it() {
    let facts = reported();
    let period = duration(QUARTER.0, QUARTER.1);

    assert_eq!(
        carrying(&filings::answering(&facts, &period), &period),
        vec![
            ("0001213900-25-013785", 40),
            ("0001213900-25-042964", 2),
            ("0001213900-25-078735", 2),
            ("0001213900-26-018584", 37),
            ("0001213900-26-059248", 2),
            ("0001213900-26-088707", 2),
        ]
    );
}

/// A period no filing reports as its own is answered by the filings that quote
/// it.
///
/// The filer never filed a 10-Q for this quarter, and it is reported in full
/// all the same — 38 facts in the 10-Q for the following year's first quarter,
/// filed 2025-02-14. A rule keyed to the filing whose period it is would find
/// none and drop a quarter the filer published; a rule keyed to the newest
/// filing that mentions it would keep two facts of the thirty-eight.
#[test]
fn a_period_no_filing_reports_as_its_own_is_answered_by_the_filings_that_quote_it() {
    let facts = reported();
    let period = duration(QUOTED.0, QUOTED.1);

    assert_eq!(
        carrying(&filings::answering(&facts, &period), &period),
        vec![
            ("0001213900-24-040632", 2),
            ("0001213900-24-067900", 2),
            ("0001213900-25-013785", 38),
            ("0001213900-25-042964", 2),
            ("0001213900-25-078735", 2),
        ]
    );
}

/// The attempt runs once inside each answering filing, and each answers only
/// what it carries.
///
/// `net_income` is one of the two lines every one of the six quotes, so all six
/// resolve it and agree. `revenue` is in the two that report the quarter and in
/// neither of the four that quote it, so those four are `Unknown` — the state
/// that says a filing was consulted and gave nothing, which is not the same as
/// the quarter having no revenue. Reading the newest filing alone would leave
/// the second line looking like the first.
#[test]
fn the_attempt_runs_once_inside_each_answering_filing_over_that_filing_alone() {
    let facts = reported();
    let registry = read(&committed());
    let period = duration(QUARTER.0, QUARTER.1);
    let answering = filings::answering(&facts, &period);

    let net_income = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::NetIncome,
        &period,
        &answering,
    );
    let held = by(&registry, "166993", NET_INCOME);
    assert_eq!(
        each(&net_income),
        vec![
            ("0001213900-25-013785", held.clone()),
            ("0001213900-25-042964", held.clone()),
            ("0001213900-25-078735", held.clone()),
            ("0001213900-26-018584", held.clone()),
            ("0001213900-26-059248", held.clone()),
            ("0001213900-26-088707", held),
        ]
    );

    let revenue = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::Revenue,
        &period,
        &answering,
    );
    let reported_in_full = by(&registry, "1022155", REVENUE);
    assert_eq!(
        each(&revenue),
        vec![
            ("0001213900-25-013785", reported_in_full.clone()),
            ("0001213900-25-042964", "unknown".to_owned()),
            ("0001213900-25-078735", "unknown".to_owned()),
            ("0001213900-26-018584", reported_in_full),
            ("0001213900-26-059248", "unknown".to_owned()),
            ("0001213900-26-088707", "unknown".to_owned()),
        ]
    );
}

/// No value reads a fact of another filing.
///
/// Asked of every concept the vocabulary publishes, at both periods several
/// filings answer, which is where a composition drawn across the union of them
/// would show: `gross_profit` as a revenue less a cost, or a `sum`, taking one
/// operand from the filing that reports the quarter and the other from a
/// comparative column that quotes it. Every fact behind every value carries the
/// accession of the filing its attempt ran inside, and the count is asserted so
/// that a run resolving nothing cannot pass this vacuously.
#[test]
fn no_value_reads_a_fact_of_another_filing() {
    let facts = reported();
    let registry = read(&committed());

    let mut checked = 0;
    for period in [
        duration(QUARTER.0, QUARTER.1),
        duration(QUOTED.0, QUOTED.1),
        instant(COUNTED),
    ] {
        let answering = filings::answering(&facts, &period);
        for concept in Concept::ALL {
            for held in filings::attempted(&registry, FILER, KIND, *concept, &period, &answering) {
                let Settled::Value(value) = held.settled() else {
                    continue;
                };
                let SetBy::Rule { facts, .. } = value.set_by() else {
                    continue;
                };
                for fact in facts {
                    assert_eq!(
                        &*fact.accession,
                        held.accession(),
                        "{concept:?} at {period:?} read a fact of another filing"
                    );
                    checked += 1;
                }
            }
        }
    }

    assert!(checked > 0, "no value resolved, so nothing was checked");
}

/// The share count at an instant no answering filing reports as its own, and
/// that the filer tagged no period-end count for, is absent with its reason.
///
/// Five filings answer 2023-12-31 and not one tags
/// `CommonStockSharesOutstanding` there, so both entries the registry reaches
/// the concept through decline for the same reason: no fact of the filing
/// answers the period asked for. The cover-page entry declines because none of
/// the five has a period of report ending that day, and the record's direction
/// for that is the absence rather than the count.
#[test]
fn the_share_count_is_absent_where_no_answering_filing_reports_the_instant_as_its_own() {
    let facts = reported();
    let registry = read(&committed());
    let period = instant(NO_COUNT);
    let answering = filings::answering(&facts, &period);

    let attempted = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &period,
        &answering,
    );

    assert_eq!(
        each(&attempted),
        vec![
            ("0001213900-24-040632", "unknown".to_owned()),
            ("0001213900-24-067900", "unknown".to_owned()),
            ("0001213900-25-013785", "unknown".to_owned()),
            ("0001213900-25-042964", "unknown".to_owned()),
            ("0001213900-25-078735", "unknown".to_owned()),
        ]
    );

    let no_fact = "no fact answering the period asked for".to_owned();
    for held in &attempted {
        let Settled::Unknown(attempt) = held.settled() else {
            unreachable!("every attempt above returned nothing")
        };
        assert_eq!(
            read_attempt(attempt),
            vec![
                (
                    settling::named(registry.version(), COVER_COUNT),
                    no_fact.clone()
                ),
                (
                    settling::named(registry.version(), PERIOD_END_COUNT),
                    no_fact.clone()
                ),
            ],
            "{}",
            held.accession()
        );
    }
}

/// What admitting the cover-page entry would have cost, measured on the same
/// instant, against what the filer itself states either side of it.
///
/// Asked with every entry admitted — which is the caller saying this filing's
/// own period of report ends on 2023-12-31, and it does not — the 10-Q filed
/// 2025-08-20 hands the concept the count on its own cover page, 60,500,000,
/// nineteen months after the instant asked about. The filer's own period-end
/// count at the quarter three months later is 60,000,000. The wrong number and
/// the absence are what this boundary chooses between, and it takes the
/// absence.
#[test]
fn the_count_the_refused_entry_would_have_supplied_is_not_the_filers_own() {
    let facts = reported();
    let registry = read(&committed());

    let quoted: Vec<&Fact> = facts
        .iter()
        .filter(|fact| &*fact.accession == NINETEEN_MONTHS_LATER)
        .collect();
    let admitted = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &instant(NO_COUNT),
        &quoted,
        Admits::EveryEntry,
    );
    assert_eq!(came_to(&admitted), by(&registry, "60500000", COVER_COUNT));

    let counted = instant(COUNTED);
    let answering = filings::answering(&facts, &counted);
    let attempted = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &counted,
        &answering,
    );
    assert_eq!(
        each(&attempted),
        vec![
            (
                "0001213900-24-040632",
                by(&registry, "60000000", PERIOD_END_COUNT)
            ),
            ("0001213900-24-067900", "unknown".to_owned()),
            ("0001213900-24-101777", "unknown".to_owned()),
            ("0001213900-25-042964", "unknown".to_owned()),
            ("0001213900-25-078735", "unknown".to_owned()),
        ]
    );
}

/// A copy of the committed registry in which the filer excludes the period-end
/// count, so the cover-page count is the only entry left for the concept.
fn cover_count_alone(case: &str) -> Registry {
    let root = planted(case);
    overriding(
        &root,
        &format!("\n[[exclude]]\nid = \"{PERIOD_END_COUNT}\"\n"),
    );
    read(&root)
}

/// What each attempt declined, where every attempt came to `Unknown`.
fn declined_in_each(attempted: &[Attempted]) -> Vec<Vec<(String, String)>> {
    attempted
        .iter()
        .map(|held| match held.settled() {
            Settled::Unknown(attempt) => read_attempt(attempt),
            _ => panic!("{} settled to a value or an exclusion", held.accession()),
        })
        .collect()
}

/// The cover-page count is admitted inside the filing whose own period of
/// report ends on the instant asked for, and inside no other filing answering
/// that instant.
///
/// The 10-Q filed 2024-05-08 reports the quarter ending 2024-03-31, and with
/// the period-end entry excluded its cover count, dated 2024-05-08, is the one
/// candidate there. It stands in for the concept, so the value names the
/// stand-in rule. The four later filings quoting that instant report periods
/// ending later, so the same entry reads nothing in them.
#[test]
fn the_cover_page_count_is_admitted_where_the_filings_period_of_report_ends_on_the_instant() {
    let facts = reported();
    let registry = cover_count_alone("cover-count-admitted-at-its-report-date");
    let period = instant(COUNTED);
    let answering = filings::answering(&facts, &period);

    let attempted = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &period,
        &answering,
    );
    assert_eq!(
        each(&attempted),
        vec![
            (REPORTS_COUNTED, by(&registry, "60000000", COVER_COUNT)),
            ("0001213900-24-067900", "unknown".to_owned()),
            ("0001213900-24-101777", "unknown".to_owned()),
            ("0001213900-25-042964", "unknown".to_owned()),
            ("0001213900-25-078735", "unknown".to_owned()),
        ]
    );

    let Settled::Value(value) = attempted[0].settled() else {
        unreachable!("the first attempt above is a value")
    };
    let SetBy::Rule { rule, facts, .. } = value.set_by() else {
        unreachable!("the value above was set by a rule")
    };
    assert_eq!(rule.reading(), Reading::StandIn);
    let read: Vec<(&str, &str, &Period, &str)> = facts
        .iter()
        .map(|fact| (&*fact.tag, &*fact.accession, &fact.period, &*fact.value))
        .collect();
    assert_eq!(
        read,
        vec![(
            "EntityCommonStockSharesOutstanding",
            REPORTS_COUNTED,
            &instant("2024-05-08"),
            "60000000"
        )]
    );
}

/// The cover-page count is admitted at no instant that no answering filing
/// reports as its own, and at no duration, whatever the duration ends on.
///
/// At 2023-12-31 the five answering filings report periods ending 2024-03-31
/// and later, so the 60,500,000 on the cover of the 10-Q filed 2025-08-20 is
/// still no candidate. At the quarter ending 2024-03-31, which the 10-Q filed
/// 2024-05-08 reports, the concept is a balance and a duration answers none,
/// so that filing admits the entry no more than the others do.
#[test]
fn the_cover_page_count_is_refused_at_an_instant_no_filing_reports_and_at_every_duration() {
    let facts = reported();
    let registry = cover_count_alone("cover-count-refused-elsewhere");
    let no_fact = vec![(
        settling::named(registry.version(), COVER_COUNT),
        "no fact answering the period asked for".to_owned(),
    )];

    for (period, answered_by) in [
        (
            instant(NO_COUNT),
            vec![
                "0001213900-24-040632",
                "0001213900-24-067900",
                "0001213900-25-013785",
                "0001213900-25-042964",
                NINETEEN_MONTHS_LATER,
            ],
        ),
        (
            duration("2024-01-01", COUNTED),
            vec![
                REPORTS_COUNTED,
                "0001213900-24-067900",
                "0001213900-25-042964",
                "0001213900-25-078735",
            ],
        ),
    ] {
        let answering = filings::answering(&facts, &period);
        let attempted = filings::attempted(
            &registry,
            FILER,
            KIND,
            Concept::SharesOutstanding,
            &period,
            &answering,
        );
        assert_eq!(
            each(&attempted),
            answered_by
                .iter()
                .map(|accession| (*accession, "unknown".to_owned()))
                .collect::<Vec<_>>(),
            "{period:?}"
        );
        assert_eq!(
            declined_in_each(&attempted),
            vec![no_fact.clone(); answered_by.len()],
            "{period:?}"
        );
    }
}

/// A filing the history names no report date for admits the cover-page count
/// nowhere, even at the instant its own statements are dated.
///
/// The 10-Q filed 2026-08-13 is the one filing answering 2026-06-30 and carries
/// a cover count of 64,125,000, which it would hand over if asked with every
/// entry. Its `report_period_end` is empty, so the condition is not met, and
/// the concept is `Unknown` exactly as it was before the field crossed.
#[test]
fn a_filing_with_no_report_period_end_never_admits_the_cover_page_count() {
    let facts = reported();
    let registry = cover_count_alone("cover-count-without-a-report-date");
    let period = instant("2026-06-30");
    let answering = filings::answering(&facts, &period);

    let attempted = filings::attempted(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &period,
        &answering,
    );
    assert_eq!(
        each(&attempted),
        vec![(NO_REPORT_DATE, "unknown".to_owned())]
    );
    assert_eq!(
        declined_in_each(&attempted),
        vec![vec![(
            settling::named(registry.version(), COVER_COUNT),
            "no fact answering the period asked for".to_owned(),
        )]]
    );

    let admitted = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &period,
        answering[0].facts(),
        Admits::EveryEntry,
    );
    assert_eq!(came_to(&admitted), by(&registry, "64125000", COVER_COUNT));
}

/// With the committed registry the period-end count still sets the share count
/// inside every filing at its own report date, so admitting the cover-page
/// count there moves no number.
///
/// This filer tags `CommonStockSharesOutstanding` at each of its nine report
/// dates in the filing that reports it. That entry reads the concept exactly and
/// the cover-page one stands in, so the cover count drops behind it.
#[test]
fn the_period_end_count_still_wins_at_every_report_date() {
    let facts = reported();
    let registry = read(&committed());

    let expected = [
        ("2024-03-31", REPORTS_COUNTED, "60000000"),
        ("2024-06-30", "0001213900-24-067900", "60000000"),
        ("2024-09-30", "0001213900-24-101777", "60000000"),
        ("2024-12-31", "0001213900-25-013785", "60000000"),
        ("2025-03-31", "0001213900-25-042964", "60500000"),
        ("2025-06-30", "0001213900-25-078735", "60500000"),
        ("2025-09-30", "0001213900-26-002691", "60500000"),
        ("2025-12-31", "0001213900-26-018584", "60500000"),
        ("2026-03-31", "0001213900-26-059248", "64125000"),
    ];

    let mut report_dates: Vec<&str> = facts
        .iter()
        .map(|fact| &*fact.report_period_end)
        .filter(|date| !date.is_empty())
        .collect();
    report_dates.sort_unstable();
    report_dates.dedup();
    assert_eq!(
        report_dates,
        expected.iter().map(|(date, ..)| *date).collect::<Vec<_>>()
    );

    for (date, reports, count) in expected {
        let period = instant(date);
        let answering = filings::answering(&facts, &period);
        let attempted = filings::attempted(
            &registry,
            FILER,
            KIND,
            Concept::SharesOutstanding,
            &period,
            &answering,
        );
        let own = attempted
            .iter()
            .find(|held| held.accession() == reports)
            .unwrap_or_else(|| panic!("{reports} does not answer {date}"));
        assert_eq!(
            came_to(own.settled()),
            by(&registry, count, PERIOD_END_COUNT),
            "{date}"
        );
    }
}

/// A period no fact of this filer carries is answered by no filing at all, and
/// nothing is attempted for it.
///
/// The empty answer rather than every filing, or the nearest one: a period a
/// caller constructed from a calendar of its own is a period this filer never
/// reported, and what it resolves to is nothing.
#[test]
fn a_period_no_fact_carries_is_answered_by_no_filing() {
    let facts = reported();
    let registry = read(&committed());
    let period = duration("2020-01-01", "2020-12-31");

    let answering = filings::answering(&facts, &period);
    assert!(answering.is_empty());
    assert!(
        filings::attempted(
            &registry,
            FILER,
            KIND,
            Concept::NetIncome,
            &period,
            &answering
        )
        .is_empty()
    );
}
