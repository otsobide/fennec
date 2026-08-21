# Documentation

How Fennec is built and how it is worked on. One folder per topic, each with
its own `README.md` as the entry point.

- [architecture/](architecture/README.md): how the system is designed. The
  layer rule, the CQRS buses, domain events, the annotated file tree, and the
  catalogue of every aggregate and value object.
- [development/](development/README.md): how the codebase is worked on. The
  branching model with its GitHub rulesets, and the test strategy.
- [guides/](guides/README.md): step-by-step recipes for extending the system
  with a new bounded context or a new HTTP app.

Start with [architecture/overview.md](architecture/overview.md) for the big
picture.

## Cross-repository conventions

Anything not specific to Fennec (the English-only rule, the small-commit
discipline, the gitflow model and the DDD service architecture as they apply
across repositories) lives in the shared `.knowledge` repository. Where this
documentation and `.knowledge` disagree, `.knowledge` wins, and the page here
is the one to fix.
