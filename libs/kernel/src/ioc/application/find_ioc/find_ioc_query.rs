//! Query for finding an ioc by id.

use shared_cqrs::query::domain::query::Query;

use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Query that requests a single ioc by its [`IocId`].
pub struct FindIocQuery {
    pub id: IocId,
}

impl Query for FindIocQuery {}
