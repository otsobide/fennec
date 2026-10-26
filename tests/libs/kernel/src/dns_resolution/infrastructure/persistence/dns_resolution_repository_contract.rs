//! Behaviour every [`DnsResolutionRepository`] adapter must show.
//!
//! Each check takes a fresh, empty repository, so the in-memory and the
//! PostgreSQL adapters run exactly the same contract.

use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::entities::dns_resolution_retraction::DnsResolutionRetraction;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).expect("a valid count")
}

/// `dns_resolution` as it stands once `source_id` reports it in `window`.
fn observed(
    dns_resolution: &DnsResolution,
    source_id: DnsResolutionSourceId,
    window: DnsResolutionWindow,
    count: Option<DnsResolutionCount>,
) -> DnsResolution {
    dns_resolution
        .observe(source_id, window, count, DnsResolutionUpdatedAt::now())
        .expect("the report must change the resolution")
        .0
}

/// A resolution of `subject` to `answer` that three sources observed, one
/// of them with a count.
fn observed_by_three_sources(
    subject: DnsResolutionSubject,
    answer: DnsResolutionAnswer,
) -> DnsResolution {
    let recorded = DnsResolutionMother::create(subject, answer);
    let twice = observed(
        &recorded,
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::random(),
        Some(count(42)),
    );
    observed(
        &twice,
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::random(),
        None,
    )
}

fn random_observed_by_three_sources() -> DnsResolution {
    observed_by_three_sources(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
    )
}

async fn save_all(repository: &dyn DnsResolutionRepository, resolutions: &[&DnsResolution]) {
    for dns_resolution in resolutions {
        repository.save(dns_resolution).await.unwrap();
    }
}

pub async fn it_finds_a_saved_resolution_with_every_observation(
    repository: &dyn DnsResolutionRepository,
) {
    let saved = random_observed_by_three_sources();
    assert_eq!(saved.observations().len(), 3);

    repository.save(&saved).await.unwrap();

    assert_eq!(
        repository.find_by_id(saved.id()).await.unwrap(),
        Some(saved)
    );
}

pub async fn it_rejects_recording_the_same_resolution_twice(
    repository: &dyn DnsResolutionRepository,
) {
    let first = DnsResolutionMother::example();
    repository.save(&first).await.unwrap();
    let again = DnsResolutionMother::create(first.subject().clone(), *first.answer());

    let result = repository.save(&again).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::AlreadyExists)
    ));
    assert_eq!(
        repository.find_by_id(first.id()).await.unwrap(),
        Some(first)
    );
}

pub async fn it_returns_none_for_an_unknown_id(repository: &dyn DnsResolutionRepository) {
    let found = repository
        .find_by_id(DnsResolutionMother::random().id())
        .await
        .unwrap();

    assert_eq!(found, None);
}

pub async fn it_adds_an_observation_on_update(repository: &dyn DnsResolutionRepository) {
    let recorded = DnsResolutionMother::random();
    repository.save(&recorded).await.unwrap();
    let added = observed(
        &recorded,
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::random(),
        Some(count(7)),
    );

    repository.update(&added).await.unwrap();

    let found = repository.find_by_id(recorded.id()).await.unwrap();
    assert_eq!(found.as_ref().map(|r| r.observations().len()), Some(2));
    assert_eq!(found, Some(added));
}

pub async fn it_changes_an_observation_on_update(repository: &dyn DnsResolutionRepository) {
    let source_id = DnsResolutionSourceIdMother::random();
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        DnsResolutionWindowMother::create(1_600_000_000, 1_600_086_400),
    );
    repository.save(&recorded).await.unwrap();
    let widened = observed(
        &recorded,
        source_id.clone(),
        DnsResolutionWindowMother::create(1_500_000_000, 1_700_000_000),
        Some(count(9)),
    );

    repository.update(&widened).await.unwrap();

    let found = repository.find_by_id(recorded.id()).await.unwrap().unwrap();
    let observation = found.observation(&source_id).unwrap();
    assert_eq!(
        *observation.window(),
        DnsResolutionWindowMother::create(1_500_000_000, 1_700_000_000)
    );
    assert_eq!(observation.count(), count(9));
    assert_eq!(found, widened);
}

