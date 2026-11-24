//! Price/earnings: the unadjusted close at the valuation date over diluted
//! earnings per common share for a year, as the filer reports them. Its form,
//! its year, its three conditions and how its absence composes are
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`'s, under the
//! method version.
//!
//! It is stated over a year, as
//! `docs/adr/which-duration-precedes-a-duration.md` defines one, so at a
//! duration that is not one it gives the year test's one reason and makes no
//! read. At a year it reads the one figure at that year and the close through
//! the one price read, E being the year's end, and neither read stops the
//! other: the figure's reason, if any, comes first and the price read's
//! follows. Every year's multiple is that year's earnings against the one
//! close, and the figure is never rolled forward to the valuation date. It
//! reads no count, forms no market capitalisation and reads no setting.
//!
//! Source of the form and the year: Damodaran, A., *Investment Valuation*,
//! 2nd ed., John Wiley and Sons, 2002, chapter 18, "Earnings Multiples", read
//! from the author's own copy of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch18.pdf>, 59
//! pages whose printed numbers match their positions, 198,889 bytes, sha256
//! `a864e654892586e6d0ab936c5c0777cfd7fe1a23c4b8e9bd81dffc89e1652e28`. Page 1:
//! "The price earnings ratio is the ratio of the market price per share to
//! the earnings per share." The equation that follows is set as an object
//! whose words the page's text carries one to a line: the market price per
//! share over the earnings per share. The same page: "The PE ratio is
//! consistently defined, with the numerator being the value of equity per
//! share and the denominator measuring earnings per share". The diluted
//! figure is one of the variants that page names: "PE ratios could be
//! computed using current earnings per share, trailing earnings per share,
//! forward earnings per share, fully diluted earnings per share and primary
//! earnings per share." Page 27, for the figure being the current P/E, over a
//! fiscal year: "If the expected growth rate in earnings per share is based
//! upon earnings in the most recent year (current earnings), the PE ratio
//! that should be used is the current PE ratio."
//!
//! Beside it: Damodaran, A., *Valuation Approaches and Metrics: A Survey of
//! the Theory and Evidence*, Stern School of Business, New York University,
//! November 2006,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
//! pages, 1,602,127 bytes, sha256
//! `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page
//! 59: "This price/earnings ratio can be estimated using current earnings per
//! share, yielding a current PE, earnings over the last 4 quarters, resulting
//! in a trailing PE, or an expected earnings per share in the next year,
//! providing a forward PE." The record reads each figure named there as a
//! year's and none as a quarter's. A forward figure is a forecast and not in
//! filings, and a trailing figure adds four quarters, which no record forms.
//!
//! The chapter's page 43: "The price earnings ratio cannot be estimated for
//! firms with negative earnings per share." The example
//! `docs/adr/what-analyze-takes-and-returns.md` gives of `declined` is "P/E
//! on non-positive earnings". That a negative denominator is a condition and
//! not a signed figure is the record's decision on those two, for the one
//! reason [`NEGATIVE_EARNINGS_PER_SHARE_DILUTED`] states.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::dividend_payout_per_share::ZERO_EARNINGS_PER_SHARE_DILUTED;
use super::reading::price::{SHARE_BASIS_DATE_NOT_A_DATE, beside_the_close};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// Diluted earnings per share are below zero. A negative multiple is a price
/// for no unit of earnings, and its sign is the denominator's and not the
/// price's. Stored as a figure it would sort below every real multiple, so a
/// screen for low multiples would rank the filer losing the most first.
const NEGATIVE_EARNINGS_PER_SHARE_DILUTED: &str = "negative_earnings_per_share_diluted";

pub(super) const PRICE_TO_EARNINGS_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "price_to_earnings_per_share",
        meaning: "The unadjusted close at the valuation date over diluted earnings per common \
                  share for a year, as the filer reports it.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[Concept::EarningsPerShareDiluted],
        conditions: &[
            NON_POSITIVE_CLOSE,
            ZERO_EARNINGS_PER_SHARE_DILUTED,
            NEGATIVE_EARNINGS_PER_SHARE_DILUTED,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: &[None],
    derived: price_to_earnings_per_share,
};

/// The year test, so that a year's price is the only one read, then the
/// figure and the price read side by side, then the three conditions in
/// order and the one division.
fn price_to_earnings_per_share(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let ([earnings_per_share_diluted], close) = beside_the_close(
        read(computing, &PRICE_TO_EARNINGS_PER_SHARE),
        computing.close_at_valuation_date(),
    )?;
    if close.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_CLOSE));
    }
    match earnings_per_share_diluted.sign() {
        Ordering::Equal => Err(declined(ZERO_EARNINGS_PER_SHARE_DILUTED)),
        Ordering::Less => Err(declined(NEGATIVE_EARNINGS_PER_SHARE_DILUTED)),
        Ordering::Greater => Ok(close
            .divided(&earnings_per_share_diluted)
            .expect("the figure is above zero")),
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{NEGATIVE_EARNINGS_PER_SHARE_DILUTED, PRICE_TO_EARNINGS_PER_SHARE};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::dividend_payout_per_share::ZERO_EARNINGS_PER_SHARE_DILUTED;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PRICE_TO_EARNINGS_PER_SHARE.entry;
        assert_eq!(entry.name, "price_to_earnings_per_share");
        assert_eq!(
            entry.meaning,
            "The unadjusted close at the valuation date over diluted earnings per common share \
             for a year, as the filer reports it."
        );
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::EarningsPerShareDiluted]);
        assert_eq!(PRICE_TO_EARNINGS_PER_SHARE.readings, [None]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_close",
                "zero_earnings_per_share_diluted",
                "negative_earnings_per_share_diluted",
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
            PRICE_TO_EARNINGS_PER_SHARE.entry.conditions,
            [
                NON_POSITIVE_CLOSE,
                ZERO_EARNINGS_PER_SHARE_DILUTED,
                NEGATIVE_EARNINGS_PER_SHARE_DILUTED,
                START_NOT_A_DATE,
                END_NOT_A_DATE,
                NOT_A_YEAR,
                OVERLAPS_ANOTHER_YEAR,
                SHARE_BASIS_DATE_NOT_A_DATE,
            ]
        );
    }

    #[test]
    fn it_reads_no_count_and_no_total_and_its_name_says_per_share_and_not_common() {
        let entry = &PRICE_TO_EARNINGS_PER_SHARE.entry;
        for unread in [
            Concept::SharesOutstanding,
            Concept::NetIncome,
            Concept::DilutedSharesWeightedAverage,
        ] {
            assert!(!entry.inputs.contains(&unread), "{unread:?}");
        }
        assert!(entry.name.contains("per_share"));
        assert!(!entry.name.contains("common"));
        assert!(entry.meaning.contains("per common share"));
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.name == PRICE_TO_EARNINGS_PER_SHARE.entry.name)
            .count();
        assert_eq!(named, 1);
    }
}
