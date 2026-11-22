//! Price/free cash flow: market capitalisation, the unadjusted close at the
//! valuation date times the average of common shares outstanding at the
//! instants a year opens and closes at, over free cash flow for the year,
//! operating cash flow less capital expenditure. Its form, its year, its four
//! conditions and how its absence composes are
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`'s, under the
//! method version.
//!
//! No copy the task opens defines the multiple. Its form, market
//! capitalisation over free cash flow, and its year are the record's own
//! decisions, on `docs/adr/discounted-cash-flow-and-margin-of-safety.md`'s
//! naming of "the price/free-cash-flow multiple" and its placing of free cash
//! flow in the one module. The record sets the market value of equity over
//! the flow for three reasons. It reuses the one term and the one module, so
//! no second product and no second subtraction is written. v2 carries no free
//! cash flow per share, and the close over free cash flow per share, divided
//! by the same count, is the same exact number by one more division. And the
//! flow is the firm's while the numerator is equity alone, as the chapter
//! cited below says of price/sales. The year is the record's too, for
//! price/sales' reason: a quarter's flow would give about four times a year's
//! multiple with nothing to mark it.
//!
//! It is stated over a year, as
//! `docs/adr/which-duration-precedes-a-duration.md` defines one, so at a
//! duration that is not one it gives the year test's one reason and makes no
//! read. At a year it reads the count at the opening instant and then the
//! closing instant, then operating cash flow and capital expenditure at the
//! year, and the close through the one price read, E being the year's end,
//! which is the date of the closing instant: the share-basis rule is asked
//! once and not per figure. Every read is made and none stops another: the
//! reads' reasons, if any, come first in input order, and the price read's
//! follows. Every year's multiple is that year's free cash flow against the
//! one close. It reads no setting.
//!
//! The numerator is formed in [`super::market_capitalisation`], at each
//! instant the count was read at and before the mean, so it is the mean of
//! the two terms. The denominator is formed in [`super::free_cash_flow`],
//! where its form is cited, the Commission's C&DI 102.07, and nothing more is
//! opened for it here. Neither the product nor the subtraction is written
//! here.
//!
//! Source of the reasons the record borrows: Damodaran, A., *Investment
//! Valuation*, 2nd ed., John Wiley and Sons, 2002, chapter 20, "Revenue
//! Multiples and Sector-Specific Multiples", read from the author's own copy
//! of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch20.pdf>, 42 pages
//! whose printed numbers match their positions, 103,261 bytes, sha256
//! `995b0d71d0a5b69322ae5ebcf3384fe6d8cb41accdb9abfd50be32546e61d246`.
//! Page 2, for the third reason: "The price to sales ratio divides an equity
//! value by revenues that are generated for the firm. Consequently, it will
//! yield lower values for more highly levered firms". Page 1 is quoted at the
//! negative condition below.
//!
//! The averaged count is `docs/adr/which-other-period-a-metric-reads.md`'s
//! rule as written, "Every other metric that pairs a flow with a balance
//! reads the average", read as reaching a count the price multiplies. Its
//! cost is the record's to name: the averaged term is the market value at no
//! date, and where the count rose over the year it is understated, so the
//! multiple reads low, which flatters. The rule is asked with E at the
//! year's end and does not see the opening count: a split dated after the
//! opening instant and before E passes it, and the average then mixes two
//! share bases. The record leaves both open, each to a superseding record of
//! its own.
//!
//! What no check can see: leverage. The flow is the firm's, before debt
//! service where the filer classifies interest in financing, and the
//! numerator is equity alone, so the multiple reads lower for a more levered
//! filer, as the chapter's page 2, quoted above, says of price/sales.
//!
//! And the kind, where the method is weakest. Damodaran, A., *Valuation
//! Approaches and Metrics: A Survey of the Theory and Evidence*, Stern School
//! of Business, New York University, November 2006,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
//! pages, 1,602,127 bytes, sha256
//! `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page
//! 15: "estimating capital expenditures and working capital for a bank, an
//! investment bank or an insurance company is difficult to do." The
//! discounted cash flow record says of the same flow that a bank's operating
//! cash flow carries flows that are its business and not its reinvestment.
//! So for a `bank`, an `insurer` or an `investment_company` the figure is
//! computed and its error has no known direction.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::free_cash_flow::{ZERO_FREE_CASH_FLOW, free_cash_flow};
use super::market_capitalisation::{NON_POSITIVE_SHARES_OUTSTANDING, market_capitalisation};
use super::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::reading::{Reading, reads};
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// Free cash flow over the year is below zero. No source states the multiple
/// at a negative denominator. Page 1 of the chapter cited above says of the
/// other multiples: "unlike earnings and book value ratios, which can become
/// negative for many firms and thus not meaningful, revenue multiples are
/// available even for the most troubled firms and for very young firms." The
/// record applies "not meaningful" here as its own decision, for one rule
/// over four multiples: a negative multiple is a price for no unit of free
/// cash flow, and its sign is the denominator's and not the price's. Stored
/// as a figure it would sort below every real multiple.
///
/// The condition is this metric's own. It is not the discounted cash flow
/// value's `non_positive_free_cash_flow`, which holds at zero too, where this
/// metric reaches [`ZERO_FREE_CASH_FLOW`].
const NEGATIVE_FREE_CASH_FLOW: &str = "negative_free_cash_flow";

