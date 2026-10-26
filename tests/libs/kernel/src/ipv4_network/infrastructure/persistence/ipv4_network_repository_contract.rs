//! Behaviour every [`Ipv4NetworkRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;

use crate::src::ipv4_network::domain::entities::mothers::ipv4_network_mother::Ipv4NetworkMother;
use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_id_mother::Ipv4NetworkIdMother;

pub async fn it_finds_a_saved_ipv4_network(repository: &dyn Ipv4NetworkRepository) {
    let ipv4_network = Ipv4NetworkMother::random();

    repository.save(&ipv4_network).await.unwrap();
    let found = repository.find_by_id(ipv4_network.id()).await.unwrap();

    assert_eq!(found, Some(ipv4_network));
}

pub async fn it_rejects_registering_the_same_network_twice(repository: &dyn Ipv4NetworkRepository) {
    repository
        .save(&Ipv4NetworkMother::create("10.0.0.0/8"))
        .await
        .unwrap();

    let result = repository
        .save(&Ipv4NetworkMother::create("  10.0.0.0/8"))
        .await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn Ipv4NetworkRepository) {
    let found = repository
        .find_by_id(&Ipv4NetworkIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_ipv4_network(repository: &dyn Ipv4NetworkRepository) {
    let ipv4_network = Ipv4NetworkMother::random();
    repository.save(&ipv4_network).await.unwrap();

    repository.delete(ipv4_network.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(ipv4_network.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn Ipv4NetworkRepository,
) {
    let result = repository.delete(&Ipv4NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4NetworkRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_network_again(repository: &dyn Ipv4NetworkRepository) {
    let ipv4_network = Ipv4NetworkMother::random();
    repository.save(&ipv4_network).await.unwrap();
    repository.delete(ipv4_network.id()).await.unwrap();

    let result = repository.save(&ipv4_network).await;

    assert!(result.is_ok());
}
