//! Domain service for finding a single ioc.

use std::sync::Arc;

use tracing::debug;

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::repositories::ioc_repository::IocRepository;
use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Domain service that looks up a single [`Ioc`] by id.
pub struct IocFinder {
    repository: Arc<dyn IocRepository>,
}

impl IocFinder {
    pub fn new(repository: Arc<dyn IocRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: IocId) -> Result<Ioc, IocRepositoryError> {
        debug!(id = %id, "Finding ioc");
        let ioc = self.repository.find_by_id(&id).await?;

        ioc.ok_or(IocRepositoryError::NotFound)
    }
}
