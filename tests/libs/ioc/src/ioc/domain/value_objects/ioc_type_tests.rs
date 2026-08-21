use ioc::ioc::domain::value_objects::ioc_type::IocType;

#[test]
fn it_parses_every_canonical_representation() {
    for raw in [
        "ipv4", "ipv6", "domain", "url", "sha256", "sha1", "md5", "email",
    ] {
        let parsed = IocType::from_str(raw).expect("a canonical value must parse");

        assert_eq!(parsed.as_str(), raw);
    }
}

#[test]
fn it_normalizes_case_and_whitespace() {
    let parsed = IocType::from_str("  IPV4  ").expect("input must be normalized");

    assert_eq!(parsed.as_str(), "ipv4");
}

#[test]
fn it_rejects_an_unknown_value() {
    let error = IocType::from_str("nope").expect_err("an unknown value must be rejected");

    assert!(error.message().contains("nope"));
}

#[test]
fn it_renders_the_canonical_representation() {
    let parsed = IocType::from_str("ipv4").expect("valid value");

    assert_eq!(parsed.to_string(), "ipv4");
}
