//! [`CommandHandler`] for the delete-DNS-resolution use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::delete_dns_resolution_command::DeleteDnsResolutionCommand;
use super::delete_dns_resolution_response::DeleteDnsResolutionResponse;
use super::dns_resolution_deleter::DnsResolutionDeleter;

/// [`CommandHandler`] that processes [`DeleteDnsResolutionCommand`]s by
/// delegating to [`DnsResolutionDeleter`].
pub struct DeleteDnsResolutionCommandHandler {
    deleter: DnsResolutionDeleter,
}

impl DeleteDnsResolutionCommandHandler {
    pub fn new(deleter: DnsResolutionDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteDnsResolutionCommand> for DeleteDnsResolutionCommandHandler {
    type Response = DeleteDnsResolutionResponse;

    async fn handle(
        &self,
        command: DeleteDnsResolutionCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteDnsResolutionResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteDnsResolutionResponse {
                    error: Some(DnsResolutionErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
