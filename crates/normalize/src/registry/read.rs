//! The registry's format, and the engine's only reading of it.
//!
//! The grammar is a subset of TOML and deliberately a narrow one — a table
//! header alone on its line, a field written `key = value`, and an operand list
//! written one operand to a line, each ending in its comma. Anything outside it
//! is refused rather than guessed at, which is the only honest answer over a
//! surface where a plausible misreading is the failure mode.
//!
//! What this refuses is what the gate over the registry refuses, as far as the
//! files it reads. A registry the gate would fail does not load, and the stage
//! does not run: a defect passed over here would surface as a concept nothing
//! reached, which is the state that means a filing was consulted and gave
//! nothing, and a data defect hidden behind that is the plausible-looking wrong
//! answer this milestone exists to prevent. The two readings are held together
//! by the committed registry, which both must accept.
//!
//! A file that fails on its shape stops there. What the rest of it claims is not
//! worth reading once the shape is wrong, and reporting every line after the
//! first mistake buries the mistake.

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::Period;

use super::tree::File;
use super::{Assertion, Form, Operand, Overrides, Problems, Rule, Source, Vocabulary};

/// One concept's file: the rules it states, and whether it declares the concept
/// out of the mapping's reach instead.
pub(super) struct Mapping {
    pub(super) rules: Vec<Rule>,
    pub(super) unreachable: bool,
}

pub(super) fn concept_file(
    file: &File,
    concept: Concept,
    words: &Vocabulary,
    problems: &mut Problems,
) -> Mapping {
    let named = words.spelling_of(concept).to_owned();
    let mut reading = Reading::new(&file.path, named, words, problems);
    let mut mapping = Mapping {
        rules: Vec::new(),
        unreachable: false,
    };

    let Some(text) = file.text() else {
        reading.say("is not text, and the registry is written in TOML");
        return mapping;
    };

    let mut entries = 0;
    let mut reason = false;
    let mut section = Section::None;

    for line in text.lines() {
        reading.line += 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if reading.in_operands {
            reading.operand_line(line);
            if reading.broken {
                return mapping;
            }
            continue;
        }

        if line == "[[entry]]" {
            reading.take_entry(concept, &mut mapping.rules);
            reading.entry = Entry::default();
            reading.pending = true;
            section = Section::Entry;
            entries += 1;
            continue;
        }

        if line == "[unreachable]" {
            reading.take_entry(concept, &mut mapping.rules);
            if mapping.unreachable {
                reading.fault("declares the concept unreachable twice");
                return mapping;
            }
            mapping.unreachable = true;
            section = Section::Unreachable;
            continue;
        }

        if line.starts_with('[') {
            reading.fault("names a table this format does not have");
            return mapping;
        }

        let Some((key, value)) = field(line) else {
            reading.fault("is neither a comment, a table, nor a field");
            return mapping;
        };

        match section {
            Section::None => {
                reading.fault("writes a field with no table above it");
                return mapping;
            }
            Section::Unreachable => {
                if key != "reason" {
                    reading.fault(&format!(
                        "writes the field {key}, which an unreachable declaration does not have"
                    ));
                } else if reason {
                    reading.fault("states its reason twice");
                } else if quoted(value).is_none_or(str::is_empty) {
                    reading.fault("states a reason that is not a string with something in it");
                } else {
                    reason = true;
                }
            }
            Section::Entry => {
                if !reading.entry_field(key, value) {
                    reading.fault(&format!(
                        "writes the field {key}, which an entry does not have"
                    ));
                }
            }
        }

        if reading.broken {
            return mapping;
        }
    }

    reading.take_entry(concept, &mut mapping.rules);
    if reading.in_operands {
        reading.say("leaves an operand list open");
        return mapping;
    }

    if mapping.unreachable {
        if !reason {
            reading.say("is declared unreachable and states no reason for it");
        }
        if entries > 0 {
            reading.say("is both mapped and declared unreachable");
        }
    }

    mapping
}

