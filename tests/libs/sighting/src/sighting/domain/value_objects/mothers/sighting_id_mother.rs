use sighting::sighting::domain::value_objects::sighting_id::SightingId;
use uuid::Uuid;

pub struct SightingIdMother;

#[allow(dead_code)]
impl SightingIdMother {
    pub fn create(value: Uuid) -> SightingId {
        SightingId::from_uuid(value)
    }

    pub fn random() -> SightingId {
        SightingId::from_uuid(Uuid::new_v4())
    }
}
