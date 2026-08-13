//! One verdict, and everything that has to be true of it to be one.
//!
//! The fields are `docs/adr/filer-decision-ledger.md`'s, and they are the part
//! of that decision it calls expensive to reverse: a journal on a real machine
//! turns a changed field into a migration. So none of them is optional, and
//! nothing is here that the ADR does not name.
//!
//! What that costs is paid at the call site, which has to say all of it. What
//! it buys is that the record this exists to prevent — a filer marked rejected,
//! by nothing, for no stated reason, at no known time — is not a record anybody
//! can write.

use std::fmt;
use std::time::SystemTime;

use crate::company::{Cik, Ticker};
use crate::source::Source;

/// Which step of the funnel judged a filer.
///
/// The funnel has three and a verdict belongs to one of them, because rejected
/// means something different at each: a seed entry nothing resolved, a filer
/// its metadata rules out, and a filer whose history says it is not one this
/// tool reads are three outcomes that would be one word without this.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Step {
    /// The seed set: what the source publishes as the filers there are.
    Seed,
    /// The metadata gate: judged on what is published about a filer, before its
    /// history is asked for.
    Metadata,
    /// The history step: judged on what the filer has actually filed.
    History,
}

/// What a verdict is filed under.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilerKey {
    /// The identifier EDGAR keys on, which is what a filer is to every endpoint
    /// that answers about one.
    Cik(Cik),
    /// A seed entry that resolved to no identifier, under whatever there was —
    /// the symbol it was listed as, the name beside it, the row as it read.
    /// Keyed by that rather than left out: an entry nothing resolved is a filer
    /// that was evaluated, and this record is the only trace it ever was.
    Unresolved(Box<str>),
}

/// Which filer a verdict is about: what it is filed under, and the names it was
/// seen under when it was judged.
///
/// Both names are as observed, and neither is looked back up when the record is
/// read. A ticker changes hands and a filer is renamed; the record is of a
/// judgement made at a moment, and a name corrected into it afterwards would be
/// a different judgement wearing the same date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Filer {
    key: FilerKey,
    ticker: Option<Ticker>,
    name: Option<Box<str>>,
}

impl Filer {
    /// The filer EDGAR keys as `cik`.
    pub fn keyed(cik: Cik) -> Self {
        Self::under(FilerKey::Cik(cik))
    }

    /// A seed entry that resolved to no identifier, keyed by `seen_as`, which is
    /// whatever the entry gave.
    pub fn unresolved(seen_as: &str) -> Self {
        Self::under(FilerKey::Unresolved(Box::from(seen_as)))
    }

    /// Seen under `ticker`.
    pub fn seen_as(mut self, ticker: &Ticker) -> Self {
        self.ticker = Some(ticker.clone());
        self
    }

    /// Named `name` where it was seen.
    pub fn named(mut self, name: &str) -> Self {
        self.name = Some(Box::from(name));
        self
    }

    /// What this filer's verdicts are filed under.
    pub fn key(&self) -> &FilerKey {
        &self.key
    }

    /// The ticker it was seen under, where it was seen under one.
    pub fn ticker(&self) -> Option<&Ticker> {
        self.ticker.as_ref()
    }

    /// The name it was seen under, where it was seen under one.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn under(key: FilerKey) -> Self {
        Self {
            key,
            ticker: None,
            name: None,
        }
    }
}

/// One value a reason was judged on: what was looked at, and what it said.
///
/// The value is kept as it read rather than as what it meant. A rejection is
/// checked in December by whoever asks what this filer's form actually was, and
/// a value already reduced to the judgement made on it answers that with the
/// judgement again.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Judged {
    what: &'static str,
    value: Box<str>,
}

impl Judged {
    /// What was looked at.
    pub fn what(&self) -> &'static str {
        self.what
    }

    /// What it said.
    pub fn value(&self) -> &str {
        &self.value
    }

    fn new(what: &'static str, value: impl fmt::Display) -> Self {
        Self {
            what,
            value: value.to_string().into_boxed_str(),
        }
    }
}

