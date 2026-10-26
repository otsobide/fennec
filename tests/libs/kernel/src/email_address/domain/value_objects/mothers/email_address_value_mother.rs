use kernel::email_address::domain::value_objects::email_address_value::EmailAddressValue;
use uuid::Uuid;

pub struct EmailAddressValueMother;

#[allow(dead_code)]
impl EmailAddressValueMother {
    pub fn create(value: &str) -> EmailAddressValue {
        EmailAddressValue::new(value)
            .expect("EmailAddressValueMother::create received an invalid address")
    }

    pub fn random() -> EmailAddressValue {
        Self::create(&format!("u{}@example.com", Uuid::new_v4().simple()))
    }
}
