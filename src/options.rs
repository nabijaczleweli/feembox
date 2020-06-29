//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use feembox::options::{Verbosity, Options};
//! let options = Options::parse();
//! if options.verbosity >= Verbosity::Human {
//!     println!("{} -> {}", options.feed.0, options.maildir.0);
//! }
//! ```


use std::path::{self, PathBuf, Path};
use clap::{Arg, AppSettings};
use std::borrow::Cow;
use mime::Mime;
use std::fs;


#[cfg(target_os="windows")]
static PATH_LIST_SEPARATOR: char = ';';
#[cfg(not(target_os="windows"))]
static PATH_LIST_SEPARATOR: char = ':';

#[cfg(target_os="windows")]
static ALTERNATIVES_TRANSFORMATIONS_ARG: &str = "-t --transform... [FROM;TO;HOW] 'Transfrom FROM mime-type to an alternative TO mime-type by running HOW'";
#[cfg(not(target_os="windows"))]
static ALTERNATIVES_TRANSFORMATIONS_ARG: &str = "-t --transform... [FROM:TO:HOW] 'Transfrom FROM mime-type to an alternative TO mime-type by running HOW'";


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
    /// The program to invoke to transform the first mime-type to the second mime-type in `from:to:how` format.
    ///
    /// Default: empty.
    pub alternatives_transformations: Vec<(Mime, Mime, String)>,
    /// What mime-type to set the content to before transformations.
    ///
    /// Default: `None`
    pub mime_override: Option<Mime>,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        #[allow(deprecated)]
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[MAILDIR] 'Where to write to the mails to. Default: .'").validator(|s| Options::parse_maildir_path(&s).map(|_| ())))
            .arg(Arg::from_usage("[FEED] 'Where to read the feed from. Default: stdin'").validator(|s| Options::parse_feed_path(&s).map(|_| ())))
            .arg(Arg::from_usage("-v --verbose... 'Print what's happening to stdout'"))
            .arg(Arg::from_usage(ALTERNATIVES_TRANSFORMATIONS_ARG)
                .validator(|s| Options::parse_alternatives_transformations(&s).map(|_| ()))
                .number_of_values(1))
            .arg(Arg::from_usage("-f --force [MIME] 'Type to force content to'").validator(|s| Options::parse_mime_override(&s).map(|_| ())))
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
            alternatives_transformations: matches.values_of("transform")
                .into_iter()
                .flatten()
                .map(Options::parse_alternatives_transformations)
                .map(|r| r.expect("Race between validation and parse"))
                .collect(),
            mime_override: matches.value_of("force").map(Options::parse_mime_override).map(|m| m.expect("Race between validation and parse")),
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

    fn parse_alternatives_transformations(s: &str) -> Result<(Mime, Mime, String), String> {
        let mut itr = s.split(PATH_LIST_SEPARATOR);
        match (itr.next(), itr.next(), itr.next(), itr.next()) {
            (Some(from), Some(to), Some(how), None) => {
                let from = from.parse().map_err(|e| format!("Transformation triple \"{}\"'s FROM not a mime-type: {}", s, e))?;
                let to = to.parse().map_err(|e| format!("Transformation triple \"{}\"'s to not a mime-type: {}", s, e))?;
                Ok((from, to, how.to_string()))
            }
            (_, _, _, Some(_)) => Err(format!("Transformation triple \"{}\" has four components", s)),
            (_, Some(_), _, _) => Err(format!("Transformation triple \"{}\" has two components", s)),
            (Some(_), _, _, _) => Err(format!("Transformation triple \"{}\" has one component", s)),
            (_, _, _, _) => Err(format!("Transformation triple \"{}\" has no components", s)),
        }
    }

    fn parse_mime_override(s: &str) -> Result<Mime, String> {
        s.parse().map_err(|e| format!("Type override \"{}\" not a mime-type: {}", s, e))
    }
}