/// Why a step judged as it did: the rule it applied, and the values it applied
/// it to.
///
/// The rule is a `&'static str`, and that is what keeps this a named reason
/// rather than prose. A name that cannot be composed out of the filer at hand
/// is one defined once, beside the step that issues it, and every record citing
/// it cites the same rule — so "why was this filer dropped" and "which filers
/// did this rule drop" are the same question asked twice. What varies per filer
/// is the evidence beside it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reason {
    name: &'static str,
    judged_on: Vec<Judged>,
}

impl Reason {
    /// The rule `name`, applied to a `value` of `what`.
    ///
    /// One value at least, because a reason with no evidence cannot be checked
    /// afterwards: "rejected for filing 20-F" is answerable only beside the form
    /// that was read.
    pub fn new(name: &'static str, what: &'static str, value: impl fmt::Display) -> Self {
        Self {
            name,
            judged_on: vec![Judged::new(what, value)],
        }
    }

    /// Another value it was judged on.
    pub fn and(mut self, what: &'static str, value: impl fmt::Display) -> Self {
        self.judged_on.push(Judged::new(what, value));
        self
    }

    /// The rule that was applied.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The values it was applied to, in the order they were given.
    pub fn judged_on(&self) -> &[Judged] {
        &self.judged_on
    }
}

/// What a step decided about a filer.
///
/// Three states, and the third is the one to be careful with. A filer nobody
/// could judge is not a filer judged and dropped: recording a transport failure
/// as a rejection is how a bad minute becomes a permanent exclusion, for a
/// reason that stopped being true the moment the source answered again. The
/// type is what keeps them apart, since no code has to remember to.
///
/// Each carries its reason. A verdict that carried none would be a decision
/// nobody can check, and a rejection is the one that most needs checking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// Judged, and it goes on to the next step.
    Admitted(Reason),
    /// Judged, and it goes no further.
    Rejected(Reason),
    /// Not judged, because the evaluation itself failed. This is not a
    /// rejection and does not become one: nothing was decided about the filer,
    /// and what is on record is that nothing was.
    Unjudged(Reason),
}

impl Verdict {
    /// Why it was judged as it was — or, for [`Verdict::Unjudged`], why it could
    /// not be judged at all.
    pub fn reason(&self) -> &Reason {
        match self {
            Verdict::Admitted(reason) | Verdict::Rejected(reason) | Verdict::Unjudged(reason) => {
                reason
            }
        }
    }
}

/// The sweep of the funnel a verdict belongs to.
///
/// A pass over the seed set is long enough to be interrupted part way, and
/// without this on the record, which verdicts belong to the last complete sweep
/// is reconstructed from timestamps — which is a guess. What a pass is numbered
/// by, and how one is told from the next, is the funnel's to say; what is fixed
/// here is that a verdict carries one.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pass(u64);

impl Pass {
    /// The pass numbered `id`.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// The number, for whoever writes it down.
    pub fn as_number(&self) -> u64 {
        self.0
    }
}

/// The version of the ruleset that judged a filer.
///
/// Rules change, and a verdict recorded under one is not a verdict under the
/// next. Reading a rejection months later means knowing which rules rejected
/// it, so the version rides the record rather than being read off the code as
/// it stands by then.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ruleset(u32);

impl Ruleset {
    /// Version `version` of a step's rules. `const`, so a step names its version
    /// once, as a constant beside the rules themselves.
    pub const fn version(version: u32) -> Self {
        Self(version)
    }

    /// The version, for whoever writes it down.
    pub fn as_number(&self) -> u32 {
        self.0
    }
}

/// When a judgement was made, and under what.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct When {
    moment: SystemTime,
    pass: Pass,
    ruleset: Ruleset,
}

