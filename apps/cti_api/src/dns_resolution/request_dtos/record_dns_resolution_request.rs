//! Request body of `POST /dns-resolutions`.

use serde::Deserialize;

/// JSON request body for `POST /dns-resolutions`: a source's report that a
/// domain name resolved to an address.
///
/// The answer is given in the field of its kind, `ipv4_address` or
/// `ipv6_address`, and exactly one of them must be present: the kind is
/// never inferred from the value. `first_seen` and `last_seen` are unix
/// seconds, and a missing `last_seen` is `first_seen`. `count` is the
/// source's cumulative count, if it counts.
///
/// Unknown fields are rejected, so a body that tries to choose the id fails
/// with `400`: the id of a resolution is always derived from its endpoints
/// ([`DnsResolutionId::from_endpoints`]).
///
/// [`DnsResolutionId::from_endpoints`]: kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId::from_endpoints
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordDnsResolutionRequest {
    pub domain_name: String,
    #[serde(default)]
    pub ipv4_address: Option<String>,
    #[serde(default)]
    pub ipv6_address: Option<String>,
    pub source_id: String,
    pub first_seen: u64,
    #[serde(default)]
    pub last_seen: Option<u64>,
    #[serde(default)]
    pub count: Option<u64>,
}
