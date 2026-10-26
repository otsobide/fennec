//! [`QueryHandler`] for the list-DNS-resolutions-by-domain-name use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::{
    DnsResolutionEntry, DnsResolutionErrorEntry,
};
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::dns_resolutions_by_domain_name_lister::DnsResolutionsByDomainNameLister;
use super::list_dns_resolutions_by_domain_name_query::ListDnsResolutionsByDomainNameQuery;
use super::list_dns_resolutions_by_domain_name_response::ListDnsResolutionsByDomainNameResponse;

/// [`QueryHandler`] that processes [`ListDnsResolutionsByDomainNameQuery`]s by
/// delegating to [`DnsResolutionsByDomainNameLister`].
pub struct ListDnsResolutionsByDomainNameQueryHandler {
    lister: DnsResolutionsByDomainNameLister,
}

impl ListDnsResolutionsByDomainNameQueryHandler {
    pub fn new(lister: DnsResolutionsByDomainNameLister) -> Self {
        Self { lister }
    }
}

#[async_trait]
impl QueryHandler<ListDnsResolutionsByDomainNameQuery>
    for ListDnsResolutionsByDomainNameQueryHandler
{
    type Response = ListDnsResolutionsByDomainNameResponse;

    async fn handle(
        &self,
        query: ListDnsResolutionsByDomainNameQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.lister.execute(query.subject).await {
            Ok((dns_resolutions, truncated)) => Ok(ListDnsResolutionsByDomainNameResponse {
                dns_resolutions: dns_resolutions
                    .iter()
                    .map(DnsResolutionEntry::from_aggregate)
                    .collect(),
                truncated,
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(ListDnsResolutionsByDomainNameResponse {
                    dns_resolutions: Vec::new(),
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
