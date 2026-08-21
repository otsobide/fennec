# Entities

Reference of every aggregate root currently modelled in the system, grouped by bounded context, with the value objects that compose each one.

The CTI domain lives in a single bounded context, `kernel`, which hosts **one module per aggregate**. The modules are isolated from each other: each carries its own `domain` / `application` / `infrastructure` tree, declares its own value object for any foreign identifier, and never imports a sibling. Promoting one of them into its own crate is a move, not a redesign, and `tests/libs/kernel/src/module_isolation_tests.rs` fails the build if that isolation is broken.

Every field in every aggregate is a **value object** — no raw `String`, `Uuid` or `SystemTime` ever leaks into an entity. Value objects are validated at construction, so once an aggregate exists it is guaranteed to be internally consistent.

The **Kind** column below classifies each value object:

- `newtype` — a thin wrapper with no validation (accepts any value of the underlying type).
  Only timestamps generated inside the domain services are newtypes.
- `validated` — construction returns `Result<_, ValueObjectValidationError>` and rejects
  invalid input. Every value that reaches the domain from outside is validated.
- `enum` — a closed set of variants, parsed from a string via `from_str`, which also
  returns `Result<_, ValueObjectValidationError>`.

Every fallible constructor reports failures through the single shared
`ValueObjectValidationError` (`libs/shared/valueobject`); no value object defines its own
error type. Identifiers additionally expose `generate()`, which produces a fresh UUID v4.

---

## `kernel` bounded context

Crate: `libs/kernel/`. Owns the whole CTI domain, one module per aggregate: `source` and `url_source` (where intelligence comes from), `ioc` (what is being tracked) and `sighting` (who reported what, and when).

### `Source`

File: `libs/kernel/src/source/domain/entities/source.rs`

Represents the origin of intelligence — a feed, producer, or external system. Carries only fields common to every kind of source; type-specific payloads live in downstream aggregates that share the same id.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `SourceId` | `uuid::Uuid` | validated | Externally provided at construction; must parse as a UUID **v4**. `SourceId::generate()` exists for platform-side creation. |
| `source_type` | `SourceType` | enum `{ Url }` | enum | Parsed from `"url"`, ignoring case and surrounding whitespace. |
| `status` | `SourceStatus` | enum `{ Active, Inactive }` | enum | Parsed from `"active"` / `"inactive"`, ignoring case and surrounding whitespace. |
| `description` | `SourceDescription` | `String` | validated | Free-form human-readable text: trimmed, may be empty, at most 1024 characters. |
| `created_at` | `SourceCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `SourceUpdatedAt` | `std::time::SystemTime` | newtype | Regenerated on every update. |

**Invariants**

- `id`, `source_type`, `created_at` are immutable after creation.
- Only `status` and `description` can change through the update use case.
- `updated_at` is always `>= created_at`.

**Errors**

- `ValueObjectValidationError` — unknown `source_type` or `status` string, id that is not a UUID v4, or description over 1024 characters.
- `SourceRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

### `UrlSource`

File: `libs/kernel/src/url_source/domain/entities/url_source.rs`

Payload-specific aggregate for a `Source` whose type is `Url`. Its identifier is shared 1:1 with the parent `Source` (`UrlSource.id == Source.id`). The relationship is by identifier only — the `url_source` module does not import from `source`.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `UrlSourceId` | `uuid::Uuid` | validated | Must parse as a UUID **v4** and equal the parent `SourceId`. |
| `url` | `UrlSourceUrl` | `String` | validated | Trimmed, non-empty, starts with `http://` or `https://`, at most 2048 characters. |
| `format` | `UrlSourceFormat` | enum `{ Plain, Csv, Json, Stix }` | enum | Parsed from `"plain"` / `"csv"` / `"json"` / `"stix"`. |
| `polling_interval` | `UrlSourcePollingInterval` | `u32` seconds | validated | Must be strictly `> 0`. |
| `created_at` | `UrlSourceCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `UrlSourceUpdatedAt` | `std::time::SystemTime` | newtype | Regenerated on every update. |

**Invariants**

- `id` and `created_at` are immutable after creation.
- `url`, `format`, and `polling_interval` can all change through the update use case.
- `updated_at` is always `>= created_at`.

**Errors**

- `ValueObjectValidationError` — id that is not a UUID v4, empty/over-long URL, URL without an `http(s)` scheme, unknown `format` string, or a polling interval of `0`.
- `UrlSourceRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

### `Ioc`

Module: `libs/kernel/src/ioc/`. File: `libs/kernel/src/ioc/domain/entities/ioc.rs`

Represents a single Indicator of Compromise: an observable (IP, domain, URL, hash, email, ...) that is considered relevant for detection or investigation.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `IocId` | `uuid::Uuid` | validated | Externally provided at construction; must parse as a UUID **v4**. |
| `ioc_type` | `IocType` | enum `{ Ipv4, Ipv6, Domain, Url, Sha256, Sha1, Md5, Email }` | enum | Parsed from `"ipv4"` / `"ipv6"` / `"domain"` / `"url"` / `"sha256"` / `"sha1"` / `"md5"` / `"email"`. |
| `value` | `IocValue` | `String` | validated | Trimmed, non-empty, at most 2048 characters. Per-type validation is intentionally deferred. |
| `created_at` | `IocCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `IocUpdatedAt` | `std::time::SystemTime` | newtype | Set to the same instant as `created_at` on creation. |

**Invariants**

- `id`, `ioc_type`, `value`, and `created_at` are immutable after creation (no update use case is exposed).
- `updated_at` is always `>= created_at`.

**Errors**

- `ValueObjectValidationError` — id that is not a UUID v4, unknown `ioc_type` string, or an empty/over-long value.
- `IocRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

