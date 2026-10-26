# AGENTS.md

Guidance for AI coding agents working in this repository. The shared
`.knowledge` repository (cloned at `.knowledge/`, git-ignored) holds the
binding cross-repository rules; where this file and `.knowledge` disagree,
`.knowledge` wins.

## What this is

Fennec is a Rust Cyber Threat Intelligence platform, compatible with STIX 2.1
and OpenCTI and learning from MISP, built as a Cargo workspace with DDD,
hexagonal architecture, CQRS and domain events. The CTI domain lives in one
bounded context, `libs/kernel`, persisted in one PostgreSQL database; HTTP
apps under `apps/` are thin Actix-Web adapters over command and query buses.

Read these before changing the model:

- `docs/data-model/`: non-negotiables, identity regimes, the catalogue of
  ~110 observable kinds in eight waves, one page per implemented kind.
- `docs/architecture/persistence.md` and `event-store.md`: one table per
  kind, the transactional event log, the unit of work.
- `docs/research/`: the OpenCTI, MISP and STIX studies the design comes from.

## Commands

```bash
make build / make dev/build      # release / dev build of the workspace
make test                        # unit + e2e + doc-tests, no database needed
make cti_api/run                 # cti_api on :8081, in memory
make format                      # cargo fmt

# Database tests (#[ignore]d by default), against PostgreSQL 18
docker compose up -d postgres
DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec cargo test --workspace -- --ignored
DATABASE_URL=postgres://fennec:fennec@localhost:5432/fennec make cti_api/run

# One suite or one test
cargo test -p kernel --test kernel ipv4_address
cargo test -p cti-api --test cti-api-e2e create_ipv4_address
```

- If `cargo` is not on the PATH of a non-interactive shell, prefix commands
  with `PATH="$HOME/.cargo/bin:$PATH"`.
- If Docker is not usable, any PostgreSQL 17+ server works; set
  `DATABASE_URL` and run the same commands.
- CI (`.github/workflows/test-and-build.yml`) runs `cargo test --workspace`
  and both builds on stable, beta and nightly, plus a `PostgreSQL 18` job
  that runs `cargo test --workspace -- --ignored` against a service
  container. It does not run fmt or clippy; run `cargo fmt` yourself.

## Architecture in one page

### Request flow

1. A controller (`apps/cti_api/src/<module>/controllers/<verb>.rs`, one
   handler per file) builds value objects from the request (invalid input is
   `400` right there) and dispatches a command or asks a query.
2. The bus (`libs/shared/cqrs`) routes by `TypeId` and returns
   `Box<dyn Any>`; the controller downcasts to the response type.
3. The handler calls the domain service and returns an envelope
   `{ error: Option<ErrorEntry> }` (plus data for queries). Business errors
   are data, with a PascalCase `concept` the controller maps to a status.
4. The domain service persists, then builds the event with its factory and
   publishes it. Nothing is published if the write fails.

### Wiring

`apps/cti_api/src/lib.rs` has `build_state()` (in memory, never reads the
environment, used by tests) and `build_postgres_state(pool)` (PostgreSQL
repositories, `OutboxEventBus`, `TransactionalCommandBus`). Both share one
private `build_buses`. `main.rs` picks PostgreSQL when `DATABASE_URL` is set
and runs the migrations first. A new use case is registered in
`build_buses` and its route in `configure_routes()`.

### Persistence and events

- `libs/shared/persistence-postgres` owns the event store schema
  (`event_store.domain_event`, monthly partitions), `PgUnitOfWork`
  (task-local transaction plus buffered events), `AggregateLock` (advisory
  lock per kind and id), `PgSession` (what repositories run statements on),
  `OutboxEventBus`, `PgEventStore` and `TransactionalCommandBus`.
- Every command commits its state and its events in one transaction, or
  neither. The decorator rolls back when no event was published, so every
  successful write must publish an event.
- Each crate with tables has its own migrations and history table
  (`sqlx.toml`): `libs/shared/persistence-postgres/migrations/` and
  `libs/kernel/migrations/` (`<UTC timestamp>_<description>.sql`).
- Every event implements `to_primitives()` (serialisation-free payload);
  event ids are UUID v7; names are `fennec.<context>.<module>.<past>`.

### Modules

`libs/kernel/src/<module>/{domain,application,infrastructure}` with one
module per aggregate: `source`, `url_source`, `sighting` and one per
observable kind (`ipv4_address`, `ipv6_address`, `domain_name`, `hostname`
and `email_address` so far; the catalogue lists the rest). Modules never import each other;
`tests/libs/kernel/src/module_isolation_tests.rs` fails the build if one
does, and every new module must be added to its `MODULES` list.

## Conventions that are not obvious from one file

- **No generic types.** Every observable kind is its own module with value
  objects that validate and canonicalise its exact syntax. Never add a
  `{type, value}` shape or a catch-all kind.
- **Identity.** Value-identified kinds (regime A) derive their id, a UUID v5,
  from the canonical value through the STIX algorithm in
  `shared-valueobject`; the create command carries only the value and the
  aggregate constructor derives the id. Other aggregates take a
  client-supplied UUID v4. See `docs/data-model/identity.md`.
- **References** to an observable from another module carry its module kind
  next to the id (`SightingObservableKind` + `SightingObservableId`).
- **Timestamps** are truncated to microseconds on construction so aggregates
  round-trip through `timestamptz`; client-supplied instants are bounded to
  1970..9999.
- Every aggregate field is a value object. Each file holds one public type;
  `mod.rs` files only declare submodules. Every module and public type has a
  doc comment. Services log with `tracing` (`debug!` on entry, `info!` on
  success, `warn!` on not-found).
- Naming: services `NounVerber`, use cases `VerbNoun{Command,Query,Response}`
  and `VerbNoun{Command,Query}Handler`, repositories `NounRepository` with
  `InMemoryNounRepository` and `SqlxPostgresNounRepository`.

## Tests

- Tests live outside the crates: `tests/libs/<crate>/` and
  `tests/apps/<app>/`, registered as `[[test]]` targets, so they only see the
  public API. In-test paths look like `crate::src::...`.
- Unit tests use hand-written mocks and Object Mothers (`random()` and
  `create(...)`). Every use case gets the standard service tests including
  persistence failure paths; every value object tests every accepted
  spelling, every rejection and both sides of every boundary.
- Each repository has one contract
  (`tests/libs/kernel/src/<module>/infrastructure/persistence/<module>_repository_contract.rs`)
  that both the in-memory and the PostgreSQL adapters run.
- Database tests are `#[sqlx::test(migrator = "...")]` plus
  `#[ignore = "requires DATABASE_URL"]`; each gets a fresh database.

## Adding an observable kind

Copy the `ipv4_address` module as the template and follow
`docs/data-model/observable-kinds.md` ("The module contract"). Build it
inside out, one commit per step: value objects, aggregate and port, events,
each use case, in-memory repository, tests, table migration and PostgreSQL
repository with the contract, delivery and e2e tests, then the kind page
under `docs/data-model/kinds/` and its status in the catalogue.

## Git

- Gitflow: `feature/<slug>` off `dev`, pull request into `dev`, merge only
  with green checks (`gh pr merge --merge --delete-branch`). Releases are a
  pull request from `dev` into `main` followed by a `vX.Y.Z` tag. Never push
  to `dev` or `main` directly, never force-push them.
- Commits are small and use `<area>: <imperative summary>` with areas such as
  `kernel:`, `shared:`, `cti-api:`, `tests:`, `docs:`, `ci:`, `chore:`.
- The user wants a five-minute pause (`sleep 300`, run in the background)
  after every commit.
