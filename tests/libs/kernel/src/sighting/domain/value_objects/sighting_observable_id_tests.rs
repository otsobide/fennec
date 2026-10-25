use kernel::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use uuid::Uuid;

#[test]
fn it_accepts_a_version_5_id_of_a_value_identified_observable() {
    let id = SightingObservableId::new("abfb438a-8baf-5f54-b19e-b85ee74b3ed7").expect("valid id");

    assert_eq!(id.to_string(), "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
}

#[test]
fn it_accepts_a_version_4_id() {
    let uuid = Uuid::new_v4();

    let id = SightingObservableId::from_uuid(uuid).expect("v4 ids are accepted");

    assert_eq!(*id.value(), uuid);
}

#[test]
fn it_trims_surrounding_whitespace() {
    let id =
        SightingObservableId::new("  abfb438a-8baf-5f54-b19e-b85ee74b3ed7 ").expect("valid id");

    assert_eq!(id.to_string(), "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
}

#[test]
fn it_rejects_other_versions() {
    let error = SightingObservableId::new("11111111-1111-1111-8111-111111111111")
        .expect_err("version 1 is neither v4 nor v5");

    assert!(error.message().contains("v4 or v5"));
}

#[test]
fn it_rejects_a_non_rfc_4122_variant() {
    let error = SightingObservableId::new("00000000-0000-5000-0000-000000000000")
        .expect_err("the NCS variant is not RFC 4122");

    assert!(error.message().contains("v4 or v5"));
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = SightingObservableId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
