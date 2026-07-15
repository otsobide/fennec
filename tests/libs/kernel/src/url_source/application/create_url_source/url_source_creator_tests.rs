use std::sync::Arc;

use kernel::url_source::application::create_url_source::url_source_creator::UrlSourceCreator;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::events::url_source_created_event::UrlSourceCreatedEvent;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::url_source_repository_mock::UrlSourceRepositoryMock;
use crate::src::url_source::domain::value_objects::mothers::url_source_format_mother::UrlSourceFormatMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_polling_interval_mother::UrlSourcePollingIntervalMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_url_mother::UrlSourceUrlMother;

fn make_creator(
    repo: Arc<UrlSourceRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> UrlSourceCreator {
    let repo: Arc<dyn UrlSourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UrlSourceCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_url_source() {
    let id = UrlSourceIdMother::random();
    let expected_id = *id.value();

    let repo = Arc::new(UrlSourceRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            id,
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::random(),
            UrlSourcePollingIntervalMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::random(),
            UrlSourcePollingIntervalMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UrlSourceCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::random(),
            UrlSourcePollingIntervalMother::random(),
        )
        .await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::AlreadyExists)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::random(),
            UrlSourcePollingIntervalMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
