//! Which of one filing's facts answer the period asked for, over the facts one
//! filer actually reported.
//!
//! Every fact and every rule here comes out of the fixture and the registry the
//! module beside this reads them from. The mapping never puts a duration element
//! under a balance concept or an area under a currency one, and the refusals
//! still have to be checkable, so those cases are planted — each as a copy of
//! the committed registry plus the one entry the case is about, so a case that
//! answers nothing answers nothing for its own reason.

mod fixture;

use std::collections::BTreeSet;

use vfi_contracts::canonical_concepts::Concept;
use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::answering::{self, Admits, Answer};
use vfi_normalize::registry::{Operand, Outcome, Registry, Rule};

use fixture::{
    ANNUAL, FILER, QUARTERLY, adding, committed, duration, filing, instant, planted, read,
    reported, rewritten,
};

const NET_INCOME: &str = "net_income|*|tag|element:us-gaap:NetIncomeLoss";
const GROSS_PROFIT: &str = "gross_profit|*|tag|element:us-gaap:GrossProfit";
const GROSS_PROFIT_LESS_COST: &str =
    "gross_profit|*|difference|concept:revenue+element:us-gaap:CostOfRevenue";
const EQUITY: &str = "shareholders_equity|*|tag|element:us-gaap:StockholdersEquity";
const COVER_COUNT: &str = "shares_outstanding|*|tag|element:dei:EntityCommonStockSharesOutstanding";
const PERIOD_END_COUNT: &str =
    "shares_outstanding|*|tag|element:us-gaap:CommonStockSharesOutstanding";
const CAPITAL_EXPENDITURE: &str = "capital_expenditure|*|sum|\
    element:us-gaap:PaymentsToAcquirePropertyPlantAndEquipment+\
    element:us-gaap:PaymentsToDevelopSoftware";
const DILUTED_SHARES: &str = "diluted_shares_weighted_average|*|tag|\
    element:us-gaap:WeightedAverageNumberOfDilutedSharesOutstanding";
const DILUTED_EPS: &str =
    "earnings_per_share_diluted|*|tag|element:us-gaap:EarningsPerShareDiluted";

/// Every case here is asked about one filing it names, which is the caller
/// saying that this filing's own period of report ends on the period asked for
/// — the condition `docs/adr/period-alignment.md` sets for admitting the entry
/// that answers the period of the filing it was reported in. Which filings a
/// filer's facts admit it for is `vfi_normalize::filings`, and is pinned there.
fn ask<'r, 'f>(
    concept: Concept,
    rule: &'r Rule,
    period: &Period,
    filing: &[&'f Fact],
) -> Answer<'r, 'f> {
    answering::ask(concept, rule, period, filing, Admits::EveryEntry)
}

/// The rule this case is about, out of the rules the registry says are eligible
/// for the concept.
///
/// Reached through the one interface rather than built here: a rule assembled
/// in a test is a rule the registry never said was eligible, and its id — which
/// is what the case names it by — would be one nothing rendered.
fn rule<'r>(registry: &'r Registry, concept: Concept, id: &str) -> &'r Rule {
    let asked = instant("1970-01-01");
    let outcome = registry.answer(FILER, None, concept, &asked).outcome;
    let eligible = match outcome {
        Outcome::Eligible(rules) => rules,
        Outcome::Asserted(held) => panic!("expected eligible rules, and got {}", held.rule()),
    };

    eligible
        .iter()
        .find(|rule| rule.id() == id)
        .copied()
        .unwrap_or_else(|| {
            let held: Vec<&str> = eligible.iter().map(|rule| rule.id()).collect();
            panic!("no rule {id} is eligible for {concept:?}; these are: {held:?}")
        })
}

/// One fact as a case reads it back: the element, the unit it crossed in, the
/// period it is stated for, and the amount as published.
fn rendered(fact: &Fact) -> String {
    let when = match &fact.period {
        Period::Instant { at } => format!("at {at}"),
        Period::Duration { start, end } => format!("from {start} to {end}"),
    };
    format!(
        "{}:{} {} {when} = {}",
        fact.taxonomy, fact.tag, fact.unit, fact.value
    )
}

