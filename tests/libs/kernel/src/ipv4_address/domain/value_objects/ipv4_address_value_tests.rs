use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

#[test]
fn it_keeps_a_canonical_dotted_quad() {
    let value = Ipv4AddressValue::new("89.19.209.154").expect("valid address");

    assert_eq!(value.to_string(), "89.19.209.154");
}

#[test]
fn it_trims_surrounding_whitespace() {
    let value = Ipv4AddressValue::new("  198.51.100.7 \n").expect("valid address");

    assert_eq!(value.to_string(), "198.51.100.7");
}

#[test]
fn it_drops_a_host_prefix() {
    let value = Ipv4AddressValue::new("198.51.100.7/32").expect("a /32 is a host");

    assert_eq!(value.to_string(), "198.51.100.7");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_address() {
    assert_eq!(
        Ipv4AddressValue::new("198.51.100.7/32").expect("valid address"),
        Ipv4AddressValue::new(" 198.51.100.7 ").expect("valid address")
    );
}

#[test]
fn it_rejects_a_network_prefix() {
    let error = Ipv4AddressValue::new("198.51.100.0/24").expect_err("a /24 is a network");

    assert!(error.message().contains("single host"));
}

#[test]
fn it_reports_the_longest_network_prefix_as_a_network() {
    let error = Ipv4AddressValue::new("198.51.100.6/31").expect_err("a /31 is a network");

    assert!(error.message().contains("got the /31 network"));
}

#[test]
fn it_rejects_malformed_prefixes() {
    for prefix in ["", "33", "032", "+32", " 32", "x", "32/32"] {
        let error = Ipv4AddressValue::new(&format!("198.51.100.7/{prefix}"))
            .expect_err("a malformed prefix must be rejected");

        assert!(
            error
                .message()
                .contains("prefix must be a length from 0 to 32"),
            "/{prefix} gave: {}",
            error.message()
        );
    }
}

#[test]
fn it_rejects_whitespace_before_the_prefix() {
    let error = Ipv4AddressValue::new("198.51.100.7 /32").expect_err("the prefix must be attached");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_accepts_the_lowest_and_highest_addresses() {
    assert_eq!(
        Ipv4AddressValue::new("0.0.0.0")
            .expect("valid address")
            .to_string(),
        "0.0.0.0"
    );
    assert_eq!(
        Ipv4AddressValue::new("255.255.255.255")
            .expect("valid address")
            .to_string(),
        "255.255.255.255"
    );
}

#[test]
fn it_rejects_more_than_four_octets() {
    let error =
        Ipv4AddressValue::new("198.51.100.7.1").expect_err("five octets are not an address");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_a_trailing_dot() {
    let error =
        Ipv4AddressValue::new("198.51.100.7.").expect_err("a trailing dot is not an address");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_non_ascii_digits() {
    let error =
        Ipv4AddressValue::new("\u{ff11}98.51.100.7").expect_err("fullwidth digits are not octets");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_trims_only_ascii_whitespace() {
    for spaced in ["\u{a0}198.51.100.7", "198.51.100.7\u{3000}"] {
        let error = Ipv4AddressValue::new(spaced).expect_err("non-ASCII spaces are not trimmed");

        assert!(error.message().contains("four decimal octets"));
    }
}

#[test]
fn it_rejects_octets_with_leading_zeros() {
    let error = Ipv4AddressValue::new("198.051.100.7").expect_err("leading zeros are ambiguous");

    assert!(error.message().contains("without leading zeros"));
}

#[test]
fn it_rejects_fewer_than_four_octets() {
    let error = Ipv4AddressValue::new("198.51.100").expect_err("three octets are not an address");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_octets_out_of_range() {
    let error = Ipv4AddressValue::new("256.51.100.7").expect_err("256 is not an octet");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_an_ipv6_address() {
    let error = Ipv4AddressValue::new("2001:db8::1").expect_err("IPv6 is not IPv4");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_a_defanged_address() {
    let error =
        Ipv4AddressValue::new("198.51.100[.]7").expect_err("defanged values are not refanged");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_rejects_an_empty_value() {
    let error = Ipv4AddressValue::new("   ").expect_err("an empty value is not an address");

    assert!(error.message().contains("four decimal octets"));
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let value = Ipv4AddressValue::new("198.51.100.7/32").expect("valid address");

    assert_eq!(
        value.contributing_properties().to_canonical_string(),
        r#"{"value":"198.51.100.7"}"#
    );
}

#[test]
fn it_classifies_special_ranges() {
    assert!(Ipv4AddressValue::new("10.1.2.3")
        .expect("valid address")
        .is_private());
    assert!(Ipv4AddressValue::new("127.0.0.1")
        .expect("valid address")
        .is_loopback());
    assert!(Ipv4AddressValue::new("198.51.100.7")
        .expect("valid address")
        .is_documentation());
    assert!(!Ipv4AddressValue::new("89.19.209.154")
        .expect("valid address")
        .is_private());
}
