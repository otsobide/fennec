use kernel::sighting::domain::value_objects::sighting_observable_kind::{
    SightingObservableKind, MAX_LENGTH,
};

#[test]
fn it_accepts_a_module_name() {
    let kind = SightingObservableKind::new("ipv4_address").expect("valid kind");

    assert_eq!(kind.value(), "ipv4_address");
}

#[test]
fn it_accepts_names_with_digits() {
    let kind = SightingObservableKind::new("ja4_fingerprint").expect("valid kind");

    assert_eq!(kind.value(), "ja4_fingerprint");
}

#[test]
fn it_trims_surrounding_whitespace() {
    let kind = SightingObservableKind::new("  domain_name ").expect("valid kind");

    assert_eq!(kind.value(), "domain_name");
}

#[test]
fn it_accepts_exactly_the_maximum_length() {
    let kind = SightingObservableKind::new(&"a".repeat(MAX_LENGTH)).expect("the limit is accepted");

    assert_eq!(kind.value().len(), MAX_LENGTH);
}

#[test]
fn it_rejects_one_character_over_the_maximum_length() {
    let error = SightingObservableKind::new(&"a".repeat(MAX_LENGTH + 1))
        .expect_err("over the limit must be rejected");

    assert!(error.message().contains("at most"));
}

#[test]
fn it_rejects_names_that_are_not_snake_case() {
    for invalid in [
        "",
        "Ipv4Address",
        "ipv4-address",
        "4ipv",
        "_ipv4",
        "ipv4_",
        "ipv4__address",
        "ipv4 address",
        "ipv4-addr",
        "ípv4",
    ] {
        let error = SightingObservableKind::new(invalid).expect_err("not a module name");

        assert!(
            error.message().contains("snake case module name"),
            "{invalid:?} gave: {}",
            error.message()
        );
    }
}

#[test]
fn it_compares_equal_for_the_same_name() {
    assert_eq!(
        SightingObservableKind::new("ipv4_address").expect("valid kind"),
        SightingObservableKind::new(" ipv4_address ").expect("valid kind")
    );
}
