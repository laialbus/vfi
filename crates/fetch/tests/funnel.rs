//! What the funnel decides, and — the half that matters more — what it does not
//! ask for.
//!
//! The golden fixtures pin a pass against documents recorded from EDGAR. These
//! are the cases a recording cannot be taken of, and the documents below are
//! written here rather than fetched, which is worth saying plainly because a
//! hand-written response is the thing this project trusts least. Two things keep
//! it honest: the shape they are written in is the shape those fixtures pin, so
//! a document here that drifted from EDGAR's would have to drift past them as
//! well; and each case turns on one thing, never on the rest being right.
//!
//! The case to read first is the negative one. A funnel that fetched every
//! filer's history and threw the rejects away afterwards would produce the same
//! verdicts as this one and would be the opposite of a funnel, so what is
//! asserted is the list of requests that left.
//!
//! Nothing here reaches the network. The transport answers from a table the case
//! wrote and has no wire under it.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::slice;

use vfi_fetch::funnel::{self, Calendar, Sweep, Unswept, admitted, gate, seed};
use vfi_fetch::ledger::{
    Filer, FilerKey, FilerLedger, InMemory, Journal, Pass, Reason, Record, Step, Unkept, Verdict,
    When,
};
use vfi_fetch::{Cleared, Declaration, Egress, History, Pace, Response, Transport};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TICKER_MAP: &str = "https://www.sec.gov/files/company_tickers.json";

fn submissions_url(cik: u64) -> String {
    format!("https://data.sec.gov/submissions/CIK{cik:010}.json")
}

/// A transport answering out of a table, keeping the URLs it was asked for in
/// the order they arrived — which is how a case says what was requested, and
/// what was not.
///
/// A URL given more than one answer answers with them in turn, and with the last
/// of them from then on. Two steps of one pass ask for a filer's document, and
/// that is two moments: a case that wants the second request to meet something
/// the first did not is the only way some of what a pass must handle can be
/// reached at all.
#[derive(Default)]
struct Answers {
    documents: BTreeMap<String, Vec<(u16, String)>>,
    asked: Vec<String>,
}

impl Answers {
    fn to(mut self, url: &str, status: u16, body: &str) -> Self {
        self.documents
            .entry(url.to_owned())
            .or_default()
            .push((status, body.to_owned()));
        self
    }
}

impl Transport for Answers {
    fn send(&mut self, request: Cleared<'_>) -> io::Result<Response> {
        let url = request.url();
        let before = self.asked.iter().filter(|asked| *asked == url).count();
        self.asked.push(url.to_owned());

        let answers = self
            .documents
            .get(url)
            .unwrap_or_else(|| panic!("{url}: this case says nothing about that request"));
        let (status, body) = &answers[before.min(answers.len() - 1)];

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

/// A calendar standing still, for the reason [`vfi_fetch::Clock`] is a
/// parameter: a record that stamped itself could only be checked by waiting for
/// the time it stamped.
struct Stopped;

impl Calendar for Stopped {
    fn now(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_767_225_600)
    }
}

fn sweep() -> Sweep {
    Sweep::new(Pass::new(1), Stopped)
}

/// One row of EDGAR's ticker map.
fn entry(row: u64, cik: u64, ticker: &str, title: &str) -> String {
    format!(r#""{row}":{{"cik_str":{cik},"ticker":"{ticker}","title":"{title}"}}"#)
}

fn map_of(entries: &[String]) -> String {
    format!("{{{}}}", entries.join(","))
}

/// One page of filings given as `(form, filed)`, in the shape the fixtures
/// record: the page a submissions document carries inline, and the page of older
/// filings that document names, are the same shape.
fn page_of(filings: &[(&str, &str)]) -> String {
    let column = |values: Vec<String>| values.join(",");
    let quoted = |value: &str| format!("\"{value}\"");

    format!(
        r#"{{"accessionNumber":[{accessions}],"filingDate":[{filed}],"reportDate":[{periods}],
            "form":[{forms}],"primaryDocument":[{documents}]}}"#,
        accessions = column(
            (0..filings.len())
                .map(|row| quoted(&format!("0000000000-26-{row:06}")))
                .collect()
        ),
        filed = column(filings.iter().map(|(_, filed)| quoted(filed)).collect()),
        periods = column(filings.iter().map(|(_, filed)| quoted(filed)).collect()),
        forms = column(filings.iter().map(|(form, _)| quoted(form)).collect()),
        documents = column(
            (0..filings.len())
                .map(|row| quoted(&format!("filing-{row}.htm")))
                .collect()
        ),
    )
}

