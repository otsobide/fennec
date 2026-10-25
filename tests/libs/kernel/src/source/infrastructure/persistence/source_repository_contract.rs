//! Behaviour every [`SourceRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::source::domain::entities::source::Source;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use kernel::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

use crate::src::source::domain::entities::mothers::source_mother::SourceMother;
use crate::src::source::domain::value_objects::mothers::source_description_mother::SourceDescriptionMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;
use crate::src::source::domain::value_objects::mothers::source_status_mother::SourceStatusMother;

fn edited(source: &Source) -> Source {
    Source::new(
        source.id().clone(),
        source.source_type().clone(),
        SourceStatusMother::inactive(),
        SourceDescriptionMother::create("Rotated feed"),
        source.created_at().clone(),
        SourceUpdatedAt::now(),
    )
}

pub async fn it_finds_a_saved_source(repository: &dyn SourceRepository) {
    let source = SourceMother::random();

    repository.save(&source).await.unwrap();

    assert_eq!(
        repository.find_by_id(source.id()).await.unwrap(),
        Some(source)
    );
}

pub async fn it_rejects_saving_the_same_id_twice(repository: &dyn SourceRepository) {
    let source = SourceMother::random();
    repository.save(&source).await.unwrap();

    let result = repository.save(&source).await;

    assert!(matches!(result, Err(SourceRepositoryError::AlreadyExists)));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn SourceRepository) {
    assert_eq!(
        repository
            .find_by_id(&SourceIdMother::random())
            .await
            .unwrap(),
        None
    );
}

pub async fn it_updates_a_saved_source(repository: &dyn SourceRepository) {
    let source = SourceMother::random();
    repository.save(&source).await.unwrap();
    let updated = edited(&source);

    repository.update(&updated).await.unwrap();

    assert_eq!(
        repository.find_by_id(source.id()).await.unwrap(),
        Some(updated)
    );
}

pub async fn it_returns_not_found_when_updating_an_unknown_source(
    repository: &dyn SourceRepository,
) {
    let result = repository.update(&SourceMother::random()).await;

    assert!(matches!(result, Err(SourceRepositoryError::NotFound)));
}

pub async fn it_deletes_a_saved_source(repository: &dyn SourceRepository) {
    let source = SourceMother::random();
    repository.save(&source).await.unwrap();

    repository.delete(source.id()).await.unwrap();

    assert_eq!(repository.find_by_id(source.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(repository: &dyn SourceRepository) {
    let result = repository.delete(&SourceIdMother::random()).await;

    assert!(matches!(result, Err(SourceRepositoryError::NotFound)));
}
