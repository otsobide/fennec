//! [`CommandHandler`] for the delete-domain-name use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::domain_name::application::find_domain_name::find_domain_name_response::DomainNameErrorEntry;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;

use super::delete_domain_name_command::DeleteDomainNameCommand;
use super::delete_domain_name_response::DeleteDomainNameResponse;
use super::domain_name_deleter::DomainNameDeleter;

/// [`CommandHandler`] that processes [`DeleteDomainNameCommand`]s by
/// delegating to [`DomainNameDeleter`].
pub struct DeleteDomainNameCommandHandler {
    deleter: DomainNameDeleter,
}

impl DeleteDomainNameCommandHandler {
    pub fn new(deleter: DomainNameDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteDomainNameCommand> for DeleteDomainNameCommandHandler {
    type Response = DeleteDomainNameResponse;

    async fn handle(
        &self,
        command: DeleteDomainNameCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteDomainNameResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    DomainNameRepositoryError::NotFound => "NotFound",
                    DomainNameRepositoryError::AlreadyExists => "AlreadyExists",
                    DomainNameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteDomainNameResponse {
                    error: Some(DomainNameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
