use std::sync::Arc;

use kernel::ipv4_network::application::find_ipv4_network::ipv4_network_finder::Ipv4NetworkFinder;
use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;

use crate::src::ipv4_network::domain::entities::mothers::ipv4_network_mother::Ipv4NetworkMother;
use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_id_mother::Ipv4NetworkIdMother;
use crate::src::mocks::ipv4_network_repository_mock::Ipv4NetworkRepositoryMock;

fn make_finder(repo: Arc<Ipv4NetworkRepositoryMock>) -> Ipv4NetworkFinder {
    let repo: Arc<dyn Ipv4NetworkRepository> = repo;
    Ipv4NetworkFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_ipv4_network_when_it_exists() {
    let ipv4_network = Ipv4NetworkMother::random();
    let expected = ipv4_network.clone();
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_returns_ipv4_network(
        ipv4_network,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv4_network_does_not_exist() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(Ipv4NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4NetworkRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(Ipv4NetworkRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(Ipv4NetworkIdMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::Unexpected(_))
    ));
}
