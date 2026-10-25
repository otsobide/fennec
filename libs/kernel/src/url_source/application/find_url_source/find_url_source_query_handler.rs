//! [`QueryHandler`] for the find-url-source use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::url_source::application::find_url_source::find_url_source_response::{
    UrlSourceEntry, UrlSourceErrorEntry,
};
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;

use super::find_url_source_query::FindUrlSourceQuery;
use super::find_url_source_response::FindUrlSourceResponse;
use super::url_source_finder::UrlSourceFinder;

/// [`QueryHandler`] that processes [`FindUrlSourceQuery`]s by delegating to
/// [`UrlSourceFinder`].
pub struct FindUrlSourceQueryHandler {
    finder: UrlSourceFinder,
}

impl FindUrlSourceQueryHandler {
    pub fn new(finder: UrlSourceFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindUrlSourceQuery> for FindUrlSourceQueryHandler {
    type Response = FindUrlSourceResponse;

    async fn handle(&self, query: FindUrlSourceQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(url_source) => Ok(FindUrlSourceResponse {
                url_source: Some(UrlSourceEntry {
                    id: url_source.id().to_string(),
                    url: url_source.url().value().to_string(),
                    format: url_source.format().to_string(),
                    polling_interval_seconds: url_source.polling_interval().seconds(),
                    created_at: url_source.created_at().value(),
                    updated_at: url_source.updated_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    UrlSourceRepositoryError::NotFound => "NotFound",
                    UrlSourceRepositoryError::AlreadyExists => "AlreadyExists",
                    UrlSourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindUrlSourceResponse {
                    url_source: None,
                    error: Some(UrlSourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
