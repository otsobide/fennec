//! Behaviour every [`SightingRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use std::time::Duration;

use kernel::sighting::domain::entities::sighting::Sighting;
use kernel::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;
use kernel::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use kernel::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use kernel::sighting::domain::value_objects::sighting_id::SightingId;
use kernel::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use kernel::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use kernel::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use kernel::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use kernel::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_count_mother::SightingCountMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_id_mother::SightingIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_id_mother::SightingObservableIdMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_observable_kind_mother::SightingObservableKindMother;
use crate::src::sighting::domain::value_objects::mothers::sighting_source_id_mother::SightingSourceIdMother;

fn sighting(
    id: SightingId,
    kind: SightingObservableKind,
    observable_id: SightingObservableId,
    source_id: SightingSourceId,
) -> Sighting {
    let created_at = SightingCreatedAt::now();
    Sighting::new(
        id,
        kind,
        observable_id,
        source_id,
        SightingFirstSeen::from_system_time(created_at.value()),
        SightingLastSeen::from_system_time(created_at.value()),
        SightingCountMother::one(),
        created_at.clone(),
        SightingUpdatedAt::from_system_time(created_at.value()),
    )
}

fn sorted_ids(sightings: &[Sighting]) -> Vec<String> {
    let mut ids: Vec<String> = sightings.iter().map(|s| s.id().to_string()).collect();
    ids.sort();
    ids
}

pub async fn it_finds_a_saved_sighting(repository: &dyn SightingRepository) {
    let saved = SightingMother::random();

    repository.save(&saved).await.unwrap();

    assert_eq!(
        repository.find_by_id(saved.id()).await.unwrap(),
        Some(saved)
    );
}

pub async fn it_reports_the_existing_id_when_the_triple_is_taken(
    repository: &dyn SightingRepository,
) {
    let first = SightingMother::random();
    repository.save(&first).await.unwrap();
    let second = sighting(
        SightingIdMother::random(),
        first.observable_kind().clone(),
        *first.observable_id(),
        first.source_id().clone(),
    );

    let result = repository.save(&second).await;

    match result {
        Err(SightingRepositoryError::PairAlreadyExists { existing_id }) => {
            assert_eq!(existing_id, *first.id());
        }
        other => panic!("expected PairAlreadyExists, got {other:?}"),
    }
}

pub async fn it_reports_a_taken_id(repository: &dyn SightingRepository) {
    let first = SightingMother::random();
    repository.save(&first).await.unwrap();
    let second = sighting(
        first.id().clone(),
        SightingObservableKindMother::random(),
        SightingObservableIdMother::random(),
        SightingSourceIdMother::random(),
    );

    let result = repository.save(&second).await;

    assert!(matches!(
        result,
        Err(SightingRepositoryError::IdAlreadyExists)
    ));
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn SightingRepository) {
    assert_eq!(
        repository
            .find_by_id(&SightingIdMother::random())
            .await
            .unwrap(),
        None
    );
}

pub async fn it_updates_an_observed_sighting(repository: &dyn SightingRepository) {
    let saved = SightingMother::random();
    repository.save(&saved).await.unwrap();
    let observed = saved.observe(
        saved.last_seen().value() + Duration::from_secs(3600),
        SightingUpdatedAt::now(),
    );

    repository.update(&observed).await.unwrap();

    assert_eq!(
        repository.find_by_id(saved.id()).await.unwrap(),
        Some(observed)
    );
}

pub async fn it_returns_not_found_when_updating_an_unknown_sighting(
    repository: &dyn SightingRepository,
) {
    let result = repository.update(&SightingMother::random()).await;

    assert!(matches!(result, Err(SightingRepositoryError::NotFound)));
}

pub async fn it_finds_a_sighting_by_its_observable_and_source(repository: &dyn SightingRepository) {
    let saved = SightingMother::random();
    repository.save(&saved).await.unwrap();

    let found = repository
        .find_by_observable_and_source(
            saved.observable_kind(),
            saved.observable_id(),
            saved.source_id(),
        )
        .await
        .unwrap();

    assert_eq!(found, Some(saved));
}

pub async fn it_lists_the_sightings_of_one_observable(repository: &dyn SightingRepository) {
    let kind = SightingObservableKindMother::create("ipv4_address");
    let observable_id = SightingObservableIdMother::create("abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
    let first = sighting(
        SightingIdMother::random(),
        kind.clone(),
        observable_id,
        SightingSourceIdMother::random(),
    );
    let second = sighting(
        SightingIdMother::random(),
        kind.clone(),
        observable_id,
        SightingSourceIdMother::random(),
    );
    let unrelated = SightingMother::random();
    for saved in [&first, &second, &unrelated] {
        repository.save(saved).await.unwrap();
    }

    let listed = repository
        .find_by_observable(&kind, &observable_id)
        .await
        .unwrap();

    assert_eq!(sorted_ids(&listed), sorted_ids(&[first, second]));
}

pub async fn it_keeps_observables_of_different_kinds_apart(repository: &dyn SightingRepository) {
    let observable_id = SightingObservableIdMother::create("abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
    let address = sighting(
        SightingIdMother::random(),
        SightingObservableKindMother::create("ipv4_address"),
        observable_id,
        SightingSourceIdMother::random(),
    );
    repository.save(&address).await.unwrap();

    let listed = repository
        .find_by_observable(
            &SightingObservableKindMother::create("hostname"),
            &observable_id,
        )
        .await
        .unwrap();

    assert!(listed.is_empty());
}

pub async fn it_lists_the_sightings_of_one_source(repository: &dyn SightingRepository) {
    let source_id = SightingSourceIdMother::random();
    let first = sighting(
        SightingIdMother::random(),
        SightingObservableKindMother::random(),
        SightingObservableIdMother::random(),
        source_id.clone(),
    );
    let second = sighting(
        SightingIdMother::random(),
        SightingObservableKindMother::random(),
        SightingObservableIdMother::random(),
        source_id.clone(),
    );
    let unrelated = SightingMother::random();
    for saved in [&first, &second, &unrelated] {
        repository.save(saved).await.unwrap();
    }

    let listed = repository.find_by_source_id(&source_id).await.unwrap();

    assert_eq!(sorted_ids(&listed), sorted_ids(&[first, second]));
}

pub async fn it_deletes_a_saved_sighting(repository: &dyn SightingRepository) {
    let saved = SightingMother::random();
    repository.save(&saved).await.unwrap();

    repository.delete(saved.id()).await.unwrap();

    assert_eq!(repository.find_by_id(saved.id()).await.unwrap(), None);
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(repository: &dyn SightingRepository) {
    let result = repository.delete(&SightingIdMother::random()).await;

    assert!(matches!(result, Err(SightingRepositoryError::NotFound)));
}
