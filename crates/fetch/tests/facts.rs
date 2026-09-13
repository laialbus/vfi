//! What crosses the boundary when a filer's facts are retrieved, and what the
//! stage answers when EDGAR answers something other than that filer's facts.
//!
//! The real document is pinned by the golden fixture, which is a recording of
//! one. What is here is what a recording cannot show: a document filed under
//! the wrong key, one whose key is spelled no way EDGAR spells one, one that is
//! not the document at all, an amount published as something other than a
//! number, and a history that names a filing two ways. Those are written by
//! hand rather than fetched — the thing this project trusts least — so each
//! case turns on one thing being wrong and asserts the answer to that flaw,
//! never that a made-up document parses.
//!
//! The first case is the exception and is written by hand for a different
//! reason. What it pins is the order the facts leave in and the characters they
//! leave as, and both need a document small enough to write out in full and
//! wide enough to have an order — which no recording of a real filer is. Its
//! amounts are the two a reading would spoil: a decimal whose trailing zero a
//! float would drop, and an integer larger than a float holds exactly.
//!
//! Nothing here reaches the network. The transport answers from a table the
//! case wrote and has no wire under it.

use std::collections::BTreeMap;
use std::io;

use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_fetch::{
    Cik, Cleared, Declaration, Egress, Pace, Response, Transport, Unretrieved, company_facts,
};

const FACTS: &str = "https://data.sec.gov/api/xbrl/companyfacts/CIK0002003750.json";
const SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0002003750.json";
const FILER: u64 = 2003750;

/// The filing every fact in [`document_of`] was reported in, and the date the
/// history publishes its period of report as ending on.
const QUARTERLY: &str = "0001213900-25-042964";
const QUARTER_END: &str = "2025-03-31";

/// A transport answering out of a table, keeping the URLs it was asked for in
/// the order they arrived — which is how a case says what was requested, and
/// what was not.
#[derive(Default)]
struct Answers {
    documents: BTreeMap<String, (u16, String)>,
    asked: Vec<String>,
}

impl Answers {
    fn to(mut self, url: &str, status: u16, body: &str) -> Self {
        self.documents
            .insert(url.to_owned(), (status, body.to_owned()));
        self
    }
}

impl Transport for Answers {
    fn send(&mut self, request: Cleared<'_>) -> io::Result<Response> {
        let url = request.url();
        self.asked.push(url.to_owned());

        let (status, body) = self
            .documents
            .get(url)
            .unwrap_or_else(|| panic!("{url}: this case says nothing about that request"));

        Ok(Response {
            status: *status,
            body: body.as_bytes().to_vec(),
        })
    }
}

fn asking(answers: Answers) -> Egress<Answers> {
    let declaration =
        Declaration::new("VFI test suite nobody@example.invalid").expect("this names somebody");

    Egress::new(answers, declaration, Pace::system())
}

/// The filer's history and `facts` as its company facts document, which is
/// every request a retrieval of its facts makes.
fn answering(facts: &str) -> Answers {
    Answers::default()
        .to(SUBMISSIONS, 200, &history_of(&[(QUARTERLY, QUARTER_END)]))
        .to(FACTS, 200, facts)
}

/// A submissions document naming the filings given, each with the report date
/// it publishes for it. The other columns are filled because the stage reads
/// them; `size` is one it does not read, left in because a document carries
/// several of those.
fn history_of(filings: &[(&str, &str)]) -> String {
    let column = |value: &dyn Fn(&(&str, &str)) -> String| {
        filings.iter().map(value).collect::<Vec<_>>().join(",")
    };

    format!(
        r#"{{"cik":"0002003750","name":"Maitong Sunshine Cultural Development Co., Ltd",
            "filings":{{"recent":{{"accessionNumber":[{}],"filingDate":[{}],
            "reportDate":[{}],"form":[{}],"primaryDocument":[{}],"size":[]}},"files":[]}}}}"#,
        column(&|(accession, _)| format!("\"{accession}\"")),
        column(&|_| "\"2025-05-14\"".to_owned()),
        column(&|(_, reported)| format!("\"{reported}\"")),
        column(&|_| "\"10-Q\"".to_owned()),
        column(&|_| "\"report.htm\"".to_owned()),
    )
}

