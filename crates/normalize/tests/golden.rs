//! The golden fixture harness for the normalize stage.
//!
//! A fixture is a directory under `fixtures/normalize/`, named for the merged
//! fetch fixture whose facts it reads. Its input is that fixture's filer,
//! whole, read out of `fixtures/fetch/<name>/expected` rather than copied here,
//! so the two cannot part, and a name with no fetch fixture behind it fails.
//! It runs through the stage under the committed registry, and `expected` is
//! what must come out, byte for byte. A later fixture is added by adding its
//! directory and nothing else.
//!
//! Beside `expected` sits `claims`: what the accepted records and the facts
//! say the rendering must hold, derived by hand, as
//! `docs/adr/golden-claims-and-baseline-re-recording.md` has it. The claims
//! are the proof and `expected` is the guard over every other line, recorded
//! from the stage only once every claim held against it. The `| ` lines of
//! `claims` are checked against `expected` here, as
//! `docs/adr/the-harness-checks-the-claims.md` decides, by the `claims` module
//! that only this target declares; the prose around them is still re-derived
//! by hand.
//!
//! `expected` is never written by the code it checks — an expected result the
//! subject generated proves only that the subject agrees with itself.
//!
//! `cargo test` does not select this target (`test = false` in the manifest);
//! `scripts/gates.sh` runs it by name. AGENTS.md counts "all tests pass" and
//! "the golden fixtures still produce their expected results" as two gates, and
//! two gates have to be able to go red apart. Run under the test gate a
//! mismatch here would report that gate failing, and the fixtures gate would be
//! a name that can never go red on its own — which is the thing a proof of
//! catch exists to rule out.

mod claims;
mod fixture;

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use fixture::{committed, read as registry, recorded_in};

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

fn expected(case: &Path) -> String {
    let path = case.join("expected");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: a fixture holds an expected ({e})", path.display()))
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

/// Every fixture directory, and never none: the baseline is committed, so no
/// case here means the baseline was deleted, and a gate that passes over
/// nothing reads exactly like one that is holding.
fn every_case() -> Vec<PathBuf> {
    let dir = stage_fixtures();
    let cases = cases(&dir);
    assert!(
        !cases.is_empty(),
        "{}: holds no fixture, so this gate checks nothing",
        dir.display()
    );
    cases
}

#[test]
fn every_fixture_produces_its_expected_result() {
    let cases = every_case();

    let registry = registry(&committed());
    let mut produced = String::new();
    let mut failures = String::new();

    for case in &cases {
        let name = case.file_name().unwrap_or_default().to_string_lossy();
        let filer = recorded_in(&name);
        let expected = expected(case);

        produced.clear();
        let failure = match vfi_normalize::normalize(&registry, &filer, &mut produced) {
            Err(undated) => Some(format!("the stage stopped: {undated}")),
            Ok(()) if produced != expected => Some(difference(&expected, &produced)),
            Ok(()) => None,
        };
        if let Some(failure) = failure {
            failures.push_str(&format!("    fixtures/{STAGE}/{name}: {failure}\n"));
        }
    }

    assert!(
        failures.is_empty(),
        "the stage no longer produces what these fixtures expect:\n{failures}"
    );
}

/// A test of its own, so that it runs and reports whether or not the stage
/// still produces `expected`: the two failures say different things are wrong.
#[test]
fn every_fixture_holds_its_claims() {
    let mut failures = String::new();

    for case in &every_case() {
        let name = case.file_name().unwrap_or_default().to_string_lossy();
        let at = format!("fixtures/{STAGE}/{name}/claims");
        let expected = expected(case);

        let claims = match fs::read_to_string(case.join("claims")) {
            Ok(claims) => Some(claims),
            Err(e) if e.kind() == ErrorKind::NotFound => None,
            Err(e) => {
                failures.push_str(&format!("    {at}: cannot be read ({e})\n"));
                continue;
            }
        };

        for failure in claims::check(claims.as_deref(), &expected) {
            match failure.line {
                Some(line) => failures.push_str(&format!(
                    "    {at}:{line}: {}\n      {:?}\n",
                    failure.reason, failure.text
                )),
                None => failures.push_str(&format!("    {at}: {}\n", failure.reason)),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "these claims do not hold against their expected:\n{failures}"
    );
}
