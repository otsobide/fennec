//! Domain service for finding a single IPv6 network.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// Domain service that looks up a single [`Ipv6Network`] by id.
pub struct Ipv6NetworkFinder {
    repository: Arc<dyn Ipv6NetworkRepository>,
}

impl Ipv6NetworkFinder {
    pub fn new(repository: Arc<dyn Ipv6NetworkRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Ipv6NetworkId,
    ) -> Result<Ipv6Network, Ipv6NetworkRepositoryError> {
        debug!(id = %id, "Finding IPv6 network");
        let ipv6_network = self.repository.find_by_id(&id).await?;

        ipv6_network.ok_or_else(|| {
            warn!(id = %id, "IPv6 network not found");
            Ipv6NetworkRepositoryError::NotFound
        })
    }
}
