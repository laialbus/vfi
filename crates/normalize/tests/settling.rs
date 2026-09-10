//! What one concept settles to, over the facts one filer actually reported.
//!
//! Both contests `docs/adr/candidate-choice.md` names as live in the merged
//! fetch fixture are here — `gross_profit`, where the tagged figure and the
//! difference both answer and agree, and the share count, where a cover-dated
//! fact and a period-end fact are present in every filing. Each is asked under
//! the kind the filer's own registry file assigns it, and each pins the state it
//! comes to and the rule that set it.
//!
//! Where the fixture has no filer for a case, the case is planted: a copy of the
//! committed registry with one entry added, or one rule excluded for this filer,
//! or one value asserted for it. That is the recovery the record names for every
//! strictness below — a file with a rule id behind it — so exercising the cases
//! that way exercises the recovery too.
//!
//! One case is not planted but broken on purpose. The five-field identity that
//! makes one element, unit, period and filing one fact is a property of the
//! boundary, and the fixture holds it over all 1,643 of its facts, so the only
//! way to ask what happens when it does not hold is to hand the procedure a
//! filing where it does not.

mod fixture;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Kind, Resolution, Silence};
use vfi_contracts::fetch_normalize::Fact;
use vfi_normalize::registry::Registry;
use vfi_normalize::settling::{self, Settled};

use fixture::{
    ANNUAL, FILER, adding, committed, duration, filing, instant, overriding, planted, read,
    reported,
};

/// The kind the filer's own registry file assigns it, read off that file's own
/// reasoning about its statements.
const KIND: Option<Kind> = Some(Kind::Operating);

/// The year the 10-K reports, and the year end it closes at.
const YEAR: (&str, &str) = ("2023-10-01", "2024-09-30");

const GROSS_PROFIT: &str = "gross_profit|*|tag|element:us-gaap:GrossProfit";
const GROSS_PROFIT_LESS_COST: &str =
    "gross_profit|*|difference|concept:revenue+element:us-gaap:CostOfRevenue";
const COVER_COUNT: &str = "shares_outstanding|*|tag|element:dei:EntityCommonStockSharesOutstanding";
const PERIOD_END_COUNT: &str =
    "shares_outstanding|*|tag|element:us-gaap:CommonStockSharesOutstanding";
const NET_INCOME: &str = "net_income|*|tag|element:us-gaap:NetIncomeLoss";
const CONTINUING: &str = "net_income|*|tag|element:us-gaap:IncomeLossFromContinuingOperations";
const PROPERTY_PAYMENTS: &str =
    "capital_expenditure|*|tag|element:us-gaap:PaymentsToAcquirePropertyPlantAndEquipment";

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

/// A value of `amount`, set by the rule `id` under this registry's version.
fn by(registry: &Registry, amount: &str, id: &str) -> String {
    format!("{amount} by {}", settling::named(registry.version(), id))
}

/// What an attempt tried, and why each was declined.
fn tried(settled: &Settled) -> Vec<(String, String)> {
    let attempt = match settled {
        Settled::Unknown(attempt) => attempt,
        held => panic!(
            "expected an attempt that returned nothing, and got {}",
            came_to(held)
        ),
    };
    read_attempt(attempt)
}

fn read_attempt(attempt: &Attempt) -> Vec<(String, String)> {
    attempt
        .declined()
        .iter()
        .map(|held| (held.candidate.to_string(), held.rule.to_string()))
        .collect()
}

/// The first contest: the tagged figure against the difference.
///
/// This filer tags `GrossProfit`, `Revenues` and `CostOfRevenue`, so both
/// candidates answer the year and both come to 365627. The tag reads the concept
/// exactly and the difference stands in for it, so step 2 settles it — and the
/// value records the tag's rule, singular, which is what a value has a field
/// for.
#[test]
fn the_tagged_gross_profit_wins_the_contest_its_difference_also_answers() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::GrossProfit,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    assert_eq!(came_to(&settled), by(&registry, "365627", GROSS_PROFIT));
}

