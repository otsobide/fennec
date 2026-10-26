use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;

use crate::src::domain_name::domain::value_objects::mothers::domain_name_value_mother::DomainNameValueMother;

pub struct DomainNameIdMother;

#[allow(dead_code)]
impl DomainNameIdMother {
    pub fn create(value: &str) -> DomainNameId {
        DomainNameId::from_value(&DomainNameValueMother::create(value))
    }

    pub fn random() -> DomainNameId {
        DomainNameId::from_value(&DomainNameValueMother::random())
    }
}
