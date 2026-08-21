use kernel::ioc::domain::value_objects::ioc_value::{IocValue, MAX_LENGTH};

#[test]
fn it_trims_surrounding_whitespace() {
    let value = IocValue::new("  198.51.100.7  ").expect("valid value");

    assert_eq!(value.value(), "198.51.100.7");
}

#[test]
fn it_accepts_exactly_the_maximum_length() {
    let value = IocValue::new("a".repeat(MAX_LENGTH)).expect("the limit itself must be accepted");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_rejects_one_character_over_the_maximum_length() {
    let error = IocValue::new("a".repeat(MAX_LENGTH + 1))
        .expect_err("one character over the limit must be rejected");

    assert!(error.message().contains("at most"));
}

#[test]
fn it_counts_characters_and_not_bytes() {
    let value =
        IocValue::new("\u{1f98a}".repeat(MAX_LENGTH)).expect("multi-byte characters count as one");

    assert_eq!(value.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_compares_equal_for_the_same_content() {
    assert_eq!(
        IocValue::new("198.51.100.7").expect("valid value"),
        IocValue::new("  198.51.100.7  ").expect("valid value")
    );
}

#[test]
fn it_rejects_an_empty_value() {
    let error = IocValue::new("   ").expect_err("an empty value must be rejected");

    assert!(error.message().contains("must not be empty"));
}
