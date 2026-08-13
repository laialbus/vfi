//! Fetch stage: retrieves filings from their source.
//!
//! Every request this stage sends leaves through one chokepoint, [`Egress`],
//! and the hosts it may reach are [`ALLOWED_HOSTS`] — one list, in one file,
//! read before anything is opened.
//!
//! What makes that a property rather than a habit is that a transport cannot be
//! handed a request the list has not seen. [`Transport::send`] takes a
//! [`Cleared`], which only the chokepoint can make, so a call site that tried
//! to send around the list does not compile, and the build gate is that
//! compilation. The other direction — a call site that opens its own connection
//! and needs no transport at all — is not something the compiler can see, and
//! the `egress` gate in `scripts/gates.sh` is what reads the workspace for it.
//!
//! The same seam is where the source's published access policy is kept:
//! requests leave no faster than [`MAX_REQUESTS_PER_SECOND`], counted by a
//! [`Pace`] that callers share rather than each keep, and every one of them
//! carries the [`Declaration`] of who is asking. That declaration names a
//! person and how to reach them, so it is the user's to supply and arrives as
//! a parameter.
//!
//! Nothing here reaches a real source yet. What does arrives as an
//! implementation of [`Transport`], and it arrives inside the chokepoint,
//! because that is the one place the gate allows it.
//!
//! What the stage asks those hosts for is [`filing_history`]: a ticker, the map
//! EDGAR keys it by, and then everything that filer has filed. Every record it
//! hands back carries the [`Source`] of the request that produced it, so a
//! later stage traces a value to the document it came out of without asking
//! again.
//!
//! Which filers it asks about at all is [`funnel`]: the filers EDGAR publishes,
//! then the ones its metadata does not rule out, then a history for those. Most
//! of the corpus stops before the last of those, which is the point — the funnel
//! is the requests it does not send.
//!
//! What the stage decides about a filer it is asked to consider goes on record
//! through [`ledger`], which is the one way to a verdict and the only way back
//! to one. That stays a module of its own rather than more names out here,
//! because a step, a reason, and a verdict mean something beside the ledger
//! they belong to and very little without it.

mod company;
mod edgar;
mod egress;
mod filing;
pub mod funnel;
mod hosts;
pub mod ledger;
mod pace;
mod policy;
mod source;

pub use company::{Cik, Company, Ticker};
pub use edgar::{
    Directory, Entry, Listed, Metadata, Retrieved, Seeds, Unretrieved, filing_history, history,
    metadata, seed_set,
};
pub use egress::{Cleared, Egress, Error, Response, Transport};
pub use filing::{Filing, History};
pub use hosts::ALLOWED_HOSTS;
pub use pace::{Clock, Pace, SystemClock};
pub use policy::{
    DECLARATION_HEADER, Declaration, MAX_REQUESTS_PER_SECOND, MINIMUM_SPACING, Undeclared,
};
pub use source::Source;
