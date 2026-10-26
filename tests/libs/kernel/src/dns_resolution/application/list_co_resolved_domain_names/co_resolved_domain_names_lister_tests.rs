use std::net::Ipv4Addr;
use std::sync::Arc;

use kernel::dns_resolution::application::list_co_resolved_domain_names::co_resolved_domain_names::CoResolvedDomainNames;
use kernel::dns_resolution::application::list_co_resolved_domain_names::co_resolved_domain_names_lister::{
    CoResolvedDomainNamesLister, ANSWER_LIMIT, PER_ANSWER_LIMIT,
};
use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;

const QUERIED: &str = "queried.example";

fn make_lister(repo: Arc<DnsResolutionRepositoryMock>) -> CoResolvedDomainNamesLister {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    CoResolvedDomainNamesLister::new(repo)
}

fn answer(address: &str) -> DnsResolutionAnswer {
    if address.contains(':') {
        DnsResolutionAnswerMother::ipv6(address)
    } else {
        DnsResolutionAnswerMother::ipv4(address)
    }
}

/// `name` resolved to `address` from `first_seen` to `last_seen` (unix
/// seconds), as one random source saw it.
fn link(name: &str, address: &str, first_seen: u64, last_seen: u64) -> DnsResolution {
    DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::create(name),
        answer(address),
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::create(first_seen, last_seen),
    )
}

/// `link` also seen by a second source from `first_seen` to `last_seen`.
fn also_seen(link: DnsResolution, first_seen: u64, last_seen: u64) -> DnsResolution {
    link.observe(
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::create(first_seen, last_seen),
        None,
        DnsResolutionUpdatedAt::now(),
    )
    .expect("a new source always changes the resolution")
    .0
}

fn nth_address(n: usize) -> String {
    Ipv4Addr::from(0x0a00_0000 + n as u32).to_string()
}

async fn pivot(dns_resolutions: Vec<DnsResolution>) -> CoResolvedDomainNames {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(dns_resolutions));
    make_lister(repo)
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await
        .unwrap()
}

fn names(result: &CoResolvedDomainNames) -> Vec<&str> {
    result
        .domain_names()
        .iter()
        .map(|co_resolved| co_resolved.domain_name().value())
        .collect()
}

fn shared_answers(result: &CoResolvedDomainNames, name: &str) -> Vec<String> {
    result
        .domain_names()
        .iter()
        .find(|co_resolved| co_resolved.domain_name().value() == name)
        .unwrap_or_else(|| panic!("{name} is not co-resolved"))
        .shared_answers()
        .iter()
        .map(ToString::to_string)
        .collect()
}

// Which names are co-resolved

#[tokio::test]
async fn it_lists_a_name_that_resolved_to_the_same_address_at_an_overlapping_time() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link("other.example", "192.0.2.1", 150, 250),
    ])
    .await;

    assert_eq!(result.subject().value(), QUERIED);
    assert_eq!(names(&result), vec!["other.example"]);
    assert_eq!(shared_answers(&result, "other.example"), vec!["192.0.2.1"]);
    assert!(!result.truncated());
}

#[tokio::test]
async fn it_ignores_a_name_that_resolved_to_the_same_address_at_another_time() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link("before.example", "192.0.2.1", 0, 99),
        link("after.example", "192.0.2.1", 201, 300),
    ])
    .await;

    assert!(result.domain_names().is_empty());
}

#[tokio::test]
async fn it_counts_windows_that_share_a_single_instant_as_overlapping() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link("ends-at-start.example", "192.0.2.1", 0, 100),
        link("starts-at-end.example", "192.0.2.1", 200, 300),
    ])
    .await;

    assert_eq!(
        names(&result),
        vec!["ends-at-start.example", "starts-at-end.example"]
    );
}

#[tokio::test]
async fn it_ignores_a_name_that_resolved_to_another_address() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link("other.example", "192.0.2.2", 100, 200),
    ])
    .await;

    assert!(result.domain_names().is_empty());
}

#[tokio::test]
async fn it_compares_the_windows_over_every_observation_of_both_resolutions() {
    // The queried name was seen in [100, 200] and [500, 600], so its window
    // is [100, 600]; spread.example was seen in [0, 10] and [300, 310], so
    // its window [0, 310] overlaps it although no single observation does.
    let result = pivot(vec![
        also_seen(link(QUERIED, "192.0.2.1", 100, 200), 500, 600),
        link("late.example", "192.0.2.1", 550, 560),
        also_seen(link("spread.example", "192.0.2.1", 0, 10), 300, 310),
        link("later.example", "192.0.2.1", 700, 800),
    ])
    .await;

    assert_eq!(names(&result), vec!["late.example", "spread.example"]);
}

#[tokio::test]
async fn it_never_lists_the_queried_name() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link(QUERIED, "192.0.2.2", 100, 200),
    ])
    .await;

    assert!(result.domain_names().is_empty());
    assert!(!result.truncated());
}

#[tokio::test]
async fn it_lists_a_name_once_with_every_answer_it_shared() {
    let result = pivot(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link(QUERIED, "192.0.2.2", 100, 200),
        link(QUERIED, "192.0.2.3", 100, 200),
        link("other.example", "192.0.2.1", 100, 200),
        link("other.example", "192.0.2.2", 300, 400),
        link("other.example", "192.0.2.3", 150, 160),
    ])
    .await;

    assert_eq!(names(&result), vec!["other.example"]);
    assert_eq!(
        shared_answers(&result, "other.example"),
        vec!["192.0.2.1", "192.0.2.3"]
    );
}