/// A submissions document naming `filings`. `overflow` is the pages of older
/// filings EDGAR names, which the gate must never follow and the step after it
/// must.
fn submissions_of(cik: u64, filings: &[(&str, &str)], overflow: &[&str]) -> String {
    format!(
        r#"{{"cik":"{cik:010}","entityType":"operating","filings":{{"recent":{recent},"files":[{files}]}}}}"#,
        recent = page_of(filings),
        files = overflow
            .iter()
            .map(|name| format!(r#"{{"name":"{name}"}}"#))
            .collect::<Vec<_>>()
            .join(","),
    )
}

/// One pass, and what it left behind: what it was recorded in, who it looked at,
/// who came out of the last step it ran, what it retrieved, and — the half that
/// matters more — what it asked for.
struct Passed {
    ledger: InMemory,
    considered: Vec<FilerKey>,
    admitted: Vec<Record>,
    retrieved: Vec<History>,
    asked: Vec<String>,
}

/// A pass over the first two steps, which is where a case about the gate ends.
fn run(answers: Answers) -> Passed {
    let mut edgar = asking(answers);
    let mut ledger = InMemory::new();
    let sweep = sweep();

    let considered = seed(&mut edgar, &mut ledger, &sweep).expect("the map answered");
    let admitted = gate(&mut edgar, &mut ledger, &sweep, &considered).expect("nothing refused");

    Passed {
        asked: edgar.transport().asked.clone(),
        ledger,
        considered,
        admitted,
        retrieved: Vec::new(),
    }
}

/// The whole funnel, in one pass: seed set, then metadata gate, then a history
/// for the filers that survived both.
fn sweep_over(answers: Answers) -> Passed {
    let mut edgar = asking(answers);
    let mut ledger = InMemory::new();
    let sweep = sweep();
    let mut retrieved = Vec::new();

    let considered = funnel::run(&mut edgar, &mut ledger, &sweep, |history| {
        retrieved.push(history);
    })
    .expect("nothing refused");
    let admitted = admitted(&ledger, Step::History, &considered).expect("a ledger in memory reads");

    Passed {
        asked: edgar.transport().asked.clone(),
        ledger,
        considered,
        admitted,
        retrieved,
    }
}

impl Passed {
    /// The key the pass filed a filer under. Nothing outside this crate can
    /// mint a CIK, which is the right way round — a key exists because EDGAR
    /// published it — so a case asks the pass for the ones it produced.
    fn keyed(&self, cik: u64) -> FilerKey {
        let published = format!("{cik:010}");
        self.considered
            .iter()
            .find(|key| matches!(key, FilerKey::Cik(keyed) if keyed.to_string() == published))
            .unwrap_or_else(|| panic!("the pass considered no filer keyed {published}"))
            .clone()
    }

    fn verdict(&self, key: &FilerKey, step: Step) -> Verdict {
        self.judged(key, step)
            .unwrap_or_else(|| panic!("{key:?} has no verdict at {step:?}"))
    }

    /// What the pass decided about a filer at one step, where it decided
    /// anything. A step that never reached a filer is the answer a case about
    /// what the funnel does not do is asking for.
    fn judged(&self, key: &FilerKey, step: Step) -> Option<Verdict> {
        self.ledger
            .verdicts(key)
            .expect("a ledger in memory refuses nothing")
            .into_iter()
            .rev()
            .find(|record| record.step() == step)
            .map(|record| record.verdict().clone())
    }

    /// The filers whose histories the pass handed on, keyed as EDGAR keys them.
    fn retrieved(&self) -> Vec<FilerKey> {
        self.retrieved
            .iter()
            .map(|history| FilerKey::Cik(history.company().cik()))
            .collect()
    }
}

/// Two filers, one of which the gate rejects. The verdicts are the ordinary
/// half; the requests are the point. A rejected filer costs the one metadata
/// document its verdict was read out of, and after that nothing — not the page
/// of older filings its own document names, and nothing the step after this one
/// would send.
#[test]
fn a_filer_the_gate_rejects_costs_nothing_after_its_verdict() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[
                entry(0, 320193, "AAPL", "Apple Inc."),
                entry(1, 1577552, "BABA", "Alibaba Group Holding Ltd"),
            ]),
        )
        .to(
            &submissions_url(320193),
            200,
            &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
        )
        .to(
            &submissions_url(1577552),
            200,
            &submissions_of(
                1577552,
                &[("20-F", "2025-06-20")],
                &["CIK0001577552-submissions-001.json"],
            ),
        ));

    assert_eq!(
        passed.asked,
        vec![
            TICKER_MAP.to_owned(),
            submissions_url(320193),
            submissions_url(1577552),
        ]
    );
    assert!(matches!(
        passed.verdict(&passed.keyed(1577552), Step::Metadata),
        Verdict::Rejected(_)
    ));
    assert_eq!(
        passed
            .admitted
            .iter()
            .map(|record| record.filer().key().clone())
            .collect::<Vec<_>>(),
        [passed.keyed(320193)]
    );
}

