//! Domain service for finding a single IPv4 network.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// Domain service that looks up a single [`Ipv4Network`] by id.
pub struct Ipv4NetworkFinder {
    repository: Arc<dyn Ipv4NetworkRepository>,
}

impl Ipv4NetworkFinder {
    pub fn new(repository: Arc<dyn Ipv4NetworkRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Ipv4NetworkId,
    ) -> Result<Ipv4Network, Ipv4NetworkRepositoryError> {
        debug!(id = %id, "Finding IPv4 network");
        let ipv4_network = self.repository.find_by_id(&id).await?;

        ipv4_network.ok_or_else(|| {
            warn!(id = %id, "IPv4 network not found");
            Ipv4NetworkRepositoryError::NotFound
        })
    }
}
