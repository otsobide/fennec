//! Domain service for finding a single domain name.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// Domain service that looks up a single [`DomainName`] by id.
pub struct DomainNameFinder {
    repository: Arc<dyn DomainNameRepository>,
}

impl DomainNameFinder {
    pub fn new(repository: Arc<dyn DomainNameRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: DomainNameId) -> Result<DomainName, DomainNameRepositoryError> {
        debug!(id = %id, "Finding domain name");
        let domain_name = self.repository.find_by_id(&id).await?;

        domain_name.ok_or_else(|| {
            warn!(id = %id, "Domain name not found");
            DomainNameRepositoryError::NotFound
        })
    }
}
