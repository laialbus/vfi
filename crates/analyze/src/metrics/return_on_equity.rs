//! Return on equity, in its total form: net income over a duration as a
//! fraction of the average of shareholders' equity at the instants the
//! duration opens and closes at.
//!
//! Net income is a flow `canonical-concepts` v2 states at a duration, and
//! shareholders' equity a balance it states at an instant; both apply to every
//! kind of filer. So the metric is stated at the duration, reads net income at
//! its row, and reads shareholders' equity averaged, which
//! `docs/adr/which-other-period-a-metric-reads.md` fixes for return on equity
//! in its table. Both are read as v2 carries them: the equity with the
//! preferred claim included, and the income before any deduction for
//! preferred dividends. The common form waits on the record that forms common
//! equity, `docs/adr/common-shareholder-forms.md`. A net loss and a negative
//! average equity are each a value that gives a negative figure.
//!
//! Source: Robinson, T. R., Henry, E., Pirie, W. L., and Broihahn, M. A.,
//! *International Financial Statement Analysis*, 3rd ed., CFA Institute
//! Investment Series, Wiley, 2015, the chapter "Financial Analysis
//! Techniques", not read from the workspace. Beside it, the CFA Institute's
//! own definition, which the list's preface makes the one to use where
//! definitions differ: *CFA Program Level II Financial Ratio List*,
//! <https://www.cfainstitute.org/sites/default/files/-/media/documents/support/programs/cfa/cfa_program_level_ii_financial_ratio_list.pdf>,
//! item 21, "Return on equity = Net income ÷ Average shareholders' equity".

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::{Computing, Derivation, Entry, Reading, Shape, Unit, declined, read};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// The average of shareholders' equity at the instants a duration opens and
/// closes at is zero, and no ratio to zero is defined.
const ZERO_AVERAGE_SHAREHOLDERS_EQUITY: &str = "zero_average_shareholders_equity";

pub(super) const RETURN_ON_EQUITY: Derivation = Derivation {
    entry: Entry {
        name: "return_on_equity",
        meaning: "Net income over the duration, divided by the average of shareholders' \
                  equity, preferred claim included, at the instants it opens and closes at: \
                  the profit the business earned for each unit of its owners' claim while \
                  earning it.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::NetIncome, Concept::ShareholdersEquity],
        conditions: &[ZERO_AVERAGE_SHAREHOLDERS_EQUITY, START_NOT_A_DATE],
    },
    readings: &[None, Some(Reading::Averaged)],
    derived: return_on_equity,
};

fn return_on_equity(computing: &Computing) -> Result<Number, Reasons> {
    let [net_income, average_shareholders_equity] = read(computing, &RETURN_ON_EQUITY)?;
    net_income
        .divided(&average_shareholders_equity)
        .ok_or_else(|| declined(ZERO_AVERAGE_SHAREHOLDERS_EQUITY))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::RETURN_ON_EQUITY;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &RETURN_ON_EQUITY.entry;
        assert_eq!(entry.name, "return_on_equity");
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [Concept::NetIncome, Concept::ShareholdersEquity]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "zero_average_shareholders_equity",
                "start_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
