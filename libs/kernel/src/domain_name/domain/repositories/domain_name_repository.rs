//! Repository trait for the domain name aggregate.

use async_trait::async_trait;

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// Async persistence contract for [`DomainName`] aggregates.
#[async_trait]
pub trait DomainNameRepository: Send + Sync {
    /// Persists a new domain name.
    ///
    /// # Errors
    ///
    /// Returns [`DomainNameRepositoryError::AlreadyExists`] if the name
    /// is already registered, or [`DomainNameRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, domain_name: &DomainName) -> Result<(), DomainNameRepositoryError>;

    /// Retrieves a domain name by its [`DomainNameId`].
    ///
    /// Returns `Ok(None)` if no domain name is found.
    async fn find_by_id(
        &self,
        id: &DomainNameId,
    ) -> Result<Option<DomainName>, DomainNameRepositoryError>;

    /// Deletes a domain name by its [`DomainNameId`].
    ///
    /// # Errors
    ///
    /// Returns [`DomainNameRepositoryError::NotFound`] if the name is not
    /// registered.
    async fn delete(&self, id: &DomainNameId) -> Result<(), DomainNameRepositoryError>;
}
