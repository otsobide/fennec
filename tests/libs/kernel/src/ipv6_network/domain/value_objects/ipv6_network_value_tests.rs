use std::net::Ipv6Addr;

use kernel::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

fn canonical(value: &str) -> String {
    Ipv6NetworkValue::new(value)
        .expect("valid network")
        .to_string()
}

fn rejection(value: &str) -> String {
    Ipv6NetworkValue::new(value)
        .expect_err("the network must be rejected")
        .message()
        .to_string()
}

fn network(value: &str) -> Ipv6NetworkValue {
    Ipv6NetworkValue::new(value).expect("valid network")
}

#[test]
fn it_keeps_a_canonical_block() {
    assert_eq!(canonical("2001:db8::/32"), "2001:db8::/32");
}

#[test]
fn it_canonicalises_the_network_address() {
    assert_eq!(
        canonical("2001:0DB8:0000:0000:0000:0000:0000:0000/32"),
        "2001:db8::/32"
    );
    assert_eq!(
        canonical("2001:db8:85a3:0:0:0:0:0/48"),
        "2001:db8:85a3::/48"
    );
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(canonical("  2001:db8::/32 \n"), "2001:db8::/32");
}

#[test]
fn it_accepts_every_prefix_from_0_to_127() {
    assert_eq!(canonical("::/0"), "::/0");
    assert_eq!(canonical("fc00::/7"), "fc00::/7");
    assert_eq!(canonical("2001:db8::6/127"), "2001:db8::6/127");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_block() {
    assert_eq!(
        network("2001:0DB8::/32"),
        network(" 2001:db8:0:0:0:0:0:0/32 ")
    );
}

#[test]
fn it_rejects_a_value_without_a_prefix() {
    assert!(rejection("2001:db8::").contains("must have a prefix length"));
}

#[test]
fn it_rejects_a_host_prefix() {
    assert!(rejection("2001:db8::7/128").contains("a /128 is an IPv6 address"));
}

#[test]
fn it_rejects_set_host_bits_and_names_the_network() {
    let message = rejection("2001:db8::7/32");

    assert!(message.contains("no host bits set"), "{message}");
    assert!(message.contains("2001:db8::/32"), "{message}");
}

#[test]
fn it_rejects_set_host_bits_at_both_ends_of_the_prefix_range() {
    assert!(rejection("::1/0").contains("no host bits set"));
    assert!(rejection("2001:db8::7/127").contains("no host bits set"));
}

#[test]
fn it_rejects_malformed_prefixes() {
    for prefix in ["", "129", "032", "+32", "3a", "3 2", "32/32"] {
        let message = rejection(&format!("2001:db8::/{prefix}"));
        assert!(
            message.contains("prefix must be a length"),
            "/{prefix:?} gave: {message}"
        );
    }
}

#[test]
fn it_rejects_a_zone_index() {
    assert!(rejection("fe80::%eth0/64").contains("zone index"));
}

#[test]
fn it_rejects_brackets() {
    assert!(rejection("[2001:db8::]/32").contains("eight groups"));
}

#[test]
fn it_rejects_an_ipv4_network() {
    assert!(rejection("198.51.100.0/24").contains("eight groups"));
}

#[test]
fn it_rejects_an_empty_value() {
    assert!(rejection("  ").contains("must have a prefix length"));
}

#[test]
fn it_exposes_its_bounds() {
    let block = network("2001:db8::/32");

    assert_eq!(
        block.network_address(),
        "2001:db8::".parse::<Ipv6Addr>().unwrap()
    );
    assert_eq!(block.prefix_length(), 32);
    assert_eq!(
        block.last_address(),
        "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff"
            .parse::<Ipv6Addr>()
            .unwrap()
    );
    assert_eq!(network("::/0").last_address(), Ipv6Addr::from(u128::MAX));
}

#[test]
fn it_tells_whether_it_contains_an_address() {
    let block = network("2001:db8::/32");

    assert!(block.contains("2001:db8::7".parse().unwrap()));
    assert!(block.contains("2001:db8:ffff::1".parse().unwrap()));
    assert!(!block.contains("2001:db9::".parse().unwrap()));
    assert!(network("::/0").contains("fe80::1".parse().unwrap()));
}

#[test]
fn it_classifies_special_ranges() {
    assert!(network("2001:db8:85a3::/48").is_documentation());
    assert!(!network("2001::/16").is_documentation());
    assert!(network("fd12:3456::/32").is_unique_local());
    assert!(network("fc00::/7").is_unique_local());
    assert!(!network("fc00::/6").is_unique_local());
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    assert_eq!(
        network(" 2001:0DB8::/32 ")
            .contributing_properties()
            .to_canonical_string(),
        r#"{"value":"2001:db8::/32"}"#
    );
}
