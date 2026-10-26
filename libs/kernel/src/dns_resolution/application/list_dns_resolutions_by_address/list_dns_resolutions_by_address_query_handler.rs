//! [`QueryHandler`] for the list-DNS-resolutions-by-address use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::{
    DnsResolutionEntry, DnsResolutionErrorEntry,
};
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::dns_resolutions_by_address_lister::DnsResolutionsByAddressLister;
use super::list_dns_resolutions_by_address_query::ListDnsResolutionsByAddressQuery;
use super::list_dns_resolutions_by_address_response::ListDnsResolutionsByAddressResponse;

/// [`QueryHandler`] that processes [`ListDnsResolutionsByAddressQuery`]s by
/// delegating to [`DnsResolutionsByAddressLister`].
pub struct ListDnsResolutionsByAddressQueryHandler {
    lister: DnsResolutionsByAddressLister,
}

impl ListDnsResolutionsByAddressQueryHandler {
    pub fn new(lister: DnsResolutionsByAddressLister) -> Self {
        Self { lister }
    }
}

#[async_trait]
impl QueryHandler<ListDnsResolutionsByAddressQuery> for ListDnsResolutionsByAddressQueryHandler {
    type Response = ListDnsResolutionsByAddressResponse;

    async fn handle(
        &self,
        query: ListDnsResolutionsByAddressQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.lister.execute(query.answer).await {
            Ok((dns_resolutions, truncated)) => Ok(ListDnsResolutionsByAddressResponse {
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
                Ok(ListDnsResolutionsByAddressResponse {
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
