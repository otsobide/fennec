use kernel::domain_name::domain::entities::domain_name::DomainName;
use kernel::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;

use crate::src::domain_name::domain::value_objects::mothers::domain_name_value_mother::DomainNameValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = DomainNameValueMother::create("hostelhotels.net");

    let domain_name = DomainName::new(value.clone(), DomainNameCreatedAt::now());

    assert_eq!(*domain_name.id(), DomainNameId::from_value(&value));
    assert_eq!(
        domain_name.id().stix_id(),
        "domain-name--07f3ac51-335f-524c-918f-86f80da2bce3"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = DomainNameValueMother::random();
    let created_at = DomainNameCreatedAt::now();

    let domain_name = DomainName::new(value.clone(), created_at);

    assert_eq!(*domain_name.value(), value);
    assert_eq!(*domain_name.created_at(), created_at);
}