pub(super) fn filer_file(
    file: &File,
    cik: &str,
    words: &Vocabulary,
    problems: &mut Problems,
) -> Overrides {
    let named = file.path.clone();
    let mut reading = Reading::new(&file.path, named, words, problems);
    let mut overrides = Overrides {
        cik: cik.into(),
        kind: None,
        included: Vec::new(),
        excluded: Vec::new(),
        asserted: Vec::new(),
    };
    let mut stated_cik = None;
    let mut stated_kind = false;
    let mut table = Table::None;

    let Some(text) = file.text() else {
        reading.say("is not text, and the registry is written in TOML");
        return overrides;
    };

    for line in text.lines() {
        reading.line += 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if reading.in_operands {
            reading.operand_line(line);
            if reading.broken {
                return overrides;
            }
            continue;
        }

        if line == "[[include]]" || line == "[[exclude]]" || line == "[[assert]]" {
            reading.take_table(table, &mut overrides);
            table = match line {
                "[[include]]" => Table::Include,
                "[[exclude]]" => Table::Exclude,
                _ => Table::Assert,
            };
            reading.pending = table == Table::Include;
            continue;
        }

        if line.starts_with('[') {
            reading.fault("names a table this format does not have");
            return overrides;
        }

        let Some((key, value)) = field(line) else {
            reading.fault("is neither a comment, a table, nor a field");
            return overrides;
        };

        match table {
            Table::None => match key {
                "cik" => {
                    if stated_cik.is_some() {
                        reading.fault("states its cik twice");
                    } else {
                        match quoted(value) {
                            Some(held) if is_cik(held) => stated_cik = Some(held.to_owned()),
                            Some(_) => reading.fault(
                                "states a cik that is not ten digits, left-padded with zeros",
                            ),
                            None => reading.fault("states a cik that is not a string"),
                        }
                    }
                }
                "kind" => {
                    if stated_kind {
                        reading.fault("assigns the filer a second kind");
                    } else {
                        stated_kind = true;
                        reading.assign_kind(value, &mut overrides);
                    }
                }
                _ => reading.fault(&format!(
                    "writes the field {key}, which a filer file does not have"
                )),
            },
            Table::Include => {
                if key == "concept" {
                    reading.name_concept("includes a rule for", value);
                } else if !reading.entry_field(key, value) {
                    reading.fault(&format!(
                        "writes the field {key}, which an include does not have"
                    ));
                }
            }
            Table::Exclude => reading.exclude_field(key, value),
            Table::Assert => reading.assert_field(key, value),
        }

        if reading.broken {
            return overrides;
        }
    }

    reading.take_table(table, &mut overrides);
    if reading.in_operands {
        reading.say("leaves an operand list open");
        return overrides;
    }

    // A filer file is about one filer and says so twice: by its name, which is
    // the CIK, and by the cik it states back. What the pair catches is the file
    // copied for a second filer and edited nowhere else.
    match stated_cik {
        None => reading.say("states no cik, so nothing in it says which filer it is about"),
        Some(held) if held != cik => reading.say(&format!(
            "states the cik {held}, which is not the filer its name binds it to"
        )),
        Some(_) => {}
    }

    overrides
}

#[derive(Clone, Copy, PartialEq)]
enum Section {
    None,
    Entry,
    Unreachable,
}

#[derive(Clone, Copy, PartialEq)]
enum Table {
    None,
    Include,
    Exclude,
    Assert,
}

/// The entry being read, in either half: an include is an entry with its concept
/// written out beside it, so one reading serves both and the two cannot drift.
#[derive(Default)]
struct Entry {
    form: Option<String>,
    form_stated: bool,
    kinds: Vec<Kind>,
    kinds_stated: bool,
    operands: Vec<Operand>,
    operands_stated: bool,
    concept: Option<Concept>,
    concept_stated: bool,
}

#[derive(Default)]
struct Excluding {
    id: Option<String>,
    stated: bool,
}

