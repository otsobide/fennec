//! PostgreSQL implementation of [`AutonomousSystemRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "autonomous_system";

const INSERT: &str = "
    INSERT INTO kernel.autonomous_system (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str =
    "SELECT id, value, created_at FROM kernel.autonomous_system WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.autonomous_system WHERE id = $1";

/// [`AutonomousSystemRepository`] backed by the `kernel.autonomous_system` table.
///
/// Writes run on the current unit of work after taking the autonomous system's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresAutonomousSystemRepository {
    session: PgSession,
}

impl SqlxPostgresAutonomousSystemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl AutonomousSystemRepository for SqlxPostgresAutonomousSystemRepository {
    async fn save(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        let id = *autonomous_system.id().value();
        let value = i64::from(autonomous_system.value().number());
        let created_at = to_offset_date_time(autonomous_system.created_at())?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(INSERT)
                        .bind(id)
                        .bind(value)
                        .bind(created_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if inserted == 0 {
            return Err(AutonomousSystemRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &AutonomousSystemId,
    ) -> Result<Option<AutonomousSystem>, AutonomousSystemRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, (Uuid, i64, OffsetDateTime)>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_optional(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(|(id, value, created_at)| to_aggregate(id, value, created_at))
            .transpose()
    }

    async fn delete(&self, id: &AutonomousSystemId) -> Result<(), AutonomousSystemRepositoryError> {
        let id = *id.value();

        let deleted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(DELETE)
                        .bind(id)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if deleted == 0 {
            return Err(AutonomousSystemRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &AutonomousSystemCreatedAt,
) -> Result<OffsetDateTime, AutonomousSystemRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| AutonomousSystemRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: i64,
    created_at: OffsetDateTime,
) -> Result<AutonomousSystem, AutonomousSystemRepositoryError> {
    let stored = u32::try_from(value)
        .map(AutonomousSystemValue::from_number)
        .map_err(|_| {
            AutonomousSystemRepositoryError::Unexpected(format!(
            "kernel.autonomous_system row {id} holds the number {value}, outside 0 to 4294967295"
        ))
        })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        AutonomousSystemRepositoryError::Unexpected(format!(
            "kernel.autonomous_system row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = AutonomousSystemCreatedAt::from_unix_micros(micros)
        .map_err(|error| AutonomousSystemRepositoryError::Unexpected(error.to_string()))?;

    let autonomous_system = AutonomousSystem::new(stored, created_at);
    if *autonomous_system.id().value() != id {
        return Err(AutonomousSystemRepositoryError::Unexpected(format!(
            "kernel.autonomous_system row {id} is stored under an id that its number {value} does not derive"
        )));
    }
    Ok(autonomous_system)
}

fn unexpected(error: sqlx::Error) -> AutonomousSystemRepositoryError {
    AutonomousSystemRepositoryError::Unexpected(error.to_string())
}
