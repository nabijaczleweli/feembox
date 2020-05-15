use feed_rs::parser::{ParseErrorKind as ParseFeedErrorKind, ParseFeedError};
use std::io::{ErrorKind as IoErrorKind, Error as IoError};
use feembox::util::DisplayParseFeedError;


#[test]
fn parse_error_no_feed_root() {
    assert_eq!(DisplayParseFeedError(ParseFeedError::ParseError(ParseFeedErrorKind::NoFeedRoot)).to_string(),
               "couldn't parse feed: no root element");
}

#[test]
fn parse_error_unknown_mime_type() {
    assert_eq!(DisplayParseFeedError(ParseFeedError::ParseError(ParseFeedErrorKind::UnknownMimeType("text/журнализм".to_string()))).to_string(),
               "couldn't parse feed: unsupported content type text/журнализм");
}

#[test]
fn parse_error_missing_content() {
    assert_eq!(DisplayParseFeedError(ParseFeedError::ParseError(ParseFeedErrorKind::MissingContent("content.type"))).to_string(),
               "couldn't parse feed: missing content element content.type");
}

#[test]
fn io_error() {
    assert_eq!(DisplayParseFeedError(ParseFeedError::IoError(IoError::new(IoErrorKind::Other, "soz mate, can't do it"))).to_string(),
               "couldn't read feed: soz mate, can't do it");
}

// Can't create a serde_json::error::Error or a xml::reader::Error
