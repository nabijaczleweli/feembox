use feed_rs::model::Person as FeedPerson;
use feembox::util::DisplayFeedPerson;


#[test]
fn name_no_uri_no_email() {
    assert_eq!(DisplayFeedPerson(&FeedPerson {
                       name: "name".to_string(),
                       uri: None,
                       email: None,
                   })
                   .to_string(),
               "name");
}

#[test]
fn name_no_uri_email() {
    assert_eq!(DisplayFeedPerson(&FeedPerson {
                       name: "name".to_string(),
                       uri: None,
                       email: Some("mail@e".to_string()),
                   })
                   .to_string(),
               "name <mail@e>");
}

#[test]
fn name_uri_no_email() {
    assert_eq!(DisplayFeedPerson(&FeedPerson {
                       name: "name".to_string(),
                       uri: Some("https://nabijaczleweli.xyz".to_string()),
                       email: None,
                   })
                   .to_string(),
               "name (https://nabijaczleweli.xyz)");
}

#[test]
fn name_uri_email() {
    assert_eq!(DisplayFeedPerson(&FeedPerson {
                       name: "name".to_string(),
                       uri: Some("https://nabijaczleweli.xyz".to_string()),
                       email: Some("mail@e".to_string()),
                   })
                   .to_string(),
               "name <mail@e> (https://nabijaczleweli.xyz)");
}
