use shared_valueobject::domain::dns::dns_name::{DnsName, MAX_LABEL_LENGTH, MAX_LENGTH};
use shared_valueobject::domain::dns::dns_name_error::DnsNameError;

fn canonical(raw: &str) -> String {
    DnsName::parse(raw).expect("valid DNS name").to_string()
}

fn error(raw: &str) -> DnsNameError {
    DnsName::parse(raw).expect_err("the DNS name must be rejected")
}

#[test]
fn it_keeps_a_canonical_name() {
    assert_eq!(canonical("mail01.corp.example"), "mail01.corp.example");
}

#[test]
fn it_accepts_a_single_label() {
    assert_eq!(canonical("localhost"), "localhost");
}

#[test]
fn it_lowercases_the_name() {
    assert_eq!(canonical("Mail01.CORP.Example"), "mail01.corp.example");
}

#[test]
fn it_drops_one_trailing_dot() {
    assert_eq!(canonical("example.com."), "example.com");
}

#[test]
fn it_converts_internationalised_labels_to_a_labels() {
    assert_eq!(canonical("faß.de"), "xn--fa-hia.de");
    assert_eq!(canonical("Bücher.example"), "xn--bcher-kva.example");
}

#[test]
fn it_keeps_underscores() {
    assert_eq!(canonical("_dmarc.example.com"), "_dmarc.example.com");
}

#[test]
fn it_accepts_a_label_of_exactly_the_maximum_length() {
    let label = "a".repeat(MAX_LABEL_LENGTH);

    assert_eq!(canonical(&label), label);
}

#[test]
fn it_rejects_a_label_one_character_over_the_maximum() {
    assert_eq!(
        error(&"a".repeat(MAX_LABEL_LENGTH + 1)),
        DnsNameError::LabelTooLong
    );
}

#[test]
fn it_accepts_a_name_of_exactly_the_maximum_length() {
    let name = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );

    assert_eq!(name.len(), MAX_LENGTH);
    assert_eq!(canonical(&name), name);
}

#[test]
fn it_rejects_a_name_one_character_over_the_maximum() {
    let name = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62)
    );

    assert_eq!(error(&name), DnsNameError::TooLong);
}

#[test]
fn it_rejects_an_empty_name() {
    assert_eq!(error(""), DnsNameError::Empty);
    assert_eq!(error("."), DnsNameError::Empty);
}

#[test]
fn it_does_not_trim_whitespace() {
    assert_eq!(error(" example.com"), DnsNameError::InvalidLabelCharacter);
}

#[test]
fn it_applies_the_uts46_mappings() {
    assert_eq!(canonical("ex\u{200b}ample.com"), "example.com");
    assert_eq!(canonical("example\u{ff0e}com"), "example.com");
    assert_eq!(canonical("a\u{301}.example"), "xn--1ca.example");
}

#[test]
fn it_rejects_code_points_that_uts46_disallows() {
    assert_eq!(
        error("ex\u{fffd}ample.com"),
        DnsNameError::NotInternationalisable
    );
    assert_eq!(
        error("ex\u{2028}ample.com"),
        DnsNameError::NotInternationalisable
    );
}

#[test]
fn it_rejects_an_invalid_a_label() {
    assert_eq!(error("xn--a.example"), DnsNameError::NotInternationalisable);
}

#[test]
fn it_rejects_empty_labels() {
    assert_eq!(error("example..com"), DnsNameError::EmptyLabel);
    assert_eq!(error(".example.com"), DnsNameError::EmptyLabel);
    assert_eq!(error("example.com.."), DnsNameError::EmptyLabel);
}

#[test]
fn it_rejects_characters_outside_the_label_alphabet() {
    for raw in [
        "*.example.com",
        "exa!mple.com",
        "exa mple.com",
        "example.com/path",
        "user@example.com",
        "example[.]com",
        "exa\u{0}mple.com",
    ] {
        assert_eq!(error(raw), DnsNameError::InvalidLabelCharacter, "{raw:?}");
    }
}

#[test]
fn it_rejects_labels_that_start_or_end_with_a_hyphen() {
    assert_eq!(error("-example.com"), DnsNameError::EdgeHyphen);
    assert_eq!(error("example-.com"), DnsNameError::EdgeHyphen);
}

#[test]
fn it_rejects_a_numeric_last_label() {
    assert_eq!(error("10.0.0.10"), DnsNameError::NumericLastLabel);
    assert_eq!(error("12345"), DnsNameError::NumericLastLabel);
}

#[test]
fn it_accepts_digits_in_other_labels() {
    assert_eq!(canonical("123.example"), "123.example");
}

#[test]
fn it_exposes_its_labels() {
    let name = DnsName::parse("www.example.co.uk").expect("valid DNS name");

    assert_eq!(
        name.labels().collect::<Vec<_>>(),
        vec!["www", "example", "co", "uk"]
    );
    assert_eq!(name.label_count(), 4);
}

#[test]
fn it_reports_whether_it_was_internationalised() {
    assert!(DnsName::parse("faß.de")
        .expect("valid DNS name")
        .is_internationalised());
    assert!(!DnsName::parse("example.com")
        .expect("valid DNS name")
        .is_internationalised());
}

#[test]
fn it_phrases_each_rule_without_a_subject() {
    assert_eq!(DnsNameError::Empty.rule(), "must not be empty");
    assert_eq!(
        DnsNameError::TooLong.rule(),
        "must be at most 253 characters"
    );
    assert_eq!(
        DnsNameError::LabelTooLong.rule(),
        "labels must be at most 63 characters"
    );
    assert_eq!(
        DnsNameError::EmptyLabel.to_string(),
        "DNS name must not have empty labels"
    );
}
