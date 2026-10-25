//! Embedded migrations of the `kernel` schema.

use sqlx::migrate::Migrator;

/// The migrations that create the `kernel` schema's tables, one per
/// aggregate kind.
///
/// They are tracked in `kernel._sqlx_migrations`, apart from the event
/// store's migrations, so both migrators run independently on the kernel
/// database.
pub static KERNEL_MIGRATOR: Migrator = sqlx::migrate!();
