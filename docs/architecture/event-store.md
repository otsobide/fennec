# Event store

How domain events become durable. Every command that changes an aggregate
appends its events to an append-only log in the same PostgreSQL transaction
as the state change. Other bounded contexts, projections and integrations
consume that log; they never read the aggregate tables. The design is argued
in [../research/postgres-scalability.md](../research/postgres-scalability.md)
sections 7 and 8.

## Non-negotiables

1. **State and events commit together.** A command either commits its state
   change and all its events, or neither. There is no window in which an
   observable exists without its `Created` event, or an event describes a
   write that was rolled back.
2. **The log is append-only.** Rows are never updated or deleted by the
   application. Retention drops whole monthly partitions.
3. **The log is complete.** Every state change has an event, and every event
   carries enough to apply it without a second query: `Created` carries the
   full state, `Updated` the old and the new values, `Deleted` the identity
   and what a subscriber needs to react.
4. **Consumers read in a gap-free order** and checkpoint in the same
   transaction as the effect they apply.

## State plus log, not rehydration

Fennec keeps aggregate state in tables and records every change in the log.
The log alone is enough to rebuild every table, and it is what new read
models are built from, so it plays the role an event store plays in event
sourcing. Aggregates are still loaded from their tables, not replayed from
their events, for two reasons:

- Invariants such as "one aggregate per canonical value" need indexed lookups
  by natural key, which a stream of events does not give.
- Observables are append-mostly: most have exactly one event in their life.
  Replaying a stream of length one buys nothing over reading a row.

## The table

```sql
CREATE SCHEMA event_store;

CREATE TABLE event_store.domain_event (
    position       bigint      GENERATED ALWAYS AS IDENTITY,
    transaction_id xid8        NOT NULL DEFAULT pg_current_xact_id(),
    recorded_at    timestamptz NOT NULL DEFAULT now(),
    occurred_on    timestamptz NOT NULL,
    event_id       uuid        NOT NULL,
    event_name     text COLLATE "C" NOT NULL,
    aggregate_type text COLLATE "C" NOT NULL,
    aggregate_id   uuid        NOT NULL,
    payload        jsonb       NOT NULL,
    CONSTRAINT domain_event_pkey PRIMARY KEY (position, recorded_at),
    CONSTRAINT domain_event_event_name_shape
        CHECK (event_name ~ '^fennec\.[a-z0-9_]+\.[a-z0-9_]+\.[a-z_]+$')
) PARTITION BY RANGE (recorded_at);

CREATE INDEX domain_event_transaction_position_idx
    ON event_store.domain_event (transaction_id, position);
CREATE INDEX domain_event_aggregate_idx
    ON event_store.domain_event (aggregate_type, aggregate_id, position);
CREATE INDEX domain_event_event_id_idx
    ON event_store.domain_event (event_id);
```

