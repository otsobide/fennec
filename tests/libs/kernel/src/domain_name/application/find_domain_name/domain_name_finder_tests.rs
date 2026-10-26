use std::sync::Arc;

use kernel::domain_name::application::find_domain_name::domain_name_finder::DomainNameFinder;
use kernel::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;

use crate::src::domain_name::domain::entities::mothers::domain_name_mother::DomainNameMother;
use crate::src::domain_name::domain::value_objects::mothers::domain_name_id_mother::DomainNameIdMother;
use crate::src::mocks::domain_name_repository_mock::DomainNameRepositoryMock;

fn make_finder(repo: Arc<DomainNameRepositoryMock>) -> DomainNameFinder {
    let repo: Arc<dyn DomainNameRepository> = repo;
    DomainNameFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_domain_name_when_it_exists() {
    let domain_name = DomainNameMother::random();
    let expected = domain_name.clone();
    let repo = Arc::new(DomainNameRepositoryMock::that_returns_domain_name(
        domain_name,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_domain_name_does_not_exist() {
    let repo = Arc::new(DomainNameRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(DomainNameIdMother::random()).await;

    assert!(matches!(result, Err(DomainNameRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(DomainNameRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(DomainNameIdMother::random()).await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::Unexpected(_))
    ));
}
