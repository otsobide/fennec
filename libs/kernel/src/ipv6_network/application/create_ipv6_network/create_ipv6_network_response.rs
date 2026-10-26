//! Response for the create-ipv6-network use case.

use crate::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::Ipv6NetworkErrorEntry;

/// Response envelope returned by [`CreateIpv6NetworkCommandHandler`].
///
/// [`CreateIpv6NetworkCommandHandler`]: super::create_ipv6_network_command_handler::CreateIpv6NetworkCommandHandler
pub struct CreateIpv6NetworkResponse {
    pub error: Option<Ipv6NetworkErrorEntry>,
}
