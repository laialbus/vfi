//! Price/book: market capitalisation, the unadjusted close at the valuation
//! date times common shares outstanding at an instant, over common equity at
//! that instant, shareholders' equity less preferred equity. Its form, its
//! instant, its four conditions and how its absence composes are
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`'s, under the
//! method version.
//!
//! It is stated at an instant, because it reads three balances and no flow,
//! and `docs/adr/which-other-period-a-metric-reads.md` says "A metric stated
//! at an instant reads only balances, at its own row." So it is computed at
//! every instant the history holds, a quarter's balance sheet date included,
//! and makes no year test. The three balances are read at that instant and at
//! no other row, and the close through the one price read, E being the
//! instant's date. Every read is made and none stops another: the balances'
//! reasons, if any, come first in input order, and the price read's follows.
//! Every instant's multiple is that instant's book against the one close. It
//! reads no setting.
//!
//! The numerator is formed in [`super::market_capitalisation`] and the
//! denominator in [`super::common_shareholder`]. Neither the product nor the
//! subtraction is written here.
//!
//! Source of the form: Damodaran, A., *Investment Valuation*, 2nd ed., John
//! Wiley and Sons, 2002, chapter 19, "Book Value Multiples", read from the
//! author's own copy of the chapter,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch19.pdf>, 42
//! pages whose printed numbers match their positions, 106,558 bytes, sha256
//! `575088a41dcdf8d98b34101b96a0658545c31c2693783401005e811507db36d3`. Page 1:
//! "The book value of equity is the difference between the book value of
//! assets and the book value of liabilities, a number that is largely
//! determined by accounting conventions." Page 2: "The price to book ratio is
//! computed by dividing the market price per share by the current book value
//! of equity per share." The common form is that page's: "You should not
//! include the portion of the equity that is attributable to preferred stock
//! in computing the book value of equity, since the price per share refers
//! only to common equity." The total form, which the metric takes and which
//! needs no book value per share, is that page's too: "Some of the problems
//! can be alleviated by computing the price to book ratio using the total
//! market value of equity and book value of equity, rather than per share
//! values." The equation that follows is set as an object whose words the
//! page's text carries one to a line: "Market Value of Equity" over "Book
//! value of equity".
//!
//! The chapter states a negative book value twice. Page 2: "the book value of
//! equity can become negative if a firm has a sustained string of negative
//! earnings reports, leading to a negative price-book value ratio." Page 5:
//! "there are firms with negative book values of equity – the result of
//! continuously losing money – where price to book ratios cannot be
//! computed." The record takes page 5's: page 2 states what the arithmetic
//! gives, and page 5 what the chapter does with it. So a negative common
//! equity is [`NEGATIVE_COMMON_EQUITY`] and not a signed figure, while
//! `return_on_common_equity` keeps giving a figure there: the sign belongs to
//! the multiple.
//!
//! Where the method is strongest by kind: Damodaran, A., *Valuation
//! Approaches and Metrics: A Survey of the Theory and Evidence*, Stern School
//! of Business, New York University, November 2006,
//! <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
//! pages, 1,602,127 bytes, sha256
//! `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page
//! 61: "price to book ratios common in financial service company valuations".
//! Its page 59 is the term's source, quoted beside the term.
//!
//! What no check can see: the class of a share. The chapter's page 2: "If
//! there are multiple classes of shares outstanding, the price per share can
//! be different for different classes of shares". The count may be of every
//! common class while the close is one class's, and nothing in v2 or the
//! crossing names the class.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::common_shareholder::{ZERO_COMMON_EQUITY, common_equity};
use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::market_capitalisation::{NON_POSITIVE_SHARES_OUTSTANDING, market_capitalisation};
use super::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
use super::reading::reads;
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;

/// Common equity is below zero. A negative multiple is a price for no unit of
/// book, and its sign is the denominator's and not the price's. Stored as a
/// figure it would sort below every real multiple, so a screen for low
/// multiples would rank the filer whose book is furthest below zero first.
const NEGATIVE_COMMON_EQUITY: &str = "negative_common_equity";

