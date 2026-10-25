//! Embedded migrations of the `event_store` schema.

use sqlx::migrate::Migrator;

/// The migrations that create the `event_store` schema: the partitioned
/// `domain_event` log, `consumer_checkpoint` and `ensure_partitions()`.
///
/// They are tracked in `event_store._sqlx_migrations`, apart from the
/// migrations of the contexts sharing the database, so each migrator runs
/// independently.
pub static EVENT_STORE_MIGRATOR: Migrator = sqlx::migrate!();
