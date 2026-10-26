use std::net::Ipv4Addr;

use kernel::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

fn canonical(value: &str) -> String {
    Ipv4NetworkValue::new(value)
        .expect("valid network")
        .to_string()
}

fn rejection(value: &str) -> String {
    Ipv4NetworkValue::new(value)
        .expect_err("the network must be rejected")
        .message()
        .to_string()
}

fn network(value: &str) -> Ipv4NetworkValue {
    Ipv4NetworkValue::new(value).expect("valid network")
}

#[test]
fn it_keeps_a_canonical_block() {
    assert_eq!(canonical("198.51.100.0/24"), "198.51.100.0/24");
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(canonical("  198.51.100.0/24 \n"), "198.51.100.0/24");
}

#[test]
fn it_accepts_every_prefix_from_0_to_31() {
    assert_eq!(canonical("0.0.0.0/0"), "0.0.0.0/0");
    assert_eq!(canonical("10.0.0.0/8"), "10.0.0.0/8");
    assert_eq!(canonical("198.51.100.6/31"), "198.51.100.6/31");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_block() {
    assert_eq!(network("198.51.100.0/24"), network(" 198.51.100.0/24 "));
}

#[test]
fn it_rejects_a_value_without_a_prefix() {
    assert!(rejection("198.51.100.0").contains("must have a prefix length"));
}

#[test]
fn it_rejects_a_host_prefix() {
    assert!(rejection("198.51.100.7/32").contains("a /32 is an IPv4 address"));
}

#[test]
fn it_rejects_set_host_bits_and_names_the_network() {
    let message = rejection("198.51.100.7/24");

    assert!(message.contains("no host bits set"), "{message}");
    assert!(message.contains("198.51.100.0/24"), "{message}");
}

#[test]
fn it_rejects_set_host_bits_at_both_ends_of_the_prefix_range() {
    assert!(rejection("0.0.0.1/0").contains("no host bits set"));
    assert!(rejection("198.51.100.7/31").contains("no host bits set"));
}

#[test]
fn it_rejects_malformed_prefixes() {
    for prefix in ["", "33", "024", "+24", "2a", "2 4", "24/24"] {
        let message = rejection(&format!("198.51.100.0/{prefix}"));
        assert!(
            message.contains("prefix must be a length"),
            "/{prefix:?} gave: {message}"
        );
    }
}

#[test]
fn it_rejects_octets_with_leading_zeros() {
    assert!(rejection("198.051.100.0/24").contains("four decimal octets"));
}

#[test]
fn it_rejects_an_abbreviated_address() {
    assert!(rejection("198.51.100/24").contains("four decimal octets"));
}

#[test]
fn it_rejects_an_ipv6_network() {
    assert!(rejection("2001:db8::/24").contains("four decimal octets"));
}

#[test]
fn it_rejects_a_defanged_network() {
    assert!(rejection("198.51.100[.]0/24").contains("four decimal octets"));
}

#[test]
fn it_rejects_an_empty_value() {
    assert!(rejection("  ").contains("must have a prefix length"));
}

#[test]
fn it_exposes_its_bounds_and_size() {
    let block = network("198.51.100.0/24");

    assert_eq!(block.network_address(), Ipv4Addr::new(198, 51, 100, 0));
    assert_eq!(block.prefix_length(), 24);
    assert_eq!(block.last_address(), Ipv4Addr::new(198, 51, 100, 255));
    assert_eq!(block.size(), 256);
    assert_eq!(network("0.0.0.0/0").size(), 1 << 32);
    assert_eq!(
        network("0.0.0.0/0").last_address(),
        Ipv4Addr::new(255, 255, 255, 255)
    );
    assert_eq!(network("198.51.100.6/31").size(), 2);
}

#[test]
fn it_tells_whether_it_contains_an_address() {
    let block = network("198.51.100.0/24");

    assert!(block.contains(Ipv4Addr::new(198, 51, 100, 0)));
    assert!(block.contains(Ipv4Addr::new(198, 51, 100, 255)));
    assert!(!block.contains(Ipv4Addr::new(198, 51, 101, 0)));
    assert!(network("0.0.0.0/0").contains(Ipv4Addr::new(203, 0, 113, 9)));
}

#[test]
fn it_tells_whether_the_whole_block_is_private() {
    assert!(network("10.0.0.0/8").is_private());
    assert!(network("172.16.0.0/12").is_private());
    assert!(network("192.168.10.0/24").is_private());
    assert!(!network("172.0.0.0/8").is_private());
    assert!(!network("198.51.100.0/24").is_private());
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    assert_eq!(
        network(" 198.51.100.0/24 ")
            .contributing_properties()
            .to_canonical_string(),
        r#"{"value":"198.51.100.0/24"}"#
    );
}
