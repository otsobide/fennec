//! PostgreSQL implementation of [`Ipv6AddressRepository`].

use std::net::IpAddr;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use crate::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "ipv6_address";

const INSERT: &str = "
    INSERT INTO kernel.ipv6_address (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "SELECT id, value, created_at FROM kernel.ipv6_address WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.ipv6_address WHERE id = $1";

/// [`Ipv6AddressRepository`] backed by the `kernel.ipv6_address` table.
///
/// Writes run on the current unit of work after taking the address's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresIpv6AddressRepository {
    session: PgSession,
}

impl SqlxPostgresIpv6AddressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl Ipv6AddressRepository for SqlxPostgresIpv6AddressRepository {
    async fn save(&self, ipv6_address: &Ipv6Address) -> Result<(), Ipv6AddressRepositoryError> {
        let id = *ipv6_address.id().value();
        let value = IpAddr::V6(*ipv6_address.value().value());
        let created_at = to_offset_date_time(ipv6_address.created_at())?;

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
            return Err(Ipv6AddressRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv6AddressId,
    ) -> Result<Option<Ipv6Address>, Ipv6AddressRepositoryError> {
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

    async fn delete(&self, id: &Ipv6AddressId) -> Result<(), Ipv6AddressRepositoryError> {
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
            return Err(Ipv6AddressRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &Ipv6AddressCreatedAt,
) -> Result<OffsetDateTime, Ipv6AddressRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| Ipv6AddressRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: IpAddr,
    created_at: OffsetDateTime,
) -> Result<Ipv6Address, Ipv6AddressRepositoryError> {
    let IpAddr::V6(address) = value else {
        return Err(Ipv6AddressRepositoryError::Unexpected(format!(
            "kernel.ipv6_address row {id} holds the non-IPv6 value {value}"
        )));
    };
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        Ipv6AddressRepositoryError::Unexpected(format!(
            "kernel.ipv6_address row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = Ipv6AddressCreatedAt::from_unix_micros(micros)
        .map_err(|error| Ipv6AddressRepositoryError::Unexpected(error.to_string()))?;

    let ipv6_address = Ipv6Address::new(Ipv6AddressValue::from_ipv6_addr(address), created_at);
    if *ipv6_address.id().value() != id {
        return Err(Ipv6AddressRepositoryError::Unexpected(format!(
            "kernel.ipv6_address row {id} is stored under an id that its value {address} does not derive"
        )));
    }
    Ok(ipv6_address)
}

fn unexpected(error: sqlx::Error) -> Ipv6AddressRepositoryError {
    Ipv6AddressRepositoryError::Unexpected(error.to_string())
}
