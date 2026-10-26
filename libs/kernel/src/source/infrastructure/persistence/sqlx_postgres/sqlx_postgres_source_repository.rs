//! PostgreSQL implementation of [`SourceRepository`].

use std::time::SystemTime;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::event_payload_json::to_offset_date_time;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_created_at::SourceCreatedAt;
use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_type::SourceType;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "source";

const INSERT: &str = "
    INSERT INTO kernel.source (id, source_type, status, description, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "
    SELECT id, source_type, status, description, created_at, updated_at
    FROM kernel.source
    WHERE id = $1";

const UPDATE: &str = "
    UPDATE kernel.source
    SET source_type = $2, status = $3, description = $4, updated_at = $5
    WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.source WHERE id = $1";

type SourceRow = (Uuid, String, String, String, OffsetDateTime, OffsetDateTime);

/// [`SourceRepository`] backed by the `kernel.source` table.
///
/// Writes run on the current unit of work after taking the source's
/// advisory lock, so an update reads and writes the committed state and
/// commits with the command's events. Rows are rebuilt through the value
/// objects; a row that fails validation is reported as `Unexpected`.
pub struct SqlxPostgresSourceRepository {
    session: PgSession,
}

impl SqlxPostgresSourceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl SourceRepository for SqlxPostgresSourceRepository {
    async fn save(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        let id = *source.id().value();
        let source_type = source.source_type().as_str();
        let status = source.status().as_str();
        let description = source.description().value().to_string();
        let created_at = timestamp(source.created_at().value())?;
        let updated_at = timestamp(source.updated_at().value())?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(INSERT)
                        .bind(id)
                        .bind(source_type)
                        .bind(status)
                        .bind(description)
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
            return Err(SourceRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &SourceId) -> Result<Option<Source>, SourceRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, SourceRow>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_optional(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(to_aggregate).transpose()
    }

    async fn update(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        let id = *source.id().value();
        let source_type = source.source_type().as_str();
        let status = source.status().as_str();
        let description = source.description().value().to_string();
        let updated_at = timestamp(source.updated_at().value())?;

        let updated = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(UPDATE)
                        .bind(id)
                        .bind(source_type)
                        .bind(status)
                        .bind(description)
                        .bind(updated_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if updated == 0 {
            return Err(SourceRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: &SourceId) -> Result<(), SourceRepositoryError> {
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
            return Err(SourceRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn timestamp(value: SystemTime) -> Result<OffsetDateTime, SourceRepositoryError> {
    to_offset_date_time(value).map_err(SourceRepositoryError::Unexpected)
}

fn to_aggregate(row: SourceRow) -> Result<Source, SourceRepositoryError> {
    let (id, source_type, status, description, created_at, updated_at) = row;
    let invalid = |error: ValueObjectValidationError| {
        SourceRepositoryError::Unexpected(format!("kernel.source row {id} is invalid: {error}"))
    };

    Ok(Source::new(
        SourceId::from_uuid(id).map_err(invalid)?,
        SourceType::from_str(&source_type).map_err(invalid)?,
        SourceStatus::from_str(&status).map_err(invalid)?,
        SourceDescription::new(description).map_err(invalid)?,
        SourceCreatedAt::from_system_time(created_at.into()),
        SourceUpdatedAt::from_system_time(updated_at.into()),
    ))
}

fn unexpected(error: sqlx::Error) -> SourceRepositoryError {
    SourceRepositoryError::Unexpected(error.to_string())
}
