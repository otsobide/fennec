//! Error types for autonomous system repository operations.

use thiserror::Error;

/// Errors that can occur during [`AutonomousSystemRepository`] operations.
///
/// [`AutonomousSystemRepository`]: crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository
#[derive(Debug, Error)]
pub enum AutonomousSystemRepositoryError {
    /// The requested autonomous system was not found.
    #[error("autonomous system not found")]
    NotFound,

    /// The autonomous system is already registered.
    #[error("autonomous system already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
