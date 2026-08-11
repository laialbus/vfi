//! What the stage answers when EDGAR answers something other than an ordinary
//! filing history.
//!
//! The ordinary case is pinned by the golden fixtures, which are recordings.
//! These are the cases a recording cannot be taken of, and the documents below
//! are written here rather than fetched — which is worth saying plainly,
//! because a hand-written response is the thing this project trusts least.
//! Two things keep that honest. The shape they are written in is the shape the
//! fixtures pin, so a document here that drifted from EDGAR's would have to
//! drift past those as well. And each case turns on one thing being wrong,
//! never on the rest being right: what is asserted is the answer to the flaw,
//! not that a made-up document parses.
//!
//! The first case is the one to read the caveat on hardest. A filer EDGAR names
//! and publishes no filing for is an outcome the task calls for and EDGAR does
//! not appear to publish — a CIK reaches the submissions endpoint by filing, so
//! every document it answers with carries at least one — and no recording of it
//! was found to take. What is pinned is this stage's answer to a history with
//! nothing in it, not EDGAR's way of saying so.
//!
//! Nothing here reaches the network. The transport answers from a table the
//! case wrote and has no wire under it.

use std::collections::BTreeMap;
use std::io;

use vfi_fetch::{
    Cleared, Declaration, Egress, Pace, Response, Retrieved, Ticker, Transport, Unretrieved,
    filing_history,
};

const TICKER_MAP: &str = "https://www.sec.gov/files/company_tickers.json";
const SUBMISSIONS: &str = "https://data.sec.gov/submissions/CIK0002003750.json";

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

/// EDGAR's ticker map with one filer in it, in the shape the fixtures record.
fn map_naming(cik: u64, ticker: &str) -> String {
    format!(
        r#"{{"0":{{"cik_str":{cik},"ticker":"{ticker}","title":"Maitong Sunshine Cultural Development Co., Ltd"}}}}"#
    )
}

/// A submissions document holding the filings given as columns. `size` is a
/// column the stage does not read, left in because a document carries several
/// of those and passing over them is part of what these cases stand on.
fn submissions_of(
    cik: &str,
    accessions: &str,
    forms: &str,
    filed: &str,
    periods: &str,
    documents: &str,
) -> String {
    format!(
        r#"{{"cik":"{cik}","entityType":"operating","name":"Maitong Sunshine Cultural Development Co., Ltd",
            "filings":{{"recent":{{"accessionNumber":[{accessions}],"filingDate":[{filed}],
            "reportDate":[{periods}],"form":[{forms}],"primaryDocument":[{documents}],
            "size":[]}},"files":[]}}}}"#
    )
}

fn ask(answers: Answers) -> Result<Retrieved, Unretrieved> {
    filing_history(&mut asking(answers), &Ticker::new("MGSD"))
}

/// The outcome the caveat above is about. A filer with nothing filed is not an
/// empty success and not a failure: the funnel that records why each filer was
/// kept or dropped has to tell it from a ticker EDGAR never heard of.
#[test]
fn a_filer_with_nothing_filed_is_its_own_outcome() {
    let empty = r#"{"cik":"0002003750","filings":{"recent":{"accessionNumber":[],
        "filingDate":[],"reportDate":[],"form":[],"primaryDocument":[]},"files":[]}}"#;

    let retrieved = ask(Answers::default()
        .to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))
        .to(SUBMISSIONS, 200, empty))
    .expect("EDGAR answered both requests");

    match retrieved {
        Retrieved::NoFilings { company, source } => {
            assert_eq!(company.cik().to_string(), "0002003750");
            assert_eq!(company.source().url(), TICKER_MAP);
            assert_eq!(source.url(), SUBMISSIONS);
        }
        other => panic!("a filer with nothing filed is the outcome, and this was {other:?}"),
    }
}

/// A ticker the map does not name says so, and says where it looked. What it
/// does not do is answer with the filer whose ticker is one character away.
#[test]
fn a_ticker_the_map_does_not_name_is_not_resolved_to_a_near_one() {
    let retrieved = filing_history(
        &mut asking(Answers::default().to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))),
        &Ticker::new("MGSE"),
    )
    .expect("EDGAR answered the one request this makes");

    match retrieved {
        Retrieved::UnknownTicker { ticker, source } => {
            assert_eq!(ticker.as_str(), "MGSE");
            assert_eq!(source.url(), TICKER_MAP);
        }
        other => panic!("an unknown ticker is the outcome, and this was {other:?}"),
    }
}

