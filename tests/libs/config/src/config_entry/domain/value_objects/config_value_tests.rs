use config::config_entry::domain::value_objects::config_value::{ConfigValue, MAX_LENGTH};

#[test]
fn it_trims_surrounding_whitespace() {
    let value = ConfigValue::new("  30  ").expect("valid value");

    assert_eq!(value.value(), "30");
}

#[test]
fn it_accepts_exactly_the_maximum_length() {
    let value =
        ConfigValue::new("a".repeat(MAX_LENGTH)).expect("the limit itself must be accepted");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_rejects_one_character_over_the_maximum_length() {
    let error = ConfigValue::new("a".repeat(MAX_LENGTH + 1))
        .expect_err("one character over the limit must be rejected");

    assert!(error.message().contains("at most"));
}

#[test]
fn it_counts_characters_and_not_bytes() {
    let value = ConfigValue::new("\u{1f98a}".repeat(MAX_LENGTH))
        .expect("multi-byte characters count as one");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_compares_equal_for_the_same_content() {
    assert_eq!(
        ConfigValue::new("30").expect("valid value"),
        ConfigValue::new("  30  ").expect("valid value")
    );
}

#[test]
fn it_accepts_an_empty_value() {
    let value = ConfigValue::new("").expect("an empty value is allowed");

    assert_eq!(value.value(), "");
}
