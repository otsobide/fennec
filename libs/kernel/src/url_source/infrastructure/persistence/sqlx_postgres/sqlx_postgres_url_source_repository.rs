//! PostgreSQL implementation of [`UrlSourceRepository`].

use std::time::SystemTime;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::event_payload_json::to_offset_date_time;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;
use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "url_source";

const INSERT: &str = "
    INSERT INTO kernel.url_source (id, url, format, polling_interval_seconds, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "
    SELECT id, url, format, polling_interval_seconds, created_at, updated_at
    FROM kernel.url_source
    WHERE id = $1";

const UPDATE: &str = "
    UPDATE kernel.url_source
    SET url = $2, format = $3, polling_interval_seconds = $4, updated_at = $5
    WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.url_source WHERE id = $1";

type UrlSourceRow = (Uuid, String, String, i64, OffsetDateTime, OffsetDateTime);

/// [`UrlSourceRepository`] backed by the `kernel.url_source` table.
///
/// Writes run on the current unit of work after taking the url source's
/// advisory lock, so an update reads and writes the committed state and
/// commits with the command's events. Rows are rebuilt through the value
/// objects; a row that fails validation is reported as `Unexpected`.
pub struct SqlxPostgresUrlSourceRepository {
    session: PgSession,
}

impl SqlxPostgresUrlSourceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl UrlSourceRepository for SqlxPostgresUrlSourceRepository {
    async fn save(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        let id = *url_source.id().value();
        let url = url_source.url().value().to_string();
        let format = url_source.format().as_str();
        let polling_interval = i64::from(url_source.polling_interval().seconds());
        let created_at = timestamp(url_source.created_at().value())?;
        let updated_at = timestamp(url_source.updated_at().value())?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(INSERT)
                        .bind(id)
                        .bind(url)
                        .bind(format)
                        .bind(polling_interval)
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
            return Err(UrlSourceRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &UrlSourceId,
    ) -> Result<Option<UrlSource>, UrlSourceRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, UrlSourceRow>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_optional(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(to_aggregate).transpose()
    }

    async fn update(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        let id = *url_source.id().value();
        let url = url_source.url().value().to_string();
        let format = url_source.format().as_str();
        let polling_interval = i64::from(url_source.polling_interval().seconds());
        let updated_at = timestamp(url_source.updated_at().value())?;

        let updated = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(UPDATE)
                        .bind(id)
                        .bind(url)
                        .bind(format)
                        .bind(polling_interval)
                        .bind(updated_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if updated == 0 {
            return Err(UrlSourceRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: &UrlSourceId) -> Result<(), UrlSourceRepositoryError> {
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
            return Err(UrlSourceRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn timestamp(value: SystemTime) -> Result<OffsetDateTime, UrlSourceRepositoryError> {
    to_offset_date_time(value).map_err(UrlSourceRepositoryError::Unexpected)
}

fn to_aggregate(row: UrlSourceRow) -> Result<UrlSource, UrlSourceRepositoryError> {
    let (id, url, format, polling_interval, created_at, updated_at) = row;
    let invalid = |error: ValueObjectValidationError| {
        UrlSourceRepositoryError::Unexpected(format!(
            "kernel.url_source row {id} is invalid: {error}"
        ))
    };
    let polling_interval = u32::try_from(polling_interval).map_err(|_| {
        UrlSourceRepositoryError::Unexpected(format!(
            "kernel.url_source row {id} has the out-of-range polling interval {polling_interval}"
        ))
    })?;

    Ok(UrlSource::new(
        UrlSourceId::from_uuid(id).map_err(invalid)?,
        UrlSourceUrl::new(url).map_err(invalid)?,
        UrlSourceFormat::from_str(&format).map_err(invalid)?,
        UrlSourcePollingInterval::from_seconds(polling_interval).map_err(invalid)?,
        UrlSourceCreatedAt::from_system_time(created_at.into()),
        UrlSourceUpdatedAt::from_system_time(updated_at.into()),
    ))
}

fn unexpected(error: sqlx::Error) -> UrlSourceRepositoryError {
    UrlSourceRepositoryError::Unexpected(error.to_string())
}
