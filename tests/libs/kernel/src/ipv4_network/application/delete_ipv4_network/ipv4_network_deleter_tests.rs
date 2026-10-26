use std::sync::Arc;

use kernel::ipv4_network::application::delete_ipv4_network::ipv4_network_deleter::Ipv4NetworkDeleter;
use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::events::ipv4_network_deleted_event::Ipv4NetworkDeletedEvent;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv4_network::domain::entities::mothers::ipv4_network_mother::Ipv4NetworkMother;
use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_id_mother::Ipv4NetworkIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv4_network_repository_mock::Ipv4NetworkRepositoryMock;

fn make_deleter(
    repo: Arc<Ipv4NetworkRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv4NetworkDeleter {
    let repo: Arc<dyn Ipv4NetworkRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv4NetworkDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_ipv4_network() {
    let ipv4_network = Ipv4NetworkMother::random();
    let id = *ipv4_network.id();
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_returns_ipv4_network(
        ipv4_network,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let ipv4_network = Ipv4NetworkMother::random();
    let id = *ipv4_network.id();
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_returns_ipv4_network(
        ipv4_network,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv4NetworkDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv4_network_does_not_exist() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(Ipv4NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4NetworkRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_ipv4_network_does_not_exist() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(Ipv4NetworkIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let ipv4_network = Ipv4NetworkMother::random();
    let id = *ipv4_network.id();
    let repo = Arc::new(
        Ipv4NetworkRepositoryMock::that_returns_ipv4_network_but_delete_fails(
            ipv4_network,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
