//! [`QueryHandler`] for the find-ipv4-address use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::{
    Ipv4AddressEntry, Ipv4AddressErrorEntry,
};
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;

use super::find_ipv4_address_query::FindIpv4AddressQuery;
use super::find_ipv4_address_response::FindIpv4AddressResponse;
use super::ipv4_address_finder::Ipv4AddressFinder;

/// [`QueryHandler`] that processes [`FindIpv4AddressQuery`]s by delegating to
/// [`Ipv4AddressFinder`].
pub struct FindIpv4AddressQueryHandler {
    finder: Ipv4AddressFinder,
}

impl FindIpv4AddressQueryHandler {
    pub fn new(finder: Ipv4AddressFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindIpv4AddressQuery> for FindIpv4AddressQueryHandler {
    type Response = FindIpv4AddressResponse;

    async fn handle(&self, query: FindIpv4AddressQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(ipv4_address) => Ok(FindIpv4AddressResponse {
                ipv4_address: Some(Ipv4AddressEntry {
                    id: ipv4_address.id().to_string(),
                    stix_id: ipv4_address.id().stix_id(),
                    value: ipv4_address.value().to_string(),
                    created_at: ipv4_address.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    Ipv4AddressRepositoryError::NotFound => "NotFound",
                    Ipv4AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindIpv4AddressResponse {
                    ipv4_address: None,
                    error: Some(Ipv4AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
