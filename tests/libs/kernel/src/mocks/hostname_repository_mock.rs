use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::hostname::domain::entities::hostname::Hostname;
use kernel::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
    FailsWithUnexpected(String),
}

pub enum DeleteBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsHostname(Mutex<Option<Hostname>>),
    FailsWithUnexpected(String),
}

pub struct HostnameRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl HostnameRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_hostname(hostname: Hostname) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsHostname(Mutex::new(Some(hostname))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_hostname_but_delete_fails(hostname: Hostname, message: String) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_hostname(hostname)
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::FailsWithUnexpected(message),
        )
    }

    fn with(save_behavior: SaveBehavior, find_by_id_behavior: FindByIdBehavior) -> Self {
        Self {
            save_behavior,
            find_by_id_behavior,
            delete_behavior: DeleteBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_ids(&self) -> Vec<Uuid> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl HostnameRepository for HostnameRepositoryMock {
    async fn save(&self, hostname: &Hostname) -> Result<(), HostnameRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(HostnameRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(HostnameRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids.lock().unwrap().push(*hostname.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &HostnameId,
    ) -> Result<Option<Hostname>, HostnameRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsHostname(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(HostnameRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &HostnameId) -> Result<(), HostnameRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(HostnameRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
