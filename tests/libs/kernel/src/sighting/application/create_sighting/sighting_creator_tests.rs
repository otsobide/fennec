use std::sync::Arc;
use std::time::SystemTime;

use shared_domain_events::domain::event_bus::EventBus;
use kernel::sighting::application::create_sighting::sighting_creator::SightingCreator;
use kernel::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use kernel::sighting::domain::events::sighting_created_event::SightingCreatedEvent;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::sighting_repository_mock::SightingRepositoryMock;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_ioc_id_mother::SightingIocIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_source_id_mother::SightingSourceIdMother;

fn make_creator(repo: Arc<SightingRepositoryMock>, bus: Arc<EventBusMock>) -> SightingCreator {
    let repo: Arc<dyn SightingRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SightingCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_sighting() {
    let id = SightingIdMother::random();
    let expected_id = *id.value();

    let repo = Arc::new(SightingRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            id,
            SightingIocIdMother::random(),
            SightingSourceIdMother::random(),
            SystemTime::now(),
        )
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(SightingRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            SightingIdMother::random(),
            SightingIocIdMother::random(),
            SightingSourceIdMother::random(),
            SystemTime::now(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SightingCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_id_already_exists_error() {
    let repo = Arc::new(SightingRepositoryMock::that_fails_with_id_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            SightingIdMother::random(),
            SightingIocIdMother::random(),
            SightingSourceIdMother::random(),
            SystemTime::now(),
        )
        .await;

    assert!(matches!(
        result,
        Err(SightingRepositoryError::IdAlreadyExists)
    ));
}

#[tokio::test]
async fn it_propagates_pair_already_exists_with_existing_id() {
    let existing_id = SightingIdMother::random();
    let expected_existing = existing_id.clone();

    let repo = Arc::new(SightingRepositoryMock::that_fails_with_pair_already_exists(
        existing_id,
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            SightingIdMother::random(),
            SightingIocIdMother::random(),
            SightingSourceIdMother::random(),
            SystemTime::now(),
        )
        .await;

    match result {
        Err(SightingRepositoryError::PairAlreadyExists { existing_id }) => {
            assert_eq!(existing_id.value(), expected_existing.value());
        }
        other => panic!("expected PairAlreadyExists, got {other:?}"),
    }
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(SightingRepositoryMock::that_fails_with_id_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            SightingIdMother::random(),
            SightingIocIdMother::random(),
            SightingSourceIdMother::random(),
            SystemTime::now(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
