//! `DnsResolution` aggregate root.
//!
//! Represents the fact that a domain name resolved to an address, as one or
//! more sources reported it (`docs/data-model/links.md`). The fact is one
//! aggregate whatever the number of sources: its id is derived from its
//! endpoints, and each source keeps its own observation inside it. The
//! endpoints are referenced by kind and value; the module imports no
//! observable module.

use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::entities::dns_resolution_retraction::DnsResolutionRetraction;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use crate::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// Aggregate root modelling one DNS resolution and every source's
/// observation of it.
///
/// Invariants: the id is always derived from the endpoints; there is at
/// least one observation, and at most one per source; the endpoints and the
/// creation instant never change.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DnsResolution {
    id: DnsResolutionId,
    subject: DnsResolutionSubject,
    answer: DnsResolutionAnswer,
    created_at: DnsResolutionCreatedAt,
    observations: Vec<DnsResolutionObservation>,
}

impl DnsResolution {
    /// Records a resolution first reported by one source, deriving its id.
    pub fn record(
        subject: DnsResolutionSubject,
        answer: DnsResolutionAnswer,
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
        count: Option<DnsResolutionCount>,
        created_at: DnsResolutionCreatedAt,
    ) -> Self {
        Self {
            id: DnsResolutionId::from_endpoints(&subject, &answer),
            subject,
            answer,
            created_at,
            observations: vec![DnsResolutionObservation::new(
                source_id, window, count, created_at,
            )],
        }
    }

    /// Rebuilds a resolution from a store, deriving its id. Returns `None`
    /// when `observations` is empty or holds a source twice, which no
    /// recorded resolution can.
    pub fn restore(
        subject: DnsResolutionSubject,
        answer: DnsResolutionAnswer,
        created_at: DnsResolutionCreatedAt,
        mut observations: Vec<DnsResolutionObservation>,
    ) -> Option<Self> {
        observations.sort_by_key(|observation| *observation.source_id().value());
        let duplicated = observations
            .windows(2)
            .any(|pair| pair[0].source_id() == pair[1].source_id());
        if observations.is_empty() || duplicated {
            return None;
        }
        Some(Self {
            id: DnsResolutionId::from_endpoints(&subject, &answer),
            subject,
            answer,
            created_at,
            observations,
        })
    }

    /// Merges a report from `source_id`: a new source adds its observation,
    /// a known one widens its window and keeps the larger count. Returns the
    /// changed resolution and the observation as it now stands, or `None`
    /// when the report adds nothing.
    pub fn observe(
        &self,
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
        count: Option<DnsResolutionCount>,
        now: DnsResolutionUpdatedAt,
    ) -> Option<(Self, DnsResolutionObservation)> {
        let mut observations = self.observations.clone();
        let changed = match observations
            .iter()
            .position(|observation| *observation.source_id() == source_id)
        {
            Some(index) => {
                let merged = observations[index].merged(window, count, now)?;
                observations[index] = merged.clone();
                merged
            }
            None => {
                let created_at = DnsResolutionCreatedAt::from_updated_at(&now);
                let added = DnsResolutionObservation::new(source_id, window, count, created_at);
                observations.push(added.clone());
                observations.sort_by_key(|observation| *observation.source_id().value());
                added
            }
        };
        Some((
            Self {
                observations,
                ..self.clone()
            },
            changed,
        ))
    }

    /// Removes the observation of `source_id` and says what that did to the
    /// resolution ([`DnsResolutionRetraction`]).
    pub fn retract(&self, source_id: &DnsResolutionSourceId) -> DnsResolutionRetraction {
        let remaining: Vec<_> = self
            .observations
            .iter()
            .filter(|observation| observation.source_id() != source_id)
            .cloned()
            .collect();
        if remaining.len() == self.observations.len() {
            DnsResolutionRetraction::NotObserved
        } else if remaining.is_empty() {
            DnsResolutionRetraction::LastObservationRetracted
        } else {
            DnsResolutionRetraction::Retracted(Self {
                observations: remaining,
                ..self.clone()
            })
        }
    }

    /// Returns the window that covers every source's observation.
    pub fn window(&self) -> DnsResolutionWindow {
        self.observations
            .iter()
            .skip(1)
            .fold(*self.observations[0].window(), |window, observation| {
                window.merged(observation.window())
            })
    }

    pub fn id(&self) -> &DnsResolutionId {
        &self.id
    }

    pub fn subject(&self) -> &DnsResolutionSubject {
        &self.subject
    }

    pub fn answer(&self) -> &DnsResolutionAnswer {
        &self.answer
    }

    pub fn created_at(&self) -> &DnsResolutionCreatedAt {
        &self.created_at
    }

    /// Returns the observations, ordered by source id.
    pub fn observations(&self) -> &[DnsResolutionObservation] {
        &self.observations
    }

    /// Returns the observation of `source_id`, if the source reported the
    /// resolution.
    pub fn observation(
        &self,
        source_id: &DnsResolutionSourceId,
    ) -> Option<&DnsResolutionObservation> {
        self.observations
            .iter()
            .find(|observation| observation.source_id() == source_id)
    }
}
