//! [`QueryHandler`] for the find-hostname use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::hostname::application::find_hostname::find_hostname_response::{
    HostnameEntry, HostnameErrorEntry,
};
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;

use super::find_hostname_query::FindHostnameQuery;
use super::find_hostname_response::FindHostnameResponse;
use super::hostname_finder::HostnameFinder;

/// [`QueryHandler`] that processes [`FindHostnameQuery`]s by delegating to
/// [`HostnameFinder`].
pub struct FindHostnameQueryHandler {
    finder: HostnameFinder,
}

impl FindHostnameQueryHandler {
    pub fn new(finder: HostnameFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindHostnameQuery> for FindHostnameQueryHandler {
    type Response = FindHostnameResponse;

    async fn handle(&self, query: FindHostnameQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(hostname) => Ok(FindHostnameResponse {
                hostname: Some(HostnameEntry {
                    id: hostname.id().to_string(),
                    stix_id: hostname.id().stix_id(),
                    value: hostname.value().to_string(),
                    created_at: hostname.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    HostnameRepositoryError::NotFound => "NotFound",
                    HostnameRepositoryError::AlreadyExists => "AlreadyExists",
                    HostnameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindHostnameResponse {
                    hostname: None,
                    error: Some(HostnameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
