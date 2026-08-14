//! Where a cleared request actually goes: HTTP/1.1 over TLS, and the only code
//! in this crate that opens a connection.
//!
//! Two things it does not do carry most of its weight.
//!
//! It does not read the URL to find the host. [`Cleared::host`] is the host the
//! list was asked about, and it is what this connects to and what it sends as
//! `Host`. Anything else — a client's own parse of the authority — is how the
//! host that was checked and the host that is reached come apart.
//!
//! It does not follow a redirect. A 3xx is handed back as the answer it is,
//! because the host a `Location` names is one no list has seen: a client that
//! follows underneath the check turns the allowlist into a check on the first
//! hop, and the body arrives from wherever the last hop was. A caller that
//! wants it followed asks [`super::Egress::fetch`] for the new URL, which reads
//! the list for that host like any other.
//!
//! Both are why the HTTP here is written out rather than taken from a client
//! library: a library is handed the URL and decides both of those for itself,
//! and following is what most of them do by default.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use native_tls::TlsConnector;

use super::{Cleared, Response, Transport};

/// The port https names, and the only one this reaches. A URL carrying a port
/// of its own does not get past the check — a port is not part of what the list
/// allows — so there is nothing else for this to be.
const PORT: u16 = 443;

/// How long a socket may go without a byte before it is taken as a wire that
/// failed.
///
/// A run is unattended, so a source that accepts a connection and then says
/// nothing has to cost a bounded amount of time rather than the whole night.
/// Well above what a slow answer takes, because a source that is merely busy
/// must not be called dead.
const ANSWER_WITHIN: Duration = Duration::from_secs(30);

/// The transport that reaches a real source.
pub struct Https {
    connector: TlsConnector,
}

impl Https {
    /// Build one, verifying certificates against the trust store the machine
    /// keeps.
    ///
    /// There is no argument for turning that off, here or anywhere a caller can
    /// reach: the switch lives on a builder, and this makes the connector
    /// rather than taking one. An allowlist over a connection nobody
    /// authenticated names a host rather than reaching one.
    pub fn new() -> io::Result<Self> {
        TlsConnector::new()
            .map(|connector| Self { connector })
            .map_err(io::Error::other)
    }
}

impl Transport for Https {
    fn send(&mut self, request: Cleared<'_>) -> io::Result<Response> {
        let host = request.host();

        let socket = TcpStream::connect((host, PORT))?;
        socket.set_read_timeout(Some(ANSWER_WITHIN))?;
        socket.set_write_timeout(Some(ANSWER_WITHIN))?;

        let stream = self
            .connector
            .connect(host, socket)
            .map_err(|why| match why {
                native_tls::HandshakeError::Failure(why) => io::Error::other(why),
                // A handshake that ran into the timeout above comes back
                // holding the stream rather than an error, and the stream is
                // not a thing an `io::Error` can carry. Its message is, and it
                // says which of the two happened.
                held => io::Error::other(held.to_string()),
            })?;

        exchange(stream, &request)
    }
}

/// Ask over a stream that is already open, and make a [`Response`] of what
/// comes back.
///
/// Kept apart from opening the stream because this is the half that can be
/// shown without a source: what goes out, and what is made of a redirect, of a
/// refusal, and of a connection that stops mid-answer, all hold over a stream a
/// test holds in memory.
fn exchange<S: Read + Write>(mut stream: S, request: &Cleared<'_>) -> io::Result<Response> {
    stream.write_all(&request_head(request))?;
    stream.flush()?;

    let mut answer = BufReader::new(stream);
    let mut line = Vec::new();

    if !read_line(&mut answer, &mut line)? {
        return Err(cut_short(
            "the connection ended before a status line arrived",
        ));
    }
    let status = status_of(&line)?;

    let mut length = None;
    let mut chunked = false;
    loop {
        if !read_line(&mut answer, &mut line)? {
            return Err(cut_short(
                "the connection ended part way through the headers",
            ));
        }
        if line.is_empty() {
            break;
        }

        let Some(colon) = line.iter().position(|byte| *byte == b':') else {
            return Err(malformed("a header line carries no name"));
        };
        let (name, value) = line.split_at(colon);
        let value = trimmed(&value[1..]);

        if name.eq_ignore_ascii_case(b"content-length") {
            length = Some(length_of(value)?);
        } else if name.eq_ignore_ascii_case(b"transfer-encoding") {
            chunked = names_no_coding_but(b"chunked", value)?;
        } else if name.eq_ignore_ascii_case(b"content-encoding") {
            names_no_coding_but(b"identity", value)?;
        }
    }

    // Chunked first where both are said: the length then describes the body
    // before it was cut up, and what arrives is the chunks.
    let body = if chunked {
        read_chunked(&mut answer)?
    } else if let Some(length) = length {
        let mut body = Vec::new();
        answer.by_ref().take(length).read_to_end(&mut body)?;
        if (body.len() as u64) < length {
            return Err(cut_short(
                "the body is shorter than the length the answer declares",
            ));
        }
        body
    } else {
        let mut body = Vec::new();
        answer.read_to_end(&mut body)?;
        body
    };

    Ok(Response { status, body })
}

