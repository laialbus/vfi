//! The fetch → analyze contract at v1: the prices one ask of the price provider
//! hands analyze.
//!
//! `contracts/fetch-analyze/v1.toml` is the surface and it is frozen. This is
//! that surface as Rust — the [`Crossing`] one ask produces, the two shapes an
//! [`Answer`] takes, the split list or its absence, and the seven reasons a
//! date has no price. Why prices cross here is
//! `docs/adr/what-analyze-takes-and-returns.md`, what the provider is asked and
//! answers is `docs/adr/price-provider.md`, and none of that argument is
//! repeated here.
//!
//! These are the provider's answer types, defined once: the provider returns
//! them, and whoever asks it attaches the filer, the ticker and which answer is
//! the valuation date's. There is no error type. A date with no price is an
//! [`Answer::Absence`], so nothing that reads this surface can treat a missing
//! price as a failure and stop.
//!
//! The covered range is not a field. It is read off the prices answered, and a
//! copy written beside them would be a claim the prices could contradict.

carries! {
    /// What one ask of the price provider hands analyze.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Crossing {
        fields {
            /// The filer the ask was made on behalf of, as ten digits
            /// left-padded with zeros. The caller's claim: nothing here shows
            /// that the closes are this filer's.
            pub cik: Box<str>,
            /// The ticker the provider was asked for.
            pub ticker: Box<str>,
            /// The answer for the date the ask named as the valuation date. Its
            /// date asked is the valuation date, which is stated nowhere else,
            /// so the two cannot disagree.
            pub at_valuation_date: Answer,
            /// The answer for each earlier date the ask named. The order means
            /// nothing.
            pub at_earlier_dates: Vec<Answer>,
            /// The split factors inside the range the request covered, or why
            /// there is no list.
            pub splits: Splits,
        }
    }
}

impl Crossing {
    /// The valuation date, as the caller named it.
    pub fn valuation_date(&self) -> &str {
        self.at_valuation_date.date_asked()
    }
}

shapes! {
    /// What the provider answered for one date asked: one of these, never both
    /// and never neither.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Answer {
        /// The source published a close on or before the date asked.
        ///
        /// The close crosses as the decimal literal the source published, for
        /// the reason EDGAR's amounts do: a binary float is a lossy reading of
        /// it, and this crate depends on nothing that could hold one
        /// losslessly.
        Price {
            date_asked: Box<str>,
            trading_date: Box<str>,
            close: Box<str>,
            retrieved_from: Box<str>,
        },
        /// There is no close for the date asked.
        Absence { date_asked: Box<str>, reason: Reason },
    }
}

impl Answer {
    /// The date this answers, as the caller named it.
    pub fn date_asked(&self) -> &str {
        match self {
            Answer::Price { date_asked, .. } | Answer::Absence { date_asked, .. } => date_asked,
        }
    }
}

shapes! {
    /// The split factors the source published inside the range the request
    /// covered, or that list's absence: one of these, never both and never
    /// neither.
    ///
    /// An empty list says no split was published inside that range. It says
    /// nothing about a date outside it.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Splits {
        Listed { factors: Vec<SplitFactor> },
        Absence { reason: Reason },
    }
}

carries! {
    /// One split factor, as the source published it.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct SplitFactor {
        fields {
            pub date: Box<str>,
            pub factor: Box<str>,
        }
    }
}

shapes! {
    /// Why a date, or the split list, has no answer. The set is closed, and
    /// none of these is an error.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Reason {
        NoKey {},
        KeyNotAHeaderValue {},
        AllowanceSpent { allowance: Box<str> },
        /// The source answered with `status`, which is not success.
        Refused { status: u16 },
        Unreachable { why: Box<str> },
        /// The source answered and the answer could not be read.
        Unreadable { why: Box<str> },
        /// The answer holds no close on or before the date asked.
        NoClose {},
    }
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
    use super::{Answer, Crossing, Reason, SplitFactor, Splits};
    use crate::published::Contract;

    /// The file this module states, relative to the repository root, named here
    /// and nowhere else in this module.
    const PATH: &str = "contracts/fetch-analyze/v1.toml";

    fn published() -> Contract {
        Contract::at(PATH)
    }

    /// A declared name as the published bytes spell it: `AllowanceSpent` is
    /// `allowance_spent`, and that is the whole of the translation between the
    /// two spellings. A name the rule does not cover leaves the readings
    /// unequal, which is the direction to be wrong in.
    fn as_published(declared: &str) -> String {
        let mut spelled = String::new();
        for (at, letter) in declared.char_indices() {
            if at > 0 && letter.is_ascii_uppercase() {
                spelled.push('_');
            }
            spelled.push(letter.to_ascii_lowercase());
        }
        spelled
    }

    fn shape_names(shapes: &[(&str, &[&str])]) -> Vec<String> {
        shapes.iter().map(|(shape, _)| as_published(shape)).collect()
    }

    fn owned(names: Vec<&str>) -> Vec<String> {
        names.into_iter().map(str::to_owned).collect()
    }

    #[test]
    fn a_crossing_carries_the_fields_the_published_bytes_name() {
        let published = published();
        assert_eq!(
            Crossing::FIELDS,
            published.names_under("crossing.field").as_slice(),
            "the fields `Crossing` declares are not the [[crossing.field]] names {PATH} states"
        );
    }

    /// Which shapes an answer takes, and for each the fields the published
    /// bytes list under that shape's own name.
    #[test]
    fn an_answer_takes_the_shapes_the_published_bytes_name_with_their_fields() {
        let published = published();
        assert_eq!(
            shape_names(Answer::SHAPES),
            owned(published.names_under("answer.shape")),
            "the shapes `Answer` declares are not the [[answer.shape]] names {PATH} states"
        );

        for (shape, carried) in Answer::SHAPES {
            let table = format!("{}.field", as_published(shape));
            assert_eq!(
                *carried,
                published.names_under(&table).as_slice(),
                "the fields `Answer::{shape}` carries are not the [[{table}]] names {PATH} states"
            );
        }
    }

    #[test]
    fn the_split_list_takes_the_shapes_the_published_bytes_name() {
        let published = published();
        assert_eq!(
            shape_names(Splits::SHAPES),
            owned(published.names_under("splits.shape")),
            "the shapes `Splits` declares are not the [[splits.shape]] names {PATH} states"
        );
        assert_eq!(
            SplitFactor::FIELDS,
            published.names_under("split.field").as_slice(),
            "the fields `SplitFactor` declares are not the [[split.field]] names {PATH} states"
        );
    }

    #[test]
    fn the_reasons_are_the_ones_the_published_bytes_name_with_what_each_carries() {
        let published = published();
        let declared: Vec<(String, Vec<&str>)> = Reason::SHAPES
            .iter()
            .map(|(reason, carried)| (as_published(reason), carried.to_vec()))
            .collect();
        let stated: Vec<(String, Vec<&str>)> = published
            .occurrences("reason")
            .iter()
            .map(|pairs| {
                let name = published.unquoted(published.value_of(pairs, "name", "[[reason]]"));
                let carried = published
                    .members(published.value_of(pairs, "carries", "[[reason]]"))
                    .into_iter()
                    .map(|field| published.unquoted(field))
                    .collect();
                (name.to_owned(), carried)
            })
            .collect();

        assert_eq!(
            declared, stated,
            "the reasons `Reason` declares are not the [[reason]] entries {PATH} states"
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
