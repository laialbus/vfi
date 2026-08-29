//! Asking EDGAR what a company has filed.
//!
//! EDGAR keys its filing endpoints on the CIK it assigned a filer and not on
//! the ticker anyone trades under, so a lookup is two documents: the map from
//! one to the other, and then the filer's own submissions. Both are fetched
//! through [`Egress`], because that is the only way out of this stage, and each
//! record that comes back carries the [`Source`] of the request that produced
//! it.
//!
//! Where these documents live is read off EDGAR's own page on accessing its
//! data —
//! <https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data>
//! — which is the same page [`crate::policy`] takes the access rate from.
//!
//! Three answers are answers rather than failures, and each is said out loud:
//! a ticker the map does not name, a filer the map names that has filed
//! nothing, and a history longer than the one page a submissions document
//! carries inline. The first two are [`Retrieved`]; the third is followed,
//! because a history that stopped at the first page would be missing its oldest
//! years and would look complete.
//!
//! A third document answers a different question, and it is [`company_facts`]:
//! not what the filer filed but what it reported, collated out of the XBRL
//! exhibits of its own filings. It is kept in a module of its own because it is
//! the one retrieval here that hands back a value the boundary defines rather
//! than a value this crate does.
//!
//! The same two documents answer the funnel's first two steps, which ask
//! smaller questions of them. The ticker map is also the list of filers there
//! are — [`seed_set`] — and a submissions document is also what EDGAR publishes
//! about one filer before anybody asks what it has filed — [`metadata`]. That
//! second one is the one to read carefully: it stops at the document, and the
//! overflow pages it names are not followed, so a filer judged out of the
//! corpus costs one request and never the rest of them.

mod documents;
mod facts;

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::Entry as Taken;
use std::fmt;

use documents::{Page, Submissions, TickerMap, TickerRow};

pub use facts::company_facts;

use crate::company::{Cik, Company, Ticker};
use crate::egress::{Egress, Transport};
use crate::filing::{Filing, History, Published};
use crate::source::Source;

/// EDGAR's map from ticker to filer.
const TICKER_MAP: &str = "https://www.sec.gov/files/company_tickers.json";

/// Where a filer's submissions document and the overflow pages of its history
/// both sit. The document is `CIK` and the padded key; a page is named by the
/// document that points at it.
const SUBMISSIONS: &str = "https://data.sec.gov/submissions/";

/// The one status this reads a document out of. Every other status is EDGAR
/// answering something other than "here it is", and which of those a caller
/// should retry, escalate, or take as a filer that is not there is a judgement
/// this crate does not make for them — so the status is handed back as it came.
const OK: u16 = 200;

fn submissions_url(cik: Cik) -> String {
    format!("{SUBMISSIONS}CIK{cik}.json")
}

/// The URL an overflow page is published at, or why the name in the document is
/// not one.
///
/// A submissions document supplies this name, and this crate builds a URL out
/// of it: a name carrying a slash or a `..` would address something other than
/// the page EDGAR meant, still on a host the list allows. So the name has to be
/// a plain file name, and one that is not stops the read rather than being
/// trimmed into one.
fn page_url(name: &str) -> Result<String, String> {
    let plain = !name.is_empty()
        && !name.starts_with('.')
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));

    if !plain {
        return Err(format!(
            "{name:?} is not a file name, so there is no page it names"
        ));
    }

    Ok(format!("{SUBMISSIONS}{name}"))
}

/// Why nothing came back.
///
/// Each of these is a request that produced no record, and each carries the
/// [`Source`] it failed at, because "EDGAR did not answer" is only actionable
/// once you know which of the two documents it did not answer for.
#[derive(Debug)]
pub enum Unretrieved {
    /// The request did not leave, or did not arrive. The reason is the
    /// chokepoint's: a host the list withholds, a URL it will not read, or a
    /// transport that could not reach the source.
    Unfetched {
        /// The request that did not go out, or came back with nothing.
        source: Source,
        /// What the chokepoint said about it.
        why: crate::Error,
    },
    /// EDGAR answered with a status other than [`OK`], so there is no document
    /// to read.
    Refused {
        /// The request that was answered.
        source: Source,
        /// The status it was answered with.
        status: u16,
    },
    /// EDGAR answered, and the body is not the document this endpoint
    /// publishes. Nothing is taken from a document that is not the one
    /// expected: a value read out of a shape this did not recognise is a guess
    /// wearing a number's clothes.
    Unreadable {
        /// The request that was answered.
        source: Source,
        /// What could not be read, in the document's own terms.
        why: String,
    },
}

impl fmt::Display for Unretrieved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unretrieved::Unfetched { source, why } => write!(f, "{source}: not fetched, {why}"),
            Unretrieved::Refused { source, status } => {
                write!(f, "{source}: answered {status}, which is not a document")
            }
            Unretrieved::Unreadable { source, why } => {
                write!(
                    f,
                    "{source}: not the document this publishes, because {why}"
                )
            }
        }
    }
}

