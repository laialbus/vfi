//! What the source publishes about being accessed, and the one place this crate
//! says it.
//!
//! Nothing here is this crate's decision. Both facts are read off the page EDGAR
//! publishes them on —
//! <https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data>
//! — because a rate that is nearly right reads exactly like one that is right
//! until the source starts refusing, and by then the run that guessed is long
//! over.

use std::fmt;
use std::time::Duration;

/// The maximum access rate EDGAR publishes: ten requests a second, "regardless
/// of the number of machines used to submit requests".
///
/// That clause is the whole reason [`crate::Pace`] is one shared thing rather
/// than one per caller. The source counts what arrives at it, not who sent it,
/// so a bound each caller keeps for itself is ten bounds and ten times the rate.
pub const MAX_REQUESTS_PER_SECOND: u32 = 10;

/// The least time between two requests leaving, which is the rate above said
/// the other way round. Derived rather than written down beside it: two
/// spellings of one number is how the two come apart.
pub const MINIMUM_SPACING: Duration = {
    let rate = MAX_REQUESTS_PER_SECOND as u64;
    let second = Duration::from_secs(1).as_nanos() as u64;
    // Rounded up where the division does not come out even. A spacing rounded
    // down is a rate above the published one, and slower is the direction to be
    // wrong in.
    Duration::from_nanos(second.div_ceil(rate))
};

/// The header a client declares itself in, spelled once.
pub const DECLARATION_HEADER: &str = "User-Agent";

/// Who is asking, as the source requires it be told.
///
/// EDGAR asks a client to "declare your traffic by updating your user agent to
/// include company specific information". Company specific information is the
/// user's and not the engine's — it names them and carries a way to reach them
/// — so it arrives as a parameter and is never a literal here.
///
/// What this type adds is that the two values which would be a declaration in
/// name only cannot be made into one. Both are refused where they are given
/// rather than where they are sent, because the answer to either is a 403 from
/// the source, which reads like a filing that is not there.
#[derive(Clone, Debug)]
pub struct Declaration(String);

impl Declaration {
    /// Take `who` as the declaration every request will carry, if it is one.
    pub fn new(who: &str) -> Result<Self, Undeclared> {
        if who.trim().is_empty() {
            return Err(Undeclared {
                why: "it names nobody, and an undeclared tool is what this exists to avoid being",
            });
        }
        if !who.is_ascii() {
            return Err(Undeclared {
                why: "a header value outside ASCII is one no two clients agree on the bytes of",
            });
        }
        if who.bytes().any(|byte| byte.is_ascii_control()) {
            return Err(Undeclared {
                why: "a control character in a header value is where a second header would begin",
            });
        }

        Ok(Self(who.to_owned()))
    }

    /// The declaration, as the user wrote it.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Why a declaration was refused. Nothing was sent under it, because an
/// [`crate::Egress`] is not built without one.
#[derive(Debug)]
pub struct Undeclared {
    /// What was wrong with it.
    pub why: &'static str,
}

impl fmt::Display for Undeclared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a declaration to send, because {}", self.why)
    }
}

impl std::error::Error for Undeclared {}

#[cfg(test)]
mod tests {
    use super::{Declaration, MAX_REQUESTS_PER_SECOND, MINIMUM_SPACING};
    use std::time::Duration;

    /// The spacing against the rate it comes from, in the terms the source
    /// publishes: whatever the rate is set to, that many requests spaced this
    /// far apart take at least a second.
    #[test]
    fn the_spacing_is_the_published_rate_said_the_other_way() {
        assert!(
            MINIMUM_SPACING * MAX_REQUESTS_PER_SECOND >= Duration::from_secs(1),
            "{MAX_REQUESTS_PER_SECOND} requests {MINIMUM_SPACING:?} apart fit inside a second"
        );
    }

    #[test]
    fn a_declaration_is_carried_as_it_was_given() {
        let who = "VFI vfi@example.invalid";

        assert_eq!(Declaration::new(who).expect("names somebody").as_str(), who);
    }

    /// Each of these would be sent as a declaration and is one in name only.
    /// The first two name nobody. The third is two headers, the second of them
    /// written by whoever supplied the value rather than by this crate. The last
    /// is bytes the transport underneath decides the meaning of.
    #[test]
    fn what_would_not_declare_anybody_is_not_a_declaration() {
        for who in [
            "",
            "   ",
            "VFI\r\nX-Forwarded-For: 10.0.0.1",
            "VFI Café vfi@example.invalid",
        ] {
            assert!(
                Declaration::new(who).is_err(),
                "{who:?} was taken as a declaration"
            );
        }
    }
}
