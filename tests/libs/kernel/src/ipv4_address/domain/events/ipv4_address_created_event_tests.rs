use kernel::ipv4_address::domain::events::create_ipv4_address_created_event::create_ipv4_address_created_event;
use kernel::ipv4_address::domain::events::ipv4_address_created_event::Ipv4AddressCreatedEvent;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let ipv4_address = Ipv4AddressMother::create("89.19.209.154");

    let primitives = create_ipv4_address_created_event(&ipv4_address)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["created_at", "id", "value"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("abfb438a-8baf-5f54-b19e-b85ee74b3ed7".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("89.19.209.154".to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(ipv4_address.created_at().value())
    );
}

#[test]
fn it_is_named_after_the_kernel_ipv4_address_module() {
    let event = create_ipv4_address_created_event(&Ipv4AddressMother::random())
        .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.ipv4_address.created");
    assert_eq!(event.event_name(), Ipv4AddressCreatedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_ipv4_address_id_as_aggregate_id() {
    let ipv4_address = Ipv4AddressMother::random();

    let event = create_ipv4_address_created_event(&ipv4_address).expect("event must be created");

    assert_eq!(event.aggregate_id(), ipv4_address.id().to_string());
}