/// What answered, one list per element operand, in the rule's own order.
fn answered(answer: &Answer) -> Vec<Vec<String>> {
    answer
        .operands()
        .iter()
        .map(|operand| operand.facts().iter().map(|fact| rendered(fact)).collect())
        .collect()
}

fn named(operand: &Operand) -> String {
    match operand {
        Operand::Element { taxonomy, tag } => format!("{taxonomy}:{tag}"),
        Operand::Concept(concept) => format!("concept {concept:?}"),
    }
}

/// The three durations one filing carries of one element, told apart by their
/// dates and by nothing else.
///
/// The 10-K reports the year, the year before it, and the 24-day stub from
/// incorporation to the first year end. Two of the three carry −40502, so a
/// rule that read the value, the tag or the filing would answer the wrong one
/// and look right doing it.
#[test]
fn each_of_three_durations_under_one_element_answers_only_its_own_period() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());
    let rule = rule(&registry, Concept::NetIncome, NET_INCOME);

    for (period, expected) in [
        (
            duration("2023-10-01", "2024-09-30"),
            "us-gaap:NetIncomeLoss USD from 2023-10-01 to 2024-09-30 = -30810",
        ),
        (
            duration("2022-10-01", "2023-09-30"),
            "us-gaap:NetIncomeLoss USD from 2022-10-01 to 2023-09-30 = -40502",
        ),
        (
            duration("2023-09-07", "2023-09-30"),
            "us-gaap:NetIncomeLoss USD from 2023-09-07 to 2023-09-30 = -40502",
        ),
    ] {
        let answer = ask(Concept::NetIncome, rule, &period, &filing);
        assert_eq!(
            answered(&answer),
            vec![vec![expected.to_owned()]],
            "asked for {period:?}"
        );
    }
}

/// Equality, and nothing looser.
///
/// A day either way is a different period, and so is the year to date that ends
/// where the year does. Every period below is one this filer reports somewhere
/// or one a day from a period it reports, and none of them is answered by the
/// facts of a filing that does not carry it.
#[test]
fn no_period_but_the_one_a_fact_states_is_answered_by_it() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());
    let rule = rule(&registry, Concept::NetIncome, NET_INCOME);

    for period in [
        duration("2023-10-01", "2024-09-29"),
        duration("2023-10-01", "2024-10-01"),
        duration("2023-10-02", "2024-09-30"),
        duration("2023-09-30", "2024-09-30"),
        duration("2023-10-01", "2024-06-30"),
        duration("2024-10-01", "2024-12-31"),
    ] {
        let answer = ask(Concept::NetIncome, rule, &period, &filing);
        assert_eq!(
            answered(&answer),
            vec![Vec::<String>::new()],
            "{period:?} was answered by a filing that does not carry it"
        );
    }
}

/// A flow is answered only by a duration and a balance only by an instant,
/// checked where the filing carries the other shape at the dates in question.
///
/// The 10-K carries `StockholdersEquity` at 2024-09-30 and `NetIncomeLoss` over
/// the year ending there. Neither of those answers the other's question.
#[test]
fn neither_measure_is_answered_by_the_other_shape() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());

    let year = duration("2023-10-01", "2024-09-30");
    let year_end = instant("2024-09-30");

    let flow = rule(&registry, Concept::NetIncome, NET_INCOME);
    let balance = rule(&registry, Concept::ShareholdersEquity, EQUITY);

    assert_eq!(
        answered(&ask(Concept::NetIncome, flow, &year, &filing)),
        vec![vec![
            "us-gaap:NetIncomeLoss USD from 2023-10-01 to 2024-09-30 = -30810".to_owned()
        ]],
        "the flow is not answered at the duration the filing states it over"
    );
    assert_eq!(
        answered(&ask(
            Concept::ShareholdersEquity,
            balance,
            &year_end,
            &filing
        )),
        vec![vec![
            "us-gaap:StockholdersEquity USD at 2024-09-30 = -7749".to_owned()
        ]],
        "the balance is not answered at the instant the filing states it at"
    );

    assert_eq!(
        answered(&ask(Concept::NetIncome, flow, &year_end, &filing)),
        vec![Vec::<String>::new()],
        "a flow was answered at an instant"
    );
    assert_eq!(
        answered(&ask(Concept::ShareholdersEquity, balance, &year, &filing)),
        vec![Vec::<String>::new()],
        "a balance was answered over a duration"
    );
}