#[derive(Default)]
struct Asserting {
    concept: Option<Concept>,
    concept_stated: bool,
    period: Option<Period>,
    period_stated: bool,
    value: Option<String>,
    value_stated: bool,
    source: Option<Source>,
    source_stated: bool,
}

struct Reading<'a> {
    path: &'a str,
    subject: String,
    line: usize,
    broken: bool,
    pending: bool,
    in_operands: bool,
    entry: Entry,
    excluding: Excluding,
    asserting: Asserting,
    words: &'a Vocabulary,
    problems: &'a mut Problems,
}

impl<'a> Reading<'a> {
    fn new(
        path: &'a str,
        subject: String,
        words: &'a Vocabulary,
        problems: &'a mut Problems,
    ) -> Self {
        Reading {
            path,
            subject,
            line: 0,
            broken: false,
            pending: false,
            in_operands: false,
            entry: Entry::default(),
            excluding: Excluding::default(),
            asserting: Asserting::default(),
            words,
            problems,
        }
    }

    /// A shape this reader cannot make sense of. It names the line, and it stops
    /// the file: everything after it would be read against a state that is
    /// already wrong.
    fn fault(&mut self, what: &str) {
        self.problems.at(self.path, self.line, what);
        self.broken = true;
    }

    /// A file that reads but states something the vocabulary or the record does
    /// not allow. It names the subject rather than the line, because what is
    /// wrong with it is not where it is written.
    fn say(&mut self, what: &str) {
        self.problems.about(&self.subject, what);
    }

    fn operand_line(&mut self, line: &str) {
        if line == "]" {
            self.in_operands = false;
            return;
        }
        match operand(line, self.words) {
            Ok(read) => self.entry.operands.push(read),
            Err(Malformed::Shape(what)) => self.fault(what),
            Err(Malformed::Unpublished(named)) => self.say(&format!(
                "has an operand naming {named}, which the published vocabulary does not publish"
            )),
        }
    }

    /// The fields an entry carries, in the general half and in a filer's include
    /// alike. A field this does not know is not a fault here: each caller names
    /// what it was reading in its own words.
    fn entry_field(&mut self, key: &str, value: &str) -> bool {
        match key {
            "form" => {
                if self.entry.form_stated {
                    self.fault("states form twice in one entry");
                } else {
                    self.entry.form_stated = true;
                    match quoted(value).filter(|held| is_bare_name(held)) {
                        Some(held) => self.entry.form = Some(held.to_owned()),
                        None => self.fault("states a form that is not a bare name"),
                    }
                }
                true
            }
            "kinds" => {
                if self.entry.kinds_stated {
                    self.fault("states kinds twice in one entry");
                } else {
                    self.entry.kinds_stated = true;
                    self.read_kinds(value);
                }
                true
            }
            "operands" => {
                if self.entry.operands_stated {
                    self.fault("states operands twice in one entry");
                } else if value != "[" {
                    self.fault(
                        "opens an operand list on a line that carries more than the bracket",
                    );
                } else {
                    self.entry.operands_stated = true;
                    self.in_operands = true;
                }
                true
            }
            _ => false,
        }
    }

    fn read_kinds(&mut self, value: &str) {
        let Some(body) = value
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        else {
            self.fault("scopes an entry to something that is not a list of kinds");
            return;
        };
        if body.trim().is_empty() {
            self.say("has an entry scoped to no kind, which no filer can match");
            return;
        }

        for part in body.split(',') {
            let Some(name) = quoted(part.trim()).filter(|held| is_bare_name(held)) else {
                self.fault("scopes an entry to a kind that is not a bare name");
                return;
            };
            let Some(kind) = self.words.kind_named(name) else {
                self.say(&format!(
                    "scopes an entry to {name}, which the published vocabulary does not publish"
                ));
                continue;
            };
            if self.entry.kinds.contains(&kind) {
                self.fault("scopes one entry to the same kind twice");
                return;
            }
            self.entry.kinds.push(kind);
        }
    }

