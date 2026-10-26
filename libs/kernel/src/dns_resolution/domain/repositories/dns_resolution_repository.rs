//! Repository trait for the DNS resolution aggregate.

use async_trait::async_trait;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Async persistence contract for [`DnsResolution`] aggregates, each stored
/// with all of its observations.
#[async_trait]
pub trait DnsResolutionRepository: Send + Sync {
    /// Persists a resolution recorded for the first time, with its
    /// observations.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::AlreadyExists`] if the
    /// resolution is already recorded, or
    /// [`DnsResolutionRepositoryError::Unexpected`] on storage failure.
    async fn save(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError>;

    /// Replaces the observations of a recorded resolution with the ones the
    /// aggregate holds; its endpoints never change.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::NotFound`] if the resolution is
    /// not recorded, or [`DnsResolutionRepositoryError::Unexpected`] on
    /// storage failure.
    async fn update(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError>;

    /// Retrieves a resolution by its [`DnsResolutionId`]; `Ok(None)` if it
    /// is not recorded.
    async fn find_by_id(
        &self,
        id: &DnsResolutionId,
    ) -> Result<Option<DnsResolution>, DnsResolutionRepositoryError>;

    /// Lists at most `limit` resolutions of `subject`, ordered by the
    /// address of their answer with IPv4 before IPv6 (the order of `inet`).
    async fn list_by_subject(
        &self,
        subject: &DnsResolutionSubject,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError>;

    /// Lists at most `limit` resolutions to `answer`, ordered by the
    /// canonical value of their subject.
    async fn list_by_answer(
        &self,
        answer: &DnsResolutionAnswer,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError>;

    /// Deletes a resolution and every observation of it.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::NotFound`] if the resolution is
    /// not recorded.
    async fn delete(&self, id: &DnsResolutionId) -> Result<(), DnsResolutionRepositoryError>;
}
