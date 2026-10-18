//! The common-shareholder terms: common equity and income available to
//! common, each formed once, here. Every metric that reads one reads it from
//! this module, and none writes either subtraction itself. What each term is,
//! and that the common dividend total is left unformed, is
//! `docs/adr/common-shareholder-forms.md`'s, under the method version.
//!
//! Each term is formed at each row its inputs were read at, before any mean,
//! so an averaged term is the average of the term.

use super::reading::Reads;
use crate::arithmetic::Number;

/// The common equity a metric divides by is zero, and no ratio to zero is
/// defined. Every metric over common equity is undefined under this one
/// condition.
pub(super) const ZERO_COMMON_EQUITY: &str = "zero_common_equity";

/// Common equity: shareholders' equity less preferred equity, at each instant
/// both were read at.
///
/// Source: Penman, S. H., *Financial Statement Analysis and Security
/// Valuation*, 5th ed., McGraw-Hill Education, 2013, not read from the
/// workspace. Beside it, the denominator of item 24 of the CFA Institute's
/// *CFA Program Level II Financial Ratio List*,
/// <https://www.cfainstitute.org/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
/// "Average common shareholders’ equity".
pub(super) fn common_equity(shareholders_equity: &Reads, preferred_equity: &Reads) -> Reads {
    shareholders_equity.with(preferred_equity, Number::minus)
}

/// Income available to common: net income less preferred dividends, over one
/// duration. A negative deduction raises it above net income, and the sign is
/// not branched on.
///
/// Source: Financial Accounting Standards Board, Statement of Financial
/// Accounting Standards No. 128, *Earnings per Share*, 1997, ¶9, codified at
/// ASC 260-10-45-11, <https://storage.fasb.org/fas128.pdf>, footnotes left
/// out: "Income available to common stockholders shall be computed by
/// deducting both the dividends declared in the period on preferred stock
/// (whether or not paid) and the dividends accumulated for the period on
/// cumulative preferred stock (whether or not earned) from income from
/// continuing operations (if that amount appears in the income statement)
/// and also from net income." For the redemption's difference from carrying
/// amount: Emerging Issues Task Force, Topic No. D-42, *The Effect on the
/// Calculation of Earnings per Share for the Redemption or Induced Conversion
/// of Preferred Stock*, SEC staff announcement, 1995, codified at ASC
/// 260-10-S99-2, not read from the workspace.
pub(super) fn income_available_to_common(net_income: &Reads, preferred_dividends: &Reads) -> Reads {
    net_income.with(preferred_dividends, Number::minus)
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use crate::metrics::DERIVATIONS;

    #[test]
    fn an_entry_reads_a_preferred_concept_exactly_where_its_name_says_common() {
        for derivation in DERIVATIONS {
            let entry = &derivation.entry;
            let preferred = entry.inputs.iter().any(|concept| {
                matches!(
                    concept,
                    Concept::PreferredEquity | Concept::PreferredDividends
                )
            });
            assert_eq!(preferred, entry.name.contains("common"), "{}", entry.name);
        }
    }

    #[test]
    fn no_entry_reads_both_dividends_paid_and_preferred_dividends() {
        for derivation in DERIVATIONS {
            let inputs = derivation.entry.inputs;
            assert!(
                !(inputs.contains(&Concept::DividendsPaid)
                    && inputs.contains(&Concept::PreferredDividends)),
                "{}",
                derivation.entry.name
            );
        }
    }
}
