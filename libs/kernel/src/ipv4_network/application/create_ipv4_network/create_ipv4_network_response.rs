//! Response for the create-ipv4-network use case.

use crate::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::Ipv4NetworkErrorEntry;

/// Response envelope returned by [`CreateIpv4NetworkCommandHandler`].
///
/// [`CreateIpv4NetworkCommandHandler`]: super::create_ipv4_network_command_handler::CreateIpv4NetworkCommandHandler
pub struct CreateIpv4NetworkResponse {
    pub error: Option<Ipv4NetworkErrorEntry>,
}
