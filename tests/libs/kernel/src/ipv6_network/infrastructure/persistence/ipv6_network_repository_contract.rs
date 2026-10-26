//! Behaviour every [`Ipv6NetworkRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;

use crate::src::ipv6_network::domain::entities::mothers::ipv6_network_mother::Ipv6NetworkMother;
use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_id_mother::Ipv6NetworkIdMother;

pub async fn it_finds_a_saved_ipv6_network(repository: &dyn Ipv6NetworkRepository) {
    let ipv6_network = Ipv6NetworkMother::random();

    repository.save(&ipv6_network).await.unwrap();
    let found = repository.find_by_id(ipv6_network.id()).await.unwrap();

    assert_eq!(found, Some(ipv6_network));
}

pub async fn it_rejects_registering_the_same_network_twice(repository: &dyn Ipv6NetworkRepository) {
    repository
        .save(&Ipv6NetworkMother::create("fd12:3456::/32"))
        .await
        .unwrap();

    let result = repository
        .save(&Ipv6NetworkMother::create("FD12:3456:0:0:0:0:0:0/32"))
        .await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn Ipv6NetworkRepository) {
    let found = repository
        .find_by_id(&Ipv6NetworkIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_ipv6_network(repository: &dyn Ipv6NetworkRepository) {
    let ipv6_network = Ipv6NetworkMother::random();
    repository.save(&ipv6_network).await.unwrap();

    repository.delete(ipv6_network.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(ipv6_network.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn Ipv6NetworkRepository,
) {
    let result = repository.delete(&Ipv6NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6NetworkRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_network_again(repository: &dyn Ipv6NetworkRepository) {
    let ipv6_network = Ipv6NetworkMother::random();
    repository.save(&ipv6_network).await.unwrap();
    repository.delete(ipv6_network.id()).await.unwrap();

    let result = repository.save(&ipv6_network).await;

    assert!(result.is_ok());
}