/// The same two refusals, stated where the element is the one the other measure
/// would have taken.
///
/// The mapping never puts a duration element under a balance concept or the
/// reverse, so both entries here are planted. `NetIncomeLoss` under a balance is
/// the duration read at its end date; `StockholdersEquity` under a flow is the
/// two instants that would have to be averaged. Neither is built, at either
/// shape of period, though the filing carries the facts and the dates line up.
#[test]
fn a_duration_is_not_read_at_its_end_and_two_instants_are_not_read_as_a_flow() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("neither-measure-from-the-other");
    adding(
        &root,
        "total_assets",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"NetIncomeLoss\" },\n]\n",
    );
    adding(
        &root,
        "interest_expense",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"StockholdersEquity\" },\n]\n",
    );
    let registry = read(&root);

    let year = duration("2023-10-01", "2024-09-30");
    let year_end = instant("2024-09-30");
    let year_start = instant("2023-09-30");

    let as_balance = rule(
        &registry,
        Concept::TotalAssets,
        "total_assets|*|tag|element:us-gaap:NetIncomeLoss",
    );
    for period in [year.clone(), year_end.clone()] {
        assert_eq!(
            answered(&ask(Concept::TotalAssets, as_balance, &period, &filing)),
            vec![Vec::<String>::new()],
            "a duration answered a balance at {period:?}"
        );
    }

    let as_flow = rule(
        &registry,
        Concept::InterestExpense,
        "interest_expense|*|tag|element:us-gaap:StockholdersEquity",
    );
    for period in [year, year_end, year_start] {
        assert_eq!(
            answered(&ask(Concept::InterestExpense, as_flow, &period, &filing)),
            vec![Vec::<String>::new()],
            "an instant answered a flow at {period:?}"
        );
    }
}

/// The one entry that answers a period other than its own.
///
/// The cover-page count in this 10-K is stamped 2024-11-25, three months after
/// the year end it reports. Under the entry the registry commits it answers the
/// period asked for whatever that period is, because the entry says the period
/// it answers is the filing's; under the same fact and the default reading it
/// answers only 2024-11-25, which is never a period end.
#[test]
fn the_entry_that_answers_its_filing_takes_a_fact_no_period_end_would() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let cover = "dei:EntityCommonStockSharesOutstanding shares at 2024-11-25 = 60000000";

    let registry = read(&committed());
    let borrowed = rule(&registry, Concept::SharesOutstanding, COVER_COUNT);
    let own = rule(&registry, Concept::SharesOutstanding, PERIOD_END_COUNT);

    for period in [
        instant("2024-09-30"),
        instant("2023-09-30"),
        instant("2024-11-25"),
    ] {
        assert_eq!(
            answered(&ask(Concept::SharesOutstanding, borrowed, &period, &filing)),
            vec![vec![cover.to_owned()]],
            "the cover-dated count did not answer {period:?}"
        );
    }

    assert_eq!(
        answered(&ask(
            Concept::SharesOutstanding,
            own,
            &instant("2024-09-30"),
            &filing
        )),
        vec![vec![
            "us-gaap:CommonStockSharesOutstanding shares at 2024-09-30 = 60000000".to_owned()
        ]],
        "the period-end count did not answer the period end it is stated at"
    );

    let root = planted("the-default-reading");
    rewritten(
        &root,
        "shares_outstanding",
        "answers = \"filing_reported_in\"\n",
        "",
    );
    let registry = read(&root);
    let default = rule(&registry, Concept::SharesOutstanding, COVER_COUNT);

    assert_eq!(
        answered(&ask(
            Concept::SharesOutstanding,
            default,
            &instant("2024-09-30"),
            &filing
        )),
        vec![Vec::<String>::new()],
        "the cover-dated count answered a period end under the default reading"
    );
    assert_eq!(
        answered(&ask(
            Concept::SharesOutstanding,
            default,
            &instant("2024-11-25"),
            &filing
        )),
        vec![vec![cover.to_owned()]],
        "the cover-dated count did not answer its own date under the default reading"
    );
}

