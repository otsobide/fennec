//! [`CommandHandler`] for the retract-DNS-resolution use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;

use super::dns_resolution_retracter::DnsResolutionRetracter;
use super::retract_dns_resolution_command::RetractDnsResolutionCommand;
use super::retract_dns_resolution_response::RetractDnsResolutionResponse;

/// [`CommandHandler`] that processes [`RetractDnsResolutionCommand`]s by
/// delegating to [`DnsResolutionRetracter`].
pub struct RetractDnsResolutionCommandHandler {
    retracter: DnsResolutionRetracter,
}

impl RetractDnsResolutionCommandHandler {
    pub fn new(retracter: DnsResolutionRetracter) -> Self {
        Self { retracter }
    }
}

#[async_trait]
impl CommandHandler<RetractDnsResolutionCommand> for RetractDnsResolutionCommandHandler {
    type Response = RetractDnsResolutionResponse;

    async fn handle(
        &self,
        command: RetractDnsResolutionCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.retracter.execute(command.id, command.source_id).await {
            Ok(()) => Ok(RetractDnsResolutionResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(RetractDnsResolutionResponse {
                    error: Some(DnsResolutionErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
