# Persistence

How the `kernel` bounded context stores its aggregates in PostgreSQL. The
domain event log that lives in the same database, and the unit of work that
writes state and events together, are described in
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
   text never reaches a cast: PostgreSQL accepts classful shorthand such as
   `10` for `10.0.0.0/8`.
5. **A duplicate never aborts a transaction.** Creates use
   `INSERT ... ON CONFLICT DO NOTHING` and read `rows_affected()`: zero rows
   means `AlreadyExists`. A unique violation (SQLSTATE 23505) would abort the
   whole unit of work, so the write path never relies on catching one.
6. **Rows map back through value object constructors.** A row that fails
   validation, or whose stored id differs from the id derived from its value,
   is a bug surfaced as `Unexpected`, never data accepted.
7. **No mixed mode.** Either every kernel module persists in PostgreSQL or
   none does. A module whose state lived in memory while its events reached
   the durable log would make the log lie after a restart.

## Platform

| Choice | Value |
|---|---|
| Server | PostgreSQL 18 (`postgres:18-alpine` in `docker-compose.yml` and as the CI service container). The floor is 17, the first version with identity columns on partitioned tables. |
| Driver | `sqlx` 0.8 with its default features (`macros`, `migrate`, `json`) plus `runtime-tokio`, `postgres`, `uuid`, `time`, `ipnet` and `mac_address` |
| Queries | Runtime queries (`sqlx::query`, `query_as` with `FromRow`) only. The compile-time `query!` macros are not used, so builds never need a database or a `.sqlx` cache across the three CI toolchains |
| Migrations | Embedded with `sqlx::migrate!("./migrations")` from `libs/kernel/migrations/`, one numbered file per change (`0001_create_event_store.sql`, `0002_create_ipv4_address.sql`). `libs/kernel/build.rs` prints `cargo:rerun-if-changed=migrations`, because Cargo does not otherwise rebuild the crate when only a migration changes. |
| Shared infrastructure | `libs/shared/persistence-postgres` (crate `shared-persistence-postgres`): the unit of work, the transactional command bus decorator, the outbox event bus, the event log writer and the partition maintenance. It contains no CTI logic. |
| Extensions | None |
| Isolation | `READ COMMITTED` for every write transaction |

## Wiring

- `build_state()` stays synchronous, takes no arguments, never reads the
  environment and always wires the in-memory repositories and the in-memory
  event bus. Unit and e2e tests use it and need no database.
- `build_postgres_state(pool: PgPool)` is async and wires the PostgreSQL
  repositories, the outbox event bus and the transactional decorator around
  the command bus. It exists once every kernel module has a PostgreSQL
  repository (rule 7).
- `main.rs` reads `DATABASE_URL`. When it is set, it builds the pool once
  before `HttpServer::new`, runs the embedded migrations and the partition
  maintenance, and calls `build_postgres_state`. When it is not, it calls
  `build_state`.

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
  is unpartitioned, so such a table has two unique B-trees. If the kind is
  ever hash-partitioned by `id`, that constraint is dropped and uniqueness
  rests on the deterministic id alone.
- Unbounded text keys (URLs, user agents) are unique on a SHA-256 digest of
  the canonical value, computed in Rust and stored as `bytea`, because a
  B-tree entry cannot exceed about a third of a page.
- Constraint names follow `<table>_<column>_<rule>` and
  `<table>_<column>_key`, so an error can be mapped by name.

A regime B kind (`file`) has a client-supplied `id uuid PRIMARY KEY`, its
STIX identifier as a column, and one partial unique index per natural key
(`CREATE UNIQUE INDEX file_sha256_key ON kernel.file (sha256) WHERE sha256 IS NOT NULL`).

