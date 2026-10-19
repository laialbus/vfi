//! Return on capital employed: operating income over a duration as a fraction
//! of the average of capital employed at the instants the duration opens and
//! closes at, before tax.
//!
//! Operating income is a flow `canonical-concepts` v2 states at a duration,
//! and total assets and current liabilities balances it states at an instant.
//! So the metric is stated at the duration, reads operating income at its
//! row, and reads both balances averaged, which
//! `docs/adr/which-other-period-a-metric-reads.md` fixes for return on capital
//! employed in its table. Capital employed is formed in [`super::capital`] at
//! each instant before the two are averaged. Neither operating income nor
//! current liabilities is a term that contributes nothing where it is absent,
//! so a filer they do not apply to has no figure, and a negative average
//! capital employed is a value that gives a figure with its sign.
//!
//! The formula is `docs/adr/capital-employed-invested-capital-and-nopat.md`'s,
//! whose source is cited beside the term.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::capital::{ZERO_CAPITAL_EMPLOYED, capital_employed};
use super::reading::reads;
use super::{Computing, Derivation, Entry, Reading, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const RETURN_ON_CAPITAL_EMPLOYED: Derivation = Derivation {
    entry: Entry {
        name: "return_on_capital_employed",
        meaning: "Operating income over the duration, divided by the average of total assets \
                  less current liabilities at the instants it opens and closes at: the \
                  profit the business earned before tax for each unit of the long-term \
                  capital it employed while earning it.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::OperatingIncome,
            Concept::TotalAssets,
            Concept::CurrentLiabilities,
        ],
        conditions: &[ZERO_CAPITAL_EMPLOYED, START_NOT_A_DATE],
    },
    readings: &[None, Some(Reading::Averaged), Some(Reading::Averaged)],
    derived: return_on_capital_employed,
};

fn return_on_capital_employed(computing: &Computing) -> Result<Number, Reasons> {
    let [operating_income, total_assets, current_liabilities] =
        reads(computing, &RETURN_ON_CAPITAL_EMPLOYED)?;
    operating_income
        .mean()
        .divided(&capital_employed(&total_assets, &current_liabilities).mean())
        .ok_or_else(|| declined(ZERO_CAPITAL_EMPLOYED))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::RETURN_ON_CAPITAL_EMPLOYED;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &RETURN_ON_CAPITAL_EMPLOYED.entry;
        assert_eq!(entry.name, "return_on_capital_employed");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        for term in [
            "Operating income over the duration",
            "total assets less current liabilities",
            "before tax",
        ] {
            assert!(entry.meaning.contains(term), "{term}");
        }
        assert_eq!(
            entry.inputs,
            [
                Concept::OperatingIncome,
                Concept::TotalAssets,
                Concept::CurrentLiabilities
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "zero_capital_employed",
                "start_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
