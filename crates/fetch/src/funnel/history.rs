//! The third step: what the filer has actually filed.
//!
//! The two steps before this one are about who to ask. This one asks, and it
//! asks about the filers the metadata gate admitted and about nobody else. That
//! negative is the whole of what a funnel is: the corpus is ten thousand filers,
//! most of them stop at the gate, and the requests this step does not send are
//! larger than the ones it does.
//!
//! Which filers those are is read back out of the ledger rather than carried
//! here. A filer the gate rejected is not in the answer whatever the keys handed
//! to this step say, and neither is one whose verdict did not land — so a
//! rejected filer being fetched is not something a caller has to remember to
//! prevent.
//!
//! The retrieval is [`crate::history`], which already exists, and this step adds
//! no second way to fetch one. What it adds is the boundary and the verdict.
//!
//! Three outcomes, and each of them goes on record. The history arrived, and the
//! filer goes on to the stage after this one. EDGAR names the filer and
//! publishes no filing for it, which is an answer rather than a failure: a
//! correct absence, and the filer goes no further because there is nothing there
//! to read. Or the retrieval itself failed, which says nothing about the filer —
//! recording that as a rejection is how a bad minute becomes a permanent
//! exclusion.
//!
//! The document this reads is the one the gate has already read, asked for a
//! second time. That is what the step boundary costs: one step hands the next
//! its verdicts and never a document it happens to be holding. It is paid for
//! the filers that survive the gate and for no others, which is the same
//! arithmetic the funnel is for.

use crate::company::Company;
use crate::edgar;
use crate::egress::{Egress, Transport};
use crate::filing::History;
use crate::ledger::{FilerKey, FilerLedger, Reason, Record, Ruleset, Step, Verdict};

use super::{Sweep, Unswept, admitted};

/// The version of this step's rules, and the version every verdict it records
/// carries. It changes when anything below does, so a verdict read back years
/// later says which reading of the rules made it.
const RULES: Ruleset = Ruleset::version(1);

/// EDGAR published what the filer has filed, so there is a history to hand on.
const THE_HISTORY_ARRIVED: &str = "the filings EDGAR publishes for this filer were retrieved";

/// EDGAR names the filer and publishes no filing under it. An answer, and the
/// end of the road for this filer: there is nothing for a later stage to read.
const NOTHING_IS_FILED: &str = "EDGAR publishes no filing for this filer";

/// The history did not arrive, so nothing was decided about the filer.
const NO_HISTORY_ARRIVED: &str = "the history did not arrive";

/// The filer is on record under no identifier, so there is nothing to ask EDGAR
/// about it with.
const NOTHING_TO_ASK_WITH: &str = "the filer is keyed by no identifier to ask with";

/// Retrieve a history for every filer the metadata gate admitted, and hand back
/// the verdicts that admitted the ones it got.
///
/// Both ends of this are the ledger's answer rather than a list carried in. Who
/// is asked about is who the gate admitted, read back out of `ledger`; who goes
/// on is who this step admitted, read back the same way. `judged` is the keys
/// that step judged — the question, not the answer.
///
/// `retrieved` is handed each history as it arrives, and is handed it only once
/// the verdict for that filer is on record. What a history becomes after that is
/// the caller's: what the next stage is owed is the fetch → normalize contract,
/// which is not settled here, and a step that returned every history of a pass
/// would be holding the whole corpus in memory until somebody settled it.
pub fn history<T: Transport, L: FilerLedger>(
    edgar: &mut Egress<T>,
    ledger: &mut L,
    sweep: &Sweep,
    judged: &[FilerKey],
    mut retrieved: impl FnMut(History),
) -> Result<Vec<Record>, Unswept> {
    let considering = admitted(ledger, Step::Metadata, judged)?;
    let mut evaluated = Vec::with_capacity(considering.len());

    for admitting in &considering {
        let filer = admitting.filer().clone();
        let (source, verdict, arrived) = match filer.key() {
            // The filer is the one the ledger holds: the key EDGAR answers to,
            // the name it was seen under, and the request that identified it,
            // which is the document the verdict admitting it was read out of.
            // Nothing is looked up again — a name corrected in here would file a
            // history under a filer this pass never judged.
            FilerKey::Cik(cik) => {
                let company = Company::new(
                    *cik,
                    filer.name().unwrap_or_default(),
                    admitting.source().clone(),
                );

                match edgar::history(edgar, &company) {
                    Ok(history) if history.filings().is_empty() => (
                        history.source().clone(),
                        Verdict::Rejected(Reason::new(NOTHING_IS_FILED, "filings", 0)),
                        None,
                    ),
                    Ok(history) => (
                        history.source().clone(),
                        Verdict::Admitted(Reason::new(
                            THE_HISTORY_ARRIVED,
                            "filings",
                            history.filings().len(),
                        )),
                        Some(history),
                    ),
                    Err(why) => (
                        why.request().clone(),
                        Verdict::Unjudged(Reason::new(NO_HISTORY_ARRIVED, "what came back", &why)),
                        None,
                    ),
                }
            }
            // Nothing the gate admits is keyed this way, and a ledger outlives
            // the pass that wrote it. A filer this step cannot ask about is one
            // it cannot judge, which is a verdict it has — and not a filer it
            // passes over in silence.
            FilerKey::Unresolved(listed) => (
                admitting.source().clone(),
                Verdict::Unjudged(Reason::new(NOTHING_TO_ASK_WITH, "listed as", listed)),
                None,
            ),
        };

        evaluated.push(filer.key().clone());
        ledger
            .record(Record::new(
                filer,
                Step::History,
                verdict,
                source,
                sweep.now(RULES),
            ))
            .map_err(|why| Unswept::Unkept { why })?;

        // After the record and not before it. A history handed on ahead of its
        // verdict is one the next stage holds while the ledger says nothing
        // about where it came from, which is the outcome this funnel is built
        // not to have.
        if let Some(history) = arrived {
            retrieved(history);
        }
    }

    admitted(ledger, Step::History, &evaluated)
}
