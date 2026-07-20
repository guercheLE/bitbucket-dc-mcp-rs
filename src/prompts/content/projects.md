# Bitbucket workflow: projects

Covers project lifecycle (create, update, delete), project settings,
project-level permissions (users, groups, public access), the project
avatar, and project restrictions.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I create a project?", "how do I
grant a group access to a project?"), then `get` the resolved operation to
confirm its actual path, method, and current schema before calling it.
Never assume a response field name — read the schema `get` returns.

## Gotchas

- A project key is immutable once set — confirm it with the user before
  creating a project, don't assume a slug-cased version of the name.
- Project-level permissions (`bitbucket_workflow_admin` covers the
  instance-wide equivalents) are separate from repository-level
  permissions — granting access at the project level cascades to every
  repository in it, which the user may or may not intend.

This is a single-resource CRUD domain — most requests here are a single
`search` → `get` → `call`, not a multi-step guided flow. If the user's
goal is really about a specific repository within the project, use
`bitbucket_workflow_repositories` instead.
