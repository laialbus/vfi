//! Asset turnover: revenue over a duration as a multiple of the average of
//! total assets at the instants the duration opens and closes at.
//!
//! Revenue is a flow `canonical-concepts` v2 states at a duration, and total
//! assets a balance it states at an instant; both apply to every kind of
//! filer. So the metric is stated at the duration, reads revenue at its row,
//! and reads total assets averaged, which
//! `docs/adr/which-other-period-a-metric-reads.md` fixes for asset turnover in
//! its table.
//!
//! Source: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
//! *International Financial Statement Analysis*, 3rd ed., CFA Institute
//! Investment Series, Wiley, 2015, the chapter "Financial Analysis
//! Techniques", not read from the workspace. Beside it, the CFA Institute's
//! own definition, which the list's preface makes the one to use where
//! definitions differ: *CFA Program Level II Financial Ratio List*,
//! <https://www.cfainstitute.org/sites/default/files/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
//! item 14, "Total asset turnover ratio = Total revenue ÷ Average total
//! assets".

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{
    Computing, Derivation, Entry, Reading, Shape, Unit, ZERO_AVERAGE_TOTAL_ASSETS, declined, read,
};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const ASSET_TURNOVER: Derivation = Derivation {
    entry: Entry {
        name: "asset_turnover",
        meaning: "Revenue over the duration, divided by the average of total assets at the \
                  instants it opens and closes at: how many times over the business turned \
                  what it held into revenue.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[Concept::Revenue, Concept::TotalAssets],
        conditions: &[ZERO_AVERAGE_TOTAL_ASSETS, START_NOT_A_DATE],
    },
    readings: &[None, Some(Reading::Averaged)],
    derived: asset_turnover,
};

fn asset_turnover(computing: &Computing) -> Result<Number, Reasons> {
    let [revenue, average_total_assets] = read(computing, &ASSET_TURNOVER)?;
    revenue
        .divided(&average_total_assets)
        .ok_or_else(|| declined(ZERO_AVERAGE_TOTAL_ASSETS))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::ASSET_TURNOVER;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &ASSET_TURNOVER.entry;
        assert_eq!(entry.name, "asset_turnover");
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::Revenue, Concept::TotalAssets]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "zero_average_total_assets",
                "start_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
