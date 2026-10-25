use std::sync::Arc;

use kernel::sighting::application::list_sightings_by_observable::sightings_by_observable_lister::SightingsByObservableLister;
use kernel::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;

use crate::src::mocks::sighting_repository_mock::SightingRepositoryMock;
use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_id_mother::SightingObservableIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_kind_mother::SightingObservableKindMother;

fn make_lister(repo: Arc<SightingRepositoryMock>) -> SightingsByObservableLister {
    let repo: Arc<dyn SightingRepository> = repo;
    SightingsByObservableLister::new(repo)
}

#[tokio::test]
async fn it_returns_the_sightings_from_the_repository() {
    let a = SightingMother::random();
    let b = SightingMother::random();
    let expected = vec![a.id().value().to_string(), b.id().value().to_string()];

    let repo = Arc::new(SightingRepositoryMock::that_returns_sightings_for_observable(vec![a, b]));
    let lister = make_lister(repo);

    let result = lister
        .execute(
            SightingObservableKindMother::random(),
            SightingObservableIdMother::random(),
        )
        .await
        .unwrap();

    let ids: Vec<String> = result.iter().map(|s| s.id().value().to_string()).collect();
    assert_eq!(ids, expected);
}

#[tokio::test]
async fn it_returns_empty_vec_when_the_observable_has_no_sightings() {
    let repo = Arc::new(SightingRepositoryMock::that_succeeds());
    let lister = make_lister(repo);

    let result = lister
        .execute(
            SightingObservableKindMother::random(),
            SightingObservableIdMother::random(),
        )
        .await
        .unwrap();

    assert!(result.is_empty());
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(SightingRepositoryMock::that_fails_on_find_by_observable(
        "storage error".to_string(),
    ));
    let lister = make_lister(repo);

    let result = lister
        .execute(
            SightingObservableKindMother::random(),
            SightingObservableIdMother::random(),
        )
        .await;

    assert!(matches!(
        result,
        Err(SightingRepositoryError::Unexpected(_))
    ));
}
