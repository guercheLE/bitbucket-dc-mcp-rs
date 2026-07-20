# Bitbucket workflow: repositories

Covers repository lifecycle (create, update, delete, recreate, fork),
browsing contents (files, raw, readme, license, archive, related repos),
and repository settings.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I create a repository?", "how do
I browse a file in a repository?"), then `get` the resolved operation to
confirm its actual path, method, and current schema before calling it.
Never assume a response field name — read the schema `get` returns.

## Gotchas

- A repository's pull-request/merge configuration (required approvals,
  required builds, merge strategy) is covered in
  `bitbucket_workflow_pull_requests`'s Step 1, not here — fetch that
  prompt if the user's real goal is about merging, not the repository
  resource itself.
- Deleting a repository is not immediately reversible from this API in
  every configuration — confirm with the user before calling a delete
  operation, and prefer checking for existing content/forks first.

This is a dual-resource CRUD domain (project + repository) — most
requests here are a single `search` → `get` → `call`, not a multi-step
guided flow.
