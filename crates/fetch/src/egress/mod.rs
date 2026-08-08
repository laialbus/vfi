//! The chokepoint: the one way out of this stage.
//!
//! [`Egress::fetch`] reads the host out of the URL, asks [`crate::hosts`], and
//! hands the request on only then. The order is the whole of it — the list is
//! read before anything is opened, so a withheld host costs no connection.
//!
//! Two things keep a caller from going around that, and neither of them is a
//! test. [`Transport::send`] takes a [`Cleared`], whose fields are private to
//! this module and which nothing outside it can construct: a call site that
//! tried to hand a transport a request the list has not seen does not compile.
//! What privacy cannot see is a call site that needs no transport at all,
//! because opening a connection is in every crate's reach; the `egress` gate
//! reads the workspace for the names that do it and allows them in this
//! directory alone.
//!
//! That is what this directory is for. Code that talks to a real source belongs
//! here, underneath the check; everything else in the crate belongs outside it,
//! where the ban holds.

use std::fmt;
use std::io;

use crate::hosts;

/// The scheme a fetch speaks, and the only one. A list of hosts is a statement
/// about who answers and says nothing about a URL that would reach the same
/// name in the clear, or one that would never leave the machine.
const SCHEME: &str = "https://";

/// A request that has been through the list, and the only thing a transport can
/// be handed.
///
/// It is made in one place — [`Egress::fetch`], after the check — so holding
/// one is evidence the check happened. That is why there is no constructor: a
/// `Cleared::new` would be a way to state the conclusion without the premise.
#[derive(Debug)]
pub struct Cleared<'a> {
    url: &'a str,
    host: &'a str,
}

impl<'a> Cleared<'a> {
    /// The URL to request, as the caller wrote it.
    pub fn url(&self) -> &'a str {
        self.url
    }

    /// The host it will reach, read out of that URL by the check that cleared
    /// it. A transport connects to this rather than reading the URL again: two
    /// readings of one URL is how the host that was checked and the host that
    /// is reached come apart.
    pub fn host(&self) -> &'a str {
        self.host
    }
}

/// What a source answered with.
#[derive(Debug)]
pub struct Response {
    /// The status it answered with. A status is an answer and not a failure: a
    /// 404 is the source saying the filing is not there, which is something the
    /// caller decides what to do about.
    pub status: u16,
    /// The body, as it arrived and undecoded.
    pub body: Vec<u8>,
}

/// Where a cleared request goes: one implementation per source.
///
/// An implementation is reachable only through [`Egress`], because this is the
/// only method that takes a [`Cleared`] and [`Egress::fetch`] is the only thing
/// that makes one.
pub trait Transport {
    /// Send `request` and wait for the answer.
    ///
    /// An error is the wire failing, not the source answering badly. It is an
    /// [`io::Error`] because that is what an implementation will be holding
    /// already, whatever it is built on.
    fn send(&mut self, request: Cleared<'_>) -> io::Result<Response>;
}

/// Why a fetch produced no response.
///
/// Each of these is returned. Asking for a host the list does not allow is an
/// ordinary thing to do — a caller may have read the URL out of a filing — so
/// it is answered rather than panicked over, and it is answered rather than
/// passed over, because a fetch that quietly did nothing is indistinguishable
/// from a source with nothing to say.
#[derive(Debug)]
pub enum Error {
    /// No host could be read out of the URL. Nothing was sent.
    Unreadable {
        /// The URL as it was given.
        url: String,
        /// What could not be read out of it.
        why: &'static str,
    },
    /// The host is not on the list. Nothing was sent and nothing was opened.
    Withheld {
        /// The host that was withheld.
        host: String,
    },
    /// The list allowed the host and the transport could not reach it.
    Unreachable {
        /// The host that was tried.
        host: String,
        /// What the transport said about it.
        why: io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unreadable { url, why } => {
                write!(f, "{url}: no host to check, because {why}")
            }
            Error::Withheld { host } => {
                write!(
                    f,
                    "{host}: withheld, because it is not a host this stage may reach"
                )
            }
            Error::Unreachable { host, why } => write!(f, "{host}: not reached, because {why}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Unreachable { why, .. } => Some(why),
            Error::Unreadable { .. } | Error::Withheld { .. } => None,
        }
    }
}

/// The chokepoint itself, holding the transport that requests leave through.
pub struct Egress<T> {
    transport: T,
}

impl<T: Transport> Egress<T> {
    /// Take `transport`, and be the only way anything reaches it.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// The transport, to read what it has been given. Lending it out hands over
    /// no way to send: [`Transport::send`] wants a [`Cleared`], and they are
    /// made here.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Fetch `url`, if its host is one this stage may reach.
    ///
    /// The list is read first and the transport is touched only after it
    /// answers, so a host it does not allow is refused with nothing sent and
    /// nothing opened. The refusal names that host: a caller looking at a
    /// withheld URL is deciding whether the list is wrong or the URL is.
    pub fn fetch(&mut self, url: &str) -> Result<Response, Error> {
        let host = host_of(url).map_err(|why| Error::Unreadable {
            url: url.to_owned(),
            why,
        })?;

        if !hosts::allows(host) {
            return Err(Error::Withheld {
                host: host.to_owned(),
            });
        }

        self.transport
            .send(Cleared { url, host })
            .map_err(|why| Error::Unreachable {
                host: host.to_owned(),
                why,
            })
    }
}

/// The host `url` would reach, or why one cannot be read out of it.
///
/// Strict on purpose, and it refuses where it could resolve. A URL that a
/// checker and a client read differently is the shape an allowlist leaks
/// through: `https://www.sec.gov@filings.example/` reaches `filings.example`,
/// and a reading that stopped at the `@` would wave through every host there
/// is. So anything with a second reading — userinfo, a port, an address
/// literal, a byte a host name is not made of — is unreadable here, and the
/// task that has a use for one of them is the one that says what it means.
fn host_of(url: &str) -> Result<&str, &'static str> {
    let authority = url.strip_prefix(SCHEME).ok_or("only https is fetched")?;
    let end = authority.find(['/', '?', '#']).unwrap_or(authority.len());
    let host = &authority[..end];

