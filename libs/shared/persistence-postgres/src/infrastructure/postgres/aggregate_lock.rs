//! The transaction-scoped advisory lock that serialises commands on one
//! aggregate.

use sha2::{Digest, Sha256};
use sqlx::PgConnection;
use uuid::Uuid;

/// The advisory lock of one aggregate, identified by its type and id.
///
/// Taking it before any statement on the aggregate serialises commands on
/// that aggregate: a second command waits until the first commits, then
/// reads the committed state, so read-modify-write needs no version check.
/// An advisory lock is not a write and assigns no transaction id, so the
/// waiting command also obtains a larger transaction id than the one it
/// waited for, which keeps the event log in per-aggregate order (see
/// `docs/architecture/event-store.md`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AggregateLock(i64);

impl AggregateLock {
    /// Derives the lock key: the first eight bytes of
    /// `SHA-256("fennec:<aggregate_type>:<aggregate_id>")`.
    pub fn new(aggregate_type: &str, aggregate_id: &Uuid) -> Self {
        let digest = Sha256::digest(format!("fennec:{aggregate_type}:{aggregate_id}"));
        let mut key = [0u8; 8];
        key.copy_from_slice(&digest[..8]);
        Self(i64::from_be_bytes(key))
    }

    /// Returns the 64-bit advisory lock key.
    pub fn key(&self) -> i64 {
        self.0
    }

    /// Takes the lock until the end of the current transaction. Taking it
    /// again in the same transaction does not block.
    ///
    /// # Errors
    ///
    /// Returns the driver error if the statement fails.
    pub async fn acquire(&self, connection: &mut PgConnection) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(self.0)
            .execute(connection)
            .await
            .map(|_| ())
    }
}
