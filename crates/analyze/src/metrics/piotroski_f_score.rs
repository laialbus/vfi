//! Piotroski's F_SCORE: the count of his nine signals that are one for a year
//! and the year before it, from zero to nine. Each signal is formed in
//! `piotroski.rs`, which the score calls through the same nine functions the
//! signal metrics call, in §2.3.4's order, and the score forms none itself.
//! It is stated over a year as `docs/adr/piotroski-signals-and-score.md`
//! fixes it.
//!
//! Its reads are the signals' reads, each made once: a read two signals share
//! gives its reason once, and each reason is the one a signal gives for the
//! same read. Total assets at the instant the year opens at are the year's
//! opening balance, the first term of its average and the last of the year
//! before's, and are read once. Cash flow from operations is read at the year
//! alone, since no signal reads it at the year before. Its `declined` is the
//! first condition its signals give, in formula order, and it never says that
//! a signal was absent.
//!
//! Origin, named as `docs/adr/which-other-period-a-metric-reads.md` names it
//! and not read from the workspace: Piotroski, J. D., "Value Investing: The
//! Use of Historical Financial Statement Information to Separate Winners from
//! Losers", *Journal of Accounting Research* 38, Supplement, 2000, pp. 1–41.
//!
//! The open copy beside it: *Selected Paper 84*, The University of Chicago
//! Graduate School of Business, 2002, read from
//! <https://www.chicagobooth.edu/~/media/FE874EE65F624AAEBD0166B1974FD74D.pdf>.
//! The section numbers below are this copy's, and its printed page numbers.
//! §2.3, p. 6: "I define the aggregate signal measure, F_SCORE, as the sum of
//! the nine binary signals." §2.3.4, p. 9: "F_SCORE = F_ROA + F_ΔROA + F_CFO +
//! F_ACCRUAL + F_ΔMARGIN + F_ΔTURN + F_ΔLEVER + F_ΔLIQUID + EQ_OFFER.", whose
//! = and + the copy's text gives as `(cid:4)` and `(cid:5)`, and "Given the
//! nine underlying signals, F_SCORE can range from a low of 0 to a high of 9".
//!
//! A filer whose shape lacks a concept a signal reads, as a bank lacks gross
//! profit and the current subtotals, has no score, and never a score over
//! seven signals: its reasons are the `input_not_applicable` of each read the
//! two signals make, and nothing stands in. Three reasons decide it. The
//! analyze record lets a `NotApplicable` term contribute nothing only where
//! the metric's definition states it, and Piotroski's states no signal that
//! does. A count over seven runs from 0 to 7 under a name that runs from 0 to
//! 9, so a ranking over stored results would mix the two. And the copy scores
//! only firms with the data for every signal, §3.1, p. 10: "After the BM
//! quintiles are formed, I retain firms in the highest BM quintile with
//! sufficient financial statement data to calculate the various financial
//! performance signals."
//!
//! The copy's portfolio cut-offs, §3.3, pp. 10–11, "I classify firms with the
//! lowest aggregate signals (F_SCORE equals 0 or 1) as low F_SCORE firms" and
//! "firms receiving the highest score (i.e., F_SCORE equals 8 or 9) have the
//! strongest fundamental signals and are classified as high F_SCORE firms",
//! are thresholds a user applies to the stored metric under M6. They enter
//! analyze nowhere, as no constant, no setting and no metric.

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::piotroski::{
    ZERO_OPENING_TOTAL_ASSETS, ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
    ZERO_PRECEDING_CURRENT_LIABILITIES, ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
    ZERO_PRECEDING_REVENUE, eq_offer, f_accrual, f_cfo, f_delta_lever, f_delta_liquid,
    f_delta_margin, f_delta_roa, f_delta_turn, f_roa, f_score,
    over_a_year_and_the_year_before_read_once,
};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
use super::{
    Computing, Derivation, Entry, Reading, Shape, Unit, ZERO_AVERAGE_TOTAL_ASSETS,
    ZERO_CURRENT_LIABILITIES, ZERO_REVENUE,
};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const PIOTROSKI_F_SCORE: Derivation = Derivation {
    entry: Entry {
        name: "piotroski_f_score",
        meaning: "The count of Piotroski's nine signals that are one for a year and the year \
                  before it, from zero to nine.",
        unit: Unit::Signals,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::NetIncome,
            Concept::TotalAssets,
            Concept::OperatingCashFlow,
            Concept::GrossProfit,
            Concept::Revenue,
            Concept::LongTermDebt,
            Concept::CurrentAssets,
            Concept::CurrentLiabilities,
            Concept::DilutedSharesWeightedAverage,
        ],
        conditions: &[
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            ZERO_OPENING_TOTAL_ASSETS,
            ZERO_PRECEDING_OPENING_TOTAL_ASSETS,
            ZERO_REVENUE,
            ZERO_PRECEDING_REVENUE,
            ZERO_AVERAGE_TOTAL_ASSETS,
            ZERO_PRECEDING_AVERAGE_TOTAL_ASSETS,
            ZERO_CURRENT_LIABILITIES,
            ZERO_PRECEDING_CURRENT_LIABILITIES,
        ],
    },
    readings: &[
        None,
        Some(Reading::Averaged),
        None,
        None,
        None,
        Some(Reading::Closing),
        Some(Reading::Closing),
        Some(Reading::Closing),
        None,
    ],
    derived: piotroski_f_score,
};

