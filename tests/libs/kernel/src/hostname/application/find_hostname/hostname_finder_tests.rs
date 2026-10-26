use std::sync::Arc;

use kernel::hostname::application::find_hostname::hostname_finder::HostnameFinder;
use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;

use crate::src::hostname::domain::entities::mothers::hostname_mother::HostnameMother;
use crate::src::hostname::domain::value_objects::mothers::hostname_id_mother::HostnameIdMother;
use crate::src::mocks::hostname_repository_mock::HostnameRepositoryMock;

fn make_finder(repo: Arc<HostnameRepositoryMock>) -> HostnameFinder {
    let repo: Arc<dyn HostnameRepository> = repo;
    HostnameFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_hostname_when_it_exists() {
    let hostname = HostnameMother::random();
    let expected = hostname.clone();
    let repo = Arc::new(HostnameRepositoryMock::that_returns_hostname(hostname));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_hostname_does_not_exist() {
    let repo = Arc::new(HostnameRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(HostnameIdMother::random()).await;

    assert!(matches!(result, Err(HostnameRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(HostnameRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(HostnameIdMother::random()).await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::Unexpected(_))
    ));
}
