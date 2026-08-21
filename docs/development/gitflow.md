# Gitflow

Two long-lived branches, short-lived work branches, and GitHub rulesets that
make the model mandatory rather than aspirational. This is the shared gitflow
model applied to Fennec; the cross-repository description lives in the
`.knowledge` repository.

## Branches

| Branch | Purpose |
|---|---|
| `main` | Released versions only. Changes exclusively through a merged pull request from `dev`, and every merge is tagged `vX.Y.Z`. |
| `dev` | Integration branch and the repository's default branch. All work lands here first, through a pull request. |
| `feature/<slug>` | One branch per unit of work, branched off `dev`, merged back through a pull request, then deleted. |
| `hotfix/<slug>` | An urgent fix for a released version. Mechanically a feature branch; a release follows immediately. |

Branch names use short kebab-case slugs: `feature/namespace-domain-events`,
`feature/value-object-validation`.

## Protections

Two rulesets, both `active` with empty bypass lists, so they bind the owner
too:

| Ruleset | Branch | What it enforces |
|---|---|---|
| `protect-dev` | `dev` | Pull request required, no force pushes, no deletion, and the required **gitflow branch name** check, which fails any pull request whose head branch is not `feature/*`, `hotfix/*` or `main`. |
| `protect-main` | `main` | The same base rules, plus the required **release source branch** check, which fails any pull request into `main` whose head branch is not `dev`. |

Both checks live in `.github/workflows/gitflow.yml`. The exact commands that
created the rulesets are in the `.knowledge` repository
(`development/branch-protections.md`); verify them with:

```bash
gh api repos/otsobide/fennec/rulesets --jq '.[] | {name, enforcement}'
```

## The day-to-day loop

```bash
git switch dev && git pull                # start from a fresh dev
git switch -c feature/<slug>              # one branch per unit of work
# ...work, committing small and often...
cargo test --workspace                    # green before the PR merges
git push -u origin feature/<slug>
gh pr create --base dev                   # open the pull request into dev
# ...checks run; merge when green...
gh pr merge --merge                       # merge commit, then delete the branch
```

A pull request into `dev` from a branch named anything other than
`feature/*`, `hotfix/*` or `main` cannot be merged: rename the branch and
reopen.

## Releasing

`main` is the released state. To cut a version:

1. Make sure `dev` is green and holds everything the release needs.
2. Open a pull request from `dev` into `main` and merge it.
3. Tag the merge commit and push the tag:

   ```bash
   git switch main && git pull
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

Releasing changes what users get: only do it when the task explicitly asks for
a release.

## Hotfixes

Branch `hotfix/<slug>` off `dev`, fix, open a pull request into `dev`, merge
it, and release immediately. There is no direct path into `main`: the release
source check accepts only `dev`. The price is that whatever else `dev` holds
ships with the fix, so keep `dev` releasable.

## Commit messages

```
<area>: <imperative summary>

<optional body: the why, when the diff does not make it obvious>
```

- Areas in this repository: the context or crate touched (`kernel:`, `ioc:`,
  `sighting:`, `config:`, `shared:`, `cti-api:`, `config-api:`), or `core:`
  for a change that sweeps every context, plus `docs:`, `ci:`, `chore:`,
  `build:`.
- Imperative and present tense: "add", "fix", "document".
- Keep the summary under about 72 characters and name the thing that changed.
- English, always.

Commits before August 2026 use Conventional Commits (`feat(sighting): ...`),
inherited from the skeleton this repository started from. New commits follow
the format above, which is the shared convention across repositories.

## Rules of thumb

- About to run `git commit` while on `dev` or `main`? Stop and create a
  `feature/<slug>` branch first.
- Never force-push `dev` or `main`. Force-pushing your own unmerged feature
  branch is fine.
- Never merge a pull request with failing or missing required checks.