fn piotroski_f_score(computing: &Computing) -> Result<Number, Reasons> {
    let (
        [
            net_income,
            total_assets,
            operating_cash_flow,
            gross_profit,
            revenue,
            long_term_debt,
            current_assets,
            current_liabilities,
            diluted_shares_weighted_average,
        ],
        [
            preceding_net_income,
            preceding_total_assets,
            preceding_gross_profit,
            preceding_revenue,
            preceding_long_term_debt,
            preceding_current_assets,
            preceding_current_liabilities,
            preceding_diluted_shares_weighted_average,
        ],
    ) = over_a_year_and_the_year_before_read_once(
        computing,
        &PIOTROSKI_F_SCORE,
        Concept::OperatingCashFlow,
    )?;
    let [opening_total_assets, preceding_opening_total_assets] =
        [&total_assets, &preceding_total_assets].map(|reads| reads.opening());
    f_score([
        f_roa(&net_income.mean(), &opening_total_assets),
        f_delta_roa(
            &net_income.mean(),
            &opening_total_assets,
            &preceding_net_income.mean(),
            &preceding_opening_total_assets,
        ),
        f_cfo(&operating_cash_flow.mean(), &opening_total_assets),
        f_accrual(
            &net_income.mean(),
            &operating_cash_flow.mean(),
            &opening_total_assets,
        ),
        f_delta_margin(
            &gross_profit.mean(),
            &revenue.mean(),
            &preceding_gross_profit.mean(),
            &preceding_revenue.mean(),
        ),
        f_delta_turn(
            &revenue.mean(),
            &opening_total_assets,
            &preceding_revenue.mean(),
            &preceding_opening_total_assets,
        ),
        f_delta_lever(
            &long_term_debt.mean(),
            &total_assets.mean(),
            &preceding_long_term_debt.mean(),
            &preceding_total_assets.mean(),
        ),
        f_delta_liquid(
            &current_assets.mean(),
            &current_liabilities.mean(),
            &preceding_current_assets.mean(),
            &preceding_current_liabilities.mean(),
        ),
        Ok(eq_offer(
            &diluted_shares_weighted_average.mean(),
            &preceding_diluted_shares_weighted_average.mean(),
        )),
    ])
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::PIOTROSKI_F_SCORE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::metrics::{Reading, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PIOTROSKI_F_SCORE.entry;
        assert_eq!(entry.name, "piotroski_f_score");
        assert!(!entry.name.contains("common"));
        assert_eq!(
            entry.meaning,
            "The count of Piotroski's nine signals that are one for a year and the year before \
             it, from zero to nine."
        );
        assert_eq!(entry.unit, Unit::Signals);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::NetIncome,
                Concept::TotalAssets,
                Concept::OperatingCashFlow,
                Concept::GrossProfit,
                Concept::Revenue,
                Concept::LongTermDebt,
                Concept::CurrentAssets,
                Concept::CurrentLiabilities,
                Concept::DilutedSharesWeightedAverage,
            ]
        );
        assert_eq!(
            PIOTROSKI_F_SCORE.readings,
            [
                None,
                Some(Reading::Averaged),
                None,
                None,
                None,
                Some(Reading::Closing),
                Some(Reading::Closing),
                Some(Reading::Closing),
                None,
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "start_not_a_date",
                "end_not_a_date",
                "not_a_year",
                "overlaps_another_year",
                "zero_opening_total_assets",
                "zero_preceding_opening_total_assets",
                "zero_revenue",
                "zero_preceding_revenue",
                "zero_average_total_assets",
                "zero_preceding_average_total_assets",
                "zero_current_liabilities",
                "zero_preceding_current_liabilities",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }
}
