# Bitbucket workflow: build and CI integration

Covers reporting and reading build status on commits, configuring
required-build merge checks on a repository, and Jira dev-panel linkage
(associating commits/branches/PRs with Jira issues for CI/deployment
visibility).

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
Confirmed directly: the id `get` alone resolves to an entirely unrelated
operation (basic-auth config) in one version and to "get a commit's build
statuses" in another. Always `search` by capability, then `get` the
resolved operation to confirm its real path, method, and current schema
before calling it — never assume a fixed id resolves to the same
operation across versions.

## Gotchas

- Required-build merge checks configured here are one of the conditions
  `bitbucket_workflow_pull_requests`'s Step 4 gates on — if the user's
  real goal is merging a PR, point them there instead of only reporting
  build status in isolation.
- Confirm the exact commit SHA (full 40-character SHA1, not an
  abbreviated form) before reporting a build status — some Bitbucket
  versions reject abbreviated SHAs for this operation.

This is a CRUD domain — most requests here are a single `search` → `get`
→ `call`, not a multi-step guided flow.
