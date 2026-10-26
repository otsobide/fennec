//! [`QueryHandler`] for the list-co-resolved-domain-names use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::dns_resolution::application::find_dns_resolution::dns_resolution_endpoint_entry::DnsResolutionEndpointEntry;
use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::co_resolved_domain_names_lister::CoResolvedDomainNamesLister;
use super::list_co_resolved_domain_names_query::ListCoResolvedDomainNamesQuery;
use super::list_co_resolved_domain_names_response::{
    CoResolvedDomainNameEntry, ListCoResolvedDomainNamesResponse,
};

/// [`QueryHandler`] that processes [`ListCoResolvedDomainNamesQuery`]s by
/// delegating to [`CoResolvedDomainNamesLister`].
pub struct ListCoResolvedDomainNamesQueryHandler {
    lister: CoResolvedDomainNamesLister,
}

impl ListCoResolvedDomainNamesQueryHandler {
    pub fn new(lister: CoResolvedDomainNamesLister) -> Self {
        Self { lister }
    }
}

#[async_trait]
impl QueryHandler<ListCoResolvedDomainNamesQuery> for ListCoResolvedDomainNamesQueryHandler {
    type Response = ListCoResolvedDomainNamesResponse;

    async fn handle(
        &self,
        query: ListCoResolvedDomainNamesQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.lister.execute(query.subject).await {
            Ok(co_resolved) => Ok(ListCoResolvedDomainNamesResponse {
                domain_name: Some(DnsResolutionEndpointEntry::from_subject(
                    co_resolved.subject(),
                )),
                co_resolved: co_resolved
                    .domain_names()
                    .iter()
                    .map(CoResolvedDomainNameEntry::from_co_resolved)
                    .collect(),
                truncated: co_resolved.truncated(),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(ListCoResolvedDomainNamesResponse {
                    domain_name: None,
                    co_resolved: Vec::new(),
                    truncated: false,
                    error: Some(DnsResolutionErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
