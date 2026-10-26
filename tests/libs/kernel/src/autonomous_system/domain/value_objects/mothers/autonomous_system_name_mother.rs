use kernel::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use uuid::Uuid;

pub struct AutonomousSystemNameMother;

#[allow(dead_code)]
impl AutonomousSystemNameMother {
    pub fn create(value: &str) -> AutonomousSystemName {
        AutonomousSystemName::new(value)
            .expect("AutonomousSystemNameMother::create received an invalid name")
    }

    pub fn random() -> AutonomousSystemName {
        Self::create(&format!("AS-NAME-{}", Uuid::new_v4().simple()))
    }
}
