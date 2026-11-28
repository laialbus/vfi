//! EV/EBIT: enterprise value, averaged over the instants a year opens and
//! closes at, over operating income for the year. Its form, its year, its
//! five conditions and how its absence composes are
//! `docs/adr/enterprise-value-and-its-multiples.md`'s, under the method
//! version.
//!
//! It is stated over a year, as
//! `docs/adr/which-duration-precedes-a-duration.md` defines one, so at a
//! duration that is not one it gives the year test's one reason and makes no
//! read. At a year it reads the count, the two debts, cash and short-term
//! investments, each at the opening instant and then the closing instant,
//! then operating income at the year, and the close through the one price
//! read, E being the year's end, which is the date of the closing instant:
//! the share-basis rule is asked once and not per figure. Every read is made
//! and none stops another: the reads' reasons, if any, come first in input
//! order, and the price read's follows. Every year's multiple is that year's
//! operating income and balances against the one close. It reads no setting
//! and no `preferred_equity`.
//!
//! The numerator is formed in [`super::enterprise_value`], at each of the two
//! instants and before the mean, so it is the mean of the two terms. Neither
//! the product, nor any part of net debt, nor the addition is written here.
//! The denominator is `operating_income` as v2 means it, "profit from the
//! ordinary business after all operating costs and before interest and tax",
//! which is EBIT and has no term and no module.
//!
//! Source of the multiple: Damodaran's chapter 18 and, beside it, his survey,
//! each cited in full at [`super::enterprise_value`] and read from the copies
//! named there. The chapter's page 55: "While enterprise value to EBITDA may
//! be the most widely used value multiple, there are close variants that are
//! sometimes used by analysts", of which the first the page names is
//! "Value/EBIT". The survey's sentence that opens at the foot of page 59,
//! "When buying a business,", and runs on at the head of page 60: "as opposed
//! to just the equity in the business, it is common to examine the value of
//! the firm as a multiple of the operating income or the earnings before
//! interest, taxes, depreciation and amortization (EBITDA)."
//!
//! EBIT as operating income: the chapter's page 49 has "FCFF = EBIT (1-t)"
//! where the survey's page 26 has "Free Cash Flow to Firm = After-tax
//! Operating Income – (Capital Expenditures – Depreciation) – Change in
//! non-cash Working Capital".
//!
//! The year: the chapter's page 51, "The firm is in stable growth and its
//! operating income is expected to grow 5% a year in perpetuity.", and its
//! page 55, of the dataset, "multiples and fundamentals by industry group in
//! the United States for the most recent year". A quarter's operating income
//! under the same enterprise value would give about four times a year's
//! multiple with nothing to mark it.
//!
//! What no source fixes, and the record decides. That "Value" in the
//! chapter's "Value/EBIT" is the enterprise value of page 46, neither copy
//! writing that numerator out, and that EV/EBIT is stated over a year, as
//! the close variant of a multiple the chapter states by the year. Debt at
//! its carrying amount, `short_term_investments` netted beside cash, and the
//! preferred claim not counted, each as [`super::enterprise_value`] says of
//! the term. Every balance averaged, where
//! `docs/adr/which-other-period-a-metric-reads.md`'s rule reaches it as
//! written, "Every other metric that pairs a flow with a balance reads the
//! average": the averaged term is the value of the firm at no date, and where
//! the count or net debt rose over the year it is understated, so the
//! multiple reads low, which flatters. An enterprise value of zero or below
//! as a condition, and a zero or a negative EBIT as conditions.
//!
//! The rule is asked with E at the year's end and does not see the opening
//! count: a split dated after the opening instant and before E passes it,
//! and the average then mixes two share bases in the equity part of the
//! term. The record leaves that open, and the averaging beside it, each to a
//! superseding record of its own.
//!
//! By kind: `operating_income` applies to `operating` and `reit` filers
//! alone, so a `bank`, an `insurer` and an `investment_company` are absent
//! with `input_not_applicable` for it. A `reit`'s `short_term_investments`
//! contribute nothing, so securities it holds outside cash are not netted.

use std::cmp::Ordering;

use vfi_contracts::analyze_store::Reasons;
use vfi_contracts::canonical_concepts::Concept;

