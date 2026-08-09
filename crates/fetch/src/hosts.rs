//! Which hosts a fetch may reach. Only that: how a request gets to one is the
//! chokepoint's business, and this file has no way to send anything.

/// Every host the fetch stage may contact.
///
/// This is the list, and it is the only one. A second copy anywhere would be a
/// second answer to "may we reach this", and the copy that drifts is the one
/// nobody reads.
///
/// Entries are whole host names, matched entire. Not suffixes: a rule that
/// allowed anything ending in `sec.gov` would allow `sec.gov.example.com`,
/// which belongs to whoever registered `example.com`.
///
/// The market-data provider M3 also brings is not here. It arrives with the
/// task that fetches from it — a host listed before anything asks for it is
/// reach nobody has a use for.
pub const ALLOWED_HOSTS: &[&str] = &[
    // EDGAR itself: the filing archives, and the browse and search endpoints.
    "www.sec.gov",
    // EDGAR's JSON: company submissions, company facts, and the frames.
    "data.sec.gov",
];

/// Whether the list allows `host`.
///
/// ASCII case-insensitive, because DNS is: `WWW.SEC.GOV` names the same host as
/// the entry above, and a comparison that missed that would withhold a host the
/// list allows — the safe direction to be wrong in, and still wrong.
pub(crate) fn allows(host: &str) -> bool {
    ALLOWED_HOSTS
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(host))
}

#[cfg(test)]
mod tests {
    use super::{ALLOWED_HOSTS, allows};

    #[test]
    fn every_listed_host_is_allowed() {
        assert!(!ALLOWED_HOSTS.is_empty(), "the list allows nothing at all");
        for host in ALLOWED_HOSTS {
            assert!(allows(host), "{host} is on the list and is not allowed");
        }
    }

    #[test]
    fn case_is_not_what_makes_a_host_a_different_host() {
        for host in ALLOWED_HOSTS {
            assert!(
                allows(&host.to_uppercase()),
                "{host} shouted is the same host"
            );
        }
    }

    #[test]
    fn a_host_the_list_does_not_name_is_not_allowed() {
        assert!(!allows("filings.example.com"));
        assert!(!allows(""));
    }

    /// The two shapes a suffix rule and a prefix rule would each wave through.
    /// Neither is the listed host; both are one registration away from being
    /// somebody else's.
    #[test]
    fn a_host_that_merely_contains_a_listed_one_is_not_allowed() {
        for host in ALLOWED_HOSTS {
            assert!(!allows(&format!("{host}.example.com")));
            assert!(!allows(&format!("example-{host}")));
        }
    }
}