/// Every filer the gate reaches leaves a verdict, and the rejected one is the
/// reason the ledger exists: it produces no filings, no metrics and no row
/// anybody can ask about, so this record is the only trace it was considered.
/// The reason names the rule and carries what it was applied to.
#[test]
fn every_filer_the_gate_evaluates_is_on_record_with_what_it_was_judged_on() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 1577552, "BABA", "Alibaba Group Holding Ltd")]),
        )
        .to(
            &submissions_url(1577552),
            200,
            &submissions_of(
                1577552,
                &[("6-K", "2025-08-01"), ("20-F", "2025-06-20")],
                &[],
            ),
        ));

    let Verdict::Rejected(reason) = passed.verdict(&passed.keyed(1577552), Step::Metadata) else {
        panic!("a filer whose annual report is a 20-F is rejected");
    };

    assert!(
        reason.name().contains("foreign private issuer"),
        "the rule has to name itself: {}",
        reason.name()
    );
    assert_eq!(
        reason
            .judged_on()
            .iter()
            .map(|judged| (judged.what(), judged.value()))
            .collect::<Vec<_>>(),
        [("annual report", "20-F"), ("filed", "2025-06-20")]
    );
}

/// The scope note is about what a filer is now. A company that filed 20-F until
/// it stopped being a foreign private issuer files 10-K today and is not one, so
/// the newest annual report answers rather than the presence of one anywhere.
#[test]
fn the_newest_annual_report_is_what_says_which_filer_this_is() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 1090727, "UPS", "United Parcel Service Inc")]),
        )
        .to(
            &submissions_url(1090727),
            200,
            &submissions_of(
                1090727,
                &[("10-K", "2026-02-10"), ("20-F", "2019-03-01")],
                &[],
            ),
        ));

    assert!(matches!(
        passed.verdict(&passed.keyed(1090727), Step::Metadata),
        Verdict::Admitted(_)
    ));
}

/// The distinction the three verdicts exist for. A metadata document that did
/// not arrive says nothing about the filer, and recording that as a rejection is
/// how a bad minute becomes a permanent exclusion — for a reason that stopped
/// being true the moment the source answered again.
#[test]
fn a_filer_whose_metadata_did_not_arrive_is_not_judged_rather_than_rejected() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
        )
        .to(&submissions_url(320193), 503, "unavailable"));

    let Verdict::Unjudged(reason) = passed.verdict(&passed.keyed(320193), Step::Metadata) else {
        panic!("a filer nobody could ask about is not judged");
    };
    assert!(
        reason
            .judged_on()
            .iter()
            .any(|judged| judged.value().contains("503")),
        "the reason has to carry what came back"
    );
    assert!(passed.admitted.is_empty());
}

/// The other half of not judged: the document arrived and names no annual
/// report, so there is nothing in it to answer the question with. Nothing is
/// decided, and the filer is not admitted on the strength of what is missing.
#[test]
fn a_filer_whose_metadata_names_no_annual_report_is_not_judged() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(
                0,
                2141099,
                "RGKUY",
                "Rigaku Holdings Corporation/ADR",
            )]),
        )
        .to(
            &submissions_url(2141099),
            200,
            &submissions_of(2141099, &[("F-6EF", "2025-10-01")], &[]),
        ));

    assert!(matches!(
        passed.verdict(&passed.keyed(2141099), Step::Metadata),
        Verdict::Unjudged(_)
    ));
    assert!(passed.admitted.is_empty());
}