/// The request, as the bytes that go out.
///
/// Three headers beyond the declaration, and each is about the protocol rather
/// than about who is asking. `Host` is what HTTP/1.1 requires and is the host
/// the list cleared. `Accept-Encoding: identity` is because a body is handed
/// back as it arrived and undecoded, so what arrives has to be the document
/// itself rather than a coding of it. `Connection: close` is because this opens
/// one connection per request and reads one answer off it.
fn request_head(request: &Cleared<'_>) -> Vec<u8> {
    let mut head = Vec::new();

    head.extend_from_slice(b"GET ");
    head.extend_from_slice(request.target().as_bytes());
    head.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    head.extend_from_slice(request.host().as_bytes());
    head.extend_from_slice(b"\r\n");

    for (name, value) in request.headers() {
        head.extend_from_slice(name.as_bytes());
        head.extend_from_slice(b": ");
        head.extend_from_slice(value.as_bytes());
        head.extend_from_slice(b"\r\n");
    }

    head.extend_from_slice(b"Accept-Encoding: identity\r\nConnection: close\r\n\r\n");
    head
}

/// The status a status line carries.
///
/// Strict about the shape, because a line this could not read is not an answer
/// to make a status out of, and a number guessed from one would be a refusal
/// and a filing told apart by nothing.
fn status_of(line: &[u8]) -> io::Result<u16> {
    let after_version = line
        .strip_prefix(b"HTTP/1.")
        .and_then(|rest| rest.split_first())
        .filter(|(version, _)| version.is_ascii_digit())
        .map(|(_, rest)| rest)
        .ok_or_else(|| malformed("the answer does not begin with an HTTP/1.x status line"))?;

    let status = after_version
        .strip_prefix(b" ")
        .and_then(|rest| rest.get(..3))
        .filter(|status| status.iter().all(u8::is_ascii_digit))
        .ok_or_else(|| malformed("the status line carries no three-digit status"))?;

    Ok(status
        .iter()
        .fold(0, |status, digit| status * 10 + u16::from(digit - b'0')))
}

/// The body of an answer that arrives in chunks, put back together.
fn read_chunked<R: BufRead>(answer: &mut R) -> io::Result<Vec<u8>> {
    let mut body = Vec::new();
    let mut line = Vec::new();

    loop {
        if !read_line(answer, &mut line)? {
            return Err(cut_short("the connection ended where a chunk size was due"));
        }
        let size = chunk_size(&line)?;
        if size == 0 {
            break;
        }

        let so_far = body.len();
        answer.by_ref().take(size).read_to_end(&mut body)?;
        if (body.len() - so_far) as u64 != size {
            return Err(cut_short(
                "a chunk is shorter than the size that announced it",
            ));
        }

        if !read_line(answer, &mut line)? || !line.is_empty() {
            return Err(malformed(
                "a chunk does not end where its size said it would",
            ));
        }
    }

    // Trailers, to the blank line that ends them, and no further. The body is
    // whole at this point, so a connection that ends here has ended after
    // everything that was asked for, however it ended.
    while matches!(read_line(answer, &mut line), Ok(true)) && !line.is_empty() {}

    Ok(body)
}