| Column | Meaning |
|---|---|
| `position` | Insertion order. Unique, but not commit order: see [Ordering](#ordering). |
| `transaction_id` | The writing transaction, used for gap-free reads. |
| `recorded_at` | Transaction start time; the partition key. All events of one command share it. |
| `occurred_on` | `DomainEvent::occurred_on()`. |
| `event_id` | `DomainEvent::event_id()`, a UUID v7 so the index appends at its right edge. |
| `event_name` | `fennec.<context>.<aggregate>.<past>`, the event's `EVENT_NAME`. |
| `aggregate_type` | The `<aggregate>` segment of the event name (`ipv4_address`). Ids of different kinds can share a UUID (see [../data-model/identity.md](../data-model/identity.md#qualify-every-reference-by-kind)), so the type is part of every lookup. |
| `aggregate_id` | `DomainEvent::aggregate_id()`. |
| `payload` | `DomainEvent::to_primitives()` rendered as JSON. |

The table is range-partitioned by month. The adapter creates the partitions
for the current month and the next two at startup and re-checks them every
day; there is no default partition, so a missing partition fails the write
loudly instead of hiding rows.

## Events as data

The domain must not depend on a serialisation crate, so events expose their
payload as plain data defined in `shared-domain-events`:

```rust
pub enum PrimitiveValue {
    Null,
    Bool(bool),
    Integer(i64),
    Text(String),
    Timestamp(SystemTime),
    List(Vec<PrimitiveValue>),
    Map(BTreeMap<String, PrimitiveValue>),
}

pub trait DomainEvent: AnyDomainEvent {
    fn event_name(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
    fn event_id(&self) -> &str;
    fn occurred_on(&self) -> SystemTime;
    fn to_primitives(&self) -> BTreeMap<String, PrimitiveValue>;
}
```

The PostgreSQL adapter turns the map into `jsonb` (timestamps as RFC 3339
UTC strings with microseconds). Payload keys are the snake case names of the
aggregate's fields, and values are the canonical primitives of its value
objects, the same ones the API returns.

## Committing state and events together

Services keep their shape (persist, build the event with its factory,
publish). The atomicity comes from infrastructure that wraps them:

1. `TransactionalCommandBus` decorates the command bus. For each dispatch it
   begins a transaction and stores a `PgUnitOfWork` in a Tokio task-local.
2. PostgreSQL repositories run their writes on the unit of work's
   transaction. A repository that hits a driver error marks the unit of work
   as failed.
3. `OutboxEventBus::publish` (synchronous, like every `EventBus`) only
   appends the events to the unit of work's buffer. Called outside a unit of
   work it returns an error, so a missing decorator fails loudly.
4. When the handler returns, the decorator rolls back if the dispatch failed,
   the unit of work was marked failed, or no event was published. Otherwise
   it appends the buffered events with one `INSERT ... SELECT FROM UNNEST`,
   commits, and then hands the events to the in-process `InMemoryEventBus`
   so synchronous subscribers still run, after the commit and best effort.

"Commit only when events were published" relies on an existing Fennec rule:
every successful write publishes at least one event, and a failed one
publishes none. Business errors travel inside the response envelope, which
the decorator cannot inspect, so this rule is what tells it the command
succeeded.

Queries are not wrapped: query handlers read through the pool directly.

In-memory mode (no `DATABASE_URL`) keeps today's wiring: in-memory
repositories, the `InMemoryEventBus`, no decorator and no log.

## Ordering

Positions come from an identity column at insert time, not at commit time,
so a transaction that inserts position 10 can commit after one that inserted
position 11. A reader that only follows positions would skip 10 for ever.
Consumers therefore read by `(transaction_id, position)` and only see
transactions older than every transaction still running:

```sql
SELECT position, transaction_id::text, event_id, event_name, aggregate_type,
       aggregate_id, payload, occurred_on, recorded_at
FROM event_store.domain_event
WHERE (transaction_id, position) > ($1::text::xid8, $2)
  AND transaction_id < pg_snapshot_xmin(pg_current_snapshot())
ORDER BY transaction_id, position
LIMIT $3;
```

Every transaction id below the snapshot's `xmin` has finished and new ones
are always larger, so once a consumer has passed a pair nothing can appear
before it. The cost is latency, not loss: a long-running write transaction
anywhere in the cluster delays every consumer until it ends, which is why
write transactions stay short and carry no network calls.

Per aggregate, events also arrive in order: commands touch one aggregate and
their first write is to that aggregate's row, so a later command on the same
aggregate cannot obtain a smaller transaction id. When multi-aggregate
commands or mutable aggregates arrive, events gain an `aggregate_version`
and consumers skip versions they have already applied.

## Consumers

```sql
CREATE TABLE event_store.consumer_checkpoint (
    consumer_name       text COLLATE "C" PRIMARY KEY,
    last_transaction_id xid8        NOT NULL DEFAULT '0',
    last_position       bigint      NOT NULL DEFAULT 0,
    updated_at          timestamptz NOT NULL DEFAULT now()
);
```

- A consumer in the same database (a projection) applies a batch and moves
  its checkpoint in one transaction, so each event takes effect exactly once.
- A consumer that feeds an external system (a webhook, a broker) publishes
  at least once; the receiver deduplicates on `event_id`.
- One instance runs per consumer: it locks its checkpoint row with
  `FOR UPDATE NOWAIT` inside the processing transaction, so a second instance
  fails fast.
- Consumers poll with adaptive back-off (short when the last batch was full,
  up to half a second when it was empty). `LISTEN` and `NOTIFY` are at most a
  wake-up hint, never the source of truth.

## Retention

The aggregate tables hold the current state, so the log can be trimmed. A
monthly partition is archived and dropped once every consumer checkpoint is
past it and the audit retention window has passed. A new read model
bootstraps from the aggregate tables plus the retained tail of the log, not
from the first event ever written.
