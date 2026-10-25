use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use kernel::ipv4_address::infrastructure::persistence::in_memory::in_memory_ipv4_address_repository::InMemoryIpv4AddressRepository;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;
use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_id_mother::Ipv4AddressIdMother;

#[tokio::test]
async fn it_finds_a_saved_ipv4_address() {
    let repository = InMemoryIpv4AddressRepository::new();
    let ipv4_address = Ipv4AddressMother::random();

    repository.save(&ipv4_address).await.unwrap();
    let found = repository.find_by_id(ipv4_address.id()).await.unwrap();

    assert_eq!(found, Some(ipv4_address));
}

#[tokio::test]
async fn it_rejects_registering_the_same_address_twice() {
    let repository = InMemoryIpv4AddressRepository::new();
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

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    let repository = InMemoryIpv4AddressRepository::new();

    let found = repository
        .find_by_id(&Ipv4AddressIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

#[tokio::test]
async fn it_deletes_a_saved_ipv4_address() {
    let repository = InMemoryIpv4AddressRepository::new();
    let ipv4_address = Ipv4AddressMother::random();
    repository.save(&ipv4_address).await.unwrap();

    repository.delete(ipv4_address.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(ipv4_address.id()).await.unwrap(),
        None
    );
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    let repository = InMemoryIpv4AddressRepository::new();

    let result = repository.delete(&Ipv4AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4AddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_accepts_a_deleted_address_again() {
    let repository = InMemoryIpv4AddressRepository::new();
    let ipv4_address = Ipv4AddressMother::random();
    repository.save(&ipv4_address).await.unwrap();
    repository.delete(ipv4_address.id()).await.unwrap();

    let result = repository.save(&ipv4_address).await;

    assert!(result.is_ok());
}
