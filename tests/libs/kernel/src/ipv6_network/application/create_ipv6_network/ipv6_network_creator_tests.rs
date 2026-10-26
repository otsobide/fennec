use std::sync::Arc;

use kernel::ipv6_network::application::create_ipv6_network::ipv6_network_creator::Ipv6NetworkCreator;
use kernel::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use kernel::ipv6_network::domain::events::ipv6_network_created_event::Ipv6NetworkCreatedEvent;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_value_mother::Ipv6NetworkValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv6_network_repository_mock::Ipv6NetworkRepositoryMock;

fn make_creator(
    repo: Arc<Ipv6NetworkRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv6NetworkCreator {
    let repo: Arc<dyn Ipv6NetworkRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv6NetworkCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_ipv6_network_under_the_id_derived_from_its_value() {
    let value = Ipv6NetworkValueMother::create("2001:db8::/32");
    let expected_id = *Ipv6NetworkId::from_value(&value).value();

    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(Ipv6NetworkValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv6NetworkCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(Ipv6NetworkValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv6NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
