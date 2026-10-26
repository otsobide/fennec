use kernel::domain_name::domain::value_objects::domain_name_value::{
    DomainNameValue, MAX_LABEL_LENGTH, MAX_LENGTH,
};

fn canonical(value: &str) -> String {
    DomainNameValue::new(value).expect("valid name").to_string()
}

fn rejection(value: &str) -> String {
    DomainNameValue::new(value)
        .expect_err("the name must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_name() {
    assert_eq!(canonical("hostelhotels.net"), "hostelhotels.net");
}

#[test]
fn it_lowercases_the_name() {
    assert_eq!(canonical("HostelHotels.NET"), "hostelhotels.net");
}

#[test]
fn it_drops_surrounding_whitespace_and_the_root_dot() {
    assert_eq!(canonical("  example.com. \n"), "example.com");
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
fn it_accepts_underscores_used_by_service_records() {
    assert_eq!(canonical("_dmarc.example.com"), "_dmarc.example.com");
}

#[test]
fn it_accepts_hyphens_inside_labels() {
    assert_eq!(
        canonical("update.example-cdn.net"),
        "update.example-cdn.net"
    );
}

#[test]
fn it_accepts_labels_of_exactly_the_maximum_length() {
    let label = "a".repeat(MAX_LABEL_LENGTH);

    assert_eq!(canonical(&format!("{label}.com")), format!("{label}.com"));
}

#[test]
fn it_rejects_a_label_one_character_over_the_maximum() {
    let label = "a".repeat(MAX_LABEL_LENGTH + 1);

    assert!(rejection(&format!("{label}.com")).contains("labels must be at most"));
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

    assert!(rejection(&name).contains("must be at most 253"));
}

#[test]
fn it_rejects_an_empty_name() {
    assert!(rejection("  .").contains("must not be empty"));
}

#[test]
fn it_rejects_a_single_label() {
    assert!(rejection("localhost").contains("at least two labels"));
}

#[test]
fn it_rejects_an_ipv4_address() {
    assert!(rejection("10.0.0.10").contains("numeric label"));
}

#[test]
fn it_rejects_empty_labels() {
    assert!(rejection("example..com").contains("empty labels"));
}

#[test]
fn it_rejects_labels_that_start_or_end_with_a_hyphen() {
    assert!(rejection("-example.com").contains("start or end with a hyphen"));
    assert!(rejection("example-.com").contains("start or end with a hyphen"));
}

#[test]
fn it_rejects_characters_that_are_not_allowed_in_labels() {
    for invalid in [
        "exa mple.com",
        "example.com/path",
        "user@example.com",
        "example[.]com",
    ] {
        let message = rejection(invalid);
        assert!(
            message.contains("may only hold") || message.contains("not a valid internationalised"),
            "{invalid:?} gave: {message}"
        );
    }
}

#[test]
fn it_rejects_a_url() {
    let message = rejection("https://example.com");

    assert!(message.contains("may only hold") || message.contains("not a valid internationalised"));
}

#[test]
fn it_exposes_its_labels_and_top_level_domain() {
    let name = DomainNameValue::new("www.Example.co.uk").expect("valid name");

    assert_eq!(
        name.labels().collect::<Vec<_>>(),
        vec!["www", "example", "co", "uk"]
    );
    assert_eq!(name.top_level_domain(), "uk");
}

#[test]
fn it_reports_whether_it_was_internationalised() {
    assert!(DomainNameValue::new("faß.de")
        .expect("valid name")
        .is_internationalised());
    assert!(!DomainNameValue::new("example.com")
        .expect("valid name")
        .is_internationalised());
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_name() {
    assert_eq!(
        DomainNameValue::new("Faß.DE.").expect("valid name"),
        DomainNameValue::new("xn--fa-hia.de").expect("valid name")
    );
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let name = DomainNameValue::new("HostelHotels.NET.").expect("valid name");

    assert_eq!(
        name.contributing_properties().to_canonical_string(),
        r#"{"value":"hostelhotels.net"}"#
    );
}
