use std::sync::Arc;

use kernel::url_source::application::delete_url_source::url_source_deleter::UrlSourceDeleter;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::events::url_source_deleted_event::UrlSourceDeletedEvent;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::url_source_repository_mock::UrlSourceRepositoryMock;
use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;

fn make_deleter(repo: Arc<UrlSourceRepositoryMock>, bus: Arc<EventBusMock>) -> UrlSourceDeleter {
    let repo: Arc<dyn UrlSourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UrlSourceDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let url_source = UrlSourceMother::random();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source(url_source));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(UrlSourceIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let url_source = UrlSourceMother::random();
    let repo = Arc::new(UrlSourceRepositoryMock::that_returns_url_source(url_source));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(UrlSourceIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UrlSourceDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_url_source_does_not_exist() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(UrlSourceIdMother::random()).await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_url_source_not_found() {
    let repo = Arc::new(UrlSourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(UrlSourceIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
