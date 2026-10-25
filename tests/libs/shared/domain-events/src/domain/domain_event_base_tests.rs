use uuid::Uuid;

use shared_domain_events::domain::domain_event::DomainEventBase;

#[test]
fn it_generates_version_7_event_ids() {
    let base = DomainEventBase::new("aggregate-1");

    let event_id = Uuid::parse_str(&base.event_id).expect("event ids are UUIDs");

    assert_eq!(event_id.get_version_num(), 7);
}

#[test]
fn it_generates_event_ids_that_sort_by_creation_time() {
    let first = DomainEventBase::new("aggregate-1");
    std::thread::sleep(std::time::Duration::from_millis(2));
    let second = DomainEventBase::new("aggregate-1");

    assert!(first.event_id < second.event_id);
}

#[test]
fn it_keeps_the_aggregate_id() {
    let base = DomainEventBase::new("aggregate-1");

    assert_eq!(base.aggregate_id, "aggregate-1");
}
