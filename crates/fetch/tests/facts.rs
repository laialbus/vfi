//! What crosses the boundary when a filer's facts are retrieved, and what the
//! stage answers when EDGAR answers something other than that filer's facts.
//!
//! The real document is pinned by the golden fixture, which is a recording of
//! one. What is here is what a recording cannot show: a document filed under
//! the wrong key, one that is not the document at all, and an amount published
//! as something other than a number. Those are written by hand rather than
//! fetched — the thing this project trusts least — so each case turns on one
//! thing being wrong and asserts the answer to that flaw, never that a made-up
//! document parses.
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
const FILER: u64 = 2003750;

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

/// A company facts document in the shape the recording holds, carrying two
/// taxonomies, four units and both period shapes.
///
/// `label`, `description`, `fy`, `fp` and `frame` are in it because the real
/// document carries them and none of them crosses: a case whose document held
/// only the fields that cross would pin nothing about the ones that do not.
/// The taxonomies are written out of order, so what comes back says which order
/// the retrieval put them in rather than which order they arrived in.
fn document_of(cik: &str) -> String {
    format!(
        r#"{{
          "cik": "{cik}",
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

fn fact(taxonomy: &str, tag: &str, unit: &str, period: Period, value: &str, form: &str) -> Fact {
    Fact {
        taxonomy: Box::from(taxonomy),
        tag: Box::from(tag),
        unit: Box::from(unit),
        period,
        value: Box::from(value),
        accession: Box::from("0001213900-25-042964"),
        form: Box::from(form),
        filed: Box::from("2025-05-14"),
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
#[test]
fn every_fact_the_document_publishes_crosses_as_it_was_published() {
    let mut edgar = asking(Answers::default().to(FACTS, 200, &document_of("0002003750")));

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

/// One request, and it is the filer's own document. The whole reason this
/// retrieval is affordable over a corpus is that a filer's history costs one
/// request whatever its length, so a second request here would be the funnel's
/// arithmetic undone.
#[test]
fn the_requests_are_the_one_document_and_nothing_else() {
    let mut edgar = asking(Answers::default().to(FACTS, 200, &document_of("0002003750")));

    company_facts(&mut edgar, Cik::new(FILER)).expect("this is the document asked for");

    assert_eq!(edgar.transport().asked, vec![FACTS.to_owned()]);
}

/// A whole company's worth of facts, every one of them well-formed and none of
/// them this filer's.
#[test]
fn facts_filed_under_another_key_are_not_read() {
    let mut edgar = asking(Answers::default().to(FACTS, 200, &document_of("0000320193")));

    match company_facts(&mut edgar, Cik::new(FILER)) {
        Err(Unretrieved::Unreadable { why, .. }) => {
            assert!(why.contains("0000320193"), "{why}");
        }
        other => panic!("a document about another filer was read: {other:?}"),
    }
}

#[test]
fn a_body_that_is_not_the_document_yields_nothing() {
    let mut edgar = asking(Answers::default().to(FACTS, 200, r#"{"cik":"0002003750"}"#));

    assert!(matches!(
        company_facts(&mut edgar, Cik::new(FILER)),
        Err(Unretrieved::Unreadable { .. })
    ));
}

/// An amount published as a string, which would cross carrying its own quotes
/// and be a value normalize could neither read nor tell from an intact one.
#[test]
fn an_amount_the_document_does_not_publish_as_a_number_stops_the_read() {
    let quoted = document_of("0002003750").replace("\"val\":64000000", "\"val\":\"64000000\"");
    let mut edgar = asking(Answers::default().to(FACTS, 200, &quoted));

    match company_facts(&mut edgar, Cik::new(FILER)) {
        Err(Unretrieved::Unreadable { why, .. }) => {
            assert!(why.contains("no number"), "{why}");
        }
        other => panic!("a quoted amount was taken as one: {other:?}"),
    }
}
