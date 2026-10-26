use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
    FailsWithUnexpected(String),
}

pub enum DeleteBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

pub enum UpdateBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsAutonomousSystem(Mutex<Option<AutonomousSystem>>),
    FailsWithUnexpected(String),
}

pub struct AutonomousSystemRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    update_behavior: UpdateBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    updated: Mutex<Vec<AutonomousSystem>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl AutonomousSystemRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_autonomous_system(autonomous_system: AutonomousSystem) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsAutonomousSystem(Mutex::new(Some(autonomous_system))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_autonomous_system_but_delete_fails(
        autonomous_system: AutonomousSystem,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_autonomous_system(autonomous_system)
        }
    }

    pub fn that_returns_autonomous_system_but_update_fails(
        autonomous_system: AutonomousSystem,
        message: String,
    ) -> Self {
        Self {
            update_behavior: UpdateBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_autonomous_system(autonomous_system)
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
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            updated: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_ids(&self) -> Vec<Uuid> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn updated(&self) -> Vec<AutonomousSystem> {
        self.updated.lock().unwrap().clone()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl AutonomousSystemRepository for AutonomousSystemRepositoryMock {
    async fn save(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => {
                Err(AutonomousSystemRepositoryError::AlreadyExists)
            }
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(AutonomousSystemRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*autonomous_system.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &AutonomousSystemId,
    ) -> Result<Option<AutonomousSystem>, AutonomousSystemRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsAutonomousSystem(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(AutonomousSystemRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn update(
        &self,
        autonomous_system: &AutonomousSystem,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        match &self.update_behavior {
            UpdateBehavior::Succeeds => {
                self.updated.lock().unwrap().push(autonomous_system.clone());
                Ok(())
            }
            UpdateBehavior::FailsWithUnexpected(message) => {
                Err(AutonomousSystemRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(
        &self,
        _id: &AutonomousSystemId,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(AutonomousSystemRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
