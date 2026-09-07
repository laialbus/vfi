//! The question asked ahead of every other: does this concept exist for the
//! accounting shape this filer presents.
//!
//! `contracts/canonical-concepts/v1.toml` publishes it as a property rather
//! than as a step — "a concept whose applies_to omits the filer's kind resolves
//! NotApplicable and is never looked up, so a lookup failure has no path to
//! NotApplicable" — and `docs/adr/candidate-choice.md` reads the same order
//! from the other side: of the four things settled before its rule runs this is
//! the first, so "a concept that reaches this rule at all is one the kind
//! admits." This is therefore not a step inside resolution. It is the gate in
//! front of it, and what it protects is the distinction the milestone is for: a
//! correct absence and a mapping that failed are never confused, because the
//! one is decided before the other can be attempted.
//!
//! It belongs to the vocabulary rather than to the registry, so it reaches
//! neither the registry nor a filing: the answer is drawn from the concept's
//! own applicability clause and from nothing else. It takes no period either.
//! The published surface makes the state "a function of the kind, so it holds
//! for every period that kind holds", and a question that accepted a period
//! would invite an answer that varied by one.
//!
//! The clause is read in one place, by the one call in `vfi-contracts` that
//! hands out a witness for a kind the clause omits. Nothing here reads it a
//! second time, and this crate holds no copy of the clauses and none of the
//! reading — a copy would agree with itself while both parted from the bytes.
//!
//! A filer whose kind has not been established proceeds on every concept.
//! `NotApplicable` claims something positive about an accounting shape, and
//! where nobody has established the shape there is no such claim to make; the
//! published surface takes it that way — every concept resolves "through the
//! attempt, to Value or Unknown, and never to a correct absence nobody
//! established" — and the witness cannot be obtained without a kind to ask a
//! clause about, so the other reading is not writable here.

use vfi_contracts::canonical_concepts::{Concept, Kind, Resolution};

/// What asking applicability answers: the concept is excluded here, or the
/// attempt proceeds.
///
/// Two answers rather than three. Whether the attempt then finds a value or
/// finds nothing is the attempt's to say, and a question that could return the
/// attempt's answer would be a question that had run one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Answer {
    /// The filer's kind is excluded from the concept, and this is the whole of
    /// what the concept resolves to. No registry is reached and no filing is
    /// read to produce it.
    ///
    /// It carries a `Resolution` rather than the kind and the clause loose,
    /// because the state is what the answer is: a caller that had to assemble
    /// it from parts would be a second place that constructs a correct absence.
    Excluded(Resolution),
    /// Nothing here excludes the concept, so what it resolves to is the
    /// attempt's answer and this module has nothing further to say about it.
    Proceeds,
}

/// Whether this filer's kind is excluded from `concept`, asked before anything
/// is looked up.
///
/// The kind arrives as an argument, as it does next door at the registry
/// interface: the engine holds no per-user state, and where a filer's asserted
/// kind is read from is settled elsewhere and does not change this answer.
///
/// `None` is a filer whose kind has not been established, and it proceeds. That
/// is the imprecise reading rather than the wrong one, which is the direction
/// this milestone takes everywhere.
pub fn ask(concept: Concept, kind: Option<Kind>) -> Answer {
    let clause = concept.definition().applies_to;

    match kind.and_then(|kind| clause.excluding(kind)) {
        Some(excluded) => Answer::Excluded(Resolution::NotApplicable { excluded }),
        None => Answer::Proceeds,
    }
}
