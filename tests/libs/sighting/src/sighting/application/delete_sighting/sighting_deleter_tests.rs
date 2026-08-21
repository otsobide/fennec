use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use sighting::sighting::application::delete_sighting::sighting_deleter::SightingDeleter;
use sighting::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use sighting::sighting::domain::events::sighting_deleted_event::SightingDeletedEvent;
use sighting::sighting::domain::repositories::sighting_repository::SightingRepository;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::sighting_repository_mock::SightingRepositoryMock;
use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;

fn make_deleter(repo: Arc<SightingRepositoryMock>, bus: Arc<EventBusMock>) -> SightingDeleter {
    let repo: Arc<dyn SightingRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SightingDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let sighting = SightingMother::random();
    let repo = Arc::new(SightingRepositoryMock::that_returns_sighting(sighting));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(SightingIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let sighting = SightingMother::random();
    let repo = Arc::new(SightingRepositoryMock::that_returns_sighting(sighting));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(SightingIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SightingDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_sighting_does_not_exist() {
    let repo = Arc::new(SightingRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(SightingIdMother::random()).await;

    assert!(matches!(result, Err(SightingRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_sighting_not_found() {
    let repo = Arc::new(SightingRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(SightingIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
