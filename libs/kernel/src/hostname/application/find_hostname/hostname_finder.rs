//! Domain service for finding a single hostname.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::repositories::hostname_repository::HostnameRepository;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// Domain service that looks up a single [`Hostname`] by id.
pub struct HostnameFinder {
    repository: Arc<dyn HostnameRepository>,
}

impl HostnameFinder {
    pub fn new(repository: Arc<dyn HostnameRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: HostnameId) -> Result<Hostname, HostnameRepositoryError> {
        debug!(id = %id, "Finding hostname");
        let hostname = self.repository.find_by_id(&id).await?;

        hostname.ok_or_else(|| {
            warn!(id = %id, "Hostname not found");
            HostnameRepositoryError::NotFound
        })
    }
}
