//! PostgreSQL implementation of [`Ipv6NetworkRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use crate::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "ipv6_network";

const INSERT: &str = "
    INSERT INTO kernel.ipv6_network (id, value, created_at)
    VALUES ($1, $2::cidr, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str =
    "SELECT id, value::text, created_at FROM kernel.ipv6_network WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.ipv6_network WHERE id = $1";

/// [`Ipv6NetworkRepository`] backed by the `kernel.ipv6_network` table.
///
/// Writes run on the current unit of work after taking the network's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresIpv6NetworkRepository {
    session: PgSession,
}

impl SqlxPostgresIpv6NetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl Ipv6NetworkRepository for SqlxPostgresIpv6NetworkRepository {
    async fn save(&self, ipv6_network: &Ipv6Network) -> Result<(), Ipv6NetworkRepositoryError> {
        let id = *ipv6_network.id().value();
        let value = ipv6_network.value().to_string();
        let created_at = to_offset_date_time(ipv6_network.created_at())?;

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
            return Err(Ipv6NetworkRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv6NetworkId,
    ) -> Result<Option<Ipv6Network>, Ipv6NetworkRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, (Uuid, String, OffsetDateTime)>(SELECT_BY_ID)
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

    async fn delete(&self, id: &Ipv6NetworkId) -> Result<(), Ipv6NetworkRepositoryError> {
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
            return Err(Ipv6NetworkRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &Ipv6NetworkCreatedAt,
) -> Result<OffsetDateTime, Ipv6NetworkRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| Ipv6NetworkRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<Ipv6Network, Ipv6NetworkRepositoryError> {
    let stored = Ipv6NetworkValue::new(&value).map_err(|error| {
        Ipv6NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv6_network row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        Ipv6NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv6_network row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = Ipv6NetworkCreatedAt::from_unix_micros(micros)
        .map_err(|error| Ipv6NetworkRepositoryError::Unexpected(error.to_string()))?;

    let ipv6_network = Ipv6Network::new(stored, created_at);
    if *ipv6_network.id().value() != id || ipv6_network.value().to_string() != value {
        return Err(Ipv6NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv6_network row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(ipv6_network)
}

fn unexpected(error: sqlx::Error) -> Ipv6NetworkRepositoryError {
    Ipv6NetworkRepositoryError::Unexpected(error.to_string())
}
