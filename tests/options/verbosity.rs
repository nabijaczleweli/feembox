use feembox::options::Verbosity;


#[test]
fn parse() {
    assert_eq!(Verbosity::from(0), Verbosity::None);
    assert_eq!(Verbosity::from(1), Verbosity::Human);
    assert_eq!(Verbosity::from(2), Verbosity::Debug);
    assert_eq!(Verbosity::from(3), Verbosity::Debug);
    assert_eq!(Verbosity::from(4), Verbosity::Debug);
    assert_eq!(Verbosity::from(5), Verbosity::Debug);
}