pub(super) const PRICE_TO_FREE_CASH_FLOW: Derivation = Derivation {
    entry: Entry {
        name: "price_to_free_cash_flow",
        meaning: "Market capitalisation, the unadjusted close at the valuation date times the \
                  average of common shares outstanding at the instants the year opens and closes \
                  at, over free cash flow for the year, operating cash flow less capital \
                  expenditure.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::SharesOutstanding,
            Concept::OperatingCashFlow,
            Concept::CapitalExpenditure,
        ],
        conditions: &[
            NON_POSITIVE_CLOSE,
            NON_POSITIVE_SHARES_OUTSTANDING,
            ZERO_FREE_CASH_FLOW,
            NEGATIVE_FREE_CASH_FLOW,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: &[Some(Reading::Averaged), None, None],
    derived: price_to_free_cash_flow,
};

/// The year test, so that a year's price is the only one read, then the
/// reads and the price read side by side, then the term's two conditions,
/// then free cash flow's two, then the one division.
fn price_to_free_cash_flow(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    match (
        reads(computing, &PRICE_TO_FREE_CASH_FLOW),
        computing.close_at_valuation_date(),
    ) {
        (Ok([shares_outstanding, operating_cash_flow, capital_expenditure]), Ok(close)) => {
            let market_capitalisation = market_capitalisation(&close, &shares_outstanding)?.mean();
            let free_cash_flow =
                free_cash_flow(&operating_cash_flow.mean(), &capital_expenditure.mean());
            match free_cash_flow.sign() {
                Ordering::Equal => Err(declined(ZERO_FREE_CASH_FLOW)),
                Ordering::Less => Err(declined(NEGATIVE_FREE_CASH_FLOW)),
                Ordering::Greater => Ok(market_capitalisation
                    .divided(&free_cash_flow)
                    .expect("free cash flow is above zero")),
            }
        }
        (inputs, close) => {
            let mut reasons = inputs
                .err()
                .into_iter()
                .flat_map(|reasons| reasons.each().cloned().collect::<Vec<_>>())
                .chain(close.err());
            let first = reasons.next().expect("a read or the price read is absent");
            Err(Reasons::of(first, reasons.collect()))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{NEGATIVE_FREE_CASH_FLOW, PRICE_TO_FREE_CASH_FLOW};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::discounted_cash_flow_value_per_share::NON_POSITIVE_FREE_CASH_FLOW;
    use crate::metrics::free_cash_flow::ZERO_FREE_CASH_FLOW;
    use crate::metrics::market_capitalisation::NON_POSITIVE_SHARES_OUTSTANDING;
    use crate::metrics::reading::Reading;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PRICE_TO_FREE_CASH_FLOW.entry;
        assert_eq!(entry.name, "price_to_free_cash_flow");
        assert_eq!(
            entry.meaning,
            "Market capitalisation, the unadjusted close at the valuation date times the average \
             of common shares outstanding at the instants the year opens and closes at, over free \
             cash flow for the year, operating cash flow less capital expenditure."
        );
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::SharesOutstanding,
                Concept::OperatingCashFlow,
                Concept::CapitalExpenditure
            ]
        );
        assert_eq!(
            PRICE_TO_FREE_CASH_FLOW.readings,
            [Some(Reading::Averaged), None, None]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_close",
                "non_positive_shares_outstanding",
                "zero_free_cash_flow",
                "negative_free_cash_flow",
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "share_basis_date_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }

    #[test]
    fn each_condition_it_reaches_is_listed_by_its_one_constant() {
        assert_eq!(
            PRICE_TO_FREE_CASH_FLOW.entry.conditions,
            [
                NON_POSITIVE_CLOSE,
                NON_POSITIVE_SHARES_OUTSTANDING,
                ZERO_FREE_CASH_FLOW,
                NEGATIVE_FREE_CASH_FLOW,
                START_NOT_A_DATE,
                END_NOT_A_DATE,
                NOT_A_YEAR,
                OVERLAPS_ANOTHER_YEAR,
                SHARE_BASIS_DATE_NOT_A_DATE,
            ]
        );
    }

    #[test]
    fn its_negative_condition_is_its_own_and_not_the_discounted_cash_flow_values() {
        let conditions = PRICE_TO_FREE_CASH_FLOW.entry.conditions;
        assert!(conditions.contains(&NEGATIVE_FREE_CASH_FLOW));
        assert!(!conditions.contains(&NON_POSITIVE_FREE_CASH_FLOW));
        assert_ne!(NEGATIVE_FREE_CASH_FLOW, NON_POSITIVE_FREE_CASH_FLOW);
    }

    #[test]
    fn its_name_says_neither_common_nor_per_share_and_its_meaning_says_common_shares() {
        let entry = &PRICE_TO_FREE_CASH_FLOW.entry;
        assert!(!entry.name.contains("common"));
        assert!(!entry.name.contains("per_share"));
        assert!(entry.meaning.contains("common shares outstanding"));
        for unread in [Concept::DilutedSharesWeightedAverage, Concept::NetIncome] {
            assert!(!entry.inputs.contains(&unread), "{unread:?}");
        }
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.name == PRICE_TO_FREE_CASH_FLOW.entry.name)
            .count();
        assert_eq!(named, 1);
    }
}
