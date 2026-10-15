//! Market prices: what the source is asked, and what asking has cost.
//!
//! So far only the second of those. [`spent`] is the record of every request
//! the provider has sent a source, kept so that a free tier's allowance holds
//! by count across sessions rather than by the source refusing. The provider
//! itself, its key and its requests are written by the task that writes them.
//!
//! Code here sits where a key will be held, and the `secrets` gate reads it as
//! such. [`spent`] is the one directory under it that keeps a file, and it
//! never names the key's type.

pub mod spent;
