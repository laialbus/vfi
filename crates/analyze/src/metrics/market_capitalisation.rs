//! Market capitalisation, formed once, here: the unadjusted close at the
//! valuation date times common shares outstanding. Every metric that reads it
//! reads it from this module, and none writes the product itself:
//! `price_to_common_book`, and price/sales, price/free cash flow, enterprise
//! value and Altman's X₄ when they are built. What it is, and
//! which metrics read it, is
//! `docs/adr/market-capitalisation-and-the-equity-multiples.md`'s, under the
//! method version.
//!
//! It is no metric of the method and is handed over nowhere: it has no
//! catalogue entry, and no name crosses `analyze-store` v1 for it. As a
//! metric it would need a unit of currency, which the record names,
//! `Currency`, and leaves open.
//!
//! It is formed at each row the count was read at, before any mean, so an
//! averaged market capitalisation is the average of the term. Its reader
//! makes the count's read and the price read, and the term's reasons are
//! theirs: no outcome says the term was absent.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;

use super::declined;
use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::reading::Reads;
use crate::arithmetic::Number;

/// A count of shares outstanding that was read is zero or below, and where a
/// reader averages, either count is. A zero count makes the market value
/// zero, which reads as a free filer, and a negative count is no count.
/// Averaged in, a zero halves the term and looks like a real figure.
pub(super) const NON_POSITIVE_SHARES_OUTSTANDING: &str = "non_positive_shares_outstanding";

