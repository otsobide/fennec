use kernel::hostname::domain::value_objects::hostname_value::{
    HostnameValue, MAX_LABEL_LENGTH, MAX_LENGTH,
};

fn canonical(value: &str) -> String {
    HostnameValue::new(value)
        .expect("valid hostname")
        .to_string()
}

fn rejection(value: &str) -> String {
    HostnameValue::new(value)
        .expect_err("the hostname must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_hostname() {
    assert_eq!(canonical("mail01.corp.example"), "mail01.corp.example");
}

#[test]
fn it_accepts_a_single_label() {
    assert_eq!(canonical("build-agent-7"), "build-agent-7");
    assert_eq!(canonical("localhost"), "localhost");
}

#[test]
fn it_lowercases_the_hostname() {
    assert_eq!(canonical("MAIL01.Corp.Example"), "mail01.corp.example");
}

#[test]
fn it_drops_surrounding_whitespace_and_the_root_dot() {
    assert_eq!(canonical("  Build-Agent-7. \n"), "build-agent-7");
}

#[test]
fn it_converts_internationalised_labels_to_a_labels() {
    assert_eq!(canonical("drucker-büro"), "xn--drucker-bro-1hb");
}

#[test]
fn it_accepts_labels_of_exactly_the_maximum_length() {
    let label = "a".repeat(MAX_LABEL_LENGTH);

    assert_eq!(canonical(&label), label);
}

#[test]
fn it_rejects_a_label_one_character_over_the_maximum() {
    assert!(rejection(&"a".repeat(MAX_LABEL_LENGTH + 1)).contains("labels must be at most"));
}

#[test]
fn it_accepts_a_hostname_of_exactly_the_maximum_length() {
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
fn it_rejects_a_hostname_one_character_over_the_maximum() {
    let name = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62)
    );

    assert!(rejection(&name).contains("must be at most 253"));
}

#[test]
fn it_rejects_an_empty_hostname() {
    assert!(rejection(" . ").contains("must not be empty"));
}

#[test]
fn it_rejects_an_hostname() {
    assert!(rejection("203.0.113.9").contains("numeric label"));
}

#[test]
fn it_rejects_a_purely_numeric_hostname() {
    assert!(rejection("12345").contains("numeric label"));
}

#[test]
fn it_rejects_empty_labels() {
    assert!(rejection("host..example").contains("empty labels"));
}

#[test]
fn it_rejects_labels_that_start_or_end_with_a_hyphen() {
    assert!(rejection("-host").contains("start or end with a hyphen"));
    assert!(rejection("host-.example").contains("start or end with a hyphen"));
}

#[test]
fn it_rejects_characters_that_are_not_allowed_in_labels() {
    for invalid in [
        "build agent",
        "host/path",
        "user@host",
        "host[.]example",
        "https://host",
    ] {
        let message = rejection(invalid);
        assert!(
            message.contains("may only hold") || message.contains("not a valid internationalised"),
            "{invalid:?} gave: {message}"
        );
    }
}

#[test]
fn it_exposes_its_machine_name() {
    let hostname = HostnameValue::new("Mail01.corp.example").expect("valid hostname");

    assert_eq!(hostname.machine_name(), "mail01");
    assert_eq!(hostname.labels().count(), 3);
    assert!(!hostname.is_single_label());
}

#[test]
fn it_reports_a_single_label_hostname() {
    let hostname = HostnameValue::new("build-agent-7").expect("valid hostname");

    assert!(hostname.is_single_label());
    assert_eq!(hostname.machine_name(), "build-agent-7");
}

#[test]
fn it_reports_whether_it_was_internationalised() {
    assert!(HostnameValue::new("drucker-büro")
        .expect("valid hostname")
        .is_internationalised());
    assert!(!HostnameValue::new("localhost")
        .expect("valid hostname")
        .is_internationalised());
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_hostname() {
    assert_eq!(
        HostnameValue::new("MAIL01.Corp.Example.").expect("valid hostname"),
        HostnameValue::new(" mail01.corp.example ").expect("valid hostname")
    );
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let hostname = HostnameValue::new("MAIL01.Corp.Example.").expect("valid hostname");

    assert_eq!(
        hostname.contributing_properties().to_canonical_string(),
        r#"{"value":"mail01.corp.example"}"#
    );
}
