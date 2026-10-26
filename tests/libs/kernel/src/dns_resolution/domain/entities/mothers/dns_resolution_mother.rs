use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

pub struct DnsResolutionMother;

#[allow(dead_code)]
impl DnsResolutionMother {
    /// A resolution of `subject` to `answer` reported by one random source in
    /// a random window.
    pub fn create(subject: DnsResolutionSubject, answer: DnsResolutionAnswer) -> DnsResolution {
        Self::reported_by(
            subject,
            answer,
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
        )
    }

    /// A resolution of `subject` to `answer` reported by `source_id` in
    /// `window`, with no count.
    pub fn reported_by(
        subject: DnsResolutionSubject,
        answer: DnsResolutionAnswer,
        source_id: DnsResolutionSourceId,
        window: DnsResolutionWindow,
    ) -> DnsResolution {
        DnsResolution::record(
            subject,
            answer,
            source_id,
            window,
            None,
            DnsResolutionCreatedAt::now(),
        )
    }

    /// A resolution of `example.com` to `93.184.216.34`, whose id is the
    /// documented vector `2da44331-2011-5539-b65e-9ff44db9c139`.
    pub fn example() -> DnsResolution {
        Self::create(
            DnsResolutionSubjectMother::create("example.com"),
            DnsResolutionAnswerMother::ipv4("93.184.216.34"),
        )
    }

    pub fn random() -> DnsResolution {
        Self::create(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
        )
    }
}
