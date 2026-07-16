//! [`QueryHandler`] for the list-sightings-by-source use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::list_sightings_by_source_query::ListSightingsBySourceQuery;
use super::list_sightings_by_source_response::ListSightingsBySourceResponse;
use super::sightings_by_source_lister::SightingsBySourceLister;

/// [`QueryHandler`] that processes [`ListSightingsBySourceQuery`]s by
/// delegating to [`SightingsBySourceLister`].
pub struct ListSightingsBySourceQueryHandler {
    lister: SightingsBySourceLister,
}

impl ListSightingsBySourceQueryHandler {
    pub fn new(lister: SightingsBySourceLister) -> Self {
        Self { lister }
    }
}

#[async_trait]
impl QueryHandler<ListSightingsBySourceQuery> for ListSightingsBySourceQueryHandler {
    type Response = ListSightingsBySourceResponse;

    async fn handle(
        &self,
        query: ListSightingsBySourceQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.lister.execute(query.source_id).await {
            Ok(sightings) => Ok(ListSightingsBySourceResponse {
                sightings: sightings
                    .into_iter()
                    .map(|s| SightingEntry {
                        id: s.id().to_string(),
                        ioc_id: s.ioc_id().to_string(),
                        source_id: s.source_id().to_string(),
                        first_seen: s.first_seen().value(),
                        last_seen: s.last_seen().value(),
                        count: s.count().value(),
                        created_at: s.created_at().value(),
                        updated_at: s.updated_at().value(),
                    })
                    .collect(),
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
                Ok(ListSightingsBySourceResponse {
                    sightings: Vec::new(),
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
