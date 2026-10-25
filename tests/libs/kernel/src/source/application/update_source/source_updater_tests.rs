use std::sync::Arc;

use kernel::source::application::update_source::source_updater::SourceUpdater;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::events::source_updated_event::SourceUpdatedEvent;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::source_repository_mock::SourceRepositoryMock;
use crate::src::source::domain::entities::mothers::source_mother::SourceMother;
use crate::src::source::domain::value_objects::mothers::source_description_mother::SourceDescriptionMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;
use crate::src::source::domain::value_objects::mothers::source_status_mother::SourceStatusMother;

fn make_updater(repo: Arc<SourceRepositoryMock>, bus: Arc<EventBusMock>) -> SourceUpdater {
    let repo: Arc<dyn SourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SourceUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let source = SourceMother::random();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source(source));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            SourceIdMother::random(),
            SourceStatusMother::inactive(),
            SourceDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.update_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let source = SourceMother::random();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source(source));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            SourceIdMother::random(),
            SourceStatusMother::inactive(),
            SourceDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SourceUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_source_does_not_exist() {
    let repo = Arc::new(SourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(
            SourceIdMother::random(),
            SourceStatusMother::inactive(),
            SourceDescriptionMother::random(),
        )
        .await;

    assert!(matches!(result, Err(SourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let source = SourceMother::random();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source_but_update_fails(
        source,
    ));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(
            SourceIdMother::random(),
            SourceStatusMother::inactive(),
            SourceDescriptionMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
