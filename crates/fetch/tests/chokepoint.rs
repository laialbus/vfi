//! That nothing leaves without passing the list.
//!
//! This is the behaviour half: the chokepoint asked for a host it may reach and
//! a host it may not, from outside the crate, where the only thing in reach is
//! what the crate publishes. The transport records what it was handed, so a
//! case can say that nothing was sent rather than only that an error came back
//! — a refusal after the request went out would look the same to a caller
//! reading the return value alone.
//!
//! The other half is not here and cannot be. That no call site reaches a source
//! around this seam is the `egress` gate over the workspace, plus the privacy
//! the compiler holds on `Cleared`; a green test here says nothing about a path
//! that skipped it.
//!
//! Nothing in this file reaches a real source. The transport is a stand-in that
//! answers from memory, and the hosts the cases ask for are the listed ones and
//! `.invalid`, which by RFC 2606 resolves nowhere.

use std::io;

use vfi_fetch::{
    ALLOWED_HOSTS, Cleared, DECLARATION_HEADER, Declaration, Egress, Error, Pace, Response,
    Transport,
};

/// A transport with no wire under it. It answers every cleared request the same
/// way and keeps what each was to send, which is what a case reads to find out
/// whether anything was sent at all.
#[derive(Default)]
struct Recorder {
    sent: Vec<String>,
    declared: Vec<String>,
}

impl Transport for Recorder {
    fn send(&mut self, request: Cleared<'_>) -> io::Result<Response> {
        self.sent.push(request.host().to_owned());
        for (name, value) in request.headers() {
            if name == DECLARATION_HEADER {
                self.declared.push(value.to_owned());
            }
        }
        Ok(Response {
            status: 200,
            body: b"a filing, if this were a real source".to_vec(),
        })
    }
}

/// The listed host the cases are built from. Read off the list rather than
/// written here, so this file is not a second place saying which hosts the
/// stage may reach.
fn listed() -> &'static str {
    ALLOWED_HOSTS
        .first()
        .expect("the list allows no host, so there is nothing to fetch from")
}

/// Who these cases say is asking. A declaration is the user's to supply, and
/// this is a test standing in for one; the address is `.invalid`, which by
/// RFC 2606 reaches nobody.
fn declaration() -> Declaration {
    Declaration::new("VFI test suite nobody@example.invalid").expect("this names somebody")
}

/// A chokepoint over a fresh recorder, paced against the machine's clock.
///
/// The real pace costs these cases nothing: each builds its own, and a pace
/// owes its first request no wait. A case here that sent twice would sleep, and
/// what a turn is worth belongs in `pacing.rs` anyway, where the clock is the
/// test's.
fn chokepoint() -> Egress<Recorder> {
    Egress::new(Recorder::default(), declaration(), Pace::system())
}

fn withheld_host(url: &str, egress: &mut Egress<Recorder>) -> String {
    match egress.fetch(url) {
        Err(Error::Withheld { host }) => host,
        other => panic!("{url}: withheld is the outcome, and this was {other:?}"),
    }
}

#[test]
fn a_listed_host_is_sent() {
    let mut egress = chokepoint();
    let host = listed();

    let response = egress
        .fetch(&format!("https://{host}/cgi-bin/browse-edgar"))
        .expect("a listed host is what the transport is for");

    assert_eq!(response.status, 200);
    assert_eq!(egress.transport().sent, vec![host.to_owned()]);
}

/// The refusal is a value, and it names the host it withheld: a caller holding
/// one is deciding whether the list is wrong or the URL is, and it cannot do
/// that without knowing which host was refused.
#[test]
fn an_unlisted_host_is_withheld_by_name_and_nothing_is_sent() {
    let mut egress = chokepoint();

    let refusal = egress
        .fetch("https://filings.example.invalid/latest")
        .expect_err("an unlisted host is not fetched");

    match &refusal {
        Error::Withheld { host } => assert_eq!(host, "filings.example.invalid"),
        other => panic!("withheld is the outcome, and this was {other:?}"),
    }
    assert!(
        refusal.to_string().contains("filings.example.invalid"),
        "the refusal has to say which host it withheld, and it said: {refusal}"
    );
    assert!(
        egress.transport().sent.is_empty(),
        "a withheld host was handed to the transport anyway"
    );
}

/// Each of these would be allowed by a rule that matched part of a host name
/// instead of the whole of it, and each belongs to somebody else.
#[test]
fn a_host_that_only_resembles_a_listed_one_is_withheld() {
    let mut egress = chokepoint();
    let listed = listed();

    for url in [
        format!("https://{listed}.example.invalid/x"),
        format!("https://example-{listed}/x"),
    ] {
        withheld_host(&url, &mut egress);
    }

    assert!(egress.transport().sent.is_empty());
}

/// A URL that would reach elsewhere is refused before the list is consulted, so
/// the transport never sees it either. What the caller gets back is the URL and
/// what could not be read out of it, rather than a host that was never really
/// there.
#[test]
fn a_url_that_reads_two_ways_is_unreadable_and_nothing_is_sent() {
    let mut egress = chokepoint();
    let listed = listed();

    for url in [
        format!("https://{listed}@filings.example.invalid/x"),
        format!("http://{listed}/x"),
        format!("https://{listed}:8443/x"),
    ] {
        match egress.fetch(&url) {
            Err(Error::Unreadable { url: given, .. }) => assert_eq!(given, url),
            other => panic!("{url}: unreadable is the outcome, and this was {other:?}"),
        }
    }

    assert!(egress.transport().sent.is_empty());
}

/// The stand-in transport failing is a third outcome, distinct from both
/// refusals: the list allowed the host and the wire is what did not work.
#[test]
fn a_transport_that_fails_is_not_a_withheld_host() {
    struct Broken;

    impl Transport for Broken {
        fn send(&mut self, _request: Cleared<'_>) -> io::Result<Response> {
            Err(io::Error::other("the stand-in transport has no wire"))
        }
    }

    let mut egress = Egress::new(Broken, declaration(), Pace::system());
    let host = listed();

    match egress.fetch(&format!("https://{host}/")) {
        Err(Error::Unreachable { host: tried, .. }) => assert_eq!(tried, host),
        other => panic!("unreachable is the outcome, and this was {other:?}"),
    }
}

/// The source asks to be told who is asking, and what it is told is what the
/// caller supplied — this crate has no name of its own to give and does not
/// invent one. A transport reads the headers rather than the declaration
/// itself, so the case reads them the same way.
#[test]
fn a_request_that_is_sent_declares_who_is_asking() {
    let mut egress = chokepoint();
    let host = listed();

    egress
        .fetch(&format!("https://{host}/cgi-bin/browse-edgar"))
        .expect("a listed host is what the transport is for");

    assert_eq!(
        egress.transport().declared,
        vec![declaration().as_str().to_owned()],
        "a request left under a declaration that was not the one supplied"
    );
}
