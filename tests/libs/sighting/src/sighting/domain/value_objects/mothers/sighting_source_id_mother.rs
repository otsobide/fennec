use sighting::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use uuid::Uuid;

pub struct SightingSourceIdMother;

#[allow(dead_code)]
impl SightingSourceIdMother {
    pub fn create(value: Uuid) -> SightingSourceId {
        SightingSourceId::from_uuid(value)
    }

    pub fn random() -> SightingSourceId {
        SightingSourceId::from_uuid(Uuid::new_v4())
    }
}