/// The size a chunk announces: hexadecimal, up to the `;` that would begin an
/// extension this has no use for.
fn chunk_size(line: &[u8]) -> io::Result<u64> {
    let announced = match line.iter().position(|byte| *byte == b';') {
        Some(extension) => &line[..extension],
        None => line,
    };
    let announced = trimmed(announced);
    if announced.is_empty() {
        return Err(malformed("a chunk announces no size"));
    }

    let mut size: u64 = 0;
    for byte in announced {
        let digit = char::from(*byte)
            .to_digit(16)
            .ok_or_else(|| malformed("a chunk announces a size that is not hexadecimal"))?;
        size = size
            .checked_mul(16)
            .and_then(|size| size.checked_add(u64::from(digit)))
            .ok_or_else(|| malformed("a chunk announces a size no answer has"))?;
    }

    Ok(size)
}

/// How long a `Content-Length` says the body is.
fn length_of(value: &[u8]) -> io::Result<u64> {
    if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
        return Err(malformed(
            "the answer declares a body length that is not a number",
        ));
    }

    let mut length: u64 = 0;
    for byte in value {
        length = length
            .checked_mul(10)
            .and_then(|length| length.checked_add(u64::from(byte - b'0')))
            .ok_or_else(|| malformed("the answer declares a body longer than there can be"))?;
    }

    Ok(length)
}

/// Whether a list of codings names `spoken`, and an error where it names any
/// other.
///
/// This speaks two: `chunked`, which it undoes, and `identity`, which is
/// nothing to undo and is what the request asked for. A body is handed back as
/// it arrived, so a body under a third coding is not the document — and handing
/// it on as one would read downstream as a source publishing nonsense rather
/// than as a client that could not undo what it was sent.
fn names_no_coding_but(spoken: &[u8], value: &[u8]) -> io::Result<bool> {
    let mut named = false;

    for coding in value.split(|byte| *byte == b',') {
        let coding = trimmed(coding);
        if coding.is_empty() {
            continue;
        }
        if !coding.eq_ignore_ascii_case(spoken) {
            return Err(malformed(
                "the answer arrives under a coding this cannot undo",
            ));
        }
        named = true;
    }

    Ok(named)
}

/// One line of the answer, without the newline that ended it. False where the
/// connection ended instead, which the caller decides the meaning of: part way
/// through the head it is an answer cut short, and after the body it is only
/// the end.
fn read_line<R: BufRead>(answer: &mut R, line: &mut Vec<u8>) -> io::Result<bool> {
    line.clear();
    if answer.read_until(b'\n', line)? == 0 || line.last() != Some(&b'\n') {
        return Ok(false);
    }

    line.pop();
    if line.last() == Some(&b'\r') {
        line.pop();
    }

    Ok(true)
}

/// `value` without the whitespace a header allows around it.
fn trimmed(value: &[u8]) -> &[u8] {
    let from = value
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(value.len());
    let to = value
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map_or(from, |last| last + 1);

    &value[from..to]
}

/// The source answered with something no answer is made of. The wire held.
fn malformed(what: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, what)
}

