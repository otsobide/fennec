//! Entry point for the CTI API service.
//!
//! With `DATABASE_URL` set, the service persists in PostgreSQL: it connects,
//! runs the event store and kernel migrations, makes sure a year of event
//! log partitions exists, and serves on the transactional command bus.
//! Without it, everything lives in memory.

use actix_web::{App, HttpServer};
use cti_api::{build_postgres_state, build_state, configure_routes};
use kernel::kernel_migrator::KERNEL_MIGRATOR;
use shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::EnvFilter;

/// How many months of event log partitions to keep ready at startup.
const EVENT_LOG_MONTHS_AHEAD: i32 = 12;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let state = match std::env::var("DATABASE_URL") {
        Ok(database_url) => {
            let pool = PgPoolOptions::new()
                .connect(&database_url)
                .await
                .map_err(std::io::Error::other)?;
            EVENT_STORE_MIGRATOR
                .run(&pool)
                .await
                .map_err(std::io::Error::other)?;
            KERNEL_MIGRATOR
                .run(&pool)
                .await
                .map_err(std::io::Error::other)?;
            sqlx::query("SELECT event_store.ensure_partitions($1)")
                .bind(EVENT_LOG_MONTHS_AHEAD)
                .execute(&pool)
                .await
                .map_err(std::io::Error::other)?;
            info!("Persisting in PostgreSQL");
            build_postgres_state(pool)
        }
        Err(_) => {
            info!("Persisting in memory: DATABASE_URL is not set");
            build_state()
        }
    };

    let port = 8081_u16;
    info!("CTI API listening on port {port}");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