/// A company facts document in the shape the recording holds, carrying two
/// taxonomies, four units and both period shapes, and publishing its key as
/// `key` — a JSON literal, so a case says which spelling it is.
///
/// `label`, `description`, `fy`, `fp` and `frame` are in it because the real
/// document carries them and none of them crosses: a case whose document held
/// only the fields that cross would pin nothing about the ones that do not.
/// The taxonomies are written out of order, so what comes back says which order
/// the retrieval put them in rather than which order they arrived in.
fn document_of(key: &str) -> String {
    format!(
        r#"{{
          "cik": {key},
          "entityName": "Maitong Sunshine Cultural Development Co., Ltd",
          "facts": {{
            "us-gaap": {{
              "Revenues": {{
                "label": "Revenues",
                "description": "Amount of revenue recognised.",
                "units": {{
                  "USD": [
                    {{"start":"2025-01-01","end":"2025-03-31","val":9007199254740993,
                      "accn":"0001213900-25-042964","fy":2025,"fp":"Q2","form":"10-Q",
                      "filed":"2025-05-14"}}
                  ],
                  "CNY": [
                    {{"start":"2025-01-01","end":"2025-03-31","val":64000000,
                      "accn":"0001213900-25-042964","fy":2025,"fp":"Q2","form":"10-Q",
                      "filed":"2025-05-14"}}
                  ]
                }}
              }},
              "EarningsPerShareBasic": {{
                "label": "Earnings Per Share, Basic",
                "description": "The amount of net income per share.",
                "units": {{
                  "USD/shares": [
                    {{"start":"2025-01-01","end":"2025-03-31","val":-0.0040,
                      "accn":"0001213900-25-042964","fy":2025,"fp":"Q2","form":"10-Q",
                      "filed":"2025-05-14"}}
                  ]
                }}
              }}
            }},
            "dei": {{
              "EntityCommonStockSharesOutstanding": {{
                "label": "Entity Common Stock, Shares Outstanding",
                "description": "Indicate number of shares outstanding.",
                "units": {{
                  "shares": [
                    {{"end":"2025-05-14","val":60500000,"accn":"0001213900-25-042964",
                      "fy":2025,"fp":"Q2","form":"10-Q","filed":"2025-05-14",
                      "frame":"CY2025Q1I"}}
                  ]
                }}
              }}
            }}
          }}
        }}"#
    )
}

/// The key as EDGAR's submissions document spells it.
const PADDED: &str = r#""0002003750""#;

fn fact(taxonomy: &str, tag: &str, unit: &str, period: Period, value: &str, form: &str) -> Fact {
    Fact {
        taxonomy: Box::from(taxonomy),
        tag: Box::from(tag),
        unit: Box::from(unit),
        period,
        value: Box::from(value),
        accession: Box::from(QUARTERLY),
        form: Box::from(form),
        filed: Box::from("2025-05-14"),
        report_period_end: Box::from(QUARTER_END),
    }
}

fn instant(at: &str) -> Period {
    Period::Instant { at: Box::from(at) }
}

fn quarter() -> Period {
    Period::Duration {
        start: Box::from("2025-01-01"),
        end: Box::from("2025-03-31"),
    }
}

/// Every fact the document publishes, in the order the keys it was nested under
/// put them in, each carrying the characters the document published.
///
/// Nothing is dropped for its taxonomy, its tag or its unit: `CNY` is a
/// currency no screen reads and it crosses beside the `USD` figure it sits next
/// to. Nothing is rewritten either — the two amounts here are exactly the ones a
/// number would spoil, and they arrive spelled as they were published.
///
/// The cover-page count is stated at the day its filing was received, and what
/// it carries as its report period end is still the quarter's: the date is the
/// history's for the filing, never the fact's own, and never `filed`.
#[test]
fn every_fact_the_document_publishes_crosses_as_it_was_published() {
    let mut edgar = asking(answering(&document_of(PADDED)));

    let filer = company_facts(&mut edgar, Cik::new(FILER)).expect("this is the document asked for");

    assert_eq!(&*filer.cik, "0002003750");
    assert_eq!(&*filer.retrieved_from, FACTS);
    assert_eq!(
        filer.facts,
        vec![
            fact(
                "dei",
                "EntityCommonStockSharesOutstanding",
                "shares",
                instant("2025-05-14"),
                "60500000",
                "10-Q",
            ),
            fact(
                "us-gaap",
                "EarningsPerShareBasic",
                "USD/shares",
                quarter(),
                "-0.0040",
                "10-Q",
            ),
            fact("us-gaap", "Revenues", "CNY", quarter(), "64000000", "10-Q"),
            fact(
                "us-gaap",
                "Revenues",
                "USD",
                quarter(),
                "9007199254740993",
                "10-Q",
            ),
        ]
    );
}

