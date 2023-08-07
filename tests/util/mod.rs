use feembox::util::{self, MESSAGE_ID_HEADER, LINK_REL_FILTER};
use feed_rs::parser::parse as parse_feed;
use std::collections::BTreeSet;
use std::fs;

mod display_feed_person;


static KDIST_XML: &[u8] = include_bytes!("../../test-data/kdist.xml");
static KDIST_ID: &str = "7edcf1fbe86570753646f6eb75db4d55";
static KDIST_ENTRY_IDS: &[&str] = &["kernel.org,mainline,5.7-rc4,2020-05-03",
                                    "kernel.org,stable,5.6.11,2020-05-06",
                                    "kernel.org,stable,5.5.19,2020-04-21",
                                    "kernel.org,longterm,5.4.39,2020-05-06",
                                    "kernel.org,longterm,4.19.121,2020-05-06",
                                    "kernel.org,longterm,4.14.179,2020-05-05",
                                    "kernel.org,longterm,4.9.222,2020-05-05",
                                    "kernel.org,longterm,4.4.222,2020-05-05",
                                    "kernel.org,longterm,3.16.83,2020-04-28",
                                    "kernel.org,linux-next,next-20200508,2020-05-08"];


#[test]
fn message_id_header() {
    assert_eq!("Message-ID", MESSAGE_ID_HEADER);
    assert_eq!("message-id", MESSAGE_ID_HEADER);
    assert_eq!("MESSAGE-ID", MESSAGE_ID_HEADER);
}

#[test]
fn link_rel_filter() {
    assert!(LINK_REL_FILTER.iter().find(|f| &"self" == *f).is_some());
    assert!(LINK_REL_FILTER.iter().find(|f| &"Self" == *f).is_some());
    assert!(LINK_REL_FILTER.iter().find(|f| &"SELF" == *f).is_some());
}

#[test]
fn message_id_for_feed_entry() {
    let kdist_feed = parse_feed(KDIST_XML).unwrap();

    for (entry, entry_id) in kdist_feed.entries.iter().zip(KDIST_ENTRY_IDS.iter()) {
        assert_eq!(util::message_id_for_feed_entry(&kdist_feed, entry), format!("{}@{}", entry_id, KDIST_ID));
    }
}


#[test]
fn message_id_for_feed_entry_feeds() {
    for mut path in fs::read_dir("test-data/feed.ids").unwrap().map(|e| e.unwrap().path()) {
        if path.extension().unwrap() != "ids" {
            continue;
        }
        let ids_s = fs::read_to_string(&path).unwrap();
        let ids = ids_s.lines().collect::<BTreeSet<_>>();

        path.set_extension("feed");
        let feed = parse_feed(&fs::read(&path).unwrap()[..]).unwrap();
        let feed_ids = feed.entries.iter().map(|entry| util::message_id_for_feed_entry(&feed, entry)).collect::<Vec<_>>();
        assert_eq!(feed_ids.iter().map(|s| s.as_str()).collect::<BTreeSet<_>>(), ids, "{}", path.display());
    }
}
