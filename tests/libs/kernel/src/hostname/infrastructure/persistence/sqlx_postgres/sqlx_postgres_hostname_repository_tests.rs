use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;
use kernel::hostname::infrastructure::persistence::sqlx_postgres::sqlx_postgres_hostname_repository::SqlxPostgresHostnameRepository;
use sqlx::PgPool;

use crate::src::hostname::domain::entities::mothers::hostname_mother::HostnameMother;
use crate::src::hostname::domain::value_objects::mothers::hostname_id_mother::HostnameIdMother;
use crate::src::hostname::infrastructure::persistence::hostname_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_hostname(pool: PgPool) {
    contract::it_finds_a_saved_hostname(&SqlxPostgresHostnameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_hostname_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_hostname_twice(&SqlxPostgresHostnameRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresHostnameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_hostname(pool: PgPool) {
    contract::it_deletes_a_saved_hostname(&SqlxPostgresHostnameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresHostnameRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_hostname_again(pool: PgPool) {
    contract::it_accepts_a_deleted_hostname_again(&SqlxPostgresHostnameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = HostnameIdMother::create("mail01.corp.example");
    sqlx::query("INSERT INTO kernel.hostname (id, value, created_at) VALUES ($1, $2, now())")
        .bind(*tampered.value())
        .bind("printer-3f.lan")
        .execute(&pool)
        .await
        .unwrap();

    let result = SqlxPostgresHostnameRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(HostnameRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresHostnameRepository::new(pool.clone());
    repository
        .save(&HostnameMother::create("PRINTER-3F.LAN."))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value FROM kernel.hostname")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "printer-3f.lan");
}
