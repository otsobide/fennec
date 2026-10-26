use actix_web::{test, App};
use serde_json::json;
use sqlx::PgPool;

use cti_api::{build_postgres_state, configure_routes};

use crate::src::postgres::support::{count, event_names, migrate_event_store};

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_commits_an_ipv4_address_with_its_created_event(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;

    let resp = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "89.19.209.154" }))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 201);
    assert_eq!(
        count(&pool, "SELECT count(*) FROM kernel.ipv4_address").await,
        1
    );
    assert_eq!(
        event_names(&pool).await,
        vec!["fennec.kernel.ipv4_address.created"]
    );
    let payload: serde_json::Value =
        sqlx::query_scalar("SELECT payload FROM event_store.domain_event")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(payload["id"], "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
    assert_eq!(payload["value"], "89.19.209.154");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_leaves_one_row_and_one_event_after_a_duplicate(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;

    for value in ["198.51.100.7", "198.51.100.7/32"] {
        test::TestRequest::post()
            .uri("/ipv4-addresses")
            .set_json(json!({ "value": value }))
            .send_request(&app)
            .await;
    }

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
async fn it_returns_500_and_keeps_nothing_when_the_event_log_rejects_the_write(pool: PgPool) {
    migrate_event_store(&pool).await;
    sqlx::query(
        "DO $$ BEGIN
             EXECUTE format('DROP TABLE event_store.domain_event_%s',
                            to_char(now() AT TIME ZONE 'UTC', 'YYYY_MM'));
         END $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;

    let resp = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "198.51.100.7" }))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), 500);
    assert_eq!(
        count(&pool, "SELECT count(*) FROM kernel.ipv4_address").await,
        0
    );
    assert_eq!(
        count(&pool, "SELECT count(*) FROM event_store.domain_event").await,
        0
    );
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_answers_an_unchanged_dns_resolution_report_without_writing_an_event(pool: PgPool) {
    migrate_event_store(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(build_postgres_state(pool.clone()))
            .configure(configure_routes),
    )
    .await;
    let report = json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": "aaaaaaaa-1111-4111-8111-111111111111",
        "first_seen": 1_700_000_000u64,
        "last_seen": 1_700_086_400u64,
        "count": 42u64
    });

    let mut outcomes = Vec::new();
    for _ in 0..2 {
        let resp = test::TestRequest::post()
            .uri("/dns-resolutions")
            .set_json(report.clone())
            .send_request(&app)
            .await;
        let status = resp.status().as_u16();
        let body: serde_json::Value = test::read_body_json(resp).await;
        outcomes.push((status, body["outcome"].as_str().unwrap().to_string()));
    }

    assert_eq!(
        outcomes,
        vec![(201, "created".to_string()), (200, "unchanged".to_string())]
    );
    assert_eq!(
        event_names(&pool).await,
        vec![
            "fennec.kernel.dns_resolution.created",
            "fennec.kernel.dns_resolution.observed"
        ]
    );
    assert_eq!(
        count(&pool, "SELECT count FROM kernel.dns_resolution_observation").await,
        42
    );
}
