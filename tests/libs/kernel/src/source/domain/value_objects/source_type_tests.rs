use kernel::source::domain::value_objects::source_type::SourceType;

#[test]
fn it_parses_every_canonical_representation() {
    for raw in ["url"] {
        let parsed = SourceType::from_str(raw).expect("a canonical value must parse");

        assert_eq!(parsed.as_str(), raw);
    }
}

#[test]
fn it_normalizes_case_and_whitespace() {
    let parsed = SourceType::from_str("  URL  ").expect("input must be normalized");

    assert_eq!(parsed.as_str(), "url");
}

#[test]
fn it_rejects_an_unknown_value() {
    let error = SourceType::from_str("nope").expect_err("an unknown value must be rejected");

    assert!(error.message().contains("nope"));
}

#[test]
fn it_renders_the_canonical_representation() {
    let parsed = SourceType::from_str("url").expect("valid value");

    assert_eq!(parsed.to_string(), "url");
}
