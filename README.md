# 🦊 Fennec

[![Test and Build](https://github.com/otsobide/fennec/actions/workflows/test-and-build.yml/badge.svg?branch=dev)](https://github.com/otsobide/fennec/actions/workflows/test-and-build.yml)

A Rust-based **Cyber Threat Intelligence (CTI)** platform, modelled after projects like MISP, built on a strict Domain-Driven Design + Hexagonal Architecture + CQRS foundation.

## What is Fennec?

Fennec is a CTI platform whose goal is to ingest, normalise, correlate, and serve threat intelligence (observables such as IP addresses, domains, file hashes and tracking ids, threat actors, campaigns, sources, sightings) under a clean, microservice-friendly architecture. Its data model is compatible with STIX 2.1 and OpenCTI and learns from MISP; it is described in [docs/data-model/](docs/data-model/README.md). The codebase is organised as a Cargo workspace where each bounded context is an isolated library crate hosting one module per aggregate, and each HTTP service is a thin Actix-Web app that wires those contexts behind CQRS buses.

The project is **early in its life**, but the core of the model is in place. The `kernel` context owns the CTI domain and holds one module per aggregate: `source` and the URL-specific `url_source` (where intelligence comes from), one module per observable kind (what is being tracked: `ipv4_address` is the first of the more than a hundred kinds in the [catalogue](docs/data-model/observable-catalogue.md)) and `sighting` (which source reported which observable, with first/last seen timestamps and an observation counter). All of them are exposed by `cti_api`. There are no generic types: every observable kind is its own aggregate with value objects that validate and canonicalise its exact syntax. The aggregates still to come (the other observable kinds, threat actors, campaigns, enrichment) plug in alongside them following exactly the same recipe.

The architecture is uncompromising on one principle: business logic depends on traits, never on concrete infrastructure. The same domain code runs against in-memory repositories in tests and PostgreSQL in production, with zero conditional compilation and no code changes in the domain or application layers. On PostgreSQL every command commits its state change and its domain events in one transaction, into an append-only event log that projections and other contexts are built from.

## Bounded contexts

| Context | Module | Aggregate | Status | Exposed by |
|---|---|---|---|---|
| `kernel` | `source` | `Source` | Full CRUD over the common fields of any intelligence source | `cti_api` (:8081) |
| `kernel` | `url_source` | `UrlSource` | Full CRUD over the URL-specific payload of a source, keyed by the same id | `cti_api` (:8081) |
| `kernel` | `domain_name` | `DomainName` | Register, find (by id or value) and delete DNS domain names, canonicalised to lowercase A-labels | `cti_api` (:8081) |
| `kernel` | `email_address` | `EmailAddress` | Register, find (by id or value) and delete email addresses: lowercase NFC local part, domain as lowercase A-labels | `cti_api` (:8081) |
| `kernel` | `hostname` | `Hostname` | Register, find (by id or value) and delete hostnames, with or without a domain, canonicalised to lowercase A-labels | `cti_api` (:8081) |
| `kernel` | `ipv4_address` | `Ipv4Address` | Register, find (by id or value) and delete IPv4 host addresses; the id is the STIX deterministic id | `cti_api` (:8081) |
| `kernel` | `ipv4_network` | `Ipv4Network` | Register, find (by id or value) and delete IPv4 CIDR blocks from `/0` to `/31` with no host bits set | `cti_api` (:8081) |
| `kernel` | `ipv6_address` | `Ipv6Address` | Register, find (by id or value) and delete IPv6 host addresses in RFC 5952 form; the id is the STIX deterministic id | `cti_api` (:8081) |
| `kernel` | `ipv6_network` | `Ipv6Network` | Register, find (by id or value) and delete IPv6 CIDR blocks from `/0` to `/127` with no host bits set | `cti_api` (:8081) |
| `kernel` | `mac_address` | `MacAddress` | Register, find (by id or value) and delete EUI-48 MAC addresses in any common spelling, stored as lowercase colon-separated octets | `cti_api` (:8081) |
| `kernel` | `sighting` | `Sighting` | Records that a source reported an observable, with first/last seen and an observation counter | `cti_api` (:8081) |
| `config` | `config_entry` | `ConfigEntry` | Generic key/value store kept as a reference example | `config_api` (:8080) |

Each context is a crate under `libs/<context>/`, and each module inside it owns one aggregate with its own `domain` / `application` / `infrastructure` tree. Apps under `apps/` wire them into HTTP: `cti_api` serves every `kernel` module, `config_api` serves `config`.

**No module imports another**, in the same crate or not. Aggregates relate by shared identifier (`sighting` declares its own `SightingObservableKind` and `SightingObservableId` rather than importing an observable module) and communicate through the command, query and event buses. That is deliberate: the day a module needs its own crate, or its own service, for scale, promoting it is a move rather than a redesign. Two architecture tests fail the build if that isolation is broken.

## Stack

| Component | Technology |
|---|---|
| Language | Rust 2021 |
| HTTP framework | Actix-Web 4 |
| Async runtime | Tokio |
| Persistence | PostgreSQL 18 through `sqlx` (one table per aggregate kind plus the `event_store` log), or in memory; both behind the same repository traits |
| Logging | `tracing` + `tracing-subscriber` (env-filtered) |
| Errors | `thiserror` for domain and repository errors |
| Validation | Value objects validated at construction, reporting through a single shared `ValueObjectValidationError` |
| Architecture | DDD + Hexagonal (Ports & Adapters) + CQRS + Domain Events |

No database is required to develop, test, or run the project: without `DATABASE_URL` everything runs in memory.

## Architecture overview

Three layers, one dependency rule:

```
HTTP (apps/)               <-- Actix-Web controllers
   │
   ▼
CQRS buses (libs/shared/)  <-- CommandBus + QueryBus
   │
   ▼
Application (libs/*/application/)
   │
   ▼
Domain (libs/*/domain/)    <-- pure types, repository traits, events
   ▲
   │
Infrastructure (libs/*/infrastructure/)
```

Arrows point inward only. The domain layer has no knowledge of frameworks, databases, or HTTP. See [docs/architecture/overview.md](docs/architecture/overview.md) for the full breakdown.

### Bounded context layout

```
<context>/
  domain/
    entities/         # Aggregate roots
    value_objects/    # Typed wrappers, validated at construction (every attribute is a VO,
                      # including timestamps; ids are UUID v4)
    repositories/     # Trait definitions only
    events/           # Domain events + factory functions
    errors/           # NotFound | AlreadyExists | Unexpected
  application/
    <verb>_<noun>/    # One folder per use case
      <noun>_<verb>er.rs              # Domain service
      <verb>_<noun>_command.rs        # Command struct (writes)
      <verb>_<noun>_query.rs          # Query struct (reads)
      <verb>_<noun>_response.rs       # Response envelope: commands carry { error }, queries
                                      # add the payload field named after the aggregate
      <verb>_<noun>_command_handler.rs / _query_handler.rs
  infrastructure/
    persistence/
      in_memory/      # HashMap-based implementations
```

## Project structure

```
fennec/
├── apps/
│   ├── cti_api/         # CTI HTTP API (port 8081), exposes every kernel module
│   └── config_api/      # Reference HTTP API (port 8080) — exposes the config BC
│
├── libs/
│   ├── kernel/          # CTI bounded context, one module per aggregate
│   │   └── src/
│   │       ├── source/      # Source: id, type, status, description, timestamps
│   │       ├── url_source/  # UrlSource: url, format, polling_interval, timestamps
│   │       ├── ipv4_address/ # Ipv4Address: an observed IPv4 host, identified by its value
│   │       └── sighting/    # Sighting: an observable reported by a source, first/last seen, count
│   ├── config/          # Reference bounded context (config_entry CRUD)
│   └── shared/
│       ├── cqrs/            # CommandBus + QueryBus (TypeId-based dispatch)
│       ├── domain-events/   # EventBus + DomainEventSubscriber
│       └── valueobject/     # ValueObjectValidationError + value object primitives
│
├── tests/
│   ├── apps/{cti_api,config_api}/           # E2E tests (HTTP → bus → repo)
│   └── libs/{kernel,config}/                # Unit tests (mocks + Object Mother)
│
├── docs/                # Documentation, one folder per topic
├── docker-compose.yml   # PostgreSQL scaffolding (currently unused)
├── Makefile             # Root Makefile (delegates to per-app Makefiles)
└── Cargo.toml           # Workspace root
```

A more detailed annotated tree lives in [docs/architecture/project-structure.md](docs/architecture/project-structure.md).

## Quick start

**Prerequisites:** Rust stable (2021 edition).

### Build

```bash
make build              # Release build (all apps)
make dev/build          # Dev build
make cti_api/build      # Release build for cti_api only
make config_api/build   # Release build for config_api only
```

### Test

```bash
make test               # Everything (unit + e2e + doc-tests)
make test/unit          # Everything except config-api (so it still runs the cti-api e2e suite)
make test/e2e           # All e2e suites
make cti_api/test/e2e   # E2E tests for cti_api only
make test/summary       # cargo test with empty-suite noise stripped
cargo test -p kernel    # One context's unit tests
```

The default suites need no database. Tests that do are `#[ignore]`d and run with a PostgreSQL 18 server:

```bash
docker compose up -d postgres
DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec cargo test --workspace -- --ignored
```

### Run

```bash
make cti_api/run        # Start cti_api on port 8081
make config_api/run     # Start config_api on port 8080
```

With `DATABASE_URL` set, `cti_api` persists in PostgreSQL: it runs the event store and kernel migrations at startup and keeps a year of event log partitions ready.

```bash
docker compose up -d postgres
DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec make cti_api/run
```

Log level defaults to `info`. Override with `RUST_LOG`:

```bash
RUST_LOG=debug make cti_api/run
```

### HTTP API

`cti_api` (:8081)

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/health` | Liveness probe |
| `POST` | `/sources` | Create a source (`id`, `source_type`, `status`, `description`) |
| `GET` `PUT` `DELETE` | `/sources/{id}` | Read, update (`status`, `description`) or delete a source |
| `POST` | `/url-sources` | Create the URL payload of a source (`id`, `url`, `format`, `polling_interval_seconds`) |
| `GET` `PUT` `DELETE` | `/url-sources/{id}` | Read, update or delete it. The id is the parent source's id |
| `POST` `GET` `DELETE` | `/domain-names`, `/domain-names?value=`, `/domain-names/{id}` | The same contract as IPv4 addresses, for domain names |
| `POST` `GET` `DELETE` | `/email-addresses`, `/email-addresses?value=`, `/email-addresses/{id}` | The same contract as IPv4 addresses, for email addresses (`@` percent-encoded in the query) |
| `POST` `GET` `DELETE` | `/hostnames`, `/hostnames?value=`, `/hostnames/{id}` | The same contract as IPv4 addresses, for hostnames |
| `POST` | `/ipv4-addresses` | Register an IPv4 address (`value` only); answers `201` with `{id, stix_id}`, or `409` with the same body if it is already registered |
| `GET` | `/ipv4-addresses?value=<address>` | Find an IPv4 address by value |
| `GET` `DELETE` | `/ipv4-addresses/{id}` | Read or delete an IPv4 address |
| `POST` `GET` `DELETE` | `/ipv4-networks`, `/ipv4-networks?value=`, `/ipv4-networks/{id}` | The same contract as IPv4 addresses, for IPv4 networks |
| `POST` `GET` `DELETE` | `/ipv6-addresses`, `/ipv6-addresses?value=`, `/ipv6-addresses/{id}` | The same contract as IPv4 addresses, for IPv6 addresses |
| `POST` `GET` `DELETE` | `/ipv6-networks`, `/ipv6-networks?value=`, `/ipv6-networks/{id}` | The same contract as IPv4 addresses, for IPv6 networks |
| `POST` `GET` `DELETE` | `/mac-addresses`, `/mac-addresses?value=`, `/mac-addresses/{id}` | The same contract as IPv4 addresses, for MAC addresses |
| `POST` | `/sightings` | Record a first sighting (`id`, `observable_kind`, `observable_id`, `source_id`, `observed_at`) |
| `POST` | `/sightings/{id}/observations` | Register another observation (`observed_at`): bumps the counter and last seen |
| `GET` `DELETE` | `/sightings/{id}` | Read or delete a sighting |
| `GET` | `/observables/{kind}/{id}/sightings` | Every sighting of one observable |
| `GET` | `/sources/{source_id}/sightings` | Every sighting reported by one source |

`config_api` (:8080) exposes the same shape for the reference context: `POST /config`, and `GET` / `PUT` / `DELETE` on `/config/{key}`.

Sources and sightings take a **UUID v4** id supplied by the caller. Value-identified observables such as IPv4 addresses take no id: theirs is the UUID v5 of their [STIX deterministic identifier](docs/data-model/identity.md), derived from the canonical value, so every producer that sees the same address gets the same id. `observed_at` is seconds since the Unix epoch. Invalid input returns `400`, a duplicate `409`, an unknown id `404`.

### Try it

```bash
BASE=http://localhost:8081

# 1. A source: where intelligence comes from
curl -X POST $BASE/sources -H 'Content-Type: application/json' -d '{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "source_type": "url",
  "status": "active",
  "description": "Primary feed"
}'

# 2. Its URL payload, keyed by the same id
curl -X POST $BASE/url-sources -H 'Content-Type: application/json' -d '{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "url": "https://example.test/feed.txt",
  "format": "plain",
  "polling_interval_seconds": 3600
}'

