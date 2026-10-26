use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use kernel::email_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_email_address_repository::SqlxPostgresEmailAddressRepository;
use sqlx::PgPool;

use crate::src::email_address::domain::entities::mothers::email_address_mother::EmailAddressMother;
use crate::src::email_address::domain::value_objects::mothers::email_address_id_mother::EmailAddressIdMother;
use crate::src::email_address::infrastructure::persistence::email_address_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_email_address(pool: PgPool) {
    contract::it_finds_a_saved_email_address(&SqlxPostgresEmailAddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_address_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_address_twice(
        &SqlxPostgresEmailAddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresEmailAddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_email_address(pool: PgPool) {
    contract::it_deletes_a_saved_email_address(&SqlxPostgresEmailAddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresEmailAddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_address_again(pool: PgPool) {
    contract::it_accepts_a_deleted_address_again(&SqlxPostgresEmailAddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = EmailAddressIdMother::create("john@example.com");
    sqlx::query("INSERT INTO kernel.email_address (id, value, created_at) VALUES ($1, $2, now())")
        .bind(*tampered.value())
        .bind("first.last+tag@mail.example.org")
        .execute(&pool)
        .await
        .unwrap();

    let result = SqlxPostgresEmailAddressRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(EmailAddressRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresEmailAddressRepository::new(pool.clone());
    repository
        .save(&EmailAddressMother::create(
            "First.Last+Tag@MAIL.example.org",
        ))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value FROM kernel.email_address")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "first.last+tag@mail.example.org");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_non_canonical_value_in_the_table(pool: PgPool) {
    let result = sqlx::query(
        "INSERT INTO kernel.email_address (id, value, created_at) VALUES ($1, $2, now())",
    )
    .bind(*EmailAddressIdMother::create("john@example.com").value())
    .bind("John@Example.COM")
    .execute(&pool)
    .await;

    let error = result.expect_err("the table must refuse a non-canonical value");
    assert_eq!(
        error.as_database_error().and_then(|e| e.constraint()),
        Some("email_address_value_is_canonical")
    );
}
