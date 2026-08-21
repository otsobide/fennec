use std::sync::Arc;
use std::time::SystemTime;

use shared_domain_events::domain::event_bus::EventBus;
use sighting::sighting::application::observe_sighting::sighting_observer::SightingObserver;
use sighting::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use sighting::sighting::domain::events::sighting_observed_event::SightingObservedEvent;
use sighting::sighting::domain::repositories::sighting_repository::SightingRepository;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::sighting_repository_mock::SightingRepositoryMock;
use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;

fn make_observer(repo: Arc<SightingRepositoryMock>, bus: Arc<EventBusMock>) -> SightingObserver {
    let repo: Arc<dyn SightingRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SightingObserver::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let sighting = SightingMother::random();
    let repo = Arc::new(SightingRepositoryMock::that_returns_sighting(sighting));
    let bus = Arc::new(EventBusMock::new());
    let observer = make_observer(repo.clone(), bus.clone());

    observer
        .execute(SightingIdMother::random(), SystemTime::now())
        .await
        .unwrap();

    assert_eq!(repo.update_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_an_observed_event() {
    let sighting = SightingMother::random();
    let repo = Arc::new(SightingRepositoryMock::that_returns_sighting(sighting));
    let bus = Arc::new(EventBusMock::new());
    let observer = make_observer(repo.clone(), bus.clone());

    observer
        .execute(SightingIdMother::random(), SystemTime::now())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SightingObservedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_sighting_does_not_exist() {
    let repo = Arc::new(SightingRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let observer = make_observer(repo.clone(), bus.clone());

    let result = observer
        .execute(SightingIdMother::random(), SystemTime::now())
        .await;

    assert!(matches!(result, Err(SightingRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_sighting_not_found() {
    let repo = Arc::new(SightingRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let observer = make_observer(repo.clone(), bus.clone());

    let _ = observer
        .execute(SightingIdMother::random(), SystemTime::now())
        .await;

    assert!(bus.published_event_names().is_empty());
}
