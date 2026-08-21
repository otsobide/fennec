# Development

How this codebase is worked on day to day.

- [gitflow.md](gitflow.md): the branching model, the GitHub rulesets that
  enforce it, the release flow, and the commit message format.
- [testing.md](testing.md): the two test suites (unit and e2e), the mocks and
  object mothers they use, and what every new use case must cover.

## Non-negotiables

1. **Never commit or push directly to `dev` or `main`.** Work travels on a
   `feature/<slug>` (or `hotfix/<slug>`) branch and lands through a pull
   request. The rulesets reject direct pushes for everyone, owner included.
2. **Small commits, made while you work.** One logical change per commit, not
   one commit at the end of the task.
3. **Green at merge time.** `cargo test --workspace` passes before a pull
   request is merged, so every commit on `dev` and `main` works.
4. **Everything in English**: code, comments, docs, commit messages, CI.
5. **Tests live outside the crates**, wired in through a `[[test]]` target. A
   new context or app without that entry is a context whose tests never run.

The cross-repository version of these rules lives in the shared `.knowledge`
repository; this page records how they apply to Fennec.
