use std::sync::Arc;

use kernel::hostname::application::create_hostname::hostname_creator::HostnameCreator;
use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::events::hostname_created_event::HostnameCreatedEvent;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::hostname::domain::value_objects::mothers::hostname_value_mother::HostnameValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::hostname_repository_mock::HostnameRepositoryMock;

fn make_creator(repo: Arc<HostnameRepositoryMock>, bus: Arc<EventBusMock>) -> HostnameCreator {
    let repo: Arc<dyn HostnameRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    HostnameCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_hostname_under_the_id_derived_from_its_value() {
    let value = HostnameValueMother::create("mail01.corp.example");
    let expected_id = *HostnameId::from_value(&value).value();

    let repo = Arc::new(HostnameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(HostnameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(HostnameValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![HostnameCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(HostnameRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(HostnameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(HostnameRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(HostnameValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(HostnameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(HostnameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(HostnameRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(HostnameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
