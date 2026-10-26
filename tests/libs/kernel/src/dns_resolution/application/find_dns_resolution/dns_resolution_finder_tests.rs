use std::sync::Arc;

use kernel::dns_resolution::application::find_dns_resolution::dns_resolution_finder::DnsResolutionFinder;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;

fn make_finder(repo: Arc<DnsResolutionRepositoryMock>) -> DnsResolutionFinder {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    DnsResolutionFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_resolution_when_it_exists() {
    let dns_resolution = DnsResolutionMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let finder = make_finder(repo.clone());

    let found = finder.execute(*dns_resolution.id()).await.unwrap();

    assert_eq!(found, dns_resolution);
    assert_eq!(repo.found_ids(), vec![*dns_resolution.id()]);
}

#[tokio::test]
async fn it_returns_not_found_when_the_resolution_does_not_exist() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(*DnsResolutionMother::random().id()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_returns_an_error_on_storage_failure() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*DnsResolutionMother::random().id()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
