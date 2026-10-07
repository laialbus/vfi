//! The analyze → store contract at v1: what the caller of analyze hands store
//! for one filer.
//!
//! `contracts/analyze-store/v1.toml` is the surface and it is frozen. This is
//! that surface as Rust — the [`HandOver`], the [`Premises`] its results were
//! computed under, the [`Results`] at each period, the two shapes an
//! [`Outcome`] takes, and the closed set a [`Reason`] names. Why a result
//! crosses this way is `docs/adr/what-analyze-takes-and-returns.md`, and none of
//! that argument is repeated here.
//!
//! The history and the crossing are not restated. They are the
//! [`canonical_concepts`](crate::canonical_concepts) and
//! [`fetch_analyze`](crate::fetch_analyze) types, carried whole, so each stays
//! in one copy and a reason can name a state in them rather than copy it. A
//! period is that first contract's [`Period`], which the published bytes
//! restate and the comparison below holds them to.
//!
//! A [`Reason`] is had only from its constructors, and anything can build the
//! [`Names`] store reads off one. Where this crate holds the state a reason
//! names — a concept's state in a [`Row`], the hand-over's [`Prices`], the
//! crossing's answers and split list — the constructor takes that state and
//! hands out nothing when it is some other one, the way v2's two absences are
//! built. Where it holds no such state, the constructor says so.

use std::num::NonZeroU32;

use crate::canonical_concepts::{Concept, History, Period, Resolution, Row};
use crate::fetch_analyze::{Answer, Crossing, SplitFactor, Splits};

carries! {
    /// What the caller of analyze hands store for one filer.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct HandOver {
        fields {
            /// The history analyze read. Its CIK names the filer, and is stated
            /// nowhere else here, so the two cannot disagree.
            history: History,
            prices: Prices,
            premises: Premises,
            /// Each period once. The order means nothing.
            results: Vec<Results>,
        }
    }
}

impl HandOver {
    /// The hand-over, or nothing where the crossing names a filer other than
    /// the history's or two of the results share a period.
    pub fn of(
        history: History,
        prices: Prices,
        premises: Premises,
        results: Vec<Results>,
    ) -> Option<Self> {
        let one_filer = match &prices {
            Prices::Asked { crossing } => *crossing.cik == *history.filer(),
            Prices::NotAsked {} => true,
        };
        (one_filer && each_once(results.iter().map(Results::period))).then_some(HandOver {
            history,
            prices,
            premises,
            results,
        })
    }

    /// The filer, as the ten digits its history carries.
    pub fn filer(&self) -> &str {
        self.history.filer()
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn prices(&self) -> &Prices {
        &self.prices
    }

    pub fn premises(&self) -> &Premises {
        &self.premises
    }

    /// Each period once. The order means nothing.
    pub fn results(&self) -> &[Results] {
        &self.results
    }
}

shapes! {
    /// The price crossing analyze read, or that none was asked: one of these,
    /// never both and never neither.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Prices {
        Asked { crossing: Crossing },
        NotAsked {},
    }
}

carries! {
    /// What every result in a hand-over was computed under, so that a replay
    /// reads the settings used and never the presets.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Premises {
        fields {
            method_version: NonZeroU32,
            /// Each name once. The order means nothing.
            settings: Vec<Setting>,
        }
    }
}

impl Premises {
    /// The premises, or nothing where two settings share a name.
    pub fn under(method_version: NonZeroU32, settings: Vec<Setting>) -> Option<Self> {
        each_once(settings.iter().map(|setting| &setting.name)).then_some(Premises {
            method_version,
            settings,
        })
    }

    pub fn method_version(&self) -> NonZeroU32 {
        self.method_version
    }

    /// Each name once. The order means nothing.
    pub fn settings(&self) -> &[Setting] {
        &self.settings
    }
}

carries! {
    /// One setting by name, with the value used, preset or chosen alike.
    ///
    /// The value crosses as the characters analyze writes, for the reason a
    /// figure does: this crate depends on nothing that could hold one
    /// losslessly.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Setting {
        fields {
            pub name: Box<str>,
            pub value: Box<str>,
        }
    }
}

