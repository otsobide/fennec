//! Response for the delete-ipv6-network use case.

use crate::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::Ipv6NetworkErrorEntry;

/// Response envelope returned by [`DeleteIpv6NetworkCommandHandler`].
///
/// [`DeleteIpv6NetworkCommandHandler`]: super::delete_ipv6_network_command_handler::DeleteIpv6NetworkCommandHandler
pub struct DeleteIpv6NetworkResponse {
    pub error: Option<Ipv6NetworkErrorEntry>,
}
