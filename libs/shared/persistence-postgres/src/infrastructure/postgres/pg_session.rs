//! Where a repository runs its statements.

use sqlx::{PgConnection, PgPool};

use crate::infrastructure::postgres::aggregate_lock::AggregateLock;
use crate::infrastructure::postgres::pg_operation::PgOperation;
use crate::infrastructure::postgres::pg_unit_of_work::PgUnitOfWork;

/// The database access every PostgreSQL repository holds.
///
/// Inside a command, an operation runs on the current [`PgUnitOfWork`]'s
/// transaction, after taking the [`AggregateLock`] of the aggregate it
/// touches, so the command uses one connection, reads its own writes and
/// commits with its events. Outside a unit of work (query handlers), it runs
/// on a pooled connection with a snapshot per statement.
#[derive(Clone)]
pub struct PgSession {
    pool: PgPool,
}

impl PgSession {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Runs `operation` for the aggregate guarded by `lock`.
    ///
    /// # Errors
    ///
    /// Returns the driver error of the lock or of the operation.
    pub async fn run<T, F>(&self, lock: AggregateLock, operation: F) -> Result<T, sqlx::Error>
    where
        T: Send + 'static,
        F: for<'c> FnOnce(&'c mut PgConnection) -> PgOperation<'c, T> + Send + 'static,
    {
        match PgUnitOfWork::current() {
            Some(unit_of_work) => {
                unit_of_work
                    .execute(move |connection| {
                        Box::pin(async move {
                            lock.acquire(&mut *connection).await?;
                            operation(connection).await
                        })
                    })
                    .await
            }
            None => {
                let mut connection = self.pool.acquire().await?;
                operation(&mut connection).await
            }
        }
    }

    /// Runs `operation` without an aggregate lock, for statements that touch
    /// no single aggregate, such as listing every sighting of a source. It
    /// still runs on the current unit of work's transaction when there is
    /// one.
    ///
    /// # Errors
    ///
    /// Returns the driver error of the operation.
    pub async fn run_unlocked<T, F>(&self, operation: F) -> Result<T, sqlx::Error>
    where
        T: Send + 'static,
        F: for<'c> FnOnce(&'c mut PgConnection) -> PgOperation<'c, T> + Send + 'static,
    {
        match PgUnitOfWork::current() {
            Some(unit_of_work) => unit_of_work.execute(operation).await,
            None => {
                let mut connection = self.pool.acquire().await?;
                operation(&mut connection).await
            }
        }
    }
}
