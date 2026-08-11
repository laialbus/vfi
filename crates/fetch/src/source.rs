//! Where a record came from.
//!
//! Every record this stage produces carries one of these, so a later stage can
//! say which request produced a value without asking the fetcher again. That is
//! the whole of what provenance is here: the request, not a summary of it, and
//! not a note about it written somewhere else.

use std::fmt;
use std::sync::Arc;

/// The request a record came from.
///
/// One request answers with many records — a submissions document publishes a
/// thousand filings — so this is shared rather than copied per record. That is
/// the only reason for the `Arc`: the URL is the whole of the value, and two
/// records from one request hold the same one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source(Arc<str>);

impl Source {
    pub(crate) fn new(url: &str) -> Self {
        Self(Arc::from(url))
    }

    /// The URL that was requested, as it was requested.
    pub fn url(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
