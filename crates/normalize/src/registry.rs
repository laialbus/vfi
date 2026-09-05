//! The one way to the tag mapping.
//!
//! `docs/adr/tag-concept-registry.md` makes the mapping data — TOML under
//! `registry/`, one file per concept and one per filer — and names it among the
//! components ANCHORS.md says are "reached only through an explicit interface,"
//! whose internal shape is private and may change freely. This module is that
//! interface, and everything about files, directories, TOML and the way an
//! override is written stops here. Adding a second consumer needs an ADR.
//!
//! The question is one question, per filer and per concept: which rules may
//! reach this concept for this filer, and what is each called. [`Registry::answer`]
//! reports which rules are eligible, never which one wins. Entries for a concept
//! are a set whose order in the file means nothing, and choosing between two that
//! both match facts in one filing is a separate record that does not exist yet —
//! so the answer comes back in the order the rule ids sort in, which is the one
//! order that carries no information about how the files were written. Nothing
//! here reads `form`, `filed` or `accession` to prefer one filing over another
//! either; reconciling amendments, restatements and periods is its own ruleset,
//! and a preference invented here would be that ruleset written where nobody
//! would look for it.
//!
//! What it cannot answer is `NotApplicable`. The vocabulary makes that state
//! constructible from a filer kind together with an applicability clause that
//! excludes it and from nothing else, and it asks applicability first, so a
//! concept that reaches the registry at all is one the filer's kind admits. A
//! concept with no eligible rule and a concept with no entry anywhere are the
//! same answer here — an attempt that ran and found nothing — and what that
//! reads as is the vocabulary's business, not this module's.
//!
//! The version is the other half of what it answers. A resolved value records
//! the pair of registry version and rule id and never the id alone, because the
//! same id under two versions may name different bytes; so every answer carries
//! the version that gave it, and the digest is computed in one place, over the
//! same bytes the answer was read from.
//!
//! A registry with a defect in it does not load and the stage does not run. It
//! never degrades to an empty eligible set: that would surface as a concept
//! nothing reached, which is the state meaning a filing was consulted and gave
//! nothing, and a data defect hidden behind it is the plausible-looking wrong
//! answer this milestone exists to prevent.

mod read;
mod sha256;
mod tree;

use std::fmt;
use std::path::Path;

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::Period;

use tree::Tree;

/// The registry as one loaded value: every rule it states, every override, and
/// the version of the bytes all of it was read from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry {
    version: Version,
    general: Vec<Vec<Rule>>,
    filers: Vec<Overrides>,
}

/// Which state of the registry answered: the digest of its own bytes.
///
/// By content rather than by a hand-written sequence, because the registry
/// changes whenever a filer uses a tag not yet listed and whenever an override
/// is written — often, and in parallel — and a digest is computed rather than
/// chosen, so two runs adding tags collide over nothing.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Version([u8; 32]);

/// What the registry answers, and the version it answered under. The two travel
/// together because a rule id read against a different version may name
/// different bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answer<'r> {
    pub version: Version,
    pub outcome: Outcome<'r>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome<'r> {
    /// An override asserts this concept for this filer over this period. The
    /// answer is that value and no eligible-rule lookup ran, and it carries the
    /// assertion's own rule id, so an asserted number is visibly asserted rather
    /// than indistinguishable from a read one.
    Asserted(&'r Assertion),
    /// The rules eligible for this filer and this concept, each carrying its id.
    ///
    /// A set, in id order, which is not a preference: nothing here ranks them or
    /// chooses among them. Empty is an answer like any other — an attempt that
    /// ran and found nothing.
    Eligible(Vec<&'r Rule>),
}

/// One rule the mapping states: what it reaches the concept through, and what it
/// is called.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    id: Box<str>,
    concept: Concept,
    kinds: Box<[Kind]>,
    form: Form,
    operands: Box<[Operand]>,
}

/// How a rule reaches its concept.
///
/// Three, not the four the format has: `assert` is a value stated for one filer
/// over one period, and it is answered as [`Outcome::Asserted`] rather than
/// handed out as a rule to try.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Form {
    /// One element.
    Tag,
    /// Two or more elements whose values add to the concept.
    Sum,
    /// One concept less one element.
    Difference,
}

/// What a rule reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    /// One element, named by its taxonomy and its tag together, because the
    /// boundary publishes both and two taxonomies may define one name.
    Element { taxonomy: Box<str>, tag: Box<str> },
    /// Another concept, which only the difference form takes.
    Concept(Concept),
}

