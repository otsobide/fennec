use kernel::sighting::domain::value_objects::sighting_id::SightingId;
use uuid::Uuid;

pub struct SightingIdMother;

#[allow(dead_code)]
impl SightingIdMother {
    pub fn create(value: Uuid) -> SightingId {
        SightingId::from_uuid(value).expect("mother must build a valid UUID v4")
    }

    pub fn random() -> SightingId {
        SightingId::generate()
    }
}
