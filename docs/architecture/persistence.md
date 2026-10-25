# Persistence

How the `kernel` bounded context stores its aggregates in PostgreSQL. The
domain event log that lives in the same database is described in
[event-store.md](event-store.md). The reasoning and the measurements behind
each rule are in
[../research/postgres-scalability.md](../research/postgres-scalability.md).

## Non-negotiables

1. **One bounded context, one database.** The `kernel` context owns one
   PostgreSQL database. Its aggregate tables live in the `kernel` schema and
   its event log in the `event_store` schema. No other context reads or
   writes them; other contexts build their own read models from the event
   log.
2. **One table per aggregate kind.** There is no shared observable table and
   no `type` column. A kind's table is named after its module
   (`kernel.ipv4_address`).
3. **Native column types.** Addresses are `inet` and `cidr`, MAC addresses
   `macaddr`, digests `bytea`, numbers the narrowest integer that holds them,
   timestamps `timestamptz`, case-folded text `text COLLATE "C"`. Never store
   hex digests or addresses as free text, and never use `citext`.
4. **The value object is the validator; the database is the backstop.** Rust
   parses, validates and canonicalises. `CHECK` constraints repeat the
   invariants that are cheap to state in SQL, so a canonicalisation bug
   surfaces as a constraint error instead of a silent duplicate. Raw user
   text never reaches a `::inet` or `::cidr` cast (PostgreSQL accepts classful
   shorthand such as `10` for `10.0.0.0/8`).
5. **A duplicate never aborts a transaction.** Creates use
   `INSERT ... ON CONFLICT DO NOTHING` and read `rows_affected()`: zero rows
   means `AlreadyExists`. A unique violation (SQLSTATE 23505) would abort the
   whole unit of work, so the write path never relies on catching one.
6. **Rows map back through value object constructors.** A row that fails
   validation is a bug surfaced as `Unexpected`, never data accepted.

## Platform

| Choice | Value |
|---|---|
| Server | PostgreSQL 18 (`docker-compose.yml`, CI service container) |
| Driver | `sqlx` 0.8 with the `runtime-tokio`, `postgres`, `uuid`, `time`, `ipnet`, `mac_address`, `migrate` and `macros` features |
| Queries | Runtime queries (`sqlx::query`, `query_as` with `FromRow`) only. The compile-time `query!` macros are not used, so builds never need a database or a `.sqlx` cache across the three CI toolchains |
| Migrations | Embedded with `sqlx::migrate!("./migrations")` from `libs/kernel/migrations/`, one numbered file per change (`0001_create_event_store.sql`, `0002_create_ipv4_address.sql`) |
| Extensions | None |
| Isolation | `READ COMMITTED` for every write transaction |

## Wiring

`cti_api` decides the persistence at startup:

- **`DATABASE_URL` set**: it connects a `PgPool`, runs the embedded
  migrations, wires the PostgreSQL repositories and wraps the command bus in
  the transactional decorator described in [event-store.md](event-store.md).
- **`DATABASE_URL` unset**: it wires the in-memory repositories and the
  in-memory event bus, exactly as before. Unit and e2e tests use this mode and
  need no database.

Running migrations from the application at startup is the first step.
Moving them to a separate deploy step that runs as a schema-owner role, with
the application connecting as a DML-only role, is planned hardening.

## Table conventions

A value-identified kind (identity regime A, see
[../data-model/identity.md](../data-model/identity.md)) gets this shape:

```sql
CREATE TABLE kernel.ipv4_address (
    id         uuid        PRIMARY KEY,
    value      inet        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv4_address_value_is_host CHECK (family(value) = 4 AND masklen(value) = 32),
    CONSTRAINT ipv4_address_value_key UNIQUE (value)
);
```

- `id` is the UUID part of the deterministic STIX identifier. The STIX type
  prefix is implied by the table, so it is not stored.
- The natural key keeps a `UNIQUE` constraint as a safety net while the table
  is unpartitioned. If a kind is ever hash-partitioned by `id`, that
  constraint is dropped and uniqueness rests on the deterministic id alone.
- Unbounded text keys (URLs, user agents) are unique on a SHA-256 digest of
  the canonical value, computed in Rust and stored as `bytea`, because a
  B-tree entry cannot exceed about a third of a page.
- Constraint names follow `<table>_<column>_<rule>` and
  `<table>_<column>_key`, so an error can be mapped by name.

Mutable aggregates add `updated_at timestamptz NOT NULL` and a
`version bigint NOT NULL` for optimistic concurrency: an update is
`UPDATE ... WHERE id = $1 AND version = $2`, and zero affected rows means
either `NotFound` or a concurrent change.

## Type mapping

| Domain value | Bound as | Column |
|---|---|---|
| Aggregate ids | `uuid::Uuid` | `uuid` |
| IPv4 or IPv6 host | `std::net::IpAddr` | `inet` with a full-width mask `CHECK` |
| IPv4 or IPv6 network | `ipnet::IpNet` | `cidr` |
| MAC address | `mac_address::MacAddress` | `macaddr` |
| Hex digests and fingerprints | `Vec<u8>` (decoded in Rust) | `bytea` with an `octet_length` `CHECK` |
| Unsigned 32-bit numbers (AS numbers, watermarks) | `i64` | `bigint` with a range `CHECK` |
| Timestamps (`SystemTime`) | `time::OffsetDateTime` | `timestamptz` |
| Canonical text | `&str` | `text COLLATE "C"` |

`SystemTime` carries nanoseconds and `timestamptz` microseconds. Timestamp
value objects truncate to microseconds when they are created, so an
aggregate read back from PostgreSQL compares equal to the one that was
saved, and the in-memory and PostgreSQL repositories behave identically.

## Reads

Query handlers read through the pool directly, outside any unit of work,
with one snapshot per statement. A lookup by value never scans: the handler
derives the deterministic id from the value and reads by primary key.

The write database serves only reads the write side needs (find by id or by
natural key). Search, listing by range, containment of addresses in networks
and similarity search on fuzzy digests belong to the read-model contexts that
will be built from the event log.

## Partitioning

Aggregate tables start unpartitioned. A kind is hash-partitioned by `id`
(`PARTITION BY HASH (id)`, modulus 32 or 64 chosen for the three to five year
size) when its table approaches the server's memory, which is the threshold
the PostgreSQL documentation gives. The kinds expected to get there first are
`url`, `domain_name`, `file` and `ipv4_address`. Because ids are derived from
values, every lookup still prunes to one partition.

The event log is range-partitioned by month from the first migration, see
[event-store.md](event-store.md).

## Testing

- Repository behaviour shared by every adapter (save, duplicate, find,
  delete, missing) is tested against the in-memory repository in the normal
  unit suite.
- The PostgreSQL repositories run the same contract in a separate test target,
  `kernel-postgres`, whose tests are marked `#[ignore = "requires DATABASE_URL"]`.
  CI runs it with `cargo test -p kernel --test kernel-postgres -- --ignored`
  against a PostgreSQL 18 service container. Locally:

  ```bash
  docker compose up -d postgres
  DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec \
    cargo test -p kernel --test kernel-postgres -- --ignored
  ```

- Each test runs inside its own transaction that is rolled back, so tests do
  not see each other's rows and can run in parallel.
