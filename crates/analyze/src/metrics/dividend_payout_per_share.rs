//! Dividend payout per share: the dividend declared per share as a fraction
//! of diluted earnings per share, over one period.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period, and
//! no price and no share count. Diluted earnings per share as the filer
//! reports them are already after preferred dividends, so the per-share form
//! is common-specific and reads no `preferred_dividends`. A
//! `dividends_declared_per_share` a silence reading set to zero is a value of
//! zero, and the payout is then zero. A loss gives a negative figure.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::{Concept, Row};

use super::{Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

/// Diluted earnings per share are zero, and no fraction of zero is defined.
const ZERO_EARNINGS_PER_SHARE_DILUTED: &str = "zero_earnings_per_share_diluted";

pub(super) const DIVIDEND_PAYOUT_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_payout_per_share",
        meaning: "Dividends declared per share over diluted earnings per share for the \
                  period: the fraction of each common share's earnings declared to it as \
                  dividends.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::DividendsDeclaredPerShare,
            Concept::EarningsPerShareDiluted,
        ],
        conditions: &[ZERO_EARNINGS_PER_SHARE_DILUTED],
    },
    derived: dividend_payout_per_share,
};

fn dividend_payout_per_share(row: &Row) -> Result<Number, Reasons> {
    let [dividends_declared_per_share, earnings_per_share_diluted] =
        read(row, DIVIDEND_PAYOUT_PER_SHARE.entry.inputs)?;
    dividends_declared_per_share
        .divided(&earnings_per_share_diluted)
        .ok_or_else(|| declined(ZERO_EARNINGS_PER_SHARE_DILUTED))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_PAYOUT_PER_SHARE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_PAYOUT_PER_SHARE.entry;
        assert_eq!(entry.name, "dividend_payout_per_share");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::DividendsDeclaredPerShare,
                Concept::EarningsPerShareDiluted
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            ["zero_earnings_per_share_diluted", INPUT_NOT_A_DECIMAL]
        );
    }
}
