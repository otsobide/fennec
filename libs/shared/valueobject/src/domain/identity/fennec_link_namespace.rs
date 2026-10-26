//! The namespace of the deterministic ids of links between observables.

use uuid::Uuid;

/// The UUID v5 namespace of every link id (`docs/data-model/links.md`):
/// `eda8415d-6a75-5fdc-9eb0-72698098a92f`, itself the UUID v5 of
/// `urn:fennec:link` in the RFC 4122 URL namespace.
///
/// Changing it changes every link id.
pub const FENNEC_LINK_NAMESPACE: Uuid = Uuid::from_u128(0xeda8415d_6a75_5fdc_9eb0_72698098a92f);
