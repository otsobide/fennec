//! The shape of a database operation run by the unit of work or a session.

use std::future::Future;
use std::pin::Pin;

/// A boxed database operation borrowing a connection for `'c`.
///
/// Repositories pass closures `|connection| Box::pin(async move { ... })`
/// that own every value they bind, so the same operation can run on the unit
/// of work's transaction or on a pooled connection.
pub type PgOperation<'c, T> = Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'c>>;
