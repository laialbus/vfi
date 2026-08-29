//! The documents EDGAR publishes, in the shape it publishes them.
//!
//! One type per document, holding only the fields this stage reads. Everything
//! else in the document is passed over rather than listed and ignored, so a
//! column EDGAR adds costs nothing here and a column this stage needs is added
//! in one place.
//!
//! Every string borrows the body it was read out of where it can, and owns a
//! copy only where the document escaped it. That is what [`Cow`] is doing on
//! every field: a borrowed `&str` alone would refuse a perfectly good document
//! the moment a company name contained a quote.

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;
use serde_json::value::RawValue;

/// EDGAR's map from ticker to filer, at
/// <https://www.sec.gov/files/company_tickers.json>.
///
/// Published as an object keyed by row number — `"0"`, `"1"`, and so on — which
/// carries no meaning beyond ordering, so the keys are read and dropped. They
/// are borrowed because a decimal index has nothing in it to escape; a document
/// that escaped one is refused rather than read, which is the direction to be
/// wrong in.
pub(super) type TickerMap<'a> = HashMap<&'a str, TickerRow<'a>>;

/// One row of that map.
#[derive(Deserialize)]
pub(super) struct TickerRow<'a> {
    /// The CIK, as a number. The name is EDGAR's; the value is not a string.
    pub(super) cik_str: u64,
    #[serde(borrow)]
    pub(super) ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub(super) title: Cow<'a, str>,
}

/// A filer's submissions document, at
/// <https://data.sec.gov/submissions/CIK##########.json>.
///
/// EDGAR publishes what it holds about a filer and a page of that filer's
/// filings in the one document, so both steps that ask about a filer read this
/// type. What each of them may take out of it is the step's business, and the
/// types they hand back are where that line is drawn.
#[derive(Deserialize)]
pub(super) struct Submissions<'a> {
    /// The CIK the document is about, padded to ten digits.
    #[serde(borrow)]
    pub(super) cik: Cow<'a, str>,
    #[serde(borrow)]
    pub(super) filings: Filings<'a>,
}

/// The filings half of that document: one page inline, and the rest — for a
/// filer with more history than a page holds — named for fetching.
#[derive(Deserialize)]
pub(super) struct Filings<'a> {
    #[serde(borrow)]
    pub(super) recent: Page<'a>,
    #[serde(borrow)]
    pub(super) files: Vec<Overflow<'a>>,
}

/// A page of filings, held as one array per column rather than one object per
/// filing. This is the shape both inside a submissions document and in the
/// separate documents the overflow is published as.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Page<'a> {
    #[serde(borrow)]
    pub(super) accession_number: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub(super) filing_date: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub(super) report_date: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub(super) form: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub(super) primary_document: Vec<Cow<'a, str>>,
}

/// Where the rest of a long history is published: a file name, under the same
/// path the submissions document itself came from.
#[derive(Deserialize)]
pub(super) struct Overflow<'a> {
    #[serde(borrow)]
    pub(super) name: Cow<'a, str>,
}

/// A filer's XBRL company facts, at
/// <https://data.sec.gov/api/xbrl/companyfacts/CIK##########.json>.
///
/// EDGAR's collation of the XBRL exhibits of the filer's own periodic filings:
/// every fact it tagged, under the taxonomy that defines the element, then the
/// element, then the unit it was reported in. Those three keys are the document
/// saying what a value is of, and each one is carried down onto every fact
/// underneath it, because the boundary this feeds carries a flat list and a
/// fact that lost its taxonomy or its unit is a number nobody can read.
///
/// Ordered maps rather than hashed ones. A list has an order and an object does
/// not, so the order is taken from the keys the document nests the facts under;
/// a hashed map would hand back whatever the table happened to hold, which is
/// not the same twice over one document.
///
/// Those keys borrow the body, so a document that escaped one is refused rather
/// than read — the same trade the ticker map's row numbers make, and the same
/// direction to be wrong in.
#[derive(Deserialize)]
pub(super) struct CompanyFacts<'a> {
    /// The filer the document is about, padded to ten digits, which is how a
    /// submissions document writes the same key.
    #[serde(borrow)]
    pub(super) cik: Cow<'a, str>,
    #[serde(borrow)]
    pub(super) facts: BTreeMap<&'a str, BTreeMap<&'a str, Element<'a>>>,
}

/// One element the filer tagged, and every value it reported under it, by unit.
///
/// The document publishes the taxonomy's label and description for the element
/// beside these. Neither crosses the boundary, so neither is named here.
#[derive(Deserialize)]
pub(super) struct Element<'a> {
    #[serde(borrow)]
    pub(super) units: BTreeMap<&'a str, Vec<Reported<'a>>>,
}

/// One value reported under an element, in one unit, by one filing.
#[derive(Deserialize)]
pub(super) struct Reported<'a> {
    /// The day the fact's period starts, where it has one. A fact stated at an
    /// instant publishes no `start` and its `end` is that instant, which is the
    /// whole of how this document says which of the two shapes a period takes.
    #[serde(borrow, default)]
    pub(super) start: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub(super) end: Cow<'a, str>,
    /// The amount, held as the characters the document publishes and not as a
    /// number read out of them. A decimal read into a binary float and written
    /// back out is no longer the literal that was published, and the literal is
    /// what this boundary carries.
    #[serde(borrow)]
    pub(super) val: &'a RawValue,
    #[serde(borrow)]
    pub(super) accn: Cow<'a, str>,
    #[serde(borrow)]
    pub(super) form: Cow<'a, str>,
    #[serde(borrow)]
    pub(super) filed: Cow<'a, str>,
}

impl Page<'_> {
    /// How many filings this page holds, or which two columns disagree.
    ///
    /// The columns are parallel arrays and nothing in the document says they
    /// are the same length. Reading row `i` out of five arrays that are not
    /// would pair one filing's accession number with another's date — a filing
    /// that never happened, and one that would look entirely ordinary
    /// downstream. So the lengths are checked once, here, before a row is read.
    pub(super) fn filings(&self) -> Result<usize, String> {
        let rows = self.accession_number.len();

        for (column, length) in [
            ("filingDate", self.filing_date.len()),
            ("reportDate", self.report_date.len()),
            ("form", self.form.len()),
            ("primaryDocument", self.primary_document.len()),
        ] {
            if length != rows {
                return Err(format!(
                    "accessionNumber holds {rows} filings and {column} holds {length}"
                ));
            }
        }

        Ok(rows)
    }
}
