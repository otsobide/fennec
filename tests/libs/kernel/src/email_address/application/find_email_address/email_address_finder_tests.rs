use std::sync::Arc;

use kernel::email_address::application::find_email_address::email_address_finder::EmailAddressFinder;
use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;

use crate::src::email_address::domain::entities::mothers::email_address_mother::EmailAddressMother;
use crate::src::email_address::domain::value_objects::mothers::email_address_id_mother::EmailAddressIdMother;
use crate::src::mocks::email_address_repository_mock::EmailAddressRepositoryMock;

fn make_finder(repo: Arc<EmailAddressRepositoryMock>) -> EmailAddressFinder {
    let repo: Arc<dyn EmailAddressRepository> = repo;
    EmailAddressFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_email_address_when_it_exists() {
    let email_address = EmailAddressMother::random();
    let expected = email_address.clone();
    let repo = Arc::new(EmailAddressRepositoryMock::that_returns_email_address(
        email_address,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_email_address_does_not_exist() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(EmailAddressIdMother::random()).await;

    assert!(matches!(result, Err(EmailAddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(EmailAddressIdMother::random()).await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::Unexpected(_))
    ));
}
