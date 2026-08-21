use kernel::sighting::domain::value_objects::sighting_count::SightingCount;

pub struct SightingCountMother;

#[allow(dead_code)]
impl SightingCountMother {
    pub fn one() -> SightingCount {
        SightingCount::one()
    }

    pub fn of(value: u64) -> SightingCount {
        SightingCount::from_u64(value).expect("SightingCountMother::of received invalid value")
    }
}
