//! PostgreSQL implementation of [`DnsResolutionRepository`].

use std::net::IpAddr;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::{PgConnection, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use crate::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "dns_resolution";

const INSERT: &str = "
    INSERT INTO kernel.dns_resolution
        (id, subject_kind, subject_value, answer_kind, answer_value, created_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT DO NOTHING";

const EXISTS: &str = "SELECT EXISTS (SELECT 1 FROM kernel.dns_resolution WHERE id = $1)";

/// Writes every observation of a resolution in one statement: a new source
/// is inserted and a stored one is replaced.
const UPSERT_OBSERVATIONS: &str = "
    INSERT INTO kernel.dns_resolution_observation
        (dns_resolution_id, source_id, first_seen, last_seen, count, created_at, updated_at)
    SELECT $1::uuid, source_id, first_seen, last_seen, count, created_at, updated_at
    FROM UNNEST($2::uuid[], $3::timestamptz[], $4::timestamptz[], $5::bigint[], $6::timestamptz[], $7::timestamptz[])
        AS observation (source_id, first_seen, last_seen, count, created_at, updated_at)
    ON CONFLICT (dns_resolution_id, source_id) DO UPDATE
    SET first_seen = EXCLUDED.first_seen,
        last_seen = EXCLUDED.last_seen,
        count = EXCLUDED.count,
        created_at = EXCLUDED.created_at,
        updated_at = EXCLUDED.updated_at";

/// Removes the observations of the sources the aggregate no longer holds.
const DELETE_OTHER_OBSERVATIONS: &str = "
    DELETE FROM kernel.dns_resolution_observation
    WHERE dns_resolution_id = $1 AND source_id <> ALL ($2::uuid[])";

/// The observations go with the resolution through `ON DELETE CASCADE`.
const DELETE: &str = "DELETE FROM kernel.dns_resolution WHERE id = $1";

/// One resolution with its observations, one row per observation. A
/// resolution without observations still yields one row, with `NULL`
/// observation columns, so it is reported instead of hidden.
const SELECT_BY_ID: &str = "
    SELECT link.id, link.subject_kind, link.subject_value, link.answer_kind, link.answer_value,
           link.created_at, observation.source_id, observation.first_seen, observation.last_seen,
           observation.count, observation.created_at AS observation_created_at,
           observation.updated_at AS observation_updated_at
    FROM kernel.dns_resolution AS link
    LEFT JOIN kernel.dns_resolution_observation AS observation
        ON observation.dns_resolution_id = link.id
    WHERE link.id = $1
    ORDER BY observation.source_id";

/// The first `$3` resolutions of one subject by answer, with their
/// observations, in one statement and therefore one snapshot.
const LIST_BY_SUBJECT: &str = "
    WITH link AS (
        SELECT id, subject_kind, subject_value, answer_kind, answer_value, created_at
        FROM kernel.dns_resolution
        WHERE subject_kind = $1 AND subject_value = $2
        ORDER BY answer_value
        LIMIT $3
    )
    SELECT link.id, link.subject_kind, link.subject_value, link.answer_kind, link.answer_value,
           link.created_at, observation.source_id, observation.first_seen, observation.last_seen,
           observation.count, observation.created_at AS observation_created_at,
           observation.updated_at AS observation_updated_at
    FROM link
    LEFT JOIN kernel.dns_resolution_observation AS observation
        ON observation.dns_resolution_id = link.id
    ORDER BY link.answer_value, link.id, observation.source_id";

/// The first `$3` resolutions to one answer by subject, with their
/// observations, in one statement and therefore one snapshot.
const LIST_BY_ANSWER: &str = "
    WITH link AS (
        SELECT id, subject_kind, subject_value, answer_kind, answer_value, created_at
        FROM kernel.dns_resolution
        WHERE answer_kind = $1 AND answer_value = $2
        ORDER BY subject_value
        LIMIT $3
    )
    SELECT link.id, link.subject_kind, link.subject_value, link.answer_kind, link.answer_value,
           link.created_at, observation.source_id, observation.first_seen, observation.last_seen,
           observation.count, observation.created_at AS observation_created_at,
           observation.updated_at AS observation_updated_at
    FROM link
    LEFT JOIN kernel.dns_resolution_observation AS observation
        ON observation.dns_resolution_id = link.id
    ORDER BY link.subject_value, link.id, observation.source_id";

/// [`DnsResolutionRepository`] backed by the `kernel.dns_resolution` table
/// and its child table `kernel.dns_resolution_observation`.
///
/// Writes and reads by id run on the current unit of work after taking the
/// resolution's advisory lock, so a command reads the committed
/// observations, writes the link and its observations in its transaction
/// and commits them with its events. A duplicate is detected with
/// `ON CONFLICT DO NOTHING`, which never aborts the unit of work. Rows are
/// rebuilt through the value objects, and a row that is not canonical, a
/// resolution without observations and a row whose id is not the one its
/// endpoints derive are reported as `Unexpected`.
pub struct SqlxPostgresDnsResolutionRepository {
    session: PgSession,
}

impl SqlxPostgresDnsResolutionRepository {
    /// Creates a repository that runs its statements on `pool`, or on the
    /// current unit of work's transaction when there is one.
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }

    async fn list(
        &self,
        statement: &'static str,
        kind: &'static str,
        value: EndpointValue,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);

        let rows = self
            .session
            .run_unlocked(move |connection| {
                Box::pin(async move {
                    let query = sqlx::query_as::<_, Row>(statement).bind(kind);
                    match value {
                        EndpointValue::Name(name) => query.bind(name),
                        EndpointValue::Address(address) => query.bind(address),
                    }
                    .bind(limit)
                    .fetch_all(connection)
                    .await
                })
            })
            .await
            .map_err(unexpected)?;

        to_aggregates(rows)
    }
}