impl Unretrieved {
    /// The request that produced no record.
    ///
    /// Every variant carries one, and a caller that has to say what it was
    /// doing when nothing came back should not have to match to find out which.
    pub fn request(&self) -> &Source {
        match self {
            Unretrieved::Unfetched { source, .. }
            | Unretrieved::Refused { source, .. }
            | Unretrieved::Unreadable { source, .. } => source,
        }
    }
}

impl std::error::Error for Unretrieved {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Unretrieved::Unfetched { why, .. } => Some(why),
            Unretrieved::Refused { .. } | Unretrieved::Unreadable { .. } => None,
        }
    }
}

/// What asking EDGAR about a ticker produced.
///
/// Two of these are the answers that are not a filing history, and they are
/// separate variants rather than an empty list and a null, because a caller
/// deciding what to do about a company — the filer funnel, recording why each
/// filer was kept or dropped — needs to tell "EDGAR has never heard of this"
/// from "EDGAR knows it and it has filed nothing".
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Retrieved {
    /// EDGAR's map does not name this ticker. Which company was meant is not
    /// guessed at: the nearest name in the map is not an answer.
    UnknownTicker {
        /// The ticker that was asked about.
        ticker: Ticker,
        /// The map that does not name it.
        source: Source,
    },
    /// EDGAR names the filer and publishes no filing for it.
    NoFilings {
        /// The filer the map named.
        company: Company,
        /// The submissions document that published no filing.
        source: Source,
    },
    /// The filings EDGAR publishes for the filer its map names.
    History(History),
}

/// EDGAR's map from ticker to the filer it keys on, fetched once.
///
/// Held as a whole because that is how EDGAR publishes it — one document for
/// every listed company — so a caller resolving many tickers fetches it once
/// and asks it many times. That is the shape the filer funnel wants: a seed set
/// of tickers is thousands of lookups against one document, not thousands of
/// requests.
pub struct Directory {
    filers: HashMap<Ticker, Company>,
    source: Source,
}

impl Directory {
    /// Fetch the map.
    pub fn fetch<T: Transport>(edgar: &mut Egress<T>) -> Result<Self, Unretrieved> {
        let source = Source::new(TICKER_MAP);
        let body = get(edgar, &source)?;

        let map: TickerMap<'_> = read(&body, &source)?;
        let mut filers = HashMap::with_capacity(map.len());

        for row in map.values() {
            let ticker = Ticker::new(&row.ticker);
            let company = Company::new(Cik::new(row.cik_str), &row.title, source.clone());

            match filers.entry(ticker) {
                Taken::Vacant(slot) => {
                    slot.insert(company);
                }
                // Two filers under one ticker, which is EDGAR's map saying two
                // things. Answering with either would be picking the company
                // the caller meant, so nothing is answered at all.
                Taken::Occupied(taken) if taken.get().cik() != company.cik() => {
                    return Err(Unretrieved::Unreadable {
                        source,
                        why: format!(
                            "{} names two filers, {} and {}",
                            taken.key(),
                            taken.get().cik(),
                            company.cik()
                        ),
                    });
                }
                Taken::Occupied(_) => {}
            }
        }

        Ok(Self { filers, source })
    }

    /// The filer EDGAR's map names for `ticker`, if it names one.
    pub fn company(&self, ticker: &Ticker) -> Option<&Company> {
        self.filers.get(ticker)
    }

    /// The request the map came from.
    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// One entry of the seed set, as EDGAR published it.
///
/// The identifier is optional because this is what the map carried and not what
/// a reader wishes it had carried. An entry naming no filer is an answer about
/// that entry — one the funnel puts on record under whatever the entry did have
/// — rather than a reason to stop reading the ten thousand around it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    row: u64,
    ticker: Ticker,
    title: Box<str>,
    key: u64,
}

impl Entry {
    /// The filer this entry names, where it names one. EDGAR assigns no filer
    /// the key zero, so an entry carrying that names none.
    pub fn cik(&self) -> Option<Cik> {
        (self.key != 0).then(|| Cik::new(self.key))
    }

    /// The key as the entry carried it, whether or not it names a filer. This
    /// is the value a verdict about the entry is checked against later, so it
    /// is handed over as it read.
    pub fn key(&self) -> u64 {
        self.key
    }

    /// The symbol the entry was listed under.
    pub fn ticker(&self) -> &Ticker {
        &self.ticker
    }

