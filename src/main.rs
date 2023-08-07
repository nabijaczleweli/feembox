extern crate mail_internals;
extern crate mail_headers;
extern crate mail_core;
extern crate mailparse;
extern crate tabwriter;
extern crate hostname;
extern crate feembox;
extern crate feed_rs;
extern crate futures;
extern crate memmap2;
#[macro_use]
extern crate clap;

use mailparse::{parse_header as parse_mail_header, msgidparse as parse_message_ids};
use mail_core::default_impl::simple_context as simple_mail_context;
use mail_headers::header_components::Domain;
use feed_rs::parser::parse as parse_feed;
use feed_rs::model::Entry as FeedEntry;
use std::process::{exit, id as pid};
use std::io::{Write, stdout, stdin};
use std::collections::BTreeMap;
use mail_internals::MailType;
use futures::future::Future;
use std::time::SystemTime;
use std::fs::{self, File};
use tabwriter::TabWriter;
use std::borrow::Cow;
use memmap2::Mmap;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = feembox::options::Options::parse();

    let feed = match opts.feed.1.as_ref() {
            Some(feed_path) => {
                parse_feed(File::open(feed_path).map_err(|e| {
                        eprintln!("Opening {} failed: {}", opts.feed.0, e);
                        2
                    })?)
            }
            None => parse_feed(stdin()),
        }.map_err(|e| {
            eprintln!("Reading {} failed: {}", opts.feed.0, e);
            3
        })?;

    if opts.verbosity >= feembox::options::Verbosity::Human {
        println!("{}: feed ID {}, title {:?}, updated {}",
                 opts.feed.0,
                 feed.id,
                 feed.title.as_ref().map(|t| &t.content[..]).unwrap_or(""),
                 feed.updated.map(|d| d.to_rfc3339().into()).unwrap_or(Cow::from("N/A")));
        println!("{} entries{}", feed.entries.len(), if feed.entries.is_empty() { '.' } else { ':' });

        if !feed.entries.is_empty() {
            let mut entries = TabWriter::new(stdout());
            writeln!(entries, "\tentry ID\ttitle\tupdated\tpublished").expect("stdout");
            for entry in &feed.entries {
                writeln!(entries,
                         "\t{}\t{:?}\t{}\t{}",
                         entry.id,
                         entry.title.as_ref().map(|t| &t.content[..]).unwrap_or(""),
                         entry.updated.map(|d| d.to_rfc3339().into()).unwrap_or(Cow::from("N/A")),
                         entry.published.map(|d| d.to_rfc3339().into()).unwrap_or(Cow::from("N/A")))
                    .expect("stdout");
            }
            entries.flush().expect("stdout");
        }
        println!();
    }


    let mut entries_ids: BTreeMap<String, &FeedEntry> = feed.entries.iter().map(|ent| (feembox::util::message_id_for_feed_entry(&feed, ent), ent)).collect();


    for dir_name in &["cur", "new"] {
        let dir_path = match opts.maildir.1.join(dir_name).canonicalize().ok().filter(|dp| dp.is_dir()) {
            Some(dp) => dp,
            None => continue,
        };

        let dir_name = format!("{}{}/", opts.maildir.0, dir_name);

        for mail in dir_path.read_dir()
            .map_err(|e| {
                eprintln!("Reading {} failed: {}", dir_name, e);
                4
            })? {
            let mail = mail.map_err(|e| {
                    eprintln!("Reading entry in {} failed: {}", dir_name, e);
                    4
                })?;

            let mail_path = mail.path();
            let mail_file = File::open(&mail_path).map_err(|e| {
                    eprintln!("Opening {}{} failed: {}", dir_name, mail.file_name().to_string_lossy(), e);
                    5
                })?;
            let mail_map = unsafe { Mmap::map(&mail_file) }.map_err(|e| {
                    eprintln!("Mapping {}{} failed: {}", dir_name, mail.file_name().to_string_lossy(), e);
                    5
                })?;

            let mut curs = &mail_map[..];
            while let Ok((header, next)) = parse_mail_header(curs) {
                if &header.get_key_ref()[..] == feembox::util::MESSAGE_ID_HEADER {
                    if let Ok(ids) = parse_message_ids(&header.get_value()) {
                        for id in &*ids {
                            if entries_ids.remove(id).is_some() && opts.verbosity >= feembox::options::Verbosity::Debug {
                                println!("{}{}: length {}", dir_name, mail.file_name().to_string_lossy(), mail_map.len());
                                print!("  ");
                                for (i, id) in ids.iter().enumerate() {
                                    if i != 0 {
                                        print!(", ");
                                    }
                                    print!("{}", id);
                                }
                                println!();
                            }
                        }
                    }
                }
                curs = &curs[next..];
            }
        }
    }


    // We don't write to cur, but NeoMutt needs it to recognise the directory as a mailbox
    for dir_name in &["tmp", "new", "cur"] {
        fs::create_dir_all(opts.maildir.1.join(dir_name)).map_err(|e| {
                eprintln!("Creating {}{}/ failed: {}", opts.maildir.0, dir_name, e);
                6
            })?;
    }


    let hostname = hostname::get().as_ref().map(|h| h.to_string_lossy().replace('/', "\\057").replace(':', "\\072").into()).unwrap_or(Cow::from(crate_name!()));
    let pid = pid();

    let mail_ctx = simple_mail_context::new(Domain::from_unchecked(format!("P{}.{}", pid, hostname)), feed.id.clone().parse().unwrap()).map_err(|e| {
            eprintln!("Creating mail context: {}", e);
            7
        })?;
    for (i, (message_id, entry)) in entries_ids.into_iter().enumerate() {
        let mail_data = feembox::assemble_mail(&feed,
                                               entry,
                                               message_id,
                                               &opts.alternatives_transformations,
                                               opts.mime_override.as_ref(),
                                               &mail_ctx).map_err(|e| {
                eprintln!("Assembling mail for entry {}: {}", entry.id, e);
                7
            })?
            .into_encodable_mail(mail_ctx.clone())
            .wait()
            .map_err(|e| {
                eprintln!("Constructing mail for entry {}: {}", entry.id, e);
                7
            })?
            .encode_into_bytes(MailType::Internationalized)
            .map_err(|e| {
                eprintln!("Encoding mail for entry {}: {}", entry.id, e);
                7
            })?;

        let timeofday = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("now < epoch");
        let mail_name = format!("{}.M{}P{}Q{}.{}", timeofday.as_secs(), timeofday.subsec_micros(), pid, i, hostname);

        let tmp_path = opts.maildir.1.join("tmp").join(&mail_name);
        File::create(&tmp_path).map_err(|e| {
                eprintln!("Creating {} in {}tmp/{}: {}", entry.id, opts.maildir.0, mail_name, e);
                8
            })?
            .write_all(&mail_data)
            .map_err(|e| {
                eprintln!("Writing {} in {}tmp/{}: {}", entry.id, opts.maildir.0, mail_name, e);
                let _ = fs::remove_file(&tmp_path);
                8
            })?;

        if opts.verbosity >= feembox::options::Verbosity::Human {
            println!("Delivering {} to {}new/{}", entry.id, opts.maildir.0, mail_name);
        }

        fs::rename(tmp_path, opts.maildir.1.join("new").join(&mail_name)).map_err(|e| {
                eprintln!("Delivering {} from tmp/ to {}new/{}: {}", entry.id, opts.maildir.0, mail_name, e);
                8
            })?;
    }

    Ok(())
}
