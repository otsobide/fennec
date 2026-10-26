//! The `dns_resolution` module parses its endpoints without importing the
//! observable modules (`docs/data-model/links.md`). These tests keep it in
//! step with them: for every spelling, both accept or both reject it with the
//! same message, and an accepted spelling gives the same canonical value and
//! the same id in both.

use std::net::IpAddr;

use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use kernel::domain_name::domain::value_objects::domain_name_value::DomainNameValue;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use kernel::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

fn domain_name_spellings() -> Vec<String> {
    let mut spellings: Vec<String> = [
        "example.com",
        "Example.COM",
        "  example.com \t\n",
        "example.com.",
        " Example.COM. ",
        "hostelhotels.net",
        "faß.de",
        "Faß.DE.",
        "xn--fa-hia.de",
        "Bücher.example",
        "xn--bcher-kva.example",
        "_dmarc.example.com",
        "update.example-cdn.net",
        "34.216.184.93.example",
        "a.b",
        "localhost",
        "localhost.",
        " localhost ",
        "",
        "  ",
        ".",
        "  . ",
        "example.com..",
        "example..com",
        ".example.com",
        "-example.com",
        "example-.com",
        "exa mple.com",
        "example.com/path",
        "user@example.com",
        "example[.]com",
        "https://example.com",
        "xn--a.com",
        "93.184.216.34",
        "\u{a0}example.com",
        "example.com\u{3000}",
        "\u{fffd}.example",
    ]
    .iter()
    .map(|spelling| spelling.to_string())
    .collect();
    spellings.push(format!("{}.com", "a".repeat(63)));
    spellings.push(format!("{}.com", "a".repeat(64)));
    let long = |last: usize| {
        format!(
            "{}.{}.{}.{}.com",
            "a".repeat(63),
            "b".repeat(63),
            "c".repeat(63),
            "d".repeat(last)
        )
    };
    spellings.push(long(57));
    spellings.push(long(58));
    spellings
}

const IPV4_ADDRESS_SPELLINGS: &[&str] = &[
    "93.184.216.34",
    "89.19.209.154",
    "  198.51.100.7 \t\n",
    "198.51.100.7/32",
    " 198.51.100.7/32 ",
    "0.0.0.0",
    "255.255.255.255",
    "198.0.100.7",
    "198.51.100.6/31",
    "0.0.0.0/0",
    "198.51.100.0/24",
    "198.51.100.7/",
    "198.51.100.7/33",
    "198.51.100.7/032",
    "198.51.100.7/+32",
    "198.51.100.7/ 32",
    "198.51.100.7/x",
    "198.51.100.7/32/32",
    "198.51.100.7/256",
    "198.51.100.7 /32",
    "198.051.100.7",
    "093.184.216.34",
    "256.255.255.255",
    "255.255.255.256",
    "198.51.100",
    "198.51.100.7.1",
    "198.51.100.7.",
    "198.51.100[.]7",
    "\u{ff11}98.51.100.7",
    "\u{a0}198.51.100.7",
    "198.51.100.7\u{3000}",
    "2001:db8::1",
    "::ffff:93.184.216.34",
    "example.com",
    "",
    "   ",
];

const IPV6_ADDRESS_SPELLINGS: &[&str] = &[
    "2001:db8::7",
    "2001:db8:85a3::8a2e:370:7334",
    "2001:0DB8:0000:0000:0000:0000:0000:0007",
    "2001:DB8:0:0:0:0:0:7/128",
    "  2001:db8::7 \n",
    "::",
    "::1",
    "FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF",
    "::ffff:192.0.2.1",
    "::FFFF:c000:0201",
    "0:0:0:0:0:ffff:192.0.2.1",
    "::ffff:93.184.216.34",
    "64:ff9b::192.0.2.1",
    "2001:db8:0:0:1:0:0:1",
    "2001:db8::1:1:1:1:1",
    "2001:db8::7/127",
    "::/0",
    "2001:db8::/32",
    "2001:db8::7/",
    "2001:db8::7/129",
    "2001:db8::7/0128",
    "2001:db8::7/+128",
    "2001:db8::7/12a",
    "2001:db8::7/1 28",
    "2001:db8::7/128/128",
    "2001:db8::7/256",
    "fe80::1%eth0",
    "fe80::1%25eth0",
    "fe80::1%eth0/128",
    "[2001:db8::7]",
    "2001:00db8::7",
    "1:2:3:4:5:6:7:8:9",
    "1:2:3:4:5:6:7",
    "2001::db8::7",
    "2001:db8::g",
    "2001[:]db8::7",
    "\u{a0}2001:db8::7",
    "93.184.216.34",
    "example.com",
    "",
    "  ",
];

