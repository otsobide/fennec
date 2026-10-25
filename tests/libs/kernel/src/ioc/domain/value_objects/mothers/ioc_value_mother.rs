use kernel::ioc::domain::value_objects::ioc_value::IocValue;
use uuid::Uuid;

pub struct IocValueMother;

#[allow(dead_code)]
impl IocValueMother {
    pub fn create(value: impl Into<String>) -> IocValue {
        IocValue::new(value).expect("IocValueMother::create received invalid value")
    }

    pub fn random() -> IocValue {
        IocValue::new(format!("value-{}", Uuid::new_v4())).expect("random value must be valid")
    }
}