/// The facts, then the filer's history, and nothing else. The facts are still
/// the one document whatever the filer's history holds; the history comes
/// after them so it is no older than the facts it is joined to.
#[test]
fn the_requests_are_the_one_document_then_the_history_and_nothing_else() {
    let mut edgar = asking(answering(&document_of(PADDED)));

    company_facts(&mut edgar, Cik::new(FILER)).expect("this is the document asked for");

    assert_eq!(
        edgar.transport().asked,
        vec![FACTS.to_owned(), SUBMISSIONS.to_owned()]
    );
}

/// The spelling most filers' documents carry. It is read as the number it is,
/// and what crosses is that number at ten digits — the key the request was
/// built with and the one `registry/filers/` binds — and every fact beside it
/// crosses exactly as it does under the padded spelling.
#[test]
fn a_key_published_as_a_bare_number_crosses_at_ten_digits() {
    let padded = company_facts(
        &mut asking(answering(&document_of(PADDED))),
        Cik::new(FILER),
    )
    .expect("this is the document asked for");
    let bare = company_facts(
        &mut asking(answering(&document_of("2003750"))),
        Cik::new(FILER),
    )
    .expect("this is the document asked for, spelled the other way");

    assert_eq!(&*bare.cik, "0002003750");
    assert_eq!(bare, padded);
}

