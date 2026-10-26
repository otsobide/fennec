use kernel::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use kernel::domain_name::infrastructure::persistence::sqlx_postgres::sqlx_postgres_domain_name_repository::SqlxPostgresDomainNameRepository;
use sqlx::PgPool;

use crate::src::domain_name::domain::entities::mothers::domain_name_mother::DomainNameMother;
use crate::src::domain_name::domain::value_objects::mothers::domain_name_id_mother::DomainNameIdMother;
use crate::src::domain_name::infrastructure::persistence::domain_name_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_domain_name(pool: PgPool) {
    contract::it_finds_a_saved_domain_name(&SqlxPostgresDomainNameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_name_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_name_twice(&SqlxPostgresDomainNameRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresDomainNameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_domain_name(pool: PgPool) {
    contract::it_deletes_a_saved_domain_name(&SqlxPostgresDomainNameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresDomainNameRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_name_again(pool: PgPool) {
    contract::it_accepts_a_deleted_name_again(&SqlxPostgresDomainNameRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = DomainNameIdMother::create("hostelhotels.net");
    sqlx::query("INSERT INTO kernel.domain_name (id, value, created_at) VALUES ($1, $2, now())")
        .bind(*tampered.value())
        .bind("update.example-cdn.net")
        .execute(&pool)
        .await
        .unwrap();

    let result = SqlxPostgresDomainNameRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresDomainNameRepository::new(pool.clone());
    repository
        .save(&DomainNameMother::create("UPDATE.Example-CDN.net."))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value FROM kernel.domain_name")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "update.example-cdn.net");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_non_canonical_value_in_the_table(pool: PgPool) {
    let result = sqlx::query(
        "INSERT INTO kernel.domain_name (id, value, created_at) VALUES ($1, $2, now())",
    )
    .bind(*DomainNameIdMother::create("hostelhotels.net").value())
    .bind("HostelHotels.NET.")
    .execute(&pool)
    .await;

    let error = result.expect_err("the table must refuse a non-canonical value");
    assert_eq!(
        error.as_database_error().and_then(|e| e.constraint()),
        Some("domain_name_value_is_canonical")
    );
}
