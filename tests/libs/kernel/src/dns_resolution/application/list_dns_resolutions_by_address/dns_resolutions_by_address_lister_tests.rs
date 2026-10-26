use std::sync::Arc;

use kernel::dns_resolution::application::list_dns_resolutions_by_address::dns_resolutions_by_address_lister::{
    DnsResolutionsByAddressLister, MAX_LISTED,
};
use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;

fn make_lister(repo: Arc<DnsResolutionRepositoryMock>) -> DnsResolutionsByAddressLister {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    DnsResolutionsByAddressLister::new(repo)
}

/// `count` resolutions to `answer`, each of a distinct domain name, in name
/// order.
fn resolutions_to(answer: &DnsResolutionAnswer, count: usize) -> Vec<DnsResolution> {
    (0..count)
        .map(|index| {
            DnsResolutionMother::create(
                DnsResolutionSubjectMother::create(&format!("d{index:04}.example")),
                *answer,
            )
        })
        .collect()
}

#[tokio::test]
async fn it_returns_the_resolutions_to_the_address() {
    let answer = DnsResolutionAnswerMother::ipv6("2001:db8::1");
    let first = DnsResolutionMother::create(DnsResolutionSubjectMother::random(), answer);
    let second = DnsResolutionMother::create(DnsResolutionSubjectMother::random(), answer);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![
        first.clone(),
        DnsResolutionMother::create(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::ipv6("2001:db8::2"),
        ),
        second.clone(),
    ]));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(answer).await.unwrap();

    assert_eq!(listed, vec![first, second]);
    assert!(!truncated);
}

#[tokio::test]
async fn it_asks_the_repository_for_one_more_than_max_listed_resolutions() {
    let answer = DnsResolutionAnswerMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_succeeds());
    let lister = make_lister(repo.clone());

    lister.execute(answer).await.unwrap();

    assert_eq!(MAX_LISTED, 1000);
    assert_eq!(repo.listed_by_answer(), vec![(answer, MAX_LISTED + 1)]);
}

#[tokio::test]
async fn it_returns_exactly_max_listed_resolutions_untruncated() {
    let answer = DnsResolutionAnswerMother::random();
    let recorded = resolutions_to(&answer, MAX_LISTED);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(recorded.clone()));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(answer).await.unwrap();

    assert_eq!(listed, recorded);
    assert!(!truncated);
}

#[tokio::test]
async fn it_truncates_to_max_listed_resolutions_when_one_more_is_recorded() {
    let answer = DnsResolutionAnswerMother::random();
    let recorded = resolutions_to(&answer, MAX_LISTED + 1);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(recorded.clone()));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(answer).await.unwrap();

    assert_eq!(listed.len(), MAX_LISTED);
    assert_eq!(listed, recorded[..MAX_LISTED]);
    assert!(truncated);
}

#[tokio::test]
async fn it_returns_an_empty_list_when_nothing_resolved_to_the_address() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![
        DnsResolutionMother::create(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::ipv4("192.0.2.1"),
        ),
    ]));
    let lister = make_lister(repo);

    let (listed, truncated) = lister
        .execute(DnsResolutionAnswerMother::ipv4("192.0.2.2"))
        .await
        .unwrap();

    assert!(listed.is_empty());
    assert!(!truncated);
}

#[tokio::test]
async fn it_returns_an_error_on_storage_failure() {
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_lists_but_list_by_answer_fails(
            Vec::new(),
            "storage error".to_string(),
        ),
    );
    let lister = make_lister(repo);

    let result = lister.execute(DnsResolutionAnswerMother::random()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
