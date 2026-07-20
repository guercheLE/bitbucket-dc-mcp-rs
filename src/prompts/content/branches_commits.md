# Bitbucket workflow: branches and commits

Covers commits, branches (including the default branch), branch
permissions/restrictions, branch-utils (e.g. checking whether a branch is
merged or safe to delete), tags, and compare/diff between refs.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I list commits on a branch?",
"how do I restrict who can push to a branch?"), then `get` the resolved
operation to confirm its actual path, method, and current schema before
calling it. Never assume a response field name — read the schema `get`
returns.

## Gotchas

- Branch permissions/restrictions here compose with
  `bitbucket-pull-requests`'s merge gate (Step 4) and
  `bitbucket-build-integration`'s required-build checks — a
  restricted branch that also requires builds needs both configured to
  behave as the user expects. Point the user to those prompts rather than
  re-deriving their content here.
- Before deleting a branch, search for how to check whether it's fully
  merged into its target — don't assume the caller has already verified
  this.

This is a CRUD domain — most requests here are a single `search` → `get`
→ `call`, not a multi-step guided flow.
