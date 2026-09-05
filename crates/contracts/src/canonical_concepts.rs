//! The canonical concept vocabulary at v1: what normalize resolves and analyze
//! consumes.
//!
//! `contracts/canonical-concepts/v1.toml` is the surface and it is frozen. This
//! is that surface as Rust — the three states a concept resolves to, the five
//! filer kinds, and the twenty-eight concepts, each read on the four axes the
//! file defines once and names once per concept. Why each concept is in the set
//! and what each reading was argued against is `docs/adr/canonical-concepts.md`
//! and the open-questions record beside it, and none of that argument is
//! repeated here.
//!
//! What the type states is the shape; what the file states is the content. A
//! concept's meaning, a kind's accounting shape, revenue's reading per kind and
//! the two conditional silence clauses are prose, so they stay in the bytes: a
//! copy of them here would be a transcription nothing compares.
//!
//! The two absences are kept apart by what each can be built from, which is the
//! reason for compiling the vocabulary rather than remembering it.
//! `NotApplicable` takes an [`Excluded`], which only a concept's own
//! applicability clause hands out and only for a kind that clause omits.
//! `Unknown` takes an [`Attempt`], which is what a resolution that ran has to
//! show for itself. The two witnesses come from disjoint inputs, so neither
//! absence is reachable from the other's evidence — and a filer whose kind has
//! not been established has no `Kind` to ask a clause about at all, so it
//! resolves through the attempt and never to a correct absence nobody
//! established.

names! {
    /// The accounting shape a filer presents, which decides which concepts
    /// apply to it and, for revenue, what one means.
    ///
    /// A fact about the filer, established before any concept is looked up. A
    /// filer whose shape fits none of these is assigned none, which is why the
    /// producer holds an optional kind and this type has no member for it.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Kind {
        Operating,
        Bank,
        Insurer,
        Reit,
        InvestmentCompany,
    }
}

names! {
    /// Whether a concept is measured over a period or at an instant.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Measure {
        Flow,
        Balance,
    }
}

names! {
    /// What a concept's figure counts.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Unit {
        Currency,
        CurrencyPerShare,
        Shares,
    }
}

names! {
    /// How a concept's figure is signed.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Sign {
        AsPresented,
        Magnitude,
    }
}

names! {
    /// What a period's facts carrying nothing a concept resolves from reads as.
    ///
    /// `Conditional` names the reading, not the condition: the file publishes
    /// that with the concept, as `zero_when` and `unknown_when`.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Silence {
        Unknown,
        Zero,
        Conditional,
    }
}

names! {
    /// The concepts the vocabulary publishes, in the order it publishes them.
    ///
    /// The set is what M5's metrics consume and nothing else, so a concept is
    /// added by publishing a version rather than by declaring a member here.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Concept {
        Revenue,
        GrossProfit,
        OperatingIncome,
        PretaxIncome,
        IncomeTaxExpense,
        NetIncome,
        InterestExpense,
        DepreciationAndAmortization,
        PreferredDividends,
        EarningsPerShareDiluted,
        DilutedSharesWeightedAverage,
        SharesOutstanding,
        DividendsDeclaredPerShare,
        TotalAssets,
        CurrentAssets,
        Inventory,
        CashAndEquivalents,
        ShortTermInvestments,
        TotalLiabilities,
        CurrentLiabilities,
        ShortTermDebt,
        LongTermDebt,
        ShareholdersEquity,
        PreferredEquity,
        RetainedEarnings,
        OperatingCashFlow,
        CapitalExpenditure,
        DividendsPaid,
    }
}

/// The kinds a concept applies to.
///
/// Obtainable only from a concept, because it is half of what `NotApplicable`
/// is built from and a clause assembled anywhere else would be one the
/// vocabulary never published.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Applicability(&'static [Kind]);

impl Applicability {
    pub fn kinds(self) -> &'static [Kind] {
        self.0
    }

    pub fn admits(self, kind: Kind) -> bool {
        self.0.contains(&kind)
    }

    /// The witness that this clause excludes `kind`, and nothing where it
    /// admits it. The only way to obtain one.
    ///
    /// It takes a kind rather than an optional one on purpose: where the kind
    /// has not been established there is nothing to ask, so the call cannot be
    /// made and the absence cannot be built.
    pub fn excluding(self, kind: Kind) -> Option<Excluded> {
        (!self.admits(kind)).then_some(Excluded { kind, clause: self })
    }
}

