//! Behaviour every [`HostnameRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;

use crate::src::hostname::domain::entities::mothers::hostname_mother::HostnameMother;
use crate::src::hostname::domain::value_objects::mothers::hostname_id_mother::HostnameIdMother;

pub async fn it_finds_a_saved_hostname(repository: &dyn HostnameRepository) {
    let hostname = HostnameMother::random();

    repository.save(&hostname).await.unwrap();
    let found = repository.find_by_id(hostname.id()).await.unwrap();

    assert_eq!(found, Some(hostname));
}

pub async fn it_rejects_registering_the_same_hostname_twice(repository: &dyn HostnameRepository) {
    repository
        .save(&HostnameMother::create("printer-3f.lan"))
        .await
        .unwrap();

    let result = repository
        .save(&HostnameMother::create("PRINTER-3F.LAN."))
        .await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn HostnameRepository) {
    let found = repository
        .find_by_id(&HostnameIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_hostname(repository: &dyn HostnameRepository) {
    let hostname = HostnameMother::random();
    repository.save(&hostname).await.unwrap();

    repository.delete(hostname.id()).await.unwrap();

    assert_eq!(repository.find_by_id(hostname.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(repository: &dyn HostnameRepository) {
    let result = repository.delete(&HostnameIdMother::random()).await;

    assert!(matches!(result, Err(HostnameRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_hostname_again(repository: &dyn HostnameRepository) {
    let hostname = HostnameMother::random();
    repository.save(&hostname).await.unwrap();
    repository.delete(hostname.id()).await.unwrap();

    let result = repository.save(&hostname).await;

    assert!(result.is_ok());
}
