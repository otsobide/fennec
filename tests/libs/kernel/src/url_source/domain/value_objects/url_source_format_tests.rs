use kernel::url_source::domain::value_objects::url_source_format::UrlSourceFormat;

#[test]
fn it_parses_every_canonical_representation() {
    for raw in ["plain", "csv", "json", "stix"] {
        let parsed = UrlSourceFormat::from_str(raw).expect("a canonical value must parse");

        assert_eq!(parsed.as_str(), raw);
    }
}

#[test]
fn it_normalizes_case_and_whitespace() {
    let parsed = UrlSourceFormat::from_str("  PLAIN  ").expect("input must be normalized");

    assert_eq!(parsed.as_str(), "plain");
}

#[test]
fn it_rejects_an_unknown_value() {
    let error = UrlSourceFormat::from_str("nope").expect_err("an unknown value must be rejected");

    assert!(error.message().contains("nope"));
}

#[test]
fn it_renders_the_canonical_representation() {
    let parsed = UrlSourceFormat::from_str("plain").expect("valid value");

    assert_eq!(parsed.to_string(), "plain");
}
