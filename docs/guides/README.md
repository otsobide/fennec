# Guides

Step-by-step recipes for extending Fennec. Both guides build inside-out
(domain, application, infrastructure, tests, delivery) and assume the patterns
described in [../architecture/](../architecture/README.md).

- [adding-a-bounded-context.md](adding-a-bounded-context.md): create a new
  library crate under `libs/`, from the first value object to the wiring in an
  app.
- [adding-an-app.md](adding-an-app.md): create a new thin HTTP service under
  `apps/`, with its routes, wiring and e2e suite.

Both recipes end the same way: the new crate is registered in the workspace
root `Cargo.toml`, its test target points at the external `tests/` tree, and
`cargo test --workspace` is green.
