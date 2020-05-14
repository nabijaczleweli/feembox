//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use feembox::Options;
//! let options = Options::parse();
//! if options.verbose {
//!     println!("{} -> {}", options.feed.0, options.maildir.0);
//! }
//! ```


use std::path::{PathBuf, Path};
use clap::{Arg, AppSettings};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    pub maildir: (Cow<'static, str>, PathBuf),
    pub feed: (Cow<'static, str>, Option<PathBuf>),
    pub verbose: bool,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        #[allow(deprecated)]
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[MAILDIR] 'Where to write to the mails to. Default: .'").validator(|s| Options::parse_maildir_path(&s).map(|_| ())))
            .arg(Arg::from_usage("[FEED] 'Where to read the feed from. Default: stdin'").validator(|s| Options::parse_feed_path(&s).map(|_| ())))
            .arg(Arg::from_usage("-v --verbose 'Print what's happening to stderr'"))
            .get_matches();

        Options {
            maildir: match matches.value_of("MAILDIR") {
                Some(maildir) => (maildir.to_string().into(), Options::parse_maildir_path(maildir).expect("Race between validation and parse")),
                None => ("./".into(), PathBuf::new()),
            },
            feed: match matches.value_of("FEED") {
                Some(feed) => {
                    match Options::parse_feed_path(feed).expect("Race between validation and parse") {
                        Some(feed_path) => (feed.to_string().into(), Some(feed_path)),
                        None => ("<stdin>".into(), None),
                    }
                }
                None => ("<stdin>".into(), None),
            },
            verbose: matches.is_present("verbose"),
        }
    }

    fn parse_maildir_path(s: &str) -> Result<PathBuf, String> {
        let path = Path::new(s);

        let parent_dir = {
            let pd = path.parent().unwrap_or(path);
            if pd == Path::new("") {
                Path::new(".")
            } else {
                pd
            }
        };
        let mail_name = path.file_name().unwrap_or(OsStr::new(".."));

        let mut path = fs::canonicalize(parent_dir).map_err(|_| format!("Parent to maildir \"{}\" doesn't exist", s))?;
        path.push(mail_name);

        Ok(match path.canonicalize() {
            Ok(canon_path) => canon_path,
            Err(_) => path,
        })
    }

    fn parse_feed_path(s: &str) -> Result<Option<PathBuf>, String> {
        if s == "-" {
            return Ok(None);
        }

        fs::canonicalize(&s).map_err(|_| format!("Feed file \"{}\" doesn't exist", s)).and_then(|f| if f.is_file() {
            Ok(Some(f))
        } else {
            Err(format!("Feed file \"{}\" ({}) not a file", s, f.display()))
        })
    }
}
