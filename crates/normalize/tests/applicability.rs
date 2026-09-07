//! What asking applicability answers, over the closed sets the vocabulary
//! publishes: every concept against every kind, and every concept against a
//! filer whose kind has not been established.
//!
//! Both sets are closed and both are small, so this sweeps them whole rather
//! than picking cases out of them. A case list would be a claim about which
//! pairs matter, and nothing here knows that.
//!
//! What each answer should be is read out of the published surface — the same
//! clause the subject reads — rather than written out beside it. A table of
//! expected answers here would be a third reading of applicability, after the
//! bytes and the module `vfi-contracts` holds, and a third reading agrees with
//! itself while parting from the bytes. So what these state is the relation
//! between the answer and the clause, which a transcription cannot fake, and
//! the tie from the clause down to the published bytes is `vfi-contracts`' own,
//! made where those bytes are read.

use vfi_contracts::canonical_concepts::{Concept, Kind, Resolution};
use vfi_normalize::applicability::{self, Answer};

/// A witness for exactly the kinds a concept's clause omits, and the attempt
/// for every kind it admits — over all twenty-eight concepts and all five
/// kinds.
///
/// The two tallies are what keeps the sweep from passing vacuously: a subject
/// that answered one way for everything would still walk the whole product, and
/// the counts are how that shows up as a failure rather than as a green run
/// over a question nobody asked.
#[test]
fn every_concept_answers_as_the_clause_it_publishes_for_every_kind() {
    let mut excluded_count = 0;
    let mut proceeded_count = 0;

    for concept in Concept::ALL {
        let clause = concept.definition().applies_to;

        for kind in Kind::ALL {
            match applicability::ask(*concept, Some(*kind)) {
                Answer::Excluded(Resolution::NotApplicable { excluded }) => {
                    assert!(
                        !clause.admits(*kind),
                        "{concept:?} was excluded for {kind:?}, which its published clause admits"
                    );
                    assert_eq!(
                        excluded.kind(),
                        *kind,
                        "{concept:?} was excluded carrying a kind other than the one asked about"
                    );
                    assert_eq!(
                        excluded.clause(),
                        clause,
                        "{concept:?} was excluded carrying a clause other than its published one"
                    );
                    excluded_count += 1;
                }
                Answer::Excluded(other) => panic!(
                    "{concept:?} against {kind:?} answered with a state that is not a correct \
                     absence: {other:?}"
                ),
                Answer::Proceeds => {
                    assert!(
                        clause.admits(*kind),
                        "{concept:?} proceeded for {kind:?}, which its published clause omits"
                    );
                    proceeded_count += 1;
                }
            }
        }
    }

    assert!(
        excluded_count > 0 && proceeded_count > 0,
        "the published vocabulary states both a concept some kind is excluded from and a concept \
         some kind is admitted to, and this sweep saw {excluded_count} exclusions and \
         {proceeded_count} attempts"
    );
}

/// The kindless filer, written down rather than left to the absence of a
/// branch.
///
/// A filer with no file in the registry has no kind, and the reading to guard
/// against is "no kind admits it". The published surface reads it the other
/// way, because the claim `NotApplicable` makes is positive and nobody has made
/// it here.
#[test]
fn a_filer_whose_kind_is_not_established_proceeds_on_every_concept() {
    for concept in Concept::ALL {
        assert_eq!(
            applicability::ask(*concept, None),
            Answer::Proceeds,
            "{concept:?} did not proceed for a filer whose kind has not been established"
        );
    }
}

/// The kinds a concept excludes are the kinds it excludes whoever is asking,
/// and asking without a kind is not asking about one of them.
///
/// This is the pair above put side by side, over the concepts that have a kind
/// to be excluded at all: the same concept answers the correct absence for an
/// excluded kind and the attempt for no kind. Read apart, the two tests above
/// could both hold of a subject that answered by concept alone.
#[test]
fn no_kind_is_not_an_excluded_kind() {
    let mut compared = 0;

    for concept in Concept::ALL {
        let clause = concept.definition().applies_to;
        let Some(omitted) = Kind::ALL.iter().find(|kind| !clause.admits(**kind)) else {
            continue;
        };

        assert!(
            matches!(
                applicability::ask(*concept, Some(*omitted)),
                Answer::Excluded(_)
            ),
            "{concept:?} did not exclude {omitted:?}, which its published clause omits"
        );
        assert_eq!(
            applicability::ask(*concept, None),
            Answer::Proceeds,
            "{concept:?} excludes {omitted:?} and answered the same way for no kind at all"
        );
        compared += 1;
    }

    assert!(
        compared > 0,
        "no published concept omits a kind, so this compares nothing"
    );
}
