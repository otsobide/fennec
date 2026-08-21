# 🦊 Fennec

[![Test and Build](https://github.com/otsobide/fennec/actions/workflows/test-and-build.yml/badge.svg?branch=dev)](https://github.com/otsobide/fennec/actions/workflows/test-and-build.yml)

A Rust-based **Cyber Threat Intelligence (CTI)** platform, modelled after projects like MISP, built on a strict Domain-Driven Design + Hexagonal Architecture + CQRS foundation.

## What is Fennec?

Fennec is a CTI platform whose goal is to ingest, normalise, correlate, and serve threat intelligence (indicators of compromise, threat actors, campaigns, sources, sightings) under a clean, microservice-friendly architecture. The codebase is organised as a Cargo workspace where each bounded context is an isolated library crate and each HTTP service is a thin Actix-Web app that wires those contexts behind CQRS buses.

The project is **early in its life**, but the core of the model is in place: `kernel` owns the intelligence sources (`Source`, plus the URL-specific `UrlSource` keyed by the same id), `ioc` owns the indicators, and `sighting` records that a source reported an indicator, with first/last seen timestamps and an observation counter. All three are exposed by `cti_api`. The contexts still to come (threat actors, campaigns, events, enrichment) plug in alongside them following exactly the same architectural recipe.

The architecture is uncompromising on one principle: business logic depends on traits, never on concrete infrastructure. The same domain code runs against an in-memory `HashMap` in tests and (eventually) a PostgreSQL store in production, with zero conditional compilation and no code changes in the domain or application layers.

## Bounded contexts

| Context | Aggregate | Status | Exposed by |
|---|---|---|---|
| `kernel` | `Source` | Full CRUD over the common fields of any intelligence source | `cti_api` (:8081) |
| `kernel` | `UrlSource` | Full CRUD over the URL-specific payload of a source, keyed by the same id | `cti_api` (:8081) |
| `ioc` | `Ioc` | Create, find and delete indicators of compromise | `cti_api` (:8081) |
| `sighting` | `Sighting` | Records that a source reported an indicator, with first/last seen and an observation counter | `cti_api` (:8081) |
| `config` | `ConfigEntry` | Generic key/value store kept as a reference example | `config_api` (:8080) |

Each context lives under `libs/<context>/` and is wired into HTTP by an app under `apps/`. An app may expose several contexts: `cti_api` serves `kernel`, `ioc` and `sighting`, while `config_api` serves the reference `config` context on its own. Cross-context communication happens **only** through domain events; aggregates in different contexts relate by shared identifier, never by importing each other's crates.

## Stack

| Component | Technology |
|---|---|
| Language | Rust 2021 |
| HTTP framework | Actix-Web 4 |
| Async runtime | Tokio |
| Persistence | In-memory (HashMap + Mutex) — pluggable via repository traits |
| Logging | `tracing` + `tracing-subscriber` (env-filtered) |
| Errors | `thiserror` for domain and repository errors |
| Validation | Value objects validated at construction, reporting through a single shared `ValueObjectValidationError` |
| Architecture | DDD + Hexagonal (Ports & Adapters) + CQRS + Domain Events |

No database is required to develop, test, or run the project locally — the default repositories are all in-memory.

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
│   ├── cti_api/         # CTI HTTP API (port 8081) — exposes kernel, ioc and sighting
│   └── config_api/      # Reference HTTP API (port 8080) — exposes the config BC
│
├── libs/
│   ├── kernel/          # CTI kernel bounded context
│   │   ├── src/source/      # Source aggregate: id, type, status, description, timestamps
│   │   └── src/url_source/  # UrlSource aggregate: url, format, polling_interval, timestamps
│   ├── ioc/             # Indicators of compromise
│   ├── sighting/        # Sightings: an ioc reported by a source
│   ├── config/          # Reference bounded context (config_entry CRUD)
│   └── shared/
│       ├── cqrs/            # CommandBus + QueryBus (TypeId-based dispatch)
│       ├── domain-events/   # EventBus + DomainEventSubscriber
│       └── valueobject/     # ValueObjectValidationError + value object primitives
│
├── tests/
│   ├── apps/{cti_api,config_api}/           # E2E tests (HTTP → bus → repo)
│   └── libs/{kernel,ioc,sighting,config}/   # Unit tests (mocks + Object Mother)
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

No database needed — the e2e suites use the in-memory repositories.

### Run

```bash
make cti_api/run        # Start cti_api on port 8081
make config_api/run     # Start config_api on port 8080
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
| `POST` | `/iocs` | Create an indicator (`id`, `ioc_type`, `value`) |
| `GET` `DELETE` | `/iocs/{id}` | Read or delete an indicator |
| `POST` | `/sightings` | Record a first sighting (`id`, `ioc_id`, `source_id`, `observed_at`) |
| `POST` | `/sightings/{id}/observations` | Register another observation (`observed_at`): bumps the counter and last seen |
| `GET` `DELETE` | `/sightings/{id}` | Read or delete a sighting |
| `GET` | `/iocs/{ioc_id}/sightings` | Every sighting of one indicator |
| `GET` | `/sources/{source_id}/sightings` | Every sighting reported by one source |

`config_api` (:8080) exposes the same shape for the reference context: `POST /config`, and `GET` / `PUT` / `DELETE` on `/config/{key}`.

Identifiers are supplied by the caller and must be **UUID v4**; `observed_at` is seconds since the Unix epoch. Invalid input returns `400`, a duplicate id `409`, an unknown id `404`.

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

# 3. An indicator
curl -X POST $BASE/iocs -H 'Content-Type: application/json' -d '{
  "id": "6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60",
  "ioc_type": "ipv4",
  "value": "198.51.100.7"
}'

# 4. The source reported that indicator
curl -X POST $BASE/sightings -H 'Content-Type: application/json' -d '{
  "id": "7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061",
  "ioc_id": "6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60",
  "source_id": "550e8400-e29b-41d4-a716-446655440000",
  "observed_at": 1750000000
}'

# 5. It reported it again: count goes to 2, last_seen moves forward
curl -X POST $BASE/sightings/7e2b3c4d-5e6f-4071-9b02-1c2d3e4f5061/observations \
  -H 'Content-Type: application/json' -d '{"observed_at": 1750003600}'

# 6. Everything seen for that indicator
curl $BASE/iocs/6f1a2b3c-4d5e-4f60-8a91-0b2c3d4e5f60/sightings
```

## Documentation

The `docs/` directory is organised one folder per topic, each with its own
`README.md` entry point. Start with
[docs/architecture/overview.md](docs/architecture/overview.md) for the big picture.

| Topic | Contents |
|---|---|
| [architecture/](docs/architecture/README.md) | [overview](docs/architecture/overview.md) (layers, dependency rule, patterns), [project-structure](docs/architecture/project-structure.md), [cqrs](docs/architecture/cqrs.md), [domain-events](docs/architecture/domain-events.md), [entities](docs/architecture/entities.md) |
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
