//! What one concept settles to, over one filing at one period.
//!
//! This is `docs/adr/candidate-choice.md`'s procedure and nothing else. It is
//! asked one question — given a filer and its kind, a concept, one period, and
//! the facts of one filing, what does the concept take? — and it answers with
//! one of the three states the vocabulary publishes.
//!
//! Four things are settled before it runs, each by a record that owns them, and
//! each already landed next door. Applicability is asked first, by
//! [`crate::applicability`], and a concept the kind excludes stops there — which
//! is why nothing below can produce a correct absence, and why nothing below
//! needs to. An assertion is total, by the registry: where a filer's file
//! asserts the concept over the period, that is the value and no rule is looked
//! up beneath it. The eligible rules are the registry's answer, reached through
//! its one interface, already reduced by kind scope and by the filer's own
//! `include` and `exclude`. And which of the filing's facts answer the period is
//! [`crate::answering`], asked of one rule at a time.
//!
//! ## The four steps
//!
//! 1. **Candidates.** An eligible rule is a candidate when every fact it needs
//!    answers the period: one fact for a `tag`, every operand for a `sum`, both
//!    terms for a `difference`. A composition missing an operand is not a
//!    candidate, because reading a missing operand as zero is reading silence as
//!    zero for that component, which the vocabulary's own silence test refused
//!    for the concepts it matters to. A `difference` whose concept operand did
//!    not itself resolve to a value is not a candidate either; that operand is
//!    resolved by this same procedure, and the registry gate has already refused
//!    a cycle among those edges, so nothing here re-walks them.
//! 2. **Exact before stand-in.** If any candidate reads the concept exactly, the
//!    candidates that only stand in for it are dropped.
//! 3. **The whole before its part.** Among what survives, a candidate whose
//!    facts are strictly contained in another survivor's is dropped. A component
//!    never competes with a composition that contains it.
//! 4. **Settle.** Exactly one survivor is the value. More than one is `Unknown`,
//!    carrying every candidate and the reason each was dropped or left
//!    undecided — including two survivors that agree, because a value records
//!    the rule that set it, singular, and a rule picked among equals is a choice
//!    nothing could replay. None is the silence reading the vocabulary publishes
//!    for the concept.
//!
//! No step is skipped and nothing is added between them. There is no tie-break:
//! not the larger, not the more common, not the one that agrees with a
//! neighbouring period. Each of those is a plausible pick, and every one of them
//! is recoverable by a per-filer `exclude` or `assert` — a file with a rule id
//! behind it, which is the difference between a choice someone signed and a
//! choice a function made quietly.
//!
//! ## A candidate is a rule together with the facts it read
//!
//! Ordinarily that is one candidate per rule, because the boundary's five-field
//! identity makes one element, unit, period and filing one fact. Where two facts
//! do share those five, or where one entry answers one period under two currency
//! keys, the rule reads two figures and there are two candidates, which step 4
//! leaves undecided: nothing crossing the boundary says which of them is the
//! undimensioned one or which currency the filing reports in, so nothing here
//! prefers one. Step 3's own words are why the shape is this way — "a candidate
//! whose facts are strictly contained in another survivor's facts" is a sentence
//! about candidates that have facts.
//!
//! ## What is written here, and what is data
//!
//! The procedure carries no per-filer branch and no table of concepts and
//! favoured tags. Which candidate reads the concept exactly is the registry's
//! `reading`; which measure, unit and silence reading a concept carries is the
//! published vocabulary's. The one place a concept is named in this source is
//! [`conditioned_on`], where the vocabulary publishes its condition as prose
//! naming another concept and there is nothing mechanical beside it to read the
//! pairing off. Nothing checks that this holds; it is written here so that a
//! later reader can see it broken.

mod figure;

use vfi_contracts::canonical_concepts::{Attempt, Concept, Declined, Kind, Resolution, Silence};
use vfi_contracts::fetch_normalize::{Fact, Period};

use crate::answering::Admits;
use crate::registry::{Assertion, Form, Operand, Outcome, Reading, Registry, Rule, Version};
use crate::{answering, applicability};
use figure::Figure;