    /// The two fields a filer file carries about the filer itself. A ticker is
    /// not one of them.
    fn assign_kind(&mut self, value: &str, overrides: &mut Overrides) {
        let Some(held) = quoted(value).filter(|held| is_bare_name(held)) else {
            self.fault("assigns a kind that is not a bare name");
            return;
        };
        match self.words.kind_named(held) {
            Some(kind) => overrides.kind = Some(kind),
            None => self.say(&format!(
                "assigns the kind {held}, which the published vocabulary does not publish"
            )),
        }
    }

    /// The concept an override is about. The general half takes it from the name
    /// of the file; a filer file is about a filer, so each override writes it
    /// out.
    fn name_concept(&mut self, what: &str, value: &str) {
        let including = self.pending;
        let stated = match including {
            true => &mut self.entry.concept_stated,
            false => &mut self.asserting.concept_stated,
        };
        if *stated {
            self.fault("names the concept of one override twice");
            return;
        }
        *stated = true;

        let Some(name) = quoted(value).filter(|held| is_bare_name(held)) else {
            self.fault("names a concept that is not a bare name");
            return;
        };
        match self.words.concept_named(name) {
            None => self.say(&format!(
                "{what} {name}, which the published vocabulary does not publish"
            )),
            Some(concept) => match including {
                true => self.entry.concept = Some(concept),
                false => self.asserting.concept = Some(concept),
            },
        }
    }

    fn exclude_field(&mut self, key: &str, value: &str) {
        if key != "id" {
            self.fault(&format!(
                "writes the field {key}, which an exclude does not have"
            ));
        } else if self.excluding.stated {
            self.fault("excludes two rules in one table");
        } else {
            self.excluding.stated = true;
            match quoted(value).filter(|held| !held.is_empty()) {
                Some(held) => self.excluding.id = Some(held.to_owned()),
                None => self.fault("excludes a rule id that is not a string with something in it"),
            }
        }
    }

    fn assert_field(&mut self, key: &str, value: &str) {
        match key {
            "concept" => self.name_concept("asserts a value for", value),
            "period" => {
                if self.asserting.period_stated {
                    self.fault("states the period of one assertion twice");
                } else {
                    self.asserting.period_stated = true;
                    self.read_period(value);
                }
            }
            "value" => {
                if self.asserting.value_stated {
                    self.fault("states the value of one assertion twice");
                } else {
                    self.asserting.value_stated = true;
                    match quoted(value).filter(|held| is_decimal(held)) {
                        Some(held) => self.asserting.value = Some(held.to_owned()),
                        None => self.fault(
                            "states a value that is not the decimal literal a filing publishes",
                        ),
                    }
                }
            }
            "source" => {
                if self.asserting.source_stated {
                    self.fault("cites the source of one assertion twice");
                } else {
                    self.asserting.source_stated = true;
                    self.read_source(value);
                }
            }
            _ => self.fault(&format!(
                "writes the field {key}, which an assertion does not have"
            )),
        }
    }

    /// A period is one of the two shapes the fetch boundary publishes — the date
    /// an instant is stated at, or the two a duration runs between — and there
    /// is no third. None of them is every period: the case an assertion is
    /// written for most often is a true zero, and a filer with no borrowings
    /// this year may borrow next year.
    fn read_period(&mut self, value: &str) {
        let Some(keys) = inline(value) else {
            self.say("states a period that is not a table of the dates it covers");
            return;
        };

        let period = match keys.as_slice() {
            [("instant", at)] => Period::Instant { at: (*at).into() },
            [("start", start), ("end", end)] | [("end", end), ("start", start)] => {
                Period::Duration {
                    start: (*start).into(),
                    end: (*end).into(),
                }
            }
            _ => {
                self.say(
                    "states a period that is neither an instant nor a duration, and an assertion covers the period it names and never every period",
                );
                return;
            }
        };

        let (start, end) = super::covered(&period);
        if !is_date(start) || !is_date(end) {
            self.say("states a period whose dates are not dates");
            return;
        }
        if start > end {
            self.say("states a period that ends before it starts");
            return;
        }
        self.asserting.period = Some(period);
    }

