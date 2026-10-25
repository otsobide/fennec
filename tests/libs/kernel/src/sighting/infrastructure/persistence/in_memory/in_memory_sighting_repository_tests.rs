use kernel::sighting::infrastructure::persistence::in_memory::in_memory_sighting_repository::InMemorySightingRepository;

use crate::src::sighting::infrastructure::persistence::sighting_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_sighting() {
    contract::it_finds_a_saved_sighting(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_reports_the_existing_id_when_the_triple_is_taken() {
    contract::it_reports_the_existing_id_when_the_triple_is_taken(
        &InMemorySightingRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_reports_a_taken_id() {
    contract::it_reports_a_taken_id(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_updates_an_observed_sighting() {
    contract::it_updates_an_observed_sighting(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_updating_an_unknown_sighting() {
    contract::it_returns_not_found_when_updating_an_unknown_sighting(
        &InMemorySightingRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_finds_a_sighting_by_its_observable_and_source() {
    contract::it_finds_a_sighting_by_its_observable_and_source(&InMemorySightingRepository::new())
        .await;
}

#[tokio::test]
async fn it_lists_the_sightings_of_one_observable() {
    contract::it_lists_the_sightings_of_one_observable(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_keeps_observables_of_different_kinds_apart() {
    contract::it_keeps_observables_of_different_kinds_apart(&InMemorySightingRepository::new())
        .await;
}

#[tokio::test]
async fn it_lists_the_sightings_of_one_source() {
    contract::it_lists_the_sightings_of_one_source(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_sighting() {
    contract::it_deletes_a_saved_sighting(&InMemorySightingRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(&InMemorySightingRepository::new())
        .await;
}
