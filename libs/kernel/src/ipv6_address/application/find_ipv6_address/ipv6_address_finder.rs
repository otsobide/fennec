//! Domain service for finding a single IPv6 address.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// Domain service that looks up a single [`Ipv6Address`] by id.
pub struct Ipv6AddressFinder {
    repository: Arc<dyn Ipv6AddressRepository>,
}

impl Ipv6AddressFinder {
    pub fn new(repository: Arc<dyn Ipv6AddressRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Ipv6AddressId,
    ) -> Result<Ipv6Address, Ipv6AddressRepositoryError> {
        debug!(id = %id, "Finding IPv6 address");
        let ipv6_address = self.repository.find_by_id(&id).await?;

        ipv6_address.ok_or_else(|| {
            warn!(id = %id, "IPv6 address not found");
            Ipv6AddressRepositoryError::NotFound
        })
    }
}