carries! {
    /// The results at one period.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Results {
        fields {
            period: Period,
            /// Each metric once. The order means nothing.
            metrics: Vec<Metric>,
        }
    }
}

impl Results {
    /// The results at `period`, or nothing where two metrics share a name.
    pub fn at(period: Period, metrics: Vec<Metric>) -> Option<Self> {
        each_once(metrics.iter().map(|metric| &metric.name)).then_some(Results { period, metrics })
    }

    pub fn period(&self) -> &Period {
        &self.period
    }

    /// Each metric once. The order means nothing.
    pub fn metrics(&self) -> &[Metric] {
        &self.metrics
    }
}

carries! {
    /// One metric and its outcome. The name is the method's: this crate knows
    /// no metric.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Metric {
        fields {
            pub name: Box<str>,
            pub outcome: Outcome,
        }
    }
}

shapes! {
    /// What a metric came to at a period: one of these, never both and never
    /// neither.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Outcome {
        /// The figure, as the characters analyze writes. Its unit is the
        /// metric's catalogue entry's.
        Value { figure: Box<str> },
        /// One reason per absent input, in the metric's own input order.
        Absent { reasons: Reasons },
    }
}

/// The reasons a metric is absent: at least one, which the split into a first
/// and the rest holds rather than checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reasons {
    first: Reason,
    rest: Vec<Reason>,
}

impl Reasons {
    pub fn of(first: Reason, rest: Vec<Reason>) -> Self {
        Reasons { first, rest }
    }

    /// Every reason, in the order they were given.
    pub fn each(&self) -> impl Iterator<Item = &Reason> {
        std::iter::once(&self.first).chain(&self.rest)
    }
}

/// Why a metric is absent, as one of the closed set [`Names`] states.
///
/// The field is private so that the constructors below are the only way to
/// have one, which is what lets each hold its witness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reason(Names);

shapes! {
    /// What a reason names: one of these. The set is closed, and none of them
    /// is an error.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Names {
        InputNotApplicable { concept: Concept, period: Period },
        InputUnknown { concept: Concept, period: Period },
        NoPeriod { computing_at: Period, sought_by: Box<str> },
        NoPrice { no_price: NoPrice },
        BasisUnproven { basis_unproven: BasisUnproven },
        Declined { condition: Box<str> },
    }
}

shapes! {
    /// How a metric that reads a price at a date has none: one of these.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum NoPrice {
        NotAsked {},
        DateNotAsked { date: Box<str> },
        AnsweredAbsent { date_asked: Box<str> },
    }
}

shapes! {
    /// How an answered price cannot be shown to be on the figure's share
    /// basis: one of these.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum BasisUnproven {
        SplitsAbsent {},
        NotCovered {
            covered_range: CoveredRange,
            period_end: Box<str>,
            trading_date: Box<str>,
        },
        SplitWithin {
            split: SplitFactor,
            period_end: Box<str>,
            trading_date: Box<str>,
        },
    }
}

carries! {
    /// The covered range, by the earliest and latest trading dates the
    /// crossing's prices carry.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct CoveredRange {
        fields {
            pub from: Box<str>,
            pub through: Box<str>,
        }
    }
}

impl Reason {
    /// `concept` is `NotApplicable` at `row`'s period, or nothing where it is
    /// in another state. The kind and clause stay in the history.
    pub fn input_not_applicable(row: &Row, concept: Concept) -> Option<Self> {
        matches!(row.of(concept), Resolution::NotApplicable { .. }).then(|| {
            Reason(Names::InputNotApplicable {
                concept,
                period: row.period().clone(),
            })
        })
    }

    /// `concept` is `Unknown` at `row`'s period, or nothing where it is in
    /// another state. What was attempted stays in the history.
    pub fn input_unknown(row: &Row, concept: Concept) -> Option<Self> {
        matches!(row.of(concept), Resolution::Unknown { .. }).then(|| {
            Reason(Names::InputUnknown {
                concept,
                period: row.period().clone(),
            })
        })
    }

