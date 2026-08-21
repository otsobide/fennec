# Architecture

## Overview

Fennec is built around four ideas that compose well in a Rust microservice codebase:

- **Domain-Driven Design (DDD)** — model the CTI domain explicitly; keep domain logic isolated from I/O.
- **Hexagonal Architecture (Ports & Adapters)** — depend on traits, not concrete implementations; swap infrastructure without touching business logic.
- **CQRS** — separate write paths (commands) from read paths (queries) so each can evolve independently.
- **Domain Events** — propagate side-effects through a decoupled event bus instead of direct calls.

The result is a layered architecture where each layer has a single responsibility and depends only on layers beneath it.

### Why this architecture?

A CTI platform has to integrate with many heterogeneous systems (feeds, sandboxes, EDRs, ticketing) and stay correct under heavy concurrent ingestion. Domain-Driven Design keeps the domain isolated — the rules of threat intelligence (sources, indicators, sightings, actors) are expressed in plain Rust types and traits, free of any framework or database coupling. When the domain changes, you modify domain code. When the database changes, you modify infrastructure code. The two never bleed into each other.

The Hexagonal Architecture (Ports & Adapters) layer on top of DDD solves a different problem: infrastructure flexibility. By depending on traits (ports) rather than concrete implementations, the system can run with PostgreSQL in production and simple in-memory HashMaps in tests — same business logic, zero code changes. Swapping the persistence backend means writing a new adapter, not editing the domain or application code.

CQRS rounds out the architecture by acknowledging that reads and writes are fundamentally different operations. Commands change state and publish domain events; queries fetch data and return it. Separating these concerns keeps each handler focused on a single responsibility and makes it straightforward to optimise read and write paths independently as the system scales.

---

The following diagram shows how the layers relate. The key principle is the **dependency rule**: inner layers never know about outer layers. Domain code has zero dependencies on frameworks, databases, or HTTP — meaning all business logic can be tested without any infrastructure.

## Layer diagram

```
┌─────────────────────────────────────────────────────┐
│                   HTTP (actix-web)                  │  <- apps/
│        Controllers: deserialise, dispatch, respond  │
├──────────────────────┬──────────────────────────────┤
│    Command Bus       │       Query Bus              │  <- shared/cqrs
│  (write operations)  │   (read operations)          │
├──────────────────────┴──────────────────────────────┤
│              Application Services                   │  <- libs/*/application/
│  Creator . Finder . Updater . Deleter               │
│  Orchestrates: repo + event bus                     │
├─────────────────────────────────────────────────────┤
│                   Domain                            │  <- libs/*/domain/
│  Aggregates . Value Objects . Events . Repo traits  │
├─────────────────────────────────────────────────────┤
│              Infrastructure                         │  <- libs/*/infrastructure/
│  InMemory / PostgreSQL repositories                 │
└─────────────────────────────────────────────────────┘
```

**Dependency rule:** arrows point inward only. Domain knows nothing outside itself. Application depends on domain. Infrastructure depends on domain and application. Apps depend on everything but are never imported by libs.

## Layers

### HTTP layer (`apps/`)

Actix-Web handlers are thin adapters:

1. Deserialise the HTTP request into a typed command or query struct (validating value objects up-front returns 400 before the bus is even touched).
2. Dispatch it through the bus.
3. Downcast the `Box<dyn Any>` response and check the error envelope.
4. Map the result to an HTTP response (status code + optional JSON body).

Handlers know nothing about the domain internals. All business logic lives in the application layer.

In `cti_api`, handlers live under `apps/cti_api/src/<resource>/controllers/{post,get,put,delete}.rs` and the request DTOs under `apps/cti_api/src/<resource>/request_dtos/`. `config_api` predates this convention and keeps the verb files directly under the resource folder — both layouts are functionally equivalent; new apps should adopt the `controllers/` + `request_dtos/` form.

### Command Bus (`libs/shared/cqrs`)

Commands represent intent to change state. A command is a plain struct; the bus routes it to the one handler registered for that type. Both `CommandHandler` and `QueryHandler` declare a `type Response` associated type. The bus boxes this response as `Box<dyn Any + Send + Sync>`, and the caller downcasts it back to the concrete type.

