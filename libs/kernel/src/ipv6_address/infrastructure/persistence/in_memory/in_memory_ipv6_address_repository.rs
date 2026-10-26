//! In-memory implementation of [`Ipv6AddressRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// In-memory implementation of [`Ipv6AddressRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same address twice fails with
/// [`Ipv6AddressRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryIpv6AddressRepository {
    store: Mutex<HashMap<Uuid, Ipv6Address>>,
}

impl InMemoryIpv6AddressRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIpv6AddressRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Ipv6AddressRepository for InMemoryIpv6AddressRepository {
    async fn save(&self, ipv6_address: &Ipv6Address) -> Result<(), Ipv6AddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *ipv6_address.id().value();
        if store.contains_key(&id) {
            return Err(Ipv6AddressRepositoryError::AlreadyExists);
        }
        store.insert(id, ipv6_address.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv6AddressId,
    ) -> Result<Option<Ipv6Address>, Ipv6AddressRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &Ipv6AddressId) -> Result<(), Ipv6AddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(Ipv6AddressRepositoryError::NotFound);
        }
        Ok(())
    }
}
