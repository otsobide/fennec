use std::sync::Arc;

use kernel::source::application::delete_source::source_deleter::SourceDeleter;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::events::source_deleted_event::SourceDeletedEvent;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::source_repository_mock::SourceRepositoryMock;
use crate::src::source::domain::entities::mothers::source_mother::SourceMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;

fn make_deleter(repo: Arc<SourceRepositoryMock>, bus: Arc<EventBusMock>) -> SourceDeleter {
    let repo: Arc<dyn SourceRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    SourceDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let source = SourceMother::random();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source(source));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(SourceIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let source = SourceMother::random();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source(source));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(SourceIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![SourceDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_source_does_not_exist() {
    let repo = Arc::new(SourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(SourceIdMother::random()).await;

    assert!(matches!(result, Err(SourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_source_not_found() {
    let repo = Arc::new(SourceRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(SourceIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
