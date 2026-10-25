use kernel::ioc::domain::value_objects::ioc_id::IocId;
use uuid::Uuid;

pub struct IocIdMother;

#[allow(dead_code)]
impl IocIdMother {
    pub fn create(value: Uuid) -> IocId {
        IocId::from_uuid(value).expect("mother must build a valid UUID v4")
    }

    pub fn random() -> IocId {
        IocId::generate()
    }
}