/// A sum reads one fact per element operand, and the operand nothing answered
/// is reported as the operand it is.
///
/// This filer tags the property payments and nothing for software, which is the
/// registry's `capital_expenditure` sum with one of its two operands missing.
/// The answer names the missing one; it does not drop it, and it does not read
/// it as nothing spent.
#[test]
fn an_operand_nothing_answers_is_named_rather_than_read_as_zero() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());
    let rule = rule(&registry, Concept::CapitalExpenditure, CAPITAL_EXPENDITURE);

    let answer = ask(
        Concept::CapitalExpenditure,
        rule,
        &duration("2023-10-01", "2024-09-30"),
        &filing,
    );

    assert_eq!(
        answered(&answer),
        vec![
            vec![
                "us-gaap:PaymentsToAcquirePropertyPlantAndEquipment USD from 2023-10-01 to \
                 2024-09-30 = 3724"
                    .to_owned()
            ],
            Vec::<String>::new(),
        ]
    );
    assert_eq!(
        answer
            .unanswered()
            .iter()
            .map(|it| named(it))
            .collect::<Vec<String>>(),
        vec!["us-gaap:PaymentsToDevelopSoftware".to_owned()]
    );
}

/// Every operand of a composition is read at the one period asked for.
///
/// The 10-Q reports the quarter and the year to date that contains it, under
/// both of the elements this planted sum reads. Asked for either period, each
/// operand answers with that period's fact and the other period's fact is not
/// in the answer — a sum across the two would be a number the filing does not
/// state.
#[test]
fn no_composition_mixes_a_quarter_with_a_year_to_date() {
    let facts = reported();
    let filing = filing(&facts, QUARTERLY);

    let root = planted("one-period-per-composition");
    adding(
        &root,
        "pretax_income",
        "\n[[entry]]\nform = \"sum\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"Revenues\" },\n  \
         { taxonomy = \"us-gaap\", tag = \"CostOfRevenue\" },\n]\n",
    );
    let registry = read(&root);
    let rule = rule(
        &registry,
        Concept::PretaxIncome,
        "pretax_income|*|sum|element:us-gaap:Revenues+element:us-gaap:CostOfRevenue",
    );

    let quarter = duration("2024-04-01", "2024-06-30");
    assert_eq!(
        answered(&ask(Concept::PretaxIncome, rule, &quarter, &filing)),
        vec![
            vec!["us-gaap:Revenues USD from 2024-04-01 to 2024-06-30 = 260916".to_owned()],
            vec!["us-gaap:CostOfRevenue USD from 2024-04-01 to 2024-06-30 = 90569".to_owned()],
        ]
    );

    let to_date = duration("2023-10-01", "2024-06-30");
    assert_eq!(
        answered(&ask(Concept::PretaxIncome, rule, &to_date, &filing)),
        vec![
            vec!["us-gaap:Revenues USD from 2023-10-01 to 2024-06-30 = 525872".to_owned()],
            vec!["us-gaap:CostOfRevenue USD from 2023-10-01 to 2024-06-30 = 252319".to_owned()],
        ]
    );

    let neither = duration("2023-10-01", "2023-12-31");
    assert_eq!(
        answered(&ask(Concept::PretaxIncome, rule, &neither, &filing)),
        vec![Vec::<String>::new(), Vec::<String>::new()],
        "a quarter the filing reports net income over, and neither of these elements at"
    );
}