/// An entry naming no filer is not left out of the pass. It was evaluated,
/// nothing resolved, and this record is the only trace it was ever there — so it
/// is filed under whatever the entry did carry, and the rest of the map is read
/// as usual around it.
#[test]
fn a_seed_entry_that_resolves_to_no_identifier_is_a_verdict_of_its_own() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[
                entry(0, 0, "ZZZZ", "A row naming no filer"),
                entry(1, 320193, "AAPL", "Apple Inc."),
            ]),
        )
        .to(
            &submissions_url(320193),
            200,
            &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
        ));

    let unresolved = FilerKey::Unresolved(Box::from("ZZZZ"));
    assert_eq!(
        passed.considered,
        [unresolved.clone(), passed.keyed(320193)]
    );
    assert!(matches!(
        passed.verdict(&unresolved, Step::Seed),
        Verdict::Rejected(_)
    ));

    // Nothing is asked about it: there is no identifier to ask with, and the
    // filer beside it is read exactly as it would have been alone.
    assert_eq!(
        passed.asked,
        vec![TICKER_MAP.to_owned(), submissions_url(320193)]
    );
}

/// One filer under several tickers is one filer. EDGAR lists a company once per
/// symbol it trades under, and a pass that took each row for a filer of its own
/// would ask about it as many times and record the same verdict as many times.
#[test]
fn a_filer_listed_under_several_tickers_is_judged_once() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[
                entry(0, 1577552, "BABA", "Alibaba Group Holding Ltd"),
                entry(1, 1577552, "BABAF", "Alibaba Group Holding Ltd"),
            ]),
        )
        .to(
            &submissions_url(1577552),
            200,
            &submissions_of(1577552, &[("20-F", "2025-06-20")], &[]),
        ));

    assert_eq!(passed.considered, [passed.keyed(1577552)]);
    assert_eq!(
        passed.asked,
        vec![TICKER_MAP.to_owned(), submissions_url(1577552)]
    );
    assert_eq!(
        passed
            .ledger
            .verdicts(&passed.keyed(1577552))
            .expect("a ledger in memory refuses nothing")
            .len(),
        2,
        "one verdict per step, and no more"
    );
}

/// The set the gate runs over is the ledger's answer, not a list handed along
/// beside it. A filer the seed step rejected is not asked about however it got
/// into the keys the gate is given.
#[test]
fn the_gate_judges_who_the_ledger_says_the_seed_step_admitted() {
    let answers = Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
        )
        .to(
            &submissions_url(320193),
            200,
            &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
        );

    let mut edgar = asking(answers);
    let mut ledger = InMemory::new();
    let sweep = sweep();

    let considered = seed(&mut edgar, &mut ledger, &sweep).expect("the map answered");
    let seeded = ledger
        .verdicts(&considered[0])
        .expect("a ledger in memory refuses nothing")
        .pop()
        .expect("the seed step recorded something about it");

    // The same filer, judged again and dropped. Nothing about the keys the gate
    // is handed has changed, so what it does next is the ledger's answer alone.
    ledger
        .record(Record::new(
            seeded.filer().clone(),
            Step::Seed,
            Verdict::Rejected(Reason::new("planted by this case", "cik", 320193)),
            seeded.source().clone(),
            seeded.when(),
        ))
        .expect("a ledger in memory refuses nothing");

    let admitted = gate(&mut edgar, &mut ledger, &sweep, &considered).expect("nothing refused");

    assert!(admitted.is_empty());
    assert_eq!(edgar.transport().asked, vec![TICKER_MAP.to_owned()]);
}

