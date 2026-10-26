//! Behaviour every [`EmailAddressRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;

use crate::src::email_address::domain::entities::mothers::email_address_mother::EmailAddressMother;
use crate::src::email_address::domain::value_objects::mothers::email_address_id_mother::EmailAddressIdMother;

pub async fn it_finds_a_saved_email_address(repository: &dyn EmailAddressRepository) {
    let email_address = EmailAddressMother::random();

    repository.save(&email_address).await.unwrap();
    let found = repository.find_by_id(email_address.id()).await.unwrap();

    assert_eq!(found, Some(email_address));
}

pub async fn it_rejects_registering_the_same_address_twice(
    repository: &dyn EmailAddressRepository,
) {
    repository
        .save(&EmailAddressMother::create(
            "first.last+tag@mail.example.org",
        ))
        .await
        .unwrap();

    let result = repository
        .save(&EmailAddressMother::create(
            "First.Last+Tag@MAIL.example.org",
        ))
        .await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn EmailAddressRepository) {
    let found = repository
        .find_by_id(&EmailAddressIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_email_address(repository: &dyn EmailAddressRepository) {
    let email_address = EmailAddressMother::random();
    repository.save(&email_address).await.unwrap();

    repository.delete(email_address.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(email_address.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn EmailAddressRepository,
) {
    let result = repository.delete(&EmailAddressIdMother::random()).await;

    assert!(matches!(result, Err(EmailAddressRepositoryError::NotFound)));
}

pub async fn it_accepts_a_deleted_address_again(repository: &dyn EmailAddressRepository) {
    let email_address = EmailAddressMother::random();
    repository.save(&email_address).await.unwrap();
    repository.delete(email_address.id()).await.unwrap();

    let result = repository.save(&email_address).await;

    assert!(result.is_ok());
}
