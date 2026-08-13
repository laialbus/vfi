//! The second step: what a filer's own metadata says about whether this tool
//! reads it.
//!
//! One exclusion, and it is GOALS.md's, stated there under "Not in scope": "no
//! foreign private issuers filing 20-F". Nothing else here rejects anybody. A
//! rule that merely looked prudent would be indistinguishable from this one in
//! the code and nothing like it in effect: a filer this step rejects produces no
//! filings, no metrics and no row a user can ask about, so a rule invented here
//! removes companies from the tool for a reason nobody can check in December.
//! A rule GOALS.md does not support is an escalation, not a rule written to fit.
//!
//! Applying that exclusion means knowing which annual report a filer files, and
//! the metadata document says so where it names one. Three answers follow, and
//! the third is the one to be careful with: a filer whose metadata names no
//! annual report at all has not been judged and is not rejected. It is a filer
//! this step could not answer about, and it is on record as exactly that — a
//! transient failure recorded as a rejection is how a bad minute becomes a
//! permanent exclusion.
//!
//! The step costs one request per filer and stops at the document. The pages of
//! older filings that document names are not followed, so a filer judged out
//! costs nothing after its verdict: no page, no history, and nothing at all in
//! the step after this one.

use crate::edgar::{self, Metadata};
use crate::egress::{Egress, Transport};
use crate::ledger::{FilerKey, FilerLedger, Reason, Record, Ruleset, Step, Verdict};

use super::{Sweep, Unswept, admitted};

/// The version of this step's rules, and the version every verdict it records
/// carries. It changes when anything below does, so a rejection read back years
/// later says which reading of the rules made it.
const RULES: Ruleset = Ruleset::version(1);

/// The annual report a foreign private issuer files. GOALS.md puts filers of it
/// outside this tool: "no foreign private issuers filing 20-F", under "Not in
/// scope". This is the only value in this file that excludes anybody.
const FOREIGN_ANNUAL_REPORT: &str = "20-F";

/// The annual report a domestic filer files. It is the other half of the same
/// question and it excludes nobody: it is what says a filer is not the kind the
/// line above names.
const DOMESTIC_ANNUAL_REPORT: &str = "10-K";

/// How EDGAR spells an amendment: the form it amends, and this after it. An
/// amended annual report is the same annual report, so it answers the same
/// question.
const AMENDED: &str = "/A";

/// The filer files the annual report GOALS.md's scope note excludes.
const FILES_THE_FOREIGN_ANNUAL_REPORT: &str = "files its annual report as a foreign private issuer";

/// The filer files the domestic annual report, so the exclusion above is not
/// about it.
const FILES_THE_DOMESTIC_ANNUAL_REPORT: &str = "files its annual report as a domestic filer";

/// The metadata arrived and names no annual report of either kind, so there is
/// nothing here to answer the question with.
const NAMES_NO_ANNUAL_REPORT: &str = "the metadata names no annual report";

/// The metadata did not arrive, so there was nothing to read at all.
const NO_METADATA_ARRIVED: &str = "the metadata did not arrive";

/// The filer is on record under no identifier, so there is nothing to ask EDGAR
/// about it with.
const NOTHING_TO_ASK_WITH: &str = "the filer is keyed by no identifier to ask with";

