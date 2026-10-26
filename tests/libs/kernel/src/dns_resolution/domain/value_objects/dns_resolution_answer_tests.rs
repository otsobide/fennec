use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;

fn ipv4(value: &str) -> String {
    DnsResolutionAnswer::ipv4_address(value)
        .expect("valid ipv4 address")
        .to_string()
}

fn ipv4_rejection(value: &str) -> String {
    DnsResolutionAnswer::ipv4_address(value)
        .expect_err("the ipv4 address must be rejected")
        .message()
        .to_string()
}

fn ipv6(value: &str) -> String {
    DnsResolutionAnswer::ipv6_address(value)
        .expect("valid ipv6 address")
        .to_string()
}

fn ipv6_rejection(value: &str) -> String {
    DnsResolutionAnswer::ipv6_address(value)
        .expect_err("the ipv6 address must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_parses_an_ipv4_address_as_an_ipv4_answer() {
    assert_eq!(
        DnsResolutionAnswer::ipv4_address("93.184.216.34").expect("valid ipv4 address"),
        DnsResolutionAnswer::Ipv4Address(Ipv4Addr::new(93, 184, 216, 34))
    );
}

#[test]
fn it_trims_surrounding_ascii_whitespace_of_an_ipv4_address() {
    assert_eq!(ipv4("  198.51.100.7 \t\n"), "198.51.100.7");
}

#[test]
fn it_trims_only_ascii_whitespace_of_an_ipv4_address() {
    for spaced in ["\u{a0}198.51.100.7", "198.51.100.7\u{3000}"] {
        assert_eq!(
            ipv4_rejection(spaced),
            format!("ipv4 address must be four decimal octets without leading zeros, got {spaced}")
        );
    }
}

#[test]
fn it_drops_an_ipv4_host_prefix() {
    assert_eq!(ipv4("198.51.100.7/32"), "198.51.100.7");
    assert_eq!(ipv4(" 198.51.100.7/32 "), "198.51.100.7");
}

#[test]
fn it_rejects_the_longest_ipv4_network_prefix() {
    assert_eq!(
        ipv4_rejection("198.51.100.6/31"),
        "ipv4 address must be a single host, got the /31 network 198.51.100.6/31"
    );
}

#[test]
fn it_rejects_the_shortest_ipv4_network_prefix() {
    assert_eq!(
        ipv4_rejection("0.0.0.0/0"),
        "ipv4 address must be a single host, got the /0 network 0.0.0.0/0"
    );
}

#[test]
fn it_rejects_malformed_ipv4_prefixes() {
    for prefix in ["", "33", "032", "+32", " 32", "x", "32/32", "256"] {
        assert_eq!(
            ipv4_rejection(&format!("198.51.100.7/{prefix}")),
            format!(
                "ipv4 address prefix must be a length from 0 to 32 without leading zeros, got /{prefix}"
            )
        );
    }
}

#[test]
fn it_rejects_whitespace_before_an_ipv4_prefix() {
    assert_eq!(
        ipv4_rejection("198.51.100.7 /32"),
        "ipv4 address must be four decimal octets without leading zeros, got 198.51.100.7 /32"
    );
}

#[test]
fn it_rejects_ipv4_octets_with_leading_zeros() {
    for padded in ["198.051.100.7", "093.184.216.34", "198.51.100.07"] {
        assert_eq!(
            ipv4_rejection(padded),
            format!("ipv4 address must be four decimal octets without leading zeros, got {padded}")
        );
    }
}

#[test]
fn it_accepts_a_zero_octet() {
    assert_eq!(ipv4("198.0.100.7"), "198.0.100.7");
}

#[test]
fn it_accepts_the_lowest_and_highest_ipv4_addresses() {
    assert_eq!(ipv4("0.0.0.0"), "0.0.0.0");
    assert_eq!(ipv4("255.255.255.255"), "255.255.255.255");
}

#[test]
fn it_rejects_an_ipv4_octet_one_over_the_highest() {
    assert!(ipv4_rejection("256.255.255.255").contains("four decimal octets"));
    assert!(ipv4_rejection("255.255.255.256").contains("four decimal octets"));
}

#[test]
fn it_rejects_ipv4_addresses_of_other_shapes() {
    for invalid in [
        "198.51.100",
        "198.51.100.7.1",
        "198.51.100.7.",
        "198.51.100[.]7",
        "\u{ff11}98.51.100.7",
        "2001:db8::1",
        "example.com",
        "",
        "   ",
    ] {
        assert_eq!(
            ipv4_rejection(invalid),
            format!(
                "ipv4 address must be four decimal octets without leading zeros, got {invalid}"
            ),
            "{invalid:?}"
        );
    }
}

#[test]
fn it_parses_an_ipv6_address_as_an_ipv6_answer() {
    assert_eq!(
        DnsResolutionAnswer::ipv6_address("2001:db8::7").expect("valid ipv6 address"),
        DnsResolutionAnswer::Ipv6Address(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 7))
    );
}