    /// Where an asserted value was read from. Nothing checks that the citation is
    /// true. What the required field buys is that an assertion with no stated
    /// ground cannot be written at all.
    fn read_source(&mut self, value: &str) {
        let Some(keys) = inline(value) else {
            self.say("cites a source that is not an accession and the line it is read from");
            return;
        };
        let (accession, line) = match keys.as_slice() {
            [("accession", accession), ("line", line)]
            | [("line", line), ("accession", accession)] => (*accession, *line),
            _ => {
                self.say("cites a source that is not an accession and the line it is read from");
                return;
            }
        };

        let mut cited = true;
        if !is_accession(accession) {
            self.say(&format!("cites {accession}, which is not an accession"));
            cited = false;
        }
        if !is_line(line) {
            self.say("cites a line that is not one");
            cited = false;
        }
        if cited {
            self.asserting.source = Some(Source {
                accession: accession.into(),
                line: line.into(),
            });
        }
    }

    /// The entry that ends where the next table begins, or where the file does.
    fn take_entry(&mut self, concept: Concept, into: &mut Vec<Rule>) {
        if !self.pending {
            return;
        }
        self.pending = false;

        let entry = std::mem::take(&mut self.entry);
        let Some(form) = entry.form.as_deref() else {
            self.say("has an entry with no form, so nothing says what it is");
            return;
        };
        if entry.operands.is_empty() {
            self.say(&format!("has a {form} entry with no operands"));
            return;
        }

        let shaped = match form {
            "tag" => match entry.operands.as_slice() {
                [Operand::Element { .. }] => Some(Form::Tag),
                _ => {
                    self.say("has a tag entry that is not one element");
                    None
                }
            },
            "sum" => {
                if entry.operands.len() < 2 {
                    self.say("has a sum entry of fewer than two elements");
                    None
                } else if entry
                    .operands
                    .iter()
                    .any(|operand| matches!(operand, Operand::Concept(_)))
                {
                    self.say("has a sum entry whose operands are not all elements");
                    None
                } else {
                    Some(Form::Sum)
                }
            }
            "difference" => match entry.operands.as_slice() {
                [Operand::Concept(_), Operand::Element { .. }] => Some(Form::Difference),
                _ => {
                    self.say("has a difference entry that is not one concept less one element");
                    None
                }
            },
            "assert" => {
                self.say(
                    "states an assertion, which belongs to a filer file as its own table and never to an entry",
                );
                None
            }
            _ => {
                self.say(&format!(
                    "has an entry whose form is {form}, and the four are tag, sum, difference and assert"
                ));
                None
            }
        };

        let Some(form) = shaped else {
            return;
        };
        into.push(Rule::stated(
            concept,
            entry.kinds,
            form,
            entry.operands,
            self.words,
        ));
    }

    /// An include, an exclude and an assertion each end where the next table
    /// begins, or where the file does.
    fn take_table(&mut self, table: Table, overrides: &mut Overrides) {
        match table {
            Table::None => {}
            Table::Include => self.take_include(overrides),
            Table::Exclude => self.take_exclude(overrides),
            Table::Assert => self.take_assertion(overrides),
        }
    }

    fn take_include(&mut self, overrides: &mut Overrides) {
        match self.entry.concept {
            Some(concept) => self.take_entry(concept, &mut overrides.included),
            None => {
                // A concept stated and unpublished is already reported; what is
                // left to say is the include that names none at all.
                if !self.entry.concept_stated {
                    self.say("includes a rule for no concept, so nothing says what it reaches");
                }
                self.pending = false;
                self.entry = Entry::default();
            }
        }
    }

    fn take_exclude(&mut self, overrides: &mut Overrides) {
        let excluding = std::mem::take(&mut self.excluding);
        let Some(id) = excluding.id else {
            if !excluding.stated {
                self.say("excludes no rule, so nothing says which one is removed");
            }
            return;
        };
        if self.reads_as_an_id(&id) {
            overrides.excluded.push(id.into());
        }
    }

