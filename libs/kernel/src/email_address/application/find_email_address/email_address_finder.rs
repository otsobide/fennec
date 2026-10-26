//! Domain service for finding a single email address.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// Domain service that looks up a single [`EmailAddress`] by id.
pub struct EmailAddressFinder {
    repository: Arc<dyn EmailAddressRepository>,
}

impl EmailAddressFinder {
    pub fn new(repository: Arc<dyn EmailAddressRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: EmailAddressId,
    ) -> Result<EmailAddress, EmailAddressRepositoryError> {
        debug!(id = %id, "Finding email address");
        let email_address = self.repository.find_by_id(&id).await?;

        email_address.ok_or_else(|| {
            warn!(id = %id, "Email address not found");
            EmailAddressRepositoryError::NotFound
        })
    }
}
