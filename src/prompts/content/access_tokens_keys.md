# Bitbucket workflow: access tokens and keys

Covers personal, project-, and repository-scoped HTTP access tokens; SSH
keys (both user-associated and per-repository deploy keys); GPG keys; and
commit-signing configuration.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — confirmed
directly: `createAccessToken_1`/`createAccessToken_2` swap which one
scopes to a repository vs. a project between versions, and
`getForRepository`/`getForRepository_1` swap which one means "get a
single SSH key" vs. "list SSH keys". Always `search` by capability, then
`get` the resolved operation to confirm its real scope, path, and schema
before calling it — never assume a fixed id means the same thing across
versions.

## Gotchas

- Confirm the intended scope (user, project, or repository) before
  creating an access token — the wrong scope grants broader or narrower
  access than the user intended.
- A token's value is returned exactly once at creation time; tell the
  user to store it immediately, and never log or echo it back in a later
  summary.

This is a CRUD domain covering several related resource types — most
requests here are a single `search` → `get` → `call`, not a multi-step
guided flow.
