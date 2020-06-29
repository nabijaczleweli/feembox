//! What if a feed, but it's a mailbox?
//!
//! # Howto
//!
//! Use [`assemble_mail()`](fn.assemble_mail.html) once for each entry.
//!
//! Consult [`main.rs`](https://github.com/nabijaczleweli/feembox/blob/trunk/src/main.rs)
//! for example parsing and maildir delivery.
//!
//! # Manpage
//!
//! ## SYNOPSIS
//!
//! `feembox [-v] [-t FROM:TO:HOW]... [-f MIME] [MAILDIR] [FEED]`<br />
//! `feembox [-v] [-t FROM:TO:HOW]... [-f MIME] [MAILDIR] < feed.xml`
//!
//! ## DESCRIPTION
//!
//! `feembox` represents an (RSS/Atom/JSON) feed as a mailbox in the [maildir](https://cr.yp.to/proto/maildir.html) format.
//!
//! ## OPTIONS
//!
//! `[MAILDIR]`
//!
//! Deliver to the specified directory instead of the CWD
//!
//! Parents must exist, all directory and its subdirs will be created as necessary
//!
//! `[FEED]`
//!
//! Read the feed from the specified file instead of stdin
//!
//! If "-" use stdin, otherwise must exist and be a file
//!
//! `-v --verbose`
//!
//! Print what's happening to the standard output,
//! if specified twice: print parse debugging information.
//!
//! `-t --transfrom <FROM:TO:HOW|FROM;TO;HOW>...`
//!
//! Define an alternative transformation invocation HOW
//! from the mime-type FROM to the mime-type TO.
//!
//! If the post content type matches FROM, "/bin/sh -c HOW" ("cmd /C HOW" on NT)
//! is executed, its standard input tied thereto, and standard output
//! to the buffer for the new multipart/alternative part TO.
//!
//! The separator between FROM, TO, and HOW is the platform's path list separator
//! (i.e. ";" on NT and ":" elsewhere).
//!
//! Can be specified multiple times, in which case each transformation is invoked once,
//! in order, on the current set of parts.
//!
//! `-f --force <MIME>`
//!
//! Force the post content type to be MIME, overriding what's specified therein.
//!
//! This is done before any transformations.
//!
//! Some feeds specify they're text/plain but are HTML,
//! this can be used to massage them right.
//!
//! ## EXIT VALUES
//!
//! |   |                                              |
//! | - |                                              |
//! | 1 | option parse error                           |
//! | 2 | feed file open failed                        |
//! | 3 | feed parse failed                            |
//! | 4 | maildir subdirectory read failed             |
//! | 5 | existing mail open(2)/mmap(2) failed         |
//! | 6 | creating a MAILDIR/tmp or MAILDIR/new failed |
//! | 7 | formatting mail failed                       |
//! | 8 | creating/writing/delivering mail failed      |
//!
//! ## EXAMPLES
//!
//! Turndown (here: https://github.com/domchristie/turndown/pull/209 lightly patched to always read from stdin),
//! can be used to turn HTML feeds (i.e. most of them) into usually pretty readable plaintext:
//!
//! ```plaintext
//! P:\Rust\feembox>cat test-data/util-linux-newer.atom |   target\debug\feembox -vt text/html;text/plain;turndown feedir
//! ~/code/feembox$ cat test-data/util-linux-newer.atom | ./target/debug/feembox -vt 'text/html:text/plain;charset=utf-8:~/code/turndown/bin/turndown.js' feedir
//! <stdin>: feed ID mailto:util-linux@vger.kernel.org, title "Util-Linux Archive on lore.kernel.org", updated 2020-05-18T11:41:20+00:00
//! 25 entries:
//!     entry ID                                       title                                                                    updated                    published
//!     urn:uuid:d2c69230-d7ba-e4cf-ee51-2b25d53119a1  "Re: [PATCH] util-linux: Some minor fixes in some manuals"               2020-05-18T11:40:38+00:00  N/A
//!     urn:uuid:d9e31d9a-5d6c-8403-9661-2d303e6fb24a  "Re: [PATCH] Fix dead references to kernel documentation"                2020-05-18T11:39:59+00:00  N/A
//!     urn:uuid:dccd47a2-d327-df9b-7ad1-9218d08a8349  "Re: Consistency fixes in util-linux man pages"                          2020-05-18T10:36:15+00:00  N/A
//!     urn:uuid:ba1cf039-280f-f33d-199d-86f5a9c1bb1b  "Re: Consistency fixes in util-linux man pages"                          2020-05-18T08:28:25+00:00  N/A
//!     urn:uuid:98e3d4cd-17c1-ac1d-3dd9-7bbe87f6402e  "[PATCH] Fix dead references to kernel documentation"                    2020-05-17T15:13:35+00:00  N/A
//!     urn:uuid:68946069-8d16-0362-57e6-feba21ebbecb  "Consistency fixes in util-linux man pages"                              2020-05-16T08:25:11+00:00  N/A
//!     urn:uuid:94158972-3786-e3c6-7e0a-dec2988cd32b  "[PATCH] ipcs.1: ipcs no longer needs read permission on IPC resources"  2020-05-16T08:10:32+00:00  N/A
//!     urn:uuid:37f8f0a9-d285-b8f9-5269-6eeb2475f9ba  "plan for v2.35.2"                                                       2020-05-15T13:05:16+00:00  N/A
//!
//! Delivering urn:uuid:ba1cf039-280f-f33d-199d-86f5a9c1bb1b to feedir/new/1591610344.M981513P12500Q1.nabuter
//! Delivering urn:uuid:d2c69230-d7ba-e4cf-ee51-2b25d53119a1 to feedir/new/1591610346.M86312P12500Q2.nabuter
//! Delivering urn:uuid:d9e31d9a-5d6c-8403-9661-2d303e6fb24a to feedir/new/1591610347.M339148P12500Q3.nabuter
//! Delivering urn:uuid:dccd47a2-d327-df9b-7ad1-9218d08a8349 to feedir/new/1591610348.M505433P12500Q4.nabuter
//! ```
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
