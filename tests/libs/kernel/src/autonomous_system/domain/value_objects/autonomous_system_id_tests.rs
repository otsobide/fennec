use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use uuid::Uuid;

use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;

fn id_of(value: &str) -> AutonomousSystemId {
    AutonomousSystemIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("15169").stix_id(),
        "autonomous-system--7afc7bdf-d070-5add-90ff-f93503b1654d"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("13335").stix_id(),
        "autonomous-system--0204c07d-e4dd-5f14-a3d5-c93cb1c5a9fc"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_autonomous_system() {
    assert_eq!(id_of("AS15169"), id_of(" 15169 "));
}

#[test]
fn it_derives_different_ids_for_different_autonomous_systems() {
    assert_ne!(id_of("13335"), id_of("65546"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("13335").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("15169");

    assert_eq!(
        AutonomousSystemId::new(&id.to_string()).expect("valid id"),
        id
    );
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = AutonomousSystemId::new("  7afc7bdf-d070-5add-90ff-f93503b1654d ").expect("valid id");

    assert_eq!(id, id_of("15169"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = AutonomousSystemId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = AutonomousSystemId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = AutonomousSystemId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
