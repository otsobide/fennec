use kernel::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

pub struct AutonomousSystemValueMother;

#[allow(dead_code)]
impl AutonomousSystemValueMother {
    pub fn create(value: &str) -> AutonomousSystemValue {
        AutonomousSystemValue::new(value)
            .expect("AutonomousSystemValueMother::create received an invalid number")
    }

    pub fn random() -> AutonomousSystemValue {
        AutonomousSystemValue::from_number(rand::random::<u32>())
    }
}
