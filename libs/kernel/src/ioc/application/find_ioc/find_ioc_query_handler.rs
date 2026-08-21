//! [`QueryHandler`] for the find-ioc use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::ioc::application::find_ioc::find_ioc_response::{IocEntry, IocErrorEntry};
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;

use super::find_ioc_query::FindIocQuery;
use super::find_ioc_response::FindIocResponse;
use super::ioc_finder::IocFinder;

/// [`QueryHandler`] that processes [`FindIocQuery`]s by delegating to
/// [`IocFinder`].
pub struct FindIocQueryHandler {
    finder: IocFinder,
}

impl FindIocQueryHandler {
    pub fn new(finder: IocFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindIocQuery> for FindIocQueryHandler {
    type Response = FindIocResponse;

    async fn handle(&self, query: FindIocQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(ioc) => Ok(FindIocResponse {
                ioc: Some(IocEntry {
                    id: ioc.id().to_string(),
                    ioc_type: ioc.ioc_type().to_string(),
                    value: ioc.value().value().to_string(),
                    created_at: ioc.created_at().value(),
                    updated_at: ioc.updated_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    IocRepositoryError::NotFound => "NotFound",
                    IocRepositoryError::AlreadyExists => "AlreadyExists",
                    IocRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindIocResponse {
                    ioc: None,
                    error: Some(IocErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
