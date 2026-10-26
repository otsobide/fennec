use kernel::email_address::domain::entities::email_address::EmailAddress;
use kernel::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;

use crate::src::email_address::domain::value_objects::mothers::email_address_value_mother::EmailAddressValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = EmailAddressValueMother::create("john@example.com");

    let email_address = EmailAddress::new(value.clone(), EmailAddressCreatedAt::now());

    assert_eq!(*email_address.id(), EmailAddressId::from_value(&value));
    assert_eq!(
        email_address.id().stix_id(),
        "email-addr--7165e2a9-671f-585d-b1e1-ca59c671d934"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = EmailAddressValueMother::random();
    let created_at = EmailAddressCreatedAt::now();

    let email_address = EmailAddress::new(value.clone(), created_at);

    assert_eq!(*email_address.value(), value);
    assert_eq!(*email_address.created_at(), created_at);
}
