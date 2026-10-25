//! In-memory implementation of [`Ipv4AddressRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// In-memory implementation of [`Ipv4AddressRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same address twice fails with
/// [`Ipv4AddressRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryIpv4AddressRepository {
    store: Mutex<HashMap<Uuid, Ipv4Address>>,
}

impl InMemoryIpv4AddressRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIpv4AddressRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Ipv4AddressRepository for InMemoryIpv4AddressRepository {
    async fn save(&self, ipv4_address: &Ipv4Address) -> Result<(), Ipv4AddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *ipv4_address.id().value();
        if store.contains_key(&id) {
            return Err(Ipv4AddressRepositoryError::AlreadyExists);
        }
        store.insert(id, ipv4_address.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv4AddressId,
    ) -> Result<Option<Ipv4Address>, Ipv4AddressRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &Ipv4AddressId) -> Result<(), Ipv4AddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(Ipv4AddressRepositoryError::NotFound);
        }
        Ok(())
    }
}
