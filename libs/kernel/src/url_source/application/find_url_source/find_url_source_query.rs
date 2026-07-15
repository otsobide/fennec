//! Query for finding a url source by id.

use shared_cqrs::query::domain::query::Query;

use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Query that requests a single url source by its [`UrlSourceId`].
pub struct FindUrlSourceQuery {
    pub id: UrlSourceId,
}

impl Query for FindUrlSourceQuery {}
