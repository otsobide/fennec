use std::sync::Arc;

use kernel::sighting::application::find_sighting::sighting_finder::SightingFinder;
use kernel::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;

use crate::src::mocks::sighting_repository_mock::SightingRepositoryMock;
use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;

fn make_finder(repo: Arc<SightingRepositoryMock>) -> SightingFinder {
    let repo: Arc<dyn SightingRepository> = repo;
    SightingFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_sighting_does_not_exist() {
    let repo = Arc::new(SightingRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(SightingIdMother::random()).await;

    assert!(matches!(result, Err(SightingRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_sighting_when_it_exists() {
    let sighting = SightingMother::random();
    let expected_id = sighting.id().clone();
    let repo = Arc::new(SightingRepositoryMock::that_returns_sighting(sighting));
    let finder = make_finder(repo);

    let result = finder.execute(expected_id.clone()).await.unwrap();

    assert_eq!(result.id().value(), expected_id.value());
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(SightingRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(SightingIdMother::random()).await;

    assert!(matches!(
        result,
        Err(SightingRepositoryError::Unexpected(_))
    ));
}
