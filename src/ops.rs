//! Usually this is a big module with the beefy functions and types and whatever, but it's just assemble_mail() today


use mail_headers::header_components::{Unstructured, MediaType, MessageId as MessageIdContent, DateTime as DateTimeContent};
use mail_core::{Metadata as MailMetadata, Resource as MailResource, Data as MailData, Mail};
use feed_rs::model::{Entry as FeedEntry, Feed, Link as FeedLink};
use self::super::util::{DisplayFeedPerson, LINK_REL_FILTER};
use mail_headers::headers::{MessageId, Subject, Date};
use mail_core::context::Context as MailContext;
use mail_headers::{HeaderKind, Header};
use linked_hash_set::LinkedHashSet;
use chrono::format as date_format;
use std::borrow::Cow;
use std::io::Write;
use mime::Mime;
use std::fmt;


def_headers! {
    test_name: def_headers,
    scope: self,

    RawFrom, unchecked { "From" }, Unstructured, maxOne, None
}


pub fn assemble_mail<Mc: MailContext>(feed: &Feed, entry: &FeedEntry, message_id: String, ctx: &Mc) -> Mail {
    let authors = entry.authors.iter().chain(&feed.authors).map(DisplayFeedPerson).collect();
    let mut mail = Mail::plain_text(EntryContextLineWriter {
                                            feed: feed,
                                            entry: entry,
                                            authors: &authors,
                                        }
                                        .to_string(),
                                    ctx)
        // TODO: choose one over the other? wq and kdist have just summary; lore.kernel.org has just content
        .wrap_with_related((entry.summary.iter().map(|summary| summary_to_mail(&summary.content, &summary.content_type, ctx)))
            .chain(entry.content.as_ref()
                .and_then(|content| content.body.as_ref().map(Cow::from)
                    .or_else(|| content.src.as_ref().map(|l| format!("Links: {}", LinkWriter(l)).into())).map(|b| (b, &content.content_type)))
                .iter().map(|(body, content_type)| summary_to_mail(body, content_type, ctx)))
            .collect());


    let subject = entry.title.as_ref().map(|t| t.content.clone()).unwrap_or_else(|| entry.id.clone());
    let date = entry.updated.as_ref().or(entry.published.as_ref()).or(feed.updated.as_ref()).or(feed.published.as_ref());
    let from = if authors.is_empty() {
        match feed.title.as_ref().or(feed.description.as_ref()) {
            Some(title) => format!("{} <>", title.content).into(),
            None => Cow::from(&feed.id),
        }
    } else {
        let mut from = vec![];
        for (i, author) in authors.iter().enumerate() {
            if i != 0 {
                from.extend(b", ");
            }
            write!(from, "{}", author).expect("from format");
        }
        String::from_utf8(from).expect("from").into()
    };

    mail.headers_mut().insert::<MessageId>(Header::new(MessageIdContent::from_unchecked(message_id)));
    mail.headers_mut().insert::<Subject>(HeaderKind::auto_body(subject).unwrap());
    if let Some(date) = date {
        mail.headers_mut().insert::<Date>(Header::new(DateTimeContent::new(*date)));
    }
    mail.headers_mut().insert::<RawFrom>(HeaderKind::auto_body(&from[..]).unwrap());
    mail
}

fn summary_to_mail<Mc: MailContext>(content: &str, content_type: &Mime, ctx: &Mc) -> Mail {
    let media_type = MediaType::new_with_params(content_type.type_().as_str(),
                                                match content_type.suffix() {
                                                    Some(suff) => format!("{}+{}", content_type.subtype().as_str(), suff.as_str()).into(),
                                                    None => Cow::from(content_type.subtype().as_str()),
                                                },
                                                content_type.params())
        .expect("media_type parse");

    Mail::new_singlepart_mail(MailResource::Data(MailData::new(content.as_bytes(), // Mostly an inlining of MailResource::plain_text()
                                                               MailMetadata {
                                                                   file_meta: Default::default(),
                                                                   media_type: media_type,
                                                                   content_id: ctx.generate_content_id(),
                                                               })))
}


struct EntryContextLineWriter<'f, 'e, 'a> {
    feed: &'f Feed,
    entry: &'e FeedEntry,
    authors: &'a LinkedHashSet<DisplayFeedPerson<'e>>,
}

impl fmt::Display for EntryContextLineWriter<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        static FORMAT_RFC2822: &[date_format::Item] = &[date_format::Item::Fixed(date_format::Fixed::RFC2822)];


        for (i, author) in self.authors.iter().enumerate() {
            match i {
                0 => f.write_str("By ")?,
                i if i != self.authors.len() => f.write_str(", ")?,
                _ => {}
            }
            write!(f, "{}", author)?;
        }
        if !self.authors.is_empty() {
            f.write_str("\n")?;
        }


        let mut dates: Vec<_> = self.entry.published.iter().chain(&self.entry.updated).collect();
        dates.dedup();
        match &dates[..] {
            [] => f.write_str("No publication date\n")?,
            [p] => write!(f, "Published on {}\n", p.format_with_items(FORMAT_RFC2822.iter()))?,
            [p, u] => {
                write!(f,
                       "Published on {}, updated on {}\n",
                       p.format_with_items(FORMAT_RFC2822.iter()),
                       u.format_with_items(FORMAT_RFC2822.iter()))?
            }
            _ => unreachable!(),
        }


        // println!("{:#?} {:#?}", self.entry.links, self.feed.links);
        let mut links: Vec<_> = self.entry
            .links
            .iter()
            .chain(&self.feed.links)
            .filter(|l| l.rel.as_ref().map(|r| !LINK_REL_FILTER.iter().find(|f| &&r[..] == *f).is_some()).unwrap_or(true))
            .collect();
        links.dedup_by_key(|l| &l.href); // TODO: unideal for sortedness reason; would need Eq+Hash wrapper
        links.dedup_by(|l, r| // kernel.org/kdist.xml has both "kernel.org/" and "kernel.org"
            l.href[..l.href.len() - l.href.ends_with('/') as usize] == r.href[..r.href.len() - r.href.ends_with('/') as usize]);
        for (i, link) in links.iter().enumerate() {
            match i {
                0 => f.write_str("Links to ")?,
                i if i != links.len() => f.write_str(", ")?,
                _ => {}
            }
            LinkWriter(link).fmt(f)?;
        }
        if !links.is_empty() {
            f.write_str("\n")?;
        }


        Ok(())
    }
}


struct LinkWriter<'l>(&'l FeedLink);

impl fmt::Display for LinkWriter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.0.title {
            write!(f, "{} <{}>", title, self.0.href)?;
        } else {
            f.write_str(&self.0.href)?;
        }

        Ok(())
    }
}
