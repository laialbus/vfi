//! Record a golden fixture from the source, through the stage's own way out.
//!
//! ```text
//! cargo run -p vfi-fetch --bin record -- \
//!     "you you@example.com" fixtures/fetch/<case> \
//!     https://www.sec.gov/files/company_tickers.json
//! ```
//!
//! A person runs this and commits what it wrote. No test runs it, and nothing
//! that runs unattended does either: a fixture is a recording taken once and
//! then argued with, and one that refreshed itself would agree with the source
//! every morning and pin nothing.
//!
//! Why it exists rather than a line of curl, which is what recorded every
//! fixture in the tree until now: what curl receives and what this stage
//! receives are two different things. Between the bytes on the wire and the body
//! a case holds sit the decodings [`Https`] undoes, the redirect it refuses to
//! follow, and the answers it will not read as a document at all — so a fixture
//! recorded by another client pins that client, and the stage is only assumed to
//! agree. Recorded through here, the case holds what the stage would have got.
//!
//! It is a caller of the stage like any other and takes no shorter path for
//! being a tool. Every request goes out through [`Egress`], so it meets the host
//! list before anything is opened and takes its turn from a [`Pace`] against the
//! rate the source publishes.
//!
//! Who is asking arrives as an argument, and there is no default to fall back on
//! when it is missing. A declaration names a person and how to reach them, which
//! is the user's to supply and never the engine's to hold: nothing here reads an
//! environment, a configuration file, or a constant for one, and a run that is
//! given none records nothing.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use vfi_fetch::{Declaration, Egress, Https, Pace, Transport};

/// The scheme the stage speaks, and the one a recording's place under a case is
/// read out of.
const SCHEME: &str = "https://";

/// The status a recording is taken from, which is the one the harness answers a
/// case's recordings with. Every other status is the source saying something
/// other than "here it is", and a body under one of those is not the document
/// the case is about — so it is reported and not written.
const DOCUMENT: u16 = 200;

/// Where the run is told what to do, in one place: who is asking, the case to
/// record into, and the URLs to record.
struct Asked {
    who: String,
    case: PathBuf,
    urls: Vec<String>,
}

/// What one recording came to.
struct Written {
    path: PathBuf,
    bytes: usize,
}

fn main() -> ExitCode {
    let asked = match read_arguments() {
        Ok(asked) => asked,
        Err(why) => {
            eprintln!("record: {why}");
            eprintln!("usage: record <who is asking> <case directory> <url>...");
            return ExitCode::from(2);
        }
    };

    let declaration = match Declaration::new(&asked.who) {
        Ok(declaration) => declaration,
        Err(why) => return stopped(&format!("{:?} is {why}", asked.who)),
    };
    let transport = match Https::new() {
        Ok(transport) => transport,
        Err(why) => return stopped(&format!("no transport to record through, because {why}")),
    };

    let mut edgar = Egress::new(transport, declaration, Pace::system());

    for url in &asked.urls {
        match record(&mut edgar, &asked.case, url) {
            Ok(written) => {
                println!("{url}");
                println!(
                    "  answered {DOCUMENT}, {} bytes, into {}",
                    written.bytes,
                    written.path.display()
                );
            }
            // Stopped at the first one rather than carried on. What is wrong
            // with one request is usually wrong with the rest of them — a
            // declaration the source refuses, a rate it is pushing back on —
            // and the run that keeps asking is the one that gets the address
            // blocked.
            Err(why) => return stopped(&why),
        }
    }

    ExitCode::SUCCESS
}

fn read_arguments() -> Result<Asked, String> {
    let mut given = env::args().skip(1);

    let (Some(who), Some(case)) = (given.next(), given.next()) else {
        return Err("who is asking and which case to record into are both wanted".to_owned());
    };
    let urls: Vec<String> = given.collect();
    if urls.is_empty() {
        return Err("no URL to record".to_owned());
    }

    Ok(Asked {
        who,
        case: PathBuf::from(case),
        urls,
    })
}

/// Fetch `url` and write what came back under `case`, at the path the URL
/// names.
///
/// The path is worked out before the request, so a URL that names no place to
/// put the answer costs no request — the same order the chokepoint reads the
/// host list in, and for the same reason.
fn record<T: Transport>(edgar: &mut Egress<T>, case: &Path, url: &str) -> Result<Written, String> {
    let path = recording_path(case, url)?;

    let answer = edgar
        .fetch(url)
        .map_err(|why| format!("{url}: not recorded, {why}"))?;
    if answer.status != DOCUMENT {
        return Err(format!(
            "{url}: answered {}, and a recording is what a source answers {DOCUMENT} with",
            answer.status
        ));
    }

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|why| format!("{}: cannot be made ({why})", dir.display()))?;
    }
    fs::write(&path, &answer.body)
        .map_err(|why| format!("{}: cannot be written ({why})", path.display()))?;

    Ok(Written {
        path,
        bytes: answer.body.len(),
    })
}

/// The path under `case` that `url` names, or why it names none.
///
/// This is the harness's own rule read backwards: it takes a case's files to be
/// the answers to the URLs their paths spell, so `www.sec.gov/files/x.json`
/// under the case is what `https://www.sec.gov/files/x.json` answered. Writing
/// the file the harness would look for is the whole of what this does.
///
/// A segment that is empty or is `.` or `..` is refused rather than resolved. It
/// addresses somewhere other than where the URL reads as pointing — `..` reaches
/// out of the case entirely — and a recording landing beside the case it was
/// meant for is a case that then asks the source for what it already has.
fn recording_path(case: &Path, url: &str) -> Result<PathBuf, String> {
    let named = url.strip_prefix(SCHEME).ok_or_else(|| {
        format!("{url}: names no path under a case, because only https is fetched")
    })?;

    let mut path = case.to_owned();
    for segment in named.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(format!(
                "{url}: names no path under a case, because {segment:?} is not a name in one"
            ));
        }
        path.push(segment);
    }

    Ok(path)
}

fn stopped(why: &str) -> ExitCode {
    eprintln!("record: {why}");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::recording_path;

    /// The mapping the harness reads: host first, then the path, under the case
    /// as it was given.
    #[test]
    fn a_recording_goes_where_the_url_names() {
        let case = Path::new("fixtures/fetch/a-case");

        assert_eq!(
            recording_path(case, "https://www.sec.gov/files/company_tickers.json"),
            Ok(PathBuf::from(
                "fixtures/fetch/a-case/www.sec.gov/files/company_tickers.json"
            ))
        );
        assert_eq!(
            recording_path(case, "https://data.sec.gov/submissions/CIK0000320193.json"),
            Ok(PathBuf::from(
                "fixtures/fetch/a-case/data.sec.gov/submissions/CIK0000320193.json"
            ))
        );
    }

    /// Every one of these would write somewhere other than the place the URL
    /// reads as naming, and the last two write outside the case altogether.
    #[test]
    fn a_url_that_names_no_place_in_a_case_records_nothing() {
        for url in [
            "http://www.sec.gov/files/company_tickers.json",
            "www.sec.gov/files/company_tickers.json",
            "https://",
            "https://www.sec.gov/",
            "https://www.sec.gov//files/company_tickers.json",
            "https://www.sec.gov/./company_tickers.json",
            "https://www.sec.gov/../company_tickers.json",
        ] {
            assert!(
                recording_path(Path::new("fixtures/fetch/a-case"), url).is_err(),
                "{url} was taken as a place in a case"
            );
        }
    }
}