#[test]
fn it_writes_an_ipv6_address_in_rfc_5952_form() {
    assert_eq!(
        ipv6("2001:0DB8:0000:0000:0000:0000:0000:0007"),
        "2001:db8::7"
    );
    assert_eq!(ipv6("::FFFF:c000:0201"), "::ffff:192.0.2.1");
    assert_eq!(ipv6("64:ff9b::192.0.2.1"), "64:ff9b::c000:201");
}

#[test]
fn it_trims_surrounding_ascii_whitespace_of_an_ipv6_address() {
    assert_eq!(ipv6("  2001:db8::7 \n"), "2001:db8::7");
}

#[test]
fn it_trims_only_ascii_whitespace_of_an_ipv6_address() {
    assert!(ipv6_rejection("\u{a0}2001:db8::7").contains("eight groups"));
}

#[test]
fn it_drops_an_ipv6_host_prefix() {
    assert_eq!(ipv6("2001:DB8:0:0:0:0:0:7/128"), "2001:db8::7");
}

#[test]
fn it_rejects_the_longest_ipv6_network_prefix() {
    assert_eq!(
        ipv6_rejection("2001:db8::7/127"),
        "ipv6 address must be a single host, got the /127 network 2001:db8::7/127"
    );
}

#[test]
fn it_rejects_the_shortest_ipv6_network_prefix() {
    assert_eq!(
        ipv6_rejection("::/0"),
        "ipv6 address must be a single host, got the /0 network ::/0"
    );
}

#[test]
fn it_rejects_malformed_ipv6_prefixes() {
    for prefix in ["", "129", "0128", "+128", "12a", "1 28", "128/128", "256"] {
        assert_eq!(
            ipv6_rejection(&format!("2001:db8::7/{prefix}")),
            format!(
                "ipv6 address prefix must be a length from 0 to 128 without leading zeros, got /{prefix}"
            )
        );
    }
}

#[test]
fn it_rejects_a_zone_index() {
    for zoned in ["fe80::1%eth0", "fe80::1%25eth0", "fe80::1%eth0/128"] {
        assert_eq!(
            ipv6_rejection(zoned),
            format!("ipv6 address must not carry a zone index, got {zoned}")
        );
    }
}

#[test]
fn it_accepts_the_lowest_and_highest_ipv6_addresses() {
    assert_eq!(ipv6("::"), "::");
    assert_eq!(
        ipv6("FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF"),
        "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
    );
}

#[test]
fn it_rejects_ipv6_addresses_of_other_shapes() {
    for invalid in [
        "[2001:db8::7]",
        "2001:00db8::7",
        "1:2:3:4:5:6:7:8:9",
        "1:2:3:4:5:6:7",
        "2001::db8::7",
        "2001:db8::g",
        "2001[:]db8::7",
        "93.184.216.34",
        "",
        "  ",
    ] {
        assert_eq!(
            ipv6_rejection(invalid),
            format!(
                "ipv6 address must be eight groups of at most four hexadecimal digits, got {invalid}"
            ),
            "{invalid:?}"
        );
    }
}

