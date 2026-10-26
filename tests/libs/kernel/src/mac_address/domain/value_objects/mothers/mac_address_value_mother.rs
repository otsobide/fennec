use kernel::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

pub struct MacAddressValueMother;

#[allow(dead_code)]
impl MacAddressValueMother {
    pub fn create(value: &str) -> MacAddressValue {
        MacAddressValue::new(value)
            .expect("MacAddressValueMother::create received an invalid address")
    }

    pub fn random() -> MacAddressValue {
        MacAddressValue::from_octets(rand::random::<[u8; 6]>())
    }
}
