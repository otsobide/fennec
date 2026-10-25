//! [`QueryHandler`] for the list-sightings-by-ioc use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::list_sightings_by_ioc_query::ListSightingsByIocQuery;
use super::list_sightings_by_ioc_response::ListSightingsByIocResponse;
use super::sightings_by_ioc_lister::SightingsByIocLister;

/// [`QueryHandler`] that processes [`ListSightingsByIocQuery`]s by delegating
/// to [`SightingsByIocLister`].
pub struct ListSightingsByIocQueryHandler {
    lister: SightingsByIocLister,
}

impl ListSightingsByIocQueryHandler {
    pub fn new(lister: SightingsByIocLister) -> Self {
        Self { lister }
    }
}

#[async_trait]
impl QueryHandler<ListSightingsByIocQuery> for ListSightingsByIocQueryHandler {
    type Response = ListSightingsByIocResponse;

    async fn handle(
        &self,
        query: ListSightingsByIocQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.lister.execute(query.ioc_id).await {
            Ok(sightings) => Ok(ListSightingsByIocResponse {
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
                Ok(ListSightingsByIocResponse {
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