#[async_trait]
impl DnsResolutionRepository for SqlxPostgresDnsResolutionRepository {
    async fn save(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        let id = *dns_resolution.id().value();
        let subject_kind = dns_resolution.subject().kind();
        let subject_value = dns_resolution.subject().value().to_string();
        let answer_kind = dns_resolution.answer().kind();
        let answer_value = dns_resolution.answer().ip();
        let created_at = to_offset_date_time(dns_resolution.created_at().unix_micros())?;
        let observations = ObservationColumns::of(dns_resolution)?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    let inserted = sqlx::query(INSERT)
                        .bind(id)
                        .bind(subject_kind)
                        .bind(subject_value)
                        .bind(answer_kind)
                        .bind(answer_value)
                        .bind(created_at)
                        .execute(&mut *connection)
                        .await?
                        .rows_affected();
                    if inserted == 0 {
                        return Ok(false);
                    }
                    observations.upsert(connection, id).await?;
                    Ok(true)
                })
            })
            .await
            .map_err(unexpected)?;

        if !inserted {
            return Err(DnsResolutionRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn update(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        let id = *dns_resolution.id().value();
        let observations = ObservationColumns::of(dns_resolution)?;

        let updated = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    let exists: bool = sqlx::query_scalar(EXISTS)
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await?;
                    if !exists {
                        return Ok(false);
                    }
                    sqlx::query(DELETE_OTHER_OBSERVATIONS)
                        .bind(id)
                        .bind(&observations.source_ids)
                        .execute(&mut *connection)
                        .await?;
                    observations.upsert(connection, id).await?;
                    Ok(true)
                })
            })
            .await
            .map_err(unexpected)?;

        if !updated {
            return Err(DnsResolutionRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &DnsResolutionId,
    ) -> Result<Option<DnsResolution>, DnsResolutionRepositoryError> {
        let id = *id.value();

        let rows = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, Row>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_all(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        Ok(to_aggregates(rows)?.into_iter().next())
    }

    async fn list_by_subject(
        &self,
        subject: &DnsResolutionSubject,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        self.list(
            LIST_BY_SUBJECT,
            subject.kind(),
            EndpointValue::Name(subject.value().to_string()),
            limit,
        )
        .await
    }

    async fn list_by_answer(
        &self,
        answer: &DnsResolutionAnswer,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        self.list(
            LIST_BY_ANSWER,
            answer.kind(),
            EndpointValue::Address(answer.ip()),
            limit,
        )
        .await
    }

    async fn delete(&self, id: &DnsResolutionId) -> Result<(), DnsResolutionRepositoryError> {
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
            return Err(DnsResolutionRepositoryError::NotFound);
        }
        Ok(())
    }
}

