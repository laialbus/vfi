//! How one verdict is spelled on a line of the journal, and how a line is read
//! back into the verdict it holds.
//!
//! This is the layout the ADR leaves open, and it is the implementation's own
//! business: nothing outside this module writes a byte of it or reads one. The
//! types below are that layout, kept apart from [`Record`] deliberately. A
//! `Record` that could serialize itself would publish this arrangement to
//! everything that can name one, and the arrangement is the part still cheap to
//! change.
//!
//! A line is the length of a record, a space, and that many bytes of JSON:
//!
//! ```text
//! 214 {"filer":{"key":{"cik":320193},…},"step":"metadata",…}
//! ```
//!
//! The length is what makes a record cut short recognisable, and recognisable
//! anywhere in the file rather than only at the end of it. A process killed part
//! way through a write leaves a prefix of one line, and the next run appends
//! after it, so the wreck of the interrupted record does not stay where a reader
//! could find it by position. What it does stay is short: fewer bytes than the
//! line declared, which no whole record ever is.
//!
//! JSON, because this crate already reads EDGAR's documents with it, and one
//! record's worth is a line a person can read in a terminal — which is most of
//! what "explain this rejection in December" costs when the tool that would
//! explain it is not to hand.

use std::borrow::Cow;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::company::{Cik, Ticker};
use crate::ledger::{Filer, FilerKey, Judged, Pass, Reason, Record, Ruleset, Step, Verdict, When};
use crate::source::Source;

/// What separates a record's length from the record.
const AFTER_LENGTH: u8 = b' ';

/// What ends a line. No record carries one inside it — JSON escapes every
/// control character — so this appears once per line and nowhere else.
pub(super) const END_OF_LINE: u8 = b'\n';

/// `verdict`, framed as the line it goes on record as, terminator included.
pub(super) fn line(verdict: &Record) -> Result<Vec<u8>, String> {
    let written = Written::of(verdict)?;
    let record = serde_json::to_vec(&written).map_err(|why| why.to_string())?;

    let mut line = Vec::with_capacity(record.len() + 16);
    line.extend_from_slice(record.len().to_string().as_bytes());
    line.push(AFTER_LENGTH);
    line.extend_from_slice(&record);
    line.push(END_OF_LINE);

    Ok(line)
}

/// The verdict `line` holds, with its terminator already off.
///
/// Three answers, and the middle one is what the length is for. `Ok(None)` is a
/// record that never landed whole: fewer bytes than it declared, which is the
/// shape a process killed mid-write leaves behind and the one thing here that
/// costs nothing to pass over. An error is a line that landed whole and says
/// something this cannot read — nothing but the journal writes to a journal, so
/// there is no innocent reading of that, and it is answered rather than skipped.
pub(super) fn read(line: &[u8]) -> Result<Option<Record>, String> {
    let Some(space) = line.iter().position(|byte| *byte == AFTER_LENGTH) else {
        return Ok(None);
    };

    let (length, record) = line.split_at(space);
    let record = &record[1..];

    let declared = std::str::from_utf8(length)
        .ok()
        .and_then(|length| length.parse::<usize>().ok())
        .ok_or_else(|| {
            format!(
                "a line begins {:?}, and a record begins with how long it is",
                String::from_utf8_lossy(&line[..line.len().min(24)])
            )
        })?;

    if record.len() != declared {
        return Ok(None);
    }

    let written: Written<'_> =
        serde_json::from_slice(record).map_err(|why| format!("a record reads as {why}"))?;

    written.into_record().map(Some)
}

/// One verdict, in the shape it is written down in.
///
/// Every string borrows what it is written out of and owns what it is read back
/// into, which is what [`Cow`] is doing on every field: writing a pass over
/// thousands of filers copies no name, and a record read back owns itself.
#[derive(Deserialize, Serialize)]
struct Written<'a> {
    filer: WrittenFiler<'a>,
    step: WrittenStep,
    verdict: WrittenVerdict<'a>,
    source: Cow<'a, str>,
    when: WrittenWhen,
}

