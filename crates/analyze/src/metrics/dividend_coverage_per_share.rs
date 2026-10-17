//! Dividend coverage per share: how many times diluted earnings per share
//! cover the dividend declared per share, over one period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period, and
//! no price and no share count. Diluted earnings per share as the filer
//! reports them are already after preferred dividends, so the per-share form
//! is common-specific and reads no `preferred_dividends`. A
//! `dividends_declared_per_share` a silence reading set to zero is a value of
//! zero, a period in which nothing was declared, and nothing is a multiple of
//! it, so the coverage is then declined. A loss gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Dividends declared per share are zero, and nothing is a multiple of zero.
const ZERO_DIVIDENDS_DECLARED_PER_SHARE: &str = "zero_dividends_declared_per_share";

pub(super) const DIVIDEND_COVERAGE_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_coverage_per_share",
        meaning: "Diluted earnings per share over dividends declared per share for the \
                  period: the times each common share's earnings cover the dividend \
                  declared to it.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::EarningsPerShareDiluted,
            Concept::DividendsDeclaredPerShare,
        ],
        conditions: &[ZERO_DIVIDENDS_DECLARED_PER_SHARE],
    },
    readings: &[None, None],
    derived: dividend_coverage_per_share,
};

fn dividend_coverage_per_share(computing: &Computing) -> Result<Number, Reasons> {
    let [earnings_per_share_diluted, dividends_declared_per_share] =
        read(computing, &DIVIDEND_COVERAGE_PER_SHARE)?;
    earnings_per_share_diluted
        .divided(&dividends_declared_per_share)
        .ok_or_else(|| declined(ZERO_DIVIDENDS_DECLARED_PER_SHARE))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_COVERAGE_PER_SHARE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_COVERAGE_PER_SHARE.entry;
        assert_eq!(entry.name, "dividend_coverage_per_share");
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::EarningsPerShareDiluted,
                Concept::DividendsDeclaredPerShare
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_dividends_declared_per_share", INPUT_NOT_A_DECIMAL]
        );
    }
}
