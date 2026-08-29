//! The fetch → normalize contract at v1: what fetch hands normalize.
//!
//! `contracts/fetch-normalize/v1.toml` is the surface and it is frozen. This is
//! that surface as Rust — one value per filer, holding the two fields the
//! retrieval is identified by and every fact the document publishes for it,
//! each fact carrying its eight. Why each field crosses and what was left out
//! is `docs/adr/fetch-normalize-contract.md`, and none of that argument is
//! repeated here.
//!
//! Every value crosses as the characters the document publishes. Nothing here
//! parses a decimal or a date: the parse is a reading, and a reading made
//! before normalize is one normalize cannot check or correct.
//!
//! The contract also states three properties — that fetch filters nothing, that
//! nothing is parsed, and that five fields identify a fact. Those are about the
//! values that cross rather than the shape of the type, so no type states them
//! and nothing here checks them; the record above routes them to a golden
//! fixture that does not exist yet.

carries! {
    /// One filer's facts, which is what one retrieval produces.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Filer {
        fields {
            /// The filer the document is about, as the document states it.
            pub cik: Box<str>,
            /// The request the document came back from — recorded by the
            /// retrieval rather than published by the document, and held once
            /// per document rather than once per fact.
            pub retrieved_from: Box<str>,
        }
        and {
            /// Every fact the document publishes for this filer.
            ///
            /// Everything it publishes: not filtered by tag, by taxonomy or by
            /// form, because normalize can read a filer's silence only if what
            /// it holds is everything the filer reported.
            pub facts: Vec<Fact>,
        }
    }
}

carries! {
    /// One fact the document publishes, with where it was reported.
    ///
    /// The last three repeat across every fact one filing reported, and they
    /// repeat on purpose: a fact that pointed at its filing instead could point
    /// at one that is not there, and a value whose filing cannot be named is
    /// the loss this contract exists to prevent.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Fact {
        fields {
            /// The taxonomy the element is defined in: `us-gaap`, `dei`, and
            /// whatever else the document publishes.
            pub taxonomy: Box<str>,
            /// The element name, as the document publishes it.
            pub tag: Box<str>,
            /// The unit key: `USD`, `shares`, `USD/shares`.
            pub unit: Box<str>,
            /// The fact's own period — not the period of the report it
            /// appeared in.
            pub period: Period,
            /// The amount, as the decimal literal the document publishes.
            ///
            /// Unparsed. A conversion made here is one normalize cannot check
            /// or undo, and a binary float is a lossy one.
            pub value: Box<str>,
            /// The accession number of the filing the fact was reported in,
            /// which is not the document it was read out of.
            pub accession: Box<str>,
            /// That filing's form type: `10-K`, `10-K/A`. The only field that
            /// tells an original from an amendment.
            pub form: Box<str>,
            /// The date that filing was received, which is what orders two
            /// reports of one period.
            pub filed: Box<str>,
        }
    }
}

shapes! {
    /// The period a fact is stated for: one of these, never both and never
    /// neither.
    ///
    /// Dates cross as the characters the document publishes, like every other
    /// value here.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Period {
        /// The date the fact is stated at.
        Instant { at: Box<str> },
        /// The date the fact's period starts and the date it ends.
        Duration { start: Box<str>, end: Box<str> },
    }
}

/// The types above against the bytes they transcribe.
///
/// The contracts gate digests `v1.toml` and never reads it, so a type that
/// drifted from the surface it states would stay green on every gate this
/// repository had before this one. This is the comparison that closes that: a
/// field renamed, added or dropped on either side leaves the two readings
/// unequal, and the failure prints both.
#[cfg(test)]
mod states_what_is_published {
    use super::{Fact, Filer, Period};

    /// The file this module states, relative to the repository root, named
    /// here and nowhere else in this module.
    const PATH: &str = "contracts/fetch-normalize/v1.toml";

    /// The bytes the contracts gate freezes, read from the crate's own
    /// directory rather than the working one, so what the check reads does not
    /// depend on where the runner was standing.
    fn published() -> String {
        let path = format!("{}/../../{PATH}", env!("CARGO_MANIFEST_DIR"));
        std::fs::read_to_string(&path)
            .unwrap_or_else(|why| panic!("{PATH} could not be read: {why}"))
    }