/// The same contest with the exact reading excluded for this filer, which is
/// what says the difference answered too rather than merely losing.
///
/// Revenues 804887 less CostOfRevenue 439260 is 365627, the number the filer
/// tagged. The concept operand is settled by this same procedure, so the
/// difference resolves only because `revenue` did.
#[test]
fn the_difference_answers_the_same_number_the_tag_does() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("the-difference-answers-too");
    overriding(&root, &format!("\n[[exclude]]\nid = \"{GROSS_PROFIT}\"\n"));
    let registry = read(&root);

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::GrossProfit,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    assert_eq!(
        came_to(&settled),
        by(&registry, "365627", GROSS_PROFIT_LESS_COST)
    );
}

/// The second contest: the cover-dated count against the period-end count.
///
/// Every filing this filer made carries both — `dei`'s cover-page count once, at
/// the filing's own date, and `us-gaap`'s twice, at the two period ends the
/// filing presents. At a period end both answer, and the period-end entry reads
/// the concept exactly while the cover-page entry stands in for it. At the cover
/// date only the cover-page entry answers, and it is the value with its own rule
/// on it, so what was read is legible rather than merely present.
#[test]
fn the_period_end_share_count_wins_the_contest_the_cover_date_count_also_answers() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());

    let at_period_end = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &instant(YEAR.1),
        &filing,
    );
    assert_eq!(
        came_to(&at_period_end),
        by(&registry, "60000000", PERIOD_END_COUNT)
    );

    let at_cover_date = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::SharesOutstanding,
        &instant("2024-11-25"),
        &filing,
    );
    assert_eq!(
        came_to(&at_cover_date),
        by(&registry, "60000000", COVER_COUNT)
    );
}

/// Two survivors are `Unknown` even where they agree.
///
/// The 10-K states `NetIncomeLoss` and
/// `NetIncomeLossAvailableToCommonStockholdersBasic` at −30810 over the same
/// year. Both entries stand in for the concept, so step 2 drops neither, and
/// neither reads a fact the other reads, so step 3 drops neither. The concept is
/// the number nobody has to choose: no tie-break is written, not the larger, not
/// the more common, and not the one a neighbouring period agrees with.
#[test]
fn two_survivors_that_agree_are_still_unknown() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("two-survivors-that-agree");
    adding(
        &root,
        "net_income",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"NetIncomeLossAvailableToCommonStockholdersBasic\" },\n]\n",
    );
    let registry = read(&root);
    let available =
        "net_income|*|tag|element:us-gaap:NetIncomeLossAvailableToCommonStockholdersBasic";

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::NetIncome,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    let one = settling::named(registry.version(), NET_INCOME);
    let other = settling::named(registry.version(), available);
    assert_eq!(
        tried(&settled),
        vec![
            (
                settling::named(registry.version(), CONTINUING),
                "no fact answering the period asked for".to_owned()
            ),
            (
                one.clone(),
                format!("undecided among survivors, which names the others: {other}")
            ),
            (
                other,
                format!("undecided among survivors, which names the others: {one}")
            ),
        ]
    );
}

/// Facts colliding on the five fields that identify a fact produce no value.
///
/// The fixture holds the property — 1,643 facts render 1,643 distinct keys — so
/// the collision is made here, as a second copy of one of its facts carrying a
/// different number. Nothing crossing the boundary says which of the two is the
/// undimensioned one, so nothing prefers either: the concept is `Unknown`, and
/// what it carries names both readings of the one rule, which is the collision.
#[test]
fn facts_that_collide_on_the_five_fields_that_identify_one_produce_no_value() {
    let facts = reported();
    let mut filing = filing(&facts, ANNUAL);

    let at = filing
        .iter()
        .position(|fact: &Fact| {
            &*fact.tag == "NetIncomeLoss" && fact.period == duration(YEAR.0, YEAR.1)
        })
        .expect("the 10-K states a net income over the year it reports");
    let mut collides = filing[at].clone();
    collides.value = "-30811".into();
    filing.push(collides);

    let registry = read(&committed());
    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::NetIncome,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    let held = settling::named(registry.version(), NET_INCOME);
    let one =
        format!("{held} reading us-gaap:NetIncomeLoss USD from 2023-10-01 to 2024-09-30 = -30810");
    let other =
        format!("{held} reading us-gaap:NetIncomeLoss USD from 2023-10-01 to 2024-09-30 = -30811");
    assert_eq!(
        tried(&settled),
        vec![
            (
                settling::named(registry.version(), CONTINUING),
                "no fact answering the period asked for".to_owned()
            ),
            (
                one.clone(),
                format!("undecided among survivors, which names the others: {other}")
            ),
            (
                other,
                format!("undecided among survivors, which names the others: {one}")
            ),
        ]
    );
}

