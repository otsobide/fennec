use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

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
    ReturnsIpv4Address(Mutex<Option<Ipv4Address>>),
    FailsWithUnexpected(String),
}

pub struct Ipv4AddressRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl Ipv4AddressRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv4_address(ipv4_address: Ipv4Address) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsIpv4Address(Mutex::new(Some(ipv4_address))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv4_address_but_delete_fails(
        ipv4_address: Ipv4Address,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_ipv4_address(ipv4_address)
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
impl Ipv4AddressRepository for Ipv4AddressRepositoryMock {
    async fn save(&self, ipv4_address: &Ipv4Address) -> Result<(), Ipv4AddressRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(Ipv4AddressRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4AddressRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*ipv4_address.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &Ipv4AddressId,
    ) -> Result<Option<Ipv4Address>, Ipv4AddressRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsIpv4Address(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4AddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &Ipv4AddressId) -> Result<(), Ipv4AddressRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(Ipv4AddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
