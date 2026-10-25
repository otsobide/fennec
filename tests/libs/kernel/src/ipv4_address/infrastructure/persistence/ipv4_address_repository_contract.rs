//! Behaviour every [`Ipv4AddressRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;
use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_id_mother::Ipv4AddressIdMother;

pub async fn it_finds_a_saved_ipv4_address(repository: &dyn Ipv4AddressRepository) {
    let ipv4_address = Ipv4AddressMother::random();

    repository.save(&ipv4_address).await.unwrap();
    let found = repository.find_by_id(ipv4_address.id()).await.unwrap();

    assert_eq!(found, Some(ipv4_address));
}

pub async fn it_rejects_registering_the_same_address_twice(repository: &dyn Ipv4AddressRepository) {
    repository
        .save(&Ipv4AddressMother::create("198.51.100.7"))
        .await
        .unwrap();

    let result = repository
        .save(&Ipv4AddressMother::create("198.51.100.7/32"))
        .await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn Ipv4AddressRepository) {
    let found = repository
        .find_by_id(&Ipv4AddressIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_ipv4_address(repository: &dyn Ipv4AddressRepository) {
    let ipv4_address = Ipv4AddressMother::random();
    repository.save(&ipv4_address).await.unwrap();

    repository.delete(ipv4_address.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(ipv4_address.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn Ipv4AddressRepository,
) {
    let result = repository.delete(&Ipv4AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4AddressRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_address_again(repository: &dyn Ipv4AddressRepository) {
    let ipv4_address = Ipv4AddressMother::random();
    repository.save(&ipv4_address).await.unwrap();
    repository.delete(ipv4_address.id()).await.unwrap();

    let result = repository.save(&ipv4_address).await;

    assert!(result.is_ok());
}
