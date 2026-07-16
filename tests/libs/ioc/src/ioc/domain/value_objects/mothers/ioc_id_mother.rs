use ioc::ioc::domain::value_objects::ioc_id::IocId;
use uuid::Uuid;

pub struct IocIdMother;

#[allow(dead_code)]
impl IocIdMother {
    pub fn create(value: Uuid) -> IocId {
        IocId::from_uuid(value)
    }

    pub fn random() -> IocId {
        IocId::from_uuid(Uuid::new_v4())
    }
}