/// Judge every filer the seed step admitted, and hand back the verdicts that
/// admitted the survivors.
///
/// Both ends of this are the ledger's answer rather than a list carried in.
/// Who is judged is who the seed step admitted, read back out of `ledger`; who
/// goes on is who this step admitted, read back the same way. `seeded` is the
/// keys that step considered — the question, not the answer.
pub fn gate<T: Transport, L: FilerLedger>(
    edgar: &mut Egress<T>,
    ledger: &mut L,
    sweep: &Sweep,
    seeded: &[FilerKey],
) -> Result<Vec<Record>, Unswept> {
    let considering = admitted(ledger, Step::Seed, seeded)?;
    let mut judged = Vec::with_capacity(considering.len());

    for seeded in &considering {
        let filer = seeded.filer().clone();
        let (source, verdict) = match filer.key() {
            FilerKey::Cik(cik) => match edgar::metadata(edgar, *cik) {
                Ok(metadata) => (metadata.source().clone(), judge(&metadata)),
                Err(why) => (
                    why.request().clone(),
                    Verdict::Unjudged(Reason::new(NO_METADATA_ARRIVED, "what came back", &why)),
                ),
            },
            // Nothing the seed step admits is keyed this way, and a ledger
            // outlives the pass that wrote it. A filer this step cannot ask
            // about is one it cannot judge, which is a verdict it has — and not
            // a filer it passes over in silence.
            FilerKey::Unresolved(listed) => (
                seeded.source().clone(),
                Verdict::Unjudged(Reason::new(NOTHING_TO_ASK_WITH, "listed as", listed)),
            ),
        };

        judged.push(filer.key().clone());
        ledger
            .record(Record::new(
                filer,
                Step::Metadata,
                verdict,
                source,
                sweep.now(RULES),
            ))
            .map_err(|why| Unswept::Unkept { why })?;
    }

    admitted(ledger, Step::Metadata, &judged)
}

/// What `metadata` says about the one exclusion this step applies.
///
/// The newest annual report is what answers it, rather than the presence of one
/// anywhere in the document. GOALS.md's scope note is about what a filer is, and
/// a company that filed 20-F until it lost foreign private issuer status and
/// files 10-K now is not one — while a company whose newest annual report is a
/// 20-F is, whatever it filed a decade ago.
fn judge(metadata: &Metadata) -> Verdict {
    let newest = metadata
        .listed()
        .iter()
        .filter_map(|listed| annual_report(listed.form()).map(|report| (listed, report)))
        .max_by(|(one, _), (other, _)| one.filed().cmp(other.filed()));

    let Some((listed, report)) = newest else {
        return Verdict::Unjudged(Reason::new(
            NAMES_NO_ANNUAL_REPORT,
            "filings named",
            metadata.listed().len(),
        ));
    };

    let reason = Reason::new(
        if report == FOREIGN_ANNUAL_REPORT {
            FILES_THE_FOREIGN_ANNUAL_REPORT
        } else {
            FILES_THE_DOMESTIC_ANNUAL_REPORT
        },
        "annual report",
        listed.form(),
    )
    .and("filed", listed.filed());

    if report == FOREIGN_ANNUAL_REPORT {
        Verdict::Rejected(reason)
    } else {
        Verdict::Admitted(reason)
    }
}

/// Which annual report `form` is, where it is one at all. Every other form
/// EDGAR publishes says nothing about the question this step asks.
fn annual_report(form: &str) -> Option<&'static str> {
    [DOMESTIC_ANNUAL_REPORT, FOREIGN_ANNUAL_REPORT]
        .into_iter()
        .find(|report| form == *report || form.strip_prefix(report) == Some(AMENDED))
}

#[cfg(test)]
mod tests {
    use super::{DOMESTIC_ANNUAL_REPORT, FOREIGN_ANNUAL_REPORT, annual_report};

    #[test]
    fn an_annual_report_is_that_form_or_an_amendment_of_it() {
        assert_eq!(annual_report("10-K"), Some(DOMESTIC_ANNUAL_REPORT));
        assert_eq!(annual_report("10-K/A"), Some(DOMESTIC_ANNUAL_REPORT));
        assert_eq!(annual_report("20-F"), Some(FOREIGN_ANNUAL_REPORT));
        assert_eq!(annual_report("20-F/A"), Some(FOREIGN_ANNUAL_REPORT));
    }

    /// The forms around these are not the question. `NT 10-K` says an annual
    /// report is late, `10-K405` and `20-FR12B` are other filings under
    /// neighbouring names, and none of them says which annual report this filer
    /// files — so none of them is read as saying it.
    #[test]
    fn a_form_that_is_not_one_of_them_is_not_read_as_one() {
        for form in [
            "NT 10-K", "10-KT", "10-K405", "10-Q", "20-FR12B", "F-6EF", "6-K", "40-F", "",
        ] {
            assert_eq!(
                annual_report(form),
                None,
                "{form} was read as an annual report"
            );
        }
    }
}
