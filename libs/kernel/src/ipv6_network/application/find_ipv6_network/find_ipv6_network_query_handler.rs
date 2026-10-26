//! [`QueryHandler`] for the find-ipv6-network use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::{
    Ipv6NetworkEntry, Ipv6NetworkErrorEntry,
};
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;

use super::find_ipv6_network_query::FindIpv6NetworkQuery;
use super::find_ipv6_network_response::FindIpv6NetworkResponse;
use super::ipv6_network_finder::Ipv6NetworkFinder;

/// [`QueryHandler`] that processes [`FindIpv6NetworkQuery`]s by delegating to
/// [`Ipv6NetworkFinder`].
pub struct FindIpv6NetworkQueryHandler {
    finder: Ipv6NetworkFinder,
}

impl FindIpv6NetworkQueryHandler {
    pub fn new(finder: Ipv6NetworkFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindIpv6NetworkQuery> for FindIpv6NetworkQueryHandler {
    type Response = FindIpv6NetworkResponse;

    async fn handle(&self, query: FindIpv6NetworkQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(ipv6_network) => Ok(FindIpv6NetworkResponse {
                ipv6_network: Some(Ipv6NetworkEntry {
                    id: ipv6_network.id().to_string(),
                    stix_id: ipv6_network.id().stix_id(),
                    value: ipv6_network.value().to_string(),
                    created_at: ipv6_network.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    Ipv6NetworkRepositoryError::NotFound => "NotFound",
                    Ipv6NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindIpv6NetworkResponse {
                    ipv6_network: None,
                    error: Some(Ipv6NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
