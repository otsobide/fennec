use kernel::hostname::domain::entities::hostname::Hostname;
use kernel::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

use crate::src::hostname::domain::value_objects::mothers::hostname_value_mother::HostnameValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = HostnameValueMother::create("mail01.corp.example");

    let hostname = Hostname::new(value.clone(), HostnameCreatedAt::now());

    assert_eq!(*hostname.id(), HostnameId::from_value(&value));
    assert_eq!(
        hostname.id().stix_id(),
        "hostname--38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = HostnameValueMother::random();
    let created_at = HostnameCreatedAt::now();

    let hostname = Hostname::new(value.clone(), created_at);

    assert_eq!(*hostname.value(), value);
    assert_eq!(*hostname.created_at(), created_at);
}
