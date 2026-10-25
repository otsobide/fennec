use kernel::sighting::domain::entities::sighting::Sighting;
use kernel::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use kernel::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use kernel::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use kernel::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

use crate::src::sighting::domain::value_objects::mothers::sighting_count_mother::SightingCountMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_id_mother::SightingObservableIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_kind_mother::SightingObservableKindMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_source_id_mother::SightingSourceIdMother;

pub struct SightingMother;

#[allow(dead_code)]
impl SightingMother {
    pub fn random() -> Sighting {
        let created_at = SightingCreatedAt::now();
        let observed_at = created_at.value();
        let first_seen = SightingFirstSeen::from_system_time(observed_at);
        let last_seen = SightingLastSeen::from_system_time(observed_at);
        let updated_at = SightingUpdatedAt::from_system_time(observed_at);

        Sighting::new(
            SightingIdMother::random(),
            SightingObservableKindMother::random(),
            SightingObservableIdMother::random(),
            SightingSourceIdMother::random(),
            first_seen,
            last_seen,
            SightingCountMother::one(),
            created_at,
            updated_at,
        )
    }
}
