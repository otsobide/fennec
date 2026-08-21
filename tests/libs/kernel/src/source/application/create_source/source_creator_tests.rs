use std::sync::Arc;

use kernel::source::application::create_source::source_creator::SourceCreator;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::events::source_created_event::SourceCreatedEvent;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::source_repository_mock::SourceRepositoryMock;
use crate::src::source::domain::value_objects::mothers::source_description_mother::SourceDescriptionMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;
use crate::src::source::domain::value_objects::mothers::source_status_mother::SourceStatusMother;
use crate::src::source::domain::value_objects::mothers::source_type_mother::SourceTypeMother;

fn make_creator(repo: Arc<SourceRepositoryMock>, bus: Arc<EventBusMock>) -> SourceCreator {
    let repo: Arc<dyn SourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SourceCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_source() {
    let id = SourceIdMother::random();
    let expected_id = *id.value();

    let repo = Arc::new(SourceRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            id,
            SourceTypeMother::random(),
            SourceStatusMother::random(),
            SourceDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(SourceRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            SourceIdMother::random(),
            SourceTypeMother::random(),
            SourceStatusMother::random(),
            SourceDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SourceCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error_when_source_already_exists() {
    let repo = Arc::new(SourceRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            SourceIdMother::random(),
            SourceTypeMother::random(),
            SourceStatusMother::random(),
            SourceDescriptionMother::random(),
        )
        .await;

    assert!(matches!(result, Err(SourceRepositoryError::AlreadyExists)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(SourceRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            SourceIdMother::random(),
            SourceTypeMother::random(),
            SourceStatusMother::random(),
            SourceDescriptionMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