/// The connection ended where more was due.
fn cut_short(what: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, what)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read, Write};

    use super::super::{Cleared, Response, host_of};
    use super::{Https, exchange};
    use crate::policy::DECLARATION_HEADER;

    /// Who the cases declare. Nothing here reaches EDGAR, so this is only a
    /// value to find again in the bytes that went out.
    const WHO: &str = "VFI vfi@example.invalid";

    /// A source held in memory: it answers with the bytes a case gives it and
    /// keeps what it was asked.
    ///
    /// Deliberately not a socket, not even a loopback one. Everything this
    /// transport decides — what goes out, and what it makes of a redirect, of a
    /// refusal, and of a connection that stops mid-answer — is decided over a
    /// stream, so a pair of buffers is the whole of what a case needs. The
    /// answer is then the same on a machine that is offline and on one whose
    /// sandbox will not open a port at all, which is more than "no live
    /// network" and is the property worth having.
    ///
    /// What is left over is opening the connection and the handshake on it, and
    /// no test can witness those. A recording taken from the source through
    /// this transport is what does.
    struct Held {
        answer: &'static [u8],
        read: usize,
        asked: Vec<u8>,
    }

    impl Read for Held {
        fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
            let left = &self.answer[self.read..];
            let taken = left.len().min(into.len());
            into[..taken].copy_from_slice(&left[..taken]);
            self.read += taken;
            Ok(taken)
        }
    }

    impl Write for Held {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.asked.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// One exchange against such a source.
    struct Exchanged {
        /// What was made of the answer.
        answered: io::Result<Response>,
        /// The bytes the source was sent, all of them.
        asked: String,
    }

    fn against(answer: &'static [u8], url: &str) -> Exchanged {
        let mut held = Held {
            answer,
            read: 0,
            asked: Vec::new(),
        };
        let answered = exchange(&mut held, &cleared(url));

        Exchanged {
            answered,
            asked: String::from_utf8(held.asked).expect("a request written in ASCII"),
        }
    }

    /// A request as the chokepoint makes one, built the same way: the host is
    /// what the check reads out of the URL, never a second answer about it.
    fn cleared(url: &str) -> Cleared<'_> {
        Cleared {
            url,
            host: host_of(url).expect("a URL the check clears"),
            declaration: WHO,
        }
    }

    #[test]
    fn an_answer_is_its_status_and_its_body() {
        let exchanged = against(
            b"HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\n{\"cik\":320193}",
            "https://data.sec.gov/submissions/CIK0000320193.json",
        );

        let answer = exchanged.answered.expect("the source answered");
        assert_eq!(answer.status, 200);
        assert_eq!(answer.body, b"{\"cik\":320193}");
    }

    /// A body cut into chunks is one document again. EDGAR sends the large ones
    /// this way, so a transport that handed the framing on as part of the body
    /// would corrupt exactly the filings that matter most — and would look, to
    /// everything downstream, like a source that publishes broken JSON.
    #[test]
    fn a_body_that_arrives_in_chunks_is_put_back_together() {
        let exchanged = against(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
              a\r\n{\"cik\":320\r\n4\r\n193}\r\n0\r\n\r\n",
            "https://data.sec.gov/submissions/CIK0000320193.json",
        );

        let answer = exchanged.answered.expect("the source answered");
        assert_eq!(answer.status, 200);
        assert_eq!(answer.body, b"{\"cik\":320193}");
    }

    /// The criterion this transport exists not to undo. The 3xx comes back as
    /// the answer it is, and the host its `Location` names — one no list has
    /// been asked about — is not requested: one request went out, and it is the
    /// one that was cleared.
    #[test]
    fn a_redirect_is_handed_back_and_not_followed() {
        let exchanged = against(
            b"HTTP/1.1 301 Moved Permanently\r\n\
              Location: https://filings.example/CIK0000320193.json\r\n\
              Content-Length: 0\r\n\r\n",
            "https://data.sec.gov/submissions/CIK0000320193.json",
        );

        let answer = exchanged.answered.expect("the source answered");
        assert_eq!(answer.status, 301);
        assert_eq!(
            exchanged.asked.matches("GET ").count(),
            1,
            "more than the cleared request went out: {:?}",
            exchanged.asked
        );
        assert!(
            !exchanged.asked.contains("filings.example"),
            "the redirect was followed to a host no list cleared: {:?}",
            exchanged.asked
        );
    }

    /// Each of these is the source saying something, and none of them is the
    /// wire failing. The stage above tells a refusal from an absence, and it
    /// cannot if both arrive as an error.
    #[test]
    fn a_status_that_is_a_refusal_is_still_an_answer() {
        for (answer, status) in [
            (
                &b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n"[..],
                403,
            ),
            (
                &b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"[..],
                404,
            ),
            (
                &b"HTTP/1.1 429 Too Many Requests\r\nContent-Length: 0\r\n\r\n"[..],
                429,
            ),
            (
                &b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n"[..],
                500,
            ),
        ] {
            let exchanged = against(
                answer,
                "https://www.sec.gov/Archives/edgar/data/320193/x.htm",
            );

            let answered = exchanged.answered.expect("the source answered");
            assert_eq!(answered.status, status);
        }
    }

    /// The other half of that: an error is the answer not arriving. A body cut
    /// off part way is the case that matters, because a transport that handed
    /// back what it had would be handing back a truncated filing as a whole one.
    #[test]
    fn a_connection_that_stops_part_way_is_an_error() {
        for answer in [
            &b""[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: 14\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\n{\"cik\""[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\na\r\n{\"cik\""[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n"[..],
        ] {
            let exchanged = against(
                answer,
                "https://data.sec.gov/submissions/CIK0000320193.json",
            );

            assert!(
                exchanged.answered.is_err(),
                "{:?} was taken as a whole answer",
                String::from_utf8_lossy(answer)
            );
        }
    }

    /// An answer no status can be read out of is not a status of zero, and a
    /// length or a chunk size that is not a number is not a length of none.
    #[test]
    fn what_is_not_an_answer_is_an_error() {
        for answer in [
            &b"nothing that is an answer\r\n\r\n"[..],
            &b"HTTP/2 200\r\n\r\n"[..],
            &b"HTTP/1.1 OK\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\na header with no name\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: some\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nsome\r\n"[..],
        ] {
            let exchanged = against(
                answer,
                "https://data.sec.gov/submissions/CIK0000320193.json",
            );

            assert!(
                exchanged.answered.is_err(),
                "{:?} was read as an answer",
                String::from_utf8_lossy(answer)
            );
        }
    }

    #[test]
    fn the_request_asks_the_cleared_host_for_what_the_url_names() {
        let exchanged = against(
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n",
            "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany",
        );

        assert!(
            exchanged
                .asked
                .starts_with("GET /cgi-bin/browse-edgar?action=getcompany HTTP/1.1\r\n"),
            "{:?}",
            exchanged.asked
        );
        assert!(
            exchanged.asked.contains("\r\nHost: www.sec.gov\r\n"),
            "{:?}",
            exchanged.asked
        );
    }

    /// The request asks for the document itself, and a body under a coding this
    /// cannot undo is refused rather than handed on. Handed on, it would reach
    /// the stage above looking like a source that publishes nonsense.
    #[test]
    fn a_body_under_a_coding_this_cannot_undo_is_not_a_body() {
        for answer in [
            &b"HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 2\r\n\r\nxx"[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, chunked\r\n\r\n0\r\n\r\n"[..],
        ] {
            let exchanged = against(
                answer,
                "https://data.sec.gov/submissions/CIK0000320193.json",
            );

            assert!(
                exchanged.answered.is_err(),
                "{:?} was handed on as the document",
                String::from_utf8_lossy(answer)
            );
        }
    }

    /// The other side of that rule, so it refuses a coding rather than refusing
    /// to say a body is uncoded.
    #[test]
    fn a_body_under_no_coding_is_the_document() {
        let exchanged = against(
            b"HTTP/1.1 200 OK\r\nContent-Encoding: identity\r\nContent-Length: 14\r\n\r\n\
              {\"cik\":320193}",
            "https://data.sec.gov/submissions/CIK0000320193.json",
        );

        let answer = exchanged.answered.expect("the source answered");
        assert_eq!(answer.body, b"{\"cik\":320193}");
    }

    /// The declaration the `Egress` was built with, under the name the policy
    /// spells — read from there rather than written out a second time here.
    #[test]
    fn the_request_declares_who_is_asking() {
        let exchanged = against(
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n",
            "https://data.sec.gov/submissions/CIK0000320193.json",
        );

        assert!(
            exchanged
                .asked
                .contains(&format!("\r\n{DECLARATION_HEADER}: {WHO}\r\n")),
            "{:?}",
            exchanged.asked
        );
    }

    /// The one thing about the connecting half that costs no connection: the
    /// verifier is built from the trust store the machine keeps, so a machine
    /// with none says so here rather than on the first request.
    #[test]
    fn a_transport_is_built_before_anything_is_opened() {
        Https::new().expect("the machine keeps a trust store to verify against");
    }
}
