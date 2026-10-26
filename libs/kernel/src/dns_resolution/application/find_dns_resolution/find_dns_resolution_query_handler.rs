//! [`QueryHandler`] for the find-DNS-resolution use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::{
    DnsResolutionEntry, DnsResolutionErrorEntry,
};
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::dns_resolution_finder::DnsResolutionFinder;
use super::find_dns_resolution_query::FindDnsResolutionQuery;
use super::find_dns_resolution_response::FindDnsResolutionResponse;

/// [`QueryHandler`] that processes [`FindDnsResolutionQuery`]s by delegating
/// to [`DnsResolutionFinder`].
pub struct FindDnsResolutionQueryHandler {
    finder: DnsResolutionFinder,
}

impl FindDnsResolutionQueryHandler {
    pub fn new(finder: DnsResolutionFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindDnsResolutionQuery> for FindDnsResolutionQueryHandler {
    type Response = FindDnsResolutionResponse;

    async fn handle(&self, query: FindDnsResolutionQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(dns_resolution) => Ok(FindDnsResolutionResponse {
                dns_resolution: Some(DnsResolutionEntry::from_aggregate(&dns_resolution)),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindDnsResolutionResponse {
                    dns_resolution: None,
                    error: Some(DnsResolutionErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
