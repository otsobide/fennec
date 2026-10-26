use std::sync::Arc;

use kernel::ipv6_network::application::delete_ipv6_network::ipv6_network_deleter::Ipv6NetworkDeleter;
use kernel::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use kernel::ipv6_network::domain::events::ipv6_network_deleted_event::Ipv6NetworkDeletedEvent;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv6_network::domain::entities::mothers::ipv6_network_mother::Ipv6NetworkMother;
use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_id_mother::Ipv6NetworkIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv6_network_repository_mock::Ipv6NetworkRepositoryMock;

fn make_deleter(
    repo: Arc<Ipv6NetworkRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv6NetworkDeleter {
    let repo: Arc<dyn Ipv6NetworkRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv6NetworkDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_ipv6_network() {
    let ipv6_network = Ipv6NetworkMother::random();
    let id = *ipv6_network.id();
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_returns_ipv6_network(
        ipv6_network,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let ipv6_network = Ipv6NetworkMother::random();
    let id = *ipv6_network.id();
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_returns_ipv6_network(
        ipv6_network,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv6NetworkDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv6_network_does_not_exist() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(Ipv6NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6NetworkRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_ipv6_network_does_not_exist() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(Ipv6NetworkIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let ipv6_network = Ipv6NetworkMother::random();
    let id = *ipv6_network.id();
    let repo = Arc::new(
        Ipv6NetworkRepositoryMock::that_returns_ipv6_network_but_delete_fails(
            ipv6_network,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
