//! Response for the delete-ipv4-network use case.

use crate::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::Ipv4NetworkErrorEntry;

/// Response envelope returned by [`DeleteIpv4NetworkCommandHandler`].
///
/// [`DeleteIpv4NetworkCommandHandler`]: super::delete_ipv4_network_command_handler::DeleteIpv4NetworkCommandHandler
pub struct DeleteIpv4NetworkResponse {
    pub error: Option<Ipv4NetworkErrorEntry>,
}