# 3. An observable: its id is derived from the value
curl -X POST $BASE/ipv4-addresses -H 'Content-Type: application/json' -d '{
  "value": "198.51.100.7"
}'
# => {"id":"12592121-be4c-5a93-a813-5a5af16e50db","stix_id":"ipv4-addr--12592121-be4c-5a93-a813-5a5af16e50db"}

# 4. The source reported that address
curl -X POST $BASE/sightings -H 'Content-Type: application/json' -d '{
  "id": "7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061",
  "observable_kind": "ipv4_address",
  "observable_id": "12592121-be4c-5a93-a813-5a5af16e50db",
  "source_id": "550e8400-e29b-41d4-a716-446655440000",
  "observed_at": 1750000000
}'

# 5. It reported it again: count goes to 2, last_seen moves forward
curl -X POST $BASE/sightings/7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061/observations \
  -H 'Content-Type: application/json' -d '{"observed_at": 1750003600}'

# 6. Everything seen for that address
curl $BASE/observables/ipv4_address/12592121-be4c-5a93-a813-5a5af16e50db/sightings
```

## Documentation

The `docs/` directory is organised one folder per topic, each with its own
`README.md` entry point. Start with
[docs/architecture/overview.md](docs/architecture/overview.md) for the big picture.

| Topic | Contents |
|---|---|
| [architecture/](docs/architecture/README.md) | [overview](docs/architecture/overview.md) (layers, dependency rule, patterns), [project-structure](docs/architecture/project-structure.md), [cqrs](docs/architecture/cqrs.md), [domain-events](docs/architecture/domain-events.md), [entities](docs/architecture/entities.md), [persistence](docs/architecture/persistence.md), [event-store](docs/architecture/event-store.md) |
| [data-model/](docs/data-model/README.md) | [identity](docs/data-model/identity.md), [observable kinds](docs/data-model/observable-kinds.md), [catalogue](docs/data-model/observable-catalogue.md), [kinds](docs/data-model/kinds/README.md), [interoperability](docs/data-model/interoperability.md) |
| [research/](docs/research/README.md) | OpenCTI, MISP and STIX 2.1 data models, pivot identifiers, PostgreSQL storage |
| [development/](docs/development/README.md) | [gitflow](docs/development/gitflow.md) (branching, rulesets, commit format), [testing](docs/development/testing.md) (mocks, object mothers, e2e) |
| [guides/](docs/guides/README.md) | [adding-a-bounded-context](docs/guides/adding-a-bounded-context.md), [adding-an-app](docs/guides/adding-an-app.md) |

Conventions shared with the other repositories (the English-only rule, the
small-commit discipline, the gitflow model, the DDD service architecture) live
in the separate `.knowledge` repository and take precedence over these pages.

## Make targets

| Target | Description |
|---|---|
| `make build` | Release build (workspace) |
| `make dev/build` | Dev build |
| `make test` | Run every test (unit, e2e, doc-tests) |
| `make test/unit` | Workspace tests except `config-api` |
| `make test/e2e` | E2E suites of every app |
| `make test/summary` | `cargo test` with empty-suite noise stripped |
| `make format` | `cargo fmt` |
| `make audit` | Security audit via cargo-audit |
| `make deps` | `cargo update` |
| `make docker/up` / `docker/down` / `docker/logs` | The unused PostgreSQL scaffolding |

Per app (`cti_api`, `config_api`), delegated to `apps/<app>/Makefile`:

| Target | Description |
|---|---|
| `make <app>/build` | Release build for that app |
| `make <app>/dev-build` | Dev build for that app |
| `make <app>/run` | Run it locally |
| `make <app>/test` | Every test of that package |
| `make <app>/test/e2e` | Only its e2e suite |

## Contributing

`dev` is the default branch and both long-lived branches are protected by
GitHub rulesets with empty bypass lists: **direct pushes to `dev` and `main`
are rejected for everyone**, owner included.

```bash
git switch dev && git pull
git switch -c feature/<slug>      # even a one-line change gets a branch
# ...work, committing small and often...
cargo test --workspace            # green before the pull request merges
git push -u origin feature/<slug>
gh pr create --base dev
```

A pull request into `dev` must come from `feature/*`, `hotfix/*` or `main`; a
pull request into `main` must come from `dev` and is a release, tagged
`vX.Y.Z`. Commit messages are `<area>: <imperative summary>`, in English, one
logical change each. The full model, including the ruleset definitions, is in
[docs/development/gitflow.md](docs/development/gitflow.md).
