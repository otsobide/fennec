use std::sync::Arc;

use kernel::url_source::application::update_url_source::url_source_updater::UrlSourceUpdater;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::events::url_source_updated_event::UrlSourceUpdatedEvent;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::url_source_repository_mock::UrlSourceRepositoryMock;
use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_format_mother::UrlSourceFormatMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_polling_interval_mother::UrlSourcePollingIntervalMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_url_mother::UrlSourceUrlMother;

fn make_updater(
    repo: Arc<UrlSourceRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> UrlSourceUpdater {
    let repo: Arc<dyn UrlSourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UrlSourceUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let url_source = UrlSourceMother::random();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source(url_source));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::csv(),
            UrlSourcePollingIntervalMother::create(60),
        )
        .await
        .unwrap();

    assert_eq!(repo.update_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let url_source = UrlSourceMother::random();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source(url_source));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::csv(),
            UrlSourcePollingIntervalMother::create(60),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UrlSourceUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_url_source_does_not_exist() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::csv(),
            UrlSourcePollingIntervalMother::create(60),
        )
        .await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let url_source = UrlSourceMother::random();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source_but_update_fails(
        url_source,
    ));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::csv(),
            UrlSourcePollingIntervalMother::create(60),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
