# Entities

Reference of every aggregate root currently modelled in the system, grouped by bounded context, with the value objects that compose each one.

Every field in every aggregate is a **value object** — no raw `String`, `Uuid` or `SystemTime` ever leaks into an entity. Value objects are validated at construction, so once an aggregate exists it is guaranteed to be internally consistent.

The **Kind** column below classifies each value object:

- `newtype` — a thin wrapper with no validation (accepts any value of the underlying type).
- `validated` — construction returns `Result<_, Error>` and rejects invalid input.
- `enum` — a closed set of variants, parsed from a string via `from_str`.

---

## `kernel` bounded context

Crate: `libs/kernel/`. Owns the intelligence-source side of the CTI domain.

### `Source`

File: `libs/kernel/src/source/domain/entities/source.rs`

Represents the origin of intelligence — a feed, producer, or external system. Carries only fields common to every kind of source; type-specific payloads live in downstream aggregates that share the same id.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `SourceId` | `uuid::Uuid` | newtype | Externally provided at construction (not generated). |
| `source_type` | `SourceType` | enum `{ Url }` | enum | Parsed from `"url"`. |
| `status` | `SourceStatus` | enum `{ Active, Inactive }` | enum | Parsed from `"active"` / `"inactive"`. |
| `description` | `SourceDescription` | `String` | newtype | Free-form human-readable text. |
| `created_at` | `SourceCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `SourceUpdatedAt` | `std::time::SystemTime` | newtype | Regenerated on every update. |

**Invariants**

- `id`, `source_type`, `created_at` are immutable after creation.
- Only `status` and `description` can change through the update use case.
- `updated_at` is always `>= created_at`.

**Errors**

- `SourceTypeError::Invalid(String)` — unknown `source_type` string.
- `SourceStatusError::Invalid(String)` — unknown `status` string.
- `SourceRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

### `UrlSource`

File: `libs/kernel/src/url_source/domain/entities/url_source.rs`

Payload-specific aggregate for a `Source` whose type is `Url`. Its identifier is shared 1:1 with the parent `Source` (`UrlSource.id == Source.id`). The relationship is by identifier only — the `url_source` module does not import from `source`.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `UrlSourceId` | `uuid::Uuid` | newtype | Must equal the parent `SourceId`. |
| `url` | `UrlSourceUrl` | `String` | validated | Non-empty and starts with `http://` or `https://`. |
| `format` | `UrlSourceFormat` | enum `{ Plain, Csv, Json, Stix }` | enum | Parsed from `"plain"` / `"csv"` / `"json"` / `"stix"`. |
| `polling_interval` | `UrlSourcePollingInterval` | `u32` seconds | validated | Must be strictly `> 0`. |
| `created_at` | `UrlSourceCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `UrlSourceUpdatedAt` | `std::time::SystemTime` | newtype | Regenerated on every update. |

**Invariants**

- `id` and `created_at` are immutable after creation.
- `url`, `format`, and `polling_interval` can all change through the update use case.
- `updated_at` is always `>= created_at`.

**Errors**

- `UrlSourceUrlError::{Empty, InvalidScheme}` — URL validation.
- `UrlSourceFormatError::Invalid(String)` — unknown `format` string.
- `UrlSourcePollingIntervalError::Zero` — polling interval must be `> 0`.
- `UrlSourceRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

## `ioc` bounded context

Crate: `libs/ioc/`. Owns the indicators-of-compromise side of the CTI domain.

### `Ioc`

File: `libs/ioc/src/ioc/domain/entities/ioc.rs`

Represents a single Indicator of Compromise: an observable (IP, domain, URL, hash, email, ...) that is considered relevant for detection or investigation.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `IocId` | `uuid::Uuid` | newtype | Externally provided at construction (not generated). |
| `ioc_type` | `IocType` | enum `{ Ipv4, Ipv6, Domain, Url, Sha256, Sha1, Md5, Email }` | enum | Parsed from `"ipv4"` / `"ipv6"` / `"domain"` / `"url"` / `"sha256"` / `"sha1"` / `"md5"` / `"email"`. |
| `value` | `IocValue` | `String` | validated | Non-empty. Per-type validation is intentionally deferred. |
| `created_at` | `IocCreatedAt` | `std::time::SystemTime` | newtype | Set to `now()` on creation. |
| `updated_at` | `IocUpdatedAt` | `std::time::SystemTime` | newtype | Set to the same instant as `created_at` on creation. |

**Invariants**