All handlers use a **response envelope pattern**: responses carry an `error: Option<SourceErrorEntry>` field (the exact `ErrorEntry` type varies per context). Domain errors become `SourceErrorEntry { message, concept }` where `concept` is a PascalCase string like `"NotFound"`, `"AlreadyExists"`, or `"Unexpected"`. The envelope pattern means that domain errors (like "not found" or "already exists") never bubble up as Rust `Err` variants to the HTTP layer. Instead, they travel inside the response struct as structured data, making it easy for controllers to map them to the appropriate HTTP status code without catching exceptions or matching on error types.

```
POST /sources  ->  CreateSourceCommand  ->  CreateSourceCommandHandler
                                              └─ SourceCreator.execute()
                                                  ├─ repo.save()
                                                  └─ event_bus.publish()
```

### Query Bus (`libs/shared/cqrs`)

Queries retrieve data without side effects. The handler returns a typed response that the bus boxes and the caller downcasts.

```
GET /sources/{id}  ->  FindSourceQuery  ->  FindSourceQueryHandler
                                              └─ SourceFinder.execute()
                                                  └─ repo.find_by_id()
```

### Application Services (`libs/*/application/`)

One service per use case (creator, finder, updater, deleter). Each service:

- Accepts typed value objects, not raw strings.
- Calls the repository trait (not a concrete type).
- Publishes domain events after a successful state change.
- Returns a typed result or a domain error.

Services are pure business logic — no HTTP, no SQL, no framework.

### Domain (`libs/*/domain/`)

The domain layer defines the rules of the business:

- **Aggregates** — root entities owning invariants (`Source`, `ConfigEntry`).
- **Value Objects** — immutable wrappers around every attribute, including timestamps (`SourceId`, `SourceType`, `SourceStatus`, `SourceCreatedAt`, `SourceUpdatedAt`).
- **Repository traits** — async interfaces declared here, implemented in infrastructure.
- **Domain Events** — facts that happened (`SourceCreatedEvent`, `SourceUpdatedEvent`, `SourceDeletedEvent`).
- **Errors** — domain-specific error types (`SourceRepositoryError`).

Nothing in the domain imports from `actix-web`, `sqlx`, or any other framework.

### Infrastructure (`libs/*/infrastructure/`)

Concrete repository implementations. Today every bounded context ships an in-memory `HashMap`-backed repo behind a `Mutex`. A PostgreSQL implementation can be added under `infrastructure/persistence/sqlx_postgres/` without touching any other layer.

## Dependency direction

```
apps/cti_api
    └── libs/kernel              (domain + application + infrastructure)
    └── libs/shared/cqrs
    └── libs/shared/domain-events

apps/config_api
    └── libs/config
    └── libs/shared/cqrs
    └── libs/shared/domain-events

libs/kernel
    └── libs/shared/cqrs
    └── libs/shared/domain-events
```

Dependencies flow inward. The domain never imports application code; application never imports HTTP code.

## Bounded Contexts

A bounded context is a self-contained module with its own domain model, its own error types, and its own persistence. In practical terms, it is a Cargo crate under `libs/` that owns everything about one area of the domain — entities, value objects, repository traits, events, and errors. Bounded contexts communicate with each other only through domain events, never by importing each other's internals. This isolation means you can modify, test, or even replace an entire context without affecting the rest of the system.

```
libs/
├── kernel/     <- Source bounded context (start of the CTI domain)
├── config/     <- ConfigEntry bounded context (reference example)
└── shared/     <- Cross-cutting infrastructure (no domain logic)
```

`kernel` is the seed of the CTI domain — it currently owns the common attributes of any intelligence source. As Fennec grows, type-specific bodies (URL feed contents, RSS payloads, vendor-specific schemas) will move into their own bounded contexts that subscribe to events from `kernel`.

When two bounded contexts need to react to each other's events, the subscriber lives in the _consuming_ context, not the _producing_ one.

### Internal structure of a bounded context

```
<context>/
├── domain/
│   ├── entities/          # Aggregate roots and entities
│   ├── value_objects/     # Typed wrappers enforcing invariants
│   ├── repositories/      # Repository traits (interfaces only — no impl)
│   ├── events/            # Domain event types and factory functions
│   └── errors/            # Repository and domain error enums
│
├── application/           # Use cases
│   ├── <verb>_<noun>/     # One folder per use case
│   │   ├── <noun>_<verb>er.rs              # Domain service (returns domain entities)
│   │   ├── <verb>_<noun>_command.rs        # Command struct (for writes)
│   │   ├── <verb>_<noun>_query.rs          # Query struct (for reads)
│   │   ├── <verb>_<noun>_response.rs       # Response DTO with error envelope
│   │   └── <verb>_<noun>_command_handler.rs / _query_handler.rs
│   └── on_<event>_<reaction>/  # Domain event subscribers
│
└── infrastructure/
    └── persistence/
        └── in_memory/     # HashMap-based in-memory implementations
```

