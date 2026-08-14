//! The two names a company goes by, and the company itself.
//!
//! A user asks about a ticker. EDGAR keys on a CIK — the number it assigned the
//! filer — and publishes a map between them. Both are names for the same thing
//! and neither substitutes for the other, so both are types: a function that
//! takes a [`Cik`] cannot be handed a ticker by accident.

use std::fmt;

use crate::source::Source;

/// The symbol a company's shares trade under, as EDGAR's map spells it.
///
/// ASCII case is folded up on the way in, because every ticker in that map is
/// upper case and DNS-style folding is the same judgement made in one place
/// rather than at each lookup. Nothing else is changed and nothing is refused:
/// what is or is not a ticker is EDGAR's to say, and a string its map does not
/// name is answered as unknown rather than guessed at.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ticker(Box<str>);

impl Ticker {
    /// Take `symbol` as the ticker to ask about.
    pub fn new(symbol: &str) -> Self {
        Self(symbol.to_ascii_uppercase().into_boxed_str())
    }

    /// The symbol, as it will be looked up.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The Central Index Key: the number EDGAR assigned a filer, and the only name
/// its filing endpoints answer to.
///
/// Held as the number it is. EDGAR writes it several ways — bare in its ticker
/// map, padded to ten digits in a submissions document and in the URL of one —
/// and a type that held the spelling would have to say which spelling, then
/// convert at every use.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cik(u64);

/// The width EDGAR pads a CIK to where it writes one padded.
const PADDED_DIGITS: usize = 10;

impl Cik {
    pub(crate) fn new(key: u64) -> Self {
        Self(key)
    }

    /// The key as a number, for whoever needs it in a form this does not print.
    pub fn as_number(&self) -> u64 {
        self.0
    }
}

/// Padded to the ten digits EDGAR pads to, because that is the form its URLs
/// and its documents carry, and the form a reader recognises.
impl fmt::Display for Cik {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0PADDED_DIGITS$}", self.0)
    }
}

/// A filer EDGAR names, and the request that identifies it.
///
/// The name is one EDGAR published — the title beside the ticker in its map —
/// and never one this crate composed. [`Company::source`] is the request the
/// filer was identified by, which is that map where a ticker was resolved
/// through it, and the filer's own document where a pass of the funnel already
/// holds the key and carries a name it recorded under it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Company {
    cik: Cik,
    name: Box<str>,
    source: Source,
}

impl Company {
    pub(crate) fn new(cik: Cik, name: &str, source: Source) -> Self {
        Self {
            cik,
            name: Box::from(name),
            source,
        }
    }

    /// The key EDGAR's filing endpoints answer to.
    pub fn cik(&self) -> Cik {
        self.cik
    }

    /// The company's name, as EDGAR published it.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The request this filer was identified by.
    pub fn source(&self) -> &Source {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::{Cik, Ticker};

    #[test]
    fn a_ticker_is_asked_about_in_the_case_edgars_map_spells_it() {
        assert_eq!(Ticker::new("aapl").as_str(), "AAPL");
        assert_eq!(Ticker::new("AAPL").as_str(), "AAPL");
        assert_eq!(Ticker::new("BRK-b").as_str(), "BRK-B");
    }

    /// Nothing but case. A ticker with a character EDGAR's map does not use is
    /// still a ticker to ask about — it is simply one the map does not name —
    /// and trimming or dropping a character here would be this crate deciding
    /// which company was meant.
    #[test]
    fn nothing_but_case_is_changed() {
        for symbol in [" AAPL", "AA PL", "AAPL.", "", "aapl\n"] {
            assert_eq!(Ticker::new(symbol).as_str(), symbol.to_ascii_uppercase());
        }
    }

    /// The form EDGAR's own URLs and documents carry.
    #[test]
    fn a_cik_prints_padded_to_ten_digits() {
        assert_eq!(Cik::new(320193).to_string(), "0000320193");
        assert_eq!(Cik::new(1750).to_string(), "0000001750");
        assert_eq!(Cik::new(1234567890).to_string(), "1234567890");
    }
}