use super::capital::NOTHING_WHERE_NOT_APPLICABLE;
use super::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
use super::enterprise_value::{NON_POSITIVE_ENTERPRISE_VALUE, enterprise_value};
use super::market_capitalisation::NON_POSITIVE_SHARES_OUTSTANDING;
use super::reading::price::{SHARE_BASIS_DATE_NOT_A_DATE, beside_the_close};
use super::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR, year};
use super::reading::{Reading, reads_with_nothing};
use super::{Computing, Derivation, Entry, Shape, Unit, declined};
use crate::arithmetic::Number;
use crate::date::START_NOT_A_DATE;

/// Operating income over the year is zero, and no ratio to zero is defined.
const ZERO_OPERATING_INCOME: &str = "zero_operating_income";

/// Operating income over the year is below zero. No copy opened states
/// EV/EBIT at a negative EBIT, and
/// `docs/adr/enterprise-value-and-its-multiples.md` applies, as its own
/// decision, the one rule
/// `docs/adr/market-capitalisation-and-the-equity-multiples.md` settled for a
/// negative denominator: "A multiple is the price paid for a unit of what the
/// denominator measures. Over a negative denominator it is a price for no
/// unit, and its sign is the denominator's, not the price's." The cost: every
/// year of operating loss is absent.
const NEGATIVE_OPERATING_INCOME: &str = "negative_operating_income";

pub(super) const ENTERPRISE_VALUE_TO_EBIT: Derivation = Derivation {
    entry: Entry {
        name: "enterprise_value_to_ebit",
        meaning: "Enterprise value over operating income for the year. Enterprise value is the \
                  average, at the instants the year opens and closes at, of market \
                  capitalisation, the unadjusted close at the valuation date times common shares \
                  outstanding, plus net debt, short-term and long-term debt less cash and \
                  equivalents and short-term investments; the preferred claim and \
                  non-controlling interests are not counted.",
        unit: Unit::Multiple,
        stated_at: Shape::Duration,
        inputs: &[
            Concept::SharesOutstanding,
            Concept::ShortTermDebt,
            Concept::LongTermDebt,
            Concept::CashAndEquivalents,
            Concept::ShortTermInvestments,
            Concept::OperatingIncome,
        ],
        conditions: &[
            NON_POSITIVE_CLOSE,
            NON_POSITIVE_SHARES_OUTSTANDING,
            NON_POSITIVE_ENTERPRISE_VALUE,
            ZERO_OPERATING_INCOME,
            NEGATIVE_OPERATING_INCOME,
            START_NOT_A_DATE,
            END_NOT_A_DATE,
            NOT_A_YEAR,
            OVERLAPS_ANOTHER_YEAR,
            SHARE_BASIS_DATE_NOT_A_DATE,
        ],
    },
    readings: &[
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        Some(Reading::Averaged),
        None,
    ],
    derived: enterprise_value_to_ebit,
};

