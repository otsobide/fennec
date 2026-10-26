use std::sync::Arc;

use kernel::ipv6_address::application::create_ipv6_address::ipv6_address_creator::Ipv6AddressCreator;
use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::events::ipv6_address_created_event::Ipv6AddressCreatedEvent;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_value_mother::Ipv6AddressValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv6_address_repository_mock::Ipv6AddressRepositoryMock;

fn make_creator(
    repo: Arc<Ipv6AddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv6AddressCreator {
    let repo: Arc<dyn Ipv6AddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv6AddressCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_ipv6_address_under_the_id_derived_from_its_value() {
    let value = Ipv6AddressValueMother::create("2001:db8:85a3::8a2e:370:7334");
    let expected_id = *Ipv6AddressId::from_value(&value).value();

    let repo = Arc::new(Ipv6AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(Ipv6AddressValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv6AddressCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(Ipv6AddressValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6AddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