/// One ticker against two filers is EDGAR's map saying two things, and either
/// answer would be this crate deciding which company was meant. So there is no
/// answer, and the refusal names the ticker that is doubled.
#[test]
fn one_ticker_naming_two_filers_is_resolved_to_neither() {
    let doubled = r#"{"0":{"cik_str":2003750,"ticker":"MGSD","title":"One"},
                      "1":{"cik_str":320193,"ticker":"MGSD","title":"Another"}}"#;

    match ask(Answers::default().to(TICKER_MAP, 200, doubled)) {
        Err(Unretrieved::Unreadable { source, why }) => {
            assert_eq!(source.url(), TICKER_MAP);
            assert!(why.contains("MGSD"), "the refusal has to name it: {why}");
        }
        other => panic!("a doubled ticker is not resolved, and this was {other:?}"),
    }
}

/// A submissions document filed under a key nobody asked for is a whole
/// company's worth of well-formed wrong facts, so none of it is taken.
#[test]
fn a_history_filed_under_another_key_is_not_read() {
    let elsewhere = submissions_of(
        "0000320193",
        r#""0000000000-26-000001""#,
        r#""10-K""#,
        r#""2026-01-01""#,
        r#""2025-12-31""#,
        r#""filing.htm""#,
    );

    match ask(Answers::default()
        .to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))
        .to(SUBMISSIONS, 200, &elsewhere))
    {
        Err(Unretrieved::Unreadable { source, why }) => {
            assert_eq!(source.url(), SUBMISSIONS);
            assert!(
                why.contains("0000320193") && why.contains("0002003750"),
                "the refusal has to say which key it got and which it asked for: {why}"
            );
        }
        other => panic!("a document about another filer is not read, and this was {other:?}"),
    }
}

/// The columns are parallel arrays and the document does not say they are the
/// same length. Reading a row out of columns that disagree would pair one
/// filing's number with another's date — a filing that never happened, and one
/// nothing downstream could tell from a real one.
#[test]
fn columns_that_disagree_on_how_many_filings_there_are_stop_the_read() {
    let ragged = submissions_of(
        "0002003750",
        r#""0000000000-26-000001","0000000000-26-000002""#,
        r#""10-K""#,
        r#""2026-01-01","2026-02-01""#,
        r#""2025-12-31","2026-01-31""#,
        r#""one.htm","two.htm""#,
    );

    match ask(Answers::default()
        .to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))
        .to(SUBMISSIONS, 200, &ragged))
    {
        Err(Unretrieved::Unreadable { source, why }) => {
            assert_eq!(source.url(), SUBMISSIONS);
            assert!(why.contains("form"), "the refusal has to say which: {why}");
        }
        other => panic!("ragged columns are not read, and this was {other:?}"),
    }
}

/// A status is an answer. Which of them is a filer that is not there, and which
/// is worth asking again, is the caller's judgement, so the status is handed
/// back as it came rather than folded into an outcome here.
#[test]
fn a_status_that_is_not_a_document_is_handed_back_as_it_came() {
    match ask(Answers::default()
        .to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))
        .to(SUBMISSIONS, 404, "not found"))
    {
        Err(Unretrieved::Refused { source, status }) => {
            assert_eq!(source.url(), SUBMISSIONS);
            assert_eq!(status, 404);
        }
        other => panic!("a refusal carries its status, and this was {other:?}"),
    }
}

/// A body that is not the document the endpoint publishes yields no records at
/// all. Half a history read out of a shape this did not recognise would be
/// worse than none: it would be a number nobody could trace.
#[test]
fn a_body_that_is_not_the_document_yields_nothing() {
    match ask(Answers::default().to(TICKER_MAP, 200, "<html>maintenance</html>")) {
        Err(Unretrieved::Unreadable { source, .. }) => assert_eq!(source.url(), TICKER_MAP),
        other => panic!("an unreadable body yields nothing, and this was {other:?}"),
    }
}

/// Every request the stage makes goes out through the chokepoint, so every one
/// of them is a URL the host list has seen. This is the behaviour half of that;
/// that no call site opens a connection around the chokepoint is the `egress`
/// gate over the workspace, and no test here can say anything about it.
#[test]
fn the_requests_are_the_two_documents_and_nothing_else() {
    let mut egress = asking(
        Answers::default()
            .to(TICKER_MAP, 200, &map_naming(2003750, "MGSD"))
            .to(
                SUBMISSIONS,
                200,
                &submissions_of("0002003750", "", "", "", "", ""),
            ),
    );

    filing_history(&mut egress, &Ticker::new("mgsd")).expect("EDGAR answered both requests");

    assert_eq!(egress.transport().asked, vec![TICKER_MAP, SUBMISSIONS]);
}
