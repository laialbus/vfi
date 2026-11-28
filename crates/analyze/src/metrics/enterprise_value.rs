//! Enterprise value, formed once, here: market capitalisation plus net debt,
//! the market's value of the common shares plus short-term and long-term debt
//! less cash and equivalents and short-term investments. Every metric that
//! reads it reads it from this module, and none writes the addition itself:
//! `enterprise_value_to_ebit`, and `enterprise_value_to_ebitda` when it is
//! built. What it is, what it does not count and which metrics read it is
//! `docs/adr/enterprise-value-and-its-multiples.md`'s, under the method
//! version.
//!
//! It writes one addition and nothing else. The market capitalisation is what
//! [`super::market_capitalisation`] gives and the net debt what `net_debt` in
//! [`super::capital`] gives, each form cited there, so neither the close
//! times the count nor any part of net debt is written here. Inside net debt
//! a `NotApplicable` `short_term_investments` contributes nothing, that
//! module's one exception as accepted, and nothing else does.
//!
//! What it does not count. The preferred claim: `preferred_equity` is not
//! read, so a filer silent on it has an enterprise value, and one that
//! carries the claim has an enterprise value understated by its carrying
//! amount. Non-controlling interests: v2 carries no concept for them, so a
//! filer that consolidates a subsidiary it does not wholly own has an
//! enterprise value short of the part it does not own. Each makes a multiple
//! of enterprise value read low, which flatters, and nothing measures either.
//!
//! It is no metric of the method and is handed over nowhere: it has no
//! catalogue entry, and no name crosses `analyze-store` v1 for it. As a
//! metric it would need a unit of currency, which the record names,
//! `Currency`, and leaves open.
//!
//! It is formed at each instant its inputs were read at, before any mean, so
//! an averaged enterprise value is the average of the term. Its reader makes
//! the five balances' reads and the price read, and the term's reasons are
//! theirs: no outcome says the term was absent.

use vfi_contracts::analyze_store::Reasons;

use super::capital::net_debt;
use super::market_capitalisation::market_capitalisation;
use super::reading::Reads;
use crate::arithmetic::Number;

/// The averaged enterprise value a multiple divides is zero or below. The
/// multiple asks this, of the average, and the term does not: an enterprise
/// value below zero at one instant and above at the other gives a figure
/// where their average is above zero.
///
/// No copy opened states either multiple at such an enterprise value, and
/// the decision is `docs/adr/enterprise-value-and-its-multiples.md`'s own.
/// At zero the multiple is zero whatever the earnings are. Below zero it
/// orders filers backwards: of two with the same negative enterprise value,
/// the one earning more has the multiple nearer zero and sorts as the dearer.
/// A negative enterprise value is no price paid, the buyer being handed more
/// cash than the firm costs, so it is no multiple of earnings. The cost: a
/// filer priced at or below its net cash is absent, found by its reason and
/// not by its rank.
pub(super) const NON_POSITIVE_ENTERPRISE_VALUE: &str = "non_positive_enterprise_value";