- `id`, `ioc_type`, `value`, and `created_at` are immutable after creation (no update use case is exposed).
- `updated_at` is always `>= created_at`.

**Errors**

- `IocTypeError::Invalid(String)` — unknown `ioc_type` string.
- `IocValueError::Empty` — value must be non-empty.
- `IocRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

## `sighting` bounded context

Crate: `libs/sighting/`. Owns the observation side of the CTI domain — records the fact that a specific `Source` reported a specific `Ioc`.

### `Sighting`

File: `libs/sighting/src/sighting/domain/entities/sighting.rs`

Represents a single observation of an ioc by a source. There is one aggregate per `(ioc_id, source_id)` pair — an IoC reported by N different sources produces N sightings. References to `Ioc` and `Source` are by identifier only; the `sighting` module does not import from `ioc` or `kernel`.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `id` | `SightingId` | `uuid::Uuid` | newtype | Externally provided at construction (not generated). |
| `ioc_id` | `SightingIocId` | `uuid::Uuid` | newtype | Identifier of the referenced `Ioc`. Externally provided. |
| `source_id` | `SightingSourceId` | `uuid::Uuid` | newtype | Identifier of the referenced `Source`. Externally provided. |
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

- `SightingCountError::Zero` — `count` must be at least `1`.
- `SightingRepositoryError::{NotFound, IdAlreadyExists, PairAlreadyExists { existing_id }, Unexpected(String)}` — persistence-layer failures. `PairAlreadyExists` carries the id of the existing sighting so the caller can immediately observe it.

---

## `config` bounded context

Crate: `libs/config/`. Reference/example context — a generic key/value store used to illustrate the architecture. Not part of the CTI domain.

### `ConfigEntry`

File: `libs/config/src/config_entry/domain/entities/config_entry.rs`

Aggregate root for a single key/value pair. The key acts as the identifier.

| Field | Value Object | Inner type | Kind | Notes |
|---|---|---|---|---|
| `key` | `ConfigKey` | `String` | newtype | Unique identifier of the entry. |
| `value` | `ConfigValue` | `String` | newtype | Free-form value stored under the key. |

**Invariants**

- `key` is immutable after creation.
- `value` can change through the update use case.

**Errors**

- `ConfigEntryRepositoryError::{NotFound, AlreadyExists, Unexpected(String)}` — persistence-layer failures.

---

## Cross-context relationships

Bounded contexts never import each other's types. When two aggregates need to be linked, they share an **identifier** and communicate through **domain events**.

| Relationship | Type | Detail |
|---|---|---|
| `UrlSource.id` ↔ `Source.id` | Shared identifier | Both aggregates are keyed by the same UUID. Neither imports the other. |
| `Sighting.ioc_id` → `Ioc.id` | Shared identifier | `sighting` references `ioc` by identifier only; it does not import the `ioc` crate. |
| `Sighting.source_id` → `Source.id` | Shared identifier | `sighting` references `source` by identifier only; it does not import the `kernel` crate. |

New context-crossing links (IoC ↔ Source, Sighting ↔ IoC, etc.) will follow the same rule: identifier plus events, never a direct import.

---

## Summary

| Context | Aggregate | Value Objects | Exposed by |
|---|---|---|---|
| `kernel` | `Source` | `SourceId`, `SourceType`, `SourceStatus`, `SourceDescription`, `SourceCreatedAt`, `SourceUpdatedAt` | `cti_api` — `/sources[/{id}]` |
| `kernel` | `UrlSource` | `UrlSourceId`, `UrlSourceUrl`, `UrlSourceFormat`, `UrlSourcePollingInterval`, `UrlSourceCreatedAt`, `UrlSourceUpdatedAt` | `cti_api` — `/url-sources[/{id}]` |
| `ioc` | `Ioc` | `IocId`, `IocType`, `IocValue`, `IocCreatedAt`, `IocUpdatedAt` | `cti_api` — `/iocs[/{id}]` |
| `sighting` | `Sighting` | `SightingId`, `SightingIocId`, `SightingSourceId`, `SightingFirstSeen`, `SightingLastSeen`, `SightingCount`, `SightingCreatedAt`, `SightingUpdatedAt` | `cti_api` — `/sightings[/{id}]`, `/sightings/{id}/observations`, `/iocs/{ioc_id}/sightings`, `/sources/{source_id}/sightings` |
| `config` | `ConfigEntry` | `ConfigKey`, `ConfigValue` | `config_api` — `/config[/{key}]` |