---

## Request flow — example: POST /sources

```
1. HTTP POST /sources
   └── source::controllers::post::handler(state, body)
         │  deserialises JSON -> CreateSourceRequest
         │  validates the UUID, source_type and status (returns 400 on failure)
         │
2.       └── state.command_bus.dispatch(Box::new(CreateSourceCommand { id, source_type, status, description }))
               │  returns Result<Box<dyn Any + Send + Sync>, CommandBusError>
               │
3.             └── CreateSourceCommandHandler::handle(cmd)
                     │  type Response = CreateSourceResponse
                     │
4.                   └── SourceCreator::execute(id, source_type, status, description)
                           │  generates SourceCreatedAt::now() and a matching SourceUpdatedAt
                           │
5.                         ├── SourceRepository::save(&source)
                           │     └── InMemorySourceRepository::save
                           │
6.                         └── EventBus::publish([SourceCreatedEvent])

7. Handler returns Ok(CreateSourceResponse { error: None })
8. Controller downcasts Box<dyn Any> -> CreateSourceResponse
9. Checks response.error -> None -> HTTP 201 Created
   (if error.concept == "AlreadyExists" -> HTTP 409 Conflict)
```

---

## Key patterns

The following patterns appear throughout the codebase. Understanding them will help you navigate and extend any bounded context.

### Value Objects

Every domain primitive is a typed wrapper — no raw `String`s, `Uuid`s, or `SystemTime`s in aggregates. This applies even to timestamps and enums: `SourceCreatedAt` wraps a `SystemTime`, `SourceType` is an enum with a validating `from_str`.

Invariants are enforced **at construction**: every value that reaches the domain from outside arrives through a fallible constructor, so once a value object exists it is valid. Failures are reported through the single shared `ValueObjectValidationError` — no value object defines an error type of its own.

```
SourceId(Uuid)                  -- new(&str) / from_uuid(Uuid) -> Result, must be a UUID v4
                                -- generate() mints a fresh one platform-side
SourceType { Url }              -- from_str -> Result, case- and whitespace-insensitive
SourceStatus { Active, Inactive }
SourceDescription(String)       -- new(..) -> Result, trimmed, <= 1024 characters
SourceCreatedAt(SystemTime)     -- generated inside the domain service, infallible
SourceUpdatedAt(SystemTime)
```

Timestamps are the only newtypes left without validation: they are produced inside the domain services, never parsed from input.

Generic primitives (`StringValueObject` and the shared `ValueObjectValidationError`) live in `shared-valueobject/domain/` and are composed into context-specific value objects via the newtype pattern.

### Entities

Structs that hold only value objects — no raw primitives as fields. Constructed via `::new(...)`, accessed via getters. The identifier is immutable by convention.

### Repository pattern

Traits defined in `domain/repositories/`, implemented in `infrastructure/persistence/`. Consumers depend on the trait, never the concrete type. This inversion of control is what allows the same business logic to run against PostgreSQL in production and an in-memory `HashMap` in tests, with zero conditional compilation or feature flags.

```rust
// Domain — the contract
#[async_trait]
pub trait SourceRepository: Send + Sync {
    async fn save(&self, source: &Source) -> Result<(), SourceRepositoryError>;
    async fn find_by_id(&self, id: &SourceId) -> Result<Option<Source>, SourceRepositoryError>;
    async fn update(&self, source: &Source) -> Result<(), SourceRepositoryError>;
    async fn delete(&self, id: &SourceId) -> Result<(), SourceRepositoryError>;
}

// Infrastructure — the implementation
pub struct InMemorySourceRepository { /* HashMap<Uuid, Source> behind a Mutex */ }
```

### CQRS

`shared-cqrs` provides `CommandBus` and `QueryBus`. Both buses dispatch to handlers by `TypeId` and return `Result<Box<dyn Any + Send + Sync>, BusError>`. Handlers declare a `type Response` associated type — the bus boxes it and callers downcast.