/// Enterprise value at each instant its inputs were read at: the market
/// capitalisation [`market_capitalisation`] gives for `close` and
/// `shares_outstanding` there, plus the net debt [`net_debt`] gives for the
/// four balances there; or the first of the market capitalisation term's two
/// conditions that holds. The term adds no condition to those two. It gives a
/// number with its sign: an enterprise value of zero or below, a filer
/// holding more cash and investments than its market capitalisation and debt
/// together, is a value of the term.
///
/// Source of the form: Damodaran, A., *Investment Valuation*, 2nd ed., John
/// Wiley and Sons, 2002, chapter 18, "Earnings Multiples", read from the
/// author's own copy of the chapter,
/// <https://pages.stern.nyu.edu/~adamodar/pdfiles/valn2ed/ch18.pdf>, 59 pages
/// whose printed numbers match their positions, 198,889 bytes, sha256
/// `a864e654892586e6d0ab936c5c0777cfd7fe1a23c4b8e9bd81dffc89e1652e28`. Page
/// 46: "The enterprise value to EBITDA multiple relates the total market
/// value of the firm, net of cash, to the earnings before interest, taxes and
/// depreciation of the firm." The equation beneath it is set as an object
/// whose words the extraction prints one to a line: `Market`, `Value`, `of`,
/// `Equity`, `+`, `Market`, `Value`, `of`, `-Debt`, `Cash`, over `EBITDA`,
/// the minus sign extracted ahead of the word it follows. Read with the
/// sentence's "net of cash", the numerator is the market value of equity plus
/// the market value of debt less cash.
///
/// The netting, on the same page: "Since the interest income from the cash is
/// not counted as part of the EBITDA, not netting out the cash will result in
/// an overstatement of the true value to EBITDA multiple." v2's
/// `operating_income` excludes interest and investment income, so the reason
/// reaches the securities that earn that income.
///
/// Debt at what is outstanding, page 47: "The market value of equity at
/// Segovia is $1,529 million and the consolidated debt outstanding at the
/// firm is $500 million.", which page 48 works as `1529`, `+`, `500` and `0`
/// over `500`.
///
/// Non-controlling interests left out, which no check can see. Page 46: "When
/// a holding is categorized as a minority holding, the operating income of a
/// firm does not reflect the income from the holding." Page 47, of a majority
/// holding that is consolidated: "The EBITDA includes 100% of the EBITDA of
/// the holding, but the numerator reflects only the percentage (not 100%) of
/// the holding that belongs to the firm. Thus, the value to EBITDA will be
/// too low, leading it to be categorized as an undervalued stock."
///
/// Beside it: Damodaran, A., *Valuation Approaches and Metrics: A Survey of
/// the Theory and Evidence*, Stern School of Business, New York University,
/// November 2006,
/// <https://pages.stern.nyu.edu/~adamodar/pdfiles/papers/valuesurvey.pdf>, 77
/// pages, 1,602,127 bytes, sha256
/// `9bb7f22c1a7a6b10e37421e126ff0c07b062425fe0b1f23d409516c3386897eb`. Page
/// 29, for `short_term_investments` netted beside cash: "The most common of
/// the non-operating assets is cash and marketable securities". The same
/// page, which names no preferred claim, and no check can see what the claim
/// left out costs: "The non-equity claims that have to be subtracted out
/// include not only all debt, but all capitalized leases as well as unfunded
/// pension plan and health care obligations."
///
/// What no source fixes, and the record decides: debt at its carrying amount,
/// where the chapter's equation sets its market value and v2 carries none;
/// `short_term_investments` netted beside cash, against the chapter's one
/// word "Cash"; and that the preferred claim is not counted.
pub(super) fn enterprise_value(
    close: &Number,
    shares_outstanding: &Reads,
    short_term_debt: &Reads,
    long_term_debt: &Reads,
    cash_and_equivalents: &Reads,
    short_term_investments: &Reads,
) -> Result<Reads, Reasons> {
    Ok(market_capitalisation(close, shares_outstanding)?.with(
        &net_debt(
            short_term_debt,
            long_term_debt,
            cash_and_equivalents,
            short_term_investments,
        ),
        Number::plus,
    ))
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{Reason, Reasons};
    use vfi_contracts::canonical_concepts::{Concept, Period, Resolution, Row, Rule, SetBy};
    use vfi_contracts::fetch_analyze::{Answer, Crossing, Splits};

    use super::{NON_POSITIVE_ENTERPRISE_VALUE, enterprise_value};
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::capital::NOTHING_WHERE_NOT_APPLICABLE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::market_capitalisation::NON_POSITIVE_SHARES_OUTSTANDING;
    use crate::metrics::reading::price::beside_the_close;
    use crate::metrics::reading::{Computing, Reading, reads_with_nothing};
    use crate::metrics::{DERIVATIONS, Derivation, Entry, Shape, Unit};
    use crate::settings::Settings;

    const OPENING: &str = "2023-12-31";

    const CLOSING: &str = "2024-12-31";

    const VALUATION: &str = "2025-06-30";

    /// The term as a reader that averages forms it over a year, and asks
    /// nothing of it: the mean of the term at the year's two instants, with
    /// its sign.
    const AVERAGED: Derivation = Derivation {
        entry: Entry {
            name: "averaged_term",
            meaning: "Market capitalisation plus net debt at each of the year's two instants, \
                      averaged.",
            unit: Unit::Multiple,
            stated_at: Shape::Duration,
            inputs: &[
                Concept::SharesOutstanding,
                Concept::ShortTermDebt,
                Concept::LongTermDebt,
                Concept::CashAndEquivalents,
                Concept::ShortTermInvestments,
            ],
            conditions: &[
                NON_POSITIVE_CLOSE,
                NON_POSITIVE_SHARES_OUTSTANDING,
                START_NOT_A_DATE,
            ],
        },
        readings: &[Some(Reading::Averaged); 5],
        derived: |computing| {
            let (
                [
                    shares_outstanding,
                    short_term_debt,
                    long_term_debt,
                    cash_and_equivalents,
                    short_term_investments,
                ],
                close,
            ) = beside_the_close(
                reads_with_nothing(computing, &AVERAGED, NOTHING_WHERE_NOT_APPLICABLE),
                computing.close_at_valuation_date(),
            )?;
            Ok(enterprise_value(
                &close,
                &shares_outstanding,
                &short_term_debt,
                &long_term_debt,
                &cash_and_equivalents,
                &short_term_investments,
            )?
            .mean())
        },
    };

    fn value(amount: &str) -> Resolution {
        Resolution::Value {
            amount: amount.into(),
            set_by: SetBy::Asserted {
                rule: Rule {
                    registry: "1".into(),
                    id: "asserted".into(),
                },
                filing: "0000320193-25-000079".into(),
            },
        }
    }

    /// A row at `period` stating the count, the two debts, cash and
    /// short-term investments as given, and every other concept a value the
    /// term does not read.
    fn row(period: Period, balances: [&str; 5]) -> Row {
        Row::at(
            period,
            std::array::from_fn(|at| {
                value(
                    AVERAGED
                        .entry
                        .inputs
                        .iter()
                        .position(|input| *input == Concept::ALL[at])
                        .map_or("7", |input| balances[input]),
                )
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

    /// The averaged term over 2024, its five balances being `opening` and
    /// `closing` at the year's two instants, beside the preset crossing
    /// closing at `close`.
    fn averaged(opening: [&str; 5], closing: [&str; 5], close: &str) -> Result<Box<str>, Reasons> {
        let history = [
            row(
                Period::Duration {
                    start: "2024-01-01".into(),
                    end: CLOSING.into(),
                },
                ["7"; 5],
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

    /// 60 × 90 + 100 + 900 − 300 − 100 is 6000 and 60 × 110 + 200 + 1300 −
    /// 400 − 100 is 7600, whose mean is 6800.
    #[test]
    fn an_averaged_term_is_the_average_of_market_capitalisation_plus_net_debt_at_each_instant() {
        assert_eq!(
            averaged(
                ["90", "100", "900", "300", "100"],
                ["110", "200", "1300", "400", "100"],
                "60"
            ),
            Ok("6800".into())
        );
    }

    /// Cash of 7900 and 6400 gives −1600 and 1600, cash of 7900 and 6600
    /// gives −1600 and 1400, and cash of 7900 and 8200 gives −1600 and −200.
    #[test]
    fn an_enterprise_value_of_zero_or_below_is_a_value_of_the_term_with_its_sign() {
        for (closing_cash, mean) in [("6400", "0"), ("6600", "-100"), ("8200", "-900")] {
            assert_eq!(
                averaged(
                    ["90", "100", "900", "7900", "100"],
                    ["110", "200", "1300", closing_cash, "100"],
                    "60"
                ),
                Ok(mean.into()),
                "{closing_cash}"
            );
        }
    }

    #[test]
    fn the_term_asks_the_market_capitalisation_terms_two_conditions_and_no_other() {
        for (close, count, condition) in [
            ("0", "110", NON_POSITIVE_CLOSE),
            ("-1", "0", NON_POSITIVE_CLOSE),
            ("60", "0", NON_POSITIVE_SHARES_OUTSTANDING),
            ("60", "-110", NON_POSITIVE_SHARES_OUTSTANDING),
        ] {
            assert_eq!(
                averaged(
                    ["90", "100", "900", "7900", "100"],
                    [count, "200", "1300", "7900", "100"],
                    close
                ),
                declined(condition),
                "{close} and {count}"
            );
        }
    }

    #[test]
    fn the_condition_is_named_as_the_record_names_it() {
        assert_eq!(
            NON_POSITIVE_ENTERPRISE_VALUE,
            "non_positive_enterprise_value"
        );
    }

    #[test]
    fn no_entry_is_named_for_enterprise_value_and_one_multiple_holds_its_characters() {
        let holding: Vec<&str> = DERIVATIONS
            .iter()
            .map(|derivation| derivation.entry.name)
            .filter(|name| name.contains("enterprise_value"))
            .collect();
        assert_eq!(holding, ["enterprise_value_to_ebit"]);
    }
}
