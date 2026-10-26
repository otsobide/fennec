//! Behaviour every [`AutonomousSystemRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;

pub async fn it_finds_a_saved_autonomous_system(repository: &dyn AutonomousSystemRepository) {
    let autonomous_system = AutonomousSystemMother::random();

    repository.save(&autonomous_system).await.unwrap();
    let found = repository.find_by_id(autonomous_system.id()).await.unwrap();

    assert_eq!(found, Some(autonomous_system));
}

pub async fn it_rejects_registering_the_same_autonomous_system_twice(
    repository: &dyn AutonomousSystemRepository,
) {
    repository
        .save(&AutonomousSystemMother::create("64512"))
        .await
        .unwrap();

    let result = repository
        .save(&AutonomousSystemMother::create("AS0.64512"))
        .await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn AutonomousSystemRepository) {
    let found = repository
        .find_by_id(&AutonomousSystemIdMother::random())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_deletes_a_saved_autonomous_system(repository: &dyn AutonomousSystemRepository) {
    let autonomous_system = AutonomousSystemMother::random();
    repository.save(&autonomous_system).await.unwrap();

    repository.delete(autonomous_system.id()).await.unwrap();

    assert_eq!(
        repository.find_by_id(autonomous_system.id()).await.unwrap(),
        None
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn AutonomousSystemRepository,
) {
    let result = repository.delete(&AutonomousSystemIdMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::NotFound)
    ));
}

pub async fn it_accepts_a_deleted_autonomous_system_again(
    repository: &dyn AutonomousSystemRepository,
) {
    let autonomous_system = AutonomousSystemMother::random();
    repository.save(&autonomous_system).await.unwrap();
    repository.delete(autonomous_system.id()).await.unwrap();

    let result = repository.save(&autonomous_system).await;

    assert!(result.is_ok());
}
