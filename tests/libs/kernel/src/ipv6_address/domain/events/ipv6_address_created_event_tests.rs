use kernel::ipv6_address::domain::events::create_ipv6_address_created_event::create_ipv6_address_created_event;
use kernel::ipv6_address::domain::events::ipv6_address_created_event::Ipv6AddressCreatedEvent;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let ipv6_address = Ipv6AddressMother::create("2001:db8:85a3::8a2e:370:7334");

    let primitives = create_ipv6_address_created_event(&ipv6_address)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["created_at", "id", "value"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("f5344921-72d2-5619-945d-8757bca4b7cd".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("2001:db8:85a3::8a2e:370:7334".to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(ipv6_address.created_at().value())
    );
}

#[test]
fn it_is_named_after_the_kernel_ipv6_address_module() {
    let event = create_ipv6_address_created_event(&Ipv6AddressMother::random())
        .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.ipv6_address.created");
    assert_eq!(event.event_name(), Ipv6AddressCreatedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_ipv6_address_id_as_aggregate_id() {
    let ipv6_address = Ipv6AddressMother::random();

    let event = create_ipv6_address_created_event(&ipv6_address).expect("event must be created");

    assert_eq!(event.aggregate_id(), ipv6_address.id().to_string());
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let ipv6_address = Ipv6AddressMother::random();

    let event = create_ipv6_address_created_event(&ipv6_address).expect("event must be created");

    assert_eq!(event.id(), ipv6_address.id());
    assert_eq!(*event.id(), Ipv6AddressId::from_value(event.value()));
}