/// A value an override states outright for one filer, one concept and one
/// period.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assertion {
    rule: Box<str>,
    concept: Concept,
    period: Period,
    value: Box<str>,
    source: Source,
}

/// The filing an assertion is read from. Nothing checks that the citation is
/// true; what the required field buys is that an assertion with no stated ground
/// cannot be written at all, which is anchor 5's ban on the uncited value applied
/// to the one place a number enters normalize with no tag behind it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    accession: Box<str>,
    line: Box<str>,
}

/// Why a registry did not load, in as many lines as it has problems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refusal {
    problems: Vec<String>,
}

impl Registry {
    /// The registry under `root`, or every reason it cannot be trusted.
    ///
    /// Where it is read from arrives as an argument rather than as a path this
    /// crate assumes: the engine holds no per-user state, and every operation
    /// takes its inputs explicitly.
    pub fn read_from(root: &Path) -> Result<Registry, Refusal> {
        let words = Vocabulary::published();
        let mut problems = Problems::none();

        let tree = match Tree::read(root) {
            Ok(tree) => tree,
            Err(why) => {
                return Err(Refusal {
                    problems: vec![why],
                });
            }
        };
        let version = Version(tree.digest());

        let general = general_half(&tree, &words, &mut problems);
        let filers = filer_half(&tree, &words, &mut problems);

        let registry = Registry {
            version,
            general,
            filers,
        };
        registry.acyclic(&words, &mut problems);

        match problems.held() {
            true => Err(Refusal {
                problems: problems.into_lines(),
            }),
            false => Ok(registry),
        }
    }

    /// The version every answer from this registry carries.
    pub fn version(&self) -> Version {
        self.version
    }

    /// The kind this filer is assigned, and none where no file assigns it one.
    ///
    /// The vocabulary routes kind assignment to this registry, so this is where
    /// a filer's kind comes from — and it is asked separately rather than read
    /// inside [`Registry::answer`] because whether a kind may instead be derived
    /// from a filer's own facts is a question the record leaves open, and an
    /// answer that took the kind as an argument does not change when it is
    /// settled.
    pub fn kind_of(&self, filer: &str) -> Option<Kind> {
        self.overrides(filer).and_then(|held| held.kind)
    }

    /// Which rules may reach `concept` for this filer over this period.
    ///
    /// The eligible set is the general set for the filer's kind, less what the
    /// filer's file excludes, plus what it includes — and it does not depend on
    /// the order the two are written in. An entry with no kind scope applies to
    /// every filer, including one whose kind has not been established; a scoped
    /// entry applies to no filer whose kind is unknown.
    ///
    /// The period is part of the question because an assertion covers the period
    /// it names and no other, so an answer that could be an assertion has to
    /// know which period is being asked about.
    pub fn answer<'r>(
        &'r self,
        filer: &str,
        kind: Option<Kind>,
        concept: Concept,
        period: &Period,
    ) -> Answer<'r> {
        let overrides = self.overrides(filer);

        if let Some(asserted) = overrides.and_then(|held| held.asserting(concept, period)) {
            return Answer {
                version: self.version,
                outcome: Outcome::Asserted(asserted),
            };
        }

        let mut eligible: Vec<&Rule> = Vec::new();
        for rule in &self.general[slot(concept)] {
            let excluded = overrides.is_some_and(|held| held.excludes(&rule.id));
            if rule.admits(kind) && !excluded {
                eligible.push(rule);
            }
        }
        if let Some(held) = overrides {
            for rule in &held.included {
                if rule.concept == concept && rule.admits(kind) {
                    eligible.push(rule);
                }
            }
        }

        eligible.sort_by(|one, other| one.id.cmp(&other.id));
        eligible.dedup_by(|one, other| one.id == other.id);

        Answer {
            version: self.version,
            outcome: Outcome::Eligible(eligible),
        }
    }

    fn overrides(&self, filer: &str) -> Option<&Overrides> {
        self.filers
            .binary_search_by(|held| (*held.cik).cmp(filer))
            .ok()
            .map(|at| &self.filers[at])
    }

    /// The concept edges the difference form draws, walked for a cycle: once as
    /// the general half draws them, and once per filer over the edges its own
    /// file leaves in place. A concept resolved from a concept resolved from
    /// itself has no first step.
    fn acyclic(&self, words: &Vocabulary, problems: &mut Problems) {
        let general: Vec<Edge> = self
            .general
            .iter()
            .flatten()
            .filter_map(|rule| rule.edge())
            .collect();
        if let Some(cycle) = cycle(&general, words) {
            problems.about(
                CONCEPTS,
                &format!("the difference form draws a cycle in the concept edges, from {cycle}"),
            );
        }

        for filer in &self.filers {
            let mut edges: Vec<Edge> = Vec::new();
            let mut touched = false;
            for rule in self.general.iter().flatten() {
                let Some(edge) = rule.edge() else { continue };
                if filer.excludes(&rule.id) {
                    touched = true;
                    continue;
                }
                edges.push(edge);
            }
            for rule in &filer.included {
                if let Some(edge) = rule.edge() {
                    touched = true;
                    edges.push(edge);
                }
            }
            if !touched {
                continue;
            }
            if let Some(cycle) = cycle(&edges, words) {
                problems.about(
                    &format!("{FILERS}{}.toml", filer.cik),
                    &format!(
                        "draws a cycle in the concept edges of its own eligible set, from {cycle}"
                    ),
                );
            }
        }
    }
}

