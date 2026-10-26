use std::sync::Arc;

use kernel::email_address::application::create_email_address::email_address_creator::EmailAddressCreator;
use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::events::email_address_created_event::EmailAddressCreatedEvent;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::email_address::domain::value_objects::mothers::email_address_value_mother::EmailAddressValueMother;
use crate::src::mocks::email_address_repository_mock::EmailAddressRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_creator(
    repo: Arc<EmailAddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> EmailAddressCreator {
    let repo: Arc<dyn EmailAddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    EmailAddressCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_email_address_under_the_id_derived_from_its_value() {
    let value = EmailAddressValueMother::create("john@example.com");
    let expected_id = *EmailAddressId::from_value(&value).value();

    let repo = Arc::new(EmailAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(EmailAddressValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![EmailAddressCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(EmailAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(EmailAddressValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(EmailAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(EmailAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
