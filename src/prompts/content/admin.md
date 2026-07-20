# Bitbucket workflow: admin

Covers instance-wide administration: users and groups (lifecycle, group
membership), instance-wide permissions (as distinct from project- or
repository-scoped permissions in `bitbucket-projects`/
`bitbucket-repositories`), license management, and
cluster/global settings.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I create a user?", "how do I add
a user to a group?", "how do I check the current license?"), then `get`
the resolved operation to confirm its actual path, method, and current
schema before calling it. Never assume a response field name — read the
schema `get` returns.

## Gotchas

- Instance-wide admin permissions are broad — before granting or
  revoking a user's admin/sys-admin status, confirm explicitly with the
  user rather than inferring intent from a vaguer request.
- User and group listings can be large. If your environment supports
  running an isolated sub-task, delegate a broad listing/search and bring
  back only the distilled result (the specific user or group found, a
  count) rather than pulling the full listing into this conversation.
  Otherwise page through it directly here.
- Deleting a user or group is not always reversible and can affect
  permissions cascaded from group membership across every project and
  repository — confirm with the user, and consider checking the user's
  or group's current permission grants first so the user understands the
  blast radius.
- License and cluster/global settings changes affect the whole instance,
  not a single project or repository — treat these as higher-stakes than
  the rest of this domain and confirm explicitly before calling a mutating
  operation.

This domain spans several resource types without the strict cross-resource
ordering that would justify full step-gating — most individual requests
are still a single `search` → `get` → `call`, just with more judgment
calls around confirmation than a typical CRUD domain.

## Also in scope

- **Instance data export/import** (~18 operations) — high-stakes and
  instance-wide; confirm explicitly with the user before starting an
  export or import. Don't confuse this with `bitbucket-mesh`'s
  per-repository Git-storage migration onto a Mesh node — a different,
  narrower capability that shares only the word "migration".
- **SSO/IdP configuration** (~13 operations) — absent entirely in 8.19;
  confirm the active API version supports it before searching for these
  operations there.
- **Rate limiting and mail server** instance settings (~14 operations
  combined) are stable across all supported versions.
- **Content Security Policy** settings (1 operation) exist only in
  10.2/10.3.
- **Two-step verification / TOTP self-enrollment** (~19 operations,
  9.4+ only) exists but isn't a fit for guided text-driven automation —
  it requires out-of-band interaction (scanning a QR code). Point the
  user to the Bitbucket UI for that instead of attempting it here.
