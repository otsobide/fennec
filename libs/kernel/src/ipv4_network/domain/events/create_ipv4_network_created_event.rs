//! Factory function for [`Ipv4NetworkCreatedEvent`].

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::events::ipv4_network_created_event::Ipv4NetworkCreatedEvent;

/// Creates an [`Ipv4NetworkCreatedEvent`] from the given IPv4 network.
pub fn create_ipv4_network_created_event(
    ipv4_network: &Ipv4Network,
) -> Result<Ipv4NetworkCreatedEvent, Ipv4NetworkRepositoryError> {
    Ok(Ipv4NetworkCreatedEvent::new(ipv4_network))
}