#[test]
fn the_subject_parses_domain_names_as_the_domain_name_module_does() {
    for spelling in domain_name_spellings() {
        match (
            DnsResolutionSubject::domain_name(&spelling),
            DomainNameValue::new(&spelling),
        ) {
            (Ok(subject), Ok(value)) => {
                assert_eq!(subject.value(), value.value(), "{spelling:?}");
                assert_eq!(
                    subject.id(),
                    *DomainNameId::from_value(&value).value(),
                    "{spelling:?}"
                );
            }
            (Err(subject), Err(value)) => {
                assert_eq!(subject.message(), value.message(), "{spelling:?}");
            }
            (subject, value) => panic!(
                "{spelling:?} diverges: the subject gave {subject:?}, the domain name module gave {value:?}"
            ),
        }
    }
}

#[test]
fn the_ipv4_answer_parses_addresses_as_the_ipv4_address_module_does() {
    for spelling in IPV4_ADDRESS_SPELLINGS {
        match (
            DnsResolutionAnswer::ipv4_address(spelling),
            Ipv4AddressValue::new(spelling),
        ) {
            (Ok(answer), Ok(value)) => {
                assert_eq!(answer.ip(), IpAddr::V4(*value.value()), "{spelling:?}");
                assert_eq!(answer.to_string(), value.to_string(), "{spelling:?}");
                assert_eq!(
                    answer.id(),
                    *Ipv4AddressId::from_value(&value).value(),
                    "{spelling:?}"
                );
            }
            (Err(answer), Err(value)) => {
                assert_eq!(answer.message(), value.message(), "{spelling:?}");
            }
            (answer, value) => panic!(
                "{spelling:?} diverges: the answer gave {answer:?}, the ipv4 address module gave {value:?}"
            ),
        }
    }
}

#[test]
fn the_ipv6_answer_parses_addresses_as_the_ipv6_address_module_does() {
    for spelling in IPV6_ADDRESS_SPELLINGS {
        match (
            DnsResolutionAnswer::ipv6_address(spelling),
            Ipv6AddressValue::new(spelling),
        ) {
            (Ok(answer), Ok(value)) => {
                assert_eq!(answer.ip(), IpAddr::V6(*value.value()), "{spelling:?}");
                assert_eq!(answer.to_string(), value.to_string(), "{spelling:?}");
                assert_eq!(
                    answer.id(),
                    *Ipv6AddressId::from_value(&value).value(),
                    "{spelling:?}"
                );
            }
            (Err(answer), Err(value)) => {
                assert_eq!(answer.message(), value.message(), "{spelling:?}");
            }
            (answer, value) => panic!(
                "{spelling:?} diverges: the answer gave {answer:?}, the ipv6 address module gave {value:?}"
            ),
        }
    }
}

#[test]
fn an_answer_read_back_from_a_store_keeps_the_id_of_its_module() {
    let ipv4 = Ipv4AddressValue::new("93.184.216.34").expect("valid address");
    let ipv6 = Ipv6AddressValue::new("::ffff:93.184.216.34").expect("valid address");

    assert_eq!(
        DnsResolutionAnswer::from_ip(IpAddr::V4(*ipv4.value())).id(),
        *Ipv4AddressId::from_value(&ipv4).value()
    );
    assert_eq!(
        DnsResolutionAnswer::from_ip(IpAddr::V6(*ipv6.value())).id(),
        *Ipv6AddressId::from_value(&ipv6).value()
    );
}

#[test]
fn every_spelling_list_holds_both_accepted_and_rejected_spellings() {
    let domain_names = domain_name_spellings();

    assert!(domain_names
        .iter()
        .any(|spelling| DomainNameValue::new(spelling).is_ok()));
    assert!(domain_names
        .iter()
        .any(|spelling| DomainNameValue::new(spelling).is_err()));
    assert!(IPV4_ADDRESS_SPELLINGS
        .iter()
        .any(|spelling| Ipv4AddressValue::new(spelling).is_ok()));
    assert!(IPV4_ADDRESS_SPELLINGS
        .iter()
        .any(|spelling| Ipv4AddressValue::new(spelling).is_err()));
    assert!(IPV6_ADDRESS_SPELLINGS
        .iter()
        .any(|spelling| Ipv6AddressValue::new(spelling).is_ok()));
    assert!(IPV6_ADDRESS_SPELLINGS
        .iter()
        .any(|spelling| Ipv6AddressValue::new(spelling).is_err()));
}
