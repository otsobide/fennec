use kernel::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use uuid::Uuid;

pub struct SightingObservableIdMother;

#[allow(dead_code)]
impl SightingObservableIdMother {
    pub fn create(value: &str) -> SightingObservableId {
        SightingObservableId::new(value)
            .expect("SightingObservableIdMother::create received an invalid id")
    }

    pub fn random() -> SightingObservableId {
        SightingObservableId::from_uuid(Uuid::new_v4()).expect("a v4 UUID is a valid observable id")
    }
}
