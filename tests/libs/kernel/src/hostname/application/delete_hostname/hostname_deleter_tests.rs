use std::sync::Arc;

use kernel::hostname::application::delete_hostname::hostname_deleter::HostnameDeleter;
use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::events::hostname_deleted_event::HostnameDeletedEvent;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::hostname::domain::entities::mothers::hostname_mother::HostnameMother;
use crate::src::hostname::domain::value_objects::mothers::hostname_id_mother::HostnameIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::hostname_repository_mock::HostnameRepositoryMock;

fn make_deleter(repo: Arc<HostnameRepositoryMock>, bus: Arc<EventBusMock>) -> HostnameDeleter {
    let repo: Arc<dyn HostnameRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    HostnameDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_hostname() {
    let hostname = HostnameMother::random();
    let id = *hostname.id();
    let repo = Arc::new(HostnameRepositoryMock::that_returns_hostname(hostname));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let hostname = HostnameMother::random();
    let id = *hostname.id();
    let repo = Arc::new(HostnameRepositoryMock::that_returns_hostname(hostname));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![HostnameDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_hostname_does_not_exist() {
    let repo = Arc::new(HostnameRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(HostnameIdMother::random()).await;

    assert!(matches!(result, Err(HostnameRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_hostname_does_not_exist() {
    let repo = Arc::new(HostnameRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(HostnameIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let hostname = HostnameMother::random();
    let id = *hostname.id();
    let repo = Arc::new(
        HostnameRepositoryMock::that_returns_hostname_but_delete_fails(
            hostname,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
