//! One filer's history, in the shape `canonical-concepts` v2 publishes.
//!
//! `docs/adr/what-normalize-emits.md` fixes what the stage hands the next: "for
//! one filer: the filer, and the set of periods Rule 1 admits, each named by
//! its dates and by nothing else. At each period, every concept the vocabulary
//! publishes, exactly once, in exactly one of the three states." This puts
//! [`crate::periods`] and [`crate::standing`] together over one filer and
//! writes what they answer in the contract's types. Nothing is decided here:
//! the periods are Rule 1's, handed over in the order it hands them, and each
//! state is what [`standing::stands`] answered, translated as it stands. No
//! filing is ordered, `filed` and `form` are unread, and the registry is asked
//! nothing about a tag.
//!
//! **The kind is the registry's.** The one [`Registry::kind_of`] holds for the
//! filer, and a filer it holds none for is run with none — which Rule 1 admits
//! no period for.
//!
//! **A value crosses as the way [`SetBy`] already names.** A rule's value is
//! `read`, carrying each element it read, the filing those facts were reported
//! in — the attempt Rule 3 chose — and the rule as the pair of registry version
//! and rule id. An assertion is `asserted`, carrying that pair and the filing it
//! cites. A silence zero is `silence`, carrying its reading and the registry it
//! was found under. None carries a field of another way.
//!
//! **An `Unknown` carries every attempt under its filing's accession,** one
//! that considered no candidate included, and at a tie each tied filing's
//! attempt beside the tie, as `docs/adr/tie-and-undated-cross-into-v2.md`
//! rules.
//!
//! **An unreadable `filed` is a stop, not a state.** The same record: "a
//! filing whose `filed` does not read as one is a broken boundary reached at
//! run time … It does not cross into v2 as a state." So [`history`] fails
//! naming every such filing it met, whether Rule 1 met it deciding which
//! periods there are or the run met it at one of them, and
//! [`Undecided::Undated`] crosses into nothing.

use std::collections::BTreeSet;
use std::fmt;

use vfi_contracts::canonical_concepts::{
    self as v2, Attempted, Attempts, Concept, Resolution, SourceTag, Tie,
};
use vfi_contracts::fetch_normalize::{self, Filer};

use crate::periods;
use crate::registry::Registry;
use crate::settling::SetBy;
use crate::standing::{self, Stands, Undecided};

/// What the stage hands the next for one filer: the filer, and a row per
/// period Rule 1 admits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct History {
    filer: Box<str>,
    periods: Vec<Row>,
}

impl History {
    /// The filer, as the ten digits the fetch boundary carries.
    pub fn filer(&self) -> &str {
        &self.filer
    }

    /// Each period Rule 1 admits, once. The order means nothing, and the last
    /// is not the latest.
    pub fn periods(&self) -> &[Row] {
        &self.periods
    }
}

/// One period, named by its dates alone, and every concept at it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Row {
    period: v2::Period,
    /// One per concept, in the order the vocabulary publishes them, which is
    /// what makes every concept present exactly once.
    resolutions: Vec<Resolution>,
}

impl Row {
    pub fn period(&self) -> &v2::Period {
        &self.period
    }

    /// Every concept the vocabulary publishes, in the order it publishes them,
    /// with the state it takes here.
    pub fn concepts(&self) -> impl Iterator<Item = (Concept, &Resolution)> {
        Concept::ALL.iter().copied().zip(&self.resolutions)
    }

    /// The state `concept` takes here.
    pub fn of(&self, concept: Concept) -> &Resolution {
        let at = Concept::ALL
            .iter()
            .position(|held| *held == concept)
            .expect("every concept is a member of the set of all of them");
        &self.resolutions[at]
    }
}

/// Filings whose `filed` does not read as a date, which the boundary publishes
/// no such thing as: the run stops on them rather than crossing a state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Undated {
    accessions: Vec<Box<str>>,
}

impl Undated {
    /// Every such filing met, each once, in the order accessions sort in.
    pub fn accessions(&self) -> &[Box<str>] {
        &self.accessions
    }
}

