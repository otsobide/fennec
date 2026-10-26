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
use crate::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use crate::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use crate::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "autonomous_system";

const INSERT: &str = "
    INSERT INTO kernel.autonomous_system (id, value, name, rir, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "
    SELECT id, value, name, rir, created_at, updated_at
    FROM kernel.autonomous_system
    WHERE id = $1";

const UPDATE: &str = "
    UPDATE kernel.autonomous_system
    SET name = $2, rir = $3, updated_at = $4
    WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.autonomous_system WHERE id = $1";

/// [`AutonomousSystemRepository`] backed by the `kernel.autonomous_system` table.
///
/// Writes run on the current unit of work after taking the autonomous system's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its number is reported as `Unexpected`.
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
        let name = autonomous_system
            .name()
            .map(|name| name.value().to_string());
        let rir = autonomous_system.rir().map(|rir| rir.as_str());
        let created_at = to_offset_date_time(autonomous_system.created_at().unix_micros())?;
        let updated_at = to_offset_date_time(autonomous_system.updated_at().unix_micros())?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(INSERT)
                        .bind(id)
                        .bind(value)
                        .bind(name)
                        .bind(rir)
                        .bind(created_at)
                        .bind(updated_at)
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
                    sqlx::query_as::<_, Row>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_optional(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(to_aggregate).transpose()
    }

    async fn update(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        let id = *autonomous_system.id().value();
        let name = autonomous_system
            .name()
            .map(|name| name.value().to_string());
        let rir = autonomous_system.rir().map(|rir| rir.as_str());
        let updated_at = to_offset_date_time(autonomous_system.updated_at().unix_micros())?;

        let updated = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(UPDATE)
                        .bind(id)
                        .bind(name)
                        .bind(rir)
                        .bind(updated_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if updated == 0 {
            return Err(AutonomousSystemRepositoryError::NotFound);
        }
        Ok(())
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

fn to_offset_date_time(micros: u64) -> Result<OffsetDateTime, AutonomousSystemRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(micros) * 1_000)
        .map_err(|error| AutonomousSystemRepositoryError::Unexpected(error.to_string()))
}

/// A `kernel.autonomous_system` row as it is read.
type Row = (
    Uuid,
    i64,
    Option<String>,
    Option<String>,
    OffsetDateTime,
    OffsetDateTime,
);

fn to_aggregate(
    (id, value, name, rir, created_at, updated_at): Row,
) -> Result<AutonomousSystem, AutonomousSystemRepositoryError> {
    let corrupt = |detail: String| {
        AutonomousSystemRepositoryError::Unexpected(format!(
            "kernel.autonomous_system row {id} {detail}"
        ))
    };

    let number = u32::try_from(value)
        .map(AutonomousSystemValue::from_number)
        .map_err(|_| corrupt(format!("holds the number {value}, outside 0 to 4294967295")))?;
    let name = name
        .map(|name| AutonomousSystemName::new(&name))
        .transpose()
        .map_err(|error| corrupt(format!("holds an invalid name: {error}")))?;
    let rir = rir
        .map(|rir| AutonomousSystemRir::new(&rir))
        .transpose()
        .map_err(|error| corrupt(format!("holds an invalid registry: {error}")))?;
    let created_at = to_micros(created_at)
        .and_then(|micros| AutonomousSystemCreatedAt::from_unix_micros(micros).ok())
        .ok_or_else(|| corrupt("has a creation instant outside 1970 to 9999".to_string()))?;
    let updated_at = to_micros(updated_at)
        .and_then(|micros| AutonomousSystemUpdatedAt::from_unix_micros(micros).ok())
        .ok_or_else(|| corrupt("has an update instant outside 1970 to 9999".to_string()))?;

    let autonomous_system = AutonomousSystem::restore(number, name, rir, created_at, updated_at);
    if *autonomous_system.id().value() != id {
        return Err(corrupt(format!(
            "is stored under an id that its number {value} does not derive"
        )));
    }
    Ok(autonomous_system)
}

fn to_micros(instant: OffsetDateTime) -> Option<u64> {
    u64::try_from(instant.unix_timestamp_nanos() / 1_000).ok()
}

fn unexpected(error: sqlx::Error) -> AutonomousSystemRepositoryError {
    AutonomousSystemRepositoryError::Unexpected(error.to_string())
}