pub(super) const PRICE_TO_COMMON_BOOK: Derivation = Derivation {
    entry: Entry {
        name: "price_to_common_book",
        meaning: "Market capitalisation, the unadjusted close at the valuation date times \
                  common shares outstanding at the instant, over common equity at that instant, \
                  shareholders' equity less preferred equity.",
        unit: Unit::Multiple,
        stated_at: Shape::Instant,
        inputs: &[
            Concept::SharesOutstanding,
            Concept::ShareholdersEquity,
            Concept::PreferredEquity,
        ],
        conditions: &[
            NON_POSITIVE_CLOSE,
            NON_POSITIVE_SHARES_OUTSTANDING,
            ZERO_COMMON_EQUITY,
            NEGATIVE_COMMON_EQUITY,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: &[None, None, None],
    derived: price_to_common_book,
};

/// The three balances and the price read side by side, then the term's two
/// conditions, then the denominator's two, then the one division.
fn price_to_common_book(computing: &Computing) -> Result<Number, Reasons> {
    match (
        reads(computing, &PRICE_TO_COMMON_BOOK),
        computing.close_at_valuation_date(),
    ) {
        (Ok([shares_outstanding, shareholders_equity, preferred_equity]), Ok(close)) => {
            let market_capitalisation = market_capitalisation(&close, &shares_outstanding)?.mean();
            let common_equity = common_equity(&shareholders_equity, &preferred_equity).mean();
            match common_equity.sign() {
                Ordering::Equal => Err(declined(ZERO_COMMON_EQUITY)),
                Ordering::Less => Err(declined(NEGATIVE_COMMON_EQUITY)),
                Ordering::Greater => Ok(market_capitalisation
                    .divided(&common_equity)
                    .expect("common equity is above zero")),
            }
        }
        (balances, close) => {
            let mut reasons = balances
                .err()
                .into_iter()
                .flat_map(|reasons| reasons.each().cloned().collect::<Vec<_>>())
                .chain(close.err());
            let first = reasons
                .next()
                .expect("a balance or the price read is absent");
            Err(Reasons::of(first, reasons.collect()))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{NEGATIVE_COMMON_EQUITY, PRICE_TO_COMMON_BOOK};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::common_shareholder::ZERO_COMMON_EQUITY;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::market_capitalisation::NON_POSITIVE_SHARES_OUTSTANDING;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &PRICE_TO_COMMON_BOOK.entry;
        assert_eq!(entry.name, "price_to_common_book");
        assert_eq!(
            entry.meaning,
            "Market capitalisation, the unadjusted close at the valuation date times common \
             shares outstanding at the instant, over common equity at that instant, \
             shareholders' equity less preferred equity."
        );
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Instant);
        assert_eq!(
            entry.inputs,
            [
                Concept::SharesOutstanding,
                Concept::ShareholdersEquity,
                Concept::PreferredEquity
            ]
        );
        assert_eq!(PRICE_TO_COMMON_BOOK.readings, [None, None, None]);
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_close",
                "non_positive_shares_outstanding",
                "zero_common_equity",
                "negative_common_equity",
                "share_basis_date_not_a_date",
                INPUT_NOT_A_DECIMAL
            ]
        );
    }

    #[test]
    fn each_condition_it_reaches_is_listed_by_its_one_constant() {
        assert_eq!(
            PRICE_TO_COMMON_BOOK.entry.conditions,
            [
                NON_POSITIVE_CLOSE,
                NON_POSITIVE_SHARES_OUTSTANDING,
                ZERO_COMMON_EQUITY,
                NEGATIVE_COMMON_EQUITY,
                SHARE_BASIS_DATE_NOT_A_DATE,
            ]
        );
    }

    #[test]
    fn it_makes_no_year_test_and_lists_none_of_the_four_year_conditions() {
        for year_condition in [
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
        ] {
            assert!(
                !PRICE_TO_COMMON_BOOK
                    .entry
                    .conditions
                    .contains(&year_condition),
                "{year_condition}"
            );
        }
    }

    #[test]
    fn its_name_says_common_and_not_per_share() {
        let entry = &PRICE_TO_COMMON_BOOK.entry;
        assert!(entry.name.contains("common"));
        assert!(!entry.name.contains("per_share"));
        for unread in [
            Concept::EarningsPerShareDiluted,
            Concept::DilutedSharesWeightedAverage,
            Concept::NetIncome,
        ] {
            assert!(!entry.inputs.contains(&unread), "{unread:?}");
        }
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.name == PRICE_TO_COMMON_BOOK.entry.name)
            .count();
        assert_eq!(named, 1);
    }
}
