//! Factory function for [`Ipv4NetworkDeletedEvent`].

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::events::ipv4_network_deleted_event::Ipv4NetworkDeletedEvent;

/// Creates an [`Ipv4NetworkDeletedEvent`] from the deleted IPv4 network.
pub fn create_ipv4_network_deleted_event(
    ipv4_network: &Ipv4Network,
) -> Result<Ipv4NetworkDeletedEvent, Ipv4NetworkRepositoryError> {
    Ok(Ipv4NetworkDeletedEvent::new(ipv4_network))
}
