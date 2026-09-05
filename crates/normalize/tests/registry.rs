//! What the one way to the registry answers, over the registry this repository
//! holds and over registries planted from it.
//!
//! Every planted case starts as a copy of the committed one and changes the one
//! thing the case is about, so a case that refuses refuses for its own reason
//! rather than for something else that was missing from a registry written by
//! hand. The committed registry is also the one thing holding this reader and
//! the gate over the data together: both read the same bytes under the same
//! grammar, and the first test below is what goes red if they part.

use std::fs;
use std::path::{Path, PathBuf};

use vfi_contracts::canonical_concepts::{Concept, Kind};
use vfi_contracts::fetch_normalize::Period;
use vfi_normalize::registry::{Outcome, Registry};

const APPLE: &str = "0000320193";

fn committed() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry")
}

/// A copy of the committed registry, to be changed by the case that asked for
/// it. Named per case, because cargo runs these at the same time.
fn planted(case: &str) -> PathBuf {
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

fn write(root: &Path, at: &str, text: &str) {
    let path = root.join(at);
    fs::create_dir_all(
        path.parent()
            .expect("a file in the registry sits in a directory"),
    )
    .expect("the directory can be created");
    fs::write(path, text).expect("the file can be written");
}

fn erase(root: &Path, at: &str) {
    fs::remove_file(root.join(at)).expect("the file can be removed");
}

fn instant(at: &str) -> Period {
    Period::Instant { at: at.into() }
}

fn read(root: &Path) -> Registry {
    Registry::read_from(root).unwrap_or_else(|why| panic!("{why}"))
}

/// The ids of the eligible rules, which is what a set of rules is compared by:
/// the id renders exactly the fields that make a rule that rule.
fn eligible<'a>(outcome: &'a Outcome<'a>) -> Vec<&'a str> {
    match outcome {
        Outcome::Eligible(rules) => rules.iter().map(|rule| rule.id()).collect(),
        Outcome::Asserted(held) => panic!(
            "expected eligible rules, and got the assertion {}",
            held.rule()
        ),
    }
}

/// Every reason a registry was refused, joined so a case can look for its own.
fn refused(case: &str, planting: impl FnOnce(&Path)) -> String {
    let root = planted(case);
    planting(&root);
    match Registry::read_from(&root) {
        Ok(_) => panic!("{case}: loaded, and this registry is one the gate would refuse"),
        Err(why) => why.problems().join("\n"),
    }
}

#[test]
fn the_committed_registry_is_one_this_reader_accepts() {
    let registry = read(&committed());

    let answer = registry.answer(
        APPLE,
        Some(Kind::Operating),
        Concept::Inventory,
        &instant("2024-09-28"),
    );
    assert_eq!(
        eligible(&answer.outcome),
        ["inventory|*|tag|element:us-gaap:InventoryNet"]
    );
}

#[test]
fn a_rule_carries_the_id_its_own_fields_render() {
    let registry = read(&committed());

    let answer = registry.answer(
        APPLE,
        Some(Kind::Operating),
        Concept::LongTermDebt,
        &instant("2024-09-28"),
    );
    assert_eq!(
        eligible(&answer.outcome),
        [
            "long_term_debt|*|sum|element:us-gaap:LongTermDebtNoncurrent+element:us-gaap:FinanceLeaseLiabilityNoncurrent",
            "long_term_debt|*|tag|element:us-gaap:FinanceLeaseLiabilityNoncurrent",
            "long_term_debt|*|tag|element:us-gaap:LongTermDebtNoncurrent",
        ]
    );
}

/// The scope is a set, so it renders sorted rather than in the order written,
/// and a rule that reaches a concept for two kinds has one id either way.
#[test]
fn a_scoped_rule_renders_the_kinds_it_admits() {
    let registry = read(&committed());

    let answer = registry.answer(
        APPLE,
        Some(Kind::Bank),
        Concept::Revenue,
        &instant("2024-09-28"),
    );
    assert_eq!(
        eligible(&answer.outcome),
        [
            "revenue|bank|sum|element:us-gaap:InterestIncomeExpenseNet+element:us-gaap:NoninterestIncome",
            "revenue|bank|tag|element:us-gaap:RevenuesNetOfInterestExpense",
        ]
    );
}

/// An entry with no scope applies to every filer, including one whose kind has
/// not been established; a scoped one applies to no filer whose kind is unknown.
/// That is what makes an unassigned filer's revenue safe and imprecise rather
/// than a bank's revenue read under an operating filer's meaning.
#[test]
fn a_filer_with_no_kind_reaches_what_no_scope_holds_back() {
    let registry = read(&committed());
    let at = instant("2024-09-28");

    let unscoped = registry.answer(APPLE, None, Concept::TotalAssets, &at);
    assert_eq!(
        eligible(&unscoped.outcome),
        ["total_assets|*|tag|element:us-gaap:Assets"]
    );

    let scoped = registry.answer(APPLE, None, Concept::Revenue, &at);
    assert!(eligible(&scoped.outcome).is_empty());
}

/// A concept whose entries this filer's kind admits none of, and a concept the
/// registry holds no entry for at all, are one answer: an attempt that ran and
/// found nothing. Neither is `NotApplicable`, which this interface cannot
/// construct — the vocabulary makes that state from a filer kind and an
/// applicability clause that excludes it, and from nothing else.
#[test]
fn no_eligible_rule_and_no_entry_at_all_are_the_same_answer() {
    let root = planted("no-entry-at-all");
    write(
        &root,
        "concepts/inventory.toml",
        "[unreachable]\nreason = \"nothing this case is about\"\n",
    );
    let registry = read(&root);
    let at = instant("2024-09-28");

    let none_eligible = registry.answer(APPLE, None, Concept::Revenue, &at);
    let no_entry = registry.answer(APPLE, Some(Kind::Operating), Concept::Inventory, &at);

    assert_eq!(none_eligible.outcome, no_entry.outcome);
    assert_eq!(none_eligible.outcome, Outcome::Eligible(Vec::new()));
}

/// The eligible set is the general set for the filer's kind, less what its file
/// excludes, plus what it includes — and the two compose to one set however they
/// are written, so there is no precedence between them to settle.
#[test]
fn include_and_exclude_compose_the_same_in_either_order() {
    const EXCLUDE: &str = "\
[[exclude]]
id = \"long_term_debt|*|tag|element:us-gaap:FinanceLeaseLiabilityNoncurrent\"
";
    const INCLUDE: &str = "\
[[include]]
concept = \"long_term_debt\"
form = \"tag\"
operands = [
  { taxonomy = \"us-gaap\", tag = \"LongTermDebtNoncurrentCustom\" },
]
";
    let heading = format!("cik = \"{APPLE}\"\nkind = \"operating\"\n\n");
    let at = instant("2024-09-28");

    let one = planted("exclude-then-include");
    write(
        &one,
        &format!("filers/{APPLE}.toml"),
        &format!("{heading}{EXCLUDE}\n{INCLUDE}"),
    );
    let other = planted("include-then-exclude");
    write(
        &other,
        &format!("filers/{APPLE}.toml"),
        &format!("{heading}{INCLUDE}\n{EXCLUDE}"),
    );

    let written_one_way = read(&one);
    let written_the_other = read(&other);
    let answer = written_one_way.answer(APPLE, Some(Kind::Operating), Concept::LongTermDebt, &at);
    let same = written_the_other.answer(APPLE, Some(Kind::Operating), Concept::LongTermDebt, &at);

    assert_eq!(
        eligible(&answer.outcome),
        [
            "long_term_debt|*|sum|element:us-gaap:LongTermDebtNoncurrent+element:us-gaap:FinanceLeaseLiabilityNoncurrent",
            "long_term_debt|*|tag|element:us-gaap:LongTermDebtNoncurrent",
            "long_term_debt|*|tag|element:us-gaap:LongTermDebtNoncurrentCustom",
        ]
    );
    assert_eq!(eligible(&answer.outcome), eligible(&same.outcome));
}

/// Where an assertion covers the filer, the concept and the period, that value
/// is the answer, no eligible-rule lookup runs, and what comes back carries the
/// assertion's own rule id — so an asserted number is visibly asserted rather
/// than indistinguishable from a read one.
#[test]
fn an_assertion_answers_for_the_period_it_covers_and_no_other() {
    let root = planted("an-assertion");
    write(
        &root,
        &format!("filers/{APPLE}.toml"),
        &format!(
            "\
cik = \"{APPLE}\"
kind = \"operating\"

[[assert]]
concept = \"long_term_debt\"
period = {{ instant = \"2024-09-28\" }}
value = \"0\"
source = {{ accession = \"0000320193-24-000123\", line = \"42\" }}
"
        ),
    );
    let registry = read(&root);

    let covered = registry.answer(
        APPLE,
        Some(Kind::Operating),
        Concept::LongTermDebt,
        &instant("2024-09-28"),
    );
    let Outcome::Asserted(asserted) = covered.outcome else {
        panic!("the assertion covers this period, so it is the answer");
    };
    assert_eq!(asserted.value(), "0");
    assert_eq!(
        asserted.rule(),
        "long_term_debt|*|assert|filer:0000320193+instant:2024-09-28"
    );
    assert_eq!(asserted.source().accession(), "0000320193-24-000123");
    assert_eq!(asserted.source().line(), "42");

    let elsewhere = registry.answer(
        APPLE,
        Some(Kind::Operating),
        Concept::LongTermDebt,
        &instant("2023-09-30"),
    );
    assert_eq!(eligible(&elsewhere.outcome).len(), 3);
}

/// The kind half of the registry is assertion-only at v1: a filer has the kind
/// its file gives it, and a filer with no file has none.
#[test]
fn a_filer_has_the_kind_its_own_file_assigns_it() {
    let root = planted("a-kind");
    write(
        &root,
        &format!("filers/{APPLE}.toml"),
        &format!("cik = \"{APPLE}\"\nkind = \"operating\"\n"),
    );
    let registry = read(&root);

    assert_eq!(registry.kind_of(APPLE), Some(Kind::Operating));
    assert_eq!(registry.kind_of("0001018724"), None);
}

/// Every answer carries the version that gave it. A resolved value records the
/// pair of version and rule id and never the id alone, because the same id under
/// two versions may name different bytes.
#[test]
fn every_answer_names_the_version_that_gave_it() {
    let registry = read(&committed());
    let answer = registry.answer(APPLE, None, Concept::TotalAssets, &instant("2024-09-28"));

    assert_eq!(answer.version, registry.version());
    assert_eq!(answer.version.rendered().len(), 64);
    assert!(
        answer
            .version
            .rendered()
            .chars()
            .all(|held| held.is_ascii_hexdigit() && !held.is_ascii_uppercase())
    );
}

/// One tree digests one way — twice over the same tree, and again over a copy of
/// it somewhere else, since where a tree is read from is not part of what it is.
#[test]
fn one_tree_always_digests_one_way() {
    let twice = read(&committed()).version();
    let elsewhere = read(&planted("digested-elsewhere")).version();

    assert_eq!(read(&committed()).version(), twice);
    assert_eq!(elsewhere, twice);
}

#[test]
fn a_changed_byte_changes_the_version() {
    let root = planted("a-changed-byte");
    let before = read(&root).version();

    let held = root.join("concepts/inventory.toml");
    let text = fs::read_to_string(&held).expect("the file can be read");
    fs::write(&held, format!("{text}\n# a byte this tree did not have\n"))
        .expect("the file can be written");

    assert_ne!(read(&root).version(), before);
}

/// A byte moved from one file to another is a different tree, which is why the
/// paths are in the digest and not only the bytes.
#[test]
fn the_same_bytes_under_other_names_are_another_version() {
    let root = planted("the-same-bytes-moved");
    let before = read(&root).version();

    let one = root.join("concepts/inventory.toml");
    let other = root.join("concepts/total_assets.toml");
    let held = fs::read_to_string(&one).expect("the file can be read");
    fs::write(
        &one,
        fs::read_to_string(&other).expect("the file can be read"),
    )
    .expect("the file can be written");
    fs::write(&other, held).expect("the file can be written");

    assert_ne!(read(&root).version(), before);
}

/// A registry the gate would refuse fails to load, and the stage does not run.
/// It never degrades to an empty eligible set: that surfaces as a concept
/// nothing reached, and a data defect hidden behind a state meaning a filing was
/// consulted is worse than a stop.
#[test]
fn a_registry_the_gate_would_refuse_does_not_load() {
    let held = refused("a-form-the-format-does-not-have", |root| {
        write(
            root,
            "concepts/inventory.toml",
            "[[entry]]\nform = \"average\"\noperands = [\n  { taxonomy = \"us-gaap\", tag = \"InventoryNet\" },\n]\n",
        );
    });
    assert!(held.contains("whose form is average"), "{held}");

    let held = refused("an-entry-field-an-entry-does-not-have", |root| {
        write(
            root,
            "concepts/inventory.toml",
            "[[entry]]\nform = \"tag\"\nprefer = \"yes\"\noperands = [\n  { taxonomy = \"us-gaap\", tag = \"InventoryNet\" },\n]\n",
        );
    });
    assert!(held.contains("writes the field prefer"), "{held}");

    let held = refused("one-id-for-two-rules", |root| {
        let entry = "[[entry]]\nform = \"tag\"\noperands = [\n  { taxonomy = \"us-gaap\", tag = \"InventoryNet\" },\n]\n";
        write(
            root,
            "concepts/inventory.toml",
            &format!("{entry}\n{entry}"),
        );
    });
    assert!(held.contains("renders one id for two rules"), "{held}");

    let held = refused("a-concept-accounted-for-neither-way", |root| {
        erase(root, "concepts/inventory.toml");
    });
    assert!(
        held.contains("inventory: is accounted for neither way"),
        "{held}"
    );

    let held = refused("a-concept-the-vocabulary-does-not-publish", |root| {
        write(
            root,
            "concepts/cost_of_revenue.toml",
            "[[entry]]\nform = \"tag\"\noperands = [\n  { taxonomy = \"us-gaap\", tag = \"CostOfRevenue\" },\n]\n",
        );
    });
    assert!(
        held.contains("names cost_of_revenue, which the published vocabulary does not publish"),
        "{held}"
    );

    let held = refused("a-concept-file-one-directory-deeper", |root| {
        write(
            root,
            "concepts/held/inventory.toml",
            "[unreachable]\nreason = \"nowhere\"\n",
        );
    });
    assert!(held.contains("is not a concept file"), "{held}");

    let held = refused("a-cycle-the-difference-form-draws", |root| {
        write(
            root,
            "concepts/revenue.toml",
            "[[entry]]\nform = \"difference\"\noperands = [\n  { concept = \"gross_profit\" },\n  { taxonomy = \"us-gaap\", tag = \"CostOfRevenue\" },\n]\n",
        );
    });
    assert!(
        held.contains("draws a cycle in the concept edges"),
        "{held}"
    );

    let held = refused("an-assertion-with-no-ground", |root| {
        write(
            root,
            &format!("filers/{APPLE}.toml"),
            &format!(
                "cik = \"{APPLE}\"\n\n[[assert]]\nconcept = \"long_term_debt\"\nperiod = {{ instant = \"2024-09-28\" }}\nvalue = \"0\"\n"
            ),
        );
    });
    assert!(held.contains("citing no source"), "{held}");

    let held = refused("two-assertions-whose-periods-overlap", |root| {
        write(
            root,
            &format!("filers/{APPLE}.toml"),
            &format!(
                "\
cik = \"{APPLE}\"

[[assert]]
concept = \"long_term_debt\"
period = {{ start = \"2024-01-01\", end = \"2024-12-31\" }}
value = \"0\"
source = {{ accession = \"0000320193-24-000123\", line = \"42\" }}

[[assert]]
concept = \"long_term_debt\"
period = {{ start = \"2024-06-30\", end = \"2025-06-30\" }}
value = \"1\"
source = {{ accession = \"0000320193-24-000123\", line = \"43\" }}
"
            ),
        );
    });
    assert!(held.contains("whose periods overlap"), "{held}");

    let held = refused("a-rule-both-included-and-excluded", |root| {
        write(
            root,
            &format!("filers/{APPLE}.toml"),
            &format!(
                "\
cik = \"{APPLE}\"

[[include]]
concept = \"inventory\"
form = \"tag\"
operands = [
  {{ taxonomy = \"us-gaap\", tag = \"InventoryNet\" }},
]

[[exclude]]
id = \"inventory|*|tag|element:us-gaap:InventoryNet\"
"
            ),
        );
    });
    assert!(held.contains("both includes and excludes"), "{held}");

    let held = refused("a-filer-file-named-for-a-ticker", |root| {
        write(root, "filers/AAPL.toml", "cik = \"0000320193\"\n");
    });
    assert!(held.contains("is not named for a CIK"), "{held}");

    let held = refused("a-filer-file-that-names-another-filer", |root| {
        write(
            root,
            &format!("filers/{APPLE}.toml"),
            "cik = \"0001018724\"\n",
        );
    });
    assert!(
        held.contains("is not the filer its name binds it to"),
        "{held}"
    );

    let held = refused("a-line-that-is-not-of-this-format", |root| {
        write(root, "concepts/inventory.toml", "[[entry]]\nform: tag\n");
    });
    assert!(
        held.contains("is neither a comment, a table, nor a field"),
        "{held}"
    );
}

#[test]
fn a_registry_that_is_not_there_is_not_an_empty_one() {
    let missing = Path::new(env!("CARGO_TARGET_TMPDIR")).join("no-registry-here");
    let _ = fs::remove_dir_all(&missing);

    assert!(Registry::read_from(&missing).is_err());
}
