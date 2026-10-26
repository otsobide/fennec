//! In-memory implementation of [`MacAddressRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// In-memory implementation of [`MacAddressRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same address twice fails with
/// [`MacAddressRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryMacAddressRepository {
    store: Mutex<HashMap<Uuid, MacAddress>>,
}

impl InMemoryMacAddressRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryMacAddressRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MacAddressRepository for InMemoryMacAddressRepository {
    async fn save(&self, mac_address: &MacAddress) -> Result<(), MacAddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *mac_address.id().value();
        if store.contains_key(&id) {
            return Err(MacAddressRepositoryError::AlreadyExists);
        }
        store.insert(id, mac_address.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &MacAddressId,
    ) -> Result<Option<MacAddress>, MacAddressRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &MacAddressId) -> Result<(), MacAddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(MacAddressRepositoryError::NotFound);
        }
        Ok(())
    }
}