All handlers use a **response envelope pattern**: responses carry an `error: Option<NounErrorEntry>` field. Query responses also carry an optional data field. Domain errors become `NounErrorEntry { message, concept }` where `concept` is a PascalCase string like `"NotFound"`, `"AlreadyExists"`, or `"Unexpected"`. The envelope pattern means that domain errors never bubble up as Rust `Err` variants to the HTTP layer. Instead, they travel inside the response struct as structured data, making it easy for controllers to map them to the appropriate HTTP status code without catching exceptions or matching on error types.

**Domain services (finders)** return domain entities — the handler maps them to DTOs. This keeps use cases simple and free of response-building logic.

See [CQRS.md](CQRS.md) for details.

### Domain Events

After a state change succeeds, the domain service publishes events to the `EventBus`. Subscribers in the `application/` layer react to these events asynchronously.

This decouples the producing use case from the consuming side effect. Neither knows the other exists — they communicate only through events.

See [DOMAIN_EVENTS.md](DOMAIN_EVENTS.md) for details.

### Logging

`tracing` + `tracing-subscriber` throughout. Initialised in each app's `main.rs` with `EnvFilter`. Override log level with `RUST_LOG`.

| Level | When to use |
|---|---|
| `debug!` | Request entry points, intermediate steps, dev diagnostics |
| `info!`  | Successful completion of important operations (created, updated, deleted) |
| `warn!`  | Expected error paths (not found, invalid input, operation failed) |

---

## build_state() and dependency wiring

Dependency injection in Fennec is manual and explicit — there is no DI framework or macro magic. Each app's `lib.rs` contains a `build_state()` function that wires everything together: it creates repository instances, builds domain services with their dependencies, creates command/query handlers, and registers them on the buses. This approach is verbose but transparent — you can trace any dependency by reading one function.

All dependency wiring happens in `build_state()` in each app's `lib.rs`. This function:

1. Constructs infrastructure implementations (repositories, event bus).
2. Builds domain services, injecting repositories and event buses.
3. Builds command and query handlers, injecting domain services.
4. Registers handlers on the command and query buses.
5. Wraps everything in `web::Data<AppState>` for Actix-Web.

```rust
pub fn build_state() -> web::Data<AppState> {
    let repo: Arc<dyn SourceRepository> = Arc::new(InMemorySourceRepository::new());
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    // Domain services
    let creator = SourceCreator::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let finder  = SourceFinder::new(Arc::clone(&repo));
    let updater = SourceUpdater::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let deleter = SourceDeleter::new(Arc::clone(&repo), Arc::clone(&event_bus));

    // Command handlers
    let mut command_bus = InMemoryCommandBus::new();
    command_bus.register(CreateSourceCommandHandler::new(creator))
        .expect("Failed to register CreateSourceCommandHandler");
    command_bus.register(UpdateSourceCommandHandler::new(updater))
        .expect("Failed to register UpdateSourceCommandHandler");
    command_bus.register(DeleteSourceCommandHandler::new(deleter))
        .expect("Failed to register DeleteSourceCommandHandler");

    // Query handlers
    let mut query_bus = InMemoryQueryBus::new();
    query_bus.register(FindSourceQueryHandler::new(finder))
        .expect("Failed to register FindSourceQueryHandler");

    web::Data::new(AppState {
        command_bus: Arc::new(command_bus),
        query_bus:   Arc::new(query_bus),
    })
}
```

`configure_routes()` in the same file registers all HTTP handlers:

```rust
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get::handler)
        .service(source::controllers::post::handler)
        .service(source::controllers::get::handler)
        .service(source::controllers::put::handler)
        .service(source::controllers::delete::handler);
}
```

Because `build_state()` takes no arguments (for in-memory backends), calling it once per test gives each test a fully isolated state — no shared mutable state, no database teardown.

---

## File and module conventions

- **One public type per file.** Each struct, enum, or trait gets its own `.rs` file.
- **File name mirrors the type name in `snake_case`.** `source.rs` -> `struct Source`.
- **`mod.rs` files only declare submodules** — no logic.
- **Folder names use `snake_case`**, package names in `Cargo.toml` use kebab-case (`cti-api`, `config-api`).
- **Test files live in `tests/<crate>/`** and mirror the source module tree.
