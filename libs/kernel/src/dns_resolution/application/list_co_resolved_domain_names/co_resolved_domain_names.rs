//! Read model for the result of the co-resolution pivot.

use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

use super::co_resolved_domain_name::CoResolvedDomainName;

/// The domain names that shared an address with `subject` at the same time,
/// as [`CoResolvedDomainNamesLister`] found them.
///
/// Invariant: `domain_names` is sorted by the canonical value of the name.
/// `truncated` is `true` when a fetch hit its limit, so more co-resolved
/// names or shared answers may exist than the result lists.
///
/// [`CoResolvedDomainNamesLister`]: super::co_resolved_domain_names_lister::CoResolvedDomainNamesLister
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoResolvedDomainNames {
    subject: DnsResolutionSubject,
    domain_names: Vec<CoResolvedDomainName>,
    truncated: bool,
}

impl CoResolvedDomainNames {
    /// Builds the result, sorting the domain names by their canonical value.
    pub fn new(
        subject: DnsResolutionSubject,
        mut domain_names: Vec<CoResolvedDomainName>,
        truncated: bool,
    ) -> Self {
        domain_names
            .sort_by(|left, right| left.domain_name().value().cmp(right.domain_name().value()));
        Self {
            subject,
            domain_names,
            truncated,
        }
    }

    /// Returns the queried domain name.
    pub fn subject(&self) -> &DnsResolutionSubject {
        &self.subject
    }

    /// Returns the co-resolved domain names, sorted by canonical value.
    pub fn domain_names(&self) -> &[CoResolvedDomainName] {
        &self.domain_names
    }

    /// Returns `true` when a fetch hit its limit and the result may be
    /// incomplete.
    pub fn truncated(&self) -> bool {
        self.truncated
    }
}
