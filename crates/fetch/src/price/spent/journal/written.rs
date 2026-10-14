//! How one entry is spelled on a line of the spent journal, and how a line is
//! read back into the entry it holds.
//!
//! The layout is the implementation's own business: nothing outside this
//! module writes a byte of it or reads one. It is framed as the filer ledger's
//! journal frames a record — the length of the entry, a space, and that many
//! bytes of JSON — and spells its moment the way that journal does:
//!
//! ```text
//! 81 {"left":{"seconds":1754000000,"nanos":0},"ticker":"AAPL","host":"api.tiingo.com"}
//! ```
//!
//! The length is what makes an entry cut short recognisable: a process killed
//! part way through a write leaves fewer bytes than the line declared, which no
//! whole entry ever is.
//!
//! An entry carries three fields and a line holding a fourth does not read as
//! one. A key or a count written beside an entry would be a line this crate
//! never writes, and it is answered rather than read past.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::epoch::{moment, since_epoch};
use crate::price::spent::Entry;

/// What separates an entry's length from the entry.
const AFTER_LENGTH: u8 = b' ';

/// What ends a line. No entry carries one inside it — JSON escapes every
/// control character — so this appears once per line and nowhere else.
pub(super) const END_OF_LINE: u8 = b'\n';

/// `entry`, framed as the line it goes on record as, terminator included.
pub(super) fn line(entry: &Entry) -> Result<Vec<u8>, String> {
    let (seconds, nanos) = since_epoch(entry.left())?;
    let written = Written {
        left: WrittenMoment { seconds, nanos },
        ticker: Cow::Borrowed(entry.ticker()),
        host: Cow::Borrowed(entry.host()),
    };
    let entry = serde_json::to_vec(&written).map_err(|why| why.to_string())?;

    let mut line = Vec::with_capacity(entry.len() + 16);
    line.extend_from_slice(entry.len().to_string().as_bytes());
    line.push(AFTER_LENGTH);
    line.extend_from_slice(&entry);
    line.push(END_OF_LINE);

    Ok(line)
}

/// The entry `line` holds, with its terminator already off.
///
/// `Ok(None)` is an entry that never landed whole: fewer bytes than it
/// declared, the shape a process killed mid-write leaves behind. An error is a
/// line that landed whole and says something this cannot read.
pub(super) fn read(line: &[u8]) -> Result<Option<Entry>, String> {
    let Some(space) = line.iter().position(|byte| *byte == AFTER_LENGTH) else {
        return Ok(None);
    };

    let (length, entry) = line.split_at(space);
    let entry = &entry[1..];

    let declared = std::str::from_utf8(length)
        .ok()
        .and_then(|length| length.parse::<usize>().ok())
        .ok_or_else(|| {
            format!(
                "a line begins {:?}, and an entry begins with how long it is",
                String::from_utf8_lossy(&line[..line.len().min(24)])
            )
        })?;

    if entry.len() != declared {
        return Ok(None);
    }

    let written: Written<'_> =
        serde_json::from_slice(entry).map_err(|why| format!("an entry reads as {why}"))?;

    Ok(Some(Entry::new(
        moment(written.left.seconds, written.left.nanos)?,
        &written.ticker,
        &written.host,
    )))
}

/// One entry, in the shape it is written down in.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Written<'a> {
    left: WrittenMoment,
    ticker: Cow<'a, str>,
    host: Cow<'a, str>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WrittenMoment {
    seconds: i64,
    nanos: u32,
}

#[cfg(test)]
mod tests {
    use super::{END_OF_LINE, line, read};
    use crate::price::spent::Entry;
    use std::time::{Duration, UNIX_EPOCH};

    fn sent() -> Entry {
        Entry::new(
            UNIX_EPOCH + Duration::new(1_754_000_000, 250),
            "BRK-B",
            "api.tiingo.com",
        )
    }

    /// The line as it goes on record, with the terminator off — which is how a
    /// reader is handed one.
    fn written(entry: &Entry) -> Vec<u8> {
        let mut written = line(entry).expect("an entry this crate made writes down");
        written.pop();
        written
    }

    #[test]
    fn a_line_holds_the_entry_it_was_written_from() {
        let entry = sent();

        assert_eq!(read(&written(&entry)), Ok(Some(entry)));
    }

    /// Every prefix of a line, which is every place a process can be killed
    /// inside one. None of them is an entry, and none of them is corruption.
    #[test]
    fn a_line_cut_short_anywhere_is_an_entry_that_never_landed() {
        let whole = written(&sent());

        for cut in 0..whole.len() {
            assert_eq!(
                read(&whole[..cut]),
                Ok(None),
                "{:?} read as something other than an entry cut short",
                String::from_utf8_lossy(&whole[..cut])
            );
        }
    }

    /// A line that landed whole and says something this cannot read is
    /// answered, not passed over — a fourth field included, since an entry has
    /// three.
    #[test]
    fn a_line_that_landed_whole_and_reads_as_no_entry_is_answered() {
        let with_a_count =
            br#"{"left":{"seconds":0,"nanos":0},"ticker":"KO","host":"api.tiingo.com","count":1}"#;
        let with_a_count = [
            with_a_count.len().to_string().as_bytes(),
            b" ",
            with_a_count,
        ]
        .concat();

        for corrupt in [
            &br#"7 {"a":1}"#[..],
            &br#"not-a-length {"a":1}"#[..],
            &b"0 "[..],
            &with_a_count[..],
        ] {
            assert!(
                read(corrupt).is_err(),
                "{:?} read as something other than corruption",
                String::from_utf8_lossy(corrupt)
            );
        }
    }

    /// A ticker carrying a raw line ending would read back as two lines,
    /// neither of them whole.
    #[test]
    fn an_entry_carries_no_line_ending_inside_it() {
        let entry = Entry::new(UNIX_EPOCH, "two\nlines", "api.tiingo.com");
        let written = written(&entry);

        assert!(!written.contains(&END_OF_LINE));
        assert_eq!(read(&written), Ok(Some(entry)));
    }
}
