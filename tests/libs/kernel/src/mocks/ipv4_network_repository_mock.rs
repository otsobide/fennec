use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

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
    ReturnsIpv4Network(Mutex<Option<Ipv4Network>>),
    FailsWithUnexpected(String),
}

pub struct Ipv4NetworkRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl Ipv4NetworkRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv4_network(ipv4_network: Ipv4Network) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsIpv4Network(Mutex::new(Some(ipv4_network))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv4_network_but_delete_fails(
        ipv4_network: Ipv4Network,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_ipv4_network(ipv4_network)
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
impl Ipv4NetworkRepository for Ipv4NetworkRepositoryMock {
    async fn save(&self, ipv4_network: &Ipv4Network) -> Result<(), Ipv4NetworkRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(Ipv4NetworkRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4NetworkRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*ipv4_network.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &Ipv4NetworkId,
    ) -> Result<Option<Ipv4Network>, Ipv4NetworkRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsIpv4Network(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4NetworkRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &Ipv4NetworkId) -> Result<(), Ipv4NetworkRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4NetworkRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
