//! Every metric of the method: each one's catalogue entry, defined beside it in
//! its own file, and how its number at a row is derived.
//!
//! A metric is added by adding its file here and its derivation to
//! [`DERIVATIONS`]. The name the hand-over carries it by is read from its
//! entry, and from no second list. How an absent input makes a metric absent
//! is `docs/adr/what-analyze-takes-and-returns.md`'s, and every metric reads
//! and writes through the arithmetic that
//! `docs/adr/how-analyze-computes-and-writes-a-figure.md` fixes.

use vfi_contracts::analyze_store::{Metric, Outcome, Reason, Reasons};
use vfi_contracts::canonical_concepts::{Concept, Period, Row};

use crate::arithmetic::{INPUT_NOT_A_DECIMAL, Number};
use crate::settings::Settings;
use reading::{Computing, Reading, read};

mod asset_turnover;
mod capital;
mod cash_ratio;
mod common_shareholder;
mod current_ratio;
mod debt_to_equity;
mod dividend_coverage;
mod dividend_coverage_per_share;
mod dividend_payout_on_earnings;
mod dividend_payout_on_free_cash_flow;
mod dividend_payout_per_share;
mod dividend_streak;
mod gross_margin;
mod interest_coverage;
mod liabilities_to_assets;
mod net_margin;
mod operating_margin;
mod quick_ratio;
mod reading;
mod retention_per_share;
mod return_on_assets;
mod return_on_capital_employed;
mod return_on_common_equity;
mod return_on_equity;
mod sustainable_growth_on_common_equity;

/// Every metric the method computes, each once.
const DERIVATIONS: &[Derivation] = &[
    gross_margin::GROSS_MARGIN,
    operating_margin::OPERATING_MARGIN,
    net_margin::NET_MARGIN,
    interest_coverage::INTEREST_COVERAGE,
    debt_to_equity::DEBT_TO_EQUITY,
    liabilities_to_assets::LIABILITIES_TO_ASSETS,
    current_ratio::CURRENT_RATIO,
    quick_ratio::QUICK_RATIO,
    cash_ratio::CASH_RATIO,
    dividend_payout_on_earnings::DIVIDEND_PAYOUT_ON_EARNINGS,
    dividend_payout_on_free_cash_flow::DIVIDEND_PAYOUT_ON_FREE_CASH_FLOW,
    dividend_coverage::DIVIDEND_COVERAGE,
    dividend_payout_per_share::DIVIDEND_PAYOUT_PER_SHARE,
    dividend_coverage_per_share::DIVIDEND_COVERAGE_PER_SHARE,
    return_on_assets::RETURN_ON_ASSETS,
    asset_turnover::ASSET_TURNOVER,
    return_on_equity::RETURN_ON_EQUITY,
    return_on_common_equity::RETURN_ON_COMMON_EQUITY,
    return_on_capital_employed::RETURN_ON_CAPITAL_EMPLOYED,
    retention_per_share::RETENTION_PER_SHARE,
    sustainable_growth_on_common_equity::SUSTAINABLE_GROWTH_ON_COMMON_EQUITY,
    dividend_streak::DIVIDEND_STREAK,
];

/// Revenue is zero, and no fraction of zero is defined. Every metric over
/// revenue is undefined under this one condition.
const ZERO_REVENUE: &str = "zero_revenue";

/// Current liabilities are zero, and no ratio to zero is defined. Every
/// liquidity ratio is undefined under this one condition.
const ZERO_CURRENT_LIABILITIES: &str = "zero_current_liabilities";

/// The average of total assets at the instants a duration opens and closes
/// at is zero, and no ratio to zero is defined. Every metric over that
/// average is undefined under this one condition.
const ZERO_AVERAGE_TOTAL_ASSETS: &str = "zero_average_total_assets";

