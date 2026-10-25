use kernel::ioc::domain::events::create_ioc_deleted_event::create_ioc_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ioc::domain::entities::mothers::ioc_mother::IocMother;

#[test]
fn it_exposes_the_identity_as_primitives() {
    let ioc = IocMother::random();

    let primitives = create_ioc_deleted_event(&ioc)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id"]);
    assert_eq!(primitives["id"], PrimitiveValue::Text(ioc.id().to_string()));
}
