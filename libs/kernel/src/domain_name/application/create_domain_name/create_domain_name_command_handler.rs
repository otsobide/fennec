//! [`CommandHandler`] for the create-domain-name use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::domain_name::application::find_domain_name::find_domain_name_response::DomainNameErrorEntry;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;

use super::create_domain_name_command::CreateDomainNameCommand;
use super::create_domain_name_response::CreateDomainNameResponse;
use super::domain_name_creator::DomainNameCreator;

/// [`CommandHandler`] that processes [`CreateDomainNameCommand`]s by
/// delegating to [`DomainNameCreator`].
pub struct CreateDomainNameCommandHandler {
    creator: DomainNameCreator,
}

impl CreateDomainNameCommandHandler {
    pub fn new(creator: DomainNameCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateDomainNameCommand> for CreateDomainNameCommandHandler {
    type Response = CreateDomainNameResponse;

    async fn handle(
        &self,
        command: CreateDomainNameCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateDomainNameResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    DomainNameRepositoryError::NotFound => "NotFound",
                    DomainNameRepositoryError::AlreadyExists => "AlreadyExists",
                    DomainNameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateDomainNameResponse {
                    error: Some(DomainNameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