/// A ledger outlives the pass that wrote it, so the gate meets records it did
/// not make. One it cannot ask EDGAR about is one it cannot judge, and that is a
/// verdict it records rather than a filer it passes over in silence.
#[test]
fn a_filer_the_gate_cannot_ask_about_is_recorded_as_not_judged() {
    let answers = Answers::default().to(
        TICKER_MAP,
        200,
        &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
    );

    let mut edgar = asking(answers);
    let mut ledger = InMemory::new();
    let sweep = sweep();

    let considered = seed(&mut edgar, &mut ledger, &sweep).expect("the map answered");
    let map = ledger
        .verdicts(&considered[0])
        .expect("a ledger in memory refuses nothing")[0]
        .source()
        .clone();

    let unresolved = Filer::unresolved("ZZZZ");
    let key = unresolved.key().clone();
    ledger
        .record(Record::new(
            unresolved,
            Step::Seed,
            Verdict::Admitted(Reason::new("planted by this case", "cik", 0)),
            map,
            When::new(
                UNIX_EPOCH + Duration::from_secs(1_767_225_600),
                Pass::new(1),
                vfi_fetch::ledger::Ruleset::version(1),
            ),
        ))
        .expect("a ledger in memory refuses nothing");

    let admitted =
        gate(&mut edgar, &mut ledger, &sweep, slice::from_ref(&key)).expect("nothing refused");

    assert!(admitted.is_empty());
    assert!(matches!(
        ledger
            .verdicts(&key)
            .expect("a ledger in memory refuses nothing")
            .into_iter()
            .rev()
            .find(|record| record.step() == Step::Metadata)
            .expect("the gate recorded something about it")
            .verdict(),
        Verdict::Unjudged(_)
    ));
}

/// A verdict that did not land takes its filer out of what comes next. The step
/// after this one is derived from the ledger, so a pass that carried on past a
/// record it could not keep would be handing that step a filer nothing was
/// written down about.
#[test]
fn a_pass_that_cannot_record_a_verdict_stops_rather_than_going_on() {
    /// A ledger that keeps nothing and says so.
    struct Refuses;

    impl FilerLedger for Refuses {
        fn record(&mut self, _: Record) -> Result<(), vfi_fetch::ledger::Unkept> {
            Err(vfi_fetch::ledger::Unkept::Unwritten {
                why: io::Error::other("the journal is not writable"),
            })
        }

        fn verdicts(&self, _: &FilerKey) -> Result<Vec<Record>, vfi_fetch::ledger::Unkept> {
            Ok(Vec::new())
        }
    }

    let mut edgar = asking(Answers::default().to(
        TICKER_MAP,
        200,
        &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
    ));

    match seed(&mut edgar, &mut Refuses, &sweep()) {
        Err(Unswept::Unkept { .. }) => {}
        other => panic!("a verdict that did not land stops the pass, and this was {other:?}"),
    }
    assert_eq!(edgar.transport().asked, vec![TICKER_MAP.to_owned()]);
}

/// The admitted set is read back rather than collected as the pass goes, so a
/// later verdict about a filer is what stands. A filer admitted in August and
/// rejected in December is out, and both judgements are still on record.
#[test]
fn the_admitted_set_is_the_last_verdict_on_record_for_the_step() {
    let passed = run(Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
        )
        .to(
            &submissions_url(320193),
            200,
            &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
        ));

    let considered = passed.considered.clone();
    let mut ledger = passed.ledger;
    let admitting = ledger
        .verdicts(&considered[0])
        .expect("a ledger in memory refuses nothing")
        .pop()
        .expect("the gate recorded something about it");

    assert_eq!(
        admitted(&ledger, Step::Metadata, &considered)
            .expect("a ledger in memory refuses nothing")
            .len(),
        1
    );

    ledger
        .record(Record::new(
            admitting.filer().clone(),
            Step::Metadata,
            Verdict::Rejected(Reason::new("planted by this case", "annual report", "20-F")),
            admitting.source().clone(),
            admitting.when(),
        ))
        .expect("a ledger in memory refuses nothing");

    assert!(
        admitted(&ledger, Step::Metadata, &considered)
            .expect("a ledger in memory refuses nothing")
            .is_empty()
    );
}

