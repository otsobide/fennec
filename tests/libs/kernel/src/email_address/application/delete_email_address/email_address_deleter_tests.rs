use std::sync::Arc;

use kernel::email_address::application::delete_email_address::email_address_deleter::EmailAddressDeleter;
use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::events::email_address_deleted_event::EmailAddressDeletedEvent;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::email_address::domain::entities::mothers::email_address_mother::EmailAddressMother;
use crate::src::email_address::domain::value_objects::mothers::email_address_id_mother::EmailAddressIdMother;
use crate::src::mocks::email_address_repository_mock::EmailAddressRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_deleter(
    repo: Arc<EmailAddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> EmailAddressDeleter {
    let repo: Arc<dyn EmailAddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    EmailAddressDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_email_address() {
    let email_address = EmailAddressMother::random();
    let id = *email_address.id();
    let repo = Arc::new(EmailAddressRepositoryMock::that_returns_email_address(
        email_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let email_address = EmailAddressMother::random();
    let id = *email_address.id();
    let repo = Arc::new(EmailAddressRepositoryMock::that_returns_email_address(
        email_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![EmailAddressDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_email_address_does_not_exist() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(EmailAddressIdMother::random()).await;

    assert!(matches!(result, Err(EmailAddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_email_address_does_not_exist() {
    let repo = Arc::new(EmailAddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(EmailAddressIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let email_address = EmailAddressMother::random();
    let id = *email_address.id();
    let repo = Arc::new(
        EmailAddressRepositoryMock::that_returns_email_address_but_delete_fails(
            email_address,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