/// The amount a silence the vocabulary reads as zero takes. The vocabulary's
/// number rather than this procedure's, and written once.
const ZERO: &str = "0";

/// What separates the two halves of a rule's name. The version is sixty-four
/// hexadecimal characters, so the pair reads apart wherever it is printed.
const PAIR: char = '|';

/// What one concept settles to: one of the three states the vocabulary
/// publishes, never two of them and never none.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Settled<'r, 'f> {
    /// The filer's kind is excluded from the concept, as [`crate::applicability`]
    /// answered before anything was looked up. It travels as that module built
    /// it, because a state assembled a second time is a second place a correct
    /// absence is constructed.
    NotApplicable(Resolution),
    /// The concept resolved to an amount.
    Value(Value<'r, 'f>),
    /// An attempt ran and returned nothing, carrying what it tried.
    Unknown(Attempt),
}

/// A resolved amount, and what set it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value<'r, 'f> {
    amount: Box<str>,
    set_by: SetBy<'r, 'f>,
}

/// What set a value, which is not one thing.
///
/// The vocabulary asks a value for the source tag it was read from, the filing
/// it was reported in and the rule that set it, and only a value a rule read out
/// of a filing's facts has all three. An asserted value has a rule and a cited
/// filing and no tag; a value the vocabulary's silence reading supplies has none
/// of the three. Keeping them apart is what stops this module writing an empty
/// tag onto a value that never had one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SetBy<'r, 'f> {
    /// One rule of the registry, over the facts of this filing that answered it.
    ///
    /// The facts are the ones the rule read directly: one for a `tag`, one per
    /// element for a `sum`, and the element term of a `difference`. A
    /// difference's concept operand is a resolved concept rather than a fact,
    /// and the rule id names it, so replaying the value under the same version
    /// walks to it again rather than reading a copy kept here.
    Rule {
        version: Version,
        rule: &'r Rule,
        facts: Vec<&'f Fact>,
    },
    /// An override stating this concept outright for this filer and this period.
    Assertion {
        version: Version,
        assertion: &'r Assertion,
    },
    /// Nothing matched, and the vocabulary reads this concept's silence as a
    /// zero — outright, or on the condition it publishes with the concept.
    Silence { reading: Silence },
}

impl<'r, 'f> Value<'r, 'f> {
    /// The amount, as a decimal literal. A `tag` and an assertion hand theirs
    /// over as the characters they were published as; a composition renders the
    /// figure it composed, at the scale of whichever operand stated the most
    /// decimal places.
    pub fn amount(&self) -> &str {
        &self.amount
    }

    pub fn set_by(&self) -> &SetBy<'r, 'f> {
        &self.set_by
    }
}

impl SetBy<'_, '_> {
    /// The rule that set the value, as the pair of registry version and rule id
    /// and never the id alone — one id under two versions may name different
    /// bytes.
    ///
    /// `None` where the vocabulary's silence reading set it, there being no rule
    /// under a silence and no id to render.
    pub fn rule(&self) -> Option<String> {
        match self {
            SetBy::Rule { version, rule, .. } => Some(named(*version, rule.id())),
            SetBy::Assertion { version, assertion } => Some(named(*version, assertion.rule())),
            SetBy::Silence { .. } => None,
        }
    }
}

/// A rule named as a resolved value records it: the registry version and the
/// rule id, together.
///
/// One rendering, used wherever a rule is named — on a value, and on every
/// candidate an attempt reports — so that the two cannot drift into two
/// spellings of one thing.
pub fn named(version: Version, id: &str) -> String {
    format!("{version}{PAIR}{id}")
}

/// The question, carried whole.
///
/// Every step below is asked about the same things, and a difference's concept
/// operand is the same question with one of them changed. Carrying them together
/// is what makes that one line rather than a row of arguments threaded through
/// every step.
#[derive(Clone, Copy)]
struct Asked<'a, 'r, 'f> {
    registry: &'r Registry,
    filer: &'a str,
    kind: Option<Kind>,
    concept: Concept,
    period: &'a Period,
    filing: &'a [&'f Fact],
    admits: Admits,
    /// Whether this concept is being settled in the course of answering a
    /// conditional silence reading's condition.
    ///
    /// A condition names one other concept. That concept's own rules may name a
    /// third by a `difference`, and the registry gate has refused a cycle among
    /// those edges — but a difference edge that walked back to a concept whose
    /// condition asked the question in the first place is a walk with no first
    /// step, and it is not an edge that gate is over. So the second condition on
    /// one walk is refused rather than asked, which costs the zero and never the
    /// other direction.
    conditioned: bool,
}

