//! Domain service for finding a single DNS resolution.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

/// Domain service that looks up a single [`DnsResolution`] by id, with all
/// of its observations.
pub struct DnsResolutionFinder {
    repository: Arc<dyn DnsResolutionRepository>,
}

impl DnsResolutionFinder {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>) -> Self {
        Self { repository }
    }

    /// Returns the resolution with the given id.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::NotFound`] if it is not
    /// recorded, or the repository's error on storage failure.
    pub async fn execute(
        &self,
        id: DnsResolutionId,
    ) -> Result<DnsResolution, DnsResolutionRepositoryError> {
        debug!(id = %id, "Finding DNS resolution");
        let dns_resolution = self.repository.find_by_id(&id).await?;

        dns_resolution.ok_or_else(|| {
            warn!(id = %id, "DNS resolution not found");
            DnsResolutionRepositoryError::NotFound
        })
    }
}
