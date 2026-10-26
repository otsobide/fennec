//! [`CommandHandler`] for the record-DNS-resolution use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

use super::dns_resolution_recorder::DnsResolutionRecorder;
use super::record_dns_resolution_command::RecordDnsResolutionCommand;
use super::record_dns_resolution_response::RecordDnsResolutionResponse;

/// [`CommandHandler`] that processes [`RecordDnsResolutionCommand`]s by
/// delegating to [`DnsResolutionRecorder`].
pub struct RecordDnsResolutionCommandHandler {
    recorder: DnsResolutionRecorder,
}

impl RecordDnsResolutionCommandHandler {
    pub fn new(recorder: DnsResolutionRecorder) -> Self {
        Self { recorder }
    }
}

#[async_trait]
impl CommandHandler<RecordDnsResolutionCommand> for RecordDnsResolutionCommandHandler {
    type Response = RecordDnsResolutionResponse;

    async fn handle(
        &self,
        command: RecordDnsResolutionCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .recorder
            .execute(
                command.subject,
                command.answer,
                command.source_id,
                command.window,
                command.count,
            )
            .await
        {
            Ok(recording) => Ok(RecordDnsResolutionResponse {
                id: Some(recording.id().to_string()),
                stix_id: Some(recording.id().stix_id()),
                relationship_type: Some(DnsResolutionId::RELATIONSHIP_TYPE.to_string()),
                outcome: Some(recording.outcome().to_string()),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    DnsResolutionRepositoryError::NotFound => "NotFound",
                    DnsResolutionRepositoryError::AlreadyExists => "AlreadyExists",
                    DnsResolutionRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(RecordDnsResolutionResponse {
                    id: None,
                    stix_id: None,
                    relationship_type: None,
                    outcome: None,
                    error: Some(DnsResolutionErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
