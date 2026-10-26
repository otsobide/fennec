//! [`QueryHandler`] for the find-domain-name use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::domain_name::application::find_domain_name::find_domain_name_response::{
    DomainNameEntry, DomainNameErrorEntry,
};
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;

use super::domain_name_finder::DomainNameFinder;
use super::find_domain_name_query::FindDomainNameQuery;
use super::find_domain_name_response::FindDomainNameResponse;

/// [`QueryHandler`] that processes [`FindDomainNameQuery`]s by delegating to
/// [`DomainNameFinder`].
pub struct FindDomainNameQueryHandler {
    finder: DomainNameFinder,
}

impl FindDomainNameQueryHandler {
    pub fn new(finder: DomainNameFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindDomainNameQuery> for FindDomainNameQueryHandler {
    type Response = FindDomainNameResponse;

    async fn handle(&self, query: FindDomainNameQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(domain_name) => Ok(FindDomainNameResponse {
                domain_name: Some(DomainNameEntry {
                    id: domain_name.id().to_string(),
                    stix_id: domain_name.id().stix_id(),
                    value: domain_name.value().to_string(),
                    created_at: domain_name.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    DomainNameRepositoryError::NotFound => "NotFound",
                    DomainNameRepositoryError::AlreadyExists => "AlreadyExists",
                    DomainNameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindDomainNameResponse {
                    domain_name: None,
                    error: Some(DomainNameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
