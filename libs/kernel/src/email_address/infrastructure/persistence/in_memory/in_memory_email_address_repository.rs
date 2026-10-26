//! In-memory implementation of [`EmailAddressRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// In-memory implementation of [`EmailAddressRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same address twice fails with
/// [`EmailAddressRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryEmailAddressRepository {
    store: Mutex<HashMap<Uuid, EmailAddress>>,
}

impl InMemoryEmailAddressRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryEmailAddressRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailAddressRepository for InMemoryEmailAddressRepository {
    async fn save(&self, email_address: &EmailAddress) -> Result<(), EmailAddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *email_address.id().value();
        if store.contains_key(&id) {
            return Err(EmailAddressRepositoryError::AlreadyExists);
        }
        store.insert(id, email_address.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &EmailAddressId,
    ) -> Result<Option<EmailAddress>, EmailAddressRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &EmailAddressId) -> Result<(), EmailAddressRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(EmailAddressRepositoryError::NotFound);
        }
        Ok(())
    }
}
