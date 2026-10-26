use kernel::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

fn number(value: &str) -> u32 {
    AutonomousSystemValue::new(value)
        .expect("valid number")
        .number()
}

fn rejection(value: &str) -> String {
    AutonomousSystemValue::new(value)
        .expect_err("the number must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_plain_number() {
    assert_eq!(number("15169"), 15169);
}

#[test]
fn it_accepts_the_as_prefix_in_any_case() {
    for spelling in ["AS15169", "as15169", "As15169", "aS15169"] {
        assert_eq!(number(spelling), 15169, "{spelling}");
    }
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(number("  AS15169 \n"), 15169);
}

#[test]
fn it_ignores_leading_zeros() {
    assert_eq!(number("AS0015169"), 15169);
}

#[test]
fn it_accepts_the_asdot_notation() {
    assert_eq!(number("1.10"), 65546);
    assert_eq!(number("AS1.10"), 65546);
    assert_eq!(number("0.15169"), 15169);
    assert_eq!(number("65535.65535"), u32::MAX);
}

#[test]
fn it_accepts_the_lowest_and_highest_numbers() {
    assert_eq!(number("0"), 0);
    assert_eq!(number("4294967295"), u32::MAX);
}

#[test]
fn it_rejects_a_number_one_above_the_range() {
    assert!(rejection("4294967296").contains("from 0 to 4294967295"));
}

#[test]
fn it_rejects_asdot_halves_above_65535() {
    assert!(rejection("65536.0").contains("asdot"));
    assert!(rejection("1.65536").contains("asdot"));
}

#[test]
fn it_rejects_malformed_asdot() {
    for value in ["1.", ".10", "1.2.3", "1. 2"] {
        assert!(rejection(value).contains("asdot"), "{value}");
    }
}

#[test]
fn it_rejects_signs_spaces_and_other_characters() {
    for value in [
        "-1", "+15169", "AS 15169", "AS-15169", "15,169", "0x3b41", "AS",
    ] {
        assert!(rejection(value).contains("decimal number"), "{value}");
    }
}

#[test]
fn it_rejects_non_ascii_digits() {
    assert!(rejection("AS１５１６９").contains("decimal number"));
}

#[test]
fn it_rejects_an_empty_value() {
    assert!(rejection("  ").contains("decimal number"));
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_number() {
    assert_eq!(
        AutonomousSystemValue::new("AS65546").expect("valid number"),
        AutonomousSystemValue::new(" as1.10 ").expect("valid number")
    );
}

#[test]
fn it_displays_the_plain_number_and_labels_it() {
    let value = AutonomousSystemValue::new("as15169").expect("valid number");

    assert_eq!(value.to_string(), "15169");
    assert_eq!(value.label(), "AS15169");
}

#[test]
fn it_exposes_its_stix_contributing_properties_as_an_integer() {
    assert_eq!(
        AutonomousSystemValue::new("AS15169")
            .expect("valid number")
            .contributing_properties()
            .to_canonical_string(),
        r#"{"number":15169}"#
    );
}

#[test]
fn it_classifies_special_ranges() {
    let value = |raw: &str| AutonomousSystemValue::new(raw).expect("valid number");

    assert!(!value("65535").is_four_byte());
    assert!(value("65536").is_four_byte());
    assert!(value("64512").is_private_use());
    assert!(value("65534").is_private_use());
    assert!(!value("65535").is_private_use());
    assert!(value("4200000000").is_private_use());
    assert!(!value("4294967295").is_private_use());
    assert!(value("64496").is_documentation());
    assert!(value("65551").is_documentation());
    assert!(!value("15169").is_documentation());
}
