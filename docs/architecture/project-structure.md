# Project Structure

Full file tree with the purpose of every directory and key file.

This document is the navigational map of the Fennec repository: every folder and every notable file is annotated with its role. Use it when you first encounter the codebase and as a reference whenever you need to find where something lives.

The repository is organised into three top-level areas:

- `apps/` — Actix-Web HTTP services that users interact with.
- `libs/` — business logic, one crate per bounded context, plus shared infrastructure. The `kernel` context hosts one module per aggregate; see [the module rule](#the-module-rule) below.
- `tests/` — external test crates that mirror the production structure.

```
.
├── Cargo.toml                  # Workspace root — lists all member crates
├── Cargo.lock                  # Pinned dependency versions (commit this)
├── Makefile                    # Root Makefile (delegates to per-app Makefiles via $(MAKE) -C)
├── docker-compose.yml          # PostgreSQL scaffolding (not yet integrated)
├── .gitignore                  # Ignores target/, CLAUDE.md, .claude/, ...
│
├── apps/
│   ├── cti_api/                # CTI HTTP API (port 8081) — exposes every kernel module
│   │   ├── Cargo.toml          # Package: cti-api, declares [lib] + [[bin]] + [[test]]
│   │   ├── Makefile            # Per-app build/run/test targets
│   │   └── src/
│   │       ├── main.rs         # Binary entry point: init tracing, build_state, HttpServer
│   │       ├── lib.rs          # Library root: AppState, build_state(), configure_routes()
│   │       ├── health/
│   │       │   ├── mod.rs
│   │       │   └── get.rs      # GET /health -> 200
│   │       ├── source/
│   │           ├── mod.rs      # pub mod controllers; request_dtos;
│   │           ├── controllers/
│   │           │   ├── post.rs    # POST   /sources       -> 201 | 400 | 409
│   │           │   ├── get.rs     # GET    /sources/{id}  -> 200 | 400 | 404
│   │           │   ├── put.rs     # PUT    /sources/{id}  -> 200 | 400 | 404
│   │           │   └── delete.rs  # DELETE /sources/{id}  -> 204 | 400 | 404
│   │           └── request_dtos/
│   │               ├── create_source_request.rs   # { id, source_type, status, description }
│   │               └── update_source_request.rs   # { status, description }
│   │       ├── url_source/     # Same shape: controllers/ + request_dtos/
│   │       ├── ioc/            # POST /iocs, GET|DELETE /iocs/{id}
│   │       └── sighting/       # /sightings, /sightings/{id}/observations,
│   │                           # /iocs/{ioc_id}/sightings, /sources/{source_id}/sightings
│   │
│   └── config_api/             # Reference HTTP API (port 8080) — exposes the config BC
│       ├── Cargo.toml          # Package: config-api
│       ├── Makefile
│       └── src/
│           ├── main.rs
│           ├── lib.rs
│           ├── health/get.rs
│           └── config_entry/
│               ├── post.rs / get.rs / put.rs / delete.rs
│               ├── create_config_entry_request.rs
│               └── update_config_entry_request.rs
│
├── libs/
│   ├── kernel/                 # CTI bounded context: one module per aggregate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # pub mod ioc; sighting; source; url_source;
│   │       ├── source/         # Expanded below as the reference module
│   │           ├── mod.rs      # pub mod application; domain; infrastructure;
│   │           │
│   │           ├── domain/
│   │           │   ├── entities/
│   │           │   │   └── source.rs                   # Source aggregate
│   │           │   ├── value_objects/
│   │           │   │   ├── source_id.rs                # SourceId(Uuid) — externally provided
│   │           │   │   ├── source_type.rs              # enum SourceType { Url } + from_str
│   │           │   │   ├── source_status.rs            # enum SourceStatus { Active, Inactive }
│   │           │   │   ├── source_description.rs       # SourceDescription(String)
│   │           │   │   ├── source_created_at.rs        # SourceCreatedAt(SystemTime)
│   │           │   │   └── source_updated_at.rs        # SourceUpdatedAt(SystemTime)
│   │           │   ├── errors/
│   │           │   │   └── source_repository_error.rs  # NotFound | AlreadyExists | Unexpected
│   │           │   ├── repositories/
│   │           │   │   └── source_repository.rs        # async trait (save/find_by_id/update/delete)
│   │           │   └── events/
│   │           │       ├── source_created_event.rs
│   │           │       ├── source_updated_event.rs     # carries old + new status / description
│   │           │       ├── source_deleted_event.rs
│   │           │       └── create_source_*_event.rs    # factory fns
│   │           │
│   │           ├── application/
│   │           │   ├── create_source/
│   │           │   │   ├── create_source_command.rs            # Command struct
│   │           │   │   ├── create_source_response.rs           # { error: Option<SourceErrorEntry> }
│   │           │   │   ├── source_creator.rs                   # Domain service (auto-sets timestamps)
│   │           │   │   └── create_source_command_handler.rs    # CommandHandler impl
│   │           │   ├── find_source/
│   │           │   │   ├── find_source_query.rs
│   │           │   │   ├── find_source_response.rs             # { source: Option<SourceEntry>, error }
│   │           │   │   ├── source_finder.rs
│   │           │   │   └── find_source_query_handler.rs
│   │           │   ├── update_source/
│   │           │   │   ├── update_source_command.rs            # only status + description (type immutable)
│   │           │   │   ├── update_source_response.rs
│   │           │   │   ├── source_updater.rs                   # regenerates updated_at
│   │           │   │   └── update_source_command_handler.rs
│   │           │   └── delete_source/
│   │           │       ├── delete_source_command.rs
│   │           │       ├── delete_source_response.rs
│   │           │       ├── source_deleter.rs
│   │           │       └── delete_source_command_handler.rs
│   │           │
│   │           └── infrastructure/
│   │               └── persistence/
│   │                   └── in_memory/
│   │                       └── in_memory_source_repository.rs  # HashMap<Uuid, Source> + Mutex
│   │       │
│   │       ├── url_source/     # UrlSource: url, format, polling_interval  (same three layers)
│   │       ├── ioc/            # Ioc: ioc_type, value                      (same three layers)
│   │       └── sighting/       # Sighting: ioc_id, source_id, first/last seen, count
│   │
│   ├── config/                 # Reference bounded context — ConfigEntry CRUD
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── config_entry/   # Same layout as kernel/source/ above
│   │
│   └── shared/
│       ├── cqrs/               # CQRS building blocks
│       │   └── src/
│       │       ├── command/
│       │       │   ├── domain/                # Command, CommandBus, CommandHandler, errors
│       │       │   └── infrastructure/in_memory/in_memory_command_bus.rs
│       │       └── query/
│       │           ├── domain/                # Query, QueryBus, QueryHandler, errors
│       │           └── infrastructure/in_memory/in_memory_query_bus.rs
│       │
│       ├── domain-events/      # Event bus building blocks
│       │   └── src/
│       │       ├── domain/
│       │       │   ├── domain_event.rs             # DomainEvent trait + DomainEventBase
│       │       │   ├── domain_event_subscriber.rs  # fn on(&self, event: &E) -> Result
│       │       │   ├── event_bus.rs                # publish(Vec<Box<dyn DomainEvent>>)
│       │       │   └── event_bus_error.rs
│       │       └── infrastructure/in_memory/in_memory_event_bus.rs
│       │
│       └── valueobject/        # Reusable value object primitives
│           └── src/
│               └── domain/
│                   ├── strings/string_value_object.rs    # StringValueObject(String)
│                   └── errors/value_object_validation_error.rs
│
├── tests/
│   ├── apps/
│   │   ├── cti_api/            # E2E tests for cti_api
│   │   │   ├── tests.rs
│   │   │   └── src/
│   │   │       ├── mod.rs
│   │   │       ├── health/health_test.rs
│   │   │       └── source/
│   │   │           ├── create_source_test.rs
│   │   │           ├── find_source_test.rs
│   │   │           ├── update_source_test.rs
│   │   │           └── delete_source_test.rs
│   │   │
│   │   └── config_api/         # E2E tests for config_api (mirrors cti_api layout)
│   │
│   └── libs/
│       ├── kernel/             # Unit tests for libs/kernel (all four modules)
│       │   ├── tests.rs
│       │   └── src/
│       │       ├── module_isolation_tests.rs       # Fails if a module imports a sibling
│       │       ├── mocks/
│       │       │   ├── source_repository_mock.rs   # Configurable behavior enums
│       │       │   ├── url_source_repository_mock.rs
│       │       │   ├── ioc_repository_mock.rs
│       │       │   ├── sighting_repository_mock.rs
│       │       │   └── event_bus_mock.rs           # Records published events
│       │       ├── url_source/  # Mirrors the source/ layout below
│       │       ├── ioc/
│       │       ├── sighting/
│       │       └── source/
│       │           ├── domain/
│       │           │   ├── entities/mothers/source_mother.rs
│       │           │   └── value_objects/mothers/
│       │           │       ├── source_id_mother.rs
│       │           │       ├── source_type_mother.rs
│       │           │       ├── source_status_mother.rs
│       │           │       └── source_description_mother.rs
│       │           └── application/
│       │               ├── create_source/source_creator_tests.rs
│       │               ├── find_source/source_finder_tests.rs
│       │               ├── update_source/source_updater_tests.rs
│       │               └── delete_source/source_deleter_tests.rs
│       │
│       └── config/             # Unit tests for libs/config (mirrors kernel layout)
│
└── docs/                       # One folder per topic, each with a README.md
    ├── README.md
    ├── architecture/
    │   ├── README.md
    │   ├── overview.md
    │   ├── project-structure.md    # <-- this file
    │   ├── cqrs.md
    │   ├── domain-events.md
    │   └── entities.md
    ├── development/
    │   ├── README.md
    │   ├── gitflow.md
    │   └── testing.md
    └── guides/
        ├── README.md
        ├── adding-a-bounded-context.md
        └── adding-an-app.md
```

---

## The module rule

`libs/` holds one crate per bounded context, but a context is not one crate per
aggregate: `kernel` hosts `source`, `url_source`, `ioc` and `sighting` as
sibling modules, and `config` hosts the reference `config_entry` module.

What makes that safe is that a module is a complete unit on its own:

- It carries its own `domain/`, `application/` and `infrastructure/` trees.
- It declares its own value object for any foreign identifier
  (`SightingIocId`, not `IocId`), so it never needs a sibling's types.
- It talks to the rest of the system only through the buses: commands and
  queries for anything a caller drives, domain events for anything it reacts to.

So promoting a module to its own crate is a move plus a `Cargo.toml`, not a
redesign: the day `sighting` needs to scale on its own, nothing has to be
untangled first. `tests/libs/kernel/src/module_isolation_tests.rs` fails the
build if a module reaches into a sibling, so the property cannot rot silently.

---

## App folder convention

Apps follow a slight variation of the bounded-context layout: HTTP handlers live under `<resource>/controllers/{post,get,put,delete}.rs` and the request DTOs under `<resource>/request_dtos/`. This keeps each resource module clean — one file per HTTP verb, DTOs side by side — and scales naturally when a single app exposes several aggregates.

```
apps/<api>/src/<resource>/
  controllers/
    post.rs / get.rs / put.rs / delete.rs
  request_dtos/
    create_<resource>_request.rs
    update_<resource>_request.rs
```

`cti_api` uses this convention. `config_api` predates it and keeps the verb files directly under the resource folder — both layouts are equivalent in behaviour; new apps should adopt the `controllers/` + `request_dtos/` form.

---

## Use case directory structure

Within each bounded context, use cases follow a consistent directory layout. Once you know the pattern, you can predict exactly where to find any command handler, query response, or domain service.

### Command use case (write)

```
create_source/
├── create_source_command.rs            # Command struct
├── create_source_response.rs           # { error: Option<SourceErrorEntry> }
├── source_creator.rs                   # Domain service -> Result<(), DomainError>
└── create_source_command_handler.rs    # CommandHandler impl with type Response
```

### Query use case (read)

```
find_source/
├── find_source_query.rs                # Query struct
├── find_source_response.rs             # { source: Option<SourceEntry>, error: Option<SourceErrorEntry> }
├── source_finder.rs                    # Domain service -> Result<Source, DomainError>
└── find_source_query_handler.rs        # QueryHandler impl — maps entity to DTO
```

---

## Naming conventions

The conventions below are strictly followed throughout the codebase. They serve two purposes: making it easy to find any type by name (convert the type name to snake_case and you have the file name), and making each type's role obvious from its name.

| Concept | Rust name | Example |
|---|---|---|
| Aggregate | `PascalCase` struct | `Source`, `ConfigEntry` |
| Value Object | `PascalCase` struct/enum | `SourceId`, `SourceType`, `SourceStatus` |
| Command | `VerbNounCommand` | `CreateSourceCommand` |
| Query | `VerbNounQuery` | `FindSourceQuery` |
| Domain Service | `NounVerber` | `SourceCreator`, `SourceFinder` |
| Command Handler | `VerbNounCommandHandler` | `CreateSourceCommandHandler` |
| Query Handler | `VerbNounQueryHandler` | `FindSourceQueryHandler` |
| Command Response | `VerbNounResponse` | `CreateSourceResponse` |
| Query Response | `VerbNounResponse` | `FindSourceResponse` |
| Data Entry DTO | `NounEntry` | `SourceEntry` |
| Error Entry DTO | `NounErrorEntry` | `SourceErrorEntry` |
| Domain Event | `NounPastEvent` | `SourceCreatedEvent` |
| Event factory fn | `create_noun_past_event` | `create_source_created_event` |
| Domain Error | `NounRepositoryError` | `SourceRepositoryError` |
| Repository trait | `NounRepository` | `SourceRepository` |
| In-memory impl | `InMemoryNounRepository` | `InMemorySourceRepository` |
| Postgres impl (future) | `SqlxPostgresNounRepository` | `SqlxPostgresSourceRepository` |

---

## File naming conventions

Files follow a one-type-per-file rule. The file name is always the snake_case version of the primary type it contains, which makes any type trivially locatable — if you are looking for `CreateSourceCommandHandler`, it is in `create_source_command_handler.rs`.

Examples:

- `source.rs` -> `struct Source`
- `source_repository.rs` -> `trait SourceRepository`
- `create_source_command.rs` -> `struct CreateSourceCommand`
- `create_source_response.rs` -> `struct CreateSourceResponse`
- `find_source_response.rs` -> `struct FindSourceResponse` + `struct SourceEntry` + `struct SourceErrorEntry`

Response files may contain the matching entry DTOs (data + error) so that consumers find them all together.
