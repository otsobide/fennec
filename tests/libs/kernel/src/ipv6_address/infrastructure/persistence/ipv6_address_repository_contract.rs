//! Behaviour every [`Ipv6AddressRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;
use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_id_mother::Ipv6AddressIdMother;

pub async fn it_finds_a_saved_ipv6_address(repository: &dyn Ipv6AddressRepository) {
    let ipv6_address = Ipv6AddressMother::random();

    repository.save(&ipv6_address).await.unwrap();
    let found = repository.find_by_id(ipv6_address.id()).await.unwrap();

    assert_eq!(found, Some(ipv6_address));
}

pub async fn it_rejects_registering_the_same_address_twice(repository: &dyn Ipv6AddressRepository) {
    repository
        .save(&Ipv6AddressMother::create("fe80::1"))
        .await
        .unwrap();

    let result = repository
        .save(&Ipv6AddressMother::create("FE80:0:0:0:0:0:0:1/128"))
        .await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn Ipv6AddressRepository) {
    let found = repository
        .find_by_id(&Ipv6AddressIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_ipv6_address(repository: &dyn Ipv6AddressRepository) {
    let ipv6_address = Ipv6AddressMother::random();
    repository.save(&ipv6_address).await.unwrap();

    repository.delete(ipv6_address.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(ipv6_address.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn Ipv6AddressRepository,
) {
    let result = repository.delete(&Ipv6AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6AddressRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_address_again(repository: &dyn Ipv6AddressRepository) {
    let ipv6_address = Ipv6AddressMother::random();
    repository.save(&ipv6_address).await.unwrap();
    repository.delete(ipv6_address.id()).await.unwrap();

    let result = repository.save(&ipv6_address).await;

    assert!(result.is_ok());
}