#[derive(Deserialize, Serialize)]
struct WrittenFiler<'a> {
    key: WrittenKey<'a>,
    ticker: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WrittenKey<'a> {
    Cik(u64),
    Unresolved(Cow<'a, str>),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WrittenStep {
    Seed,
    Metadata,
    History,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WrittenVerdict<'a> {
    Admitted(WrittenReason<'a>),
    Rejected(WrittenReason<'a>),
    Unjudged(WrittenReason<'a>),
}

#[derive(Deserialize, Serialize)]
struct WrittenReason<'a> {
    rule: Cow<'a, str>,
    judged_on: Vec<WrittenJudged<'a>>,
}

#[derive(Deserialize, Serialize)]
struct WrittenJudged<'a> {
    what: Cow<'a, str>,
    value: Cow<'a, str>,
}

/// The moment as a count from the epoch — whole seconds, then the nanoseconds
/// after that second — beside the pass and the ruleset it was judged under.
///
/// The seconds run negative for a moment before 1970 and the nanoseconds never
/// do, so the two always add. A machine whose clock disagrees with the calendar
/// is not this crate's to correct: what the record says is what the clock said
/// when the judgement was made.
#[derive(Deserialize, Serialize)]
struct WrittenWhen {
    seconds: i64,
    nanos: u32,
    pass: u64,
    ruleset: u32,
}

impl<'a> Written<'a> {
    fn of(verdict: &'a Record) -> Result<Self, String> {
        let filer = verdict.filer();

        Ok(Self {
            filer: WrittenFiler {
                key: match filer.key() {
                    FilerKey::Cik(cik) => WrittenKey::Cik(cik.as_number()),
                    FilerKey::Unresolved(seen_as) => WrittenKey::Unresolved(Cow::Borrowed(seen_as)),
                },
                ticker: filer.ticker().map(|ticker| Cow::Borrowed(ticker.as_str())),
                name: filer.name().map(Cow::Borrowed),
            },
            step: match verdict.step() {
                Step::Seed => WrittenStep::Seed,
                Step::Metadata => WrittenStep::Metadata,
                Step::History => WrittenStep::History,
            },
            verdict: match verdict.verdict() {
                Verdict::Admitted(reason) => WrittenVerdict::Admitted(WrittenReason::of(reason)),
                Verdict::Rejected(reason) => WrittenVerdict::Rejected(WrittenReason::of(reason)),
                Verdict::Unjudged(reason) => WrittenVerdict::Unjudged(WrittenReason::of(reason)),
            },
            source: Cow::Borrowed(verdict.source().url()),
            when: WrittenWhen::of(verdict.when())?,
        })
    }

    fn into_record(self) -> Result<Record, String> {
        let mut filer = match self.filer.key {
            WrittenKey::Cik(cik) => Filer::keyed(Cik::new(cik)),
            WrittenKey::Unresolved(seen_as) => Filer::unresolved(&seen_as),
        };
        if let Some(ticker) = self.filer.ticker {
            filer = filer.seen_as(&Ticker::new(&ticker));
        }
        if let Some(name) = self.filer.name {
            filer = filer.named(&name);
        }

        let step = match self.step {
            WrittenStep::Seed => Step::Seed,
            WrittenStep::Metadata => Step::Metadata,
            WrittenStep::History => Step::History,
        };
        let verdict = match self.verdict {
            WrittenVerdict::Admitted(reason) => Verdict::Admitted(reason.into_reason()),
            WrittenVerdict::Rejected(reason) => Verdict::Rejected(reason.into_reason()),
            WrittenVerdict::Unjudged(reason) => Verdict::Unjudged(reason.into_reason()),
        };

        Ok(Record::new(
            filer,
            step,
            verdict,
            Source::new(&self.source),
            self.when.into_when()?,
        ))
    }
}

impl<'a> WrittenReason<'a> {
    fn of(reason: &'a Reason) -> Self {
        Self {
            rule: Cow::Borrowed(reason.name()),
            judged_on: reason
                .judged_on()
                .iter()
                .map(|judged| WrittenJudged {
                    what: Cow::Borrowed(judged.what()),
                    value: Cow::Borrowed(judged.value()),
                })
                .collect(),
        }
    }

    fn into_reason(self) -> Reason {
        Reason::read_back(
            self.rule.into_owned(),
            self.judged_on
                .into_iter()
                .map(|judged| {
                    Judged::read_back(judged.what.into_owned(), judged.value.into_owned())
                })
                .collect(),
        )
    }
}

impl WrittenWhen {
    fn of(when: When) -> Result<Self, String> {
        let (seconds, nanos) = since_epoch(when.moment())?;

        Ok(Self {
            seconds,
            nanos,
            pass: when.pass().as_number(),
            ruleset: when.ruleset().as_number(),
        })
    }

    fn into_when(self) -> Result<When, String> {
        Ok(When::new(
            moment(self.seconds, self.nanos)?,
            Pass::new(self.pass),
            Ruleset::version(self.ruleset),
        ))
    }
}

/// `moment` as whole seconds from the epoch and the nanoseconds after that
/// second, the seconds running negative before 1970 and the nanoseconds never.
fn since_epoch(moment: SystemTime) -> Result<(i64, u32), String> {
    let uncountable =
        |elapsed| format!("{elapsed:?} from the epoch is further than a record counts");

    match moment.duration_since(UNIX_EPOCH) {
        Ok(after) => Ok((
            i64::try_from(after.as_secs()).map_err(|_| uncountable(after))?,
            after.subsec_nanos(),
        )),
        Err(before) => {
            let before = before.duration();
            let seconds = i64::try_from(before.as_secs()).map_err(|_| uncountable(before))?;

            match before.subsec_nanos() {
                0 => Ok((-seconds, 0)),
                nanos => Ok((-seconds - 1, 1_000_000_000 - nanos)),
            }
        }
    }
}

/// The moment `seconds` and `nanos` from the epoch count to, or why they count
/// to none.
fn moment(seconds: i64, nanos: u32) -> Result<SystemTime, String> {
    let unreachable =
        || format!("{seconds}s and {nanos}ns from the epoch is not a moment this machine holds");

    if nanos >= 1_000_000_000 {
        return Err(unreachable());
    }

    let whole = Duration::from_secs(seconds.unsigned_abs());
    let counted = if seconds < 0 {
        UNIX_EPOCH.checked_sub(whole)
    } else {
        UNIX_EPOCH.checked_add(whole)
    };

    counted
        .and_then(|counted| counted.checked_add(Duration::from_nanos(nanos.into())))
        .ok_or_else(unreachable)
}

#[cfg(test)]
mod tests {
    use super::{END_OF_LINE, line, moment, read, since_epoch};
    use crate::company::{Cik, Ticker};
    use crate::ledger::{Filer, Pass, Reason, Record, Ruleset, Step, Verdict, When};
    use crate::source::Source;
    use std::time::{Duration, UNIX_EPOCH};

    fn unjudged() -> Record {
        Record::new(
            Filer::unresolved("ZZZZ"),
            Step::Seed,
            Verdict::Unjudged(Reason::new("the source did not answer", "status", 503)),
            Source::new("https://www.sec.gov/files/company_tickers.json"),
            When::new(
                UNIX_EPOCH + Duration::new(1_754_000_000, 250),
                Pass::new(7),
                Ruleset::version(1),
            ),
        )
    }

    /// The line as it goes on record, with the terminator off — which is how a
    /// reader is handed one.
    fn written(verdict: &Record) -> Vec<u8> {
        let mut written = line(verdict).expect("a record this crate made writes down");
        written.pop();
        written
    }

    /// A seed entry nothing resolved, which is the record with the least in it:
    /// no identifier, no ticker, no name, and a verdict that is not a judgement.
    #[test]
    fn a_line_holds_the_verdict_it_was_written_from() {
        let verdict = unjudged();

        assert_eq!(read(&written(&verdict)), Ok(Some(verdict)));
    }

    /// The record the funnel will write most of, out and back with every field
    /// it carries — the names it was seen under included, which a record keeps
    /// as observed rather than looks up again.
    #[test]
    fn a_rejection_reads_back_with_the_evidence_it_was_made_on() {
        let verdict = Record::new(
            Filer::keyed(Cik::new(2003750))
                .seen_as(&Ticker::new("MGSD"))
                .named("Maitong Sunshine Cultural Development Co., Ltd"),
            Step::Metadata,
            Verdict::Rejected(
                Reason::new("filed as a foreign private issuer", "form", "20-F").and("filings", 12),
            ),
            Source::new("https://data.sec.gov/submissions/CIK0002003750.json"),
            When::new(UNIX_EPOCH, Pass::new(1), Ruleset::version(3)),
        );

        assert_eq!(read(&written(&verdict)), Ok(Some(verdict)));
    }

    /// Every prefix of a line, which is every place a process can be killed
    /// inside one. None of them is a record, and none of them is corruption
    /// either — a record that never landed whole is the one thing a reader
    /// passes over.
    #[test]
    fn a_line_cut_short_anywhere_is_a_record_that_never_landed() {
        let whole = written(&unjudged());

        for cut in 0..whole.len() {
            assert_eq!(
                read(&whole[..cut]),
                Ok(None),
                "{:?} read as something other than a record cut short",
                String::from_utf8_lossy(&whole[..cut])
            );
        }
    }

    /// The other half of that: a line that landed whole and says something this
    /// cannot read is answered, not passed over.
    #[test]
    fn a_line_that_landed_whole_and_reads_as_nothing_is_answered() {
        for corrupt in [
            &br#"7 {"a":1}"#[..],
            &br#"not-a-length {"a":1}"#[..],
            &b"0 "[..],
        ] {
            assert!(
                read(corrupt).is_err(),
                "{:?} read as something other than corruption",
                String::from_utf8_lossy(corrupt)
            );
        }
    }

    /// What the reader splits lines on, and what the writer must therefore never
    /// put inside one. A record carrying a raw line ending would read back as
    /// two records, neither of them whole.
    #[test]
    fn a_record_carries_no_line_ending_inside_it() {
        let verdict = Record::new(
            Filer::unresolved("two\nlines").named("a name\nwith a line ending in it"),
            Step::Seed,
            Verdict::Rejected(Reason::new(
                "the row is not a ticker",
                "seen as",
                "two\nlines",
            )),
            Source::new("https://www.sec.gov/files/company_tickers.json"),
            When::new(UNIX_EPOCH, Pass::new(1), Ruleset::version(1)),
        );
        let written = written(&verdict);

        assert!(!written.contains(&END_OF_LINE));
        assert_eq!(read(&written), Ok(Some(verdict)));
    }

    /// A moment out and back, on both sides of the epoch and either side of a
    /// whole second. A record that quietly moved a judgement by a second would
    /// be the ledger disagreeing with itself about when it was made.
    #[test]
    fn a_moment_counts_from_the_epoch_and_back_to_itself() {
        for judged_at in [
            UNIX_EPOCH,
            UNIX_EPOCH + Duration::new(1_754_000_000, 1),
            UNIX_EPOCH + Duration::new(1_754_000_000, 999_999_999),
            UNIX_EPOCH - Duration::new(1, 0),
            UNIX_EPOCH - Duration::new(0, 1),
            UNIX_EPOCH - Duration::new(86_400, 500_000_000),
        ] {
            let (seconds, nanos) = since_epoch(judged_at).expect("a moment a machine holds counts");

            assert!(nanos < 1_000_000_000);
            assert_eq!(moment(seconds, nanos), Ok(judged_at));
        }
    }

    /// A count no moment answers to is what a corrupt record looks like from
    /// here, and it is refused rather than folded into the nearest moment.
    #[test]
    fn a_count_that_is_no_moment_is_refused() {
        assert!(moment(0, 1_000_000_000).is_err());
    }
}
