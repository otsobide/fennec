use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use shared_valueobject::domain::dns::dns_name::{MAX_LABEL_LENGTH, MAX_LENGTH};

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;

fn canonical(value: &str) -> String {
    DnsResolutionSubject::domain_name(value)
        .expect("valid domain name")
        .value()
        .to_string()
}

fn rejection(value: &str) -> String {
    DnsResolutionSubject::domain_name(value)
        .expect_err("the domain name must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_name() {
    assert_eq!(canonical("example.com"), "example.com");
}

#[test]
fn it_lowercases_the_name() {
    assert_eq!(canonical("Example.COM"), "example.com");
}

#[test]
fn it_drops_surrounding_ascii_whitespace() {
    assert_eq!(canonical("  example.com \t\n"), "example.com");
}

#[test]
fn it_trims_only_ascii_whitespace() {
    for spaced in ["\u{a0}example.com", "example.com\u{3000}"] {
        assert!(
            DnsResolutionSubject::domain_name(spaced).is_err(),
            "{spaced:?} must be rejected"
        );
    }
}

#[test]
fn it_drops_the_root_dot() {
    assert_eq!(canonical("example.com."), "example.com");
    assert_eq!(canonical(" Example.COM. "), "example.com");
}

#[test]
fn it_drops_only_one_root_dot() {
    assert_eq!(
        rejection("example.com.."),
        "domain name must not have empty labels, got example.com.."
    );
}

#[test]
fn it_converts_internationalised_labels_to_a_labels() {
    assert_eq!(canonical("faß.de"), "xn--fa-hia.de");
    assert_eq!(canonical("Bücher.example"), "xn--bcher-kva.example");
}

#[test]
fn it_keeps_a_labels_as_they_are() {
    assert_eq!(canonical("xn--fa-hia.de"), "xn--fa-hia.de");
}

#[test]
fn it_rejects_an_invalid_a_label() {
    assert_eq!(
        rejection("xn--a.com"),
        "domain name is not a valid internationalised name, got xn--a.com"
    );
}

#[test]
fn it_accepts_underscores_and_inner_hyphens() {
    assert_eq!(canonical("_dmarc.example.com"), "_dmarc.example.com");
    assert_eq!(
        canonical("update.example-cdn.net"),
        "update.example-cdn.net"
    );
}

#[test]
fn it_accepts_two_labels() {
    assert_eq!(canonical("a.b"), "a.b");
}

#[test]
fn it_rejects_a_single_label() {
    assert_eq!(
        rejection("localhost"),
        "domain name must have at least two labels, got localhost"
    );
}

#[test]
fn it_rejects_a_single_label_with_a_root_dot() {
    assert_eq!(
        rejection("localhost."),
        "domain name must have at least two labels, got localhost."
    );
}

#[test]
fn it_quotes_the_raw_value_in_its_messages() {
    assert_eq!(
        rejection(" localhost "),
        "domain name must have at least two labels, got  localhost "
    );
}

#[test]
fn it_rejects_an_empty_name() {
    assert_eq!(rejection(""), "domain name must not be empty, got ");
    assert_eq!(rejection("  . "), "domain name must not be empty, got   . ");
}

#[test]
fn it_rejects_empty_labels() {
    assert_eq!(
        rejection("example..com"),
        "domain name must not have empty labels, got example..com"
    );
    assert_eq!(
        rejection(".example.com"),
        "domain name must not have empty labels, got .example.com"
    );
}

#[test]
fn it_accepts_a_label_of_exactly_the_maximum_length() {
    let name = format!("{}.com", "a".repeat(MAX_LABEL_LENGTH));

    assert_eq!(canonical(&name), name);
}

#[test]
fn it_rejects_a_label_one_character_over_the_maximum() {
    let name = format!("{}.com", "a".repeat(MAX_LABEL_LENGTH + 1));

    assert_eq!(
        rejection(&name),
        format!("domain name labels must be at most 63 characters, got {name}")
    );
}

#[test]
fn it_accepts_a_name_of_exactly_the_maximum_length() {
    let name = format!(
        "{}.{}.{}.{}.com",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(57)
    );
    assert_eq!(name.len(), MAX_LENGTH);

    assert_eq!(canonical(&name), name);
}

#[test]
fn it_rejects_a_name_one_character_over_the_maximum() {
    let name = format!(
        "{}.{}.{}.{}.com",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(58)
    );

    assert_eq!(
        rejection(&name),
        format!("domain name must be at most 253 characters, got {name}")
    );
}

#[test]
fn it_rejects_labels_that_start_or_end_with_a_hyphen() {
    assert_eq!(
        rejection("-example.com"),
        "domain name labels must not start or end with a hyphen, got -example.com"
    );
    assert_eq!(
        rejection("example-.com"),
        "domain name labels must not start or end with a hyphen, got example-.com"
    );
}

#[test]
fn it_rejects_characters_that_are_not_allowed_in_labels() {
    for invalid in [
        "exa mple.com",
        "example.com/path",
        "user@example.com",
        "example[.]com",
        "https://example.com",
    ] {
        let message = rejection(invalid);
        assert!(
            message.starts_with("domain name labels may only hold")
                || message.starts_with("domain name is not a valid internationalised name"),
            "{invalid:?} gave: {message}"
        );
        assert!(message.ends_with(&format!(", got {invalid}")));
    }
}

#[test]
fn it_rejects_an_ipv4_address() {
    assert_eq!(
        rejection("93.184.216.34"),
        "domain name must not end in a numeric label, got 93.184.216.34"
    );
}

#[test]
fn it_accepts_a_numeric_label_before_the_last_one() {
    assert_eq!(canonical("34.216.184.93.example"), "34.216.184.93.example");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_name() {
    assert_eq!(
        DnsResolutionSubjectMother::create("Faß.DE."),
        DnsResolutionSubjectMother::create(" xn--fa-hia.de ")
    );
}

#[test]
fn it_is_a_domain_name() {
    assert_eq!(DnsResolutionSubjectMother::random().kind(), "domain_name");
}

#[test]
fn it_displays_its_canonical_value() {
    assert_eq!(
        DnsResolutionSubjectMother::create("Example.COM.").to_string(),
        "example.com"
    );
}

#[test]
fn it_derives_the_id_of_the_domain_name_module() {
    assert_eq!(
        DnsResolutionSubjectMother::create("example.com")
            .id()
            .to_string(),
        "bedb4899-d24b-5401-bc86-8f6b4cc18ec7"
    );
    assert_eq!(
        DnsResolutionSubjectMother::create("hostelhotels.net")
            .id()
            .to_string(),
        "07f3ac51-335f-524c-918f-86f80da2bce3"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_name() {
    assert_eq!(
        DnsResolutionSubjectMother::create("HostelHotels.NET.").id(),
        DnsResolutionSubjectMother::create(" hostelhotels.net ").id()
    );
}

#[test]
fn it_derives_different_ids_for_different_names() {
    assert_ne!(
        DnsResolutionSubjectMother::create("example.com").id(),
        DnsResolutionSubjectMother::create("example.org").id()
    );
}

#[test]
fn it_exposes_its_kind_qualified_reference() {
    assert_eq!(
        DnsResolutionSubjectMother::create("Example.COM")
            .reference()
            .to_canonical_string(),
        r#"{"id":"bedb4899-d24b-5401-bc86-8f6b4cc18ec7","kind":"domain_name"}"#
    );
}