    if host.is_empty() {
        return Err("it names no host");
    }
    if host.contains('@') {
        return Err("the host of a URL carrying userinfo is what follows the @");
    }
    if host.contains(':') {
        return Err("a port is not part of what the list allows");
    }
    if !host
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'.')
    {
        return Err("a host name here is ASCII letters, digits, - and .");
    }

    Ok(host)
}

#[cfg(test)]
mod tests {
    use super::host_of;

    #[test]
    fn the_host_is_the_authority_and_stops_where_it_stops() {
        assert_eq!(host_of("https://data.sec.gov"), Ok("data.sec.gov"));
        assert_eq!(host_of("https://data.sec.gov/"), Ok("data.sec.gov"));
        assert_eq!(
            host_of("https://data.sec.gov/api/xbrl/frames"),
            Ok("data.sec.gov")
        );
        assert_eq!(host_of("https://data.sec.gov?q=1"), Ok("data.sec.gov"));
        assert_eq!(host_of("https://data.sec.gov#top"), Ok("data.sec.gov"));
    }

    /// The case the list exists to survive. Every one of these reads as an
    /// allowed host to something that looks at the front of the URL, and every
    /// one of them reaches somewhere else.
    #[test]
    fn a_url_that_would_reach_elsewhere_is_unreadable() {
        assert!(host_of("https://www.sec.gov@filings.example/x").is_err());
        assert!(host_of("https://www.sec.gov:pass@filings.example/x").is_err());
        assert!(host_of("https://www.sec.gov\\@filings.example/x").is_err());
    }

    #[test]
    fn what_is_not_a_plain_named_host_is_unreadable() {
        assert!(host_of("https://").is_err());
        assert!(host_of("https:///archives").is_err());
        assert!(host_of("https://www.sec.gov:443/x").is_err());
        assert!(host_of("https://[::1]/x").is_err());
        assert!(host_of("https://www.sec%2egov/x").is_err());
        assert!(host_of("https://ｗww.sec.gov/x").is_err());
    }

    /// The scheme is refused before a host is read at all, so none of these is
    /// a host the list is then asked about. It is matched exactly, so the last
    /// one — which a client would accept, since a scheme is case-insensitive —
    /// is refused as well rather than folded and accepted. Refusing a URL
    /// somebody would have to rewrite costs a message; resolving one costs a
    /// second reading of it.
    #[test]
    fn a_scheme_that_is_not_exactly_https_is_unreadable() {
        assert!(host_of("http://www.sec.gov/x").is_err());
        assert!(host_of("file:///etc/passwd").is_err());
        assert!(host_of("www.sec.gov").is_err());
        assert!(host_of("HTTPS://www.sec.gov/x").is_err());
    }

    /// Case is carried through untouched rather than folded here. Whether two
    /// spellings are one host is the list's judgement to make, and making it
    /// twice is how the two answers come apart.
    #[test]
    fn the_host_is_read_as_it_was_written() {
        assert_eq!(host_of("https://WWW.SEC.GOV/x"), Ok("WWW.SEC.GOV"));
    }
}
