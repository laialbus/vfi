//! What the stage writes, one line at a time.
//!
//! The golden fixture holds a whole filer, but no merged fixture reaches a
//! `NotApplicable`, an assertion, a tie or a version other than the one the
//! run is under, so each is pinned here, with every line written out by hand.
//! A later fixture that reaches one then edits nothing in the crate.

mod fixture;

use vfi_contracts::fetch_normalize::{Fact, Filer, Period};
use vfi_normalize::history::history;
use vfi_normalize::normalize;
use vfi_normalize::registry::Registry;
use vfi_normalize::rendering;

use fixture::{FILER, committed, instant, overriding, planted, read};

/// A balance sheet date no merged fixture reaches.
const AT: &str = "2030-06-30";

const EARLIER: &str = "0000000001-30-000001";
const LATER: &str = "0000000001-31-000001";
const SAME_DAY: &str = "0000000000-31-000001";

/// The filer the committed registry holds as a bank.
const BANK: &str = "0001778784";

fn fact(tag: &str, value: &str, period: Period, filing: (&str, &str, &str)) -> Fact {
    let (accession, form, filed) = filing;
    Fact {
        taxonomy: "us-gaap".into(),
        tag: tag.into(),
        unit: "USD".into(),
        period,
        value: value.into(),
        accession: accession.into(),
        form: form.into(),
        filed: filed.into(),
        report_period_end: "".into(),
    }
}

fn at(tag: &str, value: &str, filing: (&str, &str, &str)) -> Fact {
    fact(tag, value, instant(AT), filing)
}

fn filer(cik: &str, facts: Vec<Fact>) -> Filer {
    Filer {
        cik: cik.into(),
        retrieved_from: "written for the case".into(),
        facts,
    }
}

fn earlier() -> (&'static str, &'static str, &'static str) {
    (EARLIER, "10-Q", "2030-08-01")
}

fn later() -> (&'static str, &'static str, &'static str) {
    (LATER, "10-Q", "2031-08-01")
}

/// Filed the day the later filing was, under a form that is not its with `/A`
/// appended.
fn same_day() -> (&'static str, &'static str, &'static str) {
    (SAME_DAY, "10-K", "2031-08-01")
}

fn written(registry: &Registry, filer: &Filer) -> String {
    let mut out = String::new();
    normalize(registry, filer, &mut out).unwrap_or_else(|why| panic!("{why}"));
    out
}

/// The lines under `concept`'s own line, up to the next concept or period.
fn block<'o>(out: &'o str, concept: &str) -> Vec<&'o str> {
    let lines: Vec<&str> = out.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.starts_with(&format!("  {concept} ")))
        .unwrap_or_else(|| panic!("no line for {concept} in:\n{out}"));
    let mut held = vec![lines[start]];
    for line in &lines[start + 1..] {
        if !line.starts_with("    ") {
            break;
        }
        held.push(line);
    }
    held
}

#[test]
fn a_read_value_and_a_silence_zero_name_their_ways() {
    let out = written(
        &read(&committed()),
        &filer(FILER, vec![at("Assets", "900", earlier())]),
    );
    assert_eq!(out.lines().next(), Some("filer 0002003750"));
    assert_eq!(
        block(&out, "total_assets"),
        [
            "  total_assets Value 900 read us-gaap:Assets in 0000000001-30-000001 rule registry|total_assets|*|tag|element:us-gaap:Assets"
        ]
    );
    assert_eq!(
        block(&out, "short_term_investments"),
        ["  short_term_investments Value 0 silence zero under registry"]
    );
}

#[test]
fn a_concept_the_kind_excludes_names_the_kind_and_the_kinds_its_clause_admits() {
    let out = written(
        &read(&committed()),
        &filer(BANK, vec![at("Assets", "900", earlier())]),
    );
    assert_eq!(
        block(&out, "operating_income"),
        ["  operating_income NotApplicable kind bank admits operating reit"]
    );
    assert_eq!(
        block(&out, "short_term_investments"),
        ["  short_term_investments NotApplicable kind bank admits operating"]
    );
}

#[test]
fn an_asserted_value_names_its_rule_and_the_filing_it_cites() {
    let root = planted("rendering-assertion");
    overriding(
        &root,
        &format!(
            "\n[[assert]]\nconcept = \"short_term_investments\"\n\
             period = {{ instant = \"{AT}\" }}\n\
             value = \"750\"\n\
             source = {{ accession = \"{EARLIER}\", line = \"12\" }}\n"
        ),
    );
    let out = written(
        &read(&root),
        &filer(FILER, vec![at("Assets", "900", later())]),
    );
    assert_eq!(
        block(&out, "short_term_investments"),
        [
            "  short_term_investments Value 750 asserted rule registry|short_term_investments|*|assert|filer:0002003750+instant:2030-06-30 citing 0000000001-30-000001"
        ]
    );
}

