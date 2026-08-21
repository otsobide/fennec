use kernel::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use uuid::Uuid;

pub struct SightingIocIdMother;

#[allow(dead_code)]
impl SightingIocIdMother {
    pub fn create(value: Uuid) -> SightingIocId {
        SightingIocId::from_uuid(value).expect("mother must build a valid UUID v4")
    }

    pub fn random() -> SightingIocId {
        SightingIocId::generate()
    }
}
