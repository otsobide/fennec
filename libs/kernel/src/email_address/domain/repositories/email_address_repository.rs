//! Repository trait for the email address aggregate.

use async_trait::async_trait;

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// Async persistence contract for [`EmailAddress`] aggregates.
#[async_trait]
pub trait EmailAddressRepository: Send + Sync {
    /// Persists a new email address.
    ///
    /// # Errors
    ///
    /// Returns [`EmailAddressRepositoryError::AlreadyExists`] if the address
    /// is already registered, or [`EmailAddressRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, email_address: &EmailAddress) -> Result<(), EmailAddressRepositoryError>;

    /// Retrieves an email address by its [`EmailAddressId`].
    ///
    /// Returns `Ok(None)` if no email address is found.
    async fn find_by_id(
        &self,
        id: &EmailAddressId,
    ) -> Result<Option<EmailAddress>, EmailAddressRepositoryError>;

    /// Deletes an email address by its [`EmailAddressId`].
    ///
    /// # Errors
    ///
    /// Returns [`EmailAddressRepositoryError::NotFound`] if the address is not
    /// registered.
    async fn delete(&self, id: &EmailAddressId) -> Result<(), EmailAddressRepositoryError>;
}
