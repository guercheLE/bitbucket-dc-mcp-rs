# Bitbucket workflow: secret scanning

Covers project- and repository-scoped secret scanning configuration,
reviewing detected findings, and managing allowlist entries for confirmed
false positives.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — the same id
can resolve to a different endpoint depending on which version is active.
`search` for what you need (e.g. "how do I list secret scanning findings
for a repository?", "how do I allowlist a detected secret?"), then `get`
the resolved operation to confirm its actual path, method, and current
schema before calling it. Never assume a response field name — read the
schema `get` returns.

## Gotchas

- Never echo the actual secret value found in a scan result back to the
  user in a summary — report its location (file/commit) and type only.
- Allowlisting a finding is a judgment call about whether it's a real
  secret; don't allowlist on the user's behalf without their explicit
  confirmation of the specific finding.

This is a CRUD domain — most requests here are a single `search` → `get`
→ `call`, not a multi-step guided flow.