    fn take_assertion(&mut self, overrides: &mut Overrides) {
        let asserting = std::mem::take(&mut self.asserting);

        if !asserting.concept_stated {
            self.say("states an assertion for no concept");
        }
        if !asserting.period_stated {
            self.say(
                "states an assertion over no period, and an assertion covers the period it names and never every period",
            );
        }
        if !asserting.value_stated {
            self.say("states an assertion with no value");
        }
        if !asserting.source_stated {
            self.say(
                "states an assertion citing no source, which is the one field it cannot be written without",
            );
        }

        let (Some(concept), Some(period), Some(value), Some(source)) = (
            asserting.concept,
            asserting.period,
            asserting.value,
            asserting.source,
        ) else {
            return;
        };

        overrides.asserted.push(Assertion {
            rule: Rule::asserted_id(concept, &overrides.cik, &period, self.words).into(),
            concept,
            period,
            value: value.into(),
            source,
        });
    }

    /// An exclude names a general rule by the id that half renders, and this
    /// reads only as much of it as says which concept and which kinds it is
    /// about. Whether a rule of that id is in the general half is not asked
    /// here: an exclude that names nothing removes nothing.
    fn reads_as_an_id(&mut self, id: &str) -> bool {
        let parts: Vec<&str> = id.split('|').collect();
        if parts.len() != 4 || parts.iter().any(|part| part.is_empty()) {
            self.say(&format!(
                "excludes {id}, which is not the shape a rule id renders in"
            ));
            return false;
        }

        let mut reads = true;
        if self.words.concept_named(parts[0]).is_none() {
            self.say(&format!(
                "excludes a rule for {}, which the published vocabulary does not publish",
                parts[0]
            ));
            reads = false;
        }
        if parts[1] != "*" {
            let scoped: Vec<&str> = parts[1].split(',').collect();
            for named in scoped {
                if self.words.kind_named(named).is_none() {
                    self.say(&format!(
                        "excludes a rule scoped to {named}, which the published vocabulary does not publish"
                    ));
                    reads = false;
                }
            }
        }
        reads
    }
}

/// What a line states, where it states a field: the key, and the value as
/// written.
fn field(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(" = ")?;
    is_bare_name(key).then_some((key, value))
}

fn quoted(value: &str) -> Option<&str> {
    let held = value.strip_prefix('"')?.strip_suffix('"')?;
    (!held.contains('"')).then_some(held)
}

fn is_bare_name(held: &str) -> bool {
    !held.is_empty()
        && held
            .chars()
            .all(|letter| letter.is_ascii_lowercase() || letter == '_')
}

