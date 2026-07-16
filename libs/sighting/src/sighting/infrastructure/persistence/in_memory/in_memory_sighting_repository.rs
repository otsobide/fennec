//! In-memory implementation of [`SightingRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// In-memory implementation of [`SightingRepository`] backed by a [`HashMap`]
/// protected by a [`Mutex`], keyed by sighting id.
///
/// Intended for tests and local development.
pub struct InMemorySightingRepository {
    store: Mutex<HashMap<Uuid, Sighting>>,
}

impl InMemorySightingRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemorySightingRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SightingRepository for InMemorySightingRepository {
    async fn save(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        let mut store = self.store.lock().unwrap();

        // Pair collision takes precedence over id collision so callers get the
        // existing id and can observe it immediately.
        if let Some(existing) = store.values().find(|s| {
            s.ioc_id() == sighting.ioc_id() && s.source_id() == sighting.source_id()
        }) {
            return Err(SightingRepositoryError::PairAlreadyExists {
                existing_id: existing.id().clone(),
            });
        }

        let id = *sighting.id().value();
        if store.contains_key(&id) {
            return Err(SightingRepositoryError::IdAlreadyExists);
        }

        store.insert(id, sighting.clone());
        Ok(())
    }

    async fn update(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *sighting.id().value();
        if !store.contains_key(&id) {
            return Err(SightingRepositoryError::NotFound);
        }
        store.insert(id, sighting.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &SightingId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn find_by_ioc_and_source(
        &self,
        ioc_id: &SightingIocId,
        source_id: &SightingSourceId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .find(|s| s.ioc_id() == ioc_id && s.source_id() == source_id)
            .cloned())
    }

    async fn find_by_ioc_id(
        &self,
        ioc_id: &SightingIocId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .filter(|s| s.ioc_id() == ioc_id)
            .cloned()
            .collect())
    }

    async fn find_by_source_id(
        &self,
        source_id: &SightingSourceId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .values()
            .filter(|s| s.source_id() == source_id)
            .cloned()
            .collect())
    }

    async fn delete(&self, id: &SightingId) -> Result<(), SightingRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(SightingRepositoryError::NotFound);
        }
        Ok(())
    }
}
