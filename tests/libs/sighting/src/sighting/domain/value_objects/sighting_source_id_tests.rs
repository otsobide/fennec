use sighting::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use uuid::Uuid;

const VALID_V4: &str = "550e8400-e29b-41d4-a716-446655440000";

#[test]
fn it_accepts_a_valid_uuid_v4() {
    let id = SightingSourceId::new(VALID_V4).expect("a UUID v4 must be accepted");

    assert_eq!(id.value().to_string(), VALID_V4);
}

#[test]
fn it_ignores_surrounding_whitespace() {
    let id = SightingSourceId::new(&format!("  {VALID_V4}  ")).expect("whitespace must be trimmed");

    assert_eq!(id.to_string(), VALID_V4);
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error =
        SightingSourceId::new("not-a-uuid").expect_err("a malformed value must be rejected");

    assert!(error.message().contains("not a valid UUID"));
}

#[test]
fn it_rejects_a_uuid_that_is_not_version_4() {
    let error = SightingSourceId::new("00000000-0000-1000-8000-000000000000")
        .expect_err("a non-v4 UUID must be rejected");

    assert!(error.message().contains("UUID v4"));
}

#[test]
fn it_rejects_the_nil_uuid() {
    assert!(SightingSourceId::from_uuid(Uuid::nil()).is_err());
}

#[test]
fn it_generates_a_version_4_identifier() {
    let id = SightingSourceId::generate();

    assert_eq!(id.value().get_version_num(), 4);
}

#[test]
fn it_compares_equal_for_the_same_uuid() {
    let one = SightingSourceId::new(VALID_V4).expect("valid id");
    let other = SightingSourceId::new(VALID_V4).expect("valid id");

    assert_eq!(one, other);
    assert_ne!(one, SightingSourceId::generate());
}
