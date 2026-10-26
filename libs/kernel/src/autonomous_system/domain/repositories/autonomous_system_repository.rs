//! Repository trait for the autonomous system aggregate.

use async_trait::async_trait;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// Async persistence contract for [`AutonomousSystem`] aggregates.
#[async_trait]
pub trait AutonomousSystemRepository: Send + Sync {
    /// Persists a new autonomous system.
    ///
    /// # Errors
    ///
    /// Returns [`AutonomousSystemRepositoryError::AlreadyExists`] if the autonomous system
    /// is already registered, or [`AutonomousSystemRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError>;

    /// Retrieves an autonomous system by its [`AutonomousSystemId`].
    ///
    /// Returns `Ok(None)` if no autonomous system is found.
    async fn find_by_id(
        &self,
        id: &AutonomousSystemId,
    ) -> Result<Option<AutonomousSystem>, AutonomousSystemRepositoryError>;

    /// Replaces the description (name, registry, update instant) of a
    /// registered autonomous system; its number and id never change.
    ///
    /// # Errors
    ///
    /// Returns [`AutonomousSystemRepositoryError::NotFound`] if the autonomous
    /// system is not registered, or [`AutonomousSystemRepositoryError::Unexpected`]
    /// on storage failure.
    async fn update(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError>;

    /// Deletes an autonomous system by its [`AutonomousSystemId`].
    ///
    /// # Errors
    ///
    /// Returns [`AutonomousSystemRepositoryError::NotFound`] if the autonomous system is not
    /// registered.
    async fn delete(&self, id: &AutonomousSystemId) -> Result<(), AutonomousSystemRepositoryError>;
}
