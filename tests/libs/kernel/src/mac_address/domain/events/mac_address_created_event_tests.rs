use kernel::mac_address::domain::events::create_mac_address_created_event::create_mac_address_created_event;
use kernel::mac_address::domain::events::mac_address_created_event::MacAddressCreatedEvent;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::mac_address::domain::entities::mothers::mac_address_mother::MacAddressMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let mac_address = MacAddressMother::create("d2:fb:49:24:37:18");

    let primitives = create_mac_address_created_event(&mac_address)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["created_at", "id", "value"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("757b1725-9903-54f5-a855-1240691d7659".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("d2:fb:49:24:37:18".to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(mac_address.created_at().value())
    );
}

#[test]
fn it_is_named_after_the_kernel_mac_address_module() {
    let event = create_mac_address_created_event(&MacAddressMother::random())
        .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.mac_address.created");
    assert_eq!(event.event_name(), MacAddressCreatedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_mac_address_id_as_aggregate_id() {
    let mac_address = MacAddressMother::random();

    let event = create_mac_address_created_event(&mac_address).expect("event must be created");

    assert_eq!(event.aggregate_id(), mac_address.id().to_string());
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let mac_address = MacAddressMother::random();

    let event = create_mac_address_created_event(&mac_address).expect("event must be created");

    assert_eq!(event.id(), mac_address.id());
    assert_eq!(*event.id(), MacAddressId::from_value(event.value()));
}
