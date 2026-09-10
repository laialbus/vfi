//! The facts one filer actually reported, and the registry the cases read them
//! against.
//!
//! Every fact a case in this directory uses comes out of
//! `fixtures/fetch/every-fact-a-filer-reported`, read from what the fetch
//! harness pins rather than written to suit a case. Facts composed for a case
//! would have the shape the case wanted, and the cases that matter — a filing
//! carrying three durations of one element, two of them the same number; a
//! cover-dated share count that never falls at a period end; a gross profit both
//! candidates answer — are all ones nobody would think to write down.
//!
//! The rules come from the registry the same way: the committed one where it has
//! an entry for the case, and a copy of it with one entry added or one line
//! changed where it does not. A copy is per case, because cargo runs these at
//! the same time.
//!
//! Both test binaries beside this read it, and each uses some of it. An item one
//! of them does not reach is not dead, so the lint that would say so is off
//! here and nowhere else.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use vfi_contracts::fetch_normalize::{Fact, Period};
use vfi_normalize::registry::Registry;

/// The fixture case, and the filer it recorded.
pub const CASE: &str = "every-fact-a-filer-reported";
pub const FILER: &str = "0002003750";

/// The 10-K filed 2024-11-25: the filing that carries three durations of
/// `NetIncomeLoss` at once, a cover-page count three months after its year end,
/// and one element in two currencies.
pub const ANNUAL: &str = "0001213900-24-101777";

/// The 10-Q filed 2024-08-13, which reports a quarter and the year to date that
/// contains it, under the same elements.
pub const QUARTERLY: &str = "0001213900-24-067900";

pub fn committed() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry")
}

/// A copy of the committed registry, to be added to by the case that asked for
/// it. Named per case, because cargo runs these at the same time.
pub fn planted(case: &str) -> PathBuf {
    let at = Path::new(env!("CARGO_TARGET_TMPDIR")).join(case);
    let _ = fs::remove_dir_all(&at);
    copy(&committed(), &at);
    at
}

fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("the scratch copy can be created");
    for entry in fs::read_dir(from).expect("the registry can be read") {
        let entry = entry.expect("an entry of the registry can be read");
        let held = entry.path();
        let there = to.join(entry.file_name());
        if held.is_dir() {
            copy(&held, &there);
        } else {
            fs::copy(&held, &there).expect("a file of the registry can be copied");
        }
    }
}

/// One entry added to a concept's file, leaving everything already in it alone.
pub fn adding(root: &Path, concept: &str, entry: &str) {
    appended(
        &root.join("concepts").join(format!("{concept}.toml")),
        entry,
    );
}

/// One table added to the filer's own file, which is where an exclusion and an
/// assertion are written.
pub fn overriding(root: &Path, table: &str) {
    appended(&root.join("filers").join(format!("{FILER}.toml")), table);
}

fn appended(path: &Path, text: &str) {
    let mut held = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: cannot be read ({e})", path.display()));
    held.push_str(text);
    fs::write(path, held).unwrap_or_else(|e| panic!("{}: cannot be written ({e})", path.display()));
}

pub fn rewritten(root: &Path, concept: &str, from: &str, to: &str) {
    let path = root.join("concepts").join(format!("{concept}.toml"));
    let text = fs::read_to_string(&path).expect("the concept's file can be read");
    assert!(
        text.contains(from),
        "{}: does not state `{from}`, so the case changed nothing",
        path.display()
    );
    fs::write(&path, text.replace(from, to)).expect("the concept's file can be written");
}

pub fn read(root: &Path) -> Registry {
    Registry::read_from(root).unwrap_or_else(|why| panic!("{why}"))
}

pub fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
}

pub fn duration(start: &str, end: &str) -> Period {
    Period::Duration {
        start: start.into(),
        end: end.into(),
    }
}

/// Every fact the fixture records, read out of what the fetch harness pins.
///
/// The count that file states is compared against the facts read out of it, so
/// a rendering this reader stopped understanding fails here rather than quietly
/// leaving a case with fewer facts than the fixture holds.
pub fn reported() -> Vec<Fact> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/fetch")
        .join(CASE)
        .join("expected");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: cannot be read ({e})", path.display()));

    let mut stated = None;
    let mut facts = Vec::new();
    for line in text.lines() {
        if let Some(count) = line.strip_prefix("facts ") {
            stated = Some(
                count
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("{}: states `facts {count}`", path.display())),
            );
        } else if let Some(fact) = line.strip_prefix("    ") {
            facts.push(read_fact(fact, &path));
        }
    }

    assert_eq!(
        Some(facts.len()),
        stated,
        "{}: states one count of facts and renders another",
        path.display()
    );
    facts
}

fn read_fact(line: &str, path: &Path) -> Fact {
    let word: Vec<&str> = line.split_whitespace().collect();
    let stated = |at: usize| -> &str {
        word.get(at)
            .unwrap_or_else(|| panic!("{}: cannot be read as a fact: {line}", path.display()))
    };

    let (period, rest) = match stated(3) {
        "at" => (instant(stated(4)), 5),
        "from" => (duration(stated(4), stated(6)), 7),
        held => panic!("{}: states a period as `{held}`: {line}", path.display()),
    };
    assert_eq!(
        [stated(rest), stated(rest + 2), stated(rest + 5)],
        ["value", "in", "filed"],
        "{}: cannot be read as a fact: {line}",
        path.display()
    );
    assert_eq!(
        word.len(),
        rest + 7,
        "{}: cannot be read as a fact: {line}",
        path.display()
    );

    Fact {
        taxonomy: stated(0).into(),
        tag: stated(1).into(),
        unit: stated(2).into(),
        period,
        value: stated(rest + 1).into(),
        accession: stated(rest + 3).into(),
        form: stated(rest + 4).into(),
        filed: stated(rest + 6).into(),
    }
}

/// The facts of one filing, which are the facts carrying its accession.
pub fn filing<'f>(facts: &'f [Fact], accession: &str) -> Vec<&'f Fact> {
    let held: Vec<&Fact> = facts
        .iter()
        .filter(|fact| &*fact.accession == accession)
        .collect();
    assert!(
        !held.is_empty(),
        "the fixture records no filing {accession}"
    );
    held
}
