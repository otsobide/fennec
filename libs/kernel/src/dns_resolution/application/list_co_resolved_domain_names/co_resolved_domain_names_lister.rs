//! Domain service for the co-resolution pivot: the domain names that shared
//! an address with a given domain name at the same time.

use std::collections::HashMap;
use std::sync::Arc;

use tracing::debug;

use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

use super::co_resolved_domain_name::CoResolvedDomainName;
use super::co_resolved_domain_names::CoResolvedDomainNames;

/// The most answers of the queried domain name the pivot follows.
pub const ANSWER_LIMIT: usize = 100;

/// The most resolutions to one answer the pivot reads.
pub const PER_ANSWER_LIMIT: usize = 1000;

/// Domain service that pivots from a domain name through its addresses to
/// the other domain names that resolved to them.
///
/// It reads the resolutions of the queried name (at most [`ANSWER_LIMIT`]),
/// then for each one the resolutions to its answer (at most
/// [`PER_ANSWER_LIMIT`]). Another name is co-resolved when its resolution to
/// that answer has a window that overlaps the window of the queried name's
/// resolution ([`DnsResolution::window`]); the queried name itself is never
/// listed. Each fetch asks for one more than its limit, so the result can say
/// it was truncated.
///
/// [`DnsResolution::window`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution::window
pub struct CoResolvedDomainNamesLister {
    repository: Arc<dyn DnsResolutionRepository>,
}

impl CoResolvedDomainNamesLister {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>) -> Self {
        Self { repository }
    }

    /// Lists the domain names co-resolved with `subject`, sorted by name,
    /// each with the answers they shared sorted by address.
    ///
    /// # Errors
    ///
    /// Returns the repository's error on storage failure.
    pub async fn execute(
        &self,
        subject: DnsResolutionSubject,
    ) -> Result<CoResolvedDomainNames, DnsResolutionRepositoryError> {
        debug!(subject = %subject, "Listing co-resolved domain names");

        let mut links = self
            .repository
            .list_by_subject(&subject, ANSWER_LIMIT + 1)
            .await?;
        let mut truncated = links.len() > ANSWER_LIMIT;
        links.truncate(ANSWER_LIMIT);

        let mut shared: HashMap<DnsResolutionSubject, Vec<DnsResolutionAnswer>> = HashMap::new();
        for link in &links {
            let mut others = self
                .repository
                .list_by_answer(link.answer(), PER_ANSWER_LIMIT + 1)
                .await?;
            if others.len() > PER_ANSWER_LIMIT {
                truncated = true;
                others.truncate(PER_ANSWER_LIMIT);
            }

            let window = link.window();
            for other in &others {
                if other.subject() == &subject || !other.window().overlaps(&window) {
                    continue;
                }
                shared
                    .entry(other.subject().clone())
                    .or_default()
                    .push(*link.answer());
            }
        }

        let domain_names = shared
            .into_iter()
            .map(|(domain_name, answers)| CoResolvedDomainName::new(domain_name, answers))
            .collect::<Vec<_>>();
        debug!(
            subject = %subject,
            co_resolved = domain_names.len(),
            truncated,
            "Listed co-resolved domain names"
        );
        Ok(CoResolvedDomainNames::new(subject, domain_names, truncated))
    }
}