#[test]
fn a_tie_is_each_tied_filings_attempt_and_then_the_tie() {
    let out = written(
        &read(&committed()),
        &filer(
            FILER,
            vec![
                at("ShortTermInvestments", "500", same_day()),
                at("MarketableSecuritiesCurrent", "200", same_day()),
                at("ShortTermInvestments", "600", later()),
                at("Assets", "900", later()),
                at("ShortTermInvestments", "700", earlier()),
            ],
        ),
    );
    assert_eq!(
        block(&out, "short_term_investments"),
        [
            "  short_term_investments Unknown",
            "    in 0000000000-31-000001",
            "      registry|short_term_investments|*|tag|element:us-gaap:AvailableForSaleSecuritiesDebtSecuritiesCurrent declined: no fact answering the period asked for",
            "      registry|short_term_investments|*|tag|element:us-gaap:OtherShortTermInvestments declined: no fact answering the period asked for",
            "      registry|short_term_investments|*|tag|element:us-gaap:MarketableSecuritiesCurrent declined: a stand-in dropped behind an exact reading",
            "    in 0000000001-31-000001",
            "      registry|short_term_investments|*|tag|element:us-gaap:AvailableForSaleSecuritiesDebtSecuritiesCurrent declined: no fact answering the period asked for",
            "      registry|short_term_investments|*|tag|element:us-gaap:MarketableSecuritiesCurrent declined: no fact answering the period asked for",
            "      registry|short_term_investments|*|tag|element:us-gaap:OtherShortTermInvestments declined: no fact answering the period asked for",
            "    tie 0000000000-31-000001 0000000001-31-000001",
        ]
    );
}

/// `total_assets` has one rule, so a filing that read it declined nothing on
/// the way.
#[test]
fn an_attempt_that_declined_nothing_says_so() {
    let out = written(
        &read(&committed()),
        &filer(
            FILER,
            vec![
                at("Assets", "900", same_day()),
                at("Assets", "950", later()),
                at("Liabilities", "400", later()),
            ],
        ),
    );
    assert_eq!(
        block(&out, "total_assets"),
        [
            "  total_assets Unknown",
            "    in 0000000000-31-000001 declining nothing",
            "    in 0000000001-31-000001 declining nothing",
            "    tie 0000000000-31-000001 0000000001-31-000001",
        ]
    );
}

#[test]
fn a_version_other_than_the_one_written_under_is_its_sixty_four_characters() {
    let registry = read(&committed());
    let history = history(
        &registry,
        &filer(FILER, vec![at("Assets", "900", earlier())]),
    )
    .unwrap_or_else(|why| panic!("{why}"));

    let root = planted("rendering-another-version");
    overriding(&root, "\n# A comment, which is bytes, so another version.\n");
    let another = read(&root).version();
    assert_ne!(another, registry.version());

    let mut out = String::new();
    rendering::history(&history, another, &mut out);
    let digest = registry.version().rendered();
    assert_eq!(
        block(&out, "total_assets"),
        [format!(
            "  total_assets Value 900 read us-gaap:Assets in 0000000001-30-000001 rule {digest}|total_assets|*|tag|element:us-gaap:Assets"
        )]
    );
    assert_eq!(
        block(&out, "short_term_investments"),
        [format!(
            "  short_term_investments Value 0 silence zero under {digest}"
        )]
    );
    assert_eq!(
        block(&out, "preferred_equity"),
        [
            "  preferred_equity Unknown".to_owned(),
            "    in 0000000001-30-000001".to_owned(),
            format!(
                "      {digest}|preferred_equity|*|tag|element:us-gaap:PreferredStockValue declined: no fact answering the period asked for"
            ),
            format!(
                "      {digest}|preferred_equity|*|tag|element:us-gaap:PreferredStockValueOutstanding declined: no fact answering the period asked for"
            ),
        ]
    );
    assert!(!out.contains(rendering::REGISTRY), "{out}");
}

/// The periods go by their first date and then their second, whatever order
/// the facts arrived in, and an instant goes before a duration that starts on
/// its date.
#[test]
fn the_periods_go_in_the_order_of_their_dates() {
    let duration = |start: &str, end: &str| Period::Duration {
        start: start.into(),
        end: end.into(),
    };
    let out = written(
        &read(&committed()),
        &filer(
            FILER,
            vec![
                fact("NetIncomeLoss", "-5", duration(AT, "2031-06-30"), earlier()),
                at("Assets", "900", earlier()),
                fact("NetIncomeLoss", "-3", duration("2030-01-01", AT), earlier()),
            ],
        ),
    );
    let periods: Vec<&str> = out
        .lines()
        .filter(|line| !line.starts_with(' '))
        .collect();
    assert_eq!(
        periods,
        [
            "filer 0002003750",
            "from 2030-01-01 to 2030-06-30",
            "at 2030-06-30",
            "from 2030-06-30 to 2031-06-30",
        ]
    );
}

#[test]
fn a_filed_that_does_not_read_as_a_date_stops_the_stage_and_writes_nothing() {
    let mut out = String::new();
    let stopped = normalize(
        &read(&committed()),
        &filer(
            FILER,
            vec![
                at("Assets", "900", earlier()),
                at("Assets", "950", (LATER, "10-Q", "2031-13-01")),
            ],
        ),
        &mut out,
    )
    .expect_err("an undated filing is a stop");
    assert_eq!(stopped.accessions(), [Box::<str>::from(LATER)]);
    assert!(out.is_empty(), "{out}");
}