/// The case that carries the weight, and the one a funnel that retrieved every
/// filer's history and threw the rejects away afterwards would fail while
/// passing every other line in this file: what is asserted is the requests that
/// left. The rejected filer costs the one metadata document its verdict was read
/// out of — not the page of older filings that document names, and no history.
#[test]
fn a_filer_the_gate_rejected_never_has_its_history_retrieved() {
    let passed = sweep_over(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[
                    entry(0, 320193, "AAPL", "Apple Inc."),
                    entry(1, 1577552, "BABA", "Alibaba Group Holding Ltd"),
                ]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            )
            .to(
                &submissions_url(1577552),
                200,
                &submissions_of(
                    1577552,
                    &[("20-F", "2025-06-20")],
                    &["CIK0001577552-submissions-001.json"],
                ),
            ),
    );

    assert_eq!(
        passed.asked,
        vec![
            TICKER_MAP.to_owned(),
            submissions_url(320193),
            submissions_url(1577552),
            submissions_url(320193),
        ]
    );
    assert_eq!(passed.retrieved(), [passed.keyed(320193)]);
    assert_eq!(
        passed.judged(&passed.keyed(1577552), Step::History),
        None,
        "a step that never reached a filer has decided nothing about it"
    );
    assert!(matches!(
        passed.verdict(&passed.keyed(320193), Step::History),
        Verdict::Admitted(_)
    ));
}

/// Every filer this step reaches leaves a verdict, and the reason carries what
/// it was judged on: how much history arrived, out of which request.
#[test]
fn a_filer_whose_history_arrived_is_on_record_with_what_arrived() {
    let passed = sweep_over(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(
                    320193,
                    &[("10-K", "2025-11-01"), ("8-K", "2025-08-04")],
                    &[],
                ),
            ),
    );

    let Verdict::Admitted(reason) = passed.verdict(&passed.keyed(320193), Step::History) else {
        panic!("a filer whose history arrived goes on");
    };

    assert_eq!(
        reason
            .judged_on()
            .iter()
            .map(|judged| (judged.what(), judged.value()))
            .collect::<Vec<_>>(),
        [("filings", "2")]
    );
    assert_eq!(
        passed
            .admitted
            .iter()
            .map(|record| record.source().to_string())
            .collect::<Vec<_>>(),
        [submissions_url(320193)]
    );
}

/// A history longer than one page is retrieved whole, and the page is asked for
/// here and nowhere earlier. The order says both: the gate stopped at the
/// document, and the step after it followed what that document names.
#[test]
fn a_history_longer_than_its_first_page_is_followed_by_this_step_alone() {
    let page = "CIK0000320193-submissions-001.json";
    let passed = sweep_over(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[page]),
            )
            .to(
                &format!("https://data.sec.gov/submissions/{page}"),
                200,
                &page_of(&[("10-K", "2015-10-28"), ("10-K", "2014-10-27")]),
            ),
    );

    assert_eq!(
        passed.asked,
        vec![
            TICKER_MAP.to_owned(),
            submissions_url(320193),
            submissions_url(320193),
            format!("https://data.sec.gov/submissions/{page}"),
        ]
    );
    assert_eq!(passed.retrieved[0].filings().len(), 3);
}

/// The distinction the three verdicts exist for, at the last step. A history
/// that did not arrive says nothing about the filer, and a filer recorded as
/// rejected for it would be out of the corpus for a reason that stopped being
/// true the moment the source answered again.
///
/// The two requests for one document are what makes the case: the gate's
/// arrived, and the one the step after it sent did not.
#[test]
fn a_filer_whose_history_did_not_arrive_is_not_judged_rather_than_rejected() {
    let passed = sweep_over(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            )
            .to(&submissions_url(320193), 503, "unavailable"),
    );

    let Verdict::Unjudged(reason) = passed.verdict(&passed.keyed(320193), Step::History) else {
        panic!("a filer whose history nobody could retrieve is not judged");
    };
    assert!(
        reason
            .judged_on()
            .iter()
            .any(|judged| judged.value().contains("503")),
        "the reason has to carry what came back"
    );
    assert!(passed.admitted.is_empty());
    assert!(passed.retrieved().is_empty());
}

/// The other half of it: EDGAR names the filer and publishes no filing under it.
/// That is an answer and not a failure — there is nothing for a later stage to
/// read, so the filer goes no further — and it is a different record from the
/// one above, which is the whole reason the two are separate verdicts.
#[test]
fn a_filer_edgar_publishes_no_filing_for_is_rejected_rather_than_left_unjudged() {
    let passed = sweep_over(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[], &[]),
            ),
    );

    assert!(matches!(
        passed.verdict(&passed.keyed(320193), Step::History),
        Verdict::Rejected(_)
    ));
    assert!(passed.admitted.is_empty());
    assert!(passed.retrieved().is_empty());
}

