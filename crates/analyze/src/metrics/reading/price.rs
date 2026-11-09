//! The one place the derivation reaches the crossing, as `reading.rs` is the
//! one place it reaches the history. The share-basis rule's three clauses are
//! `docs/adr/what-analyze-takes-and-returns.md`'s, and the comparisons it
//! makes and the condition it declines under are
//! `docs/adr/share-basis-rule-dates.md`'s, all under the method version.
//!
//! Every date is read through `Date::read` and compared through
//! `Date::on_or_before`. Nothing is sorted, no row of the history is found,
//! and no date asked is read.

use std::iter;

use vfi_contracts::analyze_store::{CoveredRange, Prices, Reason};
use vfi_contracts::fetch_analyze::{Answer, Crossing, Splits};

use crate::arithmetic::{INPUT_NOT_A_DECIMAL, Number};
use crate::date::Date;

/// The `declined` condition of the share-basis rule at a date it compares
/// that is outside the grammar. It is `declined` because the price was
/// answered and the rule is undefined at a date that states no day, as
/// `docs/adr/share-basis-rule-dates.md` names it.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "no metric reads a price before the margin")
)]
pub(in crate::metrics) const SHARE_BASIS_DATE_NOT_A_DATE: &str = "share_basis_date_not_a_date";

/// Nothing where a close traded `trading_date` can be put beside a figure
/// whose period ends `period_end`; otherwise the one reason of the first
/// clause that fails or cannot be read. This is "the one function every
/// price-dependent metric calls", `docs/adr/what-analyze-takes-and-returns.md`
/// says, and its clauses are that record's:
///
/// 1. The split list is present. No date is read.
/// 2. The covered range, among the trading dates of the prices `crossing`
///    carries, an absence widening nothing, starts on or before the earlier
///    of `period_end` and `trading_date` and ends on or after the later.
/// 3. The list holds no split dated on or after the earlier. The first the
///    list holds, in its order, is the one named.
///
/// The close traded `trading_date` is a price in `crossing`, so a covered
/// range exists. `docs/adr/share-basis-rule-dates.md` fixes the comparisons
/// and that each clause reads its dates before comparing any.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "no metric reads a price before the margin")
)]
pub(in crate::metrics) fn share_basis(
    crossing: &Crossing,
    trading_date: &str,
    period_end: &str,
) -> Result<(), Reason> {
    let Splits::Listed { factors } = &crossing.splits else {
        return Err(Reason::splits_absent(crossing).expect("the split list is an absence"));
    };

    let read = |date: &str| {
        Date::read(date).ok_or_else(|| Reason::declined(SHARE_BASIS_DATE_NOT_A_DATE.into()))
    };

    let end = read(period_end)?;
    let traded = read(trading_date)?;
    let prices = iter::once(&crossing.at_valuation_date)
        .chain(&crossing.at_earlier_dates)
        .filter_map(|answer| match answer {
            Answer::Price {
                trading_date: date, ..
            } => Some(read(date)),
            Answer::Absence { .. } => None,
        })
        .collect::<Result<Vec<Date>, Reason>>()?;
    let (earlier, later) = if end.on_or_before(traded) {
        (end, traded)
    } else {
        (traded, end)
    };
    let (from, through) = prices
        .into_iter()
        .map(|date| (date, date))
        .reduce(|(from, through), (date, _)| {
            (
                if from.on_or_before(date) { from } else { date },
                if date.on_or_before(through) {
                    through
                } else {
                    date
                },
            )
        })
        .expect("the close traded `trading_date` is a price in the crossing");
    if !(from.on_or_before(earlier) && later.on_or_before(through)) {
        return Err(Reason::not_covered(
            CoveredRange {
                from: from.written(),
                through: through.written(),
            },
            period_end.into(),
            trading_date.into(),
        ));
    }

    let splits = factors
        .iter()
        .map(|split| Ok((split, read(&split.date)?)))
        .collect::<Result<Vec<_>, Reason>>()?;
    splits
        .iter()
        .find(|(_, date)| earlier.on_or_before(*date))
        .map_or(Ok(()), |(split, _)| {
            Err(
                Reason::split_within(crossing, split, period_end.into(), trading_date.into())
                    .expect("the split is one the list holds"),
            )
        })
}

