use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

use crate::src::hostname::domain::value_objects::mothers::hostname_value_mother::HostnameValueMother;

pub struct HostnameIdMother;

#[allow(dead_code)]
impl HostnameIdMother {
    pub fn create(value: &str) -> HostnameId {
        HostnameId::from_value(&HostnameValueMother::create(value))
    }

    pub fn random() -> HostnameId {
        HostnameId::from_value(&HostnameValueMother::random())
    }
}
