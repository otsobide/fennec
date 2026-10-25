use kernel::source::domain::value_objects::source_status::SourceStatus;

#[test]
fn it_parses_every_canonical_representation() {
    for raw in ["active", "inactive"] {
        let parsed = SourceStatus::from_str(raw).expect("a canonical value must parse");

        assert_eq!(parsed.as_str(), raw);
    }
}

#[test]
fn it_normalizes_case_and_whitespace() {
    let parsed = SourceStatus::from_str("  ACTIVE  ").expect("input must be normalized");

    assert_eq!(parsed.as_str(), "active");
}

#[test]
fn it_rejects_an_unknown_value() {
    let error = SourceStatus::from_str("nope").expect_err("an unknown value must be rejected");

    assert!(error.message().contains("nope"));
}

#[test]
fn it_renders_the_canonical_representation() {
    let parsed = SourceStatus::from_str("active").expect("valid value");

    assert_eq!(parsed.to_string(), "active");
}
