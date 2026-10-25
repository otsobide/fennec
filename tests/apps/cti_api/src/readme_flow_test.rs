//! Replays the flow documented in the repository README, so the example cannot
//! drift away from the API it documents.

use actix_web::{test, App};
use cti_api::{build_state, configure_routes};

#[actix_web::test]
async fn readme_example_flow_works() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let source = test::TestRequest::post()
        .uri("/sources")
        .set_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "source_type": "url", "status": "active", "description": "Primary feed"
        }))
        .send_request(&app)
        .await;
    assert_eq!(source.status().as_u16(), 201, "step 1 create source");

    let url_source = test::TestRequest::post().uri("/url-sources").set_json(serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "url": "https://example.test/feed.txt", "format": "plain", "polling_interval_seconds": 3600
    })).send_request(&app).await;
    assert_eq!(
        url_source.status().as_u16(),
        201,
        "step 2 create url source"
    );

    let ioc = test::TestRequest::post().uri("/iocs").set_json(serde_json::json!({
        "id": "6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60", "ioc_type": "ipv4", "value": "198.51.100.7"
    })).send_request(&app).await;
    assert_eq!(ioc.status().as_u16(), 201, "step 3 create ioc");

    let sighting = test::TestRequest::post()
        .uri("/sightings")
        .set_json(serde_json::json!({
            "id": "7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061",
            "ioc_id": "6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60",
            "source_id": "550e8400-e29b-41d4-a716-446655440000",
            "observed_at": 1750000000u64
        }))
        .send_request(&app)
        .await;
    assert_eq!(sighting.status().as_u16(), 201, "step 4 create sighting");

    let observation = test::TestRequest::post()
        .uri("/sightings/7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061/observations")
        .set_json(serde_json::json!({ "observed_at": 1750003600u64 }))
        .send_request(&app)
        .await;
    assert_eq!(observation.status().as_u16(), 200, "step 5 observe again");

    let listed = test::TestRequest::get()
        .uri("/iocs/6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60/sightings")
        .send_request(&app)
        .await;
    assert_eq!(listed.status().as_u16(), 200, "step 6 list by ioc");
    let body: serde_json::Value = test::read_body_json(listed).await;
    assert_eq!(
        body[0]["count"], 2,
        "the second observation bumped the counter"
    );
    assert_eq!(
        body[0]["last_seen"], 1750003600u64,
        "last_seen moved forward"
    );

    // The README claims non-v4 identifiers are rejected with 400.
    let non_v4 = test::TestRequest::post().uri("/iocs").set_json(serde_json::json!({
        "id": "11111111-1111-1111-1111-111111111111", "ioc_type": "ipv4", "value": "198.51.100.9"
    })).send_request(&app).await;
    assert_eq!(non_v4.status().as_u16(), 400, "non-v4 id must be rejected");
}
