//! Various utility functions.


use feed_rs::model::{Person as FeedPerson, Entry as FeedEntry, Feed};
use std::hash::{Hasher, Hash};
use unicase::Ascii;
use std::fmt;


/// Case-insensitive matcher for the Message-ID header
pub static MESSAGE_ID_HEADER: Ascii<&str> = Ascii::new("Message-ID");

/// Matchers for the `rel=`s to not include in the status attachment
pub static LINK_REL_FILTER: &[Ascii<&str>] = &[Ascii::new("self")];


/// `Display` a `Person` as `name <email> (uri)`
#[derive(Debug, PartialEq)]
pub struct DisplayFeedPerson<'p>(pub &'p FeedPerson);

impl fmt::Display for DisplayFeedPerson<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.name)?;
        if let Some(email) = &self.0.email {
            f.write_str(" <")?;
            f.write_str(&email)?;
            f.write_str(">")?;
        }
        if let Some(uri) = &self.0.uri {
            f.write_str(" (")?;
            f.write_str(&uri)?;
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl Hash for DisplayFeedPerson<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
        self.0.uri.hash(state);
        self.0.email.hash(state);
    }
}

impl Eq for DisplayFeedPerson<'_> {}


/// Get the Message-ID corresponding to the specified entry in the specified feed
///
/// This amounts to `"entry.id@feed.id"`, but non-ASCII characters of either are Unicode-escaped
pub fn message_id_for_feed_entry(feed: &Feed, entry: &FeedEntry) -> String {
    fn expand(whom: &mut String, with: &str) {
        for c in with.chars() {
            if !c.is_ascii() {
                for u in c.escape_unicode() {
                    whom.push(u);
                }
            } else {
                whom.push(c);
            }
        }
    }

    let mut ret = String::with_capacity(entry.id.len() + 1 + feed.id.len());
    expand(&mut ret, &entry.id);
    ret.push('@');
    expand(&mut ret, &feed.id);
    ret
}
