use std::sync::Arc;

use kernel::ipv4_network::application::create_ipv4_network::ipv4_network_creator::Ipv4NetworkCreator;
use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::events::ipv4_network_created_event::Ipv4NetworkCreatedEvent;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_value_mother::Ipv4NetworkValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv4_network_repository_mock::Ipv4NetworkRepositoryMock;

fn make_creator(
    repo: Arc<Ipv4NetworkRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv4NetworkCreator {
    let repo: Arc<dyn Ipv4NetworkRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv4NetworkCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_ipv4_network_under_the_id_derived_from_its_value() {
    let value = Ipv4NetworkValueMother::create("198.51.100.0/24");
    let expected_id = *Ipv4NetworkId::from_value(&value).value();

    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(Ipv4NetworkValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv4NetworkCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(Ipv4NetworkValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(Ipv4NetworkValueMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