impl fmt::Display for Undated {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let named: Vec<&str> = self.accessions.iter().map(|held| &**held).collect();
        write!(
            out,
            "filings whose filed does not read as a date: {}",
            named.join(", ")
        )
    }
}

/// `filer`'s history as `canonical-concepts` v2 publishes it, over every fact
/// fetch handed over for it and the registry.
pub fn history(registry: &Registry, filer: &Filer) -> Result<History, Undated> {
    let cik = &*filer.cik;
    let kind = registry.kind_of(cik);
    let facts = &filer.facts;

    let mut undated = BTreeSet::new();
    let admitted = periods::admitted(registry, cik, kind, facts, &mut undated);

    let mut rows = Vec::with_capacity(admitted.len());
    for period in admitted {
        let mut resolutions = Vec::with_capacity(Concept::ALL.len());
        for concept in Concept::ALL {
            let stood = standing::stands(registry, cik, kind, *concept, period, facts)
                .expect("a fact carries every period Rule 1 admits, so a filing answers it");
            match crossed(stood) {
                Ok(resolution) => resolutions.push(resolution),
                Err(accessions) => undated.extend(accessions),
            }
        }
        rows.push(Row {
            period: dated(period),
            resolutions,
        });
    }

    if !undated.is_empty() {
        return Err(Undated {
            accessions: undated.into_iter().map(Into::into).collect(),
        });
    }
    Ok(History {
        filer: filer.cik.clone(),
        periods: rows,
    })
}

/// What stands, as the state v2 publishes, or the filings whose `filed` left
/// Rule 3 nothing to order.
fn crossed<'f>(stood: Stands<'_, 'f>) -> Result<Resolution, Vec<&'f str>> {
    let unsettled = match stood {
        Stands::NotApplicable(state) => return Ok(state),
        Stands::Value(value) => {
            return Ok(Resolution::Value {
                amount: value.amount().into(),
                set_by: way(value.set_by()),
            });
        }
        Stands::Unknown(unsettled) => unsettled,
    };

    let tie = match unsettled.undecided() {
        Some(Undecided::Undated(accessions)) => return Err(accessions.clone()),
        Some(Undecided::Tie(accessions)) => Some(Tie::between(
            accessions.iter().map(|held| (*held).into()).collect(),
        )),
        None => None,
    };

    let mut each = unsettled
        .attempted()
        .iter()
        .map(|(accession, attempt)| Attempted::in_filing((*accession).into(), attempt.clone()));
    let first = each
        .next()
        .expect("an Unknown is built from an attempt that ran, and a tie from two");
    let attempted = Attempts::in_filings(first, each.collect());
    Ok(Resolution::Unknown {
        attempted: match tie {
            Some(tie) => attempted.undecided_by(tie),
            None => attempted,
        },
    })
}

/// The way a value was set, with the carriage v2 fixes for that way.
fn way(set_by: &SetBy) -> v2::SetBy {
    match set_by {
        SetBy::Rule {
            version,
            rule,
            facts,
        } => v2::SetBy::Read {
            source_tags: facts
                .iter()
                .map(|fact| SourceTag {
                    taxonomy: fact.taxonomy.clone(),
                    tag: fact.tag.clone(),
                })
                .collect(),
            filing: facts
                .first()
                .expect("every form a rule takes reads at least one element")
                .accession
                .clone(),
            rule: v2::Rule {
                registry: version.rendered().into(),
                id: rule.id().into(),
            },
        },
        SetBy::Assertion { version, assertion } => v2::SetBy::Asserted {
            rule: v2::Rule {
                registry: version.rendered().into(),
                id: assertion.rule().into(),
            },
            filing: assertion.source().accession().into(),
        },
        SetBy::Silence { reading, version } => v2::SetBy::Silence {
            reading: *reading,
            registry: version.rendered().into(),
        },
    }
}

/// A period as v2 names it: the dates, as the characters the facts carried.
fn dated(period: &fetch_normalize::Period) -> v2::Period {
    match period {
        fetch_normalize::Period::Instant { at } => v2::Period::Instant { at: at.clone() },
        fetch_normalize::Period::Duration { start, end } => v2::Period::Duration {
            start: start.clone(),
            end: end.clone(),
        },
    }
}
