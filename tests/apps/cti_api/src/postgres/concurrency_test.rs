use actix_web::{test, App};
use futures::future::join_all;
use serde_json::json;
use sqlx::PgPool;

use cti_api::{build_postgres_state, configure_routes};

use crate::src::postgres::support::{count, migrate_event_store};

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_creates_one_address_and_one_event_under_concurrent_registrations(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;

    let statuses: Vec<u16> = join_all((0..8).map(|_| {
        test::TestRequest::post()
            .uri("/ipv4-addresses")
            .set_json(json!({ "value": "198.51.100.7" }))
            .send_request(&app)
    }))
    .await
    .into_iter()
    .map(|resp| resp.status().as_u16())
    .collect();

    assert_eq!(statuses.iter().filter(|status| **status == 201).count(), 1);
    assert_eq!(statuses.iter().filter(|status| **status == 409).count(), 7);
    assert_eq!(
        count(&pool, "SELECT count(*) FROM kernel.ipv4_address").await,
        1
    );
    assert_eq!(
        count(&pool, "SELECT count(*) FROM event_store.domain_event").await,
        1
    );
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_loses_no_concurrent_observation_of_a_sighting(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;
    let created = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061",
            "observable_kind": "ipv4_address",
            "observable_id": "12592121-be4c-5a93-a813-5a5af16e50db",
            "source_id": "550e8400-e29b-41d4-a716-446655440000",
            "observed_at": 1_750_000_000u64
        }))
        .send_request(&app)
        .await;
    assert_eq!(created.status(), 201);

    let statuses: Vec<u16> = join_all((1..=10u64).map(|offset| {
        test::TestRequest::post()
            .uri("/sightings/7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061/observations")
            .set_json(json!({ "observed_at": 1_750_000_000u64 + offset }))
            .send_request(&app)
    }))
    .await
    .into_iter()
    .map(|resp| resp.status().as_u16())
    .collect();

    assert!(statuses.iter().all(|status| *status == 200), "{statuses:?}");
    assert_eq!(count(&pool, "SELECT count FROM kernel.sighting").await, 11);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM event_store.domain_event WHERE event_name = 'fennec.kernel.sighting.observed'"
        )
        .await,
        10
    );
}
