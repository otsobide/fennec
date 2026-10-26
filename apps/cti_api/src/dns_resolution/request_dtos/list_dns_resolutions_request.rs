//! Query string of `GET /dns-resolutions`.

use serde::Deserialize;

/// Query string of `GET /dns-resolutions?domain_name=<name>`,
/// `?ipv4_address=<address>` or `?ipv6_address=<address>`.
///
/// Exactly one parameter must be present, and it names the kind of the
/// endpoint the list filters on. Unknown parameters are rejected with `400`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListDnsResolutionsRequest {
    #[serde(default)]
    pub domain_name: Option<String>,
    #[serde(default)]
    pub ipv4_address: Option<String>,
    #[serde(default)]
    pub ipv6_address: Option<String>,
}
