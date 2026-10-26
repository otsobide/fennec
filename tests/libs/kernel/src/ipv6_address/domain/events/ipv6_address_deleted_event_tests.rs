use kernel::ipv6_address::domain::events::create_ipv6_address_deleted_event::create_ipv6_address_deleted_event;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let ipv6_address = Ipv6AddressMother::create("2001:db8:85a3::8a2e:370:7334");

    let event = create_ipv6_address_deleted_event(&ipv6_address).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.ipv6_address.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("f5344921-72d2-5619-945d-8757bca4b7cd".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("2001:db8:85a3::8a2e:370:7334".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let ipv6_address = Ipv6AddressMother::random();

    let event = create_ipv6_address_deleted_event(&ipv6_address).expect("event must be created");

    assert_eq!(event.id(), ipv6_address.id());
    assert_eq!(*event.id(), Ipv6AddressId::from_value(event.value()));
}
