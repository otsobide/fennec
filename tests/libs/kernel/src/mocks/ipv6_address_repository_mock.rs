use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

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
    ReturnsIpv6Address(Mutex<Option<Ipv6Address>>),
    FailsWithUnexpected(String),
}

pub struct Ipv6AddressRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl Ipv6AddressRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv6_address(ipv6_address: Ipv6Address) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsIpv6Address(Mutex::new(Some(ipv6_address))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_ipv6_address_but_delete_fails(
        ipv6_address: Ipv6Address,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_ipv6_address(ipv6_address)
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
impl Ipv6AddressRepository for Ipv6AddressRepositoryMock {
    async fn save(&self, ipv6_address: &Ipv6Address) -> Result<(), Ipv6AddressRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(Ipv6AddressRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(Ipv6AddressRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*ipv6_address.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &Ipv6AddressId,
    ) -> Result<Option<Ipv6Address>, Ipv6AddressRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsIpv6Address(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(Ipv6AddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &Ipv6AddressId) -> Result<(), Ipv6AddressRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(Ipv6AddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