pub async fn it_removes_an_observation_on_update(repository: &dyn DnsResolutionRepository) {
    let saved = random_observed_by_three_sources();
    repository.save(&saved).await.unwrap();
    let withdrawn = saved.observations()[1].source_id().clone();
    let DnsResolutionRetraction::Retracted(remaining) = saved.retract(&withdrawn) else {
        panic!("two sources must remain");
    };

    repository.update(&remaining).await.unwrap();

    let found = repository.find_by_id(saved.id()).await.unwrap().unwrap();
    assert_eq!(found.observation(&withdrawn), None);
    assert_eq!(found, remaining);
}

pub async fn it_adds_changes_and_removes_observations_in_one_update(
    repository: &dyn DnsResolutionRepository,
) {
    let saved = random_observed_by_three_sources();
    repository.save(&saved).await.unwrap();
    let changed = saved.observations()[0].source_id().clone();
    let withdrawn = saved.observations()[2].source_id().clone();
    let DnsResolutionRetraction::Retracted(retracted) = saved.retract(&withdrawn) else {
        panic!("two sources must remain");
    };
    let widened = observed(
        &retracted,
        changed,
        DnsResolutionWindowMother::create(1_000_000_000, 2_000_000_000),
        Some(count(1_000)),
    );
    let replaced = observed(
        &widened,
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::random(),
        None,
    );

    repository.update(&replaced).await.unwrap();

    assert_eq!(
        repository.find_by_id(saved.id()).await.unwrap(),
        Some(replaced)
    );
}

pub async fn it_returns_not_found_when_updating_an_unknown_resolution(
    repository: &dyn DnsResolutionRepository,
) {
    let unknown = random_observed_by_three_sources();

    let result = repository.update(&unknown).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
    assert_eq!(repository.find_by_id(unknown.id()).await.unwrap(), None);
}

pub async fn it_lists_the_resolutions_of_a_subject_by_answer(
    repository: &dyn DnsResolutionRepository,
) {
    let subject = DnsResolutionSubjectMother::create("example.com");
    let resolution =
        |answer: DnsResolutionAnswer| observed_by_three_sources(subject.clone(), answer);
    let ipv6_documentation = resolution(DnsResolutionAnswerMother::ipv6("2001:db8::1"));
    let ipv4_ten = resolution(DnsResolutionAnswerMother::ipv4("10.0.0.1"));
    let ipv6_loopback = resolution(DnsResolutionAnswerMother::ipv6("::1"));
    let ipv4_documentation = resolution(DnsResolutionAnswerMother::ipv4("192.0.2.1"));
    let ipv4_nine = resolution(DnsResolutionAnswerMother::ipv4("9.9.9.9"));
    let other_subject_same_answer = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("example.net"),
        DnsResolutionAnswerMother::ipv4("10.0.0.1"),
    );
    let subdomain = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("www.example.com"),
        DnsResolutionAnswerMother::ipv4("192.0.2.2"),
    );
    save_all(
        repository,
        &[
            &ipv6_documentation,
            &other_subject_same_answer,
            &ipv4_ten,
            &ipv6_loopback,
            &subdomain,
            &ipv4_documentation,
            &ipv4_nine,
        ],
    )
    .await;

    let listed = repository.list_by_subject(&subject, 100).await.unwrap();

    assert_eq!(
        listed,
        vec![
            ipv4_nine,
            ipv4_ten,
            ipv4_documentation,
            ipv6_loopback,
            ipv6_documentation
        ]
    );
}

pub async fn it_limits_the_resolutions_of_a_subject(repository: &dyn DnsResolutionRepository) {
    let subject = DnsResolutionSubjectMother::random();
    let third = DnsResolutionMother::create(
        subject.clone(),
        DnsResolutionAnswerMother::ipv6("2001:db8::53"),
    );
    let first = observed_by_three_sources(
        subject.clone(),
        DnsResolutionAnswerMother::ipv4("198.51.100.7"),
    );
    let second = DnsResolutionMother::create(
        subject.clone(),
        DnsResolutionAnswerMother::ipv4("203.0.113.9"),
    );
    save_all(repository, &[&third, &first, &second]).await;

    let limited = repository.list_by_subject(&subject, 2).await.unwrap();
    let none = repository.list_by_subject(&subject, 0).await.unwrap();

    assert_eq!(limited, vec![first, second]);
    assert_eq!(none, Vec::<DnsResolution>::new());
}

