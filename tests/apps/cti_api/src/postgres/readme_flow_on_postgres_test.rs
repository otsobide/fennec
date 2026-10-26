use actix_web::{test, App};
use serde_json::json;
use sqlx::PgPool;

use cti_api::{build_postgres_state, configure_routes};

use crate::src::postgres::support::{event_names, migrate_event_store};

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_records_every_step_of_the_readme_flow_in_the_event_log(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;

    let steps = [
        (
            "/sources",
            json!({
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "source_type": "url", "status": "active", "description": "Primary feed"
            }),
        ),
        (
            "/url-sources",
            json!({
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "url": "https://example.test/feed.txt", "format": "plain",
                "polling_interval_seconds": 3600
            }),
        ),
        ("/ipv4-addresses", json!({ "value": "198.51.100.7" })),
        (
            "/sightings",
            json!({
                "id": "7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061",
                "observable_kind": "ipv4_address",
                "observable_id": "12592121-be4c-5a93-a813-5a5af16e50db",
                "source_id": "550e8400-e29b-41d4-a716-446655440000",
                "observed_at": 1_750_000_000u64
            }),
        ),
        (
            "/sightings/7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061/observations",
            json!({ "observed_at": 1_750_003_600u64 }),
        ),
    ];
    for (uri, body) in steps {
        let resp = test::TestRequest::post()
            .uri(uri)
            .set_json(body)
            .send_request(&app)
            .await;
        assert!(
            resp.status().is_success(),
            "{uri} answered {}",
            resp.status()
        );
    }

    let listed = test::TestRequest::get()
        .uri("/observables/ipv4_address/12592121-be4c-5a93-a813-5a5af16e50db/sightings")
        .send_request(&app)
        .await;
    let body: serde_json::Value = test::read_body_json(listed).await;
    assert_eq!(body[0]["count"], 2);
    assert_eq!(
        event_names(&pool).await,
        vec![
            "fennec.kernel.source.created",
            "fennec.kernel.url_source.created",
            "fennec.kernel.ipv4_address.created",
            "fennec.kernel.sighting.created",
            "fennec.kernel.sighting.observed",
        ]
    );
}