### `Sighting`

Module: `libs/kernel/src/sighting/`. Records the fact that a specific `Source` reported a specific `Ioc`. File: `libs/kernel/src/sighting/domain/entities/sighting.rs`

Represents a single observation of an ioc by a source. There is one aggregate per `(ioc_id, source_id)` pair — an IoC reported by N different sources produces N sightings. References to `Ioc` and `Source` are by identifier only; the `sighting` module does not import from the `ioc` or `source` modules.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `SightingId` | `uuid::Uuid` | validated | Externally provided at construction; must parse as a UUID **v4**. |
| `ioc_id` | `SightingIocId` | `uuid::Uuid` | validated | Identifier of the referenced `Ioc`. Externally provided; must be a UUID **v4**. |
| `source_id` | `SightingSourceId` | `uuid::Uuid` | validated | Identifier of the referenced `Source`. Externally provided; must be a UUID **v4**. |
| `first_seen` | `SightingFirstSeen` | `std::time::SystemTime` | newtype | Set from the caller-supplied `observed_at` on creation; never mutated. |
| `last_seen` | `SightingLastSeen` | `std::time::SystemTime` | newtype | Set from `observed_at` on creation; refreshed on each observation to `max(previous, observed_at)`. |
| `count` | `SightingCount` | `u64` | validated | Must be `>= 1`. Starts at `1`; incremented by `1` on each observation. |
| `created_at` | `SightingCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `SightingUpdatedAt` | `std::time::SystemTime` | newtype | Set to the same instant as `created_at` on creation; regenerated on each observation. |

**Invariants**

- `id`, `ioc_id`, `source_id`, `first_seen`, `created_at` are immutable after creation.
- `count` is always `>= 1`.
- `last_seen` is always `>= first_seen`.
- `updated_at` is always `>= created_at`.
- Only one sighting exists per `(ioc_id, source_id)` pair.

**Errors**

- `ValueObjectValidationError` — id, `ioc_id` or `source_id` that is not a UUID v4, or a `count` below `1`.
- `SightingRepositoryError::{NotFound, IdAlreadyExists, PairAlreadyExists { existing_id }, Unexpected(String)}` — persistence-layer failures. `PairAlreadyExists` carries the id of the existing sighting so the caller can immediately observe it.

---

## `config` bounded context

Crate: `libs/config/`. Reference/example context — a generic key/value store used to illustrate the architecture. Not part of the CTI domain.

### `ConfigEntry`

File: `libs/config/src/config_entry/domain/entities/config_entry.rs`

Aggregate root for a single key/value pair. The key acts as the identifier.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `key` | `ConfigKey` | `String` | validated | Unique identifier of the entry: trimmed, non-empty, at most 255 characters. |
| `value` | `ConfigValue` | `String` | validated | Free-form value stored under the key: trimmed, may be empty, at most 4096 characters. |

**Invariants**

- `key` is immutable after creation.
- `value` can change through the update use case.

**Errors**

- `ConfigEntryRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

## Relationships between aggregates

No module imports another module's types, whether the two live in the same crate or not. When two aggregates need to be linked, they share an **identifier** and communicate through the **buses**: commands and queries for anything a caller drives, domain events for anything a module reacts to.

| Relationship | Type | Detail |
|---|---|---|
| `UrlSource.id` ↔ `Source.id` | Shared identifier | Both aggregates are keyed by the same UUID. Neither imports the other. |
| `Sighting.ioc_id` → `Ioc.id` | Shared identifier | `sighting` declares its own `SightingIocId`; it never uses `IocId`. |
| `Sighting.source_id` → `Source.id` | Shared identifier | `sighting` declares its own `SightingSourceId`; it never uses `SourceId`. |

This is what keeps every module extractable: the day one of them needs its own crate (or its own service) for scale, the identifiers already match and no import has to be untangled. New links follow the same rule: identifier plus buses, never a direct import.

---

## Summary

| Context | Aggregate (module) | Value Objects | Exposed by |
|---|---|---|---|
| `kernel` | `Source` | `SourceId`, `SourceType`, `SourceStatus`, `SourceDescription`, `SourceCreatedAt`, `SourceUpdatedAt` | `cti_api` — `/sources[/{id}]` |
| `kernel` | `UrlSource` | `UrlSourceId`, `UrlSourceUrl`, `UrlSourceFormat`, `UrlSourcePollingInterval`, `UrlSourceCreatedAt`, `UrlSourceUpdatedAt` | `cti_api` — `/url-sources[/{id}]` |
| `kernel` | `Ioc` | `IocId`, `IocType`, `IocValue`, `IocCreatedAt`, `IocUpdatedAt` | `cti_api` — `/iocs[/{id}]` |
| `kernel` | `Sighting` | `SightingId`, `SightingIocId`, `SightingSourceId`, `SightingFirstSeen`, `SightingLastSeen`, `SightingCount`, `SightingCreatedAt`, `SightingUpdatedAt` | `cti_api` — `/sightings[/{id}]`, `/sightings/{id}/observations`, `/iocs/{ioc_id}/sightings`, `/sources/{source_id}/sightings` |
| `config` | `ConfigEntry` | `ConfigKey`, `ConfigValue` | `config_api` — `/config[/{key}]` |
