use kernel::autonomous_system::domain::events::create_autonomous_system_deleted_event::create_autonomous_system_deleted_event;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let autonomous_system = AutonomousSystemMother::create("15169");

    let event =
        create_autonomous_system_deleted_event(&autonomous_system).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(
        event.event_name(),
        "fennec.kernel.autonomous_system.deleted"
    );
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("7afc7bdf-d070-5add-90ff-f93503b1654d".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("15169".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let autonomous_system = AutonomousSystemMother::random();

    let event =
        create_autonomous_system_deleted_event(&autonomous_system).expect("event must be created");

    assert_eq!(event.id(), autonomous_system.id());
    assert_eq!(*event.id(), AutonomousSystemId::from_value(event.value()));
}
