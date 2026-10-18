//! Retention per share: one less the dividend declared per share as a
//! fraction of diluted earnings per share, over one period. It is the
//! complement of `dividend_payout_per_share` at the same row, in exact
//! arithmetic, which `docs/adr/common-shareholder-forms.md` fixes.
//!
//! Both are flows `canonical-concepts` v2 states at a duration and applies to
//! every kind of filer, so the metric reads one row and no other period, and
//! no price and no share count. Diluted earnings per share are already after
//! preferred dividends, so it reads no preferred concept. A
//! `dividends_declared_per_share` a silence reading set to zero gives a
//! retention of one. A loss gives a figure above one, as the payout gives a
//! negative figure, and no condition is added for it.
//!
//! Source: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
//! *International Financial Statement Analysis*, 3rd ed., CFA Institute
//! Investment Series, Wiley, 2015, the chapter "Financial Analysis
//! Techniques", not read from the workspace; and Pinto, J. E., Henry, E.,
//! Robinson, T. R., and Stowe, J. D., *Equity Asset Valuation*, 3rd ed., CFA
//! Institute Investment Series, Wiley, 2015, the chapter "Discounted Dividend
//! Valuation", not read from the workspace. Beside them, the CFA Institute's
//! *CFA Program Level II Financial Ratio List*,
//! <https://www.cfainstitute.org/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
//! item 35, "Dividend payout ratio = Common share dividends ÷ Net income
//! attributable to common shares", and item 36, "Retention rate = (Net income
//! attributable to common shares – Common share dividends) ÷ Net income
//! attributable to common shares = 1 – Payout ratio". The list states the
//! ratio over totals; the record forms it per share, over the one common pair
//! v2 carries.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::dividend_payout_per_share::ZERO_EARNINGS_PER_SHARE_DILUTED;
use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;

pub(super) const RETENTION_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "retention_per_share",
        meaning: "One less dividends declared per share over diluted earnings per share for \
                  the period, the complement of the per-share payout: the fraction of each \
                  common share's earnings kept rather than declared to it as dividends.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::DividendsDeclaredPerShare,
            Concept::EarningsPerShareDiluted,
        ],
        conditions: &[ZERO_EARNINGS_PER_SHARE_DILUTED],
    },
    readings: &[None, None],
    derived: retention_per_share,
};

fn retention_per_share(computing: &Computing) -> Result<Number, Reasons> {
    let [dividends_declared_per_share, earnings_per_share_diluted] =
        read(computing, &RETENTION_PER_SHARE)?;
    dividends_declared_per_share
        .divided(&earnings_per_share_diluted)
        .map(|payout| Number::one().minus(&payout))
        .ok_or_else(|| declined(ZERO_EARNINGS_PER_SHARE_DILUTED))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::RETENTION_PER_SHARE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &RETENTION_PER_SHARE.entry;
        assert_eq!(entry.name, "retention_per_share");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for words in ["each common share's", "complement of the per-share payout"] {
            assert!(entry.meaning.contains(words), "{words}");
        }
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
