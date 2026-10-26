use kernel::sighting::infrastructure::persistence::sqlx_postgres::sqlx_postgres_sighting_repository::SqlxPostgresSightingRepository;
use sqlx::PgPool;

use crate::src::sighting::infrastructure::persistence::sighting_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_sighting(pool: PgPool) {
    contract::it_finds_a_saved_sighting(&SqlxPostgresSightingRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_the_existing_id_when_the_triple_is_taken(pool: PgPool) {
    contract::it_reports_the_existing_id_when_the_triple_is_taken(
        &SqlxPostgresSightingRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_taken_id(pool: PgPool) {
    contract::it_reports_a_taken_id(&SqlxPostgresSightingRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresSightingRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_updates_an_observed_sighting(pool: PgPool) {
    contract::it_updates_an_observed_sighting(&SqlxPostgresSightingRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_updating_an_unknown_sighting(pool: PgPool) {
    contract::it_returns_not_found_when_updating_an_unknown_sighting(
        &SqlxPostgresSightingRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_sighting_by_its_observable_and_source(pool: PgPool) {
    contract::it_finds_a_sighting_by_its_observable_and_source(
        &SqlxPostgresSightingRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lists_the_sightings_of_one_observable(pool: PgPool) {
    contract::it_lists_the_sightings_of_one_observable(&SqlxPostgresSightingRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_keeps_observables_of_different_kinds_apart(pool: PgPool) {
    contract::it_keeps_observables_of_different_kinds_apart(&SqlxPostgresSightingRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lists_the_sightings_of_one_source(pool: PgPool) {
    contract::it_lists_the_sightings_of_one_source(&SqlxPostgresSightingRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_sighting(pool: PgPool) {
    contract::it_deletes_a_saved_sighting(&SqlxPostgresSightingRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresSightingRepository::new(pool),
    )
    .await;
}
