//! Various utility functions.


use feed_rs::parser::{ParseErrorKind as ParseFeedErrorKind, ParseFeedError};
use unicase::Ascii;
use std::fmt;


/// Case-insensitive matcher for the Message-ID header
pub static MESSAGE_ID_HEADER: Ascii<&str> = Ascii::new("Message-ID");


/// An equivalent of a `Display` implementation for `ParseFeedError`
///
/// Submitted upstream in https://github.com/feed-rs/feed-rs/pull/42
#[derive(Debug)]
pub struct DisplayParseFeedError(pub ParseFeedError);

impl fmt::Display for DisplayParseFeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ParseFeedError::ParseError(ParseFeedErrorKind::NoFeedRoot) => f.write_str("couldn't parse feed: no root element"),
            ParseFeedError::ParseError(ParseFeedErrorKind::UnknownMimeType(mime)) => write!(f, "couldn't parse feed: unsupported content type {}", mime),
            ParseFeedError::ParseError(ParseFeedErrorKind::MissingContent(elem)) => write!(f, "couldn't parse feed: missing content element {}", elem),
            ParseFeedError::IoError(ie) => write!(f, "couldn't read feed: {}", ie),
            ParseFeedError::JsonSerde(je) => write!(f, "couldn't parse JSON: {}", je),
            ParseFeedError::XmlReader(xe) => write!(f, "couldn't parse XML: {}", xe),
        }
    }
}
