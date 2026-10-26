use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use kernel::autonomous_system::infrastructure::persistence::sqlx_postgres::sqlx_postgres_autonomous_system_repository::SqlxPostgresAutonomousSystemRepository;
use sqlx::PgPool;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;
use crate::src::autonomous_system::infrastructure::persistence::autonomous_system_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_autonomous_system(pool: PgPool) {
    contract::it_finds_a_saved_autonomous_system(&SqlxPostgresAutonomousSystemRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_autonomous_system_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_autonomous_system_twice(
        &SqlxPostgresAutonomousSystemRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresAutonomousSystemRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_autonomous_system(pool: PgPool) {
    contract::it_deletes_a_saved_autonomous_system(&SqlxPostgresAutonomousSystemRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresAutonomousSystemRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_autonomous_system_again(pool: PgPool) {
    contract::it_accepts_a_deleted_autonomous_system_again(
        &SqlxPostgresAutonomousSystemRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = AutonomousSystemIdMother::create("15169");
    sqlx::query(
        "INSERT INTO kernel.autonomous_system (id, value, created_at, updated_at) VALUES ($1, $2, now(), now())",
    )
    .bind(*tampered.value())
    .bind(64_512_i64)
    .execute(&pool)
    .await
    .unwrap();

    let result = SqlxPostgresAutonomousSystemRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresAutonomousSystemRepository::new(pool.clone());
    repository
        .save(&AutonomousSystemMother::create("AS0.64512"))
        .await
        .unwrap();

    let stored: i64 = sqlx::query_scalar("SELECT value FROM kernel.autonomous_system")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, 64_512_i64);
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_number_outside_the_range_in_the_table(pool: PgPool) {
    for value in [-1_i64, 4_294_967_296] {
        let result = sqlx::query(
            "INSERT INTO kernel.autonomous_system (id, value, created_at, updated_at) VALUES ($1, $2, now(), now())",
        )
        .bind(*AutonomousSystemIdMother::create("15169").value())
        .bind(value)
        .execute(&pool)
        .await;

        let error = result.expect_err("the table must refuse the number");
        assert_eq!(
            error.as_database_error().and_then(|e| e.constraint()),
            Some("autonomous_system_value_is_in_range"),
            "{value}"
        );
    }
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_keeps_the_description_of_a_saved_autonomous_system(pool: PgPool) {
    contract::it_keeps_the_description_of_a_saved_autonomous_system(
        &SqlxPostgresAutonomousSystemRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_replaces_the_description_on_update(pool: PgPool) {
    contract::it_replaces_the_description_on_update(&SqlxPostgresAutonomousSystemRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_updating_an_unknown_autonomous_system(pool: PgPool) {
    contract::it_returns_not_found_when_updating_an_unknown_autonomous_system(
        &SqlxPostgresAutonomousSystemRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_an_unknown_registry_or_a_control_character_in_the_table(pool: PgPool) {
    for (column, value, constraint) in [
        ("rir", "IANA", "autonomous_system_rir_is_known"),
        ("name", "GOOGLE\nARIN", "autonomous_system_name_is_valid"),
        ("name", "", "autonomous_system_name_is_valid"),
    ] {
        let result = sqlx::query(sqlx::AssertSqlSafe(format!(
            "INSERT INTO kernel.autonomous_system (id, value, {column}, created_at, updated_at) VALUES ($1, 15169, $2, now(), now())"
        )))
        .bind(*AutonomousSystemIdMother::create("15169").value())
        .bind(value)
        .execute(&pool)
        .await;

        let error = result.expect_err("the table must refuse the value");
        assert_eq!(
            error.as_database_error().and_then(|e| e.constraint()),
            Some(constraint),
            "{column} = {value:?}"
        );
    }
}
