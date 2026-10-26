//! Domain service for finding a single MAC address.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// Domain service that looks up a single [`MacAddress`] by id.
pub struct MacAddressFinder {
    repository: Arc<dyn MacAddressRepository>,
}

impl MacAddressFinder {
    pub fn new(repository: Arc<dyn MacAddressRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: MacAddressId) -> Result<MacAddress, MacAddressRepositoryError> {
        debug!(id = %id, "Finding MAC address");
        let mac_address = self.repository.find_by_id(&id).await?;

        mac_address.ok_or_else(|| {
            warn!(id = %id, "MAC address not found");
            MacAddressRepositoryError::NotFound
        })
    }
}