    /// The name beside it.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// What there is to file this entry under when it names no filer: the
    /// symbol, the name beside it, or failing both the row it was read from.
    /// One of the three is always there, so an entry nothing resolved is never
    /// recorded under nothing.
    pub fn listed_as(&self) -> Cow<'_, str> {
        if !self.ticker.as_str().is_empty() {
            Cow::Borrowed(self.ticker.as_str())
        } else if !self.title.is_empty() {
            Cow::Borrowed(&self.title)
        } else {
            Cow::Owned(self.row.to_string())
        }
    }
}

/// The filers EDGAR publishes, and where it published them.
pub struct Seeds {
    entries: Vec<Entry>,
    source: Source,
}

impl Seeds {
    /// The entries, in the order the map publishes them.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The request the map came from.
    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// The seed set: every filer EDGAR's map publishes, in the order it publishes
/// them.
///
/// The same document [`Directory`] is built from, read as a list rather than as
/// a map, because the two steps want different things of it. A lookup wants one
/// filer by ticker and does not care what order the rows came in; a pass over
/// the seed set is the rows themselves, and it has to make the same set in the
/// same order every time it runs. So the row numbers, which a lookup reads and
/// drops, are what this orders by.
pub fn seed_set<T: Transport>(edgar: &mut Egress<T>) -> Result<Seeds, Unretrieved> {
    let source = Source::new(TICKER_MAP);
    let body = get(edgar, &source)?;

    let map: TickerMap<'_> = read(&body, &source)?;
    let mut rows: Vec<(u64, &TickerRow<'_>)> = Vec::with_capacity(map.len());

    for (row, entry) in &map {
        let number = row.parse::<u64>().map_err(|_| Unretrieved::Unreadable {
            source: source.clone(),
            why: format!("{row:?} is not a row number, and this map is keyed by one"),
        })?;
        rows.push((number, entry));
    }
    rows.sort_unstable_by_key(|(number, _)| *number);

    let entries = rows
        .into_iter()
        .map(|(row, entry)| Entry {
            row,
            ticker: Ticker::new(&entry.ticker),
            title: Box::from(entry.title.as_ref()),
            key: entry.cik_str,
        })
        .collect();

    Ok(Seeds { entries, source })
}

/// One filing as a metadata document lists it: which form it was filed on, and
/// when.
///
/// Deliberately less than a [`Filing`]. What is missing — the accession number,
/// the document it names — is what a later stage would need to read the filing
/// itself, and leaving it out is what keeps a metadata document from standing
/// in for the history the step after this one retrieves. The step boundary is
/// this type rather than a rule somebody remembers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listed {
    form: Box<str>,
    filed: Box<str>,
}

impl Listed {
    /// The form it was filed on, as EDGAR spells it.
    pub fn form(&self) -> &str {
        &self.form
    }

    /// The day it was filed, as EDGAR writes one.
    pub fn filed(&self) -> &str {
        &self.filed
    }
}

/// What EDGAR publishes about one filer, before its history is asked for.
pub struct Metadata {
    cik: Cik,
    listed: Vec<Listed>,
    source: Source,
}

impl Metadata {
    /// The filer this is about.
    pub fn cik(&self) -> Cik {
        self.cik
    }

    /// The filings the document names, in the order it names them.
    pub fn listed(&self) -> &[Listed] {
        &self.listed
    }

