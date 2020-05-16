use feembox::util::MESSAGE_ID_HEADER;

mod display_parse_feed_error;


#[test]
fn message_id_header() {
    assert!("Message-ID" == MESSAGE_ID_HEADER);
    assert!("message-id" == MESSAGE_ID_HEADER);
    assert!("MESSAGE-ID" == MESSAGE_ID_HEADER);
}
