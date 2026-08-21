use kernel::source::domain::value_objects::source_description::{SourceDescription, MAX_LENGTH};

#[test]
fn it_trims_surrounding_whitespace() {
    let value = SourceDescription::new("  primary feed  ").expect("valid value");

    assert_eq!(value.value(), "primary feed");
}

#[test]
fn it_accepts_exactly_the_maximum_length() {
    let value =
        SourceDescription::new("a".repeat(MAX_LENGTH)).expect("the limit itself must be accepted");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_rejects_one_character_over_the_maximum_length() {
    let error = SourceDescription::new("a".repeat(MAX_LENGTH + 1))
        .expect_err("one character over the limit must be rejected");

    assert!(error.message().contains("at most"));
}

#[test]
fn it_counts_characters_and_not_bytes() {
    let value = SourceDescription::new("\u{1f98a}".repeat(MAX_LENGTH))
        .expect("multi-byte characters count as one");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_compares_equal_for_the_same_content() {
    assert_eq!(
        SourceDescription::new("primary feed").expect("valid value"),
        SourceDescription::new("  primary feed  ").expect("valid value")
    );
}

#[test]
fn it_accepts_an_empty_value() {
    let value = SourceDescription::new("").expect("an empty value is allowed");

    assert_eq!(value.value(), "");
}