#[test]
fn it_keeps_an_ipv4_mapped_address_as_an_ipv6_answer() {
    let answer = DnsResolutionAnswerMother::ipv6("::ffff:93.184.216.34");

    assert_eq!(answer.kind(), "ipv6_address");
    assert_ne!(answer, DnsResolutionAnswerMother::ipv4("93.184.216.34"));
}

#[test]
fn it_wraps_an_ipv4_address_read_back_from_a_store() {
    let address = Ipv4Addr::new(93, 184, 216, 34);

    let answer = DnsResolutionAnswer::from_ip(IpAddr::V4(address));

    assert_eq!(answer, DnsResolutionAnswer::Ipv4Address(address));
    assert_eq!(answer.ip(), IpAddr::V4(address));
}

#[test]
fn it_wraps_an_ipv6_address_read_back_from_a_store() {
    let address = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 7);

    let answer = DnsResolutionAnswer::from_ip(IpAddr::V6(address));

    assert_eq!(answer, DnsResolutionAnswer::Ipv6Address(address));
    assert_eq!(answer.ip(), IpAddr::V6(address));
}

#[test]
fn it_keeps_the_family_of_an_ipv4_mapped_address_read_back_from_a_store() {
    let mapped = Ipv4Addr::new(93, 184, 216, 34).to_ipv6_mapped();

    assert_eq!(
        DnsResolutionAnswer::from_ip(IpAddr::V6(mapped)),
        DnsResolutionAnswer::Ipv6Address(mapped)
    );
}

#[test]
fn it_round_trips_through_its_address() {
    let answer = DnsResolutionAnswerMother::random();

    assert_eq!(DnsResolutionAnswer::from_ip(answer.ip()), answer);
}

#[test]
fn it_names_the_module_of_each_kind() {
    assert_eq!(
        DnsResolutionAnswerMother::ipv4("93.184.216.34").kind(),
        "ipv4_address"
    );
    assert_eq!(
        DnsResolutionAnswerMother::ipv6("2001:db8::7").kind(),
        "ipv6_address"
    );
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_address() {
    assert_eq!(
        DnsResolutionAnswerMother::ipv4("198.51.100.7/32"),
        DnsResolutionAnswerMother::ipv4(" 198.51.100.7 ")
    );
    assert_eq!(
        DnsResolutionAnswerMother::ipv6("2001:DB8:0:0:0:0:0:7/128"),
        DnsResolutionAnswerMother::ipv6(" 2001:db8::7 ")
    );
}

#[test]
fn it_derives_the_id_of_the_ipv4_address_module() {
    assert_eq!(
        DnsResolutionAnswerMother::ipv4("93.184.216.34")
            .id()
            .to_string(),
        "e9778aa6-5cd6-5a44-a746-7d375d1db1e0"
    );
    assert_eq!(
        DnsResolutionAnswerMother::ipv4("89.19.209.154")
            .id()
            .to_string(),
        "abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
}

#[test]
fn it_derives_the_id_of_the_ipv6_address_module() {
    assert_eq!(
        DnsResolutionAnswerMother::ipv6("2001:DB8::7/128")
            .id()
            .to_string(),
        "9ed76cd5-ab3b-55a5-b6ec-b55c3efb0be2"
    );
    assert_eq!(
        DnsResolutionAnswerMother::ipv6("2001:db8:85a3::8a2e:370:7334")
            .id()
            .to_string(),
        "f5344921-72d2-5619-945d-8757bca4b7cd"
    );
}

#[test]
fn it_exposes_its_kind_qualified_reference() {
    assert_eq!(
        DnsResolutionAnswerMother::ipv4("93.184.216.34/32")
            .reference()
            .to_canonical_string(),
        r#"{"id":"e9778aa6-5cd6-5a44-a746-7d375d1db1e0","kind":"ipv4_address"}"#
    );
    assert_eq!(
        DnsResolutionAnswerMother::ipv6("2001:db8::7")
            .reference()
            .to_canonical_string(),
        r#"{"id":"9ed76cd5-ab3b-55a5-b6ec-b55c3efb0be2","kind":"ipv6_address"}"#
    );
}