/// One entry answering one period under two currency keys settles to neither.
///
/// The filer states `OperatingLeasePayments` over the year in both `CNY` and
/// `USD`. The boundary publishes the unit key and not the filing's reporting
/// currency, so nothing here can say which of the two the concept is measured
/// in, and it says so rather than taking one.
#[test]
fn one_entry_answering_in_two_currencies_settles_to_neither() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("two-currencies-settle-to-neither");
    adding(
        &root,
        "interest_expense",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"OperatingLeasePayments\" },\n]\n",
    );
    for id in [
        "interest_expense|*|tag|element:us-gaap:InterestExpense",
        "interest_expense|*|tag|element:us-gaap:InterestExpenseDebt",
        "interest_expense|*|tag|element:us-gaap:InterestAndDebtExpense",
        "interest_expense|*|tag|element:us-gaap:InterestExpenseBorrowings",
    ] {
        overriding(&root, &format!("\n[[exclude]]\nid = \"{id}\"\n"));
    }
    let registry = read(&root);

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::InterestExpense,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    let held = settling::named(
        registry.version(),
        "interest_expense|*|tag|element:us-gaap:OperatingLeasePayments",
    );
    assert_eq!(
        tried(&settled)
            .iter()
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<String>>(),
        vec![
            format!(
                "{held} reading us-gaap:OperatingLeasePayments CNY from 2023-10-01 to \
                 2024-09-30 = 259605"
            ),
            format!(
                "{held} reading us-gaap:OperatingLeasePayments USD from 2023-10-01 to \
                 2024-09-30 = 37000"
            ),
        ]
    );
}

/// A composition missing an operand is not a candidate, and the element it does
/// have resolves on its own.
///
/// This filer tags the property payments and nothing for software, which is the
/// registry's `capital_expenditure` sum with one of its two operands missing.
/// The sum is not a candidate — reading the missing operand as zero would read
/// silence as zero for that component — and the single element is the sole
/// candidate, which is the reading the record states for exactly this filer.
#[test]
fn a_composition_missing_an_operand_is_not_a_candidate() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::CapitalExpenditure,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    assert_eq!(came_to(&settled), by(&registry, "3724", PROPERTY_PAYMENTS));
}

/// A component never competes with a composition that contains it, and the
/// composition is composed exactly.
///
/// At 2023-09-30 this filer states Liabilities 82407, LiabilitiesCurrent 76485
/// and OperatingLeaseLiabilityNoncurrent 5922 — the classified balance sheet its
/// registry file reads its kind off. With the committed total excluded, the
/// planted sum and the planted component both read the concept exactly, so step
/// 2 settles nothing; step 3 drops the component, whose one fact is one of the
/// sum's two. What is left composes to the total the filer states.
#[test]
fn a_component_is_dropped_behind_the_composition_that_contains_it() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("the-whole-before-its-part");
    adding(
        &root,
        "total_liabilities",
        "\n[[entry]]\nform = \"sum\"\nreading = \"exact\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"LiabilitiesCurrent\" },\n  \
         { taxonomy = \"us-gaap\", tag = \"OperatingLeaseLiabilityNoncurrent\" },\n]\n\
         \n[[entry]]\nform = \"tag\"\nreading = \"exact\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"LiabilitiesCurrent\" },\n]\n",
    );
    overriding(
        &root,
        "\n[[exclude]]\nid = \"total_liabilities|*|tag|element:us-gaap:Liabilities\"\n",
    );
    let registry = read(&root);

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::TotalLiabilities,
        &instant("2023-09-30"),
        &filing,
    );

    assert_eq!(
        came_to(&settled),
        by(
            &registry,
            "82407",
            "total_liabilities|*|sum|element:us-gaap:LiabilitiesCurrent+\
             element:us-gaap:OperatingLeaseLiabilityNoncurrent"
        )
    );
}

