# Architecture

How the system is designed: Domain-Driven Design, Hexagonal Architecture
(Ports and Adapters), CQRS, and domain events, implemented as a Cargo
workspace of bounded contexts behind thin HTTP apps.

## Documents

- [overview.md](overview.md): the layer diagram, the dependency rule, the
  request flow, and the patterns used throughout the codebase.
- [project-structure.md](project-structure.md): the annotated file tree, from
  the workspace root down to the inside of an aggregate module.
- [cqrs.md](cqrs.md): how commands and queries travel from a controller to a
  handler, and how responses come back.
- [domain-events.md](domain-events.md): event structs, naming, factories, and
  the event bus that connects bounded contexts.
- [entities.md](entities.md): every aggregate and the value objects that
  compose it, with invariants and errors.

## Non-negotiables

1. **Dependencies point inward.** HTTP depends on the buses, the buses on the
   application layer, the application layer on the domain. Infrastructure
   implements domain traits. The domain imports no framework, no driver, no
   serialization crate.
2. **Every aggregate field is a value object**, timestamps included. Values
   that arrive from outside are validated at construction and report failures
   through the shared `ValueObjectValidationError`.
3. **Business failures are data.** Handlers return a response envelope whose
   `error.concept` the delivery layer maps to an HTTP status. Bus errors are
   reserved for wiring bugs.
4. **Contexts communicate only through domain events.** They never import each
   other's crates; aggregates in different contexts relate by shared
   identifier only.
5. **Events are published only after a successful write**, and always built
   through their factory function.
