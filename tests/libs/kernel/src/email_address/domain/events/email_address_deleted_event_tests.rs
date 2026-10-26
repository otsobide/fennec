use kernel::email_address::domain::events::create_email_address_deleted_event::create_email_address_deleted_event;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::email_address::domain::entities::mothers::email_address_mother::EmailAddressMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let email_address = EmailAddressMother::create("john@example.com");

    let event = create_email_address_deleted_event(&email_address).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.email_address.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("7165e2a9-671f-585d-b1e1-ca59c671d934".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("john@example.com".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let email_address = EmailAddressMother::random();

    let event = create_email_address_deleted_event(&email_address).expect("event must be created");

    assert_eq!(event.id(), email_address.id());
    assert_eq!(*event.id(), EmailAddressId::from_value(event.value()));
}