// Ordering

#[tokio::test]
async fn it_sorts_the_names_by_value_and_their_shared_answers_by_address() {
    let result = pivot(vec![
        link(QUERIED, "10.0.0.10", 100, 200),
        link(QUERIED, "2001:db8::1", 100, 200),
        link(QUERIED, "10.0.0.9", 100, 200),
        link("b.example", "10.0.0.10", 100, 200),
        link("c.example", "2001:db8::1", 100, 200),
        link("a.example", "2001:db8::1", 100, 200),
        link("a.example", "10.0.0.10", 100, 200),
        link("a.example", "10.0.0.9", 100, 200),
    ])
    .await;

    assert_eq!(names(&result), vec!["a.example", "b.example", "c.example"]);
    assert_eq!(
        shared_answers(&result, "a.example"),
        vec!["10.0.0.9", "10.0.0.10", "2001:db8::1"]
    );
}

// Limits and truncation

#[tokio::test]
async fn it_asks_for_one_more_than_each_limit() {
    let queried = DnsResolutionSubjectMother::create(QUERIED);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link(QUERIED, "192.0.2.2", 100, 200),
    ]));
    let lister = make_lister(repo.clone());

    lister.execute(queried.clone()).await.unwrap();

    assert_eq!(ANSWER_LIMIT, 100);
    assert_eq!(PER_ANSWER_LIMIT, 1000);
    assert_eq!(repo.listed_by_subject(), vec![(queried, ANSWER_LIMIT + 1)]);
    assert_eq!(
        repo.listed_by_answer(),
        vec![
            (answer("192.0.2.1"), PER_ANSWER_LIMIT + 1),
            (answer("192.0.2.2"), PER_ANSWER_LIMIT + 1)
        ]
    );
}

#[tokio::test]
async fn it_is_not_truncated_when_the_name_has_exactly_answer_limit_answers() {
    let links = (0..ANSWER_LIMIT)
        .map(|n| link(QUERIED, &nth_address(n), 100, 200))
        .collect::<Vec<_>>();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(links));
    let lister = make_lister(repo.clone());

    let result = lister
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await
        .unwrap();

    assert!(!result.truncated());
    assert_eq!(repo.listed_by_answer().len(), ANSWER_LIMIT);
}

#[tokio::test]
async fn it_follows_at_most_answer_limit_answers_and_says_it_truncated() {
    let mut dns_resolutions = (0..=ANSWER_LIMIT)
        .map(|n| link(QUERIED, &nth_address(n), 100, 200))
        .collect::<Vec<_>>();
    dns_resolutions.push(link("first.example", &nth_address(0), 100, 200));
    dns_resolutions.push(link(
        "dropped.example",
        &nth_address(ANSWER_LIMIT),
        100,
        200,
    ));
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(dns_resolutions));
    let lister = make_lister(repo.clone());

    let result = lister
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await
        .unwrap();

    assert!(result.truncated());
    assert_eq!(repo.listed_by_answer().len(), ANSWER_LIMIT);
    assert_eq!(names(&result), vec!["first.example"]);
}

#[tokio::test]
async fn it_is_not_truncated_when_an_answer_has_exactly_per_answer_limit_resolutions() {
    let mut dns_resolutions = vec![link(QUERIED, "192.0.2.1", 100, 200)];
    dns_resolutions.extend(
        (1..PER_ANSWER_LIMIT - 1).map(|n| link(&format!("n{n:04}.example"), "192.0.2.1", 300, 400)),
    );
    dns_resolutions.push(link("last.example", "192.0.2.1", 100, 200));

    let result = pivot(dns_resolutions).await;

    assert!(!result.truncated());
    assert_eq!(names(&result), vec!["last.example"]);
}

#[tokio::test]
async fn it_reads_at_most_per_answer_limit_resolutions_per_answer_and_says_it_truncated() {
    let mut dns_resolutions = vec![
        link(QUERIED, "192.0.2.1", 100, 200),
        link("first.example", "192.0.2.1", 100, 200),
    ];
    dns_resolutions.extend(
        (2..PER_ANSWER_LIMIT).map(|n| link(&format!("n{n:04}.example"), "192.0.2.1", 300, 400)),
    );
    dns_resolutions.push(link("dropped.example", "192.0.2.1", 100, 200));

    let result = pivot(dns_resolutions).await;

    assert!(result.truncated());
    assert_eq!(names(&result), vec!["first.example"]);
}

// Empty results and failures

#[tokio::test]
async fn it_returns_an_empty_result_when_the_name_has_no_resolutions() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![link(
        "other.example",
        "192.0.2.1",
        100,
        200,
    )]));
    let lister = make_lister(repo.clone());

    let result = lister
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await
        .unwrap();

    assert_eq!(result.subject().value(), QUERIED);
    assert!(result.domain_names().is_empty());
    assert!(!result.truncated());
    assert!(repo.listed_by_answer().is_empty());
}

#[tokio::test]
async fn it_returns_an_error_when_listing_the_resolutions_of_the_name_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_list_by_subject(
        "storage error".to_string(),
    ));
    let lister = make_lister(repo);

    let result = lister
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_an_error_when_listing_the_resolutions_to_an_answer_fails() {
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_lists_but_list_by_answer_fails(
            vec![link(QUERIED, "192.0.2.1", 100, 200)],
            "storage error".to_string(),
        ),
    );
    let lister = make_lister(repo);

    let result = lister
        .execute(DnsResolutionSubjectMother::create(QUERIED))
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
