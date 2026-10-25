//! Behaviour every [`UrlSourceRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::url_source::domain::entities::url_source::UrlSource;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use kernel::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;

use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_format_mother::UrlSourceFormatMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_polling_interval_mother::UrlSourcePollingIntervalMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_url_mother::UrlSourceUrlMother;

fn edited(url_source: &UrlSource) -> UrlSource {
    UrlSource::new(
        url_source.id().clone(),
        UrlSourceUrlMother::create("https://example.test/rotated.json"),
        UrlSourceFormatMother::json(),
        UrlSourcePollingIntervalMother::create(4_294_967_295),
        url_source.created_at().clone(),
        UrlSourceUpdatedAt::now(),
    )
}

pub async fn it_finds_a_saved_url_source(repository: &dyn UrlSourceRepository) {
    let url_source = UrlSourceMother::random();

    repository.save(&url_source).await.unwrap();

    assert_eq!(
        repository.find_by_id(url_source.id()).await.unwrap(),
        Some(url_source)
    );
}

pub async fn it_rejects_saving_the_same_id_twice(repository: &dyn UrlSourceRepository) {
    let url_source = UrlSourceMother::random();
    repository.save(&url_source).await.unwrap();

    let result = repository.save(&url_source).await;

    assert!(matches!(
        result,
        Err(UrlSourceRepositoryError::AlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn UrlSourceRepository) {
    assert_eq!(
        repository
            .find_by_id(&UrlSourceIdMother::random())
            .await
            .unwrap(),
        None
    );
}

pub async fn it_updates_a_saved_url_source(repository: &dyn UrlSourceRepository) {
    let url_source = UrlSourceMother::random();
    repository.save(&url_source).await.unwrap();
    let updated = edited(&url_source);

    repository.update(&updated).await.unwrap();

    assert_eq!(
        repository.find_by_id(url_source.id()).await.unwrap(),
        Some(updated)
    );
}

pub async fn it_returns_not_found_when_updating_an_unknown_url_source(
    repository: &dyn UrlSourceRepository,
) {
    let result = repository.update(&UrlSourceMother::random()).await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::NotFound)));
}

pub async fn it_deletes_a_saved_url_source(repository: &dyn UrlSourceRepository) {
    let url_source = UrlSourceMother::random();
    repository.save(&url_source).await.unwrap();

    repository.delete(url_source.id()).await.unwrap();

    assert_eq!(repository.find_by_id(url_source.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn UrlSourceRepository,
) {
    let result = repository.delete(&UrlSourceIdMother::random()).await;

    assert!(matches!(result, Err(UrlSourceRepositoryError::NotFound)));
}
