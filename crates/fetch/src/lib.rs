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
//! Nothing here reaches a real source yet. What does arrives as an
//! implementation of [`Transport`], and it arrives inside the chokepoint,
//! because that is the one place the gate allows it.

mod egress;
mod hosts;

pub use egress::{Cleared, Egress, Error, Response, Transport};
pub use hosts::ALLOWED_HOSTS;