/// The set this step runs over is the ledger's answer, not a list handed along
/// beside it. A filer the gate rejected is not asked about however it got into
/// the keys this step is given.
#[test]
fn the_history_step_asks_about_who_the_ledger_says_the_gate_admitted() {
    let answers = Answers::default()
        .to(
            TICKER_MAP,
            200,
            &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
        )
        .to(
            &submissions_url(320193),
            200,
            &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
        );

    let mut edgar = asking(answers);
    let mut ledger = InMemory::new();
    let sweep = sweep();

    let considered = seed(&mut edgar, &mut ledger, &sweep).expect("the map answered");
    let judged = gate(&mut edgar, &mut ledger, &sweep, &considered).expect("nothing refused");
    let admitting = judged[0].clone();

    // The same filer, judged again and dropped. Nothing about the keys this step
    // is handed has changed, so what it does next is the ledger's answer alone.
    ledger
        .record(Record::new(
            admitting.filer().clone(),
            Step::Metadata,
            Verdict::Rejected(Reason::new("planted by this case", "annual report", "20-F")),
            admitting.source().clone(),
            admitting.when(),
        ))
        .expect("a ledger in memory refuses nothing");

    let mut retrieved = Vec::new();
    let admitted = funnel::history(
        &mut edgar,
        &mut ledger,
        &sweep,
        &considered,
        |history: History| retrieved.push(history),
    )
    .expect("nothing refused");

    assert!(admitted.is_empty());
    assert!(retrieved.is_empty());
    assert_eq!(
        edgar.transport().asked,
        vec![TICKER_MAP.to_owned(), submissions_url(320193)]
    );
}

/// A ledger that takes `keeps` records and then refuses everything, which is
/// what a pass killed part way through one looks like from the inside: the
/// verdicts before the cut landed, and nothing after it did.
///
/// What it took is kept as well. Nothing outside this crate can mint a CIK,
/// which is the right way round — a key exists because EDGAR published it — so a
/// case that has to name a filer afterwards names one the pass filed a verdict
/// under.
struct Until<L> {
    ledger: L,
    keeps: usize,
    took: Vec<FilerKey>,
}

impl<L: FilerLedger> Until<L> {
    fn keeping(ledger: L, keeps: usize) -> Self {
        Self {
            ledger,
            keeps,
            took: Vec::new(),
        }
    }
}

impl<L: FilerLedger> FilerLedger for Until<L> {
    fn record(&mut self, verdict: Record) -> Result<(), Unkept> {
        if self.keeps == 0 {
            return Err(Unkept::Unwritten {
                why: io::Error::other("the pass is gone"),
            });
        }

        self.keeps -= 1;
        self.took.push(verdict.filer().key().clone());
        self.ledger.record(verdict)
    }

    fn verdicts(&self, filer: &FilerKey) -> Result<Vec<Record>, Unkept> {
        self.ledger.verdicts(filer)
    }
}

/// A directory of this case's own, outside the repository, made by the case and
/// taken away with it however the case ends. Named for the case and the process,
/// so two of them never share one.
struct Scratch(PathBuf);

impl Scratch {
    fn named(case: &str) -> Self {
        let at = std::env::temp_dir().join(format!("vfi-funnel-{}-{case}", std::process::id()));
        let _ = fs::remove_dir_all(&at);
        fs::create_dir_all(&at).expect("a directory of its own is a case's to make");

        Self(at)
    }

