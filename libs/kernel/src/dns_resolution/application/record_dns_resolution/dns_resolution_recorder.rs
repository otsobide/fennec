//! Domain service for recording a source's report of a DNS resolution.

use std::sync::Arc;

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::create_dns_resolution_created_event::create_dns_resolution_created_event;
use crate::dns_resolution::domain::events::create_dns_resolution_observed_event::create_dns_resolution_observed_event;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use crate::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

use super::dns_resolution_recording::DnsResolutionRecording;

/// Domain service that records a source's report of a [`DnsResolution`].
///
/// The id is derived from the endpoints, so every source and every retry
/// converges on one aggregate:
///
/// - a resolution not recorded yet is saved with the report as its first
///   observation, and a [`DnsResolutionCreatedEvent`] then a
///   [`DnsResolutionObservedEvent`] are published;
/// - a recorded one merges the report into the source's observation; when
///   that changes it, the resolution is updated and a
///   [`DnsResolutionObservedEvent`] is published;
/// - a report that changes nothing writes and publishes nothing.
///
/// Every [`DnsResolutionObservedEvent`] carries the reported window and
/// count next to the merged ones, so the event log keeps each report.
///
/// Nothing is published if a write fails.
///
/// [`DnsResolutionCreatedEvent`]: crate::dns_resolution::domain::events::dns_resolution_created_event::DnsResolutionCreatedEvent
/// [`DnsResolutionObservedEvent`]: crate::dns_resolution::domain::events::dns_resolution_observed_event::DnsResolutionObservedEvent
pub struct DnsResolutionRecorder {
    repository: Arc<dyn DnsResolutionRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DnsResolutionRecorder {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Records the report and says what it did.
    ///
    /// # Errors
    ///
    /// Returns the repository's error when a read or a write fails, and
    /// [`DnsResolutionRepositoryError::Unexpected`] when publishing fails.
    pub async fn execute(
        &self,
        subject: DnsResolutionSubject,
        answer: DnsResolutionAnswer,
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
        count: Option<DnsResolutionCount>,
    ) -> Result<DnsResolutionRecording, DnsResolutionRepositoryError> {
        let id = DnsResolutionId::from_endpoints(&subject, &answer);
        debug!(
            id = %id,
            subject = %subject,
            answer = %answer,
            source_id = %source_id,
            "Recording DNS resolution"
        );

        let Some(recorded) = self.repository.find_by_id(&id).await? else {
            let dns_resolution = DnsResolution::record(
                subject,
                answer,
                source_id,
                window,
                count,
                DnsResolutionCreatedAt::now(),
            );
            self.repository.save(&dns_resolution).await?;

            // `record` gives a new resolution exactly one observation.
            let first_observation = &dns_resolution.observations()[0];
            let created = create_dns_resolution_created_event(&dns_resolution)?;
            let observed = create_dns_resolution_observed_event(
                &dns_resolution,
                first_observation,
                &window,
                count,
            )?;
            self.publish(vec![Box::new(created), Box::new(observed)])?;

            info!(id = %id, "DNS resolution created");
            return Ok(DnsResolutionRecording::Created(id));
        };

        let Some((updated, observation)) =
            recorded.observe(source_id, window, count, DnsResolutionUpdatedAt::now())
        else {
            info!(id = %id, "DNS resolution unchanged");
            return Ok(DnsResolutionRecording::Unchanged(id));
        };

        self.repository.update(&updated).await?;

        let observed =
            create_dns_resolution_observed_event(&updated, &observation, &window, count)?;
        self.publish(vec![Box::new(observed)])?;

        info!(id = %id, source_id = %observation.source_id(), "DNS resolution observed");
        Ok(DnsResolutionRecording::Observed(id))
    }

    fn publish(
        &self,
        events: Vec<Box<dyn DomainEvent>>,
    ) -> Result<(), DnsResolutionRepositoryError> {
        self.event_bus
            .publish(events)
            .map_err(|e| DnsResolutionRepositoryError::Unexpected(e.to_string()))
    }
}
