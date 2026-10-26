use kernel::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use kernel::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

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