pub async fn it_lists_the_resolutions_to_an_answer_by_subject(
    repository: &dyn DnsResolutionRepository,
) {
    let answer = DnsResolutionAnswerMother::ipv4("192.0.2.1");
    let resolution = |subject: &str| {
        observed_by_three_sources(DnsResolutionSubjectMother::create(subject), answer)
    };
    let b = resolution("b.example.com");
    let a = resolution("a.example.com");
    let a_hyphen_b = resolution("a-b.example.com");
    let a_zero = resolution("a0.example.com");
    let ab = resolution("ab.example.com");
    let other_answer = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("a.example.com"),
        DnsResolutionAnswerMother::ipv4("192.0.2.2"),
    );
    let mapped = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("a.example.com"),
        DnsResolutionAnswerMother::ipv6("::ffff:192.0.2.1"),
    );
    save_all(
        repository,
        &[&b, &mapped, &a, &a_hyphen_b, &other_answer, &a_zero, &ab],
    )
    .await;

    let listed = repository.list_by_answer(&answer, 100).await.unwrap();
    let listed_mapped = repository
        .list_by_answer(mapped.answer(), 100)
        .await
        .unwrap();

    assert_eq!(listed, vec![a_hyphen_b, a, a_zero, ab, b]);
    assert_eq!(listed_mapped, vec![mapped]);
}

pub async fn it_limits_the_resolutions_to_an_answer(repository: &dyn DnsResolutionRepository) {
    let answer = DnsResolutionAnswerMother::random();
    let second = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("mail.example.org"),
        answer,
    );
    let third = DnsResolutionMother::create(
        DnsResolutionSubjectMother::create("www.example.org"),
        answer,
    );
    let first =
        observed_by_three_sources(DnsResolutionSubjectMother::create("example.org"), answer);
    save_all(repository, &[&second, &third, &first]).await;

    let limited = repository.list_by_answer(&answer, 2).await.unwrap();
    let none = repository.list_by_answer(&answer, 0).await.unwrap();

    assert_eq!(limited, vec![first, second]);
    assert_eq!(none, Vec::<DnsResolution>::new());
}

pub async fn it_lists_nothing_for_an_unknown_endpoint(repository: &dyn DnsResolutionRepository) {
    repository
        .save(&random_observed_by_three_sources())
        .await
        .unwrap();

    let by_subject = repository
        .list_by_subject(&DnsResolutionSubjectMother::random(), 100)
        .await
        .unwrap();
    let by_answer = repository
        .list_by_answer(&DnsResolutionAnswerMother::ipv4("192.0.2.255"), 100)
        .await
        .unwrap();

    assert_eq!(by_subject, Vec::<DnsResolution>::new());
    assert_eq!(by_answer, Vec::<DnsResolution>::new());
}

pub async fn it_deletes_a_saved_resolution(repository: &dyn DnsResolutionRepository) {
    let saved = random_observed_by_three_sources();
    let kept = random_observed_by_three_sources();
    save_all(repository, &[&saved, &kept]).await;

    repository.delete(saved.id()).await.unwrap();

    assert_eq!(repository.find_by_id(saved.id()).await.unwrap(), None);
    assert_eq!(
        repository
            .list_by_subject(saved.subject(), 100)
            .await
            .unwrap(),
        Vec::<DnsResolution>::new()
    );
    assert_eq!(repository.find_by_id(kept.id()).await.unwrap(), Some(kept));
}

pub async fn it_accepts_a_deleted_resolution_again(repository: &dyn DnsResolutionRepository) {
    let deleted = random_observed_by_three_sources();
    repository.save(&deleted).await.unwrap();
    repository.delete(deleted.id()).await.unwrap();
    let recorded_again = DnsResolutionMother::create(deleted.subject().clone(), *deleted.answer());

    repository.save(&recorded_again).await.unwrap();

    assert_eq!(
        repository.find_by_id(deleted.id()).await.unwrap(),
        Some(recorded_again)
    );
}

pub async fn it_returns_not_found_when_deleting_an_unknown_id(
    repository: &dyn DnsResolutionRepository,
) {
    let result = repository.delete(DnsResolutionMother::random().id()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
}
