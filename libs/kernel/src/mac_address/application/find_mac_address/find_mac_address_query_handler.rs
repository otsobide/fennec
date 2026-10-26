//! [`QueryHandler`] for the find-mac-address use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::mac_address::application::find_mac_address::find_mac_address_response::{
    MacAddressEntry, MacAddressErrorEntry,
};
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;

use super::find_mac_address_query::FindMacAddressQuery;
use super::find_mac_address_response::FindMacAddressResponse;
use super::mac_address_finder::MacAddressFinder;

/// [`QueryHandler`] that processes [`FindMacAddressQuery`]s by delegating to
/// [`MacAddressFinder`].
pub struct FindMacAddressQueryHandler {
    finder: MacAddressFinder,
}

impl FindMacAddressQueryHandler {
    pub fn new(finder: MacAddressFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindMacAddressQuery> for FindMacAddressQueryHandler {
    type Response = FindMacAddressResponse;

    async fn handle(&self, query: FindMacAddressQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(mac_address) => Ok(FindMacAddressResponse {
                mac_address: Some(MacAddressEntry {
                    id: mac_address.id().to_string(),
                    stix_id: mac_address.id().stix_id(),
                    value: mac_address.value().to_string(),
                    created_at: mac_address.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    MacAddressRepositoryError::NotFound => "NotFound",
                    MacAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    MacAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindMacAddressResponse {
                    mac_address: None,
                    error: Some(MacAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
