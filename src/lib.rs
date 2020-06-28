//! What if a feed, but it's a mailbox?
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD
//!   * Embark Studios


extern crate linked_hash_set;
#[macro_use]
extern crate mail_headers;
extern crate mail_core;
extern crate feed_rs;
extern crate unicase;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate mime;


mod ops;

pub mod util;
pub mod options;

pub use ops::assemble_mail;
