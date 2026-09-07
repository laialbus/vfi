//! Normalize stage: resolves filing tags to canonical facts.
//!
//! Two modules, in the order the vocabulary fixes between them.
//! [`applicability`] is asked first, of the published clauses alone, and a
//! concept the filer's kind excludes stops there. [`registry`] is the one way
//! to the tag mapping, which is data rather than code, and a concept reaches it
//! only having got past the question above.
//!
//! Nothing resolves through either yet: they are the doors the rule that
//! chooses among several candidates in one filing, and the ruleset that
//! reconciles amendments, restatements and periods, will knock on.

pub mod applicability;
pub mod registry;

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
