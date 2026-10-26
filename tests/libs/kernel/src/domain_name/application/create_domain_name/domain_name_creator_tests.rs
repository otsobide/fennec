use std::sync::Arc;

use kernel::domain_name::application::create_domain_name::domain_name_creator::DomainNameCreator;
use kernel::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use kernel::domain_name::domain::events::domain_name_created_event::DomainNameCreatedEvent;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::domain_name::domain::value_objects::mothers::domain_name_value_mother::DomainNameValueMother;
use crate::src::mocks::domain_name_repository_mock::DomainNameRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_creator(repo: Arc<DomainNameRepositoryMock>, bus: Arc<EventBusMock>) -> DomainNameCreator {
    let repo: Arc<dyn DomainNameRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    DomainNameCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_domain_name_under_the_id_derived_from_its_value() {
    let value = DomainNameValueMother::create("hostelhotels.net");
    let expected_id = *DomainNameId::from_value(&value).value();

    let repo = Arc::new(DomainNameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(DomainNameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(DomainNameValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![DomainNameCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(DomainNameRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(DomainNameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(DomainNameRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(DomainNameValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(DomainNameRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(DomainNameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(DomainNameRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(DomainNameValueMother::random()).await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