/// A difference reports its element term and says nothing about its concept
/// operand.
///
/// Whether `revenue` resolved to a `Value` is only a resolved concept's to say,
/// so it is not in the answer at all — not as answered, and not as an operand
/// nothing answered. Both of this filer's `gross_profit` entries answer the same
/// period here, which is the contest the record names; nothing in this answer
/// settles it.
#[test]
fn a_difference_answers_for_its_element_and_leaves_its_concept_operand_alone() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);
    let registry = read(&committed());
    let year = duration("2023-10-01", "2024-09-30");

    let difference = rule(&registry, Concept::GrossProfit, GROSS_PROFIT_LESS_COST);
    let answer = ask(Concept::GrossProfit, difference, &year, &filing);

    assert_eq!(
        answer.operands().len(),
        1,
        "the answer holds one reading per element operand, and the concept operand is not one"
    );
    assert_eq!(
        named(answer.operands()[0].operand()),
        "us-gaap:CostOfRevenue"
    );
    assert_eq!(
        answered(&answer),
        vec![vec![
            "us-gaap:CostOfRevenue USD from 2023-10-01 to 2024-09-30 = 439260".to_owned()
        ]]
    );
    assert!(answer.unanswered().is_empty());

    let tagged = rule(&registry, Concept::GrossProfit, GROSS_PROFIT);
    assert_eq!(
        answered(&ask(Concept::GrossProfit, tagged, &year, &filing)),
        vec![vec![
            "us-gaap:GrossProfit USD from 2023-10-01 to 2024-09-30 = 365627".to_owned()
        ]],
        "the tagged figure answers the same period, and choosing between the two is not here"
    );
}

/// A fact answers only in the unit the vocabulary names for the concept.
///
/// This filer states an area in `sqm` and per-share amounts in `USD/shares`,
/// both at periods it also reports money and counts at. A concept measured in
/// currency does not take the area, and one measured in shares does not take the
/// per-share amount — and each is checked beside the entry that does take it, so
/// the refusal is the unit rather than the period.
#[test]
fn a_fact_answers_only_in_the_unit_the_concept_is_measured_in() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("only-the-published-unit");
    adding(
        &root,
        "total_assets",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"AreaOfLand\" },\n]\n",
    );
    adding(
        &root,
        "diluted_shares_weighted_average",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"EarningsPerShareDiluted\" },\n]\n",
    );
    let registry = read(&root);
    let year = duration("2023-10-01", "2024-09-30");

    let area = rule(
        &registry,
        Concept::TotalAssets,
        "total_assets|*|tag|element:us-gaap:AreaOfLand",
    );
    assert_eq!(
        answered(&ask(
            Concept::TotalAssets,
            area,
            &instant("2023-09-01"),
            &filing
        )),
        vec![Vec::<String>::new()],
        "an area answered a concept measured in currency, at the instant it is stated at"
    );

    let per_share = rule(
        &registry,
        Concept::DilutedSharesWeightedAverage,
        "diluted_shares_weighted_average|*|tag|element:us-gaap:EarningsPerShareDiluted",
    );
    assert_eq!(
        answered(&ask(
            Concept::DilutedSharesWeightedAverage,
            per_share,
            &year,
            &filing
        )),
        vec![Vec::<String>::new()],
        "an amount per share answered a concept measured in shares"
    );

    let count = rule(
        &registry,
        Concept::DilutedSharesWeightedAverage,
        DILUTED_SHARES,
    );
    assert_eq!(
        answered(&ask(
            Concept::DilutedSharesWeightedAverage,
            count,
            &year,
            &filing
        )),
        vec![vec![
            "us-gaap:WeightedAverageNumberOfDilutedSharesOutstanding shares from 2023-10-01 to \
             2024-09-30 = 55150820"
                .to_owned()
        ]]
    );

    let registry = read(&committed());
    let eps = rule(&registry, Concept::EarningsPerShareDiluted, DILUTED_EPS);
    assert_eq!(
        answered(&ask(Concept::EarningsPerShareDiluted, eps, &year, &filing)),
        vec![vec![
            "us-gaap:EarningsPerShareDiluted USD/shares from 2023-10-01 to 2024-09-30 = -0.0006"
                .to_owned()
        ]]
    );
}

