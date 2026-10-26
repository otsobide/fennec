//! Query string of `GET /dns-resolutions/co-resolved`.

use serde::Deserialize;

/// Query string of `GET /dns-resolutions/co-resolved?domain_name=<name>`:
/// the domain name to pivot from. Unknown parameters are rejected with
/// `400`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCoResolvedDomainNamesRequest {
    pub domain_name: String,
}