/// Market capitalisation: `close`, the close at the valuation date, times
/// `shares_outstanding` at each instant the count was read at; or the first
/// of its two conditions that holds, [`NON_POSITIVE_CLOSE`] and then
/// [`NON_POSITIVE_SHARES_OUTSTANDING`]. Both are asked here, so they are
/// asked only where the price read and every count read gave a number.
///
/// Source: Damodaran, A., *Valuation Approaches and Metrics: A Survey of the
/// Theory and Evidence*, Stern School of Business, New York University,
/// November 2006,
/// <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
/// pages, 1,602,127 bytes, sha256
/// `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page
/// 59: "the price per share of a stock is a function both of the value of the
/// equity in a company and the number of shares outstanding in the firm.
/// Thus, a stock split that doubles the number of units will approximately
/// halve the stock price." Beside it, the numerator of the equation on page 2
/// of chapter 19 of his *Investment Valuation*, cited in full at
/// [`super::price_to_common_book`]: "Market Value of Equity". No copy opened
/// states the product in symbols. The product is the record's reading of
/// these pages: the market value of equity is the price per share times the
/// count.
pub(super) fn market_capitalisation(
    close: &Number,
    shares_outstanding: &Reads,
) -> Result<Reads, Reasons> {
    if close.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_CLOSE));
    }
    shares_outstanding.each(|count| match count.sign() {
        Ordering::Greater => Ok(close.times(count)),
        Ordering::Equal | Ordering::Less => Err(declined(NON_POSITIVE_SHARES_OUTSTANDING)),
    })
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};
    use vfi_contracts::canonical_concepts::{Concept, Period, Resolution, Row, Rule, SetBy};
    use vfi_contracts::fetch_analyze::{Answer, Crossing, Splits};

    use super::{NON_POSITIVE_SHARES_OUTSTANDING, market_capitalisation};
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::reading::{Computing, Reading, reads};
    use crate::metrics::{DERIVATIONS, Derivation, Entry, Shape, Unit};
    use crate::settings::Settings;

    const OPENING: &str = "2023-12-31";

    const CLOSING: &str = "2024-12-31";

    const VALUATION: &str = "2025-06-30";

    /// The term as a reader that averages forms it over a year: the count at
    /// the instants the year opens and closes at, each times the close, then
    /// the mean.
    const AVERAGED: Derivation = Derivation {
        entry: Entry {
            name: "averaged_market_capitalisation",
            meaning: "The close times the count at each of the year's two instants, averaged.",
            unit: Unit::Multiple,
            stated_at: Shape::Duration,
            inputs: &[Concept::SharesOutstanding],
            conditions: &[
                NON_POSITIVE_CLOSE,
                NON_POSITIVE_SHARES_OUTSTANDING,
                START_NOT_A_DATE,
            ],
        },
        readings: &[Some(Reading::Averaged)],
        derived: |computing| {
            let [shares_outstanding] = reads(computing, &AVERAGED)?;
            let close = computing
                .close_at_valuation_date()
                .map_err(|reason| Reasons::of(reason, Vec::new()))?;
            Ok(market_capitalisation(&close, &shares_outstanding)?.mean())
        },
    };

    /// A row at `period` where every concept states `amount`.
    fn row(period: Period, amount: &str) -> Row {
        Row::at(
            period,
            std::array::from_fn(|_| Resolution::Value {
                amount: amount.into(),
                set_by: SetBy::Asserted {
                    rule: Rule {
                        registry: "1".into(),
                        id: "asserted".into(),
                    },
                    filing: "0000320193-25-000079".into(),
                },
            }),
        )
    }

    fn traded(date: &str, close: &str) -> Answer {
        Answer::Price {
            date_asked: date.into(),
            trading_date: date.into(),
            close: close.into(),
            retrieved_from: "https://example.invalid/prices".into(),
        }
    }

    /// The averaged term over 2024, the count being `opening` and `closing`
    /// at the year's two instants, beside the preset crossing closing at
    /// `close`.
    fn averaged(opening: &str, closing: &str, close: &str) -> Result<Box<str>, Reasons> {
        let history = [
            row(
                Period::Duration {
                    start: "2024-01-01".into(),
                    end: CLOSING.into(),
                },
                "1",
            ),
            row(Period::Instant { at: OPENING.into() }, opening),
            row(Period::Instant { at: CLOSING.into() }, closing),
        ];
        let crossing = Crossing {
            cik: "0000000001".into(),
            ticker: "VFI".into(),
            at_valuation_date: traded(VALUATION, close),
            at_earlier_dates: vec![traded(CLOSING, "50")],
            splits: Splits::Listed {
                factors: Vec::new(),
            },
        };
        let settings = Settings::preset();
        (AVERAGED.derived)(&Computing::at(
            &history[0],
            &history,
            Some(&crossing),
            &settings,
        ))
        .map(|number| number.written())
    }

    fn declined(condition: &'static str) -> Result<Box<str>, Reasons> {
        Err(Reasons::of(Reason::declined(condition.into()), Vec::new()))
    }

    /// 60 × 90 is 5400 and 60 × 110 is 6600, whose mean is 6000.
    #[test]
    fn an_averaged_term_is_the_average_of_the_close_times_each_count() {
        assert_eq!(averaged("90", "110", "60"), Ok("6000".into()));
    }

    /// 60 × 45, the mean of 90 and 0, would be written `2700`.
    #[test]
    fn either_count_at_zero_or_below_declines_and_is_never_averaged_in() {
        for (opening, closing) in [("90", "0"), ("0", "110"), ("90", "-110"), ("-90", "110")] {
            assert_eq!(
                averaged(opening, closing, "60"),
                declined(NON_POSITIVE_SHARES_OUTSTANDING),
                "{opening} and {closing}"
            );
        }
    }

    #[test]
    fn the_close_is_asked_before_the_count() {
        for close in ["0", "-1"] {
            assert_eq!(
                averaged("90", "0", close),
                declined(NON_POSITIVE_CLOSE),
                "{close}"
            );
            assert_eq!(
                averaged("90", "110", close),
                declined(NON_POSITIVE_CLOSE),
                "{close}"
            );
        }
    }

    #[test]
    fn the_condition_is_named_as_the_record_names_it() {
        assert_eq!(
            NON_POSITIVE_SHARES_OUTSTANDING,
            "non_positive_shares_outstanding"
        );
    }

    #[test]
    fn no_entry_is_named_for_market_capitalisation() {
        for derivation in DERIVATIONS {
            let name = derivation.entry.name;
            assert!(!name.contains("market_cap"), "{name}");
            assert!(!name.contains("market_value"), "{name}");
        }
    }
}
