use kernel::ioc::domain::events::create_ioc_created_event::create_ioc_created_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ioc::domain::entities::mothers::ioc_mother::IocMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let ioc = IocMother::random();

    let primitives = create_ioc_created_event(&ioc)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["created_at", "id", "ioc_type", "updated_at", "value"]
    );
    assert_eq!(primitives["id"], PrimitiveValue::Text(ioc.id().to_string()));
    assert_eq!(
        primitives["ioc_type"],
        PrimitiveValue::Text(ioc.ioc_type().as_str().to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text(ioc.value().value().to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(ioc.created_at().value())
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(ioc.updated_at().value())
    );
}
