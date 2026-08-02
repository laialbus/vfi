//! Normalize stage: resolves filing tags to canonical facts.

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