/// The canonical value of the endpoint a list filters on: a name is `text`,
/// an address is `inet`.
enum EndpointValue {
    Name(String),
    Address(IpAddr),
}

/// The observations of a resolution as the column arrays `UNNEST` reads.
struct ObservationColumns {
    source_ids: Vec<Uuid>,
    first_seen: Vec<OffsetDateTime>,
    last_seen: Vec<OffsetDateTime>,
    counts: Vec<i64>,
    created_at: Vec<OffsetDateTime>,
    updated_at: Vec<OffsetDateTime>,
}

impl ObservationColumns {
    fn of(dns_resolution: &DnsResolution) -> Result<Self, DnsResolutionRepositoryError> {
        let observations = dns_resolution.observations();
        let mut columns = Self {
            source_ids: Vec::with_capacity(observations.len()),
            first_seen: Vec::with_capacity(observations.len()),
            last_seen: Vec::with_capacity(observations.len()),
            counts: Vec::with_capacity(observations.len()),
            created_at: Vec::with_capacity(observations.len()),
            updated_at: Vec::with_capacity(observations.len()),
        };
        for observation in observations {
            let count = observation.count().value();
            columns.source_ids.push(*observation.source_id().value());
            columns.first_seen.push(to_offset_date_time(
                observation.window().first_seen_micros(),
            )?);
            columns.last_seen.push(to_offset_date_time(
                observation.window().last_seen_micros(),
            )?);
            columns.counts.push(i64::try_from(count).map_err(|_| {
                DnsResolutionRepositoryError::Unexpected(format!(
                    "dns resolution count {count} does not fit in a bigint"
                ))
            })?);
            columns
                .created_at
                .push(to_offset_date_time(observation.created_at().unix_micros())?);
            columns
                .updated_at
                .push(to_offset_date_time(observation.updated_at().unix_micros())?);
        }
        Ok(columns)
    }

    /// Writes every observation of the resolution `id` on `connection`.
    async fn upsert(&self, connection: &mut PgConnection, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(UPSERT_OBSERVATIONS)
            .bind(id)
            .bind(&self.source_ids)
            .bind(&self.first_seen)
            .bind(&self.last_seen)
            .bind(&self.counts)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(connection)
            .await
            .map(|_| ())
    }
}

/// A `kernel.dns_resolution` row joined with one of its observations, as it
/// is read; the observation columns are `NULL` when it has none.
#[derive(sqlx::FromRow)]
struct Row {
    id: Uuid,
    subject_kind: String,
    subject_value: String,
    answer_kind: String,
    answer_value: IpAddr,
    created_at: OffsetDateTime,
    source_id: Option<Uuid>,
    first_seen: Option<OffsetDateTime>,
    last_seen: Option<OffsetDateTime>,
    count: Option<i64>,
    observation_created_at: Option<OffsetDateTime>,
    observation_updated_at: Option<OffsetDateTime>,
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(micros: u64) -> Result<OffsetDateTime, DnsResolutionRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(micros) * 1_000)
        .map_err(|error| DnsResolutionRepositoryError::Unexpected(error.to_string()))
}

fn to_micros(instant: OffsetDateTime) -> Option<u64> {
    u64::try_from(instant.unix_timestamp_nanos() / 1_000).ok()
}

/// Rebuilds the resolutions of `rows`, which hold the rows of each
/// resolution next to each other, in the order the rows list them.
fn to_aggregates(rows: Vec<Row>) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
    let mut resolutions: Vec<Vec<Row>> = Vec::new();
    for row in rows {
        match resolutions.last_mut() {
            Some(resolution) if resolution[0].id == row.id => resolution.push(row),
            _ => resolutions.push(vec![row]),
        }
    }
    resolutions.into_iter().map(to_aggregate).collect()
}