impl When {
    /// At `moment`, during `pass`, under `ruleset`.
    ///
    /// The moment is given rather than read from the machine here, for the
    /// reason [`crate::Pace`]'s clock is a parameter: a record that stamped
    /// itself could only be checked by waiting for the time it stamped.
    pub fn new(moment: SystemTime, pass: Pass, ruleset: Ruleset) -> Self {
        Self {
            moment,
            pass,
            ruleset,
        }
    }

    /// The moment the judgement was made.
    pub fn moment(&self) -> SystemTime {
        self.moment
    }

    /// The sweep it belongs to.
    pub fn pass(&self) -> Pass {
        self.pass
    }

    /// The version of the rules that made it.
    pub fn ruleset(&self) -> Ruleset {
        self.ruleset
    }
}

/// One verdict, as it goes on record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    filer: Filer,
    step: Step,
    verdict: Verdict,
    source: Source,
    when: When,
}

impl Record {
    /// What `step` decided about `filer`, out of what `source` answered with,
    /// at `when`.
    pub fn new(filer: Filer, step: Step, verdict: Verdict, source: Source, when: When) -> Self {
        Self {
            filer,
            step,
            verdict,
            source,
            when,
        }
    }

    /// The filer this was decided about.
    pub fn filer(&self) -> &Filer {
        &self.filer
    }

    /// The step that decided it.
    pub fn step(&self) -> Step {
        self.step
    }

    /// What it decided, and why.
    pub fn verdict(&self) -> &Verdict {
        &self.verdict
    }

    /// The request the values in the reason were read out of, so a verdict
    /// traces back to the bytes it was made from without asking the source
    /// again.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// When it was decided, and under what.
    pub fn when(&self) -> When {
        self.when
    }
}

#[cfg(test)]
mod tests {
    use super::{Filer, FilerKey, Reason, Verdict};
    use crate::company::{Cik, Ticker};

    /// The names are as observed and the key is not composed out of them. A
    /// record read back names the ticker the judgement was made under, whoever
    /// trades under it now.
    #[test]
    fn a_filer_is_keyed_by_its_identifier_and_named_by_what_was_seen() {
        let filer = Filer::keyed(Cik::new(320193))
            .seen_as(&Ticker::new("AAPL"))
            .named("Apple Inc.");

        assert_eq!(filer.key(), &FilerKey::Cik(Cik::new(320193)));
        assert_eq!(filer.ticker(), Some(&Ticker::new("AAPL")));
        assert_eq!(filer.name(), Some("Apple Inc."));
    }

    /// A seed entry nothing resolved is still filed under something. What is
    /// unavailable is the identifier, not the record.
    #[test]
    fn an_entry_that_resolved_to_no_identifier_is_keyed_by_what_there_was() {
        let filer = Filer::unresolved("ZZZZ");

        assert_eq!(filer.key(), &FilerKey::Unresolved(Box::from("ZZZZ")));
        assert_eq!(filer.ticker(), None);
        assert_eq!(filer.name(), None);
    }

    #[test]
    fn a_reason_carries_the_rule_and_every_value_it_was_judged_on() {
        let reason = Reason::new("filed as a foreign private issuer", "form", "20-F")
            .and("filings read", 12);

        assert_eq!(reason.name(), "filed as a foreign private issuer");
        assert_eq!(
            reason
                .judged_on()
                .iter()
                .map(|judged| (judged.what(), judged.value()))
                .collect::<Vec<_>>(),
            [("form", "20-F"), ("filings read", "12")]
        );
    }

    /// The distinction the ledger exists for, at the one place it can be made
    /// mechanically: one reason, two verdicts, and they are not equal. A filer
    /// nobody could judge does not compare, sort, or read back as one that was
    /// judged and dropped.
    #[test]
    fn not_judged_is_not_rejected_even_for_the_same_reason() {
        let reason = Reason::new("the source did not answer", "status", 503);

        assert_ne!(
            Verdict::Unjudged(reason.clone()),
            Verdict::Rejected(reason.clone())
        );
        assert_ne!(Verdict::Admitted(reason.clone()), Verdict::Rejected(reason));
    }
}
