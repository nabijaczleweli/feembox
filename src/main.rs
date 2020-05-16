extern crate mailparse;
extern crate tabwriter;
extern crate feembox;
extern crate feed_rs;
extern crate memmap;

use mailparse::{parse_header as parse_mail_header, msgidparse as parse_message_ids};
use feembox::util::DisplayParseFeedError;
use feed_rs::parser::parse as parse_feed;
use feed_rs::model::Entry as FeedEntry;
use std::io::{Write, stdout, stdin};
use std::collections::BTreeMap;
use tabwriter::TabWriter;
use std::process::exit;
use std::borrow::Cow;
use std::fs::File;
use memmap::Mmap;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = feembox::Options::parse();
    println!("{:#?}", opts);

    let feed = match opts.feed.1.as_ref() {
            Some(feed_path) => {
                parse_feed(File::open(feed_path).map_err(|e| {
                        eprintln!("Opening {} failed: {}", opts.feed.0, e);
                        2
                    })?)
            }
            None => parse_feed(stdin()),
        }.map_err(|e| {
            eprintln!("Reading {} failed: {}", opts.feed.0, DisplayParseFeedError(e));
            3
        })?;

    if opts.verbose {
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


    let mut entries_ids: BTreeMap<String, &FeedEntry> = BTreeMap::new();


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

            if opts.verbose {
                println!("{}{}: length {}", dir_name, mail.file_name().to_string_lossy(), mail_map.len());
            }

            let mut curs = &mail_map[..];
            while let Ok((header, next)) = parse_mail_header(curs) {
                if header.get_key() == feembox::util::MESSAGE_ID_HEADER {
                    if let Ok(ids) = parse_message_ids(&header.get_value()) {
                        if opts.verbose {
                            print!("  ");
                            for (i, id) in ids.iter().enumerate() {
                                if i != 0 {
                                    print!(", ");
                                }
                                print!("{}", id);
                            }
                            println!();
                        }

                        for id in &*ids {
                            entries_ids.remove(id);
                        }
                    }
                }
                curs = &curs[next..];
            }
        }
    }

    Ok(())
}
