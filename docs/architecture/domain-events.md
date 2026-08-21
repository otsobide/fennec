# Domain Events

Domain events record facts that happened inside the domain. They decouple the code that causes a state change from the code that reacts to it.

## Why domain events?

Consider what happens when you save a new source: the system might also need to write an audit log, invalidate a cache, replicate to another store, or notify another bounded context (e.g. an IoC ingester that needs to know about a new feed). Without events, `SourceCreator` would have to import and call each of those services directly. That means the creator — which belongs to the `kernel` bounded context — would suddenly depend on every module that cares about new sources. Two unrelated concerns become tightly coupled, and the problem compounds every time you add another side effect.

Domain events break that coupling. After saving the new source, the creator simply publishes a `SourceCreatedEvent`. It does not know or care who is listening. Separately, any number of subscribers can listen for that specific event and perform their own work. Neither side imports the other. You can add, remove, or modify subscribers without touching the code that publishes events, and the publisher can be tested in complete isolation from any subscriber.

The practical benefit is extensibility. Need to send a notification when a source is created? Add a subscriber. Need to log every deletion? Add a subscriber. Need to synchronise data to an external system? Add a subscriber. The publishing code never changes, and existing subscribers are unaffected by new ones. This keeps each piece of the system small, focused, and independently testable.

## Core concepts

| Concept | Role |
|---|---|
| `DomainEvent` | Trait that all events implement |
| `DomainEventBase` | Common metadata: `aggregate_id`, `event_id`, `occurred_on` |
| `EventBus` | Delivers a batch of events to all registered subscribers |
| `DomainEventSubscriber<E>` | Reacts to one specific event type |

## DomainEvent trait

```rust
// libs/shared/domain-events/src/domain/domain_event.rs
pub trait DomainEvent: AnyDomainEvent {
    fn event_name(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
    fn event_id(&self) -> &str;
    fn occurred_on(&self) -> SystemTime;
}

pub struct DomainEventBase {
    pub aggregate_id: String,
    pub event_id:     String,    // UUID v4
    pub occurred_on:  SystemTime,
}
```

Every concrete event also declares an `EVENT_NAME` constant used by subscribers and tests:

```rust
impl SourceCreatedEvent {
    pub const EVENT_NAME: &'static str = "fennec.kernel.source.created";
}
```

The naming scheme is `<product>.<bounded_context>.<aggregate>.<past_tense>`. This keeps event names self-describing across the whole product.

## EventBus trait

```rust
pub trait EventBus: Send + Sync {
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError>;
}
```

## DomainEventSubscriber trait

```rust
#[async_trait]
pub trait DomainEventSubscriber<E: DomainEvent>: Send + Sync {
    async fn on(&self, event: &E) -> Result<(), Box<dyn Error + Send + Sync>>;
}
```

## In-memory implementation

The current implementation is synchronous and in-process — when an event is published, all subscribers run before the `publish` call returns. This is simple and sufficient for a single-process application. If the system grows to need asynchronous processing or cross-service events, the `EventBus` trait can be implemented with a message broker (RabbitMQ, Kafka, NATS) without changing any domain code, because the domain only depends on the trait, never on the concrete implementation.

`InMemoryEventBus` holds a `HashMap<TypeId, Vec<HandlerFn>>`. When `publish` is called it iterates the events, looks up each `TypeId`, and calls every registered handler for that type.

```
EventBus::publish([SourceCreatedEvent, ...])
  │
  ├─▶ TypeId::of::<SourceCreatedEvent>()
  │     ├─▶ subscriber_1.on(&event)
  │     └─▶ subscriber_2.on(&event)
  │
  └─▶ (other event types)
```

## Event factory functions

Events are created through dedicated factory functions rather than calling constructors directly. This keeps event construction logic in one place, isolates it from the application service, and makes it easy to ensure all required fields are populated from the aggregate. If the event structure changes, only the factory function needs updating — the application service that publishes the event remains untouched.

```rust
// libs/kernel/src/source/domain/events/create_source_created_event.rs
pub fn create_source_created_event(
    source: &Source,
) -> Result<SourceCreatedEvent, SourceRepositoryError> {
    Ok(SourceCreatedEvent::new(
        source.id().clone(),
        source.source_type().clone(),
        source.status().clone(),
        source.description().clone(),
        source.created_at().clone(),
        source.updated_at().clone(),
    ))
}
```

## Lifecycle in a use case

The order of operations matters: events are always published **after** the repository write succeeds. This ensures that subscribers never react to changes that were not actually persisted. If the save fails, no event is published and no side effects happen. This "persist first, publish second" rule is a simple but important guarantee that keeps the system consistent.

```rust
// Inside SourceCreator::execute()
self.repository.save(&source).await?;

let event = create_source_created_event(&source)?;
self.event_bus
    .publish(vec![Box::new(event)])
    .map_err(|e| SourceRepositoryError::Unexpected(e.to_string()))?;
```

The event is published **after** the repository operation succeeds. If the save fails, no event is published — ensuring the bus only receives facts that actually happened.

## Events currently in the system

| Event | Published by | Carries |
|---|---|---|
| `SourceCreatedEvent` | `SourceCreator` | full snapshot of the new source (id, type, status, description, timestamps) |
| `SourceUpdatedEvent` | `SourceUpdater` | id + old/new status + old/new description + new `updated_at` |
| `SourceDeletedEvent` | `SourceDeleter` | id of the deleted source |
| `ConfigEntryCreatedEvent` / `ConfigEntryUpdatedEvent` / `ConfigEntryDeletedEvent` | reference context | analogous fields for `ConfigEntry` |

Fennec currently ships **no subscribers** — the event bus infrastructure is wired and ready but the subscriber list is empty. Subscribers will be added as the platform grows (cache invalidation, audit logging, cross-context reactions such as IoC ingesters reacting to new sources).

## Adding a new event

1. Create the event struct in `libs/<context>/src/<context>/domain/events/<noun>_<past>_event.rs`.
2. Implement `DomainEvent` and declare `EVENT_NAME` following `fennec.<context>.<aggregate>.<past>`.
3. Create a factory function `create_<noun>_<past>_event.rs`.
4. Expose both from the domain's `events/mod.rs`.
5. Publish it from the relevant application service after the repository write succeeds.

## Adding a subscriber

1. Create a struct that implements `DomainEventSubscriber<YourEvent>`.
2. Register it on the `EventBus` inside `build_state()`:

```rust
event_bus.subscribe::<SourceCreatedEvent, _>(
    Arc::new(MySubscriber::new(...))
);
```

The subscriber lives in the _consuming_ context, not the _producing_ one. A subscriber that reacts to `SourceCreatedEvent` and writes an audit log lives in the audit context, not in `kernel`.