    /// The history holds no row for a period the metric reads.
    ///
    /// A missing row is no state the history holds, and the period sought is
    /// the metric's rule's, so this carries what the record names and checks
    /// nothing.
    pub fn no_period(computing_at: Period, sought_by: Box<str>) -> Self {
        Reason(Names::NoPeriod {
            computing_at,
            sought_by,
        })
    }

    /// No price was asked, or nothing where one was.
    pub fn not_asked(prices: &Prices) -> Option<Self> {
        matches!(prices, Prices::NotAsked {}).then(|| {
            Reason(Names::NoPrice {
                no_price: NoPrice::NotAsked {},
            })
        })
    }

    /// `crossing` holds no answer for `date`, or nothing where it holds one.
    /// A date asked is `date` when the two are equal character for character.
    pub fn date_not_asked(crossing: &Crossing, date: &str) -> Option<Self> {
        answer_at(crossing, date).is_none().then(|| {
            Reason(Names::NoPrice {
                no_price: NoPrice::DateNotAsked { date: date.into() },
            })
        })
    }

    /// `crossing`'s answer for `date` is an absence, or nothing where it is a
    /// price or there is none. The absence's reason stays in the crossing.
    pub fn answered_absent(crossing: &Crossing, date: &str) -> Option<Self> {
        match answer_at(crossing, date)? {
            Answer::Absence { date_asked, .. } => Some(Reason(Names::NoPrice {
                no_price: NoPrice::AnsweredAbsent {
                    date_asked: date_asked.clone(),
                },
            })),
            Answer::Price { .. } => None,
        }
    }

    /// `crossing`'s split list is an absence, or nothing where it is listed.
    /// The absence's reason stays in the crossing.
    pub fn splits_absent(crossing: &Crossing) -> Option<Self> {
        matches!(crossing.splits, Splits::Absence { .. }).then(|| {
            Reason(Names::BasisUnproven {
                basis_unproven: BasisUnproven::SplitsAbsent {},
            })
        })
    }

    /// The covered range does not contain both dates.
    ///
    /// The range and whether it contains a date are read by comparing dates,
    /// which this crate does not read, so this carries what the record names
    /// and checks nothing.
    pub fn not_covered(
        covered_range: CoveredRange,
        period_end: Box<str>,
        trading_date: Box<str>,
    ) -> Self {
        Reason(Names::BasisUnproven {
            basis_unproven: BasisUnproven::NotCovered {
                covered_range,
                period_end,
                trading_date,
            },
        })
    }

    /// `split` is in `crossing`'s list, or nothing where the list is absent or
    /// does not hold it.
    ///
    /// A split has no identity in its list but its date and factor, so it is
    /// named by them. Whether it falls inside the span the share-basis rule
    /// reads is a comparison of dates, which this crate does not read, so that
    /// half is not checked here.
    pub fn split_within(
        crossing: &Crossing,
        split: &SplitFactor,
        period_end: Box<str>,
        trading_date: Box<str>,
    ) -> Option<Self> {
        let listed = match &crossing.splits {
            Splits::Listed { factors } => factors.contains(split),
            Splits::Absence { .. } => false,
        };
        listed.then(|| {
            Reason(Names::BasisUnproven {
                basis_unproven: BasisUnproven::SplitWithin {
                    split: split.clone(),
                    period_end,
                    trading_date,
                },
            })
        })
    }

    /// Every input was present and the metric's definition states it is
    /// undefined at them.
    ///
    /// The condition is the metric's catalogue entry's, which this crate does
    /// not hold, so this carries the name it is given and checks nothing.
    pub fn declined(condition: Box<str>) -> Self {
        Reason(Names::Declined { condition })
    }

    /// What this reason names.
    pub fn names(&self) -> &Names {
        &self.0
    }
}

/// `crossing`'s answer for `date`, of which a crossing holds at most one.
fn answer_at<'a>(crossing: &'a Crossing, date: &str) -> Option<&'a Answer> {
    std::iter::once(&crossing.at_valuation_date)
        .chain(&crossing.at_earlier_dates)
        .find(|answer| answer.date_asked() == date)
}

fn each_once<T: PartialEq>(items: impl Iterator<Item = T>) -> bool {
    let items: Vec<T> = items.collect();
    items
        .iter()
        .enumerate()
        .all(|(at, item)| !items[..at].contains(item))
}

