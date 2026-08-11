//! What EDGAR publishes about a filing, and the history those filings make up.
//!
//! Each field is carried as EDGAR published it. Nothing here parses a date,
//! folds a form type, or decides what a filing means — those are readings of
//! the data, and a reading made in the fetch stage is one the normalize stage
//! cannot check or correct. The one thing that is read is EDGAR's own spelling
//! of absence, an empty string, which becomes [`None`].
//!
//! What of a filing is carried is what identifies it and locates its documents.
//! A submissions document publishes more columns than these; the rest arrive
//! when a stage has a use for one, and what the next stage is owed is the
//! fetch → normalize contract, which is not settled here.

use crate::company::Company;
use crate::source::Source;

/// One filing, as EDGAR published it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Filing {
    accession: Box<str>,
    form: Box<str>,
    filed: Box<str>,
    period: Option<Box<str>>,
    document: Box<str>,
    source: Source,
}

impl Filing {
    /// The accession number, which is EDGAR's name for this filing.
    pub fn accession(&self) -> &str {
        &self.accession
    }

    /// The form type: `10-K`, `8-K`, `4`, and the rest of EDGAR's vocabulary.
    pub fn form(&self) -> &str {
        &self.form
    }

    /// The date EDGAR received it, as EDGAR wrote it.
    pub fn filed(&self) -> &str {
        &self.filed
    }

    /// The date of the period reported on, where EDGAR publishes one.
    ///
    /// [`None`] where it does not. A filing that reports on no period — an
    /// ownership form, a correspondence letter — carries an empty string in
    /// the document, and that is EDGAR saying there is none rather than a value
    /// that failed to arrive.
    pub fn period(&self) -> Option<&str> {
        self.period.as_deref()
    }

    /// The primary document of the filing, named relative to the filing's own
    /// directory in EDGAR's archives.
    pub fn document(&self) -> &str {
        &self.document
    }

    /// The request this filing came from.
    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// A builder for the one place a filing is made, which is a submissions
/// document being read. Kept crate-private so that a `Filing` outside this
/// crate is always one EDGAR published.
pub(crate) struct Published<'a> {
    pub accession: &'a str,
    pub form: &'a str,
    pub filed: &'a str,
    pub period: &'a str,
    pub document: &'a str,
}

impl Published<'_> {
    pub(crate) fn into_filing(self, source: &Source) -> Filing {
        Filing {
            accession: Box::from(self.accession),
            form: Box::from(self.form),
            filed: Box::from(self.filed),
            period: match self.period {
                "" => None,
                period => Some(Box::from(period)),
            },
            document: Box::from(self.document),
            source: source.clone(),
        }
    }
}

/// Every filing EDGAR publishes for one company.
///
/// In EDGAR's order, which is newest first, and across every document EDGAR
/// splits it over — a filer with more than a page of history has the rest under
/// its own URLs, and a history that stopped at the first page would be missing
/// exactly the oldest years without saying so.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct History {
    company: Company,
    source: Source,
    filings: Vec<Filing>,
}

impl History {
    pub(crate) fn new(company: Company, source: Source, filings: Vec<Filing>) -> Self {
        Self {
            company,
            source,
            filings,
        }
    }

    /// The company these were filed by.
    pub fn company(&self) -> &Company {
        &self.company
    }

    /// The request that published the history: the document that says what this
    /// filer has filed, and how much of it is on pages of its own.
    ///
    /// Not the same as any one filing's [`Filing::source`], which is the page
    /// that filing was read from — the two agree for the first page and part
    /// company after it.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// The filings, newest first.
    pub fn filings(&self) -> &[Filing] {
        &self.filings
    }
}
