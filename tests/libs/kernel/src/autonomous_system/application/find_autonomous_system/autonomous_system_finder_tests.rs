use std::sync::Arc;

use kernel::autonomous_system::application::find_autonomous_system::autonomous_system_finder::AutonomousSystemFinder;
use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;
use crate::src::mocks::autonomous_system_repository_mock::AutonomousSystemRepositoryMock;

fn make_finder(repo: Arc<AutonomousSystemRepositoryMock>) -> AutonomousSystemFinder {
    let repo: Arc<dyn AutonomousSystemRepository> = repo;
    AutonomousSystemFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_autonomous_system_when_it_exists() {
    let autonomous_system = AutonomousSystemMother::random();
    let expected = autonomous_system.clone();
    let repo =
        Arc::new(AutonomousSystemRepositoryMock::that_returns_autonomous_system(autonomous_system));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_autonomous_system_does_not_exist() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(AutonomousSystemIdMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(AutonomousSystemIdMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
}
