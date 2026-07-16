use std::sync::Arc;

use ioc::ioc::application::find_ioc::ioc_finder::IocFinder;
use ioc::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use ioc::ioc::domain::repositories::ioc_repository::IocRepository;

use crate::src::ioc::domain::entities::mothers::ioc_mother::IocMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_id_mother::IocIdMother;
use crate::src::mocks::ioc_repository_mock::IocRepositoryMock;

fn make_finder(repo: Arc<IocRepositoryMock>) -> IocFinder {
    let repo: Arc<dyn IocRepository> = repo;
    IocFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_ioc_does_not_exist() {
    let repo = Arc::new(IocRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(IocIdMother::random()).await;

    assert!(matches!(result, Err(IocRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_ioc_when_it_exists() {
    let ioc = IocMother::random();
    let expected_id = ioc.id().clone();
    let repo = Arc::new(IocRepositoryMock::that_returns_ioc(ioc));
    let finder = make_finder(repo);

    let result = finder.execute(expected_id.clone()).await.unwrap();

    assert_eq!(result.id().value(), expected_id.value());
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(IocRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(IocIdMother::random()).await;

    assert!(matches!(result, Err(IocRepositoryError::Unexpected(_))));
}
