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
   application.
3. **The log is complete.** Every state change has an event, and every event
   carries enough to apply it without a second query: `Created` carries the
   full state, `Updated` the old and the new values, `Deleted` the identity
   and what a subscriber needs to react.
4. **Consumers read in a gap-free order.** Consumers inside the kernel
   database checkpoint in the same transaction as their effect and apply each
   event exactly once; every other consumer is at least once and deduplicates
   on `event_id`.

## State plus log, not rehydration

Fennec keeps aggregate state in tables and records every change in the log.
The log is what other contexts and new read models are built from, so it
plays the role an event store plays in event sourcing. Aggregates are still
loaded from their tables, not replayed from their events, for two reasons:

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
| `event_id` | `DomainEvent::event_id()`. `DomainEventBase::new` generates it with `Uuid::now_v7()` (time-ordered), so the index appends at its right edge. |
| `event_name` | `fennec.<context>.<aggregate>.<past>`, the event's `EVENT_NAME`. |
| `aggregate_type` | The `<aggregate>` segment of the event name, which is the module kind (`ipv4_address`). Ids of different kinds can share a UUID (see [../data-model/identity.md](../data-model/identity.md#qualify-every-reference-by-module-kind)), so the type is part of every lookup. |
| `aggregate_id` | `DomainEvent::aggregate_id()`. |
| `payload` | `DomainEvent::to_primitives()` rendered as JSON. |

### Partitions

The table is range-partitioned by month and has no default partition, so a
missing partition fails the write loudly instead of hiding rows. The first
migration defines `event_store.ensure_partitions(months_ahead integer)`,
which creates every missing month from the current UTC month on with
`CREATE TABLE ... PARTITION OF`, serialised across instances with
`pg_advisory_xact_lock`. Creating a partition briefly locks the parent, so
it only runs where traffic is low: the migration calls it for twelve months
ahead, and the startup maintenance step calls it again, so a running system
always has a year of partitions ready and the live path never creates one.
The schema and the function live in
`libs/shared/persistence-postgres/migrations/`.

## Events as data

The domain must not depend on a serialisation crate, so events expose their
payload as plain data defined in `shared-domain-events`:

```rust
pub enum PrimitiveValue {
    Null,
    Bool(bool),
    Integer(i64),
    UnsignedInteger(u64),
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

`to_primitives` has no default implementation: every event, in every
context, states its payload. Payload keys follow the event structs:

- `Created`: one key per aggregate field (`id`, `value`, `created_at`).
- `Updated`: `id` plus `old_<field>` and `new_<field>` for each changed field,
  plus any fact the event records (`observed_at` on a sighting observation).
- `Deleted`: `id` plus what a subscriber needs to react (the canonical value
  of a value-identified observable).

Values are the canonical primitives of the value objects. The PostgreSQL
adapter renders the map as `jsonb`, with timestamps as RFC 3339 UTC strings
with microseconds, whatever representation the HTTP API uses for them.

## Committing state and events together

Services keep their shape (persist, build the event with its factory,
publish). The atomicity comes from infrastructure in
`shared-persistence-postgres` that wraps them:

1. `TransactionalCommandBus` decorates the command bus. For each dispatch it
   begins a transaction and stores a `PgUnitOfWork` in a Tokio task-local.
2. Every repository call made while a unit of work is in scope, reads
   included, runs on its transaction, so a command uses exactly one
   connection and reads its own writes. A repository that hits a driver error
   marks the unit of work as failed. Calls made outside a unit of work (query
   handlers) use the pool.
3. `OutboxEventBus::publish` (synchronous, like every `EventBus`) only
   appends the events to the unit of work's buffer. Called outside a unit of
   work it returns an error, so a missing decorator fails loudly.
4. When the handler returns, the decorator rolls back if the dispatch failed,
   the unit of work was marked failed, or no event was published, and returns
   the handler's response unchanged. Otherwise it appends the buffered events
   with one `INSERT ... SELECT FROM UNNEST`, commits, hands the events to the
   in-process `InMemoryEventBus` (so synchronous subscribers still run, after
   the commit and best effort), and returns the handler's response.
5. If the append or the commit fails, the decorator discards the handler's
   response, which already claims success, and returns
   `CommandBusError::HandlerError`. Controllers already turn bus errors into
   `500`. This is the one deliberate exception to "bus errors are reserved for
   wiring bugs": an infrastructure failure of the unit of work, after the
   handler has run, cannot be expressed in the handler's envelope.

"Commit only when events were published" relies on an existing Fennec rule:
every successful write publishes at least one event, and a failed one
publishes none. Business errors travel inside the response envelope, which
the decorator cannot inspect, so this rule is what tells it the command
succeeded.

In-memory mode (`build_state()`) keeps today's wiring: in-memory
repositories, the `InMemoryEventBus`, no decorator and no log. There is no
mixed mode (see [persistence.md](persistence.md#non-negotiables)).

### One command at a time per aggregate

Before its first statement on an aggregate, a repository call inside a unit
of work takes a transaction-scoped advisory lock on that aggregate:

```sql
SELECT pg_advisory_xact_lock($1);  -- $1: first 8 bytes of SHA-256("fennec:<aggregate_type>:<uuid>") as i64
```

The lock is re-entrant within the transaction and released at commit or
rollback. It gives two guarantees:

- **No lost updates.** A second command on the same aggregate waits until
  the first has committed, then reads the committed state. A read-modify-write
  (observing a sighting, renaming an autonomous system) needs no version
  check and no `SELECT ... FOR UPDATE`.
- **Per-aggregate event order.** PostgreSQL assigns a transaction id at a
  transaction's first write. An advisory lock is not a write, so the second
  command cannot obtain its transaction id until the first has committed.
  Commands on one aggregate therefore commit in transaction id order, and a
  consumer reading by `(transaction_id, position)` sees their events in
  order. Without the lock this would not hold: a row lock or an `UPDATE`
  assigns the id before waiting, so the command that commits second could
  hold the smaller id.

The `cti-api-pg-e2e` suite verifies both guarantees with concurrent commands
on one aggregate.

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
before it. The cost is latency, not loss: any transaction that holds a
transaction id (any write, row lock or `SELECT ... FOR UPDATE`) delays every
consumer until it ends. Write transactions therefore stay short and carry no
network calls, and so do consumer transactions.

## Consumers

```sql
CREATE TABLE event_store.consumer_checkpoint (
    consumer_name       text COLLATE "C" PRIMARY KEY,
    last_transaction_id xid8        NOT NULL DEFAULT '0',
    last_position       bigint      NOT NULL DEFAULT 0,
    updated_at          timestamptz NOT NULL DEFAULT now()
);
```

- Only consumers that run inside the kernel database (kernel-owned
  projections and relays) write `event_store.consumer_checkpoint`. A
  projection applies a batch and moves its checkpoint in one short
  transaction, so each event takes effect exactly once.
- A relay that feeds an external system (a webhook, a broker, another
  context's read model in its own database) publishes outside any
  transaction, then moves its checkpoint in a short transaction. Delivery is
  at least once; the receiver deduplicates on `event_id`.
- Other bounded contexts that poll the log keep their checkpoint in their
  own store and never write kernel's schemas.
- One instance runs per consumer, elected with a session-level
  `pg_try_advisory_lock` on a dedicated connection. That lock assigns no
  transaction id, so holding it does not delay other consumers.
- Consumers poll with adaptive back-off (short when the last batch was full,
  up to half a second when it was empty). `LISTEN` and `NOTIFY` are at most a
  wake-up hint, never the source of truth.

## Retention

There is no retention yet: every partition is kept. Dropping a month would
make the log incomplete for any read model built later, and read models may
not bootstrap from the aggregate tables, which belong to the kernel context.

Retention becomes possible once the kernel offers a snapshot use case that
re-emits the current state of every aggregate as
`fennec.kernel.<kind>.snapshotted` events. A new read model then starts from
the latest complete snapshot instead of the first event, and partitions older
than both that snapshot and every consumer checkpoint can be archived and
dropped.
