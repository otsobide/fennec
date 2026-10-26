//! Domain service for listing the DNS resolutions of a domain name.

use std::sync::Arc;

use tracing::debug;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// The most resolutions [`DnsResolutionsByDomainNameLister`] returns.
pub const MAX_LISTED: usize = 1000;

/// Domain service that returns the [`DnsResolution`]s of a domain name,
/// ordered by the address of their answer with IPv4 before IPv6 (the order
/// of `inet`), at most [`MAX_LISTED`] of them.
///
/// It asks the repository for one more than [`MAX_LISTED`], so it can say
/// whether the list was cut short. Returns an empty vector if none is
/// recorded.
pub struct DnsResolutionsByDomainNameLister {
    repository: Arc<dyn DnsResolutionRepository>,
}

impl DnsResolutionsByDomainNameLister {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>) -> Self {
        Self { repository }
    }

    /// Lists the resolutions of `subject`, with `true` when more than
    /// [`MAX_LISTED`] are recorded and only the first [`MAX_LISTED`] are
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns the repository's error on storage failure.
    pub async fn execute(
        &self,
        subject: DnsResolutionSubject,
    ) -> Result<(Vec<DnsResolution>, bool), DnsResolutionRepositoryError> {
        debug!(subject = %subject, "Listing DNS resolutions by domain name");

        let mut dns_resolutions = self
            .repository
            .list_by_subject(&subject, MAX_LISTED + 1)
            .await?;
        let truncated = dns_resolutions.len() > MAX_LISTED;
        dns_resolutions.truncate(MAX_LISTED);

        debug!(
            subject = %subject,
            listed = dns_resolutions.len(),
            truncated,
            "Listed DNS resolutions by domain name"
        );
        Ok((dns_resolutions, truncated))
    }
}