/// A filer's kind together with the applicability clause that excludes it: the
/// whole of what `NotApplicable` is built from, and reachable only through
/// [`Applicability::excluding`].
///
/// Two fields rather than one value because the state carries both, and one
/// value rather than two fields on the state because a kind and a clause put
/// side by side are a pair anyone can make, including a pair where the clause
/// admits the kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Excluded {
    kind: Kind,
    clause: Applicability,
}

impl Excluded {
    pub fn kind(self) -> Kind {
        self.kind
    }

    pub fn clause(self) -> Applicability {
        self.clause
    }
}

/// One candidate a resolution attempt considered, and the rule that declined
/// it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Declined {
    pub candidate: Box<str>,
    pub rule: Box<str>,
}

/// What a resolution attempt that ran has to show for itself, which is the
/// whole of what `Unknown` is built from.
///
/// This crate holds no logic and cannot watch an attempt run. What it holds is
/// that this witness and [`Excluded`] are made from disjoint inputs — candidates
/// and rules on one side, a kind and a clause on the other — so a lookup that
/// failed has nothing to build a correct absence out of, and a kind that
/// excludes a concept has nothing to build a failure out of.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attempt {
    declined: Vec<Declined>,
}

impl Attempt {
    pub fn that_ran(declined: Vec<Declined>) -> Self {
        Attempt { declined }
    }

    pub fn declined(&self) -> &[Declined] {
        &self.declined
    }
}

shapes! {
    /// What a concept resolves to: one of the three states, never both and
    /// never neither.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Resolution {
        /// The concept resolved to an amount, with the provenance M4 requires
        /// of every resolved value.
        ///
        /// The amount crosses as the characters it was published as, for the
        /// reason it crossed into normalize that way: a binary float is a lossy
        /// reading of a published decimal, and this crate depends on nothing
        /// that could hold one losslessly.
        Value {
            amount: Box<str>,
            source_tag: Box<str>,
            filing: Box<str>,
            rule: Box<str>,
        },
        /// The concept does not exist for this filer's accounting shape.
        NotApplicable { excluded: Excluded },
        /// A resolution attempt ran and returned nothing.
        Unknown { attempted: Attempt },
    }
}

/// What the vocabulary states about a concept besides its name, in the order a
/// published entry states its keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Definition {
    pub measure: Measure,
    pub unit: Unit,
    pub sign: Sign,
    pub applies_to: Applicability,
    pub silence: Silence,
}

