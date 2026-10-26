use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_value_mother::AutonomousSystemValueMother;

pub struct AutonomousSystemIdMother;

#[allow(dead_code)]
impl AutonomousSystemIdMother {
    pub fn create(value: &str) -> AutonomousSystemId {
        AutonomousSystemId::from_value(&AutonomousSystemValueMother::create(value))
    }

    pub fn random() -> AutonomousSystemId {
        AutonomousSystemId::from_value(&AutonomousSystemValueMother::random())
    }
}
