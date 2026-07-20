# Bitbucket workflow: webhooks

Covers creating, listing, updating, and deleting webhooks at both the
project and repository scope, and inspecting a webhook's event types and
delivery history.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I create a repository webhook?",
"how do I list a webhook's recent deliveries?"), then `get` the resolved
operation to confirm its actual path, method, and current schema before
calling it. Never assume a response field name — read the schema `get`
returns.

## Gotchas

- Confirm with the user whether they want a project-scoped webhook
  (fires for every repository in the project) or a repository-scoped one
  — don't assume based on which one is mentioned first.
- A webhook's target URL and secret are sensitive; never log or echo the
  configured secret back in a summary.
- Delivery statistics sit alongside the delivery-history operation
  already covered above — search for how to fetch a webhook's delivery
  statistics if the user wants aggregate success/failure counts rather
  than the raw history.

This is a CRUD domain — most requests here are a single `search` → `get`
→ `call`, not a multi-step guided flow.