impl Concept {
    /// This concept as the vocabulary reads it.
    pub fn definition(self) -> Definition {
        use Kind::{Operating, Reit};
        use Measure::{Balance, Flow};
        use Sign::{AsPresented, Magnitude};
        use Unit::{Currency, CurrencyPerShare, Shares};

        /// One published entry, its arguments in the order the entry states its
        /// keys, so an arm below reads against the lines it transcribes.
        const fn stated(
            measure: Measure,
            unit: Unit,
            sign: Sign,
            applies_to: &'static [Kind],
            silence: Silence,
        ) -> Definition {
            Definition {
                measure,
                unit,
                sign,
                applies_to: Applicability(applies_to),
                silence,
            }
        }

        match self {
            Concept::Revenue => stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown),
            Concept::GrossProfit => {
                stated(Flow, Currency, AsPresented, &[Operating], Silence::Unknown)
            }
            Concept::OperatingIncome => stated(
                Flow,
                Currency,
                AsPresented,
                &[Operating, Reit],
                Silence::Unknown,
            ),
            Concept::PretaxIncome => {
                stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::IncomeTaxExpense => {
                stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::NetIncome => stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown),
            Concept::InterestExpense => {
                stated(Flow, Currency, Magnitude, Kind::ALL, Silence::Unknown)
            }
            Concept::DepreciationAndAmortization => {
                stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::PreferredDividends => {
                stated(Flow, Currency, Magnitude, Kind::ALL, Silence::Unknown)
            }
            Concept::EarningsPerShareDiluted => stated(
                Flow,
                CurrencyPerShare,
                AsPresented,
                Kind::ALL,
                Silence::Unknown,
            ),
            Concept::DilutedSharesWeightedAverage => {
                stated(Flow, Shares, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::SharesOutstanding => {
                stated(Balance, Shares, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::DividendsDeclaredPerShare => stated(
                Flow,
                CurrencyPerShare,
                AsPresented,
                Kind::ALL,
                Silence::Conditional,
            ),
            Concept::TotalAssets => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::CurrentAssets => stated(
                Balance,
                Currency,
                AsPresented,
                &[Operating],
                Silence::Unknown,
            ),
            Concept::Inventory => stated(
                Balance,
                Currency,
                AsPresented,
                &[Operating],
                Silence::Unknown,
            ),
            Concept::CashAndEquivalents => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::ShortTermInvestments => {
                stated(Balance, Currency, AsPresented, &[Operating], Silence::Zero)
            }
            Concept::TotalLiabilities => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::CurrentLiabilities => stated(
                Balance,
                Currency,
                AsPresented,
                &[Operating],
                Silence::Unknown,
            ),
            Concept::ShortTermDebt => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::LongTermDebt => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::ShareholdersEquity => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::PreferredEquity => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::RetainedEarnings => {
                stated(Balance, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::OperatingCashFlow => {
                stated(Flow, Currency, AsPresented, Kind::ALL, Silence::Unknown)
            }
            Concept::CapitalExpenditure => {
                stated(Flow, Currency, Magnitude, Kind::ALL, Silence::Unknown)
            }
            Concept::DividendsPaid => {
                stated(Flow, Currency, Magnitude, Kind::ALL, Silence::Conditional)
            }
        }
    }
}

/// The types above against the bytes they transcribe.
///
/// The contracts gate digests `v1.toml` and never reads it, so a type that
/// drifted from the surface it states would stay green on every gate this
/// repository has. This is the comparison that closes that: a name, a member or
/// a reading changed on either side leaves the two readings unequal, and the
/// failure prints both.
#[cfg(test)]
mod states_what_is_published {
    use super::{Concept, Kind, Measure, Resolution, Sign, Silence, Unit};
    use crate::published::Contract;

    /// The file this module states, relative to the repository root, named here
    /// and nowhere else in this module.
    const PATH: &str = "contracts/canonical-concepts/v1.toml";

    fn published() -> Contract {
        Contract::at(PATH)
    }

    /// A declared name as the published bytes spell it: `InvestmentCompany` is
    /// `investment_company`, and that is the whole of the translation between
    /// the two spellings. A name the rule does not cover leaves the readings
    /// unequal, which is the direction to be wrong in: it reports a difference
    /// that is only a spelling, and never passes over one that is not.
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

    fn owned(names: Vec<&str>) -> Vec<String> {
        names.into_iter().map(str::to_owned).collect()
    }

    /// A state's published name is its shape's name unchanged, so nothing is
    /// translated here. What the published bytes do not state is how many
    /// fields a state carries, so that half of `SHAPES` has nothing to be
    /// compared against and the comparison is over the names.
    #[test]
    fn the_states_are_the_ones_the_published_bytes_name() {
        let published = published();
        let declared: Vec<&str> = Resolution::SHAPES.iter().map(|(shape, _)| *shape).collect();

        assert_eq!(
            declared,
            published.names_under("state"),
            "the shapes `Resolution` declares are not the [[state]] names {PATH} states"
        );
    }

    #[test]
    fn the_kinds_are_the_ones_the_published_bytes_name() {
        let published = published();
        let declared: Vec<String> = Kind::ALL
            .iter()
            .map(|kind| as_published(kind.declared()))
            .collect();

        assert_eq!(
            declared,
            owned(published.names_under("kind")),
            "the members `Kind` declares are not the [[kind]] names {PATH} states"
        );
    }

    #[test]
    fn the_concepts_are_the_ones_the_published_bytes_name() {
        let published = published();
        let declared: Vec<String> = Concept::ALL
            .iter()
            .map(|concept| as_published(concept.declared()))
            .collect();

        assert_eq!(
            declared,
            owned(published.names_under("concept")),
            "the members `Concept` declares are not the [[concept]] names {PATH} states"
        );
    }

    /// The four axes a concept is read on. Each is published as a set of
    /// definitions rather than a set of entries, so the members are the table
    /// names under the axis.
    #[test]
    fn the_readings_are_the_ones_the_published_bytes_define() {
        let published = published();
        let axes = [
            (
                "measure",
                Measure::ALL
                    .iter()
                    .map(|measure| as_published(measure.declared()))
                    .collect::<Vec<String>>(),
            ),
            (
                "unit",
                Unit::ALL
                    .iter()
                    .map(|unit| as_published(unit.declared()))
                    .collect(),
            ),
            (
                "sign",
                Sign::ALL
                    .iter()
                    .map(|sign| as_published(sign.declared()))
                    .collect(),
            ),
            (
                "silence",
                Silence::ALL
                    .iter()
                    .map(|silence| as_published(silence.declared()))
                    .collect(),
            ),
        ];

        for (axis, declared) in axes {
            assert_eq!(
                declared,
                owned(published.tables_under(axis)),
                "the members declared for `{axis}` are not the [{axis}.…] tables {PATH} defines"
            );
        }
    }

    #[test]
    fn each_concept_is_read_as_the_published_bytes_read_it() {
        let published = published();
        let stated = published.occurrences("concept");
        assert_eq!(
            Concept::ALL.len(),
            stated.len(),
            "`Concept` declares a different number of members than {PATH} states entries"
        );

        for (concept, entry) in Concept::ALL.iter().zip(&stated) {
            let name = published.unquoted(published.value_of(entry, "name", "[[concept]]"));
            let read = concept.definition();

            for (axis, declared) in [
                ("measure", as_published(read.measure.declared())),
                ("unit", as_published(read.unit.declared())),
                ("sign", as_published(read.sign.declared())),
                ("silence", as_published(read.silence.declared())),
            ] {
                assert_eq!(
                    declared,
                    published.unquoted(published.value_of(entry, axis, "[[concept]]")),
                    "`{name}` is declared with a {axis} {PATH} does not state for it"
                );
            }

            let admitted: Vec<String> = read
                .applies_to
                .kinds()
                .iter()
                .map(|kind| as_published(kind.declared()))
                .collect();
            let stated_kinds: Vec<String> = published
                .members(published.value_of(entry, "applies_to", "[[concept]]"))
                .into_iter()
                .map(|kind| published.unquoted(kind).to_owned())
                .collect();

            assert_eq!(
                admitted, stated_kinds,
                "`{name}` is declared applicable to kinds {PATH} does not state for it"
            );
        }
    }

    /// The enums close what the file closes. Nothing here compares a shape: it
    /// checks that the premise the shape rests on is still the published one,
    /// because an enum over a set the file had reopened would be wrong in a way
    /// no name comparison could show.
    #[test]
    fn the_sets_the_types_close_are_the_sets_the_published_bytes_close() {
        let published = published();

        for set in ["state_set", "kind_set"] {
            let stated = published.keys_of(set);
            assert_eq!(
                published.value_of(&stated, "closed", set),
                "true",
                "{PATH} no longer states [{set}] closed, and the enum closing it rests on that"
            );
        }
    }
}

/// The separation the milestone asks for, as what each absence can be built
/// from.
#[cfg(test)]
mod the_absences_are_built_from_disjoint_witnesses {
    use super::{Concept, Kind};

    /// The one route to `NotApplicable`, walked over every concept and every
    /// kind: a witness for exactly the kinds a clause omits, and none for the
    /// kinds it admits. A lookup that failed holds neither of these, so it has
    /// no route at all.
    #[test]
    fn a_clause_hands_out_a_witness_for_the_kinds_it_omits_and_no_others() {
        for concept in Concept::ALL {
            let clause = concept.definition().applies_to;
            for kind in Kind::ALL {
                match clause.excluding(*kind) {
                    Some(excluded) => {
                        assert!(
                            !clause.admits(*kind),
                            "{concept:?} handed out a witness against {kind:?}, which it applies to"
                        );
                        assert_eq!(excluded.kind(), *kind);
                        assert_eq!(excluded.clause(), clause);
                    }
                    None => assert!(
                        clause.admits(*kind),
                        "{concept:?} handed out no witness against {kind:?}, which it omits"
                    ),
                }
            }
        }
    }
}
