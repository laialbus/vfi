//! Normalize stage: resolves filing tags to canonical facts.
//!
//! Five modules, in the order the records fix between them.
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
//!
//! Nothing runs over a filer's history end to end through them yet. Which
//! canonical periods a filer has, and which of several answering filings sets
//! the value, are the two rules of the alignment ruleset this crate does not
//! carry, so the period is still the caller's to name.

pub mod answering;
pub mod applicability;
pub mod filings;
pub mod registry;
pub mod settling;

/// Resolves one filing's facts into the canonical facts the later stages read,
/// appending them to `out`.
///
/// Nothing resolves yet, so the filing reaches `out` as it was given. What
/// would change it is a versioned tag registry with per-company overrides —
/// data, not branching code — and that is M4's to build; a rule invented here
/// to give the stage something to do is the guess that looks right and
/// corrupts results quietly. The golden fixture beside this pins the doing of
/// nothing, so the change that lands the first mapping rule is the change that
/// states a new expected result for it.
pub fn normalize(filing: &str, out: &mut String) {
    out.push_str(filing);
}