/// An assertion covering the filer, the concept and the period is total: it is
/// the value, and no rule is looked up beneath it.
///
/// The rule the value records is the assertion's own, so an asserted number is
/// visibly asserted rather than indistinguishable from a read one — and this is
/// asserted over a period the registry's own entry answers, which is what says
/// no lookup ran beneath it.
#[test]
fn an_assertion_is_the_value_and_no_rule_is_looked_up_beneath_it() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("an-assertion-is-total");
    overriding(
        &root,
        "\n[[assert]]\nconcept = \"net_income\"\n\
         period = { start = \"2023-10-01\", end = \"2024-09-30\" }\n\
         value = \"-30810\"\n\
         source = { accession = \"0001213900-24-101777\", line = \"42\" }\n",
    );
    let registry = read(&root);

    let settled = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::NetIncome,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    assert_eq!(
        came_to(&settled),
        by(
            &registry,
            "-30810",
            "net_income|*|assert|filer:0002003750+duration:2023-10-01:2024-09-30"
        )
    );
}

/// Applicability is asked first, of the published clause alone.
///
/// `gross_profit` is published for the `operating` kind and no other, so a filer
/// asked about it under `bank` is answered before any rule is looked up — and
/// answered with the state the vocabulary makes constructible from a kind and a
/// clause, which no lookup that failed could have built.
#[test]
fn a_concept_the_kind_excludes_is_answered_before_anything_is_looked_up() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());

    let settled = settling::settle(
        &registry,
        FILER,
        Some(Kind::Bank),
        Concept::GrossProfit,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );

    match settled {
        Settled::NotApplicable(Resolution::NotApplicable { excluded }) => {
            assert_eq!(excluded.kind(), Kind::Bank);
            assert!(!excluded.clause().admits(Kind::Bank));
        }
        held => panic!(
            "expected the concept to be inapplicable, and got {}",
            came_to(&held)
        ),
    }
}

/// Where nothing matched, every concept takes the silence reading the published
/// vocabulary gives it — read off that surface here rather than restated.
///
/// The question is asked of a filing carrying no fact, because that is the one
/// way to ask it of every concept at once: the cover-page entry answers whatever
/// instant it is handed, so no period this filer does not report would leave the
/// share count silent. The conditional pair takes its zero here on the condition
/// the surface publishes with it — the period's facts are silent on both members
/// alike — which is a dividend suspension, and the reading the streak in a
/// later milestone is measured against.
#[test]
fn every_concept_takes_the_silence_reading_the_published_vocabulary_gives_it() {
    let registry = read(&committed());
    let nothing: Vec<Fact> = Vec::new();

    for concept in Concept::ALL {
        let read = concept.definition();
        let settled = settling::settle(
            &registry,
            FILER,
            KIND,
            *concept,
            &duration(YEAR.0, YEAR.1),
            &nothing,
        );

        let expected = match read.applies_to.admits(Kind::Operating) {
            false => "not applicable".to_owned(),
            true => match read.silence {
                Silence::Unknown => "unknown".to_owned(),
                Silence::Zero | Silence::Conditional => {
                    "0 by the published silence reading".to_owned()
                }
            },
        };
        assert_eq!(came_to(&settled), expected, "{concept:?}");
    }
}

/// The condition under the conditional zero, on the side that refuses it.
///
/// `dividends_declared_per_share` takes a zero where the period's facts are
/// silent on it and on `dividends_paid` alike, and the vocabulary publishes an
/// `Unknown` where the other member resolves to a non-zero value. With that
/// value asserted for this filer, the silent member is a mapping that failed
/// rather than a period with no dividend, and it reads as one.
#[test]
fn a_conditional_zero_is_refused_where_the_concept_it_names_resolves() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("the-conditional-zero-refused");
    overriding(
        &root,
        "\n[[assert]]\nconcept = \"dividends_paid\"\n\
         period = { start = \"2023-10-01\", end = \"2024-09-30\" }\n\
         value = \"1000\"\n\
         source = { accession = \"0001213900-24-101777\", line = \"58\" }\n",
    );
    let registry = read(&root);

    let paid = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::DividendsPaid,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );
    assert_eq!(
        came_to(&paid),
        by(
            &registry,
            "1000",
            "dividends_paid|*|assert|filer:0002003750+duration:2023-10-01:2024-09-30"
        )
    );

    let declared = settling::settle(
        &registry,
        FILER,
        KIND,
        Concept::DividendsDeclaredPerShare,
        &duration(YEAR.0, YEAR.1),
        &filing,
    );
    assert_eq!(came_to(&declared), "unknown");
}
