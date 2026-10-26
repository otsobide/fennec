use kernel::autonomous_system::domain::events::create_autonomous_system_updated_event::create_autonomous_system_updated_event;
use kernel::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use kernel::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_name_mother::AutonomousSystemNameMother;

#[test]
fn it_carries_the_new_and_the_previous_description() {
    let previous = AutonomousSystemMother::create("15169");
    let updated_at = AutonomousSystemUpdatedAt::now();
    let updated = previous.describe(
        Some(AutonomousSystemNameMother::create("GOOGLE")),
        Some(AutonomousSystemRir::Arin),
        updated_at,
    );

    let event =
        create_autonomous_system_updated_event(&updated, &previous).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(
        event.event_name(),
        "fennec.kernel.autonomous_system.updated"
    );
    assert_eq!(event.aggregate_id(), "7afc7bdf-d070-5add-90ff-f93503b1654d");
    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "id",
            "new_name",
            "new_rir",
            "old_name",
            "old_rir",
            "updated_at",
            "value"
        ]
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("15169".to_string())
    );
    assert_eq!(
        primitives["new_name"],
        PrimitiveValue::Text("GOOGLE".to_string())
    );
    assert_eq!(
        primitives["new_rir"],
        PrimitiveValue::Text("ARIN".to_string())
    );
    assert_eq!(primitives["old_name"], PrimitiveValue::Null);
    assert_eq!(primitives["old_rir"], PrimitiveValue::Null);
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(updated_at.value())
    );
}

#[test]
fn it_writes_a_cleared_description_as_null() {
    let previous = AutonomousSystemMother::random().describe(
        Some(AutonomousSystemNameMother::create("RIPE-NCC-AS")),
        Some(AutonomousSystemRir::RipeNcc),
        AutonomousSystemUpdatedAt::now(),
    );
    let updated = previous.describe(None, None, AutonomousSystemUpdatedAt::now());

    let primitives = create_autonomous_system_updated_event(&updated, &previous)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(primitives["new_name"], PrimitiveValue::Null);
    assert_eq!(primitives["new_rir"], PrimitiveValue::Null);
    assert_eq!(
        primitives["old_name"],
        PrimitiveValue::Text("RIPE-NCC-AS".to_string())
    );
    assert_eq!(
        primitives["old_rir"],
        PrimitiveValue::Text("RIPE NCC".to_string())
    );
}
