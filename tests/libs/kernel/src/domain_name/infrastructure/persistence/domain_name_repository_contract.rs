//! Behaviour every [`DomainNameRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;

use crate::src::domain_name::domain::entities::mothers::domain_name_mother::DomainNameMother;
use crate::src::domain_name::domain::value_objects::mothers::domain_name_id_mother::DomainNameIdMother;

pub async fn it_finds_a_saved_domain_name(repository: &dyn DomainNameRepository) {
    let domain_name = DomainNameMother::random();

    repository.save(&domain_name).await.unwrap();
    let found = repository.find_by_id(domain_name.id()).await.unwrap();

    assert_eq!(found, Some(domain_name));
}

pub async fn it_rejects_registering_the_same_name_twice(repository: &dyn DomainNameRepository) {
    repository
        .save(&DomainNameMother::create("update.example-cdn.net"))
        .await
        .unwrap();

    let result = repository
        .save(&DomainNameMother::create("UPDATE.Example-CDN.net."))
        .await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn DomainNameRepository) {
    let found = repository
        .find_by_id(&DomainNameIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_domain_name(repository: &dyn DomainNameRepository) {
    let domain_name = DomainNameMother::random();
    repository.save(&domain_name).await.unwrap();

    repository.delete(domain_name.id()).await.unwrap();

    assert_eq!(repository.find_by_id(domain_name.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn DomainNameRepository,
) {
    let result = repository.delete(&DomainNameIdMother::random()).await;

    assert!(matches!(result, Err(DomainNameRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_name_again(repository: &dyn DomainNameRepository) {
    let domain_name = DomainNameMother::random();
    repository.save(&domain_name).await.unwrap();
    repository.delete(domain_name.id()).await.unwrap();

    let result = repository.save(&domain_name).await;

    assert!(result.is_ok());
}
