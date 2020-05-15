extern crate feembox;
extern crate feed_rs;

use feembox::util::DisplayParseFeedError;
use feed_rs::parser::parse as parse_feed;
use std::process::exit;
use std::io::stdin;
use std::fs::File;


fn main() {
    let result = actual_main().err().unwrap_or(0);
    exit(result);
}

fn actual_main() -> Result<(), i32> {
    let opts = feembox::Options::parse();
    println!("{:#?}", opts);

    let (feed_input_name, feed_path) = opts.feed;
    let feed = match feed_path {
            Some(feed_path) => {
                parse_feed(File::open(feed_path).map_err(|e| {
                        eprintln!("Opening {} failed: {}", feed_input_name, e);
                        1
                    })?)
            }
            None => parse_feed(stdin()),
        }.map_err(|e| {
            eprintln!("Reading {} failed: {}", feed_input_name, DisplayParseFeedError(e));
            2
        })?;

    if opts.verbose {
        println!("{}: read feed with ID {} ", feed_input_name, feed.id);
    }

    Ok(())
}
