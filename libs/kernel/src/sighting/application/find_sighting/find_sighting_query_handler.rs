//! [`QueryHandler`] for the find-sighting use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::find_sighting_query::FindSightingQuery;
use super::find_sighting_response::FindSightingResponse;
use super::sighting_finder::SightingFinder;

/// [`QueryHandler`] that processes [`FindSightingQuery`]s by delegating to
/// [`SightingFinder`].
pub struct FindSightingQueryHandler {
    finder: SightingFinder,
}

impl FindSightingQueryHandler {
    pub fn new(finder: SightingFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindSightingQuery> for FindSightingQueryHandler {
    type Response = FindSightingResponse;

    async fn handle(&self, query: FindSightingQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(sighting) => Ok(FindSightingResponse {
                sighting: Some(SightingEntry {
                    id: sighting.id().to_string(),
                    ioc_id: sighting.ioc_id().to_string(),
                    source_id: sighting.source_id().to_string(),
                    first_seen: sighting.first_seen().value(),
                    last_seen: sighting.last_seen().value(),
                    count: sighting.count().value(),
                    created_at: sighting.created_at().value(),
                    updated_at: sighting.updated_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let (concept, existing_id) = match &e {
                    SightingRepositoryError::NotFound => ("NotFound", None),
                    SightingRepositoryError::IdAlreadyExists => ("AlreadyExists", None),
                    SightingRepositoryError::PairAlreadyExists { existing_id } => {
                        ("PairAlreadyExists", Some(existing_id.to_string()))
                    }
                    SightingRepositoryError::Unexpected(_) => ("Unexpected", None),
                };
                Ok(FindSightingResponse {
                    sighting: None,
                    error: Some(SightingErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                        existing_id,
                    }),
                })
            }
        }
    }
}