/// The types above against the bytes they transcribe.
///
/// The contracts gate digests `v1.toml` and never reads it, so a type that
/// drifted from the surface it states would stay green on every other gate.
/// This is the comparison that closes that: a field, a shape or a reason
/// renamed, added or dropped on either side leaves the two readings unequal,
/// and the failure prints both.
#[cfg(test)]
mod states_what_is_published {
    use super::{
        BasisUnproven, CoveredRange, HandOver, Metric, Names, NoPrice, Outcome, Premises, Prices,
        Results, Setting,
    };
    use crate::canonical_concepts::Period;
    use crate::published::{as_published, Contract};

    /// The file this module states, relative to the repository root, named here
    /// and nowhere else in this module.
    const PATH: &str = "contracts/analyze-store/v1.toml";

    fn published() -> Contract {
        Contract::at(PATH)
    }

    #[test]
    fn each_value_carries_the_fields_the_published_bytes_name() {
        let published = published();
        for (declared, table) in [
            (HandOver::FIELDS, "handover.field"),
            (Premises::FIELDS, "premises.field"),
            (Setting::FIELDS, "setting.field"),
            (Results::FIELDS, "results.field"),
            (Metric::FIELDS, "metric.field"),
            (CoveredRange::FIELDS, "covered_range.field"),
        ] {
            assert_eq!(
                declared,
                published.names_under(table).as_slice(),
                "the fields declared are not the [[{table}]] names {PATH} states"
            );
        }
    }

    /// Which shapes each takes, and what each shape carries, against the
    /// entries that state a name and a carries line apiece.
    #[test]
    fn each_shape_and_reason_carries_what_the_published_bytes_name() {
        let published = published();
        for (shapes, table) in [
            (Prices::SHAPES, "prices.shape"),
            (Outcome::SHAPES, "outcome.shape"),
            (Names::SHAPES, "reason"),
            (NoPrice::SHAPES, "no_price.shape"),
            (BasisUnproven::SHAPES, "basis_unproven.shape"),
        ] {
            let declared: Vec<(String, Vec<&str>)> = shapes
                .iter()
                .map(|(shape, carried)| (as_published(shape), carried.to_vec()))
                .collect();
            let stated: Vec<(String, Vec<&str>)> = published
                .occurrences(table)
                .iter()
                .map(|pairs| {
                    let name = published.unquoted(published.value_of(pairs, "name", table));
                    let carried = published
                        .members(published.value_of(pairs, "carries", table))
                        .into_iter()
                        .map(|field| published.unquoted(field))
                        .collect();
                    (name.to_owned(), carried)
                })
                .collect();

            assert_eq!(
                declared, stated,
                "the shapes declared are not the [[{table}]] entries {PATH} states"
            );
        }
    }

    /// The period is v2's type, so what is compared is that the restatement in
    /// these bytes names the shapes and date counts that type declares.
    #[test]
    fn a_period_takes_the_shapes_the_published_bytes_restate() {
        let declared: Vec<(String, usize)> = Period::SHAPES
            .iter()
            .map(|(shape, dates)| (as_published(shape), dates.len()))
            .collect();

        let published = published();
        let stated: Vec<(String, usize)> = published
            .occurrences("period.shape")
            .iter()
            .map(|pairs| {
                let name = published
                    .unquoted(published.value_of(pairs, "name", "[[period.shape]]"))
                    .to_owned();
                let dates = published.value_of(pairs, "dates", "[[period.shape]]");
                let dates = dates.parse().unwrap_or_else(|_| {
                    panic!("{PATH} states `dates = {dates}`, which is no count")
                });
                (name, dates)
            })
            .collect();

        assert_eq!(
            declared, stated,
            "the shapes `Period` declares are not the [[period.shape]] entries {PATH} restates"
        );
    }

    /// The enum closes what the file closes. An enum over a set the file had
    /// reopened would be wrong in a way no name comparison could show.
    #[test]
    fn the_set_the_type_closes_is_the_set_the_published_bytes_close() {
        let published = published();
        let stated = published.keys_of("reason_set");
        assert_eq!(
            published.value_of(&stated, "closed", "reason_set"),
            "true",
            "{PATH} no longer states [reason_set] closed, and the enum closing it rests on that"
        );
    }
}

