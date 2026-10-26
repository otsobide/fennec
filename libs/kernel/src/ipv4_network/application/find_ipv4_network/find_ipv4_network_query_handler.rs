//! [`QueryHandler`] for the find-ipv4-network use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::{
    Ipv4NetworkEntry, Ipv4NetworkErrorEntry,
};
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;

use super::find_ipv4_network_query::FindIpv4NetworkQuery;
use super::find_ipv4_network_response::FindIpv4NetworkResponse;
use super::ipv4_network_finder::Ipv4NetworkFinder;

/// [`QueryHandler`] that processes [`FindIpv4NetworkQuery`]s by delegating to
/// [`Ipv4NetworkFinder`].
pub struct FindIpv4NetworkQueryHandler {
    finder: Ipv4NetworkFinder,
}

impl FindIpv4NetworkQueryHandler {
    pub fn new(finder: Ipv4NetworkFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindIpv4NetworkQuery> for FindIpv4NetworkQueryHandler {
    type Response = FindIpv4NetworkResponse;

    async fn handle(&self, query: FindIpv4NetworkQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(ipv4_network) => Ok(FindIpv4NetworkResponse {
                ipv4_network: Some(Ipv4NetworkEntry {
                    id: ipv4_network.id().to_string(),
                    stix_id: ipv4_network.id().stix_id(),
                    value: ipv4_network.value().to_string(),
                    created_at: ipv4_network.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    Ipv4NetworkRepositoryError::NotFound => "NotFound",
                    Ipv4NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindIpv4NetworkResponse {
                    ipv4_network: None,
                    error: Some(Ipv4NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
