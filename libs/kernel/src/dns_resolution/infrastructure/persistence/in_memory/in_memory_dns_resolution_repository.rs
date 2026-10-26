//! In-memory implementation of [`DnsResolutionRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// In-memory implementation of [`DnsResolutionRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`], keyed by the derived id.
///
/// Each entry is a whole [`DnsResolution`] with its observations, so an
/// update replaces them all. Lists are ordered as the PostgreSQL adapter
/// orders them: by address, IPv4 before IPv6, and by the byte order of the
/// name. Intended for tests and local development.
pub struct InMemoryDnsResolutionRepository {
    store: Mutex<HashMap<Uuid, DnsResolution>>,
}

impl InMemoryDnsResolutionRepository {
    /// Creates an empty repository.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryDnsResolutionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DnsResolutionRepository for InMemoryDnsResolutionRepository {
    async fn save(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *dns_resolution.id().value();
        if store.contains_key(&id) {
            return Err(DnsResolutionRepositoryError::AlreadyExists);
        }
        store.insert(id, dns_resolution.clone());
        Ok(())
    }

    async fn update(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *dns_resolution.id().value();
        if !store.contains_key(&id) {
            return Err(DnsResolutionRepositoryError::NotFound);
        }
        store.insert(id, dns_resolution.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &DnsResolutionId,
    ) -> Result<Option<DnsResolution>, DnsResolutionRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn list_by_subject(
        &self,
        subject: &DnsResolutionSubject,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        let store = self.store.lock().unwrap();
        let mut resolutions: Vec<DnsResolution> = store
            .values()
            .filter(|dns_resolution| dns_resolution.subject() == subject)
            .cloned()
            .collect();
        resolutions.sort_by_key(|dns_resolution| dns_resolution.answer().ip());
        resolutions.truncate(limit);
        Ok(resolutions)
    }

    async fn list_by_answer(
        &self,
        answer: &DnsResolutionAnswer,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        let store = self.store.lock().unwrap();
        let mut resolutions: Vec<DnsResolution> = store
            .values()
            .filter(|dns_resolution| dns_resolution.answer() == answer)
            .cloned()
            .collect();
        resolutions.sort_by(|left, right| left.subject().value().cmp(right.subject().value()));
        resolutions.truncate(limit);
        Ok(resolutions)
    }

    async fn delete(&self, id: &DnsResolutionId) -> Result<(), DnsResolutionRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(DnsResolutionRepositoryError::NotFound);
        }
        Ok(())
    }
}
