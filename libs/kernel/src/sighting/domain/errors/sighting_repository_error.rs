//! Error types for sighting repository operations.

use thiserror::Error;

use crate::sighting::domain::value_objects::sighting_id::SightingId;

/// Errors that can occur during
/// [`SightingRepository`](crate::sighting::domain::repositories::sighting_repository::SightingRepository)
/// operations.
#[derive(Debug, Error)]
pub enum SightingRepositoryError {
    /// The requested sighting was not found.
    #[error("sighting not found")]
    NotFound,

    /// A sighting with the same id already exists.
    #[error("sighting id already exists")]
    IdAlreadyExists,

    /// A sighting for the same `(ioc_id, source_id)` pair already exists.
    ///
    /// The `existing_id` allows callers to immediately observe the existing
    /// sighting rather than retrying construction.
    #[error("sighting pair already exists (existing id: {existing_id})")]
    PairAlreadyExists { existing_id: SightingId },

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
