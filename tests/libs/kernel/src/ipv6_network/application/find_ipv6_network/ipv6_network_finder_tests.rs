use std::sync::Arc;

use kernel::ipv6_network::application::find_ipv6_network::ipv6_network_finder::Ipv6NetworkFinder;
use kernel::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;

use crate::src::ipv6_network::domain::entities::mothers::ipv6_network_mother::Ipv6NetworkMother;
use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_id_mother::Ipv6NetworkIdMother;
use crate::src::mocks::ipv6_network_repository_mock::Ipv6NetworkRepositoryMock;

fn make_finder(repo: Arc<Ipv6NetworkRepositoryMock>) -> Ipv6NetworkFinder {
    let repo: Arc<dyn Ipv6NetworkRepository> = repo;
    Ipv6NetworkFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_ipv6_network_when_it_exists() {
    let ipv6_network = Ipv6NetworkMother::random();
    let expected = ipv6_network.clone();
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_returns_ipv6_network(
        ipv6_network,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv6_network_does_not_exist() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(Ipv6NetworkIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6NetworkRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(Ipv6NetworkRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(Ipv6NetworkIdMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::Unexpected(_))
    ));
}