impl Rule {
    /// What this rule is called: the canonical rendering of exactly the fields
    /// that make it that rule, derived from them rather than written beside
    /// them, so it cannot be mistyped, cannot be copied onto a second rule, and
    /// cannot drift from what it names.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn form(&self) -> Form {
        self.form
    }

    pub fn operands(&self) -> &[Operand] {
        &self.operands
    }

    fn stated(
        concept: Concept,
        kinds: Vec<Kind>,
        form: Form,
        operands: Vec<Operand>,
        words: &Vocabulary,
    ) -> Rule {
        let mut id = String::new();
        id.push_str(words.spelling_of(concept));
        id.push('|');
        id.push_str(&scope_rendered(&kinds, words));
        id.push('|');
        id.push_str(match form {
            Form::Tag => "tag",
            Form::Sum => "sum",
            Form::Difference => "difference",
        });
        id.push('|');
        for (at, operand) in operands.iter().enumerate() {
            if at > 0 {
                id.push('+');
            }
            match operand {
                Operand::Concept(named) => {
                    id.push_str("concept:");
                    id.push_str(words.spelling_of(*named));
                }
                Operand::Element { taxonomy, tag } => {
                    id.push_str("element:");
                    id.push_str(taxonomy);
                    id.push(':');
                    id.push_str(tag);
                }
            }
        }

        Rule {
            id: id.into(),
            concept,
            kinds: kinds.into(),
            form,
            operands: operands.into(),
        }
    }

    /// An assertion's id, which renders the same four fields with the filer and
    /// the period standing where a rule's operands stand — those being what
    /// makes an assertion the assertion it is.
    fn asserted_id(concept: Concept, filer: &str, period: &Period, words: &Vocabulary) -> String {
        let covered = match period {
            Period::Instant { at } => format!("instant:{at}"),
            Period::Duration { start, end } => format!("duration:{start}:{end}"),
        };
        format!(
            "{}|*|assert|filer:{filer}+{covered}",
            words.spelling_of(concept)
        )
    }

    fn admits(&self, kind: Option<Kind>) -> bool {
        self.kinds.is_empty() || kind.is_some_and(|kind| self.kinds.contains(&kind))
    }

    fn edge(&self) -> Option<Edge> {
        match self.operands.first() {
            Some(Operand::Concept(from)) if self.form == Form::Difference => {
                Some(Edge(slot(self.concept), slot(*from)))
            }
            _ => None,
        }
    }
}

impl Assertion {
    /// The rule id this assertion renders, which is what a resolved value
    /// records beside the registry version.
    pub fn rule(&self) -> &str {
        &self.rule
    }

