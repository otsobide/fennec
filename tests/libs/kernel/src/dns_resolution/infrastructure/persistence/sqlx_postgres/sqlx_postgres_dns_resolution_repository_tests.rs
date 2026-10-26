use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use kernel::dns_resolution::infrastructure::persistence::sqlx_postgres::sqlx_postgres_dns_resolution_repository::SqlxPostgresDnsResolutionRepository;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;
use crate::src::dns_resolution::infrastructure::persistence::dns_resolution_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_resolution_with_every_observation(pool: PgPool) {
    contract::it_finds_a_saved_resolution_with_every_observation(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_recording_the_same_resolution_twice(pool: PgPool) {
    contract::it_rejects_recording_the_same_resolution_twice(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresDnsResolutionRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_adds_an_observation_on_update(pool: PgPool) {
    contract::it_adds_an_observation_on_update(&SqlxPostgresDnsResolutionRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_changes_an_observation_on_update(pool: PgPool) {
    contract::it_changes_an_observation_on_update(&SqlxPostgresDnsResolutionRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_removes_an_observation_on_update(pool: PgPool) {
    contract::it_removes_an_observation_on_update(&SqlxPostgresDnsResolutionRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_adds_changes_and_removes_observations_in_one_update(pool: PgPool) {
    contract::it_adds_changes_and_removes_observations_in_one_update(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_updating_an_unknown_resolution(pool: PgPool) {
    contract::it_returns_not_found_when_updating_an_unknown_resolution(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lists_the_resolutions_of_a_subject_by_answer(pool: PgPool) {
    contract::it_lists_the_resolutions_of_a_subject_by_answer(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_limits_the_resolutions_of_a_subject(pool: PgPool) {
    contract::it_limits_the_resolutions_of_a_subject(&SqlxPostgresDnsResolutionRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lists_the_resolutions_to_an_answer_by_subject(pool: PgPool) {
    contract::it_lists_the_resolutions_to_an_answer_by_subject(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_limits_the_resolutions_to_an_answer(pool: PgPool) {
    contract::it_limits_the_resolutions_to_an_answer(&SqlxPostgresDnsResolutionRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lists_nothing_for_an_unknown_endpoint(pool: PgPool) {
    contract::it_lists_nothing_for_an_unknown_endpoint(&SqlxPostgresDnsResolutionRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_resolution(pool: PgPool) {
    contract::it_deletes_a_saved_resolution(&SqlxPostgresDnsResolutionRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_resolution_again(pool: PgPool) {
    contract::it_accepts_a_deleted_resolution_again(&SqlxPostgresDnsResolutionRepository::new(
        pool,
    ))
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresDnsResolutionRepository::new(pool),
    )
    .await;
}

/// Inserts a link row as is, bypassing the value objects.
async fn insert_link(
    pool: &PgPool,
    id: Uuid,
    (subject_kind, subject_value): (&str, &str),
    (answer_kind, answer_value): (&str, &str),
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO kernel.dns_resolution (id, subject_kind, subject_value, answer_kind, answer_value, created_at)
         VALUES ($1, $2, $3, $4, $5::inet, now())",
    )
    .bind(id)
    .bind(subject_kind)
    .bind(subject_value)
    .bind(answer_kind)
    .bind(answer_value)
    .execute(pool)
    .await
}

/// Inserts an observation row of a new source as is, bypassing the value
/// objects.
async fn insert_observation(
    pool: &PgPool,
    dns_resolution_id: Uuid,
    (first_seen, last_seen): (&str, &str),
    count: i64,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO kernel.dns_resolution_observation
             (dns_resolution_id, source_id, first_seen, last_seen, count, created_at, updated_at)
         VALUES ($1, $2, $3::timestamptz, $4::timestamptz, $5, now(), now())",
    )
    .bind(dns_resolution_id)
    .bind(Uuid::new_v4())
    .bind(first_seen)
    .bind(last_seen)
    .bind(count)
    .execute(pool)
    .await
}

async fn observation_rows(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM kernel.dns_resolution_observation")
        .fetch_one(pool)
        .await
        .unwrap()
}

fn violated_constraint(result: Result<PgQueryResult, sqlx::Error>) -> Option<String> {
    let error = result.err()?;
    error
        .as_database_error()?
        .constraint()
        .map(|constraint| constraint.to_string())
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_endpoints(pool: PgPool) {
    let repository = SqlxPostgresDnsResolutionRepository::new(pool.clone());
    repository
        .save(&DnsResolutionMother::create(
            DnsResolutionSubjectMother::create("WWW.Example.COM."),
            DnsResolutionAnswerMother::ipv6("2001:DB8:0:0:0:0:0:1/128"),
        ))
        .await
        .unwrap();

    let stored: (String, String, String, String, i32) = sqlx::query_as(
        "SELECT subject_kind, subject_value, answer_kind, host(answer_value), masklen(answer_value)
         FROM kernel.dns_resolution",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(
        stored,
        (
            "domain_name".to_string(),
            "www.example.com".to_string(),
            "ipv6_address".to_string(),
            "2001:db8::1".to_string(),
            128
        )
    );
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_one_row_per_observation_and_deletes_them_with_the_resolution(pool: PgPool) {
    let repository = SqlxPostgresDnsResolutionRepository::new(pool.clone());
    let (observed, _) = DnsResolutionMother::random()
        .observe(
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
            DnsResolutionUpdatedAt::now(),
        )
        .unwrap();
    repository.save(&observed).await.unwrap();
    let before = observation_rows(&pool).await;

    repository.delete(observed.id()).await.unwrap();

    assert_eq!(before, 2);
    assert_eq!(observation_rows(&pool).await, 0);
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_endpoints(pool: PgPool) {
    let tampered = *DnsResolutionMother::random().id();
    insert_link(
        &pool,
        *tampered.value(),
        ("domain_name", "example.com"),
        ("ipv4_address", "93.184.216.34"),
    )
    .await
    .unwrap();
    insert_observation(
        &pool,
        *tampered.value(),
        ("2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z"),
        1,
    )
    .await
    .unwrap();
    let repository = SqlxPostgresDnsResolutionRepository::new(pool);

    let found = repository.find_by_id(&tampered).await;
    let by_subject = repository
        .list_by_subject(&DnsResolutionSubjectMother::create("example.com"), 10)
        .await;
    let by_answer = repository
        .list_by_answer(&DnsResolutionAnswerMother::ipv4("93.184.216.34"), 10)
        .await;

    assert!(matches!(
        found,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(matches!(
        by_subject,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(matches!(
        by_answer,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_resolution_without_observations(pool: PgPool) {
    let example = DnsResolutionMother::example();
    insert_link(
        &pool,
        *example.id().value(),
        ("domain_name", "example.com"),
        ("ipv4_address", "93.184.216.34"),
    )
    .await
    .unwrap();

    let result = SqlxPostgresDnsResolutionRepository::new(pool)
        .find_by_id(example.id())
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_subject_that_is_not_a_canonical_domain_name_in_the_table(pool: PgPool) {
    let too_long = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62)
    );
    for (kind, value) in [
        ("hostname", "example.com"),
        ("domain_name", "Example.com"),
        ("domain_name", "example.com."),
        ("domain_name", "localhost"),
        ("domain_name", "example.123"),
        ("domain_name", "exa mple.com"),
        ("domain_name", too_long.as_str()),
    ] {
        let result = insert_link(
            &pool,
            Uuid::new_v4(),
            (kind, value),
            ("ipv4_address", "192.0.2.1"),
        )
        .await;

        assert_eq!(
            violated_constraint(result).as_deref(),
            Some("dns_resolution_subject_is_domain_name"),
            "{kind} {value}"
        );
    }
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_subject_at_both_length_bounds_in_the_table(pool: PgPool) {
    let longest = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(longest.len(), 253);
    for value in ["a.b", longest.as_str()] {
        let result = insert_link(
            &pool,
            Uuid::new_v4(),
            ("domain_name", value),
            ("ipv4_address", "192.0.2.1"),
        )
        .await;

        assert!(result.is_ok(), "{value}: {result:?}");
    }
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_an_answer_that_does_not_match_its_kind_in_the_table(pool: PgPool) {
    for (kind, value) in [
        ("ipv4_address", "2001:db8::1"),
        ("ipv4_address", "::ffff:192.0.2.1"),
        ("ipv6_address", "192.0.2.1"),
        ("ipv4_address", "192.0.2.0/24"),
        ("ipv6_address", "2001:db8::/64"),
        ("mac_address", "192.0.2.1"),
    ] {
        let result = insert_link(
            &pool,
            Uuid::new_v4(),
            ("domain_name", "example.com"),
            (kind, value),
        )
        .await;

        assert_eq!(
            violated_constraint(result).as_deref(),
            Some("dns_resolution_answer_matches_kind"),
            "{kind} {value}"
        );
    }
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_the_same_endpoints_under_another_id_in_the_table(pool: PgPool) {
    let example = DnsResolutionMother::example();
    SqlxPostgresDnsResolutionRepository::new(pool.clone())
        .save(&example)
        .await
        .unwrap();

    let result = insert_link(
        &pool,
        Uuid::new_v4(),
        ("domain_name", "example.com"),
        ("ipv4_address", "93.184.216.34"),
    )
    .await;

    assert_eq!(
        violated_constraint(result).as_deref(),
        Some("dns_resolution_subject_answer_key")
    );
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_window_that_ends_before_it_starts_in_the_table(pool: PgPool) {
    let saved = DnsResolutionMother::random();
    SqlxPostgresDnsResolutionRepository::new(pool.clone())
        .save(&saved)
        .await
        .unwrap();
    let id = *saved.id().value();

    let reversed = insert_observation(
        &pool,
        id,
        ("2024-01-01T00:00:00.000001Z", "2024-01-01T00:00:00Z"),
        1,
    )
    .await;
    let instant = insert_observation(
        &pool,
        id,
        ("2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z"),
        1,
    )
    .await;

    assert_eq!(
        violated_constraint(reversed).as_deref(),
        Some("dns_resolution_observation_window_is_ordered")
    );
    assert!(instant.is_ok(), "{instant:?}");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_count_below_one_in_the_table(pool: PgPool) {
    let saved = DnsResolutionMother::random();
    SqlxPostgresDnsResolutionRepository::new(pool.clone())
        .save(&saved)
        .await
        .unwrap();
    let id = *saved.id().value();
    let window = ("2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z");

    for count in [0_i64, -1] {
        let result = insert_observation(&pool, id, window, count).await;

        assert_eq!(
            violated_constraint(result).as_deref(),
            Some("dns_resolution_observation_count_is_positive"),
            "{count}"
        );
    }
    let one = insert_observation(&pool, id, window, 1).await;
    assert!(one.is_ok(), "{one:?}");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_orders_subjects_by_bytes_whatever_the_database_collation(pool: PgPool) {
    let collation: Option<String> = sqlx::query_scalar(
        "SELECT collation_name::text FROM information_schema.columns
         WHERE table_schema = 'kernel' AND table_name = 'dns_resolution' AND column_name = 'subject_value'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(collation.as_deref(), Some("C"));
}
