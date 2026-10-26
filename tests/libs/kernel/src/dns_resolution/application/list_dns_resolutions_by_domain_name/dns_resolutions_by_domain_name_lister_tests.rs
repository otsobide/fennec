use std::net::Ipv4Addr;
use std::sync::Arc;

use kernel::dns_resolution::application::list_dns_resolutions_by_domain_name::dns_resolutions_by_domain_name_lister::{
    DnsResolutionsByDomainNameLister, MAX_LISTED,
};
use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;

fn make_lister(repo: Arc<DnsResolutionRepositoryMock>) -> DnsResolutionsByDomainNameLister {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    DnsResolutionsByDomainNameLister::new(repo)
}

/// `count` resolutions of `subject`, each to a distinct IPv4 address, in
/// address order.
fn resolutions_of(subject: &DnsResolutionSubject, count: usize) -> Vec<DnsResolution> {
    (0..count)
        .map(|index| {
            let address = Ipv4Addr::from(0x0a00_0000 + index as u32);
            DnsResolutionMother::create(
                subject.clone(),
                DnsResolutionAnswerMother::ipv4(&address.to_string()),
            )
        })
        .collect()
}

#[tokio::test]
async fn it_returns_the_resolutions_of_the_domain_name() {
    let subject = DnsResolutionSubjectMother::random();
    let first = DnsResolutionMother::create(subject.clone(), DnsResolutionAnswerMother::random());
    let second = DnsResolutionMother::create(subject.clone(), DnsResolutionAnswerMother::random());
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![
        first.clone(),
        DnsResolutionMother::random(),
        second.clone(),
    ]));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(subject).await.unwrap();

    assert_eq!(listed, vec![first, second]);
    assert!(!truncated);
}

#[tokio::test]
async fn it_asks_the_repository_for_one_more_than_max_listed_resolutions() {
    let subject = DnsResolutionSubjectMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_succeeds());
    let lister = make_lister(repo.clone());

    lister.execute(subject.clone()).await.unwrap();

    assert_eq!(MAX_LISTED, 1000);
    assert_eq!(repo.listed_by_subject(), vec![(subject, MAX_LISTED + 1)]);
}

#[tokio::test]
async fn it_returns_exactly_max_listed_resolutions_untruncated() {
    let subject = DnsResolutionSubjectMother::random();
    let recorded = resolutions_of(&subject, MAX_LISTED);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(recorded.clone()));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(subject).await.unwrap();

    assert_eq!(listed, recorded);
    assert!(!truncated);
}

#[tokio::test]
async fn it_truncates_to_max_listed_resolutions_when_one_more_is_recorded() {
    let subject = DnsResolutionSubjectMother::random();
    let recorded = resolutions_of(&subject, MAX_LISTED + 1);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(recorded.clone()));
    let lister = make_lister(repo);

    let (listed, truncated) = lister.execute(subject).await.unwrap();

    assert_eq!(listed.len(), MAX_LISTED);
    assert_eq!(listed, recorded[..MAX_LISTED]);
    assert!(truncated);
}

#[tokio::test]
async fn it_returns_an_empty_list_when_the_domain_name_has_no_resolutions() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_lists(vec![
        DnsResolutionMother::random(),
    ]));
    let lister = make_lister(repo);

    let (listed, truncated) = lister
        .execute(DnsResolutionSubjectMother::random())
        .await
        .unwrap();

    assert!(listed.is_empty());
    assert!(!truncated);
}

#[tokio::test]
async fn it_returns_an_error_on_storage_failure() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_list_by_subject(
        "storage error".to_string(),
    ));
    let lister = make_lister(repo);

    let result = lister.execute(DnsResolutionSubjectMother::random()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