/// Two facts under one entry answering one period are both named.
///
/// This filer states its lease payments twice for the same year, once in CNY and
/// once in USD, and nothing crossing the boundary says which currency it reports
/// in. Both answer, and which of them the concept takes — if either — is settled
/// above this and not here.
#[test]
fn two_currencies_under_one_entry_are_both_named() {
    let facts = reported();
    let filing = filing(&facts, ANNUAL);

    let root = planted("two-currencies-under-one-entry");
    adding(
        &root,
        "interest_expense",
        "\n[[entry]]\nform = \"tag\"\nreading = \"stand_in\"\noperands = [\n  \
         { taxonomy = \"us-gaap\", tag = \"OperatingLeasePayments\" },\n]\n",
    );
    let registry = read(&root);
    let rule = rule(
        &registry,
        Concept::InterestExpense,
        "interest_expense|*|tag|element:us-gaap:OperatingLeasePayments",
    );

    assert_eq!(
        answered(&ask(
            Concept::InterestExpense,
            rule,
            &duration("2023-10-01", "2024-09-30"),
            &filing
        )),
        vec![vec![
            "us-gaap:OperatingLeasePayments CNY from 2023-10-01 to 2024-09-30 = 259605".to_owned(),
            "us-gaap:OperatingLeasePayments USD from 2023-10-01 to 2024-09-30 = 37000".to_owned(),
        ]]
    );
}

/// Every period this filer reports, asked of every one of its filings.
///
/// The sweep is what says the cases above are not the only ones that hold. It
/// asks each of the filer's durations of a flow and each of its instants of a
/// balance, filing by filing, and holds every answer to two things: the fact
/// answered is stated for exactly the period asked for, and it came from the
/// filing that was asked. The tally is what keeps it from passing vacuously —
/// every fact under the two elements answers exactly once across the sweep, so
/// nothing was missed and nothing answered twice.
///
/// The filer's durations run from a 24-day stub to a 366-day year, ten lengths
/// in all, and this asks every one of them.
#[test]
fn every_fact_answers_at_its_own_period_and_at_no_other() {
    let facts = reported();
    let registry = read(&committed());
    let flow = rule(&registry, Concept::NetIncome, NET_INCOME);
    let balance = rule(&registry, Concept::ShareholdersEquity, EQUITY);

    let filings: BTreeSet<&str> = facts.iter().map(|fact| &*fact.accession).collect();
    let mut durations = BTreeSet::new();
    let mut instants = BTreeSet::new();
    for fact in &facts {
        match &fact.period {
            Period::Duration { start, end } => {
                durations.insert((start.to_string(), end.to_string()));
            }
            Period::Instant { at } => {
                instants.insert(at.to_string());
            }
        }
    }

    let mut answers = 0;
    let mut silences = 0;
    for accession in &filings {
        let filing = filing(&facts, accession);

        for (start, end) in &durations {
            let period = duration(start, end);
            let answer = ask(Concept::NetIncome, flow, &period, &filing);
            answers += held(&answer, &period, accession, &mut silences);
        }

        for at in &instants {
            let period = instant(at);
            let answer = ask(Concept::ShareholdersEquity, balance, &period, &filing);
            answers += held(&answer, &period, accession, &mut silences);
        }
    }

    let stated = facts
        .iter()
        .filter(|fact| &*fact.tag == "NetIncomeLoss" || &*fact.tag == "StockholdersEquity")
        .count();
    assert_eq!(
        answers, stated,
        "the fixture states {stated} facts under the two elements swept, and {answers} answered"
    );
    assert!(
        silences > 0,
        "every filing answered every period, so the sweep asked nothing"
    );
}

/// One answer of the sweep: how many facts it holds, having checked each is
/// stated for the period asked and came from the filing asked.
fn held(answer: &Answer, period: &Period, accession: &str, silences: &mut usize) -> usize {
    let mut answered = 0;
    for operand in answer.operands() {
        if operand.facts().is_empty() {
            *silences += 1;
        }
        for fact in operand.facts() {
            assert_eq!(
                &fact.period,
                period,
                "{} answered {period:?}",
                rendered(fact)
            );
            assert_eq!(
                &*fact.accession,
                accession,
                "{} answered inside {accession}",
                rendered(fact)
            );
            answered += 1;
        }
    }
    answered
}
