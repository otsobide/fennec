//! Entity for what one source saw of a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// One source's observation of a resolution, a child entity of the
/// [`DnsResolution`] aggregate identified by its source within it.
///
/// [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DnsResolutionObservation {
    source_id: DnsResolutionSourceId,
    window: DnsResolutionWindow,
    count: DnsResolutionCount,
    created_at: DnsResolutionCreatedAt,
    updated_at: DnsResolutionUpdatedAt,
}

impl DnsResolutionObservation {
    /// Records a source's first report; a missing count is one sighting.
    pub fn new(
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
        count: Option<DnsResolutionCount>,
        created_at: DnsResolutionCreatedAt,
    ) -> Self {
        Self {
            source_id,
            window,
            count: count.unwrap_or_else(DnsResolutionCount::one),
            created_at,
            updated_at: DnsResolutionUpdatedAt::from_created_at(&created_at),
        }
    }

    /// Rebuilds an observation from every stored field.
    pub fn restore(
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
        count: DnsResolutionCount,
        created_at: DnsResolutionCreatedAt,
        updated_at: DnsResolutionUpdatedAt,
    ) -> Self {
        Self {
            source_id,
            window,
            count,
            created_at,
            updated_at,
        }
    }

    /// Merges a new report from the same source: the window widens to cover
    /// both and the count takes the larger value. Returns `None` when the
    /// report adds nothing, so nothing has to be written or published.
    pub fn merged(
        &self,
        window: DnsResolutionWindow,
        count: Option<DnsResolutionCount>,
        updated_at: DnsResolutionUpdatedAt,
    ) -> Option<Self> {
        let merged_window = self.window.merged(&window);
        let merged_count = count.map_or(self.count, |count| self.count.max(count));
        if merged_window == self.window && merged_count == self.count {
            return None;
        }
        Some(Self {
            window: merged_window,
            count: merged_count,
            updated_at,
            ..self.clone()
        })
    }

    pub fn source_id(&self) -> &DnsResolutionSourceId {
        &self.source_id
    }

    pub fn window(&self) -> &DnsResolutionWindow {
        &self.window
    }

    pub fn count(&self) -> DnsResolutionCount {
        self.count
    }

    pub fn created_at(&self) -> &DnsResolutionCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DnsResolutionUpdatedAt {
        &self.updated_at
    }
}
