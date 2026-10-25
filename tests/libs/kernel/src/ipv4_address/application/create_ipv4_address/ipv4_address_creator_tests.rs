use std::sync::Arc;

use kernel::ipv4_address::application::create_ipv4_address::ipv4_address_creator::Ipv4AddressCreator;
use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::events::ipv4_address_created_event::Ipv4AddressCreatedEvent;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_value_mother::Ipv4AddressValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv4_address_repository_mock::Ipv4AddressRepositoryMock;

fn make_creator(
    repo: Arc<Ipv4AddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv4AddressCreator {
    let repo: Arc<dyn Ipv4AddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv4AddressCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_ipv4_address_under_the_id_derived_from_its_value() {
    let value = Ipv4AddressValueMother::create("89.19.209.154");
    let expected_id = *Ipv4AddressId::from_value(&value).value();

    let repo = Arc::new(Ipv4AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(Ipv4AddressValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv4AddressCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(Ipv4AddressValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
