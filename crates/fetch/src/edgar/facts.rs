//! Retrieving the facts a filer reported.
//!
//! One document, one request per filer: EDGAR's collation of the XBRL exhibits
//! of that filer's own periodic filings. So a fact arrives stamped with the
//! filing it was reported in without that filing being retrieved, and a filer
//! with decades of history costs the one request a filer with a single year of
//! it costs. Why that document rather than the filings themselves is
//! `docs/adr/fetch-normalize-contract.md`, and the argument is not repeated
//! here.
//!
//! The one thing a fact carries that the document does not publish is the date
//! its filing's period of report ends, which only the filer's submissions
//! document states. So the history is retrieved too and joined to each fact by
//! accession, and a filing it does not name carries that date empty rather than
//! one read off anything else.
//! Why the date crosses at all is `docs/adr/fetch-normalize-v2.md`.
//!
//! What comes back is the boundary's own value, built from the type
//! `vfi-contracts` publishes. A second shape of the same thing here would be
//! the unchecked copy the one-source-of-truth invariant bans.
//!
//! Two things this does not do, both on purpose, and both properties the
//! fixture beside it pins.
//!
//! Nothing is filtered. Every fact the document publishes crosses — every
//! taxonomy, every element, every unit, every form. A tag list in here would be
//! normalize's data reaching back into an earlier stage, which is the edge the
//! crate graph refuses; and it would take away the last reading of silence,
//! because normalize can say a filer reported nothing under an element only if
//! what it holds is everything the filer reported.
//!
//! Nothing is parsed. Amounts and dates cross as the characters the document
//! publishes. That is why an amount is read out of the body as raw JSON rather
//! than as a number: a decimal taken into a binary float and written back out
//! is no longer the literal the filer reported, and a reading made here is one
//! normalize cannot check or undo.

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::Entry as Taken;

use serde_json::value::RawValue;
use vfi_contracts::fetch_normalize::{Fact, Filer, Period};

use super::documents::CompanyFacts;
use super::{Unretrieved, filings, get, read};
use crate::company::{Cik, PADDED_DIGITS};
use crate::egress::{Egress, Transport};
use crate::filing::Filing;
use crate::source::Source;

/// Where a filer's company facts sit. The document is `CIK` and the key padded
/// to the ten digits [`Cik`]'s own `Display` writes, which is how the
/// submissions endpoint names the same filer.
const COMPANY_FACTS: &str = "https://data.sec.gov/api/xbrl/companyfacts/";

fn company_facts_url(cik: Cik) -> String {
    format!("{COMPANY_FACTS}CIK{cik}.json")
}

/// Every fact EDGAR publishes for the filer it keys as `cik`.
///
/// The facts arrive nested three deep — taxonomy, then element, then unit — and
/// leave flat, each one carrying those three keys down along with the filing
/// that reported it. Their order is the order of the keys they were nested
/// under, because the document's objects have no order of their own and a list
/// handed across a boundary has to be the same list twice.
///
/// The filer's history is asked for after its facts, every page of it, so it is
/// no older than they are and names every filing they were reported in unless
/// EDGAR's two documents disagree. A document that is not this filer's facts
/// costs no history at all.
pub fn company_facts<T: Transport>(edgar: &mut Egress<T>, cik: Cik) -> Result<Filer, Unretrieved> {
    let source = Source::new(&company_facts_url(cik));
    let body = get(edgar, &source)?;

    let document: CompanyFacts<'_> = read(&body, &source)?;
    let filed_under = key(document.cik, &source)?;
    about(filed_under, cik, &source)?;

    let (published_by, history) = filings(edgar, cik)?;
    let reported_to = report_dates(&history, &published_by)?;

    let mut facts = Vec::new();
    for (taxonomy, elements) in &document.facts {
        for (tag, element) in elements {
            for (unit, reported) in &element.units {
                facts.reserve(reported.len());
                for entry in reported {
                    facts.push(Fact {
                        taxonomy: Box::from(*taxonomy),
                        tag: Box::from(*tag),
                        unit: Box::from(*unit),
                        period: match &entry.start {
                            Some(start) => Period::Duration {
                                start: Box::from(start.as_ref()),
                                end: Box::from(entry.end.as_ref()),
                            },
                            None => Period::Instant {
                                at: Box::from(entry.end.as_ref()),
                            },
                        },
                        value: amount(entry.val, &source)?,
                        accession: Box::from(entry.accn.as_ref()),
                        form: Box::from(entry.form.as_ref()),
                        filed: Box::from(entry.filed.as_ref()),
                        report_period_end: Box::from(
                            reported_to.get(entry.accn.as_ref()).copied().unwrap_or(""),
                        ),
                    });
                }
            }
        }
    }

    Ok(Filer {
        cik: Box::from(filed_under.to_string()),
        retrieved_from: Box::from(source.url()),
        facts,
    })
}

