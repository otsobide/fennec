use kernel::dns_resolution::infrastructure::persistence::in_memory::in_memory_dns_resolution_repository::InMemoryDnsResolutionRepository;

use crate::src::dns_resolution::infrastructure::persistence::dns_resolution_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_resolution_with_every_observation() {
    contract::it_finds_a_saved_resolution_with_every_observation(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_rejects_recording_the_same_resolution_twice() {
    contract::it_rejects_recording_the_same_resolution_twice(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_adds_an_observation_on_update() {
    contract::it_adds_an_observation_on_update(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_changes_an_observation_on_update() {
    contract::it_changes_an_observation_on_update(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_removes_an_observation_on_update() {
    contract::it_removes_an_observation_on_update(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_adds_changes_and_removes_observations_in_one_update() {
    contract::it_adds_changes_and_removes_observations_in_one_update(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_returns_not_found_when_updating_an_unknown_resolution() {
    contract::it_returns_not_found_when_updating_an_unknown_resolution(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_lists_the_resolutions_of_a_subject_by_answer() {
    contract::it_lists_the_resolutions_of_a_subject_by_answer(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_limits_the_resolutions_of_a_subject() {
    contract::it_limits_the_resolutions_of_a_subject(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_lists_the_resolutions_to_an_answer_by_subject() {
    contract::it_lists_the_resolutions_to_an_answer_by_subject(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_limits_the_resolutions_to_an_answer() {
    contract::it_limits_the_resolutions_to_an_answer(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_lists_nothing_for_an_unknown_endpoint() {
    contract::it_lists_nothing_for_an_unknown_endpoint(&InMemoryDnsResolutionRepository::new())
        .await;
}

#[tokio::test]
async fn it_deletes_a_saved_resolution() {
    contract::it_deletes_a_saved_resolution(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_accepts_a_deleted_resolution_again() {
    contract::it_accepts_a_deleted_resolution_again(&InMemoryDnsResolutionRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryDnsResolutionRepository::new(),
    )
    .await;
}
