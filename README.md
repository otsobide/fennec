# 🦊 Fennec

A Rust-based **Cyber Threat Intelligence (CTI)** platform, modelled after projects like MISP, built on a strict Domain-Driven Design + Hexagonal Architecture + CQRS foundation.

## What is Fennec?

Fennec is a CTI platform whose goal is to ingest, normalise, correlate, and serve threat intelligence (indicators of compromise, threat actors, campaigns, sources, sightings) under a clean, microservice-friendly architecture. The codebase is organised as a Cargo workspace where each bounded context is an isolated library crate and each HTTP service is a thin Actix-Web app that wires those contexts behind CQRS buses.

The project today is **early in its life**: the `kernel` bounded context owns the `Source` aggregate (origins of intelligence — feeds, producers, external systems) with a full CRUD lifecycle exposed by `cti_api`. New contexts (IoCs, actors, events, sightings...) will plug in alongside `kernel` following exactly the same architectural recipe.

The architecture is uncompromising on one principle: business logic depends on traits, never on concrete infrastructure. The same domain code runs against an in-memory `HashMap` in tests and (eventually) a PostgreSQL store in production, with zero conditional compilation and no code changes in the domain or application layers.

## Bounded contexts

| Context | Aggregate | Status | Exposed by |
|---|---|---|---|
| `kernel` | `Source` | Full CRUD over the common fields of any intelligence source | `cti_api` (:8081) |
| `config` | `ConfigEntry` | Generic key/value store kept as a reference example | `config_api` (:8080) |

Each context lives under `libs/<context>/` and is wired into HTTP through a matching app under `apps/<context>_api/`. Cross-context communication happens **only** through domain events.

## Stack

| Component | Technology |
|---|---|
| Language | Rust 2021 |
| HTTP framework | Actix-Web 4 |
| Async runtime | Tokio |
| Persistence | In-memory (HashMap + Mutex) — pluggable via repository traits |
| Logging | `tracing` + `tracing-subscriber` (env-filtered) |
| Errors | `thiserror` |
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

Arrows point inward only. The domain layer has no knowledge of frameworks, databases, or HTTP. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the full breakdown.

### Bounded context layout

```
<context>/
  domain/
    entities/         # Aggregate roots
    value_objects/    # Typed wrappers (every attribute is a VO, including timestamps)
    repositories/     # Trait definitions only
    events/           # Domain events + factory functions
    errors/           # NotFound | AlreadyExists | Unexpected
  application/
    <verb>_<noun>/    # One folder per use case
      <noun>_<verb>er.rs              # Domain service
      <verb>_<noun>_command.rs        # Command struct (writes)
      <verb>_<noun>_query.rs          # Query struct (reads)
      <verb>_<noun>_response.rs       # Response envelope: { data?, error? }
      <verb>_<noun>_command_handler.rs / _query_handler.rs
  infrastructure/
    persistence/
      in_memory/      # HashMap-based implementations
```

## Project structure

```
fennec/
├── apps/
│   ├── cti_api/         # CTI HTTP API (port 8081) — exposes the kernel BC
│   └── config_api/      # Reference HTTP API (port 8080) — exposes the config BC
│
├── libs/
│   ├── kernel/          # CTI kernel bounded context
│   │   └── src/source/    # Source aggregate: id, type, status, description, created_at, updated_at
│   ├── config/          # Reference bounded context (config_entry CRUD)
│   └── shared/
│       ├── cqrs/            # CommandBus + QueryBus (TypeId-based dispatch)
│       ├── domain-events/   # EventBus + DomainEventSubscriber
│       └── valueobject/     # Reusable value object primitives
│
├── tests/
│   ├── apps/{cti_api,config_api}/   # E2E tests (HTTP → bus → repo)
│   └── libs/{kernel,config}/        # Unit tests (mocks + Object Mother)
│
├── docs/                # Architecture documentation
├── docker-compose.yml   # PostgreSQL scaffolding (currently unused)
├── Makefile             # Root Makefile (delegates to per-app Makefiles)
└── Cargo.toml           # Workspace root
```

A more detailed annotated tree lives in [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md).

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
make test/unit          # Unit tests only
make test/e2e           # All e2e suites
make cti_api/test/e2e   # E2E tests for cti_api only
make test/summary       # cargo test with empty-suite noise stripped
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

### Try it

```bash
# Create a source
curl -X POST http://localhost:8081/sources \
  -H 'Content-Type: application/json' \
  -d '{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "source_type": "url",
    "status": "active",
    "description": "Primary feed"
  }'

# Retrieve it
curl http://localhost:8081/sources/550e8400-e29b-41d4-a716-446655440000
```

## Documentation

The `docs/` directory contains detailed guides. Start with **ARCHITECTURE.md** for the big picture.

| Document | Description |
|---|---|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design, layer diagram, key patterns |
| [ENTITIES.md](docs/ENTITIES.md) | Every aggregate and its value objects, grouped by bounded context |
| [PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) | Full annotated file tree |
| [CQRS.md](docs/CQRS.md) | How commands and queries flow through the system |
| [DOMAIN_EVENTS.md](docs/DOMAIN_EVENTS.md) | Event-driven communication between bounded contexts |
| [TESTING.md](docs/TESTING.md) | Test strategy: mocks, Object Mother, e2e |
| [ADDING_A_BOUNDED_CONTEXT.md](docs/ADDING_A_BOUNDED_CONTEXT.md) | Step-by-step guide to adding a new domain module |
| [ADDING_AN_APP.md](docs/ADDING_AN_APP.md) | Step-by-step guide to adding a new HTTP service |
| [GIT_FLOW.md](docs/GIT_FLOW.md) | Branching strategy and commit conventions |

## Make targets

| Target | Description |
|---|---|
| `make build` | Release build (all apps) |
| `make dev/build` | Dev build |
| `make cti_api/build` | Release build for cti_api |
| `make config_api/build` | Release build for config_api |
| `make test` | Run all tests |
| `make test/unit` | Unit tests only |
| `make test/e2e` | All e2e tests |
| `make cti_api/test/e2e` | E2E tests for cti_api |
| `make cti_api/run` | Run cti_api locally |
| `make config_api/run` | Run config_api locally |
| `make format` | `cargo fmt` |
| `make audit` | Security audit via cargo-audit |
| `make docker/up` | Start containers via Docker Compose |
| `make docker/down` | Stop containers |