/// Each filing the history names, by accession, with the report date the
/// history publishes for it — empty where it publishes none.
///
/// One accession named twice with two dates is the document saying two things
/// about one filing, and choosing between them would be this stage deciding
/// which period the filing reports. So nothing is read from it.
fn report_dates<'h>(
    filings: &'h [Filing],
    history: &Source,
) -> Result<HashMap<&'h str, &'h str>, Unretrieved> {
    let mut dates = HashMap::with_capacity(filings.len());

    for filing in filings {
        let date = filing.period().unwrap_or("");
        match dates.entry(filing.accession()) {
            Taken::Vacant(slot) => {
                slot.insert(date);
            }
            Taken::Occupied(taken) if *taken.get() != date => {
                return Err(Unretrieved::Unreadable {
                    source: history.clone(),
                    why: format!(
                        "it names {} twice, reporting to {:?} and to {date:?}",
                        taken.key(),
                        taken.get()
                    ),
                });
            }
            Taken::Occupied(_) => {}
        }
    }

    Ok(dates)
}

/// The filer's key as the document publishes it, in either of its spellings:
/// the ten-digit string or the bare number.
///
/// Both are read as the number they spell, and what crosses is that number
/// written back at ten digits. A key that is neither a string nor a number is
/// not the document this endpoint publishes. Nor is one spelled with more than
/// ten digits or with anything but digits: dropping digits or padding wider
/// would make a key `registry/filers/` cannot bind, or another filer's.
fn key(published: &RawValue, source: &Source) -> Result<Cik, Unretrieved> {
    let literal = published.get();
    let refused = |why: &str| Unretrieved::Unreadable {
        source: source.clone(),
        why: format!("{literal} is published where the filer's key is, and {why}"),
    };

    let digits: Cow<'_, str> = match literal.as_bytes().first() {
        Some(b'"') => {
            Cow::Owned(serde_json::from_str(literal).map_err(|_| refused("is no string"))?)
        }
        Some(byte) if byte.is_ascii_digit() || *byte == b'-' => Cow::Borrowed(literal),
        _ => return Err(refused("is neither a string nor a number")),
    };

    match digits.parse::<u64>() {
        Ok(number)
            if digits.len() <= PADDED_DIGITS
                && digits.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            Ok(Cik::new(number))
        }
        _ => Err(refused(&format!(
            "no key of {PADDED_DIGITS} digits spells it"
        ))),
    }
}

/// That the document EDGAR answered with is about the filer that was asked
/// about.
///
/// The same check a submissions document gets and for the same reason: a
/// document filed under a key nobody asked about is a whole company's worth of
/// facts that are wrong and well-formed. It compares two keys and reads
/// neither: the document's was read by [`key`], so the key that crosses is the
/// one the document states and never the one that was asked with.
fn about(filed_under: Cik, cik: Cik, source: &Source) -> Result<(), Unretrieved> {
    if filed_under == cik {
        return Ok(());
    }

    Err(Unretrieved::Unreadable {
        source: source.clone(),
        why: format!("it is filed under {filed_under}, and {cik} is what was asked for"),
    })
}

/// The amount, as the characters the document published it as.
///
/// Nothing is converted. What is checked is that the document published a
/// number at all: the raw text of a JSON string carries its quotes, so a quoted
/// amount would cross as `"12"` — a value normalize could not read and could
/// not tell from one that arrived intact. A document publishing something else
/// where an amount belongs is not the document this endpoint publishes, and
/// nothing is taken from it.
fn amount(published: &RawValue, source: &Source) -> Result<Box<str>, Unretrieved> {
    let literal = published.get();

    if !literal
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'-')
    {
        return Err(Unretrieved::Unreadable {
            source: source.clone(),
            why: format!("{literal} is published where an amount is, and is no number"),
        });
    }

    Ok(Box::from(literal))
}

#[cfg(test)]
mod tests {
    use super::{COMPANY_FACTS, company_facts_url};
    use crate::company::Cik;

    /// Padded to ten digits, which is the only spelling the endpoint answers
    /// to.
    #[test]
    fn a_filers_facts_are_named_by_its_padded_key() {
        assert_eq!(
            company_facts_url(Cik::new(320193)),
            format!("{COMPANY_FACTS}CIK0000320193.json")
        );
        assert_eq!(
            company_facts_url(Cik::new(2003750)),
            format!("{COMPANY_FACTS}CIK0002003750.json")
        );
    }
}
