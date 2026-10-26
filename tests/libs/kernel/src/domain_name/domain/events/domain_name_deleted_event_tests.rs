use kernel::domain_name::domain::events::create_domain_name_deleted_event::create_domain_name_deleted_event;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::domain_name::domain::entities::mothers::domain_name_mother::DomainNameMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let domain_name = DomainNameMother::create("hostelhotels.net");

    let event = create_domain_name_deleted_event(&domain_name).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.domain_name.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("07f3ac51-335f-524c-918f-86f80da2bce3".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("hostelhotels.net".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let domain_name = DomainNameMother::random();

    let event = create_domain_name_deleted_event(&domain_name).expect("event must be created");

    assert_eq!(event.id(), domain_name.id());
    assert_eq!(*event.id(), DomainNameId::from_value(event.value()));
}
