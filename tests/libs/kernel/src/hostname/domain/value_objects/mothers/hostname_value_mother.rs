use kernel::hostname::domain::value_objects::hostname_value::HostnameValue;
use uuid::Uuid;

pub struct HostnameValueMother;

#[allow(dead_code)]
impl HostnameValueMother {
    pub fn create(value: &str) -> HostnameValue {
        HostnameValue::new(value).expect("HostnameValueMother::create received an invalid name")
    }

    pub fn random() -> HostnameValue {
        Self::create(&format!("h{}", Uuid::new_v4().simple()))
    }
}