    fn journal(&self) -> PathBuf {
        self.0.join("verdicts")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn journal(path: &Path) -> Journal {
    Journal::at(path).expect("a journal opens where a case put one")
}

/// What a filer's record says, as (step, pass) pairs in the order it was
/// written. Read out of a journal reopened over the same path, which is what the
/// run after an interrupted one is.
fn on_record(journal: &Journal, key: &FilerKey) -> Vec<(Step, u64)> {
    journal
        .verdicts(key)
        .expect("a journal a pass wrote reads back")
        .iter()
        .map(|record| (record.step(), record.when().pass().as_number()))
        .collect()
}

/// A pass over two filers, killed after three verdicts. What it completed is on
/// the journal and reads back; the filer it never finished with has what it had
/// when the pass reached it and nothing about the step it never got to, and no
/// history was retrieved for anybody.
#[test]
fn a_pass_that_stops_part_way_leaves_every_verdict_it_completed_and_says_nothing_more() {
    let scratch = Scratch::named("interrupted");
    let path = scratch.journal();

    let mut edgar = asking(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[
                    entry(0, 320193, "AAPL", "Apple Inc."),
                    entry(1, 1090727, "UPS", "United Parcel Service Inc"),
                ]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            )
            .to(
                &submissions_url(1090727),
                200,
                &submissions_of(1090727, &[("10-K", "2026-02-10")], &[]),
            ),
    );

    let mut ledger = Until::keeping(journal(&path), 3);
    let mut retrieved = Vec::new();

    match funnel::run(&mut edgar, &mut ledger, &sweep(), |history| {
        retrieved.push(history);
    }) {
        Err(Unswept::Unkept { .. }) => {}
        other => panic!("a verdict that did not land stops the pass, and this was {other:?}"),
    }

    // The first two verdicts of a pass are the seed step's, one per filer, in
    // the order EDGAR published them.
    let (finished, reached) = (ledger.took[0].clone(), ledger.took[1].clone());
    drop(ledger);

    let journal = journal(&path);
    assert_eq!(
        on_record(&journal, &finished),
        [(Step::Seed, 1), (Step::Metadata, 1)]
    );
    assert_eq!(on_record(&journal, &reached), [(Step::Seed, 1)]);
    assert!(retrieved.is_empty());
}

/// Which sweep a verdict belongs to is on the verdict. The two passes here are
/// dated the same to the second — the calendar stands still — so the pass number
/// is the only thing telling the record of the one that died from the record of
/// the one that finished, which is the whole reason it is written down.
#[test]
fn the_pass_is_what_tells_an_interrupted_sweep_from_the_complete_one_after_it() {
    let scratch = Scratch::named("two-passes");
    let path = scratch.journal();

    let answers = || {
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            )
    };

    let mut ledger = Until::keeping(journal(&path), 1);
    assert!(funnel::run(&mut asking(answers()), &mut ledger, &sweep(), |_| ()).is_err());
    drop(ledger);

    let mut ledger = journal(&path);
    let complete = Sweep::new(Pass::new(2), Stopped);
    let considered = funnel::run(&mut asking(answers()), &mut ledger, &complete, |_| ())
        .expect("the pass after it ran to the end");
    drop(ledger);

    let journal = journal(&path);
    let key = considered[0].clone();
    assert_eq!(
        on_record(&journal, &key),
        [
            (Step::Seed, 1),
            (Step::Seed, 2),
            (Step::Metadata, 2),
            (Step::History, 2)
        ]
    );

    let moments: Vec<_> = journal
        .verdicts(&key)
        .expect("a journal a pass wrote reads back")
        .iter()
        .map(|record| record.when().moment())
        .collect();
    assert!(
        moments.windows(2).all(|two| two[0] == two[1]),
        "the two passes are dated the same, so nothing but the pass tells them apart"
    );
}

/// A history reaches whoever the pass hands it to only after the verdict for it
/// is on record. The retrieval happened — the document was asked for twice, once
/// by each step — and the verdict did not land, so nothing was handed on: what
/// the next stage never gets is a history the ledger cannot account for.
#[test]
fn a_history_is_handed_on_only_once_its_verdict_is_on_record() {
    let mut edgar = asking(
        Answers::default()
            .to(
                TICKER_MAP,
                200,
                &map_of(&[entry(0, 320193, "AAPL", "Apple Inc.")]),
            )
            .to(
                &submissions_url(320193),
                200,
                &submissions_of(320193, &[("10-K", "2025-11-01")], &[]),
            ),
    );

    // The seed verdict and the metadata verdict, and then nothing.
    let mut ledger = Until::keeping(InMemory::new(), 2);
    let mut retrieved = Vec::new();

    match funnel::run(&mut edgar, &mut ledger, &sweep(), |history| {
        retrieved.push(history);
    }) {
        Err(Unswept::Unkept { .. }) => {}
        other => panic!("a verdict that did not land stops the pass, and this was {other:?}"),
    }

    assert!(retrieved.is_empty());
    assert_eq!(
        edgar.transport().asked,
        vec![
            TICKER_MAP.to_owned(),
            submissions_url(320193),
            submissions_url(320193),
        ]
    );
}
