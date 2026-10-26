use sqlx::PgPool;

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_creates_the_event_log_with_a_year_of_monthly_partitions(pool: PgPool) {
    let partitions: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_inherits WHERE inhparent = 'event_store.domain_event'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(partitions, 13);
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_has_a_partition_for_the_current_month(pool: PgPool) {
    let current: Option<String> = sqlx::query_scalar(
        "SELECT to_regclass(format('event_store.domain_event_%s', to_char(now() AT TIME ZONE 'UTC', 'YYYY_MM')))::text",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(current.is_some());
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_keeps_its_migration_history_in_the_event_store_schema(pool: PgPool) {
    let own: Option<String> =
        sqlx::query_scalar("SELECT to_regclass('event_store._sqlx_migrations')::text")
            .fetch_one(&pool)
            .await
            .unwrap();
    let default: Option<String> =
        sqlx::query_scalar("SELECT to_regclass('public._sqlx_migrations')::text")
            .fetch_one(&pool)
            .await
            .unwrap();

    assert!(own.is_some());
    assert!(default.is_none());
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_ensures_partitions_idempotently(pool: PgPool) {
    sqlx::query("SELECT event_store.ensure_partitions(14)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("SELECT event_store.ensure_partitions(14)")
        .execute(&pool)
        .await
        .unwrap();

    let partitions: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_inherits WHERE inhparent = 'event_store.domain_event'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(partitions, 15);
}