    /// The amount, as the decimal literal the filing publishes it as. Unparsed,
    /// for the reason it crosses the fetch boundary that way: a binary float is
    /// a lossy reading of a published decimal.
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn period(&self) -> &Period {
        &self.period
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

impl Source {
    pub fn accession(&self) -> &str {
        &self.accession
    }

    pub fn line(&self) -> &str {
        &self.line
    }
}

impl Version {
    /// The sixty-four lowercase hexadecimal characters this version is named by.
    pub fn rendered(&self) -> String {
        sha256::rendered(&self.0)
    }
}

/// A version is read and written as its digest, so that is what it shows —
/// thirty-two bytes printed as bytes are not something anyone can compare
/// against a stored value.
impl fmt::Debug for Version {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "Version({})", self.rendered())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&self.rendered())
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            out,
            "the registry is not what its record and the published vocabulary allow:"
        )?;
        for problem in &self.problems {
            writeln!(out, "  {problem}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Refusal {}

impl Refusal {
    /// Every problem found, one to a line. All of them rather than the first:
    /// a registry is fixed by someone reading what is wrong with it.
    pub fn problems(&self) -> &[String] {
        &self.problems
    }
}

/// What one filer's file changes about the general mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Overrides {
    cik: Box<str>,
    kind: Option<Kind>,
    included: Vec<Rule>,
    excluded: Vec<Box<str>>,
    asserted: Vec<Assertion>,
}

impl Overrides {
    fn excludes(&self, id: &str) -> bool {
        self.excluded.iter().any(|held| &**held == id)
    }

    fn asserting(&self, concept: Concept, period: &Period) -> Option<&Assertion> {
        self.asserted
            .iter()
            .find(|held| held.concept == concept && held.period == *period)
    }
}

/// A concept resolved from another concept: the edge the difference form draws.
#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge(usize, usize);

/// The problems a reading found. All of them, because a registry is fixed by
/// someone reading what is wrong with it, and a reader that stopped at the first
/// would report a second only after the first was fixed.
struct Problems(Vec<String>);

impl Problems {
    fn none() -> Self {
        Problems(Vec::new())
    }

    fn at(&mut self, path: &str, line: usize, what: &str) {
        self.0.push(format!("{path} line {line}: {what}"));
    }

    fn about(&mut self, subject: &str, what: &str) {
        self.0.push(format!("{subject}: {what}"));
    }

    fn held(&self) -> bool {
        !self.0.is_empty()
    }

    fn into_lines(self) -> Vec<String> {
        self.0
    }
}

/// The published vocabulary, spelled the way the registry writes it.
///
/// Rendered from the type's own declaration rather than written out again: a
/// list here would be a second copy of the published set, and the copy is what
/// drifts. `InvestmentCompany` is `investment_company`, and that is the whole of
/// the translation between the two spellings.
struct Vocabulary {
    concepts: Vec<(String, Concept)>,
    kinds: Vec<(String, Kind)>,
}

impl Vocabulary {
    fn published() -> Self {
        Vocabulary {
            concepts: Concept::ALL
                .iter()
                .map(|concept| (spelled(&format!("{concept:?}")), *concept))
                .collect(),
            kinds: Kind::ALL
                .iter()
                .map(|kind| (spelled(&format!("{kind:?}")), *kind))
                .collect(),
        }
    }

    fn concept_named(&self, name: &str) -> Option<Concept> {
        self.concepts
            .iter()
            .find(|(spelling, _)| spelling == name)
            .map(|(_, concept)| *concept)
    }

    fn kind_named(&self, name: &str) -> Option<Kind> {
        self.kinds
            .iter()
            .find(|(spelling, _)| spelling == name)
            .map(|(_, kind)| *kind)
    }

    fn spelling_of(&self, concept: Concept) -> &str {
        &self.concepts[slot(concept)].0
    }

    fn spelling_of_kind(&self, kind: Kind) -> &str {
        self.kinds
            .iter()
            .find(|(_, held)| *held == kind)
            .map(|(spelling, _)| spelling.as_str())
            .expect("every published kind is in the vocabulary")
    }
}

fn spelled(declared: &str) -> String {
    let mut spelling = String::with_capacity(declared.len() + 2);
    for (at, letter) in declared.char_indices() {
        if at > 0 && letter.is_ascii_uppercase() {
            spelling.push('_');
        }
        spelling.push(letter.to_ascii_lowercase());
    }
    spelling
}

/// Where a concept sits in the published order, which is what the general half
/// is held in: the vocabulary publishes a closed set, so a slot per member is
/// the whole of the indexing needed.
fn slot(concept: Concept) -> usize {
    Concept::ALL
        .iter()
        .position(|held| *held == concept)
        .expect("every published concept is one of ALL")
}

/// The scope a rule renders in its id. It is a set, so it renders sorted; a
/// scope that rendered in the order written would give one rule two ids.
fn scope_rendered(kinds: &[Kind], words: &Vocabulary) -> String {
    if kinds.is_empty() {
        return "*".to_owned();
    }
    let mut named: Vec<&str> = kinds
        .iter()
        .map(|kind| words.spelling_of_kind(*kind))
        .collect();
    named.sort_unstable();
    named.join(",")
}

/// The dates a period covers, as the characters the boundary publishes them as.
/// Nothing here parses a date: the parse is a reading, and two dates written this
/// way compare as they are written.
fn covered(period: &Period) -> (&str, &str) {
    match period {
        Period::Instant { at } => (at, at),
        Period::Duration { start, end } => (start, end),
    }
}

const CONCEPTS: &str = "concepts/";
const FILERS: &str = "filers/";

fn general_half(tree: &Tree, words: &Vocabulary, problems: &mut Problems) -> Vec<Vec<Rule>> {
    let mut general: Vec<Vec<Rule>> = Concept::ALL.iter().map(|_| Vec::new()).collect();
    let mut accounted: Vec<bool> = Concept::ALL.iter().map(|_| false).collect();
    let mut rendered: Vec<Box<str>> = Vec::new();

    let (files, misplaced) = tree.inside(CONCEPTS);
    for path in misplaced {
        problems.about(
            path,
            "is not a concept file, and this directory holds one per concept and nothing else",
        );
    }

    for file in files {
        let named = file.stem(CONCEPTS);
        let Some(concept) = words.concept_named(named) else {
            problems.about(
                &file.path,
                &format!("names {named}, which the published vocabulary does not publish"),
            );
            continue;
        };

        let mapping = read::concept_file(file, concept, words, problems);
        accounted[slot(concept)] = mapping.unreachable || !mapping.rules.is_empty();

        for rule in mapping.rules {
            if rendered.contains(&rule.id) {
                problems.about(named, &format!("renders one id for two rules, {}", rule.id));
                continue;
            }
            rendered.push(rule.id.clone());
            general[slot(concept)].push(rule);
        }
    }

    for (concept, held) in Concept::ALL.iter().zip(&accounted) {
        if !held {
            problems.about(
                words.spelling_of(*concept),
                "is accounted for neither way, by no entry and no unreachable declaration",
            );
        }
    }

    general
}

fn filer_half(tree: &Tree, words: &Vocabulary, problems: &mut Problems) -> Vec<Overrides> {
    let mut filers: Vec<Overrides> = Vec::new();

    let (files, misplaced) = tree.inside(FILERS);
    for path in misplaced {
        problems.about(
            path,
            "is not a filer file, and this directory holds one per filer and nothing else",
        );
    }

    for file in files {
        let named = file.stem(FILERS);
        if !read::is_cik(named) {
            problems.about(
                &file.path,
                "is not named for a CIK, which is ten digits left-padded with zeros and never a ticker",
            );
            continue;
        }

        let overrides = read::filer_file(file, named, words, problems);

        for rule in &overrides.included {
            if overrides.excludes(&rule.id) {
                problems.about(
                    &file.path,
                    &format!("both includes and excludes the rule {}", rule.id),
                );
            }
        }

        // Two assertions for one concept whose periods overlap are ambiguous
        // override data, and this refuses them rather than choosing between
        // them: an ambiguous override is a defect, and Unknown would hide it
        // behind a state that means a filing was consulted.
        for (at, held) in overrides.asserted.iter().enumerate() {
            let (start, end) = covered(&held.period);
            for other in &overrides.asserted[..at] {
                let (from, to) = covered(&other.period);
                if other.concept == held.concept && from <= end && start <= to {
                    problems.about(
                        &file.path,
                        &format!(
                            "states two assertions for {} whose periods overlap",
                            words.spelling_of(held.concept)
                        ),
                    );
                }
            }
        }

        filers.push(overrides);
    }

    filers.sort_by(|one, other| one.cik.cmp(&other.cik));
    filers
}

/// The first cycle these edges draw, named by the step that closes it.
fn cycle(edges: &[Edge], words: &Vocabulary) -> Option<String> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Untouched,
        Open,
        Closed,
    }

    fn visit(
        at: usize,
        edges: &[Edge],
        state: &mut Vec<State>,
        words: &Vocabulary,
    ) -> Option<String> {
        state[at] = State::Open;
        for Edge(from, to) in edges.iter().filter(|Edge(from, _)| *from == at) {
            match state[*to] {
                State::Open => {
                    return Some(format!(
                        "{} to {}",
                        words.spelling_of(Concept::ALL[*from]),
                        words.spelling_of(Concept::ALL[*to])
                    ));
                }
                State::Untouched => {
                    if let Some(found) = visit(*to, edges, state, words) {
                        return Some(found);
                    }
                }
                State::Closed => {}
            }
        }
        state[at] = State::Closed;
        None
    }

    let mut state = vec![State::Untouched; Concept::ALL.len()];
    for Edge(from, _) in edges {
        if state[*from] == State::Untouched
            && let Some(found) = visit(*from, edges, &mut state, words)
        {
            return Some(found);
        }
    }
    None
}
