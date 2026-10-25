use kernel::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;

pub struct SightingObservableKindMother;

#[allow(dead_code)]
impl SightingObservableKindMother {
    pub fn create(value: &str) -> SightingObservableKind {
        SightingObservableKind::new(value)
            .expect("SightingObservableKindMother::create received an invalid kind")
    }

    pub fn random() -> SightingObservableKind {
        Self::create("ipv4_address")
    }
}