Mutable aggregates add `updated_at timestamptz NOT NULL`. They need no
version column: the unit of work serialises commands per aggregate (see
[event-store.md](event-store.md#one-command-at-a-time-per-aggregate)), so a
read-modify-write inside one command cannot lose a concurrent update.

## Type mapping

| Domain value | Bound as | Column |
|---|---|---|
| Aggregate ids | `uuid::Uuid` | `uuid` |
| IPv4 or IPv6 host | `std::net::IpAddr` | `inet` with a full-width mask `CHECK` |
| IPv4 or IPv6 network | The canonical string, cast in SQL as `$1::text::cidr` | `cidr` with a `masklen` bound `CHECK` |
| MAC address | `mac_address::MacAddress` | `macaddr` |
| Hex digests and fingerprints | `Vec<u8>` (decoded in Rust) | `bytea` with an `octet_length` `CHECK` |
| Unsigned 32-bit numbers (AS numbers, watermarks) | `i64` | `bigint` with a range `CHECK` |
| Timestamps (`SystemTime`) | `time::OffsetDateTime` | `timestamptz` |
| Canonical text | `&str` | `text COLLATE "C"` |

Networks are bound as text on purpose. `sqlx` encodes `ipnet::IpNet` as an
`inet`, and the implicit `inet` to `cidr` cast silently zeroes host bits, so
a value object bug letting `198.51.100.7/24` through would be stored as
`198.51.100.0/24` under an id hashed from the wrong string. `cidr` input
from text rejects host bits instead. Decoding uses `ipnet::IpNet`.

Timestamps:

- `SystemTime` carries nanoseconds and `timestamptz` microseconds. Timestamp
  value objects truncate to microseconds when they are created, so an
  aggregate read back from PostgreSQL compares equal to the one that was
  saved, and the in-memory and PostgreSQL repositories behave identically.
- Timestamp value objects built from client input reject values outside
  1970-01-01 to 9999-12-31 UTC, the range both `time` and PostgreSQL
  represent. The adapter converts with checked arithmetic and maps a failure
  to `Unexpected` instead of panicking.

## Reads

- Inside a command, every repository call (reads included) runs on the unit
  of work's transaction, so a command uses exactly one connection and its
  reads see its own writes.
- Query handlers read through the pool directly, outside any unit of work,
  with one snapshot per statement.
- A regime A lookup by value never scans: the handler derives the
  deterministic id from the value and reads by primary key. A regime B
  lookup by natural key uses that key's unique index.

The write database serves only reads the write side needs (find by id or by
natural key). Search, listing by range, containment of addresses in networks
and similarity search on fuzzy digests belong to the read-model contexts that
will be built from the event log.

## Partitioning

Aggregate tables start unpartitioned. A regime A kind is hash-partitioned by
`id` (`PARTITION BY HASH (id)`, modulus 32 or 64 chosen for the three to five
year size) when its table approaches the server's memory, the threshold the
PostgreSQL documentation gives. Because its ids are derived from values,
every lookup still prunes to one partition. The first candidates are `url`,
`domain_name` and `ipv4_address`.

Regime B kinds (`file`, `x509_certificate`, `artifact`) are not partitioned
by id: a unique index on a partitioned table must contain the partition key,
so partitioning by id would drop the per-digest uniqueness that deduplicates
them. If one must be partitioned, each natural key moves to its own lookup
table (`kernel.file_sha256 (sha256 bytea PRIMARY KEY, file_id uuid NOT NULL)`)
hash-partitioned by that key.

The event log is range-partitioned by month from the first migration, see
[event-store.md](event-store.md).

## Testing

- Repository behaviour shared by every adapter (save, duplicate, find,
  delete, missing) is tested against the in-memory repository in the normal
  unit suite.
- The PostgreSQL repositories run the same contract in a separate test
  target, `kernel-postgres`. Each test is a `#[sqlx::test]` that gets a fresh
  database with the kernel migrations applied, so tests are isolated even
  when they insert the same deterministic ids, and concurrency tests can
  commit across connections. The tests are marked
  `#[ignore = "requires DATABASE_URL"]`.
- A second target, `cti-api-pg-e2e`, runs the HTTP API on
  `build_postgres_state` and asserts what only a real database can show:
  state and events commit together, a forced failure of the event append
  returns `500` and leaves neither the row nor the event, and concurrent
  creates of one value produce one row and one `Created` event.
- CI runs both targets with `-- --ignored` against a PostgreSQL 18 service
  container. Locally:

  ```bash
  docker compose up -d postgres
  export DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec
  cargo test -p kernel --test kernel-postgres -- --ignored
  cargo test -p cti-api --test cti-api-pg-e2e -- --ignored
  ```