/// A whole company's worth of facts, every one of them well-formed and none of
/// them this filer's.
#[test]
fn facts_filed_under_another_key_are_not_read() {
    let mut edgar = asking(answering(&document_of(r#""0000320193""#)));

    match company_facts(&mut edgar, Cik::new(FILER)) {
        Err(Unretrieved::Unreadable { why, .. }) => {
            assert!(why.contains("0000320193"), "{why}");
        }
        other => panic!("a document about another filer was read: {other:?}"),
    }
}

/// The comparison is on the number, so the spelling a document about another
/// filer arrives in changes nothing: neither crosses, and the answer names the
/// key it was filed under and the key that was asked for. Nothing is joined to
/// it either, so the history is never asked for.
#[test]
fn facts_filed_under_another_key_are_not_read_in_either_spelling() {
    for key in [r#""0000320193""#, "320193"] {
        let mut edgar = asking(answering(&document_of(key)));

        match company_facts(&mut edgar, Cik::new(FILER)) {
            Err(Unretrieved::Unreadable { why, .. }) => {
                assert!(why.contains("0000320193"), "{key}: {why}");
                assert!(why.contains("0002003750"), "{key}: {why}");
            }
            other => panic!("{key}: a document about another filer was read: {other:?}"),
        }
        assert_eq!(edgar.transport().asked, vec![FACTS.to_owned()], "{key}");
    }
}

/// Neither of the two spellings, so not the document this endpoint publishes,
/// and nothing is taken from it — not even the reason to ask for a history.
#[test]
fn a_key_that_is_neither_a_string_nor_a_number_is_not_read() {
    for key in ["null", "true", "[2003750]", r#"{"cik":2003750}"#] {
        let mut edgar = asking(answering(&document_of(key)));

        match company_facts(&mut edgar, Cik::new(FILER)) {
            Err(Unretrieved::Unreadable { why, .. }) => {
                assert!(
                    why.contains("neither a string nor a number"),
                    "{key}: {why}"
                );
            }
            other => panic!("{key} was read as a filer's key: {other:?}"),
        }
        assert_eq!(edgar.transport().asked, vec![FACTS.to_owned()], "{key}");
    }
}

/// Every one of these is this filer's key if it is read loosely: its last ten
/// digits, its value with the zeros stripped, its value as a float. None of
/// them is a key of ten digits, so none is truncated or padded wider into one,
/// and nothing is taken from the document — not even the reason to ask for a
/// history.
#[test]
fn a_key_no_ten_digits_spell_is_neither_truncated_nor_padded_wider() {
    for key in [
        "10002003750",
        "20037500000000000000000",
        "2003750.0",
        "2.00375e6",
        "-2003750",
        r#""00002003750""#,
        r#""2003750.0""#,
        r#""+2003750""#,
        r#""""#,
    ] {
        let mut edgar = asking(answering(&document_of(key)));

        match company_facts(&mut edgar, Cik::new(FILER)) {
            Err(Unretrieved::Unreadable { why, .. }) => {
                assert!(
                    why.contains("no key of 10 digits spells it"),
                    "{key}: {why}"
                );
            }
            other => panic!("{key} was read as a filer's key: {other:?}"),
        }
        assert_eq!(edgar.transport().asked, vec![FACTS.to_owned()], "{key}");
    }
}

/// The two ways a fact's report period end is empty, beside the way it is not.
/// A filing the history names with a report date carries that date; one it
/// names with none carries none; and one it does not name at all carries none
/// either, rather than a date read off the fact.
#[test]
fn a_report_period_end_is_the_historys_date_for_the_filing_and_empty_where_it_has_none() {
    let dated = document_of(PADDED);
    let undated = dated.replacen(QUARTERLY, "0001213900-25-000001", 1);
    let unnamed = undated.replacen(QUARTERLY, "0001213900-25-099999", 1);

    let answers = Answers::default()
        .to(
            SUBMISSIONS,
            200,
            &history_of(&[(QUARTERLY, QUARTER_END), ("0001213900-25-000001", "")]),
        )
        .to(FACTS, 200, &unnamed);

    let filer = company_facts(&mut asking(answers), Cik::new(FILER))
        .expect("this is the document asked for");

    let carried: Vec<(&str, &str)> = filer
        .facts
        .iter()
        .map(|fact| (&*fact.accession, &*fact.report_period_end))
        .collect();

    // Rewritten in the order the document writes them, USD then CNY, and
    // leaving in unit order, CNY then USD.
    assert_eq!(
        carried,
        vec![
            (QUARTERLY, QUARTER_END),
            (QUARTERLY, QUARTER_END),
            ("0001213900-25-099999", ""),
            ("0001213900-25-000001", ""),
        ]
    );
}

/// One filing named twice with two report dates is the history saying two
/// things, and choosing one would be this stage deciding which period the
/// filing reports. Nothing crosses.
#[test]
fn a_history_naming_one_filing_with_two_report_dates_yields_nothing() {
    let answers = Answers::default()
        .to(
            SUBMISSIONS,
            200,
            &history_of(&[(QUARTERLY, QUARTER_END), (QUARTERLY, "2024-12-31")]),
        )
        .to(FACTS, 200, &document_of(PADDED));
    let mut edgar = asking(answers);

    match company_facts(&mut edgar, Cik::new(FILER)) {
        Err(Unretrieved::Unreadable { source, why }) => {
            assert_eq!(source.url(), SUBMISSIONS);
            assert!(why.contains(QUARTERLY), "{why}");
        }
        other => panic!("a filing with two report dates was joined: {other:?}"),
    }
}

/// No history, no facts: a report period end has nothing to be read out of, so
/// the retrieval stops where the history did rather than carrying every date
/// empty.
#[test]
fn facts_without_the_history_beside_them_are_not_retrieved() {
    let answers = Answers::default()
        .to(SUBMISSIONS, 404, "")
        .to(FACTS, 200, &document_of(PADDED));
    let mut edgar = asking(answers);

    assert!(matches!(
        company_facts(&mut edgar, Cik::new(FILER)),
        Err(Unretrieved::Refused { status: 404, .. })
    ));
    assert_eq!(
        edgar.transport().asked,
        vec![FACTS.to_owned(), SUBMISSIONS.to_owned()]
    );
}

#[test]
fn a_body_that_is_not_the_document_yields_nothing() {
    let mut edgar = asking(answering(r#"{"cik":"0002003750"}"#));

    assert!(matches!(
        company_facts(&mut edgar, Cik::new(FILER)),
        Err(Unretrieved::Unreadable { .. })
    ));
}

/// An amount published as a string, which would cross carrying its own quotes
/// and be a value normalize could neither read nor tell from an intact one.
#[test]
fn an_amount_the_document_does_not_publish_as_a_number_stops_the_read() {
    let quoted = document_of(PADDED).replace("\"val\":64000000", "\"val\":\"64000000\"");
    let mut edgar = asking(answering(&quoted));

    match company_facts(&mut edgar, Cik::new(FILER)) {
        Err(Unretrieved::Unreadable { why, .. }) => {
            assert!(why.contains("no number"), "{why}");
        }
        other => panic!("a quoted amount was taken as one: {other:?}"),
    }
}