/// What each reason is built from, and what the hand-over refuses.
#[cfg(test)]
mod the_reasons_are_built_from_their_witnesses {
    use std::num::NonZeroU32;

    use super::{
        BasisUnproven, HandOver, Metric, Names, NoPrice, Outcome, Premises, Prices, Reason,
        Results, Setting,
    };
    use crate::canonical_concepts::{
        Attempt, Attempted, Attempts, Concept, History, Kind, Period, Resolution, Row, SetBy,
        Silence,
    };
    use crate::fetch_analyze::{self, Answer, Crossing, SplitFactor, Splits};

    const CIK: &str = "0000320193";
    const PRICED: &str = "2026-10-01";
    const ANSWERED_ABSENT: &str = "2025-12-31";
    const NOT_ASKED: &str = "2024-12-31";

    fn year_end() -> Period {
        Period::Instant {
            at: ANSWERED_ABSENT.into(),
        }
    }

    fn a_value() -> Resolution {
        Resolution::Value {
            amount: "0".into(),
            set_by: SetBy::Silence {
                reading: Silence::Zero,
                registry: "1".into(),
            },
        }
    }

    fn not_applicable() -> Resolution {
        Resolution::NotApplicable {
            excluded: Concept::GrossProfit
                .definition()
                .applies_to
                .excluding(Kind::Bank)
                .expect("gross_profit applies to operating filers alone"),
        }
    }

    fn unknown() -> Resolution {
        Resolution::Unknown {
            attempted: Attempts::in_filings(
                Attempted::in_filing(
                    "0000320193-25-000079".into(),
                    Attempt::that_ran(Vec::new()),
                ),
                Vec::new(),
            ),
        }
    }

    fn row_where(concept: Concept, state: &Resolution) -> Row {
        Row::at(
            year_end(),
            std::array::from_fn(|at| {
                if Concept::ALL[at] == concept {
                    state.clone()
                } else {
                    a_value()
                }
            }),
        )
    }

    fn split() -> SplitFactor {
        SplitFactor {
            date: "2026-06-02".into(),
            factor: "2:1".into(),
        }
    }

    fn crossing(cik: &str, splits: Splits) -> Crossing {
        Crossing {
            cik: cik.into(),
            ticker: "AAPL".into(),
            at_valuation_date: Answer::Price {
                date_asked: PRICED.into(),
                trading_date: "2026-09-30".into(),
                close: "254.63".into(),
                retrieved_from: "https://example.invalid/prices".into(),
            },
            at_earlier_dates: vec![Answer::Absence {
                date_asked: ANSWERED_ABSENT.into(),
                reason: fetch_analyze::Reason::NoClose {},
            }],
            splits,
        }
    }

    fn listed() -> Splits {
        Splits::Listed {
            factors: vec![split()],
        }
    }

    fn absent_list() -> Splits {
        Splits::Absence {
            reason: fetch_analyze::Reason::NoKey {},
        }
    }

    #[test]
    fn an_input_absence_is_built_only_from_that_state_in_the_history() {
        let concept = Concept::GrossProfit;
        for state in [a_value(), not_applicable(), unknown()] {
            let row = row_where(concept, &state);
            let named = Names::InputNotApplicable {
                concept,
                period: year_end(),
            };
            assert_eq!(
                Reason::input_not_applicable(&row, concept).map(|reason| reason.names().clone()),
                matches!(state, Resolution::NotApplicable { .. }).then_some(named),
                "input_not_applicable was not built exactly from {state:?}"
            );

            let named = Names::InputUnknown {
                concept,
                period: year_end(),
            };
            assert_eq!(
                Reason::input_unknown(&row, concept).map(|reason| reason.names().clone()),
                matches!(state, Resolution::Unknown { .. }).then_some(named),
                "input_unknown was not built exactly from {state:?}"
            );
        }
    }

