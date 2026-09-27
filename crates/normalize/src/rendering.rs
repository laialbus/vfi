//! One filer's history, written out as a normalize golden fixture's `expected`
//! holds it.
//!
//! `docs/adr/what-normalize-emits.md` fixes the content, under "What a normalize
//! fixture's `expected` renders", and leaves the spelling to the task that
//! writes the first fixture. This is that spelling, and nothing in it is read
//! back: a line is for a reader deciding whether the stage is right.
//!
//! ```text
//! filer <cik>
//! at <date>
//! from <start> to <end>
//!   <concept> Value <amount> read <taxonomy>:<tag>… in <accession> rule <version>|<rule id>
//!   <concept> Value <amount> asserted rule <version>|<rule id> citing <accession>
//!   <concept> Value <amount> silence <reading> under <version>
//!   <concept> NotApplicable kind <kind> admits <kind>…
//!   <concept> Unknown
//!     in <accession>
//!       <version>|<rule id> declined: <reason>
//!     in <accession> declining nothing
//!     tie <accession>…
//! ```
//!
//! **The periods go in the order of their dates' characters,** the first date
//! and then the second, an instant before a duration starting that day. The
//! set has no order of its own, so this is the fixture's and reads nothing but
//! the dates, as the record asks.
//!
//! **Every concept, in the order the vocabulary publishes them,** one to a
//! line, naming its state as v2 names it. A `Value` names its way as a word,
//! `read`, `asserted` or `silence`, so a silence zero and a read zero differ on
//! their line even where the amounts agree.
//!
//! **An `Unknown` is its attempts, one filing to a line,** each followed by the
//! candidates it declined and the reason each was declined, and then the tie
//! where Rule 3 left one. An attempt that declined nothing says so rather than
//! leaving the filing's line bare.
//!
//! **A registry version is the word [`REGISTRY`] wherever it is the one the
//! history is rendered under, and its sixty-four characters wherever it is
//! not,** candidate names included. A registry edit then leaves `expected`
//! alone, and a value naming another registry still reads differently.

use std::cmp::Ordering;

use vfi_contracts::canonical_concepts::{
    Attempted, Concept, Excluded, Period, Resolution, Rule, SetBy,
};

use crate::history::{History, Row};
use crate::registry::{Version, spelled};
use crate::settling::PAIR;

/// What a registry version renders as where it is the version the history is
/// rendered under.
pub const REGISTRY: &str = "registry";

/// `history`, written onto `out`, with `under` as the registry it was read
/// under.
pub fn history(history: &History, under: Version, out: &mut String) {
    let under = under.rendered();
    let concepts: Vec<String> = Concept::ALL
        .iter()
        .map(|concept| spelled(&format!("{concept:?}")))
        .collect();

    out.push_str("filer ");
    out.push_str(history.filer());
    out.push('\n');

    let mut rows: Vec<&Row> = history.periods().iter().collect();
    rows.sort_by(|one, other| by_dates(one.period(), other.period()));

    for row in rows {
        period(row.period(), out);
        for ((_, resolution), concept) in row.concepts().zip(&concepts) {
            out.push_str("  ");
            out.push_str(concept);
            out.push(' ');
            state(resolution, &under, out);
        }
    }
}

fn by_dates(one: &Period, other: &Period) -> Ordering {
    dates(one).cmp(&dates(other))
}

fn dates(period: &Period) -> (&str, Option<&str>) {
    match period {
        Period::Instant { at } => (at, None),
        Period::Duration { start, end } => (start, Some(end)),
    }
}

fn period(period: &Period, out: &mut String) {
    match period {
        Period::Instant { at } => {
            out.push_str("at ");
            out.push_str(at);
        }
        Period::Duration { start, end } => {
            out.push_str("from ");
            out.push_str(start);
            out.push_str(" to ");
            out.push_str(end);
        }
    }
    out.push('\n');
}

fn state(resolution: &Resolution, under: &str, out: &mut String) {
    match resolution {
        Resolution::Value { amount, set_by } => {
            out.push_str("Value ");
            out.push_str(amount);
            out.push(' ');
            way(set_by, under, out);
            out.push('\n');
        }
        Resolution::NotApplicable { excluded } => not_applicable(*excluded, out),
        Resolution::Unknown { attempted } => {
            out.push_str("Unknown\n");
            for each in attempted.in_each_filing() {
                attempt(each, under, out);
            }
            if let Some(tie) = attempted.tie() {
                out.push_str("    tie");
                for accession in tie.accessions() {
                    out.push(' ');
                    out.push_str(accession);
                }
                out.push('\n');
            }
        }
    }
}

fn way(set_by: &SetBy, under: &str, out: &mut String) {
    match set_by {
        SetBy::Read {
            source_tags,
            filing,
            rule,
        } => {
            out.push_str("read");
            for source in source_tags {
                out.push(' ');
                out.push_str(&source.taxonomy);
                out.push(':');
                out.push_str(&source.tag);
            }
            out.push_str(" in ");
            out.push_str(filing);
            out.push_str(" rule ");
            pair(rule, under, out);
        }
        SetBy::Asserted { rule, filing } => {
            out.push_str("asserted rule ");
            pair(rule, under, out);
            out.push_str(" citing ");
            out.push_str(filing);
        }
        SetBy::Silence { reading, registry } => {
            out.push_str("silence ");
            out.push_str(&spelled(&format!("{reading:?}")));
            out.push_str(" under ");
            version(registry, under, out);
        }
    }
}

fn not_applicable(excluded: Excluded, out: &mut String) {
    out.push_str("NotApplicable kind ");
    out.push_str(&spelled(&format!("{:?}", excluded.kind())));
    out.push_str(" admits");
    for kind in excluded.clause().kinds() {
        out.push(' ');
        out.push_str(&spelled(&format!("{kind:?}")));
    }
    out.push('\n');
}

fn attempt(attempted: &Attempted, under: &str, out: &mut String) {
    out.push_str("    in ");
    out.push_str(attempted.accession());

    let declined = attempted.attempt().declined();
    if declined.is_empty() {
        out.push_str(" declining nothing\n");
        return;
    }
    out.push('\n');
    for each in declined {
        out.push_str("      ");
        versioned(&each.candidate, under, out);
        out.push_str(" declined: ");
        versioned(&each.rule, under, out);
        out.push('\n');
    }
}

fn pair(rule: &Rule, under: &str, out: &mut String) {
    version(&rule.registry, under, out);
    out.push(PAIR);
    out.push_str(&rule.id);
}

fn version(held: &str, under: &str, out: &mut String) {
    out.push_str(if held == under { REGISTRY } else { held });
}

/// `text` with every mention of the registry it is rendered under written as
/// the word. A candidate is named by the pair of registry version and rule id,
/// and a reason can name other candidates the same way.
fn versioned(text: &str, under: &str, out: &mut String) {
    let mut pieces = text.split(under);
    if let Some(first) = pieces.next() {
        out.push_str(first);
    }
    for piece in pieces {
        out.push_str(REGISTRY);
        out.push_str(piece);
    }
}
