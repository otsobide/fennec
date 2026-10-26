//! Domain service for listing the DNS resolutions to an address.

use std::sync::Arc;

use tracing::debug;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;

/// The most resolutions [`DnsResolutionsByAddressLister`] returns.
pub const MAX_LISTED: usize = 1000;

/// Domain service that returns the [`DnsResolution`]s to an address, ordered
/// by the canonical value of their subject, at most [`MAX_LISTED`] of them.
///
/// It asks the repository for one more than [`MAX_LISTED`], so it can say
/// whether the list was cut short. Returns an empty vector if none is
/// recorded.
pub struct DnsResolutionsByAddressLister {
    repository: Arc<dyn DnsResolutionRepository>,
}

impl DnsResolutionsByAddressLister {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>) -> Self {
        Self { repository }
    }

    /// Lists the resolutions to `answer`, with `true` when more than
    /// [`MAX_LISTED`] are recorded and only the first [`MAX_LISTED`] are
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns the repository's error on storage failure.
    pub async fn execute(
        &self,
        answer: DnsResolutionAnswer,
    ) -> Result<(Vec<DnsResolution>, bool), DnsResolutionRepositoryError> {
        debug!(answer = %answer, "Listing DNS resolutions by address");

        let mut dns_resolutions = self
            .repository
            .list_by_answer(&answer, MAX_LISTED + 1)
            .await?;
        let truncated = dns_resolutions.len() > MAX_LISTED;
        dns_resolutions.truncate(MAX_LISTED);

        debug!(
            answer = %answer,
            listed = dns_resolutions.len(),
            truncated,
            "Listed DNS resolutions by address"
        );
        Ok((dns_resolutions, truncated))
    }
}
