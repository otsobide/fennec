//! [`QueryHandler`] for the find-ipv6-address use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::{
    Ipv6AddressEntry, Ipv6AddressErrorEntry,
};
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;

use super::find_ipv6_address_query::FindIpv6AddressQuery;
use super::find_ipv6_address_response::FindIpv6AddressResponse;
use super::ipv6_address_finder::Ipv6AddressFinder;

/// [`QueryHandler`] that processes [`FindIpv6AddressQuery`]s by delegating to
/// [`Ipv6AddressFinder`].
pub struct FindIpv6AddressQueryHandler {
    finder: Ipv6AddressFinder,
}

impl FindIpv6AddressQueryHandler {
    pub fn new(finder: Ipv6AddressFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindIpv6AddressQuery> for FindIpv6AddressQueryHandler {
    type Response = FindIpv6AddressResponse;

    async fn handle(&self, query: FindIpv6AddressQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(ipv6_address) => Ok(FindIpv6AddressResponse {
                ipv6_address: Some(Ipv6AddressEntry {
                    id: ipv6_address.id().to_string(),
                    stix_id: ipv6_address.id().stix_id(),
                    value: ipv6_address.value().to_string(),
                    created_at: ipv6_address.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    Ipv6AddressRepositoryError::NotFound => "NotFound",
                    Ipv6AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindIpv6AddressResponse {
                    ipv6_address: None,
                    error: Some(Ipv6AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