    /// One answer per date asked: a date with a price has no `no_price`, and
    /// every other date in a crossing has exactly one of the two built from
    /// it.
    #[test]
    fn a_date_in_a_crossing_meets_at_most_one_way_to_have_no_price() {
        let crossing = crossing(CIK, listed());
        let no_price = |reason: Option<Reason>| reason.map(|reason| reason.names().clone());

        for (date, expected_not_asked, expected_absent) in [
            (PRICED, None, None),
            (
                ANSWERED_ABSENT,
                None,
                Some(NoPrice::AnsweredAbsent {
                    date_asked: ANSWERED_ABSENT.into(),
                }),
            ),
            (
                NOT_ASKED,
                Some(NoPrice::DateNotAsked {
                    date: NOT_ASKED.into(),
                }),
                None,
            ),
        ] {
            let wrap = |no_price| Names::NoPrice { no_price };
            assert_eq!(
                no_price(Reason::date_not_asked(&crossing, date)),
                expected_not_asked.map(wrap),
                "date_not_asked at {date}"
            );
            assert_eq!(
                no_price(Reason::answered_absent(&crossing, date)),
                expected_absent.map(wrap),
                "answered_absent at {date}"
            );
        }
    }

    #[test]
    fn not_asked_is_built_only_from_prices_not_asked() {
        assert_eq!(
            Reason::not_asked(&Prices::NotAsked {}).map(|reason| reason.names().clone()),
            Some(Names::NoPrice {
                no_price: NoPrice::NotAsked {},
            })
        );
        let asked = Prices::Asked {
            crossing: crossing(CIK, listed()),
        };
        assert_eq!(Reason::not_asked(&asked), None);
    }

    #[test]
    fn a_split_reason_is_built_only_from_the_crossings_split_list() {
        let with_list = crossing(CIK, listed());
        let without = crossing(CIK, absent_list());

        assert_eq!(Reason::splits_absent(&with_list), None);
        assert_eq!(
            Reason::splits_absent(&without).map(|reason| reason.names().clone()),
            Some(Names::BasisUnproven {
                basis_unproven: BasisUnproven::SplitsAbsent {},
            })
        );

        let within = |crossing: &Crossing, split: &SplitFactor| {
            Reason::split_within(crossing, split, "2025-09-27".into(), "2026-09-30".into())
        };
        assert!(within(&with_list, &split()).is_some());
        assert_eq!(within(&without, &split()), None);
        let unlisted = SplitFactor {
            factor: "4:1".into(),
            ..split()
        };
        assert_eq!(
            within(&with_list, &unlisted),
            None,
            "a split the list does not hold built split_within"
        );
    }

    fn premises() -> Premises {
        Premises::under(NonZeroU32::MIN, Vec::new()).expect("no settings share a name")
    }

    fn results_at(period: Period) -> Results {
        Results::at(period, Vec::new()).expect("no metrics share a name")
    }

    #[test]
    fn a_hand_over_is_one_filers_with_each_period_once() {
        let history = || History::of(CIK.into(), Vec::new());
        let asked = |cik: &str| Prices::Asked {
            crossing: crossing(cik, listed()),
        };

        let of = |prices: Prices, results: Vec<Results>| {
            HandOver::of(history(), prices, premises(), results)
        };
        assert!(of(asked(CIK), vec![results_at(year_end())]).is_some());
        assert!(of(Prices::NotAsked {}, Vec::new()).is_some());
        assert!(
            of(asked("0000789019"), Vec::new()).is_none(),
            "a crossing for another filer was handed over"
        );
        assert!(
            of(
                Prices::NotAsked {},
                vec![results_at(year_end()), results_at(year_end())]
            )
            .is_none(),
            "a period was handed over twice"
        );
    }

    #[test]
    fn a_name_is_stated_once_among_settings_and_among_metrics() {
        let setting = || Setting {
            name: "a".into(),
            value: "1".into(),
        };
        assert!(Premises::under(NonZeroU32::MIN, vec![setting(), setting()]).is_none());

        let metric = || Metric {
            name: "a".into(),
            outcome: Outcome::Value { figure: "1".into() },
        };
        assert!(Results::at(year_end(), vec![metric(), metric()]).is_none());
    }
}
