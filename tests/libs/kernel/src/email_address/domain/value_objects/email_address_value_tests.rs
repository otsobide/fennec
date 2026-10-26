use kernel::email_address::domain::value_objects::email_address_value::{
    EmailAddressValue, MAX_LABEL_LENGTH, MAX_LENGTH, MAX_LOCAL_PART_LENGTH,
};

fn canonical(value: &str) -> String {
    EmailAddressValue::new(value)
        .expect("valid email address")
        .to_string()
}

fn rejection(value: &str) -> String {
    EmailAddressValue::new(value)
        .expect_err("the email address must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_address() {
    assert_eq!(canonical("john@example.com"), "john@example.com");
}

#[test]
fn it_lowercases_the_whole_address() {
    assert_eq!(
        canonical("John.Smith@Example.COM"),
        "john.smith@example.com"
    );
}

#[test]
fn it_drops_surrounding_whitespace() {
    assert_eq!(canonical("  john@example.com \n"), "john@example.com");
}

#[test]
fn it_drops_the_root_dot_of_the_domain() {
    assert_eq!(canonical("john@example.com."), "john@example.com");
}

#[test]
fn it_keeps_subaddresses_and_atom_specials() {
    assert_eq!(
        canonical("first.last+tag@mail.example.org"),
        "first.last+tag@mail.example.org"
    );
    assert_eq!(
        canonical("!#$%&'*+-/=?^_`{|}~@example.com"),
        "!#$%&'*+-/=?^_`{|}~@example.com"
    );
}

#[test]
fn it_converts_an_internationalised_domain_to_a_labels() {
    assert_eq!(
        canonical("Billing@Bücher.example"),
        "billing@xn--bcher-kva.example"
    );
}

#[test]
fn it_accepts_an_internationalised_local_part() {
    assert_eq!(canonical("JOSÉ@example.com"), "josé@example.com");
}

#[test]
fn it_puts_the_local_part_in_nfc() {
    assert_eq!(
        canonical("jose\u{301}@example.com"),
        "jos\u{e9}@example.com"
    );
}

#[test]
fn it_accepts_a_local_part_of_exactly_the_maximum_length() {
    let address = format!("{}@example.com", "a".repeat(MAX_LOCAL_PART_LENGTH));

    assert_eq!(canonical(&address), address);
}

#[test]
fn it_rejects_a_local_part_one_octet_over_the_maximum() {
    let address = format!("{}@example.com", "a".repeat(MAX_LOCAL_PART_LENGTH + 1));

    assert!(rejection(&address).contains("local part must be at most 64 octets"));
}

#[test]
fn it_counts_the_local_part_in_octets() {
    let at_limit = format!("{}@example.com", "é".repeat(MAX_LOCAL_PART_LENGTH / 2));
    let over_limit = format!("{}a@example.com", "é".repeat(MAX_LOCAL_PART_LENGTH / 2));

    assert_eq!(canonical(&at_limit), at_limit);
    assert!(rejection(&over_limit).contains("local part must be at most 64 octets"));
}

#[test]
fn it_accepts_domain_labels_of_exactly_the_maximum_length() {
    let address = format!("john@{}.com", "a".repeat(MAX_LABEL_LENGTH));

    assert_eq!(canonical(&address), address);
}

#[test]
fn it_rejects_a_domain_label_one_character_over_the_maximum() {
    let address = format!("john@{}.com", "a".repeat(MAX_LABEL_LENGTH + 1));

    assert!(rejection(&address).contains("labels must be at most"));
}

#[test]
fn it_accepts_an_address_of_exactly_the_maximum_length() {
    let address = format!(
        "{}@{}.{}.{}.com",
        "l".repeat(64),
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(57)
    );

    assert_eq!(address.len(), MAX_LENGTH);
    assert_eq!(canonical(&address), address);
}

#[test]
fn it_rejects_an_address_one_octet_over_the_maximum() {
    let address = format!(
        "{}@{}.{}.{}.com",
        "l".repeat(64),
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(58)
    );

    assert!(rejection(&address).contains("must be at most 254 octets"));
}

#[test]
fn it_rejects_an_empty_address() {
    assert!(rejection("  ").contains("must not be empty"));
}

#[test]
fn it_rejects_an_address_without_exactly_one_at_sign() {
    assert!(rejection("john.example.com").contains("exactly one @"));
    assert!(rejection("john@@example.com").contains("exactly one @"));
    assert!(rejection("john@mail@example.com").contains("exactly one @"));
    assert!(rejection("john[at]example.com").contains("exactly one @"));
}

#[test]
fn it_rejects_an_empty_local_part() {
    assert!(rejection("@example.com").contains("local part must not be empty"));
}

#[test]
fn it_rejects_a_quoted_local_part() {
    assert!(rejection("\"john\"@example.com").contains("must not be quoted"));
}

#[test]
fn it_rejects_misplaced_dots_in_the_local_part() {
    for invalid in [
        ".john@example.com",
        "john.@example.com",
        "jo..hn@example.com",
    ] {
        assert!(
            rejection(invalid).contains("two dots in a row"),
            "{invalid:?}"
        );
    }
}

#[test]
fn it_rejects_characters_that_are_not_allowed_in_the_local_part() {
    for invalid in [
        "john smith@example.com",
        "John Smith <john@example.com>",
        "mailto:john@example.com",
        "jo(hn)@example.com",
        "jo\u{a0}hn@example.com",
    ] {
        assert!(
            rejection(invalid).contains("local part may only hold"),
            "{invalid:?}"
        );
    }
}

#[test]
fn it_rejects_an_address_literal() {
    assert!(rejection("john@[192.0.2.1]").contains("not an address literal"));
    assert!(rejection("john@[IPv6:2001:db8::1]").contains("not an address literal"));
}

#[test]
fn it_rejects_an_empty_domain() {
    assert!(rejection("john@").contains("domain must not be empty"));
    assert!(rejection("john@.").contains("domain must not be empty"));
}

#[test]
fn it_rejects_a_single_label_domain() {
    assert!(rejection("john@localhost").contains("at least two labels"));
}

#[test]
fn it_rejects_a_numeric_last_label() {
    assert!(rejection("john@192.0.2.1").contains("numeric label"));
}

#[test]
fn it_rejects_invalid_domain_labels() {
    assert!(rejection("john@example..com").contains("empty labels"));
    assert!(rejection("john@-example.com").contains("start or end with a hyphen"));
    for invalid in [
        "john@exa mple.com",
        "john@example[.]com",
        "john@example.com/path",
    ] {
        let message = rejection(invalid);
        assert!(
            message.contains("labels may only hold")
                || message.contains("not a valid internationalised"),
            "{invalid:?} gave: {message}"
        );
    }
}

#[test]
fn it_exposes_its_local_part_and_domain() {
    let address = EmailAddressValue::new("First.Last+Tag@MAIL.example.org").expect("valid address");

    assert_eq!(address.local_part(), "first.last+tag");
    assert_eq!(address.domain(), "mail.example.org");
}

#[test]
fn it_reports_whether_it_was_internationalised() {
    assert!(EmailAddressValue::new("billing@bücher.example")
        .expect("valid address")
        .is_internationalised());
    assert!(EmailAddressValue::new("josé@example.com")
        .expect("valid address")
        .is_internationalised());
    assert!(!EmailAddressValue::new("john@example.com")
        .expect("valid address")
        .is_internationalised());
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_address() {
    assert_eq!(
        EmailAddressValue::new("John@Example.COM.").expect("valid address"),
        EmailAddressValue::new(" john@example.com ").expect("valid address")
    );
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let address = EmailAddressValue::new("John@Example.COM").expect("valid address");

    assert_eq!(
        address.contributing_properties().to_canonical_string(),
        r#"{"value":"john@example.com"}"#
    );
}
