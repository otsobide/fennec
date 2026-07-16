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

New context-crossing links (IoC ↔ Source, Sighting ↔ IoC, etc.) will follow the same rule: identifier plus events, never a direct import.

---

## Summary

| Context | Aggregate | Value Objects | Exposed by |
|---|---|---|---|
| `kernel` | `Source` | `SourceId`, `SourceType`, `SourceStatus`, `SourceDescription`, `SourceCreatedAt`, `SourceUpdatedAt` | `cti_api` — `/sources[/{id}]` |
| `kernel` | `UrlSource` | `UrlSourceId`, `UrlSourceUrl`, `UrlSourceFormat`, `UrlSourcePollingInterval`, `UrlSourceCreatedAt`, `UrlSourceUpdatedAt` | `cti_api` — `/url-sources[/{id}]` |
| `config` | `ConfigEntry` | `ConfigKey`, `ConfigValue` | `config_api` — `/config[/{key}]` |
