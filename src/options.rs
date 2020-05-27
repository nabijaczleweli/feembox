//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use feembox::options::Options;
//! let options = Options::parse();
//! if options.verbose {
//!     println!("{} -> {}", options.feed.0, options.maildir.0);
//! }
//! ```


use std::path::{self, PathBuf, Path};
use clap::{Arg, AppSettings};
use std::borrow::Cow;
use std::fs;


/// Verbosity level to print to stdout
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    /// Print nothing
    None,
    /// Print data for human consumption
    Human,
    /// Print values useful for debugging
    Debug,
}

impl From<u64> for Verbosity {
    fn from(n: u64) -> Verbosity {
        match n {
            0 => Verbosity::None,
            1 => Verbosity::Human,
            _ => Verbosity::Debug,
        }
    }
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The location of the maildir to deliver to
    ///
    /// Parents must exist. Subdirs and itself will be created as needed. Default: CWD
    pub maildir: (Cow<'static, str>, PathBuf),
    /// The location of the maildir to deliver to, or `-` for stdin
    ///
    /// If `None`: read from stdin
    ///
    /// Default: "<stdin>" + `None`
    pub feed: (Cow<'static, str>, Option<PathBuf>),
    /// Print what's happening to stdout
    pub verbosity: Verbosity,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        #[allow(deprecated)]
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[MAILDIR] 'Where to write to the mails to. Default: .'").validator(|s| Options::parse_maildir_path(&s).map(|_| ())))
            .arg(Arg::from_usage("[FEED] 'Where to read the feed from. Default: stdin'").validator(|s| Options::parse_feed_path(&s).map(|_| ())))
            .arg(Arg::from_usage("-v --verbose 'Print what's happening to stdout'").multiple(true))
            .get_matches();

        Options {
            maildir: match matches.value_of("MAILDIR") {
                Some(maildir) => {
                    ({
                         let mut md = maildir.to_string();
                         if !md.ends_with(path::is_separator) {
                             md.push('/');
                         }
                         md.into()
                     },
                     Options::parse_maildir_path(maildir).expect("Race between validation and parse"))
                }
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
            verbosity: matches.occurrences_of("verbose").into(),
        }
    }

    fn parse_maildir_path(s: &str) -> Result<PathBuf, String> {
        let full_path: Vec<_> = Path::new(s).components().collect();

        match full_path.len() {
            0 => Err(format!("Path to maildir \"{}\" empty", s)),
            1 => Ok(full_path.iter().collect()),
            _ => {
                let parent_dir: PathBuf = full_path.iter().take(full_path.len() - 1).collect();

                let mut path = fs::canonicalize(parent_dir).map_err(|_| format!("Parent to maildir \"{}\" doesn't exist", s))?;
                path.push(full_path[full_path.len() - 1]);

                Ok(match path.canonicalize() {
                    Ok(canon_path) => canon_path,
                    Err(_) => path,
                })
            }
        }
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
