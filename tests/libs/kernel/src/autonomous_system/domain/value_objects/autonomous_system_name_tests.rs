use kernel::autonomous_system::domain::value_objects::autonomous_system_name::{
    AutonomousSystemName, MAX_LENGTH,
};

fn rejection(value: &str) -> String {
    AutonomousSystemName::new(value)
        .expect_err("the name must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_the_name_as_registered() {
    assert_eq!(
        AutonomousSystemName::new("GOOGLE")
            .expect("valid name")
            .value(),
        "GOOGLE"
    );
    assert_eq!(
        AutonomousSystemName::new("Cloudflare, Inc.")
            .expect("valid name")
            .value(),
        "Cloudflare, Inc."
    );
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(
        AutonomousSystemName::new("  GOOGLE \n")
            .expect("valid name")
            .value(),
        "GOOGLE"
    );
}

#[test]
fn it_accepts_a_name_of_exactly_the_maximum_length() {
    let name = "a".repeat(MAX_LENGTH);

    assert_eq!(
        AutonomousSystemName::new(&name)
            .expect("valid name")
            .value(),
        name
    );
}

#[test]
fn it_rejects_a_name_one_character_over_the_maximum() {
    assert!(rejection(&"a".repeat(MAX_LENGTH + 1)).contains("at most 255 characters"));
}

#[test]
fn it_counts_characters_not_bytes() {
    let name = "é".repeat(MAX_LENGTH);

    assert!(AutonomousSystemName::new(&name).is_ok());
}

#[test]
fn it_rejects_an_empty_name() {
    assert!(rejection("   ").contains("must not be empty"));
}

#[test]
fn it_rejects_control_characters() {
    assert!(rejection("GOOGLE\nARIN").contains("control characters"));
    assert!(rejection("GOO\u{7}GLE").contains("control characters"));
}
