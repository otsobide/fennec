//! Behaviour every [`MacAddressRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;

use crate::src::mac_address::domain::entities::mothers::mac_address_mother::MacAddressMother;
use crate::src::mac_address::domain::value_objects::mothers::mac_address_id_mother::MacAddressIdMother;

pub async fn it_finds_a_saved_mac_address(repository: &dyn MacAddressRepository) {
    let mac_address = MacAddressMother::random();

    repository.save(&mac_address).await.unwrap();
    let found = repository.find_by_id(mac_address.id()).await.unwrap();

    assert_eq!(found, Some(mac_address));
}

pub async fn it_rejects_registering_the_same_address_twice(repository: &dyn MacAddressRepository) {
    repository
        .save(&MacAddressMother::create("02:00:5e:10:00:00"))
        .await
        .unwrap();

    let result = repository
        .save(&MacAddressMother::create("02005E100000"))
        .await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn MacAddressRepository) {
    let found = repository
        .find_by_id(&MacAddressIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_mac_address(repository: &dyn MacAddressRepository) {
    let mac_address = MacAddressMother::random();
    repository.save(&mac_address).await.unwrap();

    repository.delete(mac_address.id()).await.unwrap();

    assert_eq!(repository.find_by_id(mac_address.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn MacAddressRepository,
) {
    let result = repository.delete(&MacAddressIdMother::random()).await;

    assert!(matches!(result, Err(MacAddressRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_address_again(repository: &dyn MacAddressRepository) {
    let mac_address = MacAddressMother::random();
    repository.save(&mac_address).await.unwrap();
    repository.delete(mac_address.id()).await.unwrap();

    let result = repository.save(&mac_address).await;

    assert!(result.is_ok());
}
