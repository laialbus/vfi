//! Normalize stage: resolves one filer's facts into its canonical history.
//!
//! [`normalize`] is the stage. Given one filer's facts as the fetch boundary
//! publishes them and a registry, it writes the filer's history as
//! `canonical-concepts` v2 publishes it — the filer, the periods Rule 1 admits,
//! and every concept at each — in the spelling [`rendering`] fixes. The golden
//! fixtures and the benchmark measure it there.
//!
//! Nine modules, in the order the records fix between them.
//! [`applicability`] is asked first, of the published clauses alone, and a
//! concept the filer's kind excludes stops there. [`registry`] is the one way
//! to the tag mapping, which is data rather than code, and a concept reaches it
//! only having got past the question above. [`answering`] takes one rule the
//! registry handed back and the facts of one filing, and says which of them
//! answer the period asked for — which is the first thing the procedure that
//! chooses among candidates asks of each rule. [`settling`] is that procedure:
//! it asks the three above, in that order, and turns what they say into one of
//! the three states the vocabulary publishes. [`filings`] is the first module
//! over more than one filing: given a filer's facts it says which of that
//! filer's filings answer a period, and runs the procedure inside each of them.
//! [`standing`] weighs those attempts: the silence reading over the period, and
//! then Rule 3, which of several answering filings sets the value. [`periods`]
//! is Rule 1, which canonical periods a filer has, asked of what stands.
//! [`history`] runs the two over one filer's history end to end and hands over
//! what `canonical-concepts` v2 publishes: the filer, its periods, and every
//! concept at each. [`rendering`] writes that out.

pub mod answering;
pub mod applicability;
pub mod filings;
pub mod history;
pub mod periods;
pub mod registry;
pub mod rendering;
pub mod settling;
pub mod standing;

use vfi_contracts::fetch_normalize::Filer;

use history::Undated;
use registry::Registry;

/// Writes `filer`'s history under `registry` onto `out`, as [`rendering`]
/// spells it, or stops on the filings whose `filed` does not read as a date and
/// writes nothing.
///
/// The benchmark's proof of catch slows the first public function this file
/// states on one line, which is this one, so its signature stays on one line.
pub fn normalize(registry: &Registry, filer: &Filer, out: &mut String) -> Result<(), Undated> {
    let history = history::history(registry, filer)?;
    rendering::history(&history, registry.version(), out);
    Ok(())
}
