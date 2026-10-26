//! [`QueryHandler`] for the find-autonomous-system use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::{
    AutonomousSystemEntry, AutonomousSystemErrorEntry,
};
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;

use super::autonomous_system_finder::AutonomousSystemFinder;
use super::find_autonomous_system_query::FindAutonomousSystemQuery;
use super::find_autonomous_system_response::FindAutonomousSystemResponse;

/// [`QueryHandler`] that processes [`FindAutonomousSystemQuery`]s by delegating to
/// [`AutonomousSystemFinder`].
pub struct FindAutonomousSystemQueryHandler {
    finder: AutonomousSystemFinder,
}

impl FindAutonomousSystemQueryHandler {
    pub fn new(finder: AutonomousSystemFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindAutonomousSystemQuery> for FindAutonomousSystemQueryHandler {
    type Response = FindAutonomousSystemResponse;

    async fn handle(
        &self,
        query: FindAutonomousSystemQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(autonomous_system) => Ok(FindAutonomousSystemResponse {
                autonomous_system: Some(AutonomousSystemEntry {
                    id: autonomous_system.id().to_string(),
                    stix_id: autonomous_system.id().stix_id(),
                    value: autonomous_system.value().to_string(),
                    name: autonomous_system
                        .name()
                        .map(|name| name.value().to_string()),
                    rir: autonomous_system.rir().map(|rir| rir.as_str().to_string()),
                    created_at: autonomous_system.created_at().value(),
                    updated_at: autonomous_system.updated_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    AutonomousSystemRepositoryError::NotFound => "NotFound",
                    AutonomousSystemRepositoryError::AlreadyExists => "AlreadyExists",
                    AutonomousSystemRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindAutonomousSystemResponse {
                    autonomous_system: None,
                    error: Some(AutonomousSystemErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
