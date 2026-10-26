//! PostgreSQL implementation of [`Ipv4AddressRepository`].

use std::net::IpAddr;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use crate::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "ipv4_address";

const INSERT: &str = "
    INSERT INTO kernel.ipv4_address (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "SELECT id, value, created_at FROM kernel.ipv4_address WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.ipv4_address WHERE id = $1";

/// [`Ipv4AddressRepository`] backed by the `kernel.ipv4_address` table.
///
/// Writes run on the current unit of work after taking the address's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresIpv4AddressRepository {
    session: PgSession,
}

impl SqlxPostgresIpv4AddressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl Ipv4AddressRepository for SqlxPostgresIpv4AddressRepository {
    async fn save(&self, ipv4_address: &Ipv4Address) -> Result<(), Ipv4AddressRepositoryError> {
        let id = *ipv4_address.id().value();
        let value = IpAddr::V4(*ipv4_address.value().value());
        let created_at = to_offset_date_time(ipv4_address.created_at())?;

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
            return Err(Ipv4AddressRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv4AddressId,
    ) -> Result<Option<Ipv4Address>, Ipv4AddressRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, (Uuid, IpAddr, OffsetDateTime)>(SELECT_BY_ID)
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

    async fn delete(&self, id: &Ipv4AddressId) -> Result<(), Ipv4AddressRepositoryError> {
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
            return Err(Ipv4AddressRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &Ipv4AddressCreatedAt,
) -> Result<OffsetDateTime, Ipv4AddressRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| Ipv4AddressRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: IpAddr,
    created_at: OffsetDateTime,
) -> Result<Ipv4Address, Ipv4AddressRepositoryError> {
    let IpAddr::V4(address) = value else {
        return Err(Ipv4AddressRepositoryError::Unexpected(format!(
            "kernel.ipv4_address row {id} holds the non-IPv4 value {value}"
        )));
    };
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        Ipv4AddressRepositoryError::Unexpected(format!(
            "kernel.ipv4_address row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = Ipv4AddressCreatedAt::from_unix_micros(micros)
        .map_err(|error| Ipv4AddressRepositoryError::Unexpected(error.to_string()))?;

    let ipv4_address = Ipv4Address::new(Ipv4AddressValue::from_ipv4_addr(address), created_at);
    if *ipv4_address.id().value() != id {
        return Err(Ipv4AddressRepositoryError::Unexpected(format!(
            "kernel.ipv4_address row {id} is stored under an id that its value {address} does not derive"
        )));
    }
    Ok(ipv4_address)
}

fn unexpected(error: sqlx::Error) -> Ipv4AddressRepositoryError {
    Ipv4AddressRepositoryError::Unexpected(error.to_string())
}
