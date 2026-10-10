//! Analyze stage: derives metrics from facts and settings. Pure.
//!
//! The one entry point is [`analyze`]. Everything it reads is handed to it on
//! the call, and nothing outlives the call. Why it takes and returns what it
//! does is `docs/adr/what-analyze-takes-and-returns.md`.

use std::fmt;
use std::num::NonZeroU32;

use vfi_contracts::analyze_store::{HandOver, Premises, Prices, Results};
use vfi_contracts::canonical_concepts::History;
use vfi_contracts::fetch_analyze::Crossing;

mod arithmetic;
mod constants;
mod method;
pub mod settings;

pub use settings::Settings;

const VERSION: NonZeroU32 =
    NonZeroU32::new(method::METHOD_VERSION).expect("the method version is a positive integer");

/// The crossing names a filer other than the history's. That is a broken call,
/// so it stops here rather than crossing as a state of the data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnotherFiler {
    history: Box<str>,
    crossing: Box<str>,
}

impl AnotherFiler {
    /// The filer the history is for, as its ten digits.
    pub fn history(&self) -> &str {
        &self.history
    }

    /// The filer the crossing names, as its ten digits.
    pub fn crossing(&self) -> &str {
        &self.crossing
    }
}

impl fmt::Display for AnotherFiler {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            out,
            "the price crossing names filer {}, and the history is filer {}",
            self.crossing, self.history
        )
    }
}

impl std::error::Error for AnotherFiler {}

/// `history`'s filer's hand-over to store, from that filer's `prices`, or none
/// where no price was asked, under `settings`.
///
/// It records the premises every result was computed under: the method
/// version and every setting by name with the value used.
pub fn analyze(
    history: History,
    prices: Option<Crossing>,
    settings: &Settings,
) -> Result<HandOver, AnotherFiler> {
    if let Some(crossing) = &prices
        && *crossing.cik != *history.filer()
    {
        return Err(AnotherFiler {
            history: history.filer().into(),
            crossing: crossing.cik.clone(),
        });
    }

    let premises =
        Premises::under(VERSION, settings.used()).expect("a field is declared once in a struct");
    let results = results(&history);
    let prices = match prices {
        Some(crossing) => Prices::Asked { crossing },
        None => Prices::NotAsked {},
    };

    Ok(HandOver::of(history, prices, premises, results)
        .expect("the filer is checked above and each period is taken once"))
}

/// The results at each period the history holds, each once. No metric is
/// defined, so each holds none.
///
/// The history states each period once. A period it repeated would still be
/// one period, and is taken at its first row.
fn results(history: &History) -> Vec<Results> {
    let rows = history.periods();
    rows.iter()
        .enumerate()
        .filter(|(at, row)| !rows[..*at].iter().any(|seen| seen.period() == row.period()))
        .map(|(_, row)| row.period().clone())
        .map(|period| Results::at(period, Vec::new()).expect("no metric is named twice"))
        .collect()
}