/// What `concept` settles to for this filer over `period`, out of `filing`'s
/// facts, with that filing asked under `admits`.
///
/// `filing` is the facts of one filing. Which filings answer a period, which
/// entries each of them is asked with, and what a restatement or an amendment
/// does to one already settled, belong to the alignment ruleset: a composition
/// drawn across two filings is a number no filing states, and nothing reaching
/// this far could tell one from a number that is stated.
///
/// The kind arrives as an argument rather than being read from the registry
/// here, as it does at every other door in this crate: the engine holds no
/// per-user state, and every operation takes its inputs explicitly.
pub fn settle<'r, 'f>(
    registry: &'r Registry,
    filer: &str,
    kind: Option<Kind>,
    concept: Concept,
    period: &Period,
    filing: &[&'f Fact],
    admits: Admits,
) -> Settled<'r, 'f> {
    asked(Asked {
        registry,
        filer,
        kind,
        concept,
        period,
        filing,
        admits,
        conditioned: false,
    })
}

fn asked<'r, 'f>(question: Asked<'_, 'r, 'f>) -> Settled<'r, 'f> {
    if let applicability::Answer::Excluded(state) =
        applicability::ask(question.concept, question.kind)
    {
        return Settled::NotApplicable(state);
    }

    match matched(question) {
        Matched::Value(value) => Settled::Value(value),
        Matched::Undecided(declined) => Settled::Unknown(Attempt::that_ran(declined)),
        Matched::Nothing(declined) => silent(question, declined),
    }
}

/// What the four steps came to, before the silence reading is asked for.
///
/// The separation is what lets a conditional silence reading ask whether the
/// concept it names is silent without asking that concept its own silence
/// reading, which names this one back. A question about facts terminates; two
/// conditions reading each other would not.
enum Matched<'r, 'f> {
    Value(Value<'r, 'f>),
    /// More than one survivor, and every candidate with its reason.
    Undecided(Vec<Declined>),
    /// No candidate at all, and every eligible rule with the reason it was not
    /// one. This is what "the period's facts carry nothing this concept resolves
    /// from" is, read over one filing.
    Nothing(Vec<Declined>),
}

fn matched<'r, 'f>(question: Asked<'_, 'r, 'f>) -> Matched<'r, 'f> {
    let answer = question.registry.answer(
        question.filer,
        question.kind,
        question.concept,
        question.period,
    );
    let version = answer.version;

    let eligible = match answer.outcome {
        Outcome::Asserted(assertion) => {
            return Matched::Value(Value {
                amount: assertion.value().into(),
                set_by: SetBy::Assertion { version, assertion },
            });
        }
        Outcome::Eligible(eligible) => eligible,
    };

    let mut candidates = Vec::new();
    let mut declined = Vec::new();
    for rule in eligible {
        candidacy(question, rule, version, &mut candidates, &mut declined);
    }

    if candidates.is_empty() {
        return Matched::Nothing(declined);
    }

    exact_before_stand_in(&mut candidates, version, &mut declined);
    the_whole_before_its_part(&mut candidates, version, &mut declined);

    if candidates.len() == 1 {
        let held = candidates.remove(0);
        return Matched::Value(Value {
            amount: held.amount,
            set_by: SetBy::Rule {
                version,
                rule: held.rule,
                facts: held.facts,
            },
        });
    }

    for (at, candidate) in candidates.iter().enumerate() {
        let others = candidates
            .iter()
            .enumerate()
            .filter(|(other, _)| *other != at)
            .map(|(_, other)| other.named(version))
            .collect();
        declined.push(Declined {
            candidate: candidate.named(version).into(),
            rule: Because::UndecidedAmong(others).stated().into(),
        });
    }
    Matched::Undecided(declined)
}

/// One rule, and the facts of this filing it read.
///
/// The amount is composed as the candidate is made rather than once it has won,
/// so that a rule whose figures cannot be composed is never a survivor whose
/// value cannot be stated.
struct Candidate<'r, 'f> {
    rule: &'r Rule,
    facts: Vec<&'f Fact>,
    amount: Box<str>,
    /// Whether more than one candidate came off this one rule, which happens
    /// only where the boundary's five-field identity did not hold, or where one
    /// entry answered one period under two currency keys. Naming a candidate by
    /// its rule tells two of those apart not at all, so where this is set the
    /// name carries the facts as well.
    contested: bool,
}

impl Candidate<'_, '_> {
    fn named(&self, version: Version) -> String {
        let held = named(version, self.rule.id());
        match self.contested {
            false => held,
            true => {
                let read: Vec<String> = self.facts.iter().map(|fact| rendered(fact)).collect();
                format!("{held} reading {}", read.join(" with "))
            }
        }
    }
}

/// Step 1, for one rule: every fact it needs, or the reason it has none.
fn candidacy<'r, 'f>(
    question: Asked<'_, 'r, 'f>,
    rule: &'r Rule,
    version: Version,
    candidates: &mut Vec<Candidate<'r, 'f>>,
    declined: &mut Vec<Declined>,
) {
    let composed_from = match operand_concept(rule) {
        None => None,
        Some(operand) => match amount_of(question, operand) {
            Some(figure) => Some(figure),
            // Not "the operand is zero", and not "the difference is the term it
            // does have": a difference of a concept that did not resolve is a
            // number nothing states.
            None => {
                declined.push(Declined {
                    candidate: named(version, rule.id()).into(),
                    rule: Because::OperandConceptUnresolved.stated().into(),
                });
                return;
            }
        },
    };

    let answered = answering::ask(
        question.concept,
        rule,
        question.period,
        question.filing,
        question.admits,
    );
    let mut read: Vec<Vec<(&'f Fact, Figure)>> = vec![Vec::new()];

    for operand in answered.operands() {
        // A fact whose published amount is not a decimal literal states no
        // figure, so it answers a question that asks for one not at all. The
        // boundary does not publish such a literal; where one arrives, this is
        // the same direction the record takes for a broken boundary reached at
        // run time — an absence, never a reading invented for it.
        let figures: Vec<(&'f Fact, Figure)> = operand
            .facts()
            .iter()
            .filter_map(|fact| Figure::read(&fact.value).map(|figure| (*fact, figure)))
            .collect();

        if figures.is_empty() {
            declined.push(Declined {
                candidate: named(version, rule.id()).into(),
                rule: match rule.form() {
                    Form::Tag => Because::NoFact.stated().into(),
                    Form::Sum | Form::Difference => Because::OperandUnanswered(operand.operand())
                        .stated()
                        .into(),
                },
            });
            return;
        }

        let mut grown = Vec::with_capacity(read.len() * figures.len());
        for held in &read {
            for figure in &figures {
                let mut one = held.clone();
                one.push(figure.clone());
                grown.push(one);
            }
        }
        read = grown;
    }

    let contested = read.len() > 1;
    for held in read {
        candidates.push(Candidate {
            rule,
            amount: compose(rule.form(), composed_from.as_ref(), &held)
                .rendered()
                .into(),
            facts: held.into_iter().map(|(fact, _)| fact).collect(),
            contested,
        });
    }
}

/// What a rule's form makes of what it read.
///
/// A `tag` is its one figure. A `sum` is its operands added, in the order the
/// entry states them. A `difference` is its concept operand less its element
/// term, which is the one order the form has: the registry's difference is one
/// concept less one element.
fn compose(form: Form, composed_from: Option<&Figure>, read: &[(&Fact, Figure)]) -> Figure {
    let (mut held, rest) = match composed_from {
        Some(figure) => (figure.clone(), read),
        None => match read.split_first() {
            Some(((_, figure), rest)) => (figure.clone(), rest),
            None => (Figure::zero(), read),
        },
    };

    for (_, figure) in rest {
        held = match form {
            Form::Difference => held.plus(&figure.negated()),
            Form::Tag | Form::Sum => held.plus(figure),
        };
    }
    held
}

/// The concept a `difference` reads, where the rule is one.
fn operand_concept(rule: &Rule) -> Option<Concept> {
    match rule.form() {
        Form::Difference => rule.operands().iter().find_map(|operand| match operand {
            Operand::Concept(concept) => Some(*concept),
            Operand::Element { .. } => None,
        }),
        Form::Tag | Form::Sum => None,
    }
}

/// What another concept settles to here, as a figure, and nothing where it
/// settled to anything but a value.
///
/// The same procedure asked again rather than a second reading of it, so an
/// operand concept is settled by the rule that settles every concept —
/// applicability, an assertion, the four steps and the silence reading, in that
/// order. The registry gate has already refused a cycle among the concept edges
/// a difference draws, so the walk ends.
fn amount_of(question: Asked<'_, '_, '_>, concept: Concept) -> Option<Figure> {
    let operand = Asked {
        concept,
        ..question
    };
    match asked(operand) {
        Settled::Value(value) => Figure::read(value.amount()),
        Settled::NotApplicable(_) | Settled::Unknown(_) => None,
    }
}

/// Step 2. A candidate that reads the concept exactly drops the candidates that
/// only stand in for it.
fn exact_before_stand_in(
    candidates: &mut Vec<Candidate<'_, '_>>,
    version: Version,
    declined: &mut Vec<Declined>,
) {
    let exact = candidates
        .iter()
        .any(|candidate| candidate.rule.reading() == Reading::Exact);
    if !exact {
        return;
    }

    candidates.retain(|candidate| {
        if candidate.rule.reading() == Reading::Exact {
            return true;
        }
        declined.push(Declined {
            candidate: candidate.named(version).into(),
            rule: Because::StandInBehindExact.stated().into(),
        });
        false
    });
}

/// Step 3. A candidate whose facts are strictly contained in another survivor's
/// is dropped, so a component never competes with a composition that contains
/// it.
///
/// Containment is transitive, so what a survivor is measured against is every
/// candidate that reached this step: one dropped here was contained in a third,
/// which contains whatever it contained.
fn the_whole_before_its_part(
    candidates: &mut Vec<Candidate<'_, '_>>,
    version: Version,
    declined: &mut Vec<Declined>,
) {
    let reached: Vec<Vec<&Fact>> = candidates
        .iter()
        .map(|candidate| candidate.facts.clone())
        .collect();

    let mut at = 0;
    candidates.retain(|candidate| {
        let facts = &reached[at];
        at += 1;
        let inside = reached
            .iter()
            .any(|other| contains(other, facts) && !contains(facts, other));
        if !inside {
            return true;
        }
        declined.push(Declined {
            candidate: candidate.named(version).into(),
            rule: Because::ContainedInALonger.stated().into(),
        });
        false
    });
}

/// Whether every fact of `part` is a fact of `whole` — the same fact of the same
/// filing, which is identity rather than equality: two facts colliding on the
/// five fields that identify one are two facts, and neither contains the other.
fn contains(whole: &[&Fact], part: &[&Fact]) -> bool {
    part.iter()
        .all(|fact| whole.iter().any(|held| std::ptr::eq(*held, *fact)))
}

/// Step 4's third case: nothing matched, so the concept takes the silence
/// reading the published vocabulary gives it.
///
/// The two conditional readings publish their zero on one condition — that the
/// period's facts are silent on this concept and on the concept named with it
/// alike — and an `Unknown` where that other concept resolves to a non-zero
/// value. Reaching here is the first half of the condition, so what is left to
/// ask is the second. Everything the pair of clauses does not cover takes the
/// reading the vocabulary's own silence test gives everywhere its zero is not
/// supported, which is `Unknown`: a zero here would be one no published clause
/// states, and the pair is the whole of what supports these two.
///
/// A member an override settled is not silent, whatever its number. Read the
/// other way the two clauses would both hold at once where an assertion states a
/// non-zero one — the facts being silent and the concept resolving all the
/// same — and only one of them can be right.
fn silent<'r, 'f>(question: Asked<'_, 'r, 'f>, declined: Vec<Declined>) -> Settled<'r, 'f> {
    let reading = question.concept.definition().silence;

    let zero = match reading {
        Silence::Unknown => false,
        Silence::Zero => true,
        Silence::Conditional => {
            conditioned_on(question.concept).is_some_and(|other| silent_on(question, other))
        }
    };

    match zero {
        true => Settled::Value(Value {
            amount: ZERO.into(),
            set_by: SetBy::Silence { reading },
        }),
        false => Settled::Unknown(Attempt::that_ran(declined)),
    }
}

/// Whether the period's facts are silent on `concept`: applicability first, as
/// everywhere, and then the four steps stopped before their own silence
/// reading, which is what the condition asks about and all it asks about.
///
/// A concept the filer's kind excludes is not silent. Nothing was asked of the
/// facts about it, so they said nothing about it either way. Neither is one
/// reached while a condition was already being answered, for the reason
/// [`Asked::conditioned`] gives.
fn silent_on(question: Asked<'_, '_, '_>, concept: Concept) -> bool {
    if question.conditioned {
        return false;
    }

    let other = Asked {
        concept,
        conditioned: true,
        ..question
    };
    applicability::ask(concept, question.kind) == applicability::Answer::Proceeds
        && matches!(matched(other), Matched::Nothing(_))
}

/// The concept a conditional silence reading is published against.
///
/// The one place in this procedure a concept is named, and it is named because
/// the vocabulary publishes the condition as prose — "the period's facts are
/// silent on this concept and on `dividends_paid` alike" — with nothing
/// mechanical beside it to read the pairing off. A concept the vocabulary
/// publishes as conditional and this does not pair takes `Unknown`, which is the
/// direction to be wrong in: the pair is what supports the zero, so a condition
/// that cannot be read supports nothing.
fn conditioned_on(concept: Concept) -> Option<Concept> {
    match concept {
        Concept::DividendsDeclaredPerShare => Some(Concept::DividendsPaid),
        Concept::DividendsPaid => Some(Concept::DividendsDeclaredPerShare),
        _ => None,
    }
}

/// Why a candidate was declined: the closed set `docs/adr/candidate-choice.md`
/// supplies, in the record's own words and with nothing added to it. A seventh
/// reason would be a decision this procedure had made on its own.
enum Because<'a> {
    NoFact,
    OperandUnanswered(&'a Operand),
    OperandConceptUnresolved,
    StandInBehindExact,
    ContainedInALonger,
    UndecidedAmong(Vec<String>),
}

impl Because<'_> {
    fn stated(&self) -> String {
        match self {
            Because::NoFact => "no fact answering the period asked for".to_owned(),
            Because::OperandUnanswered(operand) => format!(
                "an operand with no fact answering it, so the composition was partial: {}",
                operand_named(operand)
            ),
            Because::OperandConceptUnresolved => {
                "an operand concept that did not resolve to a value".to_owned()
            }
            Because::StandInBehindExact => "a stand-in dropped behind an exact reading".to_owned(),
            Because::ContainedInALonger => {
                "a candidate contained in a longer composition".to_owned()
            }
            Because::UndecidedAmong(others) => format!(
                "undecided among survivors, which names the others: {}",
                others.join(", ")
            ),
        }
    }
}

fn operand_named(operand: &Operand) -> String {
    match operand {
        Operand::Element { taxonomy, tag } => format!("{taxonomy}:{tag}"),
        Operand::Concept(concept) => format!("{concept:?}"),
    }
}

/// One fact as an attempt reports it: the element, the unit it crossed in, the
/// period it is stated for, and the amount as published. The amount is in it
/// because where two facts have to be told apart at all, the five fields that
/// identify a fact are the ones they share.
fn rendered(fact: &Fact) -> String {
    let when = match &fact.period {
        Period::Instant { at } => format!("at {at}"),
        Period::Duration { start, end } => format!("from {start} to {end}"),
    };
    format!(
        "{}:{} {} {when} = {}",
        fact.taxonomy, fact.tag, fact.unit, fact.value
    )
}
