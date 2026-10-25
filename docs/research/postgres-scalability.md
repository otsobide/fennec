# PostgreSQL write-side design for Fennec observables and the domain event log

Research notes for the observable/IOC data-model design. Target: one strongly typed table per observable kind (no generic type/value table), hundreds of millions of observables, a durable domain event log, Rust + sqlx 0.8.x + Actix-Web, DDD/hexagonal/CQRS as in the current Fennec codebase.

## 0. Conventions, sources, labels

- **[SRC]** = stated by source code or official docs, with a citation. **[REC]** = my recommendation. **[INF]** = my inference from the sources (not stated by them). **[VERIFY]** = I am not fully sure; check before relying on it.
- Paths are relative to the clone root they belong to:
  - `FENNEC/` = the Fennec repository root (the repo itself, read-only).
  - `OCTI/` = `.opencti/`, with `GQL/` = `.opencti/opencti-platform/opencti-graphql/`.
  - `MISP/` = `.misp/`.
  - `STIX/` = `.stix2-json-schemas/`.
  - `SQLX-CORE/`, `SQLX-PG/`, `SQLX/` = `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-core-0.9.0/`, `sqlx-postgres-0.9.0/`, `sqlx-0.9.0/`. Only **0.9.0** sources are in the local cargo registry; no 0.8.x source is cached. Where I cite 0.9.0 code for something the task asks about in 0.8.x, I say whether it differs.
  - `PGPROC` = `src/include/catalog/pg_proc.dat` of PostgreSQL `REL_17_STABLE` (downloaded to the scratchpad for grepping function volatility).
