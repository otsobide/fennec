//! [`QueryHandler`] for the find-email-address use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::email_address::application::find_email_address::find_email_address_response::{
    EmailAddressEntry, EmailAddressErrorEntry,
};
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;

use super::email_address_finder::EmailAddressFinder;
use super::find_email_address_query::FindEmailAddressQuery;
use super::find_email_address_response::FindEmailAddressResponse;

/// [`QueryHandler`] that processes [`FindEmailAddressQuery`]s by delegating to
/// [`EmailAddressFinder`].
pub struct FindEmailAddressQueryHandler {
    finder: EmailAddressFinder,
}

impl FindEmailAddressQueryHandler {
    pub fn new(finder: EmailAddressFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindEmailAddressQuery> for FindEmailAddressQueryHandler {
    type Response = FindEmailAddressResponse;

    async fn handle(&self, query: FindEmailAddressQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(email_address) => Ok(FindEmailAddressResponse {
                email_address: Some(EmailAddressEntry {
                    id: email_address.id().to_string(),
                    stix_id: email_address.id().stix_id(),
                    value: email_address.value().to_string(),
                    created_at: email_address.created_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    EmailAddressRepositoryError::NotFound => "NotFound",
                    EmailAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    EmailAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindEmailAddressResponse {
                    email_address: None,
                    error: Some(EmailAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