enum Malformed {
    Shape(&'static str),
    Unpublished(String),
}

/// An operand carries its comma even when it is the last one. TOML allows that,
/// and requiring it is what keeps this grammar a subset: a reader that took the
/// comma as optional would accept files no TOML library can.
fn operand(line: &str, words: &Vocabulary) -> Result<Operand, Malformed> {
    let shaped = line
        .strip_suffix(',')
        .and_then(|held| held.strip_prefix('{'))
        .and_then(|held| held.strip_suffix('}'));
    let Some(body) = shaped else {
        return Err(Malformed::Shape(
            "is neither an operand ending in its comma nor the end of the operand list",
        ));
    };

    let mut taxonomy = None;
    let mut tag = None;
    let mut named = None;
    let mut keys = 0;

    for part in body.split(',') {
        let stated = field(part.trim()).and_then(|(key, value)| Some((key, quoted(value)?)));
        let Some((key, value)) = stated else {
            return Err(Malformed::Shape(
                "writes an operand field that is not a name and a quoted value",
            ));
        };
        keys += 1;
        match key {
            "taxonomy" => taxonomy = Some(value),
            "tag" => tag = Some(value),
            "concept" => named = Some(value),
            _ => {
                return Err(Malformed::Shape(
                    "writes an operand field that an operand does not have",
                ));
            }
        }
    }

    match (keys, taxonomy, tag, named) {
        (2, Some(taxonomy), Some(tag), None) => {
            if !is_taxonomy(taxonomy) || !is_tag(tag) {
                return Err(Malformed::Shape(
                    "names an element no taxonomy and tag spell",
                ));
            }
            Ok(Operand::Element {
                taxonomy: taxonomy.into(),
                tag: tag.into(),
            })
        }
        (1, None, None, Some(named)) => match words.concept_named(named) {
            Some(concept) => Ok(Operand::Concept(concept)),
            None => Err(Malformed::Unpublished(named.to_owned())),
        },
        _ => Err(Malformed::Shape(
            "writes an operand that is neither an element nor a concept",
        )),
    }
}

/// An inline table on one line — the shape a period and a source are written in,
/// without the list an operand sits in. A key written twice makes it no reading
/// at all, rather than whichever of the two was written last.
fn inline(value: &str) -> Option<Vec<(&str, &str)>> {
    let body = value.strip_prefix('{')?.strip_suffix('}')?;
    let mut keys: Vec<(&str, &str)> = Vec::new();

    for part in body.split(',') {
        let (key, value) = field(part.trim())?;
        let value = quoted(value)?;
        if keys.iter().any(|(held, _)| *held == key) {
            return None;
        }
        keys.push((key, value));
    }

    (!keys.is_empty()).then_some(keys)
}

/// The CIK the fetch boundary keys its retrieval by: ten digits, left-padded
/// with zeros, and never a ticker, which is reassigned between companies and
/// does not cross the boundary at all.
pub(super) fn is_cik(held: &str) -> bool {
    held.len() == 10 && held.chars().all(|digit| digit.is_ascii_digit())
}

/// An accession, in the shape the document publishes it. The shape is the whole
/// of what is checkable about a citation here.
fn is_accession(held: &str) -> bool {
    let mut parts = held.split('-');
    let digits = |part: Option<&str>, count: usize| {
        part.is_some_and(|part| {
            part.len() == count && part.chars().all(|held| held.is_ascii_digit())
        })
    };
    digits(parts.next(), 10)
        && digits(parts.next(), 2)
        && digits(parts.next(), 6)
        && parts.next().is_none()
}

fn is_line(held: &str) -> bool {
    !held.is_empty() && !held.starts_with('0') && held.chars().all(|digit| digit.is_ascii_digit())
}

/// A date as the characters the boundary publishes, which is what lets two of
/// them be compared as they are written.
fn is_date(held: &str) -> bool {
    let digits = |part: &str, count: usize| {
        part.len() == count && part.chars().all(|held| held.is_ascii_digit())
    };
    let mut parts = held.split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    if !digits(year, 4) || !digits(month, 2) || !digits(day, 2) {
        return false;
    }
    let month: u32 = month.parse().unwrap_or_default();
    let day: u32 = day.parse().unwrap_or_default();
    (1..=12).contains(&month) && (1..=31).contains(&day)
}

/// The decimal literal a filing publishes, unparsed: a binary float is a lossy
/// reading of one, and nothing here needs its value.
fn is_decimal(held: &str) -> bool {
    let held = held.strip_prefix('-').unwrap_or(held);
    let digits = |part: &str| !part.is_empty() && part.chars().all(|held| held.is_ascii_digit());
    let mut parts = held.split('.');
    let whole = parts.next().unwrap_or_default();
    match parts.next() {
        None => digits(whole),
        Some(fraction) => digits(whole) && digits(fraction) && parts.next().is_none(),
    }
}

fn is_taxonomy(held: &str) -> bool {
    let mut letters = held.chars();
    letters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && letters.all(|held| held.is_ascii_lowercase() || held.is_ascii_digit() || held == '-')
}

fn is_tag(held: &str) -> bool {
    let mut letters = held.chars();
    letters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && letters.all(|held| held.is_ascii_alphanumeric())
}