    /// Every occurrence of the array-of-tables `table`, each as the key/value
    /// pairs it states, in the order the file states them.
    ///
    /// Enough TOML to read a frozen file of a known shape, and no more: full
    /// lines, a `#` only at the start of one, and one value per key. Anything
    /// it does not understand inside a table it is reading stops the test,
    /// because a reader that skipped a line it could not parse would agree with
    /// a type that had dropped the same field.
    fn occurrences<'a>(published: &'a str, table: &str) -> Vec<Vec<(&'a str, &'a str)>> {
        let header = format!("[[{table}]]");
        let mut found: Vec<Vec<(&str, &str)>> = Vec::new();
        let mut inside = false;

        for line in published.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') {
                inside = line == header;
                if inside {
                    found.push(Vec::new());
                }
                continue;
            }
            if !inside {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                panic!("{PATH} states `{line}` under [[{table}]], which is not a key and a value");
            };
            found
                .last_mut()
                .expect("a table was entered before its keys were read")
                .push((key.trim(), value.trim()));
        }

        assert!(
            !found.is_empty(),
            "{PATH} states no [[{table}]], so this reading of it is unusable"
        );
        found
    }

    fn value_of<'a>(pairs: &[(&'a str, &'a str)], key: &str, table: &str) -> &'a str {
        let mut stated = pairs.iter().filter(|(name, _)| *name == key);
        let value = stated
            .next()
            .unwrap_or_else(|| panic!("an entry under [[{table}]] in {PATH} states no `{key}`"))
            .1;
        assert!(
            stated.next().is_none(),
            "an entry under [[{table}]] in {PATH} states `{key}` more than once"
        );
        value
    }

    fn unquoted(value: &str) -> &str {
        value
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
            .unwrap_or_else(|| panic!("{PATH} states `{value}` where a quoted string was expected"))
    }

    /// The `name` of every entry under `table`, in the order the file states
    /// them.
    fn names_under<'a>(published: &'a str, table: &str) -> Vec<&'a str> {
        occurrences(published, table)
            .iter()
            .map(|pairs| unquoted(value_of(pairs, "name", table)))
            .collect()
    }

    #[test]
    fn a_filer_carries_the_fields_the_published_bytes_name() {
        let published = published();
        assert_eq!(
            Filer::FIELDS,
            names_under(&published, "filer.field").as_slice(),
            "the fields `Filer` declares are not the [[filer.field]] names {PATH} states"
        );
    }

    #[test]
    fn a_fact_carries_the_fields_the_published_bytes_name() {
        let published = published();
        assert_eq!(
            Fact::FIELDS,
            names_under(&published, "fact.field").as_slice(),
            "the fields `Fact` declares are not the [[fact.field]] names {PATH} states"
        );
    }

    /// That a period is one shape and never both and never neither is the
    /// enum's, and the compiler holds it. What is compared here is which shapes
    /// there are and how many dates each takes.
    ///
    /// A shape's published name is its variant name lowercased, which is the
    /// whole of the translation between the two spellings. A name the rule does
    /// not cover leaves the readings unequal, which is the direction to be
    /// wrong in: it reports a difference that is only a spelling, and never
    /// passes over one that is not.
    #[test]
    fn a_period_takes_the_shapes_the_published_bytes_name() {
        let declared: Vec<(String, usize)> = Period::SHAPES
            .iter()
            .map(|(shape, dates)| (shape.to_ascii_lowercase(), *dates))
            .collect();

        let stated: Vec<(String, usize)> = occurrences(&published(), "period.shape")
            .iter()
            .map(|pairs| {
                let name = unquoted(value_of(pairs, "name", "period.shape")).to_owned();
                let dates = value_of(pairs, "dates", "period.shape");
                let dates = dates.parse().unwrap_or_else(|_| {
                    panic!("{PATH} states `dates = {dates}`, which is no count")
                });
                (name, dates)
            })
            .collect();

        assert_eq!(
            declared, stated,
            "the shapes `Period` declares are not the [[period.shape]] entries {PATH} states"
        );
    }
}