/// The year test, so that a year's price is the only one read, then the
/// reads and the price read side by side, then the market capitalisation
/// term's two conditions where the term is formed, then the averaged
/// enterprise value's one, then operating income's two, then the one
/// division.
fn enterprise_value_to_ebit(computing: &Computing) -> Result<Number, Reasons> {
    year(computing).map_err(|reason| Reasons::of(reason, Vec::new()))?;
    let (
        [
            shares_outstanding,
            short_term_debt,
            long_term_debt,
            cash_and_equivalents,
            short_term_investments,
            operating_income,
        ],
        close,
    ) = beside_the_close(
        reads_with_nothing(
            computing,
            &ENTERPRISE_VALUE_TO_EBIT,
            NOTHING_WHERE_NOT_APPLICABLE,
        ),
        computing.close_at_valuation_date(),
    )?;
    let enterprise_value = enterprise_value(
        &close,
        &shares_outstanding,
        &short_term_debt,
        &long_term_debt,
        &cash_and_equivalents,
        &short_term_investments,
    )?
    .mean();
    if enterprise_value.sign() != Ordering::Greater {
        return Err(declined(NON_POSITIVE_ENTERPRISE_VALUE));
    }
    let operating_income = operating_income.mean();
    match operating_income.sign() {
        Ordering::Equal => Err(declined(ZERO_OPERATING_INCOME)),
        Ordering::Less => Err(declined(NEGATIVE_OPERATING_INCOME)),
        Ordering::Greater => Ok(enterprise_value
            .divided(&operating_income)
            .expect("operating income is above zero")),
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::canonical_concepts::Concept;

    use super::{ENTERPRISE_VALUE_TO_EBIT, NEGATIVE_OPERATING_INCOME, ZERO_OPERATING_INCOME};
    use crate::arithmetic::INPUT_NOT_A_DECIMAL;
    use crate::date::START_NOT_A_DATE;
    use crate::metrics::discounted_cash_flow_margin_of_safety::NON_POSITIVE_CLOSE;
    use crate::metrics::enterprise_value::NON_POSITIVE_ENTERPRISE_VALUE;
    use crate::metrics::market_capitalisation::NON_POSITIVE_SHARES_OUTSTANDING;
    use crate::metrics::reading::Reading;
    use crate::metrics::reading::price::SHARE_BASIS_DATE_NOT_A_DATE;
    use crate::metrics::reading::year::{END_NOT_A_DATE, NOT_A_YEAR, OVERLAPS_ANOTHER_YEAR};
    use crate::metrics::{DERIVATIONS, Shape, Unit};

    #[test]
    fn the_entry_states_the_metric_once() {
        let entry = &ENTERPRISE_VALUE_TO_EBIT.entry;
        assert_eq!(entry.name, "enterprise_value_to_ebit");
        assert_eq!(
            entry.meaning,
            "Enterprise value over operating income for the year. Enterprise value is the \
             average, at the instants the year opens and closes at, of market capitalisation, \
             the unadjusted close at the valuation date times common shares outstanding, plus \
             net debt, short-term and long-term debt less cash and equivalents and short-term \
             investments; the preferred claim and non-controlling interests are not counted."
        );
        assert_eq!(entry.unit, Unit::Multiple);
        assert_eq!(entry.stated_at, Shape::Duration);
        assert_eq!(
            entry.inputs,
            [
                Concept::SharesOutstanding,
                Concept::ShortTermDebt,
                Concept::LongTermDebt,
                Concept::CashAndEquivalents,
                Concept::ShortTermInvestments,
                Concept::OperatingIncome
            ]
        );
        assert_eq!(
            ENTERPRISE_VALUE_TO_EBIT.readings,
            [
                Some(Reading::Averaged),
                Some(Reading::Averaged),
                Some(Reading::Averaged),
                Some(Reading::Averaged),
                Some(Reading::Averaged),
                None
            ]
        );
        assert_eq!(
            entry.declined().collect::<Vec<_>>(),
            [
                "non_positive_close",
                "non_positive_shares_outstanding",
                "non_positive_enterprise_value",
                "zero_operating_income",
                "negative_operating_income",
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
            ENTERPRISE_VALUE_TO_EBIT.entry.conditions,
            [
                NON_POSITIVE_CLOSE,
                NON_POSITIVE_SHARES_OUTSTANDING,
                NON_POSITIVE_ENTERPRISE_VALUE,
                ZERO_OPERATING_INCOME,
                NEGATIVE_OPERATING_INCOME,
                START_NOT_A_DATE,
                END_NOT_A_DATE,
                NOT_A_YEAR,
                OVERLAPS_ANOTHER_YEAR,
                SHARE_BASIS_DATE_NOT_A_DATE,
            ]
        );
    }

    #[test]
    fn it_reads_neither_the_preferred_claim_nor_depreciation_and_amortisation() {
        for unread in [
            Concept::PreferredEquity,
            Concept::DepreciationAndAmortization,
        ] {
            assert!(
                !ENTERPRISE_VALUE_TO_EBIT.entry.inputs.contains(&unread),
                "{unread:?}"
            );
        }
    }

    #[test]
    fn its_name_says_neither_common_nor_per_share_and_its_meaning_says_what_is_not_counted() {
        let entry = &ENTERPRISE_VALUE_TO_EBIT.entry;
        assert!(!entry.name.contains("common"));
        assert!(!entry.name.contains("per_share"));
        assert!(entry.meaning.contains("common shares outstanding"));
        assert!(
            entry
                .meaning
                .contains("the preferred claim and non-controlling interests are not counted")
        );
    }

    #[test]
    fn the_hand_over_carries_the_entrys_name_once() {
        let named = DERIVATIONS
            .iter()
            .filter(|derivation| derivation.entry.name == ENTERPRISE_VALUE_TO_EBIT.entry.name)
            .count();
        assert_eq!(named, 1);
    }
}
