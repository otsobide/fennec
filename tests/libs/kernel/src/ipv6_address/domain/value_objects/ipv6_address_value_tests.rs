use std::net::Ipv4Addr;

use kernel::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

fn canonical(value: &str) -> String {
    Ipv6AddressValue::new(value)
        .expect("valid address")
        .to_string()
}

fn rejection(value: &str) -> String {
    Ipv6AddressValue::new(value)
        .expect_err("the address must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_address() {
    assert_eq!(
        canonical("2001:db8:85a3::8a2e:370:7334"),
        "2001:db8:85a3::8a2e:370:7334"
    );
}

#[test]
fn it_lowercases_and_drops_leading_zeros() {
    assert_eq!(
        canonical("2001:0DB8:85A3:0000:0000:8A2E:0370:7334"),
        "2001:db8:85a3::8a2e:370:7334"
    );
}

#[test]
fn it_compresses_the_longest_run_of_zero_groups() {
    assert_eq!(canonical("2001:db8:0:0:1:0:0:0"), "2001:db8:0:0:1::");
    assert_eq!(canonical("2001:0:0:1:0:0:0:1"), "2001:0:0:1::1");
}

#[test]
fn it_compresses_the_leftmost_run_on_a_tie() {
    assert_eq!(canonical("2001:db8:0:0:1:0:0:1"), "2001:db8::1:0:0:1");
}

#[test]
fn it_does_not_compress_a_single_zero_group() {
    assert_eq!(canonical("2001:db8:0:1:1:1:1:1"), "2001:db8:0:1:1:1:1:1");
}

#[test]
fn it_expands_an_overcompressed_spelling() {
    assert_eq!(canonical("2001:db8::1:1:1:1:1"), "2001:db8:0:1:1:1:1:1");
}

#[test]
fn it_writes_ipv4_mapped_addresses_with_a_dotted_tail() {
    assert_eq!(canonical("::FFFF:c000:0201"), "::ffff:192.0.2.1");
    assert_eq!(canonical("0:0:0:0:0:ffff:192.0.2.1"), "::ffff:192.0.2.1");
}

#[test]
fn it_writes_other_dotted_tails_in_hexadecimal() {
    assert_eq!(canonical("64:ff9b::192.0.2.1"), "64:ff9b::c000:201");
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(canonical("  2001:db8::7 \n"), "2001:db8::7");
}

#[test]
fn it_trims_only_ascii_whitespace() {
    assert!(rejection("\u{a0}2001:db8::7").contains("eight groups"));
}

#[test]
fn it_drops_a_host_prefix() {
    assert_eq!(canonical("2001:DB8:0:0:0:0:0:7/128"), "2001:db8::7");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_address() {
    assert_eq!(
        Ipv6AddressValue::new("2001:DB8:0:0:0:0:0:7/128").expect("valid address"),
        Ipv6AddressValue::new(" 2001:db8::7 ").expect("valid address")
    );
}

#[test]
fn it_accepts_the_lowest_and_highest_addresses() {
    assert_eq!(canonical("::"), "::");
    assert_eq!(
        canonical("FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF"),
        "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
    );
}

#[test]
fn it_rejects_a_network_prefix() {
    assert!(rejection("2001:db8::/32").contains("single host"));
}

#[test]
fn it_reports_the_longest_network_prefix_as_a_network() {
    assert!(rejection("2001:db8::7/127").contains("/127 network"));
}

#[test]
fn it_rejects_malformed_prefixes() {
    for prefix in ["", "129", "0128", "+128", "12a", "1 28"] {
        let message = rejection(&format!("2001:db8::7/{prefix}"));
        assert!(
            message.contains("prefix must be a length"),
            "/{prefix:?} gave: {message}"
        );
    }
}

#[test]
fn it_rejects_a_zone_index() {
    assert!(rejection("fe80::1%eth0").contains("zone index"));
    assert!(rejection("fe80::1%25eth0").contains("zone index"));
}

#[test]
fn it_rejects_brackets() {
    assert!(rejection("[2001:db8::7]").contains("eight groups"));
}

#[test]
fn it_rejects_groups_of_more_than_four_digits() {
    assert!(rejection("2001:00db8::7").contains("eight groups"));
}

#[test]
fn it_rejects_too_many_or_too_few_groups() {
    assert!(rejection("1:2:3:4:5:6:7:8:9").contains("eight groups"));
    assert!(rejection("1:2:3:4:5:6:7").contains("eight groups"));
}

#[test]
fn it_rejects_more_than_one_compression() {
    assert!(rejection("2001::db8::7").contains("eight groups"));
}

#[test]
fn it_rejects_non_hexadecimal_digits() {
    assert!(rejection("2001:db8::g").contains("eight groups"));
}

#[test]
fn it_rejects_an_ipv4_address() {
    assert!(rejection("192.0.2.1").contains("eight groups"));
}

#[test]
fn it_rejects_a_defanged_address() {
    assert!(rejection("2001[:]db8::7").contains("eight groups"));
}

#[test]
fn it_rejects_an_empty_value() {
    assert!(rejection("  ").contains("eight groups"));
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let value = Ipv6AddressValue::new("2001:0DB8::7/128").expect("valid address");

    assert_eq!(
        value.contributing_properties().to_canonical_string(),
        r#"{"value":"2001:db8::7"}"#
    );
}

#[test]
fn it_classifies_special_ranges() {
    let address = |value: &str| Ipv6AddressValue::new(value).expect("valid address");

    assert!(address("::1").is_loopback());
    assert!(!address("::2").is_loopback());
    assert!(address("2001:db8::7").is_documentation());
    assert!(!address("2001:db9::7").is_documentation());
    assert!(address("fd12:3456::1").is_unique_local());
    assert!(address("fc00::1").is_unique_local());
    assert!(!address("fe00::1").is_unique_local());
    assert!(address("fe80::1").is_link_local());
    assert!(address("febf::1").is_link_local());
    assert!(!address("fec0::1").is_link_local());
}

#[test]
fn it_exposes_the_ipv4_address_it_maps() {
    let address = |value: &str| Ipv6AddressValue::new(value).expect("valid address");

    assert_eq!(
        address("::ffff:192.0.2.1").ipv4_mapped(),
        Some(Ipv4Addr::new(192, 0, 2, 1))
    );
    assert_eq!(address("2001:db8::7").ipv4_mapped(), None);
}
