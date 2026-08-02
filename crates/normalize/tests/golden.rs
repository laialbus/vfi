//! The golden fixture harness for the normalize stage.
//!
//! A fixture is a directory under `fixtures/normalize/`. `input` is what goes
//! into the stage and `expected` is what must come out; both are data, and
//! neither is ever written by the code it checks — an expected result the
//! subject generated proves only that the subject agrees with itself. The
//! directory's name says what the case pins.
//!
//! `cargo test` does not select this target (`test = false` in the manifest);
//! `scripts/gates.sh` runs it by name. AGENTS.md counts "all tests pass" and
//! "the golden fixtures still produce their expected results" as two gates, and
//! two gates have to be able to go red apart. Run under the test gate a
//! mismatch here would report that gate failing, and the fixtures gate would be
//! a name that can never go red on its own — which is the thing a proof of
//! catch exists to rule out.

use std::fs;
use std::path::{Path, PathBuf};

/// `fixtures/<stage>/` is this harness's half of `fixtures/`, and it is what
/// scripts/gates.sh reads to decide which harnesses to run. A stage directory
/// with no harness to match is a fixture nobody runs, so the gate goes red
/// there rather than here.
const STAGE: &str = "normalize";

fn stage_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(STAGE)
}

/// Every fixture directory, in name order: `read_dir` yields whatever order the
/// filesystem holds them in, and a failure list that reorders between two runs
/// of the same tree is one nobody can diff.
fn cases(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: the fixtures cannot be read ({e})", dir.display()));

    let mut cases: Vec<PathBuf> = entries
        .map(|entry| {
            entry
                .unwrap_or_else(|e| panic!("{}: an entry cannot be read ({e})", dir.display()))
                .path()
        })
        .filter(|path| path.is_dir())
        .collect();

    cases.sort();
    cases
}

fn read(case: &Path, name: &str) -> String {
    let path = case.join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("{}: a fixture holds an input and an expected ({e})", path.display())
    })
}

/// The first line the two disagree on, rather than both files whole. A golden
/// failure is read by someone deciding which of the two is wrong, and that
/// decision is made at the difference.
fn difference(expected: &str, produced: &str) -> String {
    let mut expected = expected.lines();
    let mut produced = produced.lines();
    let mut number = 1;

    loop {
        match (expected.next(), produced.next()) {
            (None, None) => {
                return "every line matches, so the two differ in what follows the last one"
                    .to_owned();
            }
            (e, p) if e == p => number += 1,
            (e, p) => {
                return format!(
                    "line {number}\n      expected: {}\n      produced: {}",
                    show(e),
                    show(p)
                );
            }
        }
    }
}

/// Quoted and escaped, because a difference that is only trailing whitespace is
/// invisible printed bare and is exactly the kind a golden gate catches.
fn show(line: Option<&str>) -> String {
    match line {
        Some(line) => format!("{line:?}"),
        None => "end of file".to_owned(),
    }
}

#[test]
fn every_fixture_produces_its_expected_result() {
    let dir = stage_fixtures();
    let cases = cases(&dir);

    // An empty sweep is not an absence: the baseline is committed, so no case
    // here means the baseline was deleted, and a gate that passes over nothing
    // reads exactly like one that is holding.
    assert!(
        !cases.is_empty(),
        "{}: holds no fixture, so this gate checks nothing",
        dir.display()
    );

    let mut produced = String::new();
    let mut failures = String::new();

    for case in &cases {
        let input = read(case, "input");
        let expected = read(case, "expected");

        produced.clear();
        vfi_normalize::normalize(&input, &mut produced);

        if produced != expected {
            let name = case.file_name().unwrap_or_default().to_string_lossy();
            failures.push_str(&format!(
                "    fixtures/{STAGE}/{name}: {}\n",
                difference(&expected, &produced)
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "the stage no longer produces what these fixtures expect:\n{failures}"
    );
}
