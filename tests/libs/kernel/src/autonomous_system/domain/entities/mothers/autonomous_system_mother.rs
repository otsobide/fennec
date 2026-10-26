use kernel::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use kernel::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;

use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_value_mother::AutonomousSystemValueMother;

pub struct AutonomousSystemMother;

#[allow(dead_code)]
impl AutonomousSystemMother {
    pub fn create(value: &str) -> AutonomousSystem {
        AutonomousSystem::new(
            AutonomousSystemValueMother::create(value),
            AutonomousSystemCreatedAt::now(),
        )
    }

    pub fn random() -> AutonomousSystem {
        AutonomousSystem::new(
            AutonomousSystemValueMother::random(),
            AutonomousSystemCreatedAt::now(),
        )
    }
}
