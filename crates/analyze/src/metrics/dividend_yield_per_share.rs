//! The dividend yield: the dividends declared per common share for a year,
//! regular and special together, over the unadjusted close at the valuation
//! date. Its form, its year, its two conditions and how its absence composes
//! are `docs/adr/dividend-yield.md`'s, under the method version.
//!
//! It is stated over a year, as
//! `docs/adr/which-duration-precedes-a-duration.md` defines one, so at a
//! duration that is not one it gives the year test's one reason and makes no
//! read. At a year it reads the one figure at that year, its own row and no
//! other, and the close through the one price read, E being the year's end,
//! and neither read stops the other: the figure's reason, if any, comes first
//! and the price read's follows. Every year's yield is that year's dividend
//! against the one close, so a past year's figure is that year's dividend at
//! today's price and not the yield a buyer had that year. It reads no
//! setting, no count and no other concept. The dividend is the filing's and
//! never the provider's, and it is never formed from `dividends_paid`.
//!
//! Source of the form and the year: Damodaran, A., *Applied Corporate
//! Finance: A User's Manual*, 3rd ed., John Wiley and Sons, chapter 10,
//! "Dividend Policy", read from the author's own copy of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/acf3E/book/ch10.pdf>, 27
//! sheets, 4,678,827 bytes, sha256
//! `48ee94e4a9f200d5afc5ae51a0d9c876068c715aebd3992360ca136b648117d6`. The
//! file prints "CHAPTER 10" and "DIVIDEND POLICY" and names no book, edition
//! or year. What ties it to the book is its path on the author's site and the
//! author's page for that edition,
//! <https://pages.stern.nyu.edu/~adamodar/New_Home_Page/ACF3E/appldCF3E.htm>,
//! which reads "This web site is designed to support the third edition of
//! "Applied Corporate Finance: A User's Manual". The publisher is John Wiley
//! and Sons." No year of publication was read, and none is written here.
//!
//! Each sheet carries two printed pages side by side, each printing its own
//! number, so a quotation is located by its printed page and read from that
//! half: page 3 is the left half of sheet 2, page 4 its right half, and page
//! 16 the right half of sheet 8.
//!
//! Page 3, for the form, and for the year on its word "Annual": "We generally
//! measure the dividends paid by a firm using one of two measures. The first
//! is the dividend yield, which relates the dividend paid to the price of the
//! stock:", followed by the line "Dividend Yield = Annual Dividends per
//! Share/Price per Share".
//!
//! Page 4, the margin note, for the close at the valuation date, beside
//! `docs/adr/price-provider.md`, which lists the yield under that date. The
//! copy breaks it over three lines: "Dividend Yield: The dollar dividend
//! per", "share divided by the current price per" and "share."
//!
//! Page 4, for a zero dividend giving the figure `0` and no condition: "In
//! both time periods, almost 65% of the overall sample of 7200 companies paid
//! no dividends, making zero the median dividend yield across all companies."
//!
//! Page 16: "For each year we will also compute the dividend yield and
//! dividend payout ratio for each firm." That illustration's table sets each
//! year's dividend over that year's "Stock price at end of year", which is a
//! past year's yield at that year's own close. This metric forms no such
//! figure: the provider record lists the past period ends "for relative yield
//! only", and no past period end's close enters a figure here.
//!
//! What no source fixes, and the record decides: the fiscal year as the
//! chapter's "Annual"; the declared figure where page 3 says "the dividend
//! paid"; a negative dividend as a condition and not a signed figure; and the
//! order of the two conditions, the dividend's before the close's, which is
//! formula order with the close as the divisor.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::dividend_streak::NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE;
use super::reading::price::{SHARE_BASIS_DATE_NOT_A_DATE, beside_the_close};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::{Computing, Derivation, Entry, Shape, Unit, declined, read};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

pub(super) const DIVIDEND_YIELD_PER_SHARE: Derivation = Derivation {
    entry: Entry {
        name: "dividend_yield_per_share",
        meaning: "The dividends declared per common share for a year, regular and special \
                  together, over the unadjusted close at the valuation date.",
        unit: Unit::FractionOfOne,
        stated_at: Shape::Duration,
        inputs: &[Concept::DividendsDeclaredPerShare],
        conditions: &[
            NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
            NON_POSITIVE_CLOSE,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: &[None],
    derived: dividend_yield_per_share,
};

/// The year test, so that a year's price is the only one read, then the
/// figure and the price read side by side, then the two conditions in formula
/// order and the one division. A dividend of zero is no condition: over a
/// close above zero it is the figure zero.
fn dividend_yield_per_share(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let ([dividends_declared_per_share], close) = beside_the_close(
        read(computing, &DIVIDEND_YIELD_PER_SHARE),
        computing.close_at_valuation_date(),
    )?;
    if dividends_declared_per_share.sign() == Ordering::Less {
        return Err(declined(NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE));
    }
    if close.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_CLOSE));
    }
    Ok(dividends_declared_per_share
        .divided(&close)
        .expect("the close is above zero"))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::DIVIDEND_YIELD_PER_SHARE;
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::dividend_coverage_per_share::ZERO_DIVIDENDS_DECLARED_PER_SHARE;
    use crate::metrics::dividend_streak::NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &DIVIDEND_YIELD_PER_SHARE.entry;
        assert_eq!(entry.name, "dividend_yield_per_share");
        assert_eq!(
            entry.meaning,
            "The dividends declared per common share for a year, regular and special together, \
             over the unadjusted close at the valuation date."
        );
        assert_eq!(entry.unit, Unit::FractionOfOne);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(entry.inputs, [Concept::DividendsDeclaredPerShare]);
        assert_eq!(DIVIDEND_YIELD_PER_SHARE.readings, [None]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "negative_dividends_declared_per_share",
                "non_positive_close",
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
            DIVIDEND_YIELD_PER_SHARE.entry.conditions,
            [
                NEGATIVE_DIVIDENDS_DECLARED_PER_SHARE,
                NON_POSITIVE_CLOSE,
                START_NOT_A_DATE,
                END_NOT_A_DATE,
                NOT_A_YEAR,
                OVERLAPS_ANOTHER_YEAR,
                SHARE_BASIS_DATE_NOT_A_DATE,
            ]
        );
    }

    #[test]
    fn a_zero_dividend_is_no_condition_of_it() {
        assert!(
            !DIVIDEND_YIELD_PER_SHARE
                .entry
                .declined()
                .any(|condition| condition == ZERO_DIVIDENDS_DECLARED_PER_SHARE)
        );
    }

    #[test]
    fn it_reads_no_total_and_no_count_and_its_name_says_per_share_and_not_common() {
        let entry = &DIVIDEND_YIELD_PER_SHARE.entry;
        for unread in [Concept::DividendsPaid, Concept::SharesOutstanding] {
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
            .filter(|derivation| derivation.entry.name == DIVIDEND_YIELD_PER_SHARE.entry.name)
            .count();
        assert_eq!(named, 1);
    }
}
