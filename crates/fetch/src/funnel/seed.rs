//! The first step: the filers there are, and a verdict for every entry.
//!
//! The set is EDGAR's own — the map it publishes of every listed filer — and
//! not a list kept in this repository. A committed list would be a second
//! source of truth about who exists, and the copy that goes stale is the one
//! nobody notices, because a filer missing from it looks exactly like a filer
//! that was never listed.
//!
//! Nothing is judged here beyond whether the entry names a filer at all. What
//! kind of filer it is belongs to the step after this one, which has the
//! filer's own metadata to answer from; this one only has the row.

use std::collections::HashSet;

use crate::edgar;
use crate::egress::{Egress, Transport};
use crate::ledger::{Filer, FilerKey, FilerLedger, Reason, Record, Ruleset, Step, Verdict};

use super::{Sweep, Unswept};

/// The version of this step's rules, and the version every verdict it records
/// carries. It changes when the two rules below do, so a verdict read back
/// years later says which reading of them made it.
const RULES: Ruleset = Ruleset::version(1);

/// The entry names the filer EDGAR keys it under, so there is something the
/// next step can ask about.
const NAMES_A_FILER: &str = "the entry names the filer EDGAR keys it under";

/// The entry names no filer. EDGAR assigns no filer the key zero, so a row
/// carrying one identifies nobody, and nothing later can ask EDGAR about it.
const NAMES_NO_FILER: &str = "the entry names no filer EDGAR could have keyed";

/// Put every filer EDGAR publishes on record, and hand back the keys they were
/// recorded under.
///
/// What comes back is who was considered, not who was kept. The step after this
/// one asks the ledger which of them it admitted, because a list of survivors
/// passed from hand to hand is a second answer to a question the ledger already
/// answers, and the two would eventually disagree.
///
/// A filer EDGAR lists under several tickers is one filer and gets one verdict,
/// under the first entry that named it. An entry that names no filer is its own
/// verdict, filed under whatever the entry did carry: it was evaluated, nothing
/// resolved, and this record is the only trace it was ever there.
pub fn seed<T: Transport, L: FilerLedger>(
    edgar: &mut Egress<T>,
    ledger: &mut L,
    sweep: &Sweep,
) -> Result<Vec<FilerKey>, Unswept> {
    let published = edgar::seed_set(edgar).map_err(|why| Unswept::Unpublished { why })?;
    let entries = published.entries();

    let mut considered = Vec::with_capacity(entries.len());
    let mut named = HashSet::with_capacity(entries.len());

    for entry in entries {
        let (filer, verdict) = match entry.cik() {
            Some(cik) => {
                if !named.insert(cik) {
                    continue;
                }
                (
                    Filer::keyed(cik)
                        .seen_as(entry.ticker())
                        .named(entry.title()),
                    Verdict::Admitted(Reason::new(NAMES_A_FILER, "cik", cik)),
                )
            }
            None => (
                Filer::unresolved(&entry.listed_as()),
                Verdict::Rejected(Reason::new(NAMES_NO_FILER, "cik", entry.key())),
            ),
        };

        considered.push(filer.key().clone());
        ledger
            .record(Record::new(
                filer,
                Step::Seed,
                verdict,
                published.source().clone(),
                sweep.now(RULES),
            ))
            .map_err(|why| Unswept::Unkept { why })?;
    }

    Ok(considered)
}