/// A metric: its entry, how it reads each input, and its number at a row of
/// the shape the entry is stated at, or the reasons it has none.
struct Derivation {
    entry: Entry,
    /// One per input, in the entry's order: the reading of an input whose
    /// measure is `balance` where the metric is stated at a duration, and
    /// none for any other input, which is read at the row itself.
    readings: &'static [Option<Reading>],
    derived: fn(&Computing) -> Result<Number, Reasons>,
}

/// What a metric states about itself, which the catalogue the shell reads is
/// generated from. No entry states a precision: every figure is written to
/// the one `SIGNIFICANT_DIGITS`.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the catalogue that reads the rest is not built yet"
    )
)]
struct Entry {
    /// The name the hand-over carries the metric by.
    name: &'static str,
    /// What the figure is, in words.
    meaning: &'static str,
    unit: Unit,
    /// The shape of period the metric is stated at. It is computed at every
    /// period of that shape the history holds, and at no other.
    stated_at: Shape,
    /// The concepts the metric reads, in the order its reasons follow.
    inputs: &'static [Concept],
    /// Each condition under which the metric's own definition is undefined at
    /// inputs that were all read.
    conditions: &'static [&'static str],
}

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "the catalogue that reads it is not built yet")
)]
impl Entry {
    /// Every `declined` condition the metric can be absent under: its own,
    /// then `INPUT_NOT_A_DECIMAL`, which every metric meets because every
    /// metric reads an amount or a close.
    fn declined(&self) -> impl Iterator<Item = &'static str> {
        self.conditions
            .iter()
            .copied()
            .chain(std::iter::once(INPUT_NOT_A_DECIMAL))
    }
}

/// The unit a figure is written in, as a scale, so that nothing presenting it
/// reads a fraction as a percentage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Unit {
    /// A ratio: `0.25` is a quarter, and never `25`.
    FractionOfOne,
    /// How many times one amount covers another, at a scale of one: `4.25` is
    /// four and a quarter times, and never `425` or a percentage.
    Multiple,
    /// A count of whole years: `3` is three years.
    Years,
}

/// One of the two shapes a period takes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Instant,
    Duration,
}

impl Shape {
    fn holds(self, period: &Period) -> bool {
        matches!(
            (self, period),
            (Shape::Instant, Period::Instant { .. }) | (Shape::Duration, Period::Duration { .. })
        )
    }
}

/// Every metric stated at `row`'s period, each once, by its entry's name, with
/// its outcome there: its number written once, or the reasons it has none.
/// `history` is every row the history holds, `row` among them, and every
/// metric is computed under `settings`.
pub(crate) fn at(row: &Row, history: &[Row], settings: &Settings) -> Vec<Metric> {
    let computing = Computing::at(row, history, settings);
    DERIVATIONS
        .iter()
        .filter(|derivation| derivation.entry.stated_at.holds(row.period()))
        .map(|derivation| Metric {
            name: derivation.entry.name.into(),
            outcome: match (derivation.derived)(&computing) {
                Ok(number) => Outcome::Value {
                    figure: number.written(),
                },
                Err(reasons) => Outcome::Absent { reasons },
            },
        })
        .collect()
}

/// The one reason a metric is absent where every input was read and its
/// definition is undefined at them, under `condition`, which its entry names.
fn declined(condition: &'static str) -> Reasons {
    Reasons::of(Reason::declined(condition.into()), Vec::new())
}

#[cfg(test)]
mod tests {
    use super::DERIVATIONS;

    #[test]
    fn no_two_entries_share_a_name() {
        for (at, derivation) in DERIVATIONS.iter().enumerate() {
            assert!(
                DERIVATIONS[..at]
                    .iter()
                    .all(|earlier| earlier.entry.name != derivation.entry.name),
                "{} is named twice",
                derivation.entry.name
            );
        }
    }

    #[test]
    fn every_entry_lists_an_input_and_a_meaning() {
        for derivation in DERIVATIONS {
            assert!(
                !derivation.entry.inputs.is_empty(),
                "{}",
                derivation.entry.name
            );
            assert!(
                !derivation.entry.meaning.is_empty(),
                "{}",
                derivation.entry.name
            );
        }
    }
}
