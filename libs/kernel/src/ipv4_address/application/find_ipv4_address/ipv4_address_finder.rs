//! Domain service for finding a single ipv4 address.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// Domain service that looks up a single [`Ipv4Address`] by id.
pub struct Ipv4AddressFinder {
    repository: Arc<dyn Ipv4AddressRepository>,
}

impl Ipv4AddressFinder {
    pub fn new(repository: Arc<dyn Ipv4AddressRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Ipv4AddressId,
    ) -> Result<Ipv4Address, Ipv4AddressRepositoryError> {
        debug!(id = %id, "Finding ipv4 address");
        let ipv4_address = self.repository.find_by_id(&id).await?;

        ipv4_address.ok_or_else(|| {
            warn!(id = %id, "Ipv4 address not found");
            Ipv4AddressRepositoryError::NotFound
        })
    }
}
