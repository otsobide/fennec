use kernel::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use kernel::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use kernel::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use kernel::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;

use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_name_mother::AutonomousSystemNameMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_value_mother::AutonomousSystemValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = AutonomousSystemValueMother::create("15169");

    let autonomous_system = AutonomousSystem::new(value, AutonomousSystemCreatedAt::now());

    assert_eq!(
        *autonomous_system.id(),
        AutonomousSystemId::from_value(&value)
    );
    assert_eq!(
        autonomous_system.id().stix_id(),
        "autonomous-system--7afc7bdf-d070-5add-90ff-f93503b1654d"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = AutonomousSystemValueMother::random();
    let created_at = AutonomousSystemCreatedAt::now();

    let autonomous_system = AutonomousSystem::new(value, created_at);

    assert_eq!(*autonomous_system.value(), value);
    assert_eq!(*autonomous_system.created_at(), created_at);
}

#[test]
fn it_starts_without_a_name_or_registry() {
    let created_at = AutonomousSystemCreatedAt::now();

    let autonomous_system =
        AutonomousSystem::new(AutonomousSystemValueMother::random(), created_at);

    assert_eq!(autonomous_system.name(), None);
    assert_eq!(autonomous_system.rir(), None);
    assert_eq!(
        autonomous_system.updated_at().unix_micros(),
        created_at.unix_micros()
    );
}

#[test]
fn it_is_described_without_changing_its_identity() {
    let original = AutonomousSystem::new(
        AutonomousSystemValueMother::create("15169"),
        AutonomousSystemCreatedAt::now(),
    );
    let name = AutonomousSystemNameMother::create("GOOGLE");
    let updated_at = AutonomousSystemUpdatedAt::now();

    let described = original.describe(
        Some(name.clone()),
        Some(AutonomousSystemRir::Arin),
        updated_at,
    );

    assert_eq!(described.id(), original.id());
    assert_eq!(described.value(), original.value());
    assert_eq!(described.created_at(), original.created_at());
    assert_eq!(described.name(), Some(&name));
    assert_eq!(described.rir(), Some(&AutonomousSystemRir::Arin));
    assert_eq!(*described.updated_at(), updated_at);
}

#[test]
fn it_clears_its_description_with_none() {
    let described = AutonomousSystem::new(
        AutonomousSystemValueMother::random(),
        AutonomousSystemCreatedAt::now(),
    )
    .describe(
        Some(AutonomousSystemNameMother::random()),
        Some(AutonomousSystemRir::RipeNcc),
        AutonomousSystemUpdatedAt::now(),
    );

    let cleared = described.describe(None, None, AutonomousSystemUpdatedAt::now());

    assert_eq!(cleared.name(), None);
    assert_eq!(cleared.rir(), None);
}

#[test]
fn it_is_restored_with_every_field_and_its_derived_id() {
    let value = AutonomousSystemValueMother::create("13335");
    let name = AutonomousSystemNameMother::create("CLOUDFLARENET");
    let created_at = AutonomousSystemCreatedAt::now();
    let updated_at = AutonomousSystemUpdatedAt::now();

    let restored = AutonomousSystem::restore(
        value,
        Some(name.clone()),
        Some(AutonomousSystemRir::Arin),
        created_at,
        updated_at,
    );

    assert_eq!(*restored.id(), AutonomousSystemId::from_value(&value));
    assert_eq!(restored.name(), Some(&name));
    assert_eq!(restored.rir(), Some(&AutonomousSystemRir::Arin));
    assert_eq!(*restored.created_at(), created_at);
    assert_eq!(*restored.updated_at(), updated_at);
}