/// Rebuilds one resolution from its rows, which are never empty.
fn to_aggregate(rows: Vec<Row>) -> Result<DnsResolution, DnsResolutionRepositoryError> {
    let link = &rows[0];
    let id = link.id;
    let corrupt = |detail: String| {
        DnsResolutionRepositoryError::Unexpected(format!("kernel.dns_resolution row {id} {detail}"))
    };

    let subject = match link.subject_kind.as_str() {
        "domain_name" => DnsResolutionSubject::domain_name(&link.subject_value)
            .map_err(|error| corrupt(format!("holds an invalid subject: {error}")))?,
        kind => return Err(corrupt(format!("holds the unknown subject kind {kind}"))),
    };
    if subject.value() != link.subject_value {
        return Err(corrupt(format!(
            "holds the subject {} in a form other than its canonical {}",
            link.subject_value,
            subject.value()
        )));
    }
    let answer = DnsResolutionAnswer::from_ip(link.answer_value);
    if answer.kind() != link.answer_kind {
        return Err(corrupt(format!(
            "holds the {} answer {answer} under the kind {}",
            answer.kind(),
            link.answer_kind
        )));
    }
    let created_at = to_micros(link.created_at)
        .and_then(|micros| DnsResolutionCreatedAt::from_unix_micros(micros).ok())
        .ok_or_else(|| corrupt("has a creation instant outside 1970 to 9999".to_string()))?;

    let observations = rows
        .iter()
        .filter_map(|row| to_observation(row, &corrupt).transpose())
        .collect::<Result<Vec<_>, _>>()?;
    let dns_resolution = DnsResolution::restore(subject, answer, created_at, observations)
        .ok_or_else(|| {
            corrupt("has no observation, or two observations of one source".to_string())
        })?;
    if *dns_resolution.id().value() != id {
        return Err(corrupt(format!(
            "is stored under an id that its endpoints {} and {} do not derive",
            dns_resolution.subject(),
            dns_resolution.answer()
        )));
    }
    Ok(dns_resolution)
}

/// Rebuilds the observation a row carries, or `None` when the resolution has
/// none.
fn to_observation(
    row: &Row,
    corrupt: &impl Fn(String) -> DnsResolutionRepositoryError,
) -> Result<Option<DnsResolutionObservation>, DnsResolutionRepositoryError> {
    let Some(source_id) = row.source_id else {
        return Ok(None);
    };
    let (Some(first_seen), Some(last_seen), Some(count), Some(created_at), Some(updated_at)) = (
        row.first_seen,
        row.last_seen,
        row.count,
        row.observation_created_at,
        row.observation_updated_at,
    ) else {
        return Err(corrupt(format!(
            "has an incomplete observation of source {source_id}"
        )));
    };
    let invalid =
        |detail: String| corrupt(format!("has an observation of source {source_id} {detail}"));

    let source_id = DnsResolutionSourceId::from_uuid(source_id)
        .map_err(|error| invalid(format!("with an invalid source id: {error}")))?;
    let window = to_micros(first_seen)
        .zip(to_micros(last_seen))
        .and_then(|(first_seen, last_seen)| {
            DnsResolutionWindow::from_unix_micros(first_seen, last_seen).ok()
        })
        .ok_or_else(|| {
            invalid("whose window is not an ordered pair of instants in 1970 to 9999".to_string())
        })?;
    let count = u64::try_from(count)
        .ok()
        .and_then(|count| DnsResolutionCount::from_u64(count).ok())
        .ok_or_else(|| invalid(format!("with the count {count}, below 1")))?;
    let created_at = to_micros(created_at)
        .and_then(|micros| DnsResolutionCreatedAt::from_unix_micros(micros).ok())
        .ok_or_else(|| invalid("created outside 1970 to 9999".to_string()))?;
    let updated_at = to_micros(updated_at)
        .and_then(|micros| DnsResolutionUpdatedAt::from_unix_micros(micros).ok())
        .ok_or_else(|| invalid("updated outside 1970 to 9999".to_string()))?;

    Ok(Some(DnsResolutionObservation::restore(
        source_id, window, count, created_at, updated_at,
    )))
}

fn unexpected(error: sqlx::Error) -> DnsResolutionRepositoryError {
    DnsResolutionRepositoryError::Unexpected(error.to_string())
}
