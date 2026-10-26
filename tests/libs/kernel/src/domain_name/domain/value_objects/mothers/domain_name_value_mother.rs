use kernel::domain_name::domain::value_objects::domain_name_value::DomainNameValue;
use uuid::Uuid;

pub struct DomainNameValueMother;

#[allow(dead_code)]
impl DomainNameValueMother {
    pub fn create(value: &str) -> DomainNameValue {
        DomainNameValue::new(value).expect("DomainNameValueMother::create received an invalid name")
    }

    pub fn random() -> DomainNameValue {
        Self::create(&format!("d{}.example", Uuid::new_v4().simple()))
    }
}
