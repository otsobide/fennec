use kernel::url_source::infrastructure::persistence::sqlx_postgres::sqlx_postgres_url_source_repository::SqlxPostgresUrlSourceRepository;
use sqlx::PgPool;

use crate::src::url_source::infrastructure::persistence::url_source_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_url_source(pool: PgPool) {
    contract::it_finds_a_saved_url_source(&SqlxPostgresUrlSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_saving_the_same_id_twice(pool: PgPool) {
    contract::it_rejects_saving_the_same_id_twice(&SqlxPostgresUrlSourceRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresUrlSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_updates_a_saved_url_source(pool: PgPool) {
    contract::it_updates_a_saved_url_source(&SqlxPostgresUrlSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_updating_an_unknown_url_source(pool: PgPool) {
    contract::it_returns_not_found_when_updating_an_unknown_url_source(
        &SqlxPostgresUrlSourceRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_url_source(pool: PgPool) {
    contract::it_deletes_a_saved_url_source(&SqlxPostgresUrlSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresUrlSourceRepository::new(pool),
    )
    .await;
}
