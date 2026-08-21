use std::sync::Arc;

use kernel::url_source::application::find_url_source::url_source_finder::UrlSourceFinder;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;

use crate::src::mocks::url_source_repository_mock::UrlSourceRepositoryMock;
use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;

fn make_finder(repo: Arc<UrlSourceRepositoryMock>) -> UrlSourceFinder {
    let repo: Arc<dyn UrlSourceRepository> = repo;
    UrlSourceFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_url_source_does_not_exist() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(UrlSourceIdMother::random()).await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_url_source_when_it_exists() {
    let url_source = UrlSourceMother::random();
    let expected_id = url_source.id().clone();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source(url_source));
    let finder = make_finder(repo);

    let result = finder.execute(expected_id.clone()).await.unwrap();

    assert_eq!(result.id().value(), expected_id.value());
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(UrlSourceIdMother::random()).await;

    assert!(matches!(
        result,
        Err(UrlSourceRepositoryError::Unexpected(_))
    ));
}