- PostgreSQL docs quoted: `docs/current` (PostgreSQL 18 at the time of writing; PG 19 is in beta, beta 4 on 2026-09-24 per postgresql.org news).
- sqlx release facts come from the upstream `CHANGELOG.md` on GitHub (main branch): **0.8.6 is the last 0.8.x (2025-05-19); 0.9.0 was released 2026-05-06.** `ipnet` support was added in 0.8.4 (PR #3710). 0.9.0 makes every `query*()` function take `impl SqlSafeStr` (dynamic SQL must be wrapped in `AssertSqlSafe`, PR #3723), removes the combined `runtime-*-tls` features (PR #3821), and its MSRV is 1.94.0.

---

## 1. Fennec baseline facts that constrain the database design [SRC]

| # | Fact | Evidence |
|---|---|---|
| F1 | `EventBus::publish` is **synchronous**: `fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError>`. A Postgres insert cannot be awaited inside it. | `FENNEC/libs/shared/domain-events/src/domain/event_bus.rs:37-48` |
| F2 | `DomainEvent` exposes only `event_name`, `aggregate_id` (`&str`), `event_id` (`&str`), `occurred_on` (`SystemTime`). There is **no payload/serialisation method**, so an event cannot be written to a table today. | `FENNEC/libs/shared/domain-events/src/domain/domain_event.rs:55-67` |
| F3 | `DomainEventBase::new` generates `event_id = Uuid::new_v4()` and `occurred_on = SystemTime::now()`. UUIDv4 is random: bad for a btree on an append-only table. | `domain_event.rs:88-94` (line 91 `Uuid::new_v4()`) |
| F4 | `InMemoryEventBus::publish` runs subscribers in-line and stops at the first error. No subscribers are registered anywhere (`AGENTS.md`). | `FENNEC/libs/shared/domain-events/src/infrastructure/in_memory/in_memory_event_bus.rs:104-114` |
| F5 | Services persist, then build the event with a factory, then publish; a publish failure is mapped to `Unexpected` **after** the write already happened. | `FENNEC/libs/kernel/src/ioc/application/create_ioc/ioc_creator.rs:47-52`; `.../sighting/application/create_sighting/sighting_creator.rs:57-62` |
| F6 | `SightingObserver` does **read-modify-write** (`find_by_id`, `observe`, `update`) with no version check. On a shared database that is a lost-update race. | `FENNEC/libs/kernel/src/sighting/application/observe_sighting/sighting_observer.rs:40-52` |
| F7 | Repository errors are small closed enums; `AlreadyExists`/`NotFound`/`Unexpected(String)`. Sighting adds `IdAlreadyExists` and `PairAlreadyExists { existing_id }`, with pair collision taking precedence over id collision. | `.../ioc/domain/errors/ioc_repository_error.rs:9-21`; `.../sighting/domain/errors/sighting_repository_error.rs:11-30`; precedence in `.../sighting/infrastructure/persistence/in_memory/in_memory_sighting_repository.rs:43-60` |
| F8 | Handlers convert errors to a `concept` string; controllers map `AlreadyExists` to 409, `NotFound` to 404, else 500; a `CommandBusError` is 500. | `.../ioc/application/create_ioc/create_ioc_command_handler.rs:36-49`; `FENNEC/apps/cti_api/src/ioc/controllers/post.rs:42-66` |
| F9 | `CommandBus::dispatch(Box<dyn Command>) -> Result<Box<dyn Any + Send + Sync>, CommandBusError>`; handler futures are boxed `+ Send` and awaited in the caller's task (not spawned). `CommandBusError::HandlerError(String)` exists. | `FENNEC/libs/shared/cqrs/src/command/domain/command_bus.rs:48-65`; `.../command/infrastructure/in_memory/in_memory_command_bus.rs:122-124`; `.../command/domain/command_bus_error.rs` |
| F10 | `build_state()` is a synchronous fn called **once** in `main` and cloned into every Actix worker; e2e tests call it per test. | `FENNEC/apps/cti_api/src/lib.rs:69-175`; `FENNEC/apps/cti_api/src/main.rs:17,22-26` |
| F11 | Current `Ioc` is the generic model the new design removes: `IocType` enum + `IocValue` string capped at `MAX_LENGTH = 2048` **characters** (so up to 8192 UTF-8 bytes). | `FENNEC/libs/kernel/src/ioc/domain/value_objects/ioc_type.rs:10-27`; `.../ioc_value.rs:8,26-42` |
| F12 | `SightingCount` wraps `u64`; timestamps are `SystemTime`. sqlx-postgres has **no** `Encode/Decode` for `u64` or `SystemTime` (no match for `SystemTime` under `SQLX-PG/src/types/`). | `FENNEC/libs/kernel/src/sighting/domain/value_objects/sighting_count.rs:12`; grep result |
| F13 | Config context uses a string key as aggregate id (`DomainEventBase::new(key.value().to_string())`); kernel aggregates use UUIDs. | `FENNEC/libs/config/src/config_entry/domain/events/config_entry_created_event.rs:28` vs `FENNEC/libs/kernel/src/sighting/domain/events/sighting_observed_event.rs:41` |
| F14 | Event names are already namespaced under the kernel: `fennec.kernel.ioc.created`, `fennec.kernel.sighting.observed`. (AGENTS.md still describes separate `libs/ioc`/`libs/sighting` crates; the workspace and `cti_api` now use `kernel::ioc` and `kernel::sighting`.) | `.../ioc/domain/events/ioc_created_event.rs:27`; `.../sighting_observed_event.rs:29`; `FENNEC/Cargo.toml` members; `FENNEC/apps/cti_api/src/lib.rs:5-46` |
| F15 | Postgres scaffolding: `postgres:16-alpine`, user/db `template`; `.env.example` has `DATABASE_URL=postgres://template:template@localhost:5432/template`. CI runs `cargo test --workspace` + dev/release builds on stable, beta, nightly; no DB service. | `FENNEC/docker-compose.yml`; `FENNEC/.env.example`; `FENNEC/.github/workflows/test-and-build.yml` |
| F16 | House rules for real adapters: translate driver errors into the domain enum (`Unexpected(stringified)` otherwise) and rebuild aggregates through value-object constructors ("a row that fails validation is a bug surfaced"). Postgres adapters are expected under `infrastructure/persistence/<technology>/` and named `SqlxPostgresNounRepository`. | `FENNEC/.knowledge/development/ddd/infrastructure.md:14-23`; `FENNEC/docs/architecture/project-structure.md` (naming table, row "Postgres impl (future)") |

---

## 2. Platform choices

- **[REC] PostgreSQL 18** (bump `docker-compose.yml` from `postgres:16-alpine`). Features used below and the version that introduced them [SRC: release notes]:
  - PG 13: `xid8`, `pg_current_xact_id()`, `pg_snapshot_xmin(pg_current_snapshot())` (catalog entries `PGPROC:10298-10319`; `pg_current_xact_id` and `pg_current_snapshot` are `provolatile => 's'`).
  - PG 13: BEFORE ROW triggers on partitioned tables (and the docs: "`BEFORE ROW` triggers on `INSERT` cannot change which partition is the final destination for a new row").
  - PG 14: BRIN `minmax_multi`/`bloom` opclasses; `default_toast_compression = lz4`. [VERIFY version numbers from memory]
  - PG 15: `UNIQUE NULLS NOT DISTINCT`; `wal_compression = zstd|lz4`. [VERIFY from memory]
  - PG 16: "Allow HOT updates if only BRIN-indexed columns are updated" [SRC: 16.0 release notes].
  - PG 17: "Allow partitioned tables to have identity columns"; "Add a builtin platform-independent collation provider ... supports `C` and `C.UTF-8`"; "Allow exclusion constraints on partitioned tables" [SRC: 17.0 release notes]. `transaction_timeout` GUC [VERIFY: from memory, PG 17].
  - PG 18: `uuidv7()`; B-tree skip scan; asynchronous I/O; `RETURNING old.*/new.*`; virtual generated columns become the **default** (write `STORED` explicitly); LIKE with nondeterministic collations; `WITHOUT OVERLAPS` temporal keys [SRC: 18.0 release notes].
- **[REC] Extensions:** none required for the write side. `pg_trgm` only if substring search is served from the write database (section 5.4). Do **not** install `citext` (section 3.5). Install `btree_gist` only if you need GiST exclusion constraints mixing scalars and ranges, and never rely on the default GiST opclass for `inet` when it is installed (section 5.2).
- **[REC] One schema per bounded context** (`kernel`, `config`) plus an `event_store` schema per database. Two roles: `fennec_owner` (owns objects, runs migrations) and `fennec_app` (DML only). This is what makes "append-only" enforceable by privileges (section 7.3).

---

## 3. Native column types

### 3.1 Type map per observable kind [REC unless marked]

| Fennec aggregate (STIX type) | Natural key column | Type | DB invariant (CHECK) | Notes |
|---|---|---|---|---|
| `Ipv4Address` (`ipv4-addr`, host) | `address` | `inet` | `family(address) = 4 AND masklen(address) = 32` | 7 bytes on disk [SRC: Table 8.21]. inet equality includes the mask (`'1.2.3.4/24'::inet <> '1.2.3.4'::inet`), hence the CHECK. |
| `Ipv4Network` (`ipv4-addr`, CIDR value) | `network` | `cidr` | `family(network) = 4 AND masklen(network) < 32` | `cidr` rejects host bits [SRC: "inet accepts values with nonzero bits to the right of the netmask, whereas cidr does not"]. `/32` is normalised to the host aggregate (MISP does the same: `MISP/app/Lib/Tools/AttributeValidationTool.php:774-777`). |
| `Ipv6Address` | `address` | `inet` | `family(address) = 6 AND masklen(address) = 128` | 19 bytes. |
| `Ipv6Network` | `network` | `cidr` | `family(network) = 6 AND masklen(network) < 128` | |
| `MacAddress` (`mac-addr`) | `value` | `macaddr` | none needed | 6 bytes. STIX only allows EUI-48 lowercase colon form `^([0-9a-f]{2}[:]){5}([0-9a-f]{2})$` (`STIX/schemas/observables/mac-addr.json:26`), so `macaddr8` is not needed for STIX. Postgres output format is lowercase colon-separated, matching STIX [INF]. |
| `AutonomousSystem` | `number` | `bigint` | `number BETWEEN 0 AND 4294967295` | STIX `number` is an unbounded `integer` (`STIX/schemas/observables/autonomous-system.json:24-27`); ASNs are 32-bit unsigned (RFC 6793) so `integer` (max 2147483647) is too small. |
| `DomainName` | `value` | `text COLLATE "C"` | `octet_length(value) BETWEEN 1 AND 253 AND value ~ '^[a-z0-9_.-]+$' AND value !~ '\.$'` (if the canonical form is the A-label) | Normalise on write (section 3.5). STIX declares `format: idn-hostname` (`STIX/schemas/observables/domain-name.json:27`), so whether the canonical form is A-label or U-label is a design decision that changes the standard id (open question Q2). |
| `EmailAddress` | `value` | `text COLLATE "C"` | `position('@' in value) > 1` (+ length) | STIX `format: email` (`STIX/schemas/observables/email-addr.json:26`). |
| `Url` | `value` + `value_sha256` | `text COLLATE "C"` + `bytea` | `octet_length(value_sha256) = 32` | Uniqueness on the digest, not on the text (btree entry limit, section 3.4). |
| `File` | `md5`, `sha1`, `sha256`, `sha512` | `bytea` each (nullable) | `octet_length(md5) = 16` etc., `num_nonnulls(md5, sha1, sha256, sha512) > 0 OR name IS NOT NULL` | One partial UNIQUE index per digest (section 4.6). `ssdeep`/`tlsh` stay `text` (not fixed-length binary; STIX patterns `^[a-zA-Z0-9/+:.]{1,128}$` and `^[a-zA-Z0-9]{70}$`, `STIX/schemas/common/hashes-type.json:43-51`). |
| Every aggregate | `id` | `uuid` | | 16 bytes fixed, no varlena header. |
| Every aggregate | `created_at`, `updated_at`, `first_seen`, `last_seen` | `timestamptz` | `first_seen <= last_seen` | Section 3.7. |
| Every mutable aggregate | `version` | `bigint NOT NULL DEFAULT 1` | `version >= 1` | Optimistic concurrency (section 4.5). |
| Ports (network-traffic) | `src_port`, `dst_port` | `integer` | `BETWEEN 0 AND 65535` | `smallint` is signed (max 32767). |
| STIX `confidence` | `confidence` | `smallint` | `BETWEEN 0 AND 100` | |
| Reference to "any observable" (e.g. sighting_of) | `observable_kind`, `observable_id` | Postgres `ENUM` + `uuid` | | No FK: contexts reference each other by UUID only (AGENTS.md). Mirrors the STIX `type--uuid` identifier prefix (`STIX/schemas/common/identifier.json:7`). sqlx maps enums with `#[derive(sqlx::Type)] #[sqlx(type_name = "...", rename_all = "snake_case")]` (`SQLX-PG/src/types/mod.rs`, Enumerations section). |

### 3.2 inet/cidr: host vs network, parsing, indexing

- [SRC] Storage: `cidr` and `inet` are "7 or 19 bytes", `macaddr` 6, `macaddr8` 8 (docs Table 8.21). On display "the /y portion is suppressed if the netmask specifies a single host" (inet), while `cidr` keeps it (`10.1.2.3/32` -> `10.1.2.3/32`, Table 8.22).
- [SRC] **Postgres accepts classful abbreviations for `cidr` input**: `10` -> `10.0.0.0/8`, `192.168` -> `192.168.0.0/24`, `128.1` -> `128.1.0.0/16` (Table 8.22). [REC] Never let Postgres parse raw user text into `inet`/`cidr`. Parse in the Rust value object (`std::net::Ipv4Addr`, `ipnet::Ipv4Net`) and bind typed values.
- [SRC] sqlx: `ipnetwork::IpNetwork` and `std::net::IpAddr` map to INET and CIDR with feature `ipnetwork`; `ipnet::IpNet` with feature `ipnet` (the `ipnetwork` feature "takes precedence over `ipnet` if both are used"). Decoding `IpAddr` from a value with a prefix shorter than the full width is an error (`SQLX-PG/src/types/mod.rs:88-118`). The encoder always sends `is_cidr = 0` and declares INET (`SQLX-PG/src/types/ipnetwork/ipnetwork.rs:38-60`), and the type is `compatible` with CIDR (`ipnetwork.rs:23-24`). [INF] A bound INET parameter written into a `cidr` column goes through the inet-to-cidr cast; the value object must already guarantee zero host bits, so the database never sees an ambiguous value.
- [REC] **Two aggregates per family (host vs network), one STIX type.** Host and network have different query patterns and invariants (a host is a point, a network is a range). Both serialise to `ipv4-addr` with `value` = `a.b.c.d` or `a.b.c.d/nn` (`STIX/schemas/observables/ipv4-addr.json:5,24-28`: "one or more IPv4 addresses expressed using CIDR notation").
- [REC] **Non-CIDR ranges** (feeds often publish `1.2.3.4-1.2.3.9`): decompose into the minimal CIDR set in Rust (`ipnet` subnets iterator) and create one `Ipv4Network`/`Ipv4Address` per block. That keeps everything STIX-expressible. A custom `CREATE TYPE inet_range AS RANGE (SUBTYPE = inet)` with a GiST index is possible but is not representable in STIX `ipv4-addr`.
- [REC] Canonical text for the standard id is produced by Rust, never by `host()`/`text()` in SQL. Rust's `Ipv6Addr` `Display` follows RFC 5952 compression [INF]; Postgres has its own formatter. MISP normalises IPv6 with `inet_pton`/`inet_ntop` and strips `/32` and `/128` (`MISP/app/Lib/Tools/AttributeValidationTool.php:763-788`).

### 3.3 macaddr vs macaddr8

- [SRC] `macaddr8`: "MAC addresses given in 6 byte format will be stored in 8 byte length format with the 4th and 5th bytes set to FF and FE" (docs 8.9.5). sqlx only ships a Rust mapping for `MACADDR` (`mac_address::MacAddress`, 6 bytes, `SQLX-PG/src/types/mac_address.rs:9-50`); `PgType::Macaddr8` exists only as type info (`SQLX-PG/src/type_info.rs:83,383,491`). [REC] Use `macaddr`. If EUI-64 is ever needed, it is a separate aggregate (`Eui64Address`, not STIX `mac-addr`) bound as `[u8; 8]` through a custom `Encode` or as text with `$1::macaddr8`.

### 3.4 Hex digests: bytea vs text

Sizes below: heap datum = short varlena header (1 byte, data up to 126 bytes) + data, or 4-byte header above that; btree leaf entry = 8-byte `IndexTupleData` + datum, MAXALIGN(8), + 4-byte line pointer. [INF: computed from the on-disk format; verify with `pg_column_size()` and `pgstattuple` on real data.]

| Algorithm | text hex heap | bytea heap | text index entry | bytea index entry | uuid (MD5 only) |
|---|---|---|---|---|---|
| MD5 | 33 B | 17 B | 52 B | 36 B | 16 B heap / 28 B index |
| SHA-1 | 41 B | 21 B | 60 B | 36 B | |
| SHA-256 | 65 B | 33 B | 84 B | 52 B | |
| SHA-512 | 132 B (4-byte header) | 65 B | 148 B | 84 B | |

Order-of-magnitude for 100 M SHA-256 rows with random inserts (about 70 % leaf fill [INF]): bytea unique index about 7 GiB vs text about 11 GiB.

- Comparison cost [INF]: `bytea` compares with `memcmp`; `text` under a libc/ICU collation calls the collation routine for ordering (equality on deterministic collations is still bytewise, but btree descent uses ordering comparisons). `COLLATE "C"` makes text comparisons bytewise too, but the size penalty stays.
- Case: hex text needs case normalisation; bytes have none. OpenCTI does **not** lowercase hash values before computing standard ids (see sibling note `opencti-identity-and-storage.md` section 1.5), so a Fennec canonical form (lowercase hex in the id JSON) produces a different id than OpenCTI for upper-case input [INF]. Keep OpenCTI's id as an alias (`x_opencti_stix_ids` equivalent) if interop needs it.
- [REC] `bytea` with `CHECK (octet_length(x) = N)`, decoded and validated in the value object (exact length, hex alphabet per `STIX/schemas/common/hashes-type.json:13-41`). Render with `encode(x, 'hex')` only for ad-hoc SQL; the Rust value object renders lowercase hex.
- Btree entry limit [SRC]: "an index entry cannot exceed approximately one-third of a page (after TOAST compression, if applicable)" (docs 65.1.1; the error text is "index row size ... exceeds btree version 4 maximum 2704"). So a UNIQUE btree directly on an unbounded URL fails for long URLs. [REC] Unique on `value_sha256 bytea`.
- [SRC] `convert_to(text, name)` is **STABLE** (`PGPROC:3765-3768`, `provolatile => 's'`), as is `textsend` (`PGPROC:7910-7912`), so `sha256(convert_to(value, 'UTF8'))` **cannot** be a generated column or index expression. `sha256(bytea)` itself is immutable (`PGPROC:7706-7708`). `value::bytea` is an I/O-conversion cast that interprets backslash escapes, so it corrupts values containing `\` [INF]. [REC] Compute the digest in Rust (the value object owns canonicalisation) and bind it.
- Only B-tree can be UNIQUE [SRC: "Currently, only B-tree indexes can be declared unique."], so a `hash` index cannot replace the digest column for uniqueness.

### 3.5 Case-insensitive values: citext vs lower() vs nondeterministic collation vs normalise-on-write

| Option | Cost | Problems | Verdict |
|---|---|---|---|
| `citext` | [SRC] "operator functions and the B-tree comparison functions must make copies of the data and convert it to lower case for comparisons. Also, only `text` can support B-Tree deduplication." The docs themselves say "Consider using nondeterministic collations ... instead of this module." | Extension dependency; slower comparisons; sqlx treats CITEXT as `String` but needs `PgCiText` for arrays (`SQLX-PG/src/types/mod.rs`, footnote 1). | No |
| `text` + `UNIQUE (lower(value))` expression index | `lower()` per insert and per lookup; query must be written as `lower(value) = lower($1)` to use the index. `lower` is immutable (`PGPROC:3566-3567`). | ON CONFLICT must name the expression (`ON CONFLICT ((lower(value)))`); the stored value keeps the original case, so two spellings coexist in reads. | Only if the original spelling must be preserved |
| Nondeterministic ICU collation (`deterministic = false`) | Comparisons through ICU | [SRC] B-tree deduplication is disabled for nondeterministic collations (docs 65.1.4.3); LIKE only supported from PG 18 [SRC]; pattern/trigram behaviour differs. | No |
| **Normalise on write in the value object, store `text COLLATE "C"`, plain `UNIQUE (value)`** | Bytewise compare, smallest index, dedup-capable for non-unique indexes, prefix LIKE works on a plain btree in C collation | Normalisation rules must be versioned because they feed the standard id | **Yes [REC]** |

- [SRC] MISP normalises on write: lowercase for hashes, emails, hostnames; domains lowercased, trimmed of `.`, converted to punycode when invalid (`MISP/app/Lib/Tools/AttributeValidationTool.php:66-118`). OpenCTI normalises at query time with an Elasticsearch keyword normalizer `filter: ['lowercase', 'asciifolding']` (`GQL/src/database/engine.ts:1183-1186`) and lowercases Domain-Name/Email-Addr in the GraphQL add path (sibling note `opencti-identity-and-storage.md` line 77-78, citing `GQL/src/domain/stixCyberObservable.js:212-215`).
- [REC] Add a light CHECK as a backstop of the value-object invariant (e.g. `value ~ '^[a-z0-9_.-]+$'` for A-label domains). It costs one regex per insert and turns a normalisation bug into a constraint error instead of a silent duplicate.

### 3.6 Numeric and range types

- [REC] Use the narrowest exact integer that holds the domain plus a CHECK (ports `integer`, ASN `bigint`, counts `bigint CHECK (count >= 1)`). `u64` in Rust becomes `i64` in the adapter with `i64::try_from`, failing as `Unexpected` (sqlx has no unsigned support, F12).
- [REC] Range types (`int4range`, `int8range`, `tstzrange`) only when the domain is a range (e.g. a port range in a firewall-rule aggregate, or an indicator validity window). PG 18 `WITHOUT OVERLAPS` [SRC] can enforce non-overlapping validity per key if that is ever needed; sqlx maps `PgRange<T>` (`SQLX-PG/src/types/mod.rs`, Types table).
- Avoid `numeric` for keys: [SRC] "`numeric` cannot use deduplication" (docs 65.1.4.3), and it is variable-width.

### 3.7 Timestamps

- [REC] Always `timestamptz` (8 bytes, stored as UTC microseconds). Map `SystemTime` to `time::OffsetDateTime` (feature `time`) or `chrono::DateTime<Utc>` (feature `chrono`) in the adapter; sqlx has no `SystemTime` mapping (F12).
- [INF] Precision trap: `SystemTime` carries nanoseconds, Postgres microseconds. A value object that round-trips through the database compares unequal unless timestamps are truncated to microseconds at construction. Truncate in the value object (`*_at::now()`), not only in the adapter, so in-memory and Postgres repositories behave identically.
- [SRC] STIX timestamps allow arbitrary fractional digits and second `60` (`STIX/schemas/common/timestamp.json:7`). Parse in Rust; decide the leap-second policy there.

### 3.8 Example DDL for typed observable tables [REC]

Column order puts 8-byte-aligned columns first to avoid padding. `id` semantics are discussed in section 4.1 (deterministic STIX UUID for value-identified kinds).

```sql
-- migrations/0002_kernel_observables.sql
CREATE SCHEMA IF NOT EXISTS kernel;

CREATE TABLE kernel.ipv4_address (
    id          uuid        NOT NULL,
    created_at  timestamptz NOT NULL,
    updated_at  timestamptz NOT NULL,
    version     bigint      NOT NULL DEFAULT 1,
    address     inet        NOT NULL,
    CONSTRAINT ipv4_address_pkey        PRIMARY KEY (id),
    CONSTRAINT ipv4_address_address_key UNIQUE (address),
    CONSTRAINT ipv4_address_is_v4_host  CHECK (family(address) = 4 AND masklen(address) = 32),
    CONSTRAINT ipv4_address_version_pos CHECK (version >= 1)
);

CREATE TABLE kernel.ipv4_network (
    id          uuid        NOT NULL,
    created_at  timestamptz NOT NULL,
    updated_at  timestamptz NOT NULL,
    version     bigint      NOT NULL DEFAULT 1,
    network     cidr        NOT NULL,
    CONSTRAINT ipv4_network_pkey        PRIMARY KEY (id),
    CONSTRAINT ipv4_network_network_key UNIQUE (network),
    CONSTRAINT ipv4_network_is_v4_net   CHECK (family(network) = 4 AND masklen(network) < 32)
);
-- "which known networks contain this host?" (>>=). inet_ops must be named explicitly.
CREATE INDEX ipv4_network_network_spgist ON kernel.ipv4_network USING spgist (network inet_ops);

CREATE TABLE kernel.mac_address (
    id uuid NOT NULL, created_at timestamptz NOT NULL, updated_at timestamptz NOT NULL,
    version bigint NOT NULL DEFAULT 1,
    value macaddr NOT NULL,
    CONSTRAINT mac_address_pkey PRIMARY KEY (id),
    CONSTRAINT mac_address_value_key UNIQUE (value)
);

CREATE TABLE kernel.autonomous_system (
    id uuid NOT NULL, created_at timestamptz NOT NULL, updated_at timestamptz NOT NULL,
    version bigint NOT NULL DEFAULT 1,
    number bigint NOT NULL,
    name   text,
    rir    text,
    CONSTRAINT autonomous_system_pkey PRIMARY KEY (id),
    CONSTRAINT autonomous_system_number_key UNIQUE (number),
    CONSTRAINT autonomous_system_number_range CHECK (number BETWEEN 0 AND 4294967295)
);

CREATE TABLE kernel.domain_name (
    id uuid NOT NULL, created_at timestamptz NOT NULL, updated_at timestamptz NOT NULL,
    version bigint NOT NULL DEFAULT 1,
    value text COLLATE "C" NOT NULL,
    CONSTRAINT domain_name_pkey PRIMARY KEY (id),
    CONSTRAINT domain_name_value_key UNIQUE (value),
    CONSTRAINT domain_name_value_canonical CHECK (
        octet_length(value) BETWEEN 1 AND 253
        AND value ~ '^[a-z0-9_.-]+$'
        AND value !~ '\.$')
);
-- Optional: subdomain/suffix search via reversed text (reverse() is immutable, PGPROC:4705-4706).
CREATE INDEX domain_name_value_reversed_idx ON kernel.domain_name ((reverse(value)));

CREATE TABLE kernel.url (
    id uuid NOT NULL, created_at timestamptz NOT NULL, updated_at timestamptz NOT NULL,
    version bigint NOT NULL DEFAULT 1,
    value_sha256 bytea NOT NULL,              -- sha256 of the canonical UTF-8 value, computed in Rust
    value        text COLLATE "C" NOT NULL,
    CONSTRAINT url_pkey PRIMARY KEY (id),
    CONSTRAINT url_value_sha256_key UNIQUE (value_sha256),
    CONSTRAINT url_value_sha256_len CHECK (octet_length(value_sha256) = 32)
);

CREATE TABLE kernel.file (
    id uuid NOT NULL, created_at timestamptz NOT NULL, updated_at timestamptz NOT NULL,
    version bigint NOT NULL DEFAULT 1,
    size   bigint,
    stix_id uuid NOT NULL,                    -- current standard id (can change when hashes are added)
    md5    bytea, sha1 bytea, sha256 bytea, sha512 bytea,
    ssdeep text, tlsh text,
    name   text,
    CONSTRAINT file_pkey PRIMARY KEY (id),
    CONSTRAINT file_stix_id_key UNIQUE (stix_id),
    CONSTRAINT file_md5_len    CHECK (md5    IS NULL OR octet_length(md5)    = 16),
    CONSTRAINT file_sha1_len   CHECK (sha1   IS NULL OR octet_length(sha1)   = 20),
    CONSTRAINT file_sha256_len CHECK (sha256 IS NULL OR octet_length(sha256) = 32),
    CONSTRAINT file_sha512_len CHECK (sha512 IS NULL OR octet_length(sha512) = 64),
    CONSTRAINT file_has_identity CHECK (num_nonnulls(md5, sha1, sha256, sha512) > 0 OR name IS NOT NULL),
    CONSTRAINT file_size_pos CHECK (size IS NULL OR size >= 0)
);
CREATE UNIQUE INDEX file_md5_key    ON kernel.file (md5)    WHERE md5    IS NOT NULL;
CREATE UNIQUE INDEX file_sha1_key   ON kernel.file (sha1)   WHERE sha1   IS NOT NULL;
CREATE UNIQUE INDEX file_sha256_key ON kernel.file (sha256) WHERE sha256 IS NOT NULL;
CREATE UNIQUE INDEX file_sha512_key ON kernel.file (sha512) WHERE sha512 IS NOT NULL;

CREATE TYPE kernel.observable_kind AS ENUM (
    'ipv4_address', 'ipv4_network', 'ipv6_address', 'ipv6_network',
    'mac_address', 'autonomous_system', 'domain_name', 'url', 'email_address', 'file');

CREATE TABLE kernel.sighting (
    id              uuid        NOT NULL,
    observable_id   uuid        NOT NULL,
    source_id       uuid        NOT NULL,
    first_seen      timestamptz NOT NULL,
    last_seen       timestamptz NOT NULL,
    count           bigint      NOT NULL,
    created_at      timestamptz NOT NULL,
    updated_at      timestamptz NOT NULL,
    version         bigint      NOT NULL DEFAULT 1,
    observable_kind kernel.observable_kind NOT NULL,
    CONSTRAINT sighting_pkey PRIMARY KEY (id),
    CONSTRAINT sighting_pair_key UNIQUE (observable_id, source_id),  -- serves list-by-observable too
    CONSTRAINT sighting_count_pos CHECK (count >= 1),
    CONSTRAINT sighting_seen_order CHECK (first_seen <= last_seen)
) WITH (fillfactor = 85);                                            -- room for HOT updates on observe
CREATE INDEX sighting_source_id_idx ON kernel.sighting (source_id);  -- list-by-source
```

Notes on the DDL:
- Constraint names are explicit because the Rust error mapper keys on `constraint()` (section 9.5).
- Observables are effectively immutable after creation [REC]: their identity is their value. Keep frequently changing data (`last_seen`, counts, scores, labels) on sightings or separate tables so observable rows almost never get UPDATEd (no bloat, no index churn).
- `sighting_pair_key (observable_id, source_id)` also serves `WHERE observable_id = $1` (leading column). [SRC] PG 18 skip scan lets a multicolumn btree serve predicates without the leading column, but only cheaply when the leading column has few distinct values; here it is high-cardinality, so list-by-source still needs its own index [INF].

---

## 4. Identity, uniqueness and deduplication

### 4.1 Which id is the primary key?

Evidence [SRC]:
- OpenCTI computes `standard_id` as `<stix-type>--` + UUIDv5 over the canonical JSON (`jsonCanonicalize`) of the id-contributing properties, namespace `OASIS_NAMESPACE = '00abedb4-aa42-466c-9c01-fed23315a9b7'` (`GQL/src/schema/general.js:56`; `GQL/src/schema/identifier.js:331-347,483-486`). Contributing properties for observables, e.g. `ENTITY_IPV4_ADDR: [{ src: 'value' }]`, `ENTITY_AUTONOMOUS_SYSTEM: [{ src: 'number' }]`, `ENTITY_HASHED_OBSERVABLE_STIX_FILE: [[{ src: 'hashes' }], [{ src: NAME_FIELD }]]` (`GQL/src/schema/identifier.js:96-140`). For File the id depends on which hashes are known and can be "upgraded" (`isStandardIdUpgraded`, `identifier.js:449-478`).
- OpenCTI also keeps a random `internal_id`, and the ES documents are keyed by it (`_id: doc._id ?? doc.internal_id`, `GQL/src/database/engine.ts:4867`).
- STIX identifiers are `^[a-z][a-z0-9-]+[a-z0-9]--<uuid>` (`STIX/schemas/common/identifier.json:7`).

[REC] Two regimes:

| Regime | Kinds | Primary key | Extra unique keys | Why |
|---|---|---|---|---|
| **A. Value-identified** | IPv4/IPv6 host and network, MAC, ASN, domain, URL, email | `id uuid` = the UUID part of the STIX standard id (UUIDv5 over canonical JSON of the contributing properties, OASIS namespace), computed by the value object | `UNIQUE (natural key)` as a safety net while the table is unpartitioned | Identity is immutable because the value is the identity. One random-key index instead of two; every lookup by value becomes a lookup by id (partition pruning works, section 6); ingestion is idempotent across processes and replicas; STIX export needs no lookup. |
| **B. Multi-key, mutable identity** | File (hash sets grow), and later X509, Artifact | `id uuid` client-supplied or random (UUIDv7 recommended) | `UNIQUE (stix_id)` + one partial UNIQUE per digest | The standard id changes when a better hash arrives, so it cannot be the PK. Same split as OpenCTI's internal_id/standard_id. |

- Fennec rule "ids are supplied by the client" (AGENTS.md) [REC] stays compatible with regime A if the server **verifies** that the supplied id equals the derived one (reject with 400 otherwise), or if observable create commands stop taking an id. This is a domain decision for the design agent (open question Q1).
- The UUID part is stored as `uuid` (16 bytes), not the `type--uuid` string (about 50 bytes); the type prefix is implied by the table.
- [INF] Random-looking keys (UUIDv4, UUIDv5, digests, inet values of attackers) all scatter btree inserts. Deterministic ids therefore do not add write cost compared with the natural-key index; they let you drop one of the two random indexes.

### 4.2 Idempotent ingestion with `INSERT ... ON CONFLICT`

[SRC] Facts from the INSERT docs:
- "`ON CONFLICT DO UPDATE` guarantees an atomic `INSERT` or `UPDATE` outcome; provided there is no independent error, one of those two outcomes is guaranteed, even under high concurrency."
- "For `ON CONFLICT DO NOTHING`, it is optional to specify a conflict_target; when omitted, conflicts with all usable constraints (and unique indexes) are handled. For `ON CONFLICT DO UPDATE`, a conflict_target must be provided."
- "Only rows that were successfully inserted or updated will be returned" by RETURNING; with `DO UPDATE ... WHERE`, a non-matching row is locked but not returned, and "all rows will be locked when the `ON CONFLICT DO UPDATE` action is taken".
- DO UPDATE "will not be allowed to affect any single existing row more than once; a cardinality violation error will be raised"; rows proposed for insertion "should not duplicate each other" on the arbiter.
- Partial unique indexes are inferred with `ON CONFLICT (col) WHERE predicate`.
- PG 18: "For a simple INSERT, all old values will be NULL. However, for an INSERT with an ON CONFLICT DO UPDATE clause, the old values may be non-NULL." (so `old.id IS NULL` means "inserted").

Patterns [REC]:

1. **Create (command semantics, AlreadyExists on duplicate).** Use `DO NOTHING` and check `rows_affected()`. This never raises 23505, so it never aborts an enclosing unit-of-work transaction (section 8):
   ```sql
   INSERT INTO kernel.ipv4_address (id, created_at, updated_at, version, address)
   VALUES ($1, $2, $2, 1, $3)
   ON CONFLICT DO NOTHING;
   -- rows_affected() == 0  ->  AlreadyExists (classify with a follow-up SELECT if the error needs detail)
   ```
2. **Get-or-create returning the id (feed ingestion).** The one-statement CTE `WITH ins AS (INSERT ... ON CONFLICT DO NOTHING RETURNING id) SELECT id FROM ins UNION ALL SELECT id FROM t WHERE key = $k LIMIT 1` has a race [INF, well known]: if the conflicting row was committed by a concurrent transaction after this statement's snapshot, the `SELECT` part cannot see it and the statement returns no row. Under READ COMMITTED, retry with a **separate** `SELECT` statement (new snapshot). With regime A ids this is unnecessary: the id is known before the insert.
3. **Upsert that merges (sighting observation), PG 18:**
   ```sql
   INSERT INTO kernel.sighting AS s
       (id, observable_id, observable_kind, source_id, first_seen, last_seen, count, created_at, updated_at, version)
   VALUES ($1, $2, $3, $4, $5, $5, 1, $6, $6, 1)
   ON CONFLICT (observable_id, source_id) DO UPDATE
   SET first_seen = LEAST(s.first_seen, EXCLUDED.first_seen),
       last_seen  = GREATEST(s.last_seen, EXCLUDED.last_seen),
       count      = s.count + 1,
       updated_at = EXCLUDED.updated_at,
       version    = s.version + 1
   RETURNING new.*, (old.id IS NULL) AS inserted;
   ```
   Before PG 18 the common trick is `RETURNING (xmax = 0) AS inserted`, which relies on an implementation detail [INF]. Evidence that this is the semantics CTI tools want: OpenCTI merges sightings with "oldest first_seen, newest last_seen" and adds `attribute_count` when the dates extend (`GQL/src/utils/upsert-utils.js:219-226`), under a Redis lock (section 10).
   - A no-op `DO UPDATE SET x = EXCLUDED.x` still writes a new row version (WAL, bloat, index churn when not HOT) [INF]. Never use a dummy DO UPDATE just to get RETURNING.
   - DDD tension: the merge rule moves into SQL. [REC] Use it only on the bulk ingestion path, with the repository returning the resulting aggregate (rebuilt from RETURNING through value objects) and an outcome flag, so the service still builds the event with the factory (`SightingCreated` vs `SightingObserved`).
4. **Batches.** `INSERT ... SELECT * FROM UNNEST($1::uuid[], $2::inet[], ...) ON CONFLICT DO NOTHING RETURNING id` (one round trip; sqlx binds `Vec<T>` as arrays). Deduplicate the batch in Rust first (cardinality rule) and **sort it by the conflict key**, otherwise two concurrent batches that overlap in different orders can deadlock waiting on each other's speculative insertions [INF]. For very large loads, `COPY` into an `UNLOGGED` staging table then `INSERT ... SELECT ... ON CONFLICT` (sqlx: `PgConnection::copy_in_raw`, `SQLX-PG/src/copy.rs:34`).
5. **Isolation.** [REC] READ COMMITTED for write transactions. Under REPEATABLE READ/SERIALIZABLE, a conflict with a row that is not visible to the snapshot raises a serialization failure instead of taking the ON CONFLICT path [VERIFY: from memory], which would turn normal duplicates into retries.

### 4.3 Race conditions and how each is closed

| Race | Where | Closed by |
|---|---|---|
| Two creates of the same observable | any create | UNIQUE constraint + `ON CONFLICT DO NOTHING`; exactly one transaction gets `rows_affected = 1` and emits `Created` [REC] |
| Create with client id X for value A while another create uses X for value B | regime B tables, sighting | PK violation path; classify with follow-up SELECT (section 4.4) |
| Lost update on read-modify-write | `SightingObserver` (F6) | (a) `SELECT ... FOR UPDATE` in `find_by_id` when called inside a write unit of work, or (b) optimistic `version` check (4.5), or (c) the single-statement upsert (4.2.3) |
| Concurrent File creates with overlapping hash sets | File (multi-key) | Transaction-level advisory locks on every digest, taken in sorted order (4.6) |
| Projection reads events out of commit order | event log | xid8 visibility rule (7.5) |
| Event emitted but state rolled back, or state committed but event lost | service flow F5 | event row in the same transaction (section 8) |

### 4.4 Mapping duplicates to the existing domain errors [REC]

- Ioc-like aggregates: `rows_affected() == 0` after `ON CONFLICT DO NOTHING` -> `AlreadyExists`.
- Sighting: keep the in-memory precedence (pair before id, `in_memory_sighting_repository.rs:43-60`):
  ```sql
  INSERT INTO kernel.sighting (...) VALUES (...) ON CONFLICT DO NOTHING;
  -- if 0 rows:
  SELECT id, (observable_id = $2 AND source_id = $3) AS same_pair
  FROM kernel.sighting
  WHERE id = $1 OR (observable_id = $2 AND source_id = $3)
  ORDER BY same_pair DESC
  LIMIT 1;
  -- same_pair = true  -> PairAlreadyExists { existing_id: id }
  -- same_pair = false -> IdAlreadyExists
  -- no row (deleted concurrently) -> retry the insert once, then Unexpected
  ```
- When a 23505 is caught instead (outside a unit of work, or by design), map by constraint name: sqlx exposes `DatabaseError::constraint()` ("If the error was caused by a conflict of a unique index, this will be the index name"; "Currently only populated by the Postgres driver", `SQLX-CORE/src/error.rs:235-243`) and `is_unique_violation()` (`SQLX-CORE/src/error.rs:260-262`), with `UNIQUE_VIOLATION = "23505"` mapped to `ErrorKind::UniqueViolation` (`SQLX-PG/src/error.rs:215-224,238-250`). Inside a transaction a 23505 aborts it, so catch it inside a savepoint: sqlx's nested `begin()` on a transaction issues `SAVEPOINT _sqlx_savepoint_{depth}` (`SQLX-CORE/src/transaction.rs:18-48,286`).

### 4.5 Optimistic concurrency [REC]

- Add a `version` value object to every mutable aggregate. Repository `update`:
  ```sql
  UPDATE kernel.sighting
  SET last_seen = $3, count = $4, updated_at = $5, version = version + 1
  WHERE id = $1 AND version = $2;
  -- rows_affected() == 0 -> SELECT 1 FROM kernel.sighting WHERE id = $1
  --   no row  -> NotFound
  --   row     -> Conflict (new concept, HTTP 409, or retried by the handler)
  ```
- The event row carries `aggregate_version` = the aggregate's version after the change, which gives consumers a per-aggregate ordering and gap check (section 7.5).
- For the hottest path (observe), option (a) in 4.3 is simplest: the Postgres repository's `find_by_id` adds `FOR UPDATE` when it runs inside a write unit of work. The row lock lasts until commit; contention is per sighting only [INF].

### 4.6 Multi-key dedup and advisory locks (File)

- Problem [INF]: a File known by MD5 only and the same File known by SHA-256 only do not collide on any unique index. A later record with both hashes collides with two different rows, which requires a merge. OpenCTI handles this by locking all "participant ids" in Redis (`lockResources(participantIds)`, `GQL/src/database/middleware.ts:3951-3956`), searching existing entities by any hash (`listEntitiesByHashes`, `middleware.ts:3963-3970`, defined at `middleware.ts:1547`), then creating, upserting or merging (`middleware.ts:4084-4114`). The lock is Redlock with `retry_delay 250`, `retry_jitter 100`, `max_ttl 60000` (`GQL/src/database/redis.ts:434-450`; `GQL/config/default.json:114-117`).
- Postgres equivalent [REC]:
  ```sql
  -- inside the unit-of-work transaction, keys sorted ascending to avoid deadlocks
  SELECT pg_advisory_xact_lock(k) FROM unnest($1::bigint[]) AS k ORDER BY k;
  SELECT id, md5, sha1, sha256, sha512 FROM kernel.file
  WHERE md5 = $2 OR sha1 = $3 OR sha256 = $4 OR sha512 = $5;   -- each arm uses its partial unique index
  -- 0 rows: insert; 1 row: add missing hashes (version check); >1 rows: merge (domain decision)
  ```
  - Keys: compute in Rust (e.g. first 8 bytes of SHA-256 over `"fennec:file:sha256:" || digest`) rather than `hashtextextended` (which is immutable, `PGPROC:1219-1221`, but is an internal function). A hash collision only causes false contention, never incorrectness [INF].
  - Avoid colliding with sqlx's own advisory locks: migrations lock `generate_lock_id(database_name)` (`SQLX-PG/src/migrate.rs:186-215,370`), the test harness locks `8318549251334697844` (`SQLX-PG/src/testing/mod.rs:136`). 64-bit derived keys make a clash practically impossible; the two-`int4` form `pg_advisory_xact_lock(classid, objid)` gives a separate key space [INF: bigint and int4-pair advisory locks are recorded with different `objsubid` in `pg_locks`].
  - Use the **transaction-scoped** form. Session-scoped locks survive commit/rollback and break under pgbouncer transaction pooling [SRC: `SQLX-PG/src/advisory_lock.rs:22-32` describes session vs transaction scope; pooling remark INF].

---

## 5. Indexing

### 5.1 B-tree

- Every UNIQUE constraint is a btree (only btree can be unique [SRC]).
- [SRC] The planner can turn `inet_col << $net` / `<<= $net` into a btree range (`key >= network_scan_first(rhs)` and `key <= network_scan_last(rhs)`) through `network_subset_support` / `match_network_subset`, but only when the right side is a **hard constant**: "Can't do anything with a non-constant or NULL comparison value" (`src/backend/utils/adt/network.c`, REL_17_STABLE, `match_network_subset`; before PG 18 it also required the btree opfamily `NETWORK_BTREE_FAM_OID`). A sqlx bind parameter is a `Param`, so after the plan cache switches to a generic plan the rewrite does not apply [INF].
  - [REC] For "hosts inside network N", compute the first and last address in Rust and query `WHERE address >= $1 AND address <= $2` on the existing `UNIQUE (address)` btree. Correct because every row is `/32` (CHECK) so inet ordering is plain address order [INF]. No extra index needed.
- [SRC] B-tree deduplication (PG 13+) shrinks non-unique indexes with repeated keys (`sighting_source_id_idx`), but "`INCLUDE` indexes can never use deduplication", nondeterministic collations, `numeric`, `jsonb`, floats and container types cannot (docs 65.1.4.3).
- [SRC] Unique indexes rely on "bottom-up index deletion" to absorb version churn (docs 65.1.4.2-65.1.4.3), which is why frequent non-HOT updates on rows with unique indexes stay manageable, as long as long-running snapshots do not block cleanup.

### 5.2 GiST / SP-GiST `inet_ops`

- [SRC] GiST `inet_ops` supports `<<`, `<<=`, `>>`, `>>=`, `=`, `<>`, `<`, `<=`, `>`, `>=`, `&&`; "For historical reasons, the `inet_ops` operator class is not the default class for types `inet` and `cidr`. To use it, mention the class name in `CREATE INDEX`" (docs 65.2 table 65.1). SP-GiST `inet_ops` supports the same operator list (docs 65.3 table 65.2).
- [SRC] Multicolumn: "Currently, only the B-tree, GiST, GIN, and BRIN index types support multiple-key-column indexes" (docs 11.3), so SP-GiST is single-column.
- **Hazard [SRC: pgsql-bugs BUG #19000, thread at hackorum.dev/topics/73234]:** with `btree_gist` installed, `CREATE INDEX ... USING gist (inet_col)` without an opclass picks **btree_gist's `gist_inet_ops`**, whose consistent function maps inet to a lossy scalar; `<<` queries then return wrong (missing) rows once the planner chooses the index. Reported on 18beta2 and reproduced on 17; Tom Lane called btree_gist's inet support "fundamentally broken" and proposed removing it. I did not confirm the fix status. [REC] Always write `USING spgist (col inet_ops)` or `USING gist (col inet_ops)` explicitly; do not install `btree_gist` unless needed; add a migration test asserting the opclass (`SELECT opcname FROM pg_index i JOIN pg_opclass o ON o.oid = i.indclass[0] ...`).
- [REC] Use SP-GiST `inet_ops` on `ipv4_network.network` / `ipv6_network.network` for "which networks contain host H" (`network >>= $1`), which works with bind parameters and generic plans. Use GiST only if you need a multicolumn index or an exclusion constraint. [VERIFY] relative build/query speed of SP-GiST vs GiST for inet is workload-dependent; benchmark with real feeds.
- Contrast [SRC]: MISP has no inet type; CIDR correlation filters `INET_ATON(value1) BETWEEN ? AND ?` plus a `value1 LIKE 'a.b.%'` prefix to hit the 255-char prefix index, with a separate path for IPv6 (`MISP/app/Model/Correlation.php:702-735`), and caches the list of all CIDRs in Redis (`Correlation.php:1044-1081`).

### 5.3 Domain and URL substring search

- [REC] Keep substring search off the write tables if possible (CQRS read side or a search engine). A trigram GIN index on hundreds of millions of URLs is large and expensive to maintain [INF], and the write path does not need it.
- If it must live in Postgres:
  ```sql
  CREATE EXTENSION IF NOT EXISTS pg_trgm;
  CREATE INDEX url_value_trgm ON kernel.url USING gin (value gin_trgm_ops) WITH (fastupdate = on);
  -- serves value LIKE '%evil%', ILIKE, ~ regex
  ```
  GIN `fastupdate` buffers inserts in a pending list (`gin_pending_list_limit`, default 4 MB [VERIFY default]) that is merged by vacuum or when full, which smooths insert cost but makes some inserts and searches slow [INF]. Patterns shorter than three characters cannot use trigrams effectively [INF].
- Suffix/subdomain search without trigrams [REC]: btree on `reverse(value)` (DDL in 3.8). For "all subdomains of example.com": `WHERE reverse(value) >= $1 AND reverse(value) < $2` with `$1 = 'moc.elpmaxe.'` and `$2 = 'moc.elpmaxe/'` (next byte after `.`) computed in Rust, plus `OR value = 'example.com'`. An explicit range works with bind parameters; a `LIKE $1` with a parameter pattern cannot use the prefix optimisation in a generic plan [INF, same Const restriction as LIKE prefix extraction]. In `COLLATE "C"`, a plain btree supports prefix ranges without `text_pattern_ops` [INF].

### 5.4 BRIN

- [REC] BRIN fits **append-only, physically time-ordered** columns: `event_store.domain_event.recorded_at` (one partition per month, rows inserted in time order). Tiny (a few pages per GB). `pages_per_range` default 128 [VERIFY]; `minmax_multi_ops` (PG 14) tolerates out-of-order outliers [VERIFY version].
- Poor fit: `sighting.last_seen` (updated in place, rows move on non-HOT updates), random-key observable tables. Use btree there if needed.
- [SRC] PG 16: HOT updates are allowed when only BRIN-indexed columns change, so a BRIN on a mutable timestamp does not block HOT [SRC: 16.0 release notes].

### 5.5 Partial and covering indexes

- Partial: per-digest unique indexes on File (`WHERE sha256 IS NOT NULL`, section 3.8); `WHERE revoked = false` style indexes if a soft-delete flag is ever added. ON CONFLICT against a partial index needs the predicate in the conflict target [SRC, 4.2].
- Covering: `CREATE UNIQUE INDEX ... (value_sha256) INCLUDE (id)` enables index-only lookups of the id by digest (if the visibility map is current, i.e. vacuum keeps up on insert-mostly tables via `autovacuum_vacuum_insert_scale_factor`, PG 13+ [VERIFY]). Cost: INCLUDE disables deduplication [SRC] and widens the index. [REC] Not needed in regime A (the id is derived from the value; no lookup required).

### 5.6 Write amplification budget [INF]

Per new observable in regime A, unpartitioned: 1 heap tuple + PK btree + natural-key btree (+ optional SP-GiST) + the event row (heap + 3-4 btrees, section 7). Rules:
- Each extra index on a write table costs one index insert per row and per non-HOT update, plus WAL and, after each checkpoint, a full-page image of every touched page. Random-key indexes touch a different leaf for almost every insert, so their working set is the whole index: keep write-table indexes small enough to stay cache-resident or ingestion throughput collapses to random I/O.
- HOT updates need: no indexed column changed and free space on the same page. Hence `fillfactor = 85` on `sighting`, and no index on `last_seen`/`count` on the write table.
- Tune autovacuum per table: the default `autovacuum_vacuum_scale_factor = 0.2` waits for 20 % dead rows (20 M on a 100 M table) [VERIFY default]; set `0.01`-`0.02` or a fixed threshold on `sighting`.
- Server: larger `max_wal_size`/`checkpoint_timeout` (fewer full-page images), `wal_compression = zstd` (PG 15+) [VERIFY], `default_toast_compression = lz4` for jsonb payloads.
- Coalesce: a feed refresh that re-observes 10 M sightings should not write 10 M updates and 10 M events if one event per sighting per batch with a count delta is acceptable to the domain [REC, open question Q5].

### 5.7 Rough sizing [INF, computed]

Heap row of `ipv4_address` (uuid, 2 x timestamptz, bigint, inet): 24-byte header + 16 + 8 + 8 + 8 + 7 = 71 -> 72 bytes + 4-byte line pointer = 76 bytes, about 107 rows per 8 kB page: **100 M rows = about 7 GiB heap**, PK uuid btree 3-4 GiB, inet btree 2-3 GiB (70-90 % fill). The event log dominates: 1 B events at 350-600 bytes each (jsonb payload) plus about 120 bytes of index entries per event = **0.5-0.7 TB per billion events**, which is why it is partitioned by time from day one (section 6).

---

## 6. Partitioning

### 6.1 Rules from the docs [SRC]

- "These benefits will normally be worthwhile only when a table would otherwise be very large ... a rule of thumb is that the size of the table should exceed the physical memory of the database server." (docs 5.12.1)
- "To create a unique or primary key constraint on a partitioned table, the partition keys must not include any expressions or function calls and the constraint's columns must include all of the partition key columns." (docs 5.12.2.3). Same for exclusion constraints, which "must compare those columns for equality".
- "The query planner is generally able to handle partition hierarchies with up to a few thousand partitions fairly well, provided that typical queries allow the query planner to prune all but a small number of partitions." (docs 5.12.6)
- Identity columns on partitioned tables need PG 17+ [SRC: 17.0 notes].

### 6.2 Decision matrix [REC]

| Table | Partition? | Method and key | Unique constraints kept | Reason |
|---|---|---|---|---|
| Value-identified observables (regime A) | Not at first. Hash-partition a kind when it approaches RAM size (likely `url`, `domain_name`, `file`, `ipv4_address` first) | `PARTITION BY HASH (id)`, generous modulus (32 or 64) chosen up front | `PRIMARY KEY (id)` only; the natural-key UNIQUE must be dropped (it cannot include the partition key unless partitioned by it) and uniqueness then rests on the deterministic id | No time locality (an observable created years ago is re-sighted today), so range partitioning brings nothing; hash partitioning gives per-partition vacuum/REINDEX CONCURRENTLY/freeze and keeps every lookup pruned (value -> id in Rust -> one partition). |
| Alternative for regime A | | `PARTITION BY HASH (address)` (natural key, not an expression) | `UNIQUE (address)` allowed; PK on `id` then **not** allowed globally | Only if ids are not derivable; id lookups would then probe all partitions. |
| Multi-key (File) | Avoid as long as possible | If forced: hash by `id` and move digest uniqueness to per-algorithm lookup tables `file_sha256 (sha256 bytea PRIMARY KEY, file_id uuid)` (each hash-partitioned by its own key) | per-lookup-table PK | Partitioning by one key forbids global uniqueness on the others. |
| `sighting` | When large: hash by `observable_id` | `UNIQUE (observable_id, source_id)` survives (contains the key); `PRIMARY KEY (id)` does not | Same trade-off: make sighting ids deterministic too (`uuidv5(observable_id, source_id)`) so `PRIMARY KEY (observable_id, id)` or id lookups prune [REC, open question Q1]. |
| `event_store.domain_event` | **Yes, from day one** | `PARTITION BY RANGE (recorded_at)`, monthly | `PRIMARY KEY (position, recorded_at)`; `event_id` and `(aggregate_id, aggregate_version)` indexed but **not** globally unique | Unbounded append-only growth; the hot partition's random indexes (`aggregate_id`, `event_id`) stay small and cached; retention is `DETACH`/`DROP PARTITION` instead of DELETE; old partitions get frozen once. |

- Hash partition DDL:
  ```sql
  CREATE TABLE kernel.url (... same columns ..., CONSTRAINT url_pkey PRIMARY KEY (id)) PARTITION BY HASH (id);
  CREATE TABLE kernel.url_p00 PARTITION OF kernel.url FOR VALUES WITH (MODULUS 32, REMAINDER 0);
  -- ... p01 .. p31
  ```
  Increasing the modulus later means moving data (a remainder of modulus 32 can be replaced by two partitions of modulus 64 only by detaching and reloading) [INF]; pick the modulus for the 3-5 year size.
- Converting an existing table later: a plain table can be attached as one partition of a new partitioned parent if a matching CHECK constraint exists (avoids the validation scan) [INF]. For the event log, start partitioned to avoid that work.
- Partition maintenance: create future monthly partitions ahead of time (pg_partman, or a small Rust maintenance job running as `fennec_owner`). [REC] No DEFAULT partition on the event log: a row outside every range should fail loudly, and a non-empty default partition makes later `ATTACH` scan it [INF].

---

## 7. The domain event log (event store / outbox)

Fennec persists **state** (aggregate tables are the source of truth) and emits domain events; it is not event-sourced. So this table is a durable, ordered journal used as the transactional outbox, the integration feed and the audit trail. Per-aggregate replay is for projections and audit, not for rebuilding aggregate state [INF from F5 and the repository design].

### 7.1 DDL [REC]

```sql
-- migrations/0003_event_store.sql
CREATE SCHEMA IF NOT EXISTS event_store;

CREATE TABLE event_store.domain_event (
    position          bigint      GENERATED ALWAYS AS IDENTITY,   -- PG 17+ on partitioned tables
    transaction_id    xid8        NOT NULL DEFAULT pg_current_xact_id(),
    aggregate_version bigint      NOT NULL,
    occurred_on       timestamptz NOT NULL,                       -- DomainEvent::occurred_on
    recorded_at       timestamptz NOT NULL DEFAULT now(),         -- transaction start time; partition key
    event_id          uuid        NOT NULL,                       -- UUIDv7 (time-ordered)
    aggregate_id      uuid        NOT NULL,
    event_name        text COLLATE "C" NOT NULL,                  -- 'fennec.kernel.sighting.observed'
    aggregate_type    text COLLATE "C" NOT NULL,                  -- 'sighting'
    payload           jsonb       NOT NULL,
    metadata          jsonb       NOT NULL DEFAULT '{}'::jsonb,   -- schema_version, correlation_id, causation_id, actor
    CONSTRAINT domain_event_pkey PRIMARY KEY (position, recorded_at),
    CONSTRAINT domain_event_name_shape CHECK (event_name ~ '^fennec\.[a-z_]+\.[a-z_]+\.[a-z_]+$'),
    CONSTRAINT domain_event_version_pos CHECK (aggregate_version >= 1)
) PARTITION BY RANGE (recorded_at);

CREATE TABLE event_store.domain_event_2026_11 PARTITION OF event_store.domain_event
    FOR VALUES FROM ('2026-11-01 00:00:00+00') TO ('2026-12-01 00:00:00+00');

-- global, gap-safe replay (7.5)
CREATE INDEX domain_event_tx_position_idx ON event_store.domain_event (transaction_id, position);
-- per-aggregate replay and gap repair
CREATE INDEX domain_event_aggregate_idx   ON event_store.domain_event (aggregate_id, aggregate_version);
-- idempotency lookups by event id (unique per partition at most; UUIDv7 makes collisions moot)
CREATE INDEX domain_event_event_id_idx    ON event_store.domain_event (event_id);
-- optional: time-range scans for audit
CREATE INDEX domain_event_recorded_brin   ON event_store.domain_event USING brin (recorded_at);
```

Column notes:
- `aggregate_id uuid`: all kernel aggregates use UUIDs (F13). The config app has its own database; give it the same table with `aggregate_id text COLLATE "C"` or derive a UUIDv5 from the key [REC].
- `event_id`: switch `DomainEventBase::new` to UUIDv7 (`uuid` crate feature `v7`) so the `event_id` index appends at the right edge (F3) [REC]. PG 18 also has `uuidv7()` [SRC].
- `event_name` as text costs about 35 bytes per row; a `smallint` lookup or enum saves that at the price of a migration per new event [INF]. Keep text until volume proves otherwise.
- `payload jsonb`: requires extending `DomainEvent` (F2), e.g. `fn payload(&self) -> serde_json::Value` or a primitives map, plus `fn aggregate_type(&self) -> &'static str` and `fn aggregate_version(&self) -> u64` [REC]. Put `schema_version` in `metadata` for upcasting later.
- `recorded_at DEFAULT now()`: all events of one transaction share it and land in the same partition [INF].
- [SRC] sqlx has no Rust type for `xid8` (no match in `SQLX-PG/src`), so select it as `transaction_id::text` and parse to `u64`, and bind checkpoints as `$1::text::xid8`.

### 7.2 Indexes vs queries

| Query | Index |
|---|---|
| Consumer polling (gap-safe) | `(transaction_id, position)` |
| Per-aggregate history | `(aggregate_id, aggregate_version)` (a `hash` index on `aggregate_id` would be smaller but equality-only and then needs a sort) [INF] |
| Lookup by `event_id` (dedupe, support) | `(event_id)` |
| Audit by time | BRIN on `recorded_at` + partition pruning |

### 7.3 Append-only enforcement [REC]

```sql
REVOKE ALL ON event_store.domain_event FROM PUBLIC;
GRANT SELECT, INSERT ON event_store.domain_event TO fennec_app;
GRANT USAGE ON SCHEMA event_store TO fennec_app;

CREATE FUNCTION event_store.reject_mutation() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'event_store.domain_event is append-only (% rejected)', TG_OP
        USING ERRCODE = 'insufficient_privilege';
END $$;

CREATE TRIGGER domain_event_no_update_delete
    BEFORE UPDATE OR DELETE ON event_store.domain_event
    FOR EACH ROW EXECUTE FUNCTION event_store.reject_mutation();
CREATE TRIGGER domain_event_no_truncate
    BEFORE TRUNCATE ON event_store.domain_event
    FOR EACH STATEMENT EXECUTE FUNCTION event_store.reject_mutation();
```
- Privileges are the real guarantee; triggers protect against mistakes by the owner. [INF] Row-level triggers on the parent are cloned to partitions; statement-level TRUNCATE triggers are not, so a direct `TRUNCATE` of a partition by its owner is not blocked by the parent trigger. The app role has no privileges on partitions (it only writes through the parent), so it cannot reach them directly.
- Retention uses `ALTER TABLE ... DETACH PARTITION ... CONCURRENTLY` and `DROP TABLE` as `fennec_owner`, which are DDL and unaffected by these triggers [INF].
- This requires migrations to run as `fennec_owner` in a separate step (sqlx-cli or a `migrate` subcommand), not with the app's DML credentials at startup [REC].

### 7.4 Consumer checkpoints [REC]

```sql
CREATE TABLE event_store.consumer_checkpoint (
    consumer_name       text COLLATE "C" PRIMARY KEY,
    last_transaction_id xid8        NOT NULL DEFAULT '0',
    last_position       bigint      NOT NULL DEFAULT 0,
    updated_at          timestamptz NOT NULL DEFAULT now()
);
GRANT SELECT, INSERT, UPDATE ON event_store.consumer_checkpoint TO fennec_app;
```
- A projection in the same database updates its read model **and** its checkpoint in one transaction, so each event takes effect exactly once [INF].
- One active instance per consumer: `SELECT ... FROM event_store.consumer_checkpoint WHERE consumer_name = $1 FOR UPDATE NOWAIT` inside the processing transaction (a second instance fails fast), or `pg_try_advisory_lock` on a dedicated session connection for leader election [REC].
- Parallelism with per-aggregate order: shard by `hashtextextended(aggregate_id::text, 0) % N` (or a Rust hash) with one checkpoint row per shard [REC].
- External consumers (webhooks, a message broker) are fed by a relay that reads like any consumer and publishes at-least-once; downstream deduplicates on `event_id` [REC].

### 7.5 Ordering and gaps under concurrent transactions

- [SRC] The problem: positions come from a sequence at insert time, not at commit. If T1 takes position 10 and T2 takes 11 and T2 commits first, a reader doing `WHERE position > last ORDER BY position` reads 11, checkpoints 11, and never sees 10. Described by Ecotone ("Gap Detection and Consistency" docs), Axon forum threads, and Oskar Dudycz ("Ordering in Postgres outbox", event-driven.io). Rolled-back transactions also leave permanent holes (harmless if the reader does not wait for them).
- **xid8 visibility solution [SRC: event-driven.io/en/ordering_in_postgres_outbox]:** store the writing transaction's id in each row and only read rows whose transaction is older than every in-flight one. The article's query (assembled from its fragments):
  ```sql
  SELECT position, message_id, message_type, data
  FROM outbox
  WHERE ((transaction_id = last_processed_transaction_id AND position > last_processed_position)
         OR (transaction_id > last_processed_transaction_id))
    AND transaction_id < pg_snapshot_xmin(pg_current_snapshot())
  ORDER BY transaction_id ASC, position ASC
  LIMIT 100;
  ```
  Its caveats: delayed reads under long transactions, and "our ordering guarantee respects when the transaction was started, not when the message was appended". The eugene-khyst `postgresql-event-sourcing` project uses the same `pg_snapshot_xmin(pg_current_snapshot())` gate (per search result; not verified in its source).
- Fennec version [REC]:
  ```sql
  SELECT position, transaction_id::text AS transaction_id, event_id, event_name, aggregate_type,
         aggregate_id, aggregate_version, payload, metadata, occurred_on, recorded_at
  FROM event_store.domain_event
  WHERE (transaction_id, position) > ($1::text::xid8, $2::bigint)
    AND transaction_id < pg_snapshot_xmin(pg_current_snapshot())
    AND recorded_at >= $3              -- pruning hint: last checkpointed recorded_at minus max transaction lifetime
  ORDER BY transaction_id, position
  LIMIT $4;
  ```
  - Why it is gap-free [INF]: every transaction id below `xmin` has finished (committed or aborted) and new transactions always get larger ids, so once a reader has passed a `(transaction_id, position)` pair nothing can appear before it. Ordering by `position` alone with the xmin gate is **not** gap-free (a transaction with a larger xid can hold a smaller position).
  - Pruning hint: `recorded_at` is the transaction start time; a transaction can obtain its xid later than it started, so subtract the maximum transaction lifetime (enforce it with `transaction_timeout` (PG 17+ [VERIFY]) and `idle_in_transaction_session_timeout` on `fennec_app`).
  - Cost [INF]: any long write transaction anywhere in the cluster delays all consumers (latency, not loss). Read-only transactions do not have an xid and do not hold `xmin` back in this sense. Keep consumers on the primary.
  - Per-aggregate order caveat [INF]: two transactions on the same aggregate are serialised by the row lock, but in a narrow window the second can hold the smaller xid, so a reader ordered by `(transaction_id, position)` can deliver version 6 before version 5. Projections must store the last applied `aggregate_version` per aggregate, skip `<=` versions as duplicates, and on a jump fetch the missing versions by `(aggregate_id, aggregate_version)` (already committed and visible) before applying.
- Alternatives [INF]:

  | Approach | Gap-free | Throughput | Complexity |
  |---|---|---|---|
  | xid8 + `pg_snapshot_xmin` (recommended) | yes | no write-side cost; one extra 8-byte column and index | low |
  | Global `pg_advisory_xact_lock(const)` before inserting events | yes (commit order = position order) | serialises all writing transactions until commit (fsync included) | low, but caps write throughput |
  | Gap detection with timeout (high-water mark, Marten-style; per Ecotone docs as well) | eventually; a slow commit past the timeout is skipped | no write cost | medium; correctness depends on a timeout |
  | Logical decoding / CDC (pgoutput, Debezium) | yes, commit order | no write cost | high operational cost |
  | Reader takes a lock that waits out in-flight inserts (eventsourced/pg PR #26 "WithCommittedReads", per search result) | yes | readers block writers briefly | medium |

### 7.6 LISTEN/NOTIFY vs polling

- [SRC] NOTIFY docs: payload "must be shorter than 8000 bytes" by default; a queue of 8 GB, and "If this queue becomes full, transactions calling NOTIFY will fail at commit"; notifications are delivered only after commit and "messages from different transactions are delivered in the order in which the transactions committed"; identical payloads in one transaction are folded.
- [SRC] sqlx `PgListener` auto-reconnects, but "As notifications are transient, any received while the connection was lost, will not be returned" (`SQLX-PG/src/listener.rs:202-207`; `eager_reconnect` at `listener.rs:102-112`).
- [SRC, third party] Recall.ai ("Postgres LISTEN/NOTIFY does not scale", July 2025) traced commit stalls to a global lock taken at commit by transactions that issued NOTIFY; their page later says a core change fixed the bottleneck, and a pgsql-hackers reply attributes the cost mainly to waking many listeners. [VERIFY] which PostgreSQL major version contains the fix.
- [REC] Polling is the source of truth; NOTIFY is at most a wake-up hint:
  - Consumers poll with adaptive backoff (for example 20 ms when the last batch was full, up to 500 ms when empty), the same shape as OpenCTI's stream reader (`XREAD COUNT 100 BLOCK 5000`, then wait `bufferTime ?? 50` ms unless the batch was full: `GQL/src/database/redis-stream.ts:145-146,167-190`).
  - Writers do not NOTIFY on the high-volume path. If low latency is needed, one statement-level `AFTER INSERT` trigger with `pg_notify('domain_event', '')` (empty payload, folded per transaction) behind a configuration switch, measured under load.
  - `PgListener` needs a dedicated session connection (not through pgbouncer transaction pooling) [INF].

### 7.7 Retention [REC]

- Because aggregate tables hold the state, the log can be trimmed: drop a monthly partition when (a) every consumer checkpoint is past the partition's maximum `(transaction_id, position)` and (b) the audit retention window has passed. Archive first (`COPY (SELECT * FROM event_store.domain_event_2025_10) TO STDOUT (FORMAT binary)` to object storage).
- New projections bootstrap from the state tables plus the retained tail of the log, not from event zero.
- Comparison [SRC]: OpenCTI's live stream is a Redis stream trimmed with `XADD ... MAXLEN ~ <redis:trimming>` (`GQL/src/database/redis-stream.ts:27,84-88`), default `"trimming": 2000000` (`GQL/config/default.json:419`); notification and activity streams default to 50000 (`redis-stream.ts:291-299,353-356`). Events larger than `redis:max_event_length` are offloaded to file storage (`redis-stream.ts:64-83`).

---

## 8. Transactional consistency between the state write and the event append

### 8.1 What happens today if the adapters are swapped naively [INF]

With a Postgres repository and an event bus that writes to `event_store` in its own statement or transaction, the current service shape (F5) produces:

| Failure point | Result |
|---|---|
| Process dies after `repository.save` committed, before `publish` | State without event: lost event, projections and integrations never learn about it |
| `publish` fails after the save committed | Client gets 500 although the observable exists; a retry gets 409 AlreadyExists and still no event |
| Event written first, then the state write fails | Ghost event describing something that does not exist |
| In-process subscriber fails (F4) | Stops dispatch; the state is already committed |

Also, `EventBus::publish` is synchronous (F1), so a Postgres-backed bus cannot await an insert inside it without blocking the executor.

### 8.2 Options

| Option | Atomic | Changes to ports and services | Notes |
|---|---|---|---|
| **1. Unit of work bound to the command, buffering event bus** (recommended) | yes | None in services or repository traits. New infrastructure: a command-bus decorator, a task-local unit of work, Postgres repositories that use it, an `EventBus` impl that buffers. `DomainEvent` gains a payload method. | Services keep "persist, build event with factory, publish". |
| 2. Repository receives the events (`save(&self, x, events)`) and writes both in its own transaction | yes | Every repository trait, in-memory repo and mock changes; services build the event before persisting | Explicit and simple; repository becomes event-aware. |
| 3. Aggregate records its events (`record`/`pull_domain_events`), repository persists aggregate + pending events | yes | Aggregates become mutable event recorders; factories are called from aggregate methods | Textbook DDD (one aggregate per transaction); the most invasive change for Fennec. |
| 4. Separate statement after the write, no shared transaction | no | none | Dual write; only acceptable for best-effort notifications. This is what OpenCTI does (ES index then Redis XADD, `GQL/src/database/middleware.ts:4205-4216`). |

### 8.3 Recommended design for Fennec (option 1) [REC]

Shape:
1. `TransactionalCommandBus` wraps the existing `InMemoryCommandBus` (decorator implementing `CommandBus`).
2. For each `dispatch`, it begins a transaction, stores a `PgUnitOfWork` in a `tokio::task_local!`, and awaits the inner dispatch inside that scope. This works because handler futures are awaited in the same task (F9), and Actix runs handlers on Tokio [INF].
3. Postgres repositories execute writes (and `FOR UPDATE` reads) on the unit of work's transaction.
4. `OutboxEventBus::publish` (synchronous) only appends the boxed events to the unit of work's buffer. It returns an error if called outside a unit of work, so a missing decorator fails loudly.
5. After the handler returns, the decorator: if the dispatch errored, any statement failed, or no event was buffered, roll back; otherwise insert the buffered events (one `INSERT ... SELECT FROM UNNEST(...)`), commit, then hand the events to the in-process `InMemoryEventBus` (post-commit, best effort; durable consumers read the log).

"Commit only if at least one event was published" relies on a Fennec invariant: every successful write use case publishes an event and failed ones publish none (AGENTS.md "Nothing is published if the write fails"; F5). Business errors travel inside the response envelope (F8), which the decorator cannot inspect through `Box<dyn Any>`. If that invariant is ever broken, add an explicit `uow.mark_successful()` or a response trait.

Sketch (infrastructure crate, e.g. `libs/shared/persistence-postgres`; names follow the house style):

```rust
// pg_unit_of_work.rs
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};

use shared_domain_events::domain::domain_event::DomainEvent;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

tokio::task_local! {
    pub static CURRENT_UOW: Arc<PgUnitOfWork>;
}

pub struct PgUnitOfWork {
    tx: Mutex<Option<Transaction<'static, Postgres>>>,
    pending: StdMutex<Vec<Box<dyn DomainEvent>>>,
    failed: AtomicBool,
}

impl PgUnitOfWork {
    pub async fn begin(pool: &PgPool) -> Result<Arc<Self>, sqlx::Error> {
        Ok(Arc::new(Self {
            tx: Mutex::new(Some(pool.begin().await?)),
            pending: StdMutex::new(Vec::new()),
            failed: AtomicBool::new(false),
        }))
    }
    pub fn current() -> Option<Arc<Self>> { CURRENT_UOW.try_with(Arc::clone).ok() }
    pub fn mark_failed(&self) { self.failed.store(true, Ordering::SeqCst); }
    pub fn buffer(&self, events: Vec<Box<dyn DomainEvent>>) { self.pending.lock().unwrap().extend(events); }
    // with_tx(..), take_pending(), take_tx() omitted
}
```

```rust
// sqlx_postgres_ipv4_address_repository.rs (save only)
async fn save(&self, a: &Ipv4Address) -> Result<(), Ipv4AddressRepositoryError> {
    let uow = PgUnitOfWork::current()
        .ok_or_else(|| Ipv4AddressRepositoryError::Unexpected("no unit of work".into()))?;
    let mut guard = uow.tx.lock().await;
    let tx = guard.as_mut()
        .ok_or_else(|| Ipv4AddressRepositoryError::Unexpected("unit of work closed".into()))?;
    let res = sqlx::query(INSERT_IPV4_ADDRESS)        // const &str: ON CONFLICT DO NOTHING
        .bind(a.id().value())
        .bind(OffsetDateTime::from(a.created_at().value()))
        .bind(OffsetDateTime::from(a.updated_at().value()))
        .bind(i64::try_from(a.version().value()).map_err(|e| Unexpected(e.to_string()))?)
        .bind(IpNetwork::from(IpAddr::V4(*a.address().value())))
        .execute(&mut **tx)                            // &mut PgConnection via Transaction: DerefMut
        .await;
    match res {
        Ok(r) if r.rows_affected() == 1 => Ok(()),
        Ok(_) => Err(Ipv4AddressRepositoryError::AlreadyExists),
        Err(e) => { uow.mark_failed(); Err(map_sqlx_error(e)) }
    }
}
```

```rust
// transactional_command_bus.rs (dispatch only)
async fn dispatch(&self, command: Box<dyn Command>)
    -> Result<Box<dyn Any + Send + Sync>, CommandBusError> {
    let uow = PgUnitOfWork::begin(&self.pool).await.map_err(handler_err)?;
    let response = CURRENT_UOW.scope(Arc::clone(&uow), self.inner.dispatch(command)).await;
    let events = uow.take_pending();
    let mut tx = uow.take_tx().await.ok_or_else(|| handler_err("unit of work closed"))?;
    if response.is_err() || uow.failed() || events.is_empty() {
        let _ = tx.rollback().await;
        return response;
    }
    append_events(&mut tx, &events).await.map_err(handler_err)?;  // INSERT ... SELECT FROM UNNEST
    tx.commit().await.map_err(handler_err)?;
    let _ = self.after_commit_bus.publish(events);                 // in-process, best effort
    response
}
```

Details and pitfalls:
- [INF] Do not rely on `commit()` to report an earlier failure: PostgreSQL answers `COMMIT` in an aborted transaction by rolling back (command tag `ROLLBACK`) rather than necessarily raising an error [VERIFY exact behaviour of sqlx `commit()` here]. That is why repositories call `mark_failed()` on any sqlx error.
- [INF] One transaction holds one pooled connection for the whole command; keep commands free of network calls and set `idle_in_transaction_session_timeout`.
- Queries (CQRS read side) do not use the unit of work; query repositories use the pool directly (snapshot per statement).
- In-memory wiring stays exactly as today (in-memory repos, `InMemoryEventBus`, no decorator), so the existing unit and e2e suites keep running without a database.
- Event construction: `append_events` needs each event's payload, aggregate type and version, hence the `DomainEvent` extension (F2). Serialisation happens in infrastructure; the domain only exposes data.
- Services could build the event before persisting (the factories only read the aggregate), which removes the "failure after write" window entirely; optional, since with option 1 a factory error after the write simply causes a rollback (no event buffered).

### 8.4 Option 2 or 3 if the team prefers explicit over ambient [REC]

- Option 2 signature: `async fn save(&self, ioc: &Ioc, events: &[&dyn DomainEvent]) -> Result<(), IocRepositoryError>`; the Postgres repo does `BEGIN; INSERT state; INSERT events; COMMIT`. The in-memory repo ignores the events or pushes them to an in-memory log (useful in tests).
- Option 3: aggregates gain `fn pull_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>>`; repositories persist them in the same transaction; services then call `event_bus.publish(aggregate.pull_domain_events())` for in-process post-commit notification.
- Both keep the outbox in the repository and need no task-local or decorator, at the cost of changing every repository trait, mock and service.

---

## 9. sqlx specifics

### 9.1 Version and Cargo features [REC]

- Pin `sqlx = "0.8.6"` if the requirement is 0.8.x; note that 0.9.0 exists (2026-05-06) with breaking changes to dynamic query strings (`SqlSafeStr`/`AssertSqlSafe`) and removed combined runtime-TLS features [SRC: CHANGELOG]. Static `&'static str` SQL constants are unaffected by `SqlSafeStr` [INF].
- Features: `runtime-tokio`, `tls-rustls` (0.8 naming; check exact feature names for the chosen version), `postgres`, `macros` (only if using macros or `#[sqlx::test]`), `migrate`, `uuid`, `time` or `chrono`, `json`, `ipnetwork` or `ipnet` (`ipnet` needs >= 0.8.4), `mac_address`.
- Put sqlx only in infrastructure crates (adapters) and the app crate; domain crates stay driver-free (F16; `FENNEC/docs/architecture/overview.md:109` "Nothing in the domain imports from `actix-web`, `sqlx`").

### 9.2 Runtime queries vs compile-time macros

| | `sqlx::query(...).bind(..)` / `query_as::<_, Row>` | `query!` / `query_as!` |
|---|---|---|
| Build needs a database | no | yes, or `SQLX_OFFLINE=true` with a committed `.sqlx/` directory (`cargo sqlx prepare --workspace`); the macro looks in `SQLX_OFFLINE_DIR`, then the crate's `.sqlx`, then the workspace `.sqlx` (`sqlx-macros-core-0.9.0/src/query/mod.rs:97-112`) |
| Type safety | runtime only; needs integration tests | compile-time column and parameter types |
| CI impact (stable/beta/nightly matrix, F15) | none | every job must have the offline cache or a DB; add `cargo sqlx prepare --check --workspace` against a live DB to catch drift |
| Custom types (inet, enums, xid8) | bind/decode Rust types directly; cast in SQL | type overrides (`as "col: Type"`) |

[REC] Runtime queries with `#[derive(sqlx::FromRow)]` row structs living in the adapter, converted to aggregates through value-object constructors (F16), plus an integration suite against Postgres. SQL in `const` strings next to the repository. Revisit macros once the schema settles.

### 9.3 Migrations

- `sqlx::migrate!("path")` embeds migrations at compile time; the path is relative to `CARGO_MANIFEST_DIR`. [SRC] Cargo does not notice new migration files on its own; add a `build.rs` with `println!("cargo:rerun-if-changed=migrations");` (`sqlx migrate build-script` generates it) or use the nightly `--cfg sqlx_macros_unstable` path (`SQLX/src/macros/mod.rs:800-829`).
- [SRC] A migration file starting with `-- no-transaction` runs outside a transaction (`SQLX-CORE/src/migrate/source.rs:234`), needed for `CREATE INDEX CONCURRENTLY`. Postgres migrations take `pg_advisory_lock(generate_lock_id(database_name))` so concurrent instances serialise (`SQLX-PG/src/migrate.rs:186-215,370`).
- [SRC] sqlx keeps one `_sqlx_migrations` table per database; in 0.9 `sqlx.toml` can rename it (`table-name = "foo._sqlx_migrations"`) and create schemas (`SQLX-CORE/src/config/migrate.rs:25-58`). This option does not exist in 0.8 [INF].
- [REC] One migrations directory per **database/app** (`apps/cti_api/migrations`), with files grouped by context in their names (`0002_kernel_observables.sql`, `0003_event_store.sql`). Multiple migrators sharing one `_sqlx_migrations` table in 0.8 would see each other's versions as unknown [INF: `Migrator::set_ignore_missing` exists, `SQLX-CORE/src/migrate/migrator.rs:130`, but it hides real drift]. Expose `pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");` from the app's lib so tests can reuse it.
- [REC] Run migrations as `fennec_owner` in a deploy step (`cti-api migrate` subcommand or sqlx-cli), not at every app start with DML credentials (section 7.3).

### 9.4 PgPool sizing and options

- [SRC] Defaults (0.9.0 `PoolOptions::new`, same values I recall for 0.8 [VERIFY]): `max_connections: 10`, `min_connections: 0`, `acquire_timeout: 30s`, `idle_timeout: 10 min`, `max_lifetime: 30 min`, `test_before_acquire: true`, `acquire_slow_threshold: 2s` (`SQLX-CORE/src/pool/options.rs:143-163`). The doc comment: "Production applications will likely want to at least modify `max_connections`".
- [REC]
  - Create **one** `PgPool` in `main` before `HttpServer::new` and pass it into `build_state(pool)` (or keep `build_state()` for in-memory and add `build_postgres_state(pool)`). It is shared by all workers, matching F10. `PgPoolOptions::connect_lazy(url)` is synchronous (`SQLX-CORE/src/pool/options.rs:571`) if a sync constructor is preferred.
  - Total connections across all app instances, workers and consumers should stay near the database's useful concurrency. A common starting point is `(2 x DB cores) + effective spindles` (PostgreSQL wiki "Number Of Database Connections" / HikariCP sizing guide) [VERIFY wording], far below `max_connections` (default 100).
  - Separate pools: API commands, projection workers / outbox relay, and a dedicated connection for `PgListener`.
  - `acquire_timeout` 2-5 s, mapping `sqlx::Error::PoolTimedOut` to a 503 rather than 500 [REC].
  - Session settings via `PgConnectOptions::options([("statement_timeout", "5s"), ("idle_in_transaction_session_timeout", "10s")])` (`SQLX-PG/src/options/mod.rs:436`) and `application_name` (`options/mod.rs:366`). [INF] pgbouncer rejects the `options` startup parameter unless configured to ignore it; set these per role (`ALTER ROLE fennec_app SET ...`) instead when behind pgbouncer.
  - pgbouncer in transaction mode breaks session advisory locks and LISTEN; sqlx's named prepared statements (`statement_cache_capacity`, `options/mod.rs:352`) need pgbouncer >= 1.21 with `max_prepared_statements` [VERIFY version].

### 9.5 Error mapping [REC]

```rust
fn map_sqlx_error(e: sqlx::Error) -> SightingRepositoryError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => match db.constraint() {
            Some("sighting_pkey") => SightingRepositoryError::IdAlreadyExists,
            // pair conflicts are normally resolved with ON CONFLICT + SELECT (section 4.4)
            _ => SightingRepositoryError::Unexpected(e.to_string()),
        },
        sqlx::Error::Database(db) if db.code().as_deref() == Some("40001")   // serialization_failure
            || db.code().as_deref() == Some("40P01") =>                       // deadlock_detected
            SightingRepositoryError::Unexpected(format!("retryable: {e}")),
        sqlx::Error::RowNotFound => SightingRepositoryError::NotFound,
        _ => SightingRepositoryError::Unexpected(e.to_string()),
    }
}
```
- 23505 -> `AlreadyExists`/`IdAlreadyExists`; 23514 (check) and 23502 (not null) mean a value-object invariant and a DB constraint disagree, which is a bug: `Unexpected` and log at `error`. `ErrorKind` only classifies constraint violations (`SQLX-CORE/src/error.rs:197-209`), so retryable codes need `code()`.
- [REC] Consider a `Conflict`/`Retryable` concept so the handler can retry optimistic-lock and serialization failures instead of returning 500.

### 9.6 Testing strategies

| Strategy | Isolation | Speed | Fennec fit |
|---|---|---|---|
| Keep in-memory unit and e2e suites | full | fastest | unchanged; still the default `cargo test` |
| Repository **contract tests** run against both in-memory and Postgres implementations | per test | medium | [REC] guarantees the in-memory repo honours the database contract (`FENNEC/.knowledge/development/ddd/infrastructure.md:14-18` asks for exactly that) |
| `#[sqlx::test]` | new database per test | medium | [SRC] "For every annotated function, a new test database is created"; needs `DATABASE_URL` with a superuser-ish role; pools of running tests "share a single connection limit"; failed tests leave their DB for debugging; old test DBs are deleted on the next run; migrations are applied automatically from `migrations/` next to `CARGO_MANIFEST_DIR`, or from `#[sqlx::test(migrator = "cti_api::MIGRATOR")]`, or disabled with `migrations = false`; fixtures via `fixtures(...)` (`SQLX/src/macros/test.md:27-60,88-120,152-159,179-228`). The harness creates databases with `create database "<name>"` and tracks them in `_sqlx_test.databases` (`SQLX-PG/src/testing/mod.rs:93-192`, master pool `max_connections(20)` at 101, per-test pool `max_connections(5)` at 176). |
| Template database (`CREATE DATABASE t TEMPLATE fennec_template`) | per test | fast once migrations grow | helper for `#[actix_web::test]` if `#[sqlx::test]` does not fit |
| Per-test schema (`SET search_path`) | per test | fast | migrations must not hard-code schema names; conflicts with schema-per-context design |
| testcontainers (`testcontainers-modules`, feature `postgres`) | per suite or per test | slow start (seconds) | needs Docker; useful locally when no Postgres runs; CI service containers are simpler |

Notes:
- [SRC] `#[sqlx::test]` runs on Tokio ("behaves identically to `#[tokio::test]`", `SQLX/src/macros/test.md:6`) and does not accept `#[tokio::test]` arguments. [VERIFY] that `actix_web::test::init_service`/`call_service` work under it (they are plain futures and should; otherwise use the template-database helper under `#[actix_web::test]`).
- [INF] Do not share a global static pool across tests: each test has its own runtime and connections created on a runtime that has shut down fail. `#[sqlx::test]` avoids this by giving each test its own pool.
- [REC] Separate test targets so `make test` still works without a database: e.g. `[[test]] name = "kernel-pg"` and `cti-api-pg-e2e`, run by `make test/pg` and in CI only. AGENTS.md target naming convention extends naturally.
- [REC] Concurrency tests that matter here: N tasks creating the same observable produce exactly one `Created` event; N concurrent observations end with `count == N` (catches F6); an event inserted by a slow transaction with a lower position is still delivered to a consumer (section 7.5).
- [REC] GitHub Actions job (add to `.github/workflows/test-and-build.yml` or a separate workflow):
  ```yaml
  jobs:
    postgres-tests:
      runs-on: ubuntu-latest
      services:
        postgres:
          image: postgres:18
          env:
            POSTGRES_USER: fennec
            POSTGRES_PASSWORD: fennec
            POSTGRES_DB: fennec
          ports: ["5432:5432"]
          options: >-
            --health-cmd "pg_isready -U fennec -d fennec"
            --health-interval 5s --health-timeout 5s --health-retries 10
      env:
        DATABASE_URL: postgres://fennec:fennec@localhost:5432/fennec
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
        - uses: Swatinem/rust-cache@v2
        - run: cargo test -p kernel --test kernel-pg
        - run: cargo test -p cti-api --test cti-api-pg-e2e
  ```
  Run it on stable only; the existing three-toolchain matrix keeps running the database-free suites.

---

## 10. How OpenCTI and MISP handle scale (evidence) and what it implies for Fennec

### 10.1 OpenCTI (Elasticsearch/OpenSearch + Redis + RabbitMQ) [SRC]

| Aspect | Evidence |
|---|---|
| Primary store is Elasticsearch/OpenSearch; observables live in `<prefix>_stix_cyber_observables*`, sightings in `<prefix>_stix_sighting_relationships*` | `GQL/src/database/utils.ts:58-69` |
| Index rollover by ILM/ISM at `max_primary_shard_size` default `'50gb'` or `max_docs` default `75000000` | `GQL/src/database/engine.ts:223-224,1096-1116` |
| Writes are bulk index calls with `refresh: true`, chunks of `MAX_BULK_OPERATIONS` (default 5000), `retry_on_conflict: ES_RETRY_ON_CONFLICT` (30) | `engine.ts:214,229-230,4849-4873` |
| No database uniqueness across rolled-over indices; dedup = Redis Redlock on all participant ids, then search by ids/hashes, then create/upsert/merge | `GQL/src/database/middleware.ts:3951-3970,4084-4114`; `GQL/src/database/redis.ts:434-450` |
| Standard id = UUIDv5(canonical JSON, OASIS namespace) | `GQL/src/schema/general.js:56-57`; `GQL/src/schema/identifier.js:331-347` |
| Event stream = Redis stream; the event is pushed **after** the ES index call, outside any transaction (dual write), and the lock is released in `finally` | `middleware.ts:4205-4225`; `GQL/src/database/redis-stream.ts:64-88` |
| Stream trimmed to about 2 M entries by default; consumers `XREAD COUNT 100 BLOCK 5000` | `GQL/config/default.json:419`; `redis-stream.ts:145-146,167-175` |
| Ingestion is queued through RabbitMQ exchanges consumed by workers | `GQL/src/database/rabbitmq.js:14-15` |
| Case-insensitive matching through an ES keyword normalizer (`lowercase`, `asciifolding`) | `engine.ts:1183-1186` |

### 10.2 MISP (MySQL/MariaDB, one `attributes` table) [SRC]

| Aspect | Evidence |
|---|---|
| All observable kinds share `attributes` with `type varchar(100)`, `value1 text`, `value2 text` (`utf8mb3_unicode_ci`, so comparisons are case-insensitive) | `MISP/INSTALL/MYSQL.sql:70-102` (columns 76-78) |
| Values are only **prefix-indexed** (`INDEX value1 (value1(255))`, same for `value2`); the only unique key besides `id` is `uuid` | `MYSQL.sql:99-101`; `MISP/db_schema.json` `indexes.attributes` |
| Uniqueness is an application check per event ("A similar attribute already exists for this event."), check-then-insert, skipped for object attributes and `fast_update` | `MISP/app/Model/MispAttribute.php:423-426,976-994` |
| Correlations are materialised **pairs** (`default_correlations`, `UNIQUE (attribute_id, 1_attribute_id, value_id)`), so k occurrences of a value produce O(k^2) rows | `MYSQL.sql:378-410` (unique at 408) |
| Correlation values are deduplicated in `correlation_values (value varchar(191) UNIQUE)`, values truncated with `mb_substr(..., 0, 191)` ("index is 191 long, missing the existing value lookup can lead to a duplicate entry"), find-then-insert with catch-and-refind | `MYSQL.sql:301-306`; `MISP/app/Model/CorrelationValue.php:13-22,28-60,68-95` |
| Over-correlation guard: values with more than `MISP.correlation_limit` (default 20) matches are blocked from correlating | `MISP/app/Model/OverCorrelatingValue.php:84-87`; `MISP/app/Model/Correlation.php:538,589-594` |
| CIDR correlation emulated with `INET_ATON(value1) BETWEEN ? AND ?` and `LIKE` prefixes (MySQL only); CIDR list cached in Redis | `Correlation.php:702-735,1044-1081` |
| Normalisation on write (lowercase, punycode, IPv6 compression, `/32` and `/128` stripped) | `MISP/app/Lib/Tools/AttributeValidationTool.php:66-118,763-788` |
| Sightings are one row per sighting event (`sightings` table) rather than a counter | `MYSQL.sql:1365-1380`; `db_schema.json` `schema.sightings` |

### 10.3 Implications [REC]

1. Per-kind tables with native types (`inet`, `cidr`, `macaddr`, `bytea`) remove MISP's string workarounds (prefix indexes, `INET_ATON`, truncated correlation values) and give exact uniqueness.
2. One row per distinct observable (global dedup by natural key or deterministic id) turns "correlation" into a join through sightings/relationships keyed by observable id: O(k) rows per value instead of MISP's O(k^2) pairs and its 20-match cap.
3. A transactional outbox in Postgres closes OpenCTI's dual-write window (ES write, then Redis XADD) and replaces Redlock+search with database constraints plus, only for multi-key File, transaction-scoped advisory locks.
4. Time-partitioned event log with retention plays the role of OpenCTI's trimmed Redis stream, but durable and ordered by commit visibility.

---

## 11. Consolidated recommendations (numbered for the design agent)

1. PostgreSQL 18; schemas per context; owner and app roles; no citext; no default GiST opclass for inet.
2. Native types per section 3.1; parse everything in Rust value objects; DB CHECK constraints mirror value-object invariants; explicit constraint names.
3. Separate host and network aggregates per IP family; decompose non-CIDR ranges into CIDR blocks.
4. Digests as `bytea` with length CHECKs; URL uniqueness on a Rust-computed SHA-256; never index raw unbounded text in a unique btree.
5. Normalise on write; `text COLLATE "C"`; version the normalisation rules (they feed standard ids).
6. Regime A: PK = deterministic STIX UUID for value-identified kinds; regime B (File): random/client id + `stix_id` + partial unique per digest + sorted transaction advisory locks.
7. `INSERT ... ON CONFLICT DO NOTHING` + `rows_affected()` for creates inside a unit of work; PG 18 `RETURNING old/new` for merge upserts on the ingestion path; sort and deduplicate batches.
8. Add `version` to mutable aggregates; optimistic updates or `FOR UPDATE` reads inside write units of work; fix the `SightingObserver` lost update.
9. Keep write tables lean: PK + natural key (+ SP-GiST on network tables); trigram/substring search on the read side; `fillfactor` and per-table autovacuum on `sighting`.
10. Event log: one partitioned (monthly, `recorded_at`) append-only table used as outbox and journal; `position` identity + `transaction_id xid8`; UUIDv7 event ids; jsonb payload; privileges + triggers for append-only; drop partitions for retention.
11. Consumers: polling ordered by `(transaction_id, position)` gated by `pg_snapshot_xmin(pg_current_snapshot())`; checkpoints in the same transaction as projection writes; per-aggregate version checks; NOTIFY only as an optional hint.
12. Consistency: command-scoped unit of work (command-bus decorator + task-local transaction + buffering `EventBus`), commit iff the command published events; extend `DomainEvent` with payload, aggregate type and version.
13. sqlx: runtime queries + `FromRow` in adapters; embedded migrations with `build.rs`; migrations run as owner in a deploy step; one shared pool built in `main`; error mapping on SQLSTATE and constraint names.
14. Tests: keep in-memory suites; add contract tests over both repositories; Postgres suites as separate test targets with `#[sqlx::test(migrator = ...)]`; CI service container job on stable.

---

## 12. Catalogue (items defined or evaluated in these notes)

| # | Item | Kind |
|---|---|---|
| 1 | `kernel.ipv4_address` (`inet`, /32) | table |
| 2 | `kernel.ipv4_network` (`cidr`, < /32) | table |
| 3 | `kernel.ipv6_address` (`inet`, /128) | table (same shape as 1) |
| 4 | `kernel.ipv6_network` (`cidr`, < /128) | table (same shape as 2) |
| 5 | `kernel.mac_address` (`macaddr`) | table |
| 6 | `kernel.autonomous_system` (`bigint` 0..4294967295) | table |
| 7 | `kernel.domain_name` (`text COLLATE "C"`, reversed index) | table |
| 8 | `kernel.email_address` (`text COLLATE "C"`) | table |
| 9 | `kernel.url` (`text` + `bytea` sha256 unique) | table |
| 10 | `kernel.file` (per-digest `bytea`, partial uniques, `stix_id`) | table |
| 11 | `kernel.sighting` (observable reference, pair unique, fillfactor) | table |
| 12 | `kernel.observable_kind` | enum type |
| 13 | `event_store.domain_event` (partitioned by month) | table |
| 14 | `event_store.consumer_checkpoint` | table |
| 15 | `event_store.reject_mutation()` + 2 triggers | function/triggers |
| 16 | SP-GiST `inet_ops` on network tables | index |
| 17 | `reverse(value)` btree on domain_name | index |
| 18 | optional `gin_trgm_ops` on url | index |
| 19 | BRIN on `recorded_at` | index |
| 20 | `(transaction_id, position)`, `(aggregate_id, aggregate_version)`, `(event_id)` | indexes |
| 21 | `ON CONFLICT DO NOTHING` + `rows_affected` create pattern | SQL pattern |
| 22 | merge upsert with `RETURNING new.*, old.id IS NULL` | SQL pattern |
| 23 | sighting duplicate classification query | SQL pattern |
| 24 | optimistic `UPDATE ... WHERE version = $2` | SQL pattern |
| 25 | sorted `pg_advisory_xact_lock` for File | SQL pattern |
| 26 | xid8-gated consumer read query | SQL pattern |
| 27 | `PgUnitOfWork` + `CURRENT_UOW` task-local | Rust type |
| 28 | `TransactionalCommandBus` decorator | Rust type |
| 29 | `OutboxEventBus` (buffering `EventBus`) | Rust type |
| 30 | `SqlxPostgresIpv4AddressRepository::save` | Rust adapter sketch |
| 31 | `map_sqlx_error` | Rust function |
| 32 | `DomainEvent` extension (`payload`, `aggregate_type`, `aggregate_version`) | trait change |
| 33 | `MIGRATOR` static + `build.rs` | build/migration item |
| 34 | GitHub Actions Postgres service job | CI item |

---

## 13. Open questions for the design agent

- Q1. Do observable aggregates keep client-supplied ids (Fennec convention) or switch to server-derived deterministic STIX UUIDs (regime A)? The answer decides PK shape, partitionability and whether natural-key UNIQUE indexes stay. Same question for sighting ids (`uuidv5(observable_id, source_id)`).
- Q2. Canonical domain form for the standard id: A-label (punycode, ASCII, cheap CHECK) or U-label (STIX `idn-hostname`)? And email local-part case folding (MISP lowercases the whole address; OpenCTI lowercases in the GraphQL path only).
- Q3. Which unit-of-work style: ambient task-local (option 1, no port changes) or explicit (options 2/3)? Option 1 depends on the "commit iff events" invariant.
- Q4. Does the domain need a `Conflict`/retryable error concept (409 or automatic retry) for optimistic locking and serialization failures?
- Q5. May observations be coalesced (one `SightingObserved` per sighting per ingestion batch with a count delta) to bound event volume?
- Q6. Event retention and audit requirements (how many months online, archive format), which fixes partition count and the bootstrap strategy for new projections.
- Q7. Is substring search on URLs/domains a write-side requirement or a read-model concern (search engine)?
- Q8. Confirm target versions: PostgreSQL 18 (identity on partitioned tables needs 17+, `uuidv7`/`RETURNING old,new` need 18) and sqlx 0.8.6 vs 0.9.0.
- Q9. Items marked [VERIFY]: BRIN/GIN/autovacuum defaults, `transaction_timeout` version, pgbouncer prepared-statement version, sqlx `commit()` behaviour on an aborted transaction, Actix test helpers under `#[sqlx::test]`, BUG #19000 fix status, and which PG version ships the NOTIFY commit-lock fix.
