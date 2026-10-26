//! Helpers for the e2e tests that run the API on PostgreSQL.

use shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR;
use sqlx::PgPool;

/// Applies the event store migrations; `#[sqlx::test]` already applied the
/// kernel's.
pub async fn migrate_event_store(pool: &PgPool) {
    EVENT_STORE_MIGRATOR.run(pool).await.unwrap();
}

pub async fn count(pool: &PgPool, sql: &'static str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

pub async fn event_names(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT event_name FROM event_store.domain_event ORDER BY transaction_id, position",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