/// The close at the valuation date, to be put beside a figure whose period
/// ends `period_end`, or the one reason there is none, in the order
/// `docs/adr/discounted-cash-flow-and-margin-of-safety.md` lists under "The
/// price read": no crossing, an absence, the share-basis rule, then the
/// close's grammar.
///
/// The answer is read by its place in the crossing, so `date_not_asked` has
/// no witness here. No other date, adjusted close or stand-in is read.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "no metric reads a price before the margin")
)]
pub(in crate::metrics) fn close_at_valuation_date(
    crossing: Option<&Crossing>,
    period_end: &str,
) -> Result<Number, Reason> {
    let crossing = crossing
        .ok_or_else(|| Reason::not_asked(&Prices::NotAsked {}).expect("no price was asked"))?;
    match &crossing.at_valuation_date {
        Answer::Absence { .. } => Err(Reason::answered_absent(crossing, crossing.valuation_date())
            .expect("the valuation date's answer is an absence")),
        Answer::Price {
            trading_date,
            close,
            ..
        } => {
            share_basis(crossing, trading_date, period_end)?;
            Number::read(close).ok_or_else(|| Reason::declined(INPUT_NOT_A_DECIMAL.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use vfi_contracts::analyze_store::{CoveredRange, Prices, Reason};
    use vfi_contracts::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

    use super::{SHARE_BASIS_DATE_NOT_A_DATE, close_at_valuation_date, share_basis};
    use crate::arithmetic::{INPUT_NOT_A_DECIMAL, Number};

    const E: &str = "2024-12-31";
    const T: &str = "2025-06-30";

    fn asked_and_traded(asked: &str, traded: &str, close: &str) -> Answer {
        Answer::Price {
            date_asked: asked.into(),
            trading_date: traded.into(),
            close: close.into(),
            retrieved_from: "https://example.invalid/prices".into(),
        }
    }

    fn traded(date: &str) -> Answer {
        asked_and_traded(date, date, "60")
    }

    fn absent(date: &str) -> Answer {
        Answer::Absence {
            date_asked: date.into(),
            reason: fetch_analyze::Reason::NoClose {},
        }
    }

    fn split(date: &str) -> SplitFactor {
        SplitFactor {
            date: date.into(),
            factor: "4.0".into(),
        }
    }

    fn listed(dates: &[&str]) -> Splits {
        Splits::Listed {
            factors: dates.iter().map(|date| split(date)).collect(),
        }
    }

    fn no_list() -> Splits {
        Splits::Absence {
            reason: fetch_analyze::Reason::NoKey {},
        }
    }

    fn crossing(valuation: Answer, earlier: Vec<Answer>, splits: Splits) -> Crossing {
        Crossing {
            cik: "0000000001".into(),
            ticker: "VFI".into(),
            at_valuation_date: valuation,
            at_earlier_dates: earlier,
            splits,
        }
    }

    fn preset_with(splits: Splits) -> Crossing {
        crossing(traded(T), vec![traded(E)], splits)
    }

    fn preset() -> Crossing {
        preset_with(listed(&[]))
    }

    fn not_covered(from: &str, through: &str, end: &str, traded: &str) -> Result<(), Reason> {
        Err(Reason::not_covered(
            CoveredRange {
                from: from.into(),
                through: through.into(),
            },
            end.into(),
            traded.into(),
        ))
    }

    fn split_within(
        crossing: &Crossing,
        date: &str,
        end: &str,
        traded: &str,
    ) -> Result<(), Reason> {
        Err(
            Reason::split_within(crossing, &split(date), end.into(), traded.into())
                .expect("the split is one the list holds"),
        )
    }

    fn splits_absent(crossing: &Crossing) -> Result<(), Reason> {
        Err(Reason::splits_absent(crossing).expect("the split list is an absence"))
    }

    fn not_a_date() -> Result<(), Reason> {
        Err(Reason::declined(SHARE_BASIS_DATE_NOT_A_DATE.into()))
    }

    #[test]
    fn the_preset_crossing_passes() {
        assert_eq!(share_basis(&preset(), T, E), Ok(()));
    }

    #[test]
    fn an_absent_split_list_is_splits_absent() {
        let crossing = preset_with(no_list());
        assert_eq!(share_basis(&crossing, T, E), splits_absent(&crossing));
    }

    #[test]
    fn a_range_that_starts_after_the_period_end_is_not_covered() {
        let crossing = crossing(traded(T), vec![traded("2025-03-31")], listed(&[]));
        assert_eq!(
            share_basis(&crossing, T, E),
            not_covered("2025-03-31", T, E, T)
        );
    }

    #[test]
    fn a_split_inside_the_span_is_split_within() {
        let crossing = preset_with(listed(&["2025-03-14"]));
        assert_eq!(
            share_basis(&crossing, T, E),
            split_within(&crossing, "2025-03-14", E, T)
        );
    }

    #[test]
    fn a_split_dated_the_earlier_is_split_within() {
        let crossing = preset_with(listed(&[E]));
        assert_eq!(
            share_basis(&crossing, T, E),
            split_within(&crossing, E, E, T)
        );
    }

    #[test]
    fn a_split_the_day_before_the_earlier_passes() {
        assert_eq!(
            share_basis(&preset_with(listed(&["2024-12-30"])), T, E),
            Ok(())
        );
    }

    #[test]
    fn the_valuation_date_alone_is_a_one_day_range() {
        let crossing = crossing(traded(T), vec![], listed(&[]));
        assert_eq!(share_basis(&crossing, T, E), not_covered(T, T, E, T));
    }

    #[test]
    fn one_price_on_the_period_end_passes_until_a_split_is_dated_that_day() {
        let alone = crossing(traded(E), vec![], listed(&[]));
        assert_eq!(share_basis(&alone, E, E), Ok(()));

        let split_that_day = crossing(traded(E), vec![], listed(&[E]));
        assert_eq!(
            share_basis(&split_that_day, E, E),
            split_within(&split_that_day, E, E, E)
        );
    }

    #[test]
    fn a_period_end_after_every_price_is_not_covered() {
        let crossing = crossing(
            traded("2024-06-28"),
            vec![traded("2023-12-29")],
            listed(&[]),
        );
        assert_eq!(
            share_basis(&crossing, "2024-06-28", E),
            not_covered("2023-12-29", "2024-06-28", E, "2024-06-28")
        );
    }

    #[test]
    fn an_absence_widens_nothing() {
        let crossing = crossing(
            traded(T),
            vec![absent(E), traded("2025-03-31")],
            listed(&[]),
        );
        assert_eq!(
            share_basis(&crossing, T, E),
            not_covered("2025-03-31", T, E, T)
        );
    }

    #[test]
    fn each_date_outside_the_grammar_declines() {
        let paired = crossing(traded("2025-6-30"), vec![traded(E)], listed(&[]));
        assert_eq!(share_basis(&paired, "2025-6-30", E), not_a_date());

        let earlier = crossing(traded(T), vec![traded("20241231")], listed(&[]));
        assert_eq!(share_basis(&earlier, T, E), not_a_date());

        let split_date = preset_with(listed(&["2025-02-30"]));
        assert_eq!(share_basis(&split_date, T, E), not_a_date());

        assert_eq!(share_basis(&preset(), T, "2024-12-1"), not_a_date());
    }

    #[test]
    fn the_first_clause_reads_no_date() {
        let crossing = crossing(traded("2025-6-30"), vec![traded(E)], no_list());
        assert_eq!(
            share_basis(&crossing, "2025-6-30", E),
            splits_absent(&crossing)
        );
    }

    #[test]
    fn the_second_clause_comes_before_a_split_it_cannot_read() {
        let crossing = crossing(traded(T), vec![], listed(&["2025-02-30"]));
        assert_eq!(share_basis(&crossing, T, E), not_covered(T, T, E, T));
    }

    #[test]
    fn every_split_is_read_before_any_is_compared() {
        let crossing = preset_with(listed(&["2025-03-14", "2025-02-30"]));
        assert_eq!(share_basis(&crossing, T, E), not_a_date());
    }

    #[test]
    fn no_date_asked_is_read() {
        let valuation_asked = crossing(
            asked_and_traded("2025-6-30", T, "60"),
            vec![traded(E)],
            listed(&[]),
        );
        assert_eq!(share_basis(&valuation_asked, T, E), Ok(()));

        let earlier_asked = crossing(traded(T), vec![traded(E), absent("20241231")], listed(&[]));
        assert_eq!(share_basis(&earlier_asked, T, E), Ok(()));
    }

    #[test]
    fn the_earlier_answers_in_either_order_give_the_same_outcome() {
        let answers = [traded(E), traded("2025-03-31")];
        let reversed = [traded("2025-03-31"), traded(E)];
        for earlier in [answers, reversed] {
            let crossing = crossing(traded(T), earlier.to_vec(), listed(&[]));
            assert_eq!(share_basis(&crossing, T, E), Ok(()));
        }

        let answers = [traded("2025-03-31"), traded("2025-04-30")];
        let reversed = [traded("2025-04-30"), traded("2025-03-31")];
        for earlier in [answers, reversed] {
            let crossing = crossing(traded(T), earlier.to_vec(), listed(&[]));
            assert_eq!(
                share_basis(&crossing, T, E),
                not_covered("2025-03-31", T, E, T)
            );
        }
    }

    #[test]
    fn the_first_split_the_list_holds_is_named_in_each_order() {
        for dates in [["2025-01-15", "2025-03-14"], ["2025-03-14", "2025-01-15"]] {
            let crossing = preset_with(listed(&dates));
            assert_eq!(
                share_basis(&crossing, T, E),
                split_within(&crossing, dates[0], E, T)
            );
        }
    }

    #[test]
    fn no_crossing_is_not_asked() {
        assert_eq!(
            close_at_valuation_date(None, E),
            Err(Reason::not_asked(&Prices::NotAsked {}).expect("no price was asked"))
        );
    }

    #[test]
    fn an_absence_at_the_valuation_date_is_answered_absent() {
        let crossing = crossing(absent(T), vec![traded(E)], listed(&[]));
        assert_eq!(
            close_at_valuation_date(Some(&crossing), E),
            Err(Reason::answered_absent(&crossing, T).expect("the answer is an absence"))
        );
    }

    #[test]
    fn the_preset_crossing_reads_the_close() {
        assert_eq!(
            close_at_valuation_date(Some(&preset()), E),
            Ok(Number::read("60").expect("60 is a decimal"))
        );
    }

    #[test]
    fn a_close_outside_the_grammar_declines() {
        let crossing = crossing(asked_and_traded(T, T, "1e2"), vec![traded(E)], listed(&[]));
        assert_eq!(
            close_at_valuation_date(Some(&crossing), E),
            Err(Reason::declined(INPUT_NOT_A_DECIMAL.into()))
        );
    }

    #[test]
    fn the_rule_comes_before_the_close_is_read() {
        let crossing = crossing(asked_and_traded(T, T, "1e2"), vec![traded(E)], no_list());
        assert_eq!(
            close_at_valuation_date(Some(&crossing), E),
            Err(Reason::splits_absent(&crossing).expect("the split list is an absence"))
        );
    }

    #[test]
    fn the_condition_is_named_as_the_record_names_it() {
        assert_eq!(SHARE_BASIS_DATE_NOT_A_DATE, "share_basis_date_not_a_date");
    }
}
