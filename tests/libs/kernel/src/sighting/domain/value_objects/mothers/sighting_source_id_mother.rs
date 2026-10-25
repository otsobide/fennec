use kernel::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use uuid::Uuid;

pub struct SightingSourceIdMother;

#[allow(dead_code)]
impl SightingSourceIdMother {
    pub fn create(value: Uuid) -> SightingSourceId {
        SightingSourceId::from_uuid(value).expect("mother must build a valid UUID v4")
    }

    pub fn random() -> SightingSourceId {
        SightingSourceId::generate()
    }
}