    /// The request it came from.
    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// What EDGAR publishes about the filer it keys as `cik`.
///
/// One request, and the pages of older filings the document names are not
/// among them. That is the whole point of the step this answers: a filer the
/// funnel is about to judge costs the same whether it has filed twice or two
/// thousand times, and one it judges out costs nothing after that.
pub fn metadata<T: Transport>(edgar: &mut Egress<T>, cik: Cik) -> Result<Metadata, Unretrieved> {
    let source = Source::new(&submissions_url(cik));
    let body = get(edgar, &source)?;

    let submissions: Submissions<'_> = read(&body, &source)?;
    filed_under(&submissions, cik, &source)?;

    let page = &submissions.filings.recent;
    let rows = page.filings().map_err(|why| Unretrieved::Unreadable {
        source: source.clone(),
        why,
    })?;

    let mut listed = Vec::with_capacity(rows);
    for row in 0..rows {
        listed.push(Listed {
            form: Box::from(page.form[row].as_ref()),
            filed: Box::from(page.filing_date[row].as_ref()),
        });
    }

    Ok(Metadata {
        cik,
        listed,
        source,
    })
}

/// That the document EDGAR answered with is the one that was asked for.
///
/// A document filed under a key nobody asked about would be a whole company's
/// worth of wrong facts, every one of them well-formed, and it would be as
/// wrong for a verdict as for a history.
fn filed_under(
    submissions: &Submissions<'_>,
    cik: Cik,
    source: &Source,
) -> Result<(), Unretrieved> {
    if submissions.cik.parse::<u64>().ok() == Some(cik.as_number()) {
        return Ok(());
    }

    Err(Unretrieved::Unreadable {
        source: source.clone(),
        why: format!(
            "it is filed under {}, and {cik} is what was asked for",
            submissions.cik
        ),
    })
}

/// Every filing EDGAR publishes for `company`, newest first.
///
/// A submissions document carries a page of filings inline and names the
/// documents the rest sit in; every one of those is fetched, so what comes back
/// is the whole history rather than its most recent page. An empty history is
/// an answer: EDGAR names the filer and publishes nothing under it.
pub fn history<T: Transport>(
    edgar: &mut Egress<T>,
    company: &Company,
) -> Result<History, Unretrieved> {
    let source = Source::new(&submissions_url(company.cik()));
    let body = get(edgar, &source)?;

    let mut filings = Vec::new();
    let overflow: Vec<String> = {
        let submissions: Submissions<'_> = read(&body, &source)?;
        filed_under(&submissions, company.cik(), &source)?;

        collect(&submissions.filings.recent, &source, &mut filings)?;
        submissions
            .filings
            .files
            .iter()
            .map(|page| page.name.as_ref().to_owned())
            .collect()
    };

    for name in overflow {
        // The refusal belongs to the submissions document, because that is
        // where a name that is not a file name was published.
        let url = page_url(&name).map_err(|why| Unretrieved::Unreadable {
            source: source.clone(),
            why,
        })?;

        let carried_by = Source::new(&url);
        let body = get(edgar, &carried_by)?;
        let page: Page<'_> = read(&body, &carried_by)?;

        collect(&page, &carried_by, &mut filings)?;
    }

    Ok(History::new(company.clone(), source, filings))
}

/// The filing history EDGAR publishes for a ticker.
///
/// Two requests at least: the map, then the filer's submissions. A caller
/// asking about more than one ticker holds a [`Directory`] and calls
/// [`history`] against it instead — the map is one document for every listed
/// company, and fetching it once per ticker is fetching it once per ticker.
pub fn filing_history<T: Transport>(
    edgar: &mut Egress<T>,
    ticker: &Ticker,
) -> Result<Retrieved, Unretrieved> {
    let directory = Directory::fetch(edgar)?;

    let Some(company) = directory.company(ticker).cloned() else {
        return Ok(Retrieved::UnknownTicker {
            ticker: ticker.clone(),
            source: directory.source().clone(),
        });
    };

    let history = history(edgar, &company)?;

    if history.filings().is_empty() {
        return Ok(Retrieved::NoFilings {
            source: history.source().clone(),
            company,
        });
    }

    Ok(Retrieved::History(history))
}

/// Fetch `source` and hand back the body EDGAR answered with.
fn get<T: Transport>(edgar: &mut Egress<T>, source: &Source) -> Result<Vec<u8>, Unretrieved> {
    let response = edgar
        .fetch(source.url())
        .map_err(|why| Unretrieved::Unfetched {
            source: source.clone(),
            why,
        })?;

    match response.status {
        OK => Ok(response.body),
        status => Err(Unretrieved::Refused {
            source: source.clone(),
            status,
        }),
    }
}

/// Read `body` as the document `source` publishes.
fn read<'a, D: serde::Deserialize<'a>>(body: &'a [u8], source: &Source) -> Result<D, Unretrieved> {
    serde_json::from_slice(body).map_err(|why| Unretrieved::Unreadable {
        source: source.clone(),
        why: why.to_string(),
    })
}

/// Take every filing on `page` into `filings`, in the order it publishes them.
fn collect(page: &Page<'_>, source: &Source, filings: &mut Vec<Filing>) -> Result<(), Unretrieved> {
    let rows = page.filings().map_err(|why| Unretrieved::Unreadable {
        source: source.clone(),
        why,
    })?;

    filings.reserve(rows);
    for row in 0..rows {
        filings.push(
            Published {
                accession: &page.accession_number[row],
                form: &page.form[row],
                filed: &page.filing_date[row],
                period: &page.report_date[row],
                document: &page.primary_document[row],
            }
            .into_filing(source),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{SUBMISSIONS, page_url};

    #[test]
    fn a_page_is_named_under_the_path_the_document_came_from() {
        assert_eq!(
            page_url("CIK0000320193-submissions-001.json"),
            Ok(format!("{SUBMISSIONS}CIK0000320193-submissions-001.json"))
        );
    }

    /// Every one of these builds a URL on a host the list allows and reaches
    /// something other than the page EDGAR named.
    #[test]
    fn a_name_that_is_not_a_file_name_names_no_page() {
        for name in [
            "",
            "..",
            "../../api/xbrl/frames",
            "sub/missions.json",
            ".hidden",
            "CIK0000320193?x=1",
            "CIK0000320193 submissions.json",
        ] {
            assert!(page_url(name).is_err(), "{name:?} was taken as a page name");
        }
    }
}
