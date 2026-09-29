//! One case for each red condition of `the-harness-checks-the-claims.md`, over
//! planted text rather than the committed fixtures, and the shape that holds
//! beside them. A check that went red on everything would pass every red case
//! here, so the holding case is what makes the others mean anything.

use super::{Failure, check};

/// Two periods of three concept lines each, one of them written under both, as
/// `expected` writes a concept line many times over.
const EXPECTED: &str = "\
filer 0000000001
at 2024-12-31
  revenue Unknown
  cash Value 5 read us-gaap:Cash in a rule registry|cash
  dividends_paid Value 0 silence conditional under registry
from 2024-01-01 to 2024-12-31
  revenue Value 7 read us-gaap:Revenues in a rule registry|revenue
  cash Unknown
  dividends_paid Value 0 silence conditional under registry
";

fn failures(claims: &str) -> Vec<Failure> {
    check(Some(claims), EXPECTED)
}

/// The one failure a case plants, at the line it was planted on.
fn only(claims: &str, line: usize, text: &str, reason: &str) {
    let failures = failures(claims);
    let seen: Vec<String> = failures
        .iter()
        .map(|failure| format!("{:?} {:?} {}", failure.line, failure.text, failure.reason))
        .collect();
    assert_eq!(
        failures.len(),
        1,
        "one failure planted, and these went red: {seen:#?}"
    );
    let failure = &failures[0];
    assert_eq!(failure.line, Some(line), "{seen:#?}");
    assert_eq!(failure.text, text, "{seen:#?}");
    assert!(failure.reason.contains(reason), "{seen:#?}");
}

#[test]
fn claims_that_hold_are_not_red() {
    let claims = "\
# A header, prose to the check.

| filer 0000000001

| at 2024-12-31
| from 2024-01-01 to 2024-12-31

Prose between groups, and a line with a bar | inside it.

| at 2024-12-31

|   revenue Unknown
|   cash Value 5 read us-gaap:Cash in a rule registry|cash

|   dividends_paid Value 0 silence conditional under registry

| from 2024-01-01 to 2024-12-31
|   revenue Value 7 read us-gaap:Revenues in a rule registry|revenue

|   dividends_paid Value 0 silence conditional under registry

## A new section ends the scope

|   cash Unknown
";
    let failures = failures(claims);
    assert!(
        failures.is_empty(),
        "{:#?}",
        failures
            .iter()
            .map(|f| (f.line, &f.reason))
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_fixture_with_no_claims_is_red() {
    let failures = check(None, EXPECTED);
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].line, None);
    assert!(failures[0].reason.contains("no claims"));
}

#[test]
fn claims_with_no_claimed_line_are_red() {
    let failures = failures("# Only prose.\n\nfiler 0000000001\n");
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].line, None);
    assert!(failures[0].reason.contains("no line beginning `| `"));
}

#[test]
fn a_bar_without_its_space_is_red() {
    only(
        "| filer 0000000001\n|filer 0000000001\n",
        2,
        "|filer 0000000001",
        "without the space",
    );
}

#[test]
fn a_line_expected_does_not_hold_is_red() {
    only(
        "| filer 0000000001\n\n|   revenue Value 8 read us-gaap:Revenues in a rule registry|revenue\n",
        3,
        "  revenue Value 8 read us-gaap:Revenues in a rule registry|revenue",
        "not a line of expected",
    );
}

/// Trailing whitespace is part of the line, so a claim that differs from
/// `expected` only there is a different line.
#[test]
fn a_trailing_space_is_part_of_the_line() {
    only(
        "| filer 0000000001 \n",
        1,
        "filer 0000000001 ",
        "not a line of expected",
    );
}

#[test]
fn a_group_that_is_not_consecutive_is_red() {
    only(
        "| filer 0000000001\n\n|   revenue Unknown\n|   dividends_paid Value 0 silence conditional under registry\n",
        3,
        "  revenue Unknown",
        "not consecutive",
    );
}

#[test]
fn a_listing_out_of_order_is_red() {
    only(
        "| from 2024-01-01 to 2024-12-31\n| at 2024-12-31\n",
        2,
        "at 2024-12-31",
        "not after the one before it",
    );
}

#[test]
fn an_anchor_expected_writes_twice_is_red() {
    let expected = format!("{EXPECTED}at 2024-12-31\n");
    let failures = check(Some("| at 2024-12-31\n|   revenue Unknown\n"), &expected);
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].line, Some(1));
    assert_eq!(failures[0].text, "at 2024-12-31");
    assert!(failures[0].reason.contains("2 times"));
}

/// The silence line is written under both periods, so only the anchor can say
/// the claim put it under the wrong one.
#[test]
fn a_placed_group_outside_its_period_is_red() {
    only(
        "| at 2024-12-31\n\n|   revenue Value 7 read us-gaap:Revenues in a rule registry|revenue\n",
        3,
        "  revenue Value 7 read us-gaap:Revenues in a rule registry|revenue",
        "placed under \"at 2024-12-31\" (line 1)",
    );
}

#[test]
fn placed_groups_out_of_order_are_red() {
    only(
        "| at 2024-12-31\n\n|   cash Value 5 read us-gaap:Cash in a rule registry|cash\n\n|   revenue Unknown\n",
        5,
        "  revenue Unknown",
        "placed under",
    );
}

/// A group written once under the anchor is still looked for after the group
/// before it, so a line `expected` writes under both periods cannot be matched
/// twice at the same place.
#[test]
fn a_placed_group_is_matched_after_the_one_before_it() {
    only(
        "| from 2024-01-01 to 2024-12-31\n\n\
         |   dividends_paid Value 0 silence conditional under registry\n\n\
         |   dividends_paid Value 0 silence conditional under registry\n",
        5,
        "  dividends_paid Value 0 silence conditional under registry",
        "placed under",
    );
}

/// A `#` line ends the anchor's scope, and so does a listing: the same line
/// after either is held only to being in `expected`.
#[test]
fn a_heading_or_a_listing_ends_the_scope() {
    let claims = "\
| at 2024-12-31

## Elsewhere

|   revenue Value 7 read us-gaap:Revenues in a rule registry|revenue

| at 2024-12-31

| at 2024-12-31
| from 2024-01-01 to 2024-12-31

|   revenue Value 7 read us-gaap:Revenues in a rule registry|revenue
";
    assert!(failures(claims).is_empty());
}

/// Every failure is collected, so a file carrying three is reported whole.
#[test]
fn every_failure_is_collected() {
    let claims = "\
|filer 0000000001

| nothing expected writes

| at 2024-12-31

|   revenue Value 7 read us-gaap:Revenues in a rule registry|revenue
";
    let lines: Vec<Option<usize>> = failures(claims).iter().map(|f| f.line).collect();
    assert_eq!(lines, [Some(1), Some(3), Some(7)]);
}
