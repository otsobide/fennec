//! In-memory implementation of [`AutonomousSystemRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// In-memory implementation of [`AutonomousSystemRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same autonomous system twice fails with
/// [`AutonomousSystemRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryAutonomousSystemRepository {
    store: Mutex<HashMap<Uuid, AutonomousSystem>>,
}

impl InMemoryAutonomousSystemRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryAutonomousSystemRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AutonomousSystemRepository for InMemoryAutonomousSystemRepository {
    async fn save(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *autonomous_system.id().value();
        if store.contains_key(&id) {
            return Err(AutonomousSystemRepositoryError::AlreadyExists);
        }
        store.insert(id, autonomous_system.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &AutonomousSystemId,
    ) -> Result<Option<AutonomousSystem>, AutonomousSystemRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn update(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *autonomous_system.id().value();
        if !store.contains_key(&id) {
            return Err(AutonomousSystemRepositoryError::NotFound);
        }
        store.insert(id, autonomous_system.clone());
        Ok(())
    }

    async fn delete(&self, id: &AutonomousSystemId) -> Result<(), AutonomousSystemRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(AutonomousSystemRepositoryError::NotFound);
        }
        Ok(())
    }
}
