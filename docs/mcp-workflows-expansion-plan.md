# Expand guided Bitbucket workflows: Mesh, PR automation rules, and README docs

## Context

`bitbucket-dc-mcp-rs` shipped its MCP prompts capability in v0.6.0: a `bitbucket_workflow` master menu plus 11 guided sub-workflow prompts in `src/prompts/`. The user asked to review the full operation catalog for gaps — domains not reasonably covered by any existing sub-workflow — and, separately, to document the shipped prompts feature in `README.md` (which currently has zero mention of it).

Two research passes (one decompressing and querying all 6 embedded API-version stores against the 11 existing `.md` files' actual content, one re-verified independently against the live database in this session) found:

**Two segments are strong, evidence-based candidates for brand-new guided workflows** — comparable order-dependent complexity to the existing `mirroring.md`, and not mentioned by any of the 11 existing prompts:

1. **Bitbucket Mesh administration + migration** — 18 ops in the default 10.3 store (19 in 8.19, 18 in 9.4; present in all 6 versions, no version hedge needed). Two path families that are one domain: `admin/git/mesh/*` (9 ops — register/list/update/delete a mesh node, generate a connectivity report, get support zips, get the control-plane public key) and `migration/mesh/*` (9 ops — preview a migration job, start it, search repos by migration state, get job status/messages/summary, cancel it). Genuine prerequisite chain: register + verify-connectivity of a node before migrating any repository onto it; migration itself is an async job (preview → start → poll → optionally cancel). Verified directly: `registerNewMeshNode` (`POST /api/latest/admin/git/mesh/nodes`), `connectivity` (`GET .../diagnostics/connectivity`), `startMeshMigration`/`previewMeshMigration`/`getMeshMigrationJob`/`cancelMeshMigrationJob` all exist exactly as described.

2. **PR automation-rule configuration** — default-reviewers (9 ops) + reviewer-groups (11 ops) + default-tasks (10 ops, **absent before 9.4** — confirmed 0 ops in the 8.19 store) + auto-merge (9 ops) + auto-decline (6 ops) = 45 ops in 10.3 (35 in 8.19). Distinct from the existing `pull_requests.md`, which drives one PR's lifecycle — this is project/repo-level *standing policy* that shapes every future PR automatically. Verified directly: reviewer-groups, auto-merge, and auto-decline each have genuine project-scoped and repo-scoped path variants (e.g. `PUT /api/latest/projects/{projectKey}/settings/auto-merge` vs. `PUT /api/latest/projects/{projectKey}/repos/{repositorySlug}/settings/auto-merge`), confirming the project-vs-repo fork is real, not assumed. Reviewer-groups (`getReviewerGroups`/`create_1` at project scope, `_1`-suffixed siblings at repo scope) must exist before a default-reviewer condition can reference one by group.

**Everything else found is either a minor gap that belongs as a short addition inside an existing `.md` file, or not worth covering.** Fold-in targets, each verified present and (unless noted) stable across all 6 versions:
- `admin.md`: instance data export/import (`migration`, ~18 ops, high-stakes/instance-wide — distinct from the new Mesh workflow, which only shares the word "migration"); SSO/IdP config (`authconfig` + `basicauth`, ~13 ops, **absent in 8.19**); rate-limit (8 ops) + mail-server (6 ops); CSP (1 op, **only in 10.2/10.3**); a one-line pointer noting two-step-verification/TOTP self-enrollment (`tsv`, ~19 ops, **9.4+ only**) isn't a fit for text-driven automation (needs out-of-band QR-code scanning).
- `repositories.md`: hook-scripts + hooks (~25 ops combined — a script must be registered before a repo can reference it by ID); labels (6 ops).
- `build_integration.md`: Code Insights reports+annotations (~13 ops, same CI-reporting audience as build status); deployment tracking (~4 ops, same CI/CD signal family).
- `pull_requests.md`: comment-likes/reactions (4-10 ops, path shape differs by version), watch (6 ops), participants (6 ops), blocker-comments (5 ops), a pointer to inbox/dashboard, rebase as a merge alternative — all additive, no rewrite of the existing Step 0-6 skeleton.
- `webhooks.md`: delivery statistics (4 ops) — one bullet, that file already covers delivery history.
- Not worth covering: markup preview (1 op), profile/recent-repos (1 op) — trivial single-endpoint utilities.

This is purely additive to the shipped feature: it reuses `prompt_router`, `.enable_prompts()`, and `render_context_header` exactly as they exist today, with **zero changes to `src/core/mcp_server.rs`**. The non-negotiable rules established in `docs/mcp-prompts-workflow-plan.md` — never name a fixed `operationId`, always phrase steps as a capability to `search` for, respect the per-domain line-count bands — apply identically to every new/changed file here.

## Approach

### Two new prompts

**`bitbucket_workflow_mesh`** — arg struct in `src/prompts/mod.rs`, same shape as the existing three:

```rust
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MeshWorkflowArgs {
    /// Bitbucket project key of the repository being migrated onto Mesh
    pub project_key: Option<String>,
    /// Repository slug within the project being migrated onto Mesh
    pub repo_slug: Option<String>,
}
```

Router method in `src/prompts/router.rs`, modeled directly on `bitbucket_workflow_mirroring_prompt` (same file, same pattern — `render_context_header` over the two fields, `include_str!("content/mesh.md")`).

`src/prompts/content/mesh.md` (target ~85-90 lines, register → verify → operate skeleton like `mirroring.md`): opens with the domain description and operation counts above, an explicit **disambiguation from `bitbucket_workflow_admin`'s instance-data migration** (shares only the word "migration"), the standard delegable/agnostic-phrasing preamble, then: Step 0 gather params, Step 1 fork on whether a target mesh node already exists, Step 2 verify connectivity before trusting the node, Step 3 preview the migration job before starting it, Step 4 start + poll until complete, Step 5 fork to cancel on error/user request, Step 6 summarize.

**`bitbucket_workflow_pr_rules`** — arg struct:

```rust
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PrRulesWorkflowArgs {
    /// Bitbucket project key the automation rules apply to
    pub project_key: Option<String>,
    /// Repository slug to scope the rules to a single repository (omit for project-wide rules)
    pub repo_slug: Option<String>,
}
```

`src/prompts/content/pr_rules.md` (target ~100-105 lines): opens with the domain description, an explicit note that this is standing policy, not single-PR execution (**disambiguation from `bitbucket_workflow_pull_requests`**), and the **default-tasks version hedge** (absent before 9.4 — confirmed 0 ops in 8.19) stated up front, then: Step 0 gather params, Step 1 fork project-scope vs. repo-scope (confirm with the user rather than silently defaulting, since project-wide rules cascade), Step 2 the reviewer-groups prerequisite, Step 3 configure default-reviewer conditions, Step 4 default tasks (gated on the version hedge), Step 5 auto-merge/auto-decline as two independent, parallelizable/delegable steps, Step 6 verify by reading the configured rules back, Step 7 summarize, closing with "Composing with other workflows" pointing back to `bitbucket_workflow_pull_requests` for acting on one already-open PR.

### Fold-ins to five existing files

Each gets a new `## Also in scope` section (or, for `webhooks.md`, a single bullet appended to its existing `## Gotchas`), inserted before each file's closing CRUD-domain sentence — purely additive, no restructuring:

| File | Current lines | Adds | Target band |
|---|---|---|---|
| `admin.md` | 41 | ~17 (migration/authconfig+basicauth/rate-limit+mail-server/csp/tsv bullets) | 40-60 |
| `repositories.md` | 28 | ~11 (hook-scripts+hooks, labels) | 20-50 |
| `build_integration.md` | 29 | ~10 (Code Insights, deployment tracking) | 20-50 |
| `pull_requests.md` | 92 | ~11 (`## Also in scope (brief)` before "Composing with other workflows") | 60-120 |
| `webhooks.md` | 25 | ~4 (one bullet on delivery statistics) | 20-50 |

All five stay comfortably within their existing target bands.

### `master.md` update

Add two rows to the menu table: `bitbucket_workflow_pr_rules` placed after `build_integration` and before `mirroring` (same admin/policy audience as `admin`/`build_integration`, deliberately *not* adjacent to `pull_requests` despite the thematic link — the routing guidance needs to disambiguate them, not encourage conflating them); `bitbucket_workflow_mesh` placed immediately after `mirroring` (same register→verify→operate shape, keeping the two infra-registration workflows together). Append one sentence to `## Routing guidance` telling the calling LLM explicitly not to confuse `pull_requests` (driving one PR) with `pr_rules` (configuring standing rules). Resulting size: ~50 lines, still under the 60-line target.

### Test updates (`tests/prompts_workflow.rs`)

- Add both new names to `EXPECTED_PROMPT_NAMES` in sorted position (`mesh` before `mirroring`; `pr_rules` before `projects`, since `_` sorts before any lowercase letter in the shared `bitbucket_workflow_p` prefix) — verify empirically by running the test rather than trusting manual sort reasoning alone.
- Generalize `mcp_protocol_lists_every_workflow_prompt_with_optional_arguments`'s `required == Some(false)` check from only `pull_requests`'s arguments to a loop over *every* listed prompt's arguments. This is a module-wide invariant (documented on `render_context_header` itself), not something specific to one prompt — it only ever checked `pull_requests` because that was the sole args-bearing prompt in the original vertical slice. Now that `mirroring`, `mesh`, and `pr_rules` all carry arguments, enforce the rule uniformly. Keep the existing `pull_requests`-specific argument-*name* check as-is underneath (a different assertion — which names are advertised — not the one being generalized).
- No other test changes needed: `every_workflow_prompt_returns_non_empty_guided_text` already iterates `EXPECTED_PROMPT_NAMES`, so once both new names are added it automatically exercises both new router-method bodies for coverage, the same way it already does for the 9 non-worked-example prompts from the original feature. No bespoke round-trip test is needed for `mesh`/`pr_rules` specifically — `mirroring` itself never got one either; only the original vertical-slice worked example (`pull_requests`) did, and that pattern doesn't need to be repeated for every new prompt going forward.

### `README.md` update

Insert a new `### Guided workflow prompts` subsection after the existing `### Connect an MCP client` and before `## Docker` (the natural anchor — both describe what an MCP client sees/uses once connected). Short intro paragraph (mentions `bitbucket_workflow` as the entry point, states the total prompt count, notes content is version-agnostic across all 6 supported versions the same way the 3 tools are, linking to `docs/SCHEMA_VERSIONS.md`), followed by a `Prompt` / `Use when the user wants to...` table listing all 14 prompts (1 master + 13 sub-workflows) — mirrors the plan doc's existing inventory-table pattern and the README's own convention of using tables only for enumerable structured facts, matching its terse, heavily-inline-coded style exactly (no prose padding, no new heading levels beyond the established `##`/`###`).

### New plan document

Persist this plan as `docs/mcp-workflows-expansion-plan.md` — a **separate** file from `docs/mcp-prompts-workflow-plan.md`, which stays untouched as the historical record of the already-shipped v0.6.0 feature. Same Context/Approach/Critical-files/Sequencing/Verification/Release structure as the original.

## Critical files

- `src/prompts/mod.rs` — add `MeshWorkflowArgs`, `PrRulesWorkflowArgs`
- `src/prompts/router.rs` — add `bitbucket_workflow_mesh_prompt`, `bitbucket_workflow_pr_rules_prompt`
- `src/prompts/content/mesh.md` (new), `src/prompts/content/pr_rules.md` (new)
- `src/prompts/content/admin.md`, `repositories.md`, `build_integration.md`, `pull_requests.md`, `webhooks.md` — fold-in additions
- `src/prompts/content/master.md` — two new rows + disambiguation sentence
- `tests/prompts_workflow.rs` — `EXPECTED_PROMPT_NAMES`, generalized required-argument loop
- `README.md` — new `### Guided workflow prompts` subsection
- `docs/mcp-workflows-expansion-plan.md` (new) — this plan, persisted first
- `Cargo.toml`/`Cargo.lock` — version bump

## Sequencing

0. Persist this plan to `docs/mcp-workflows-expansion-plan.md` first — **done**, this file.
1. Implement `bitbucket_workflow_mesh` (arg struct, router method, `content/mesh.md`) — lowest risk, mechanically re-proves the already-shipped `mirroring.md` skeleton.
2. Implement `bitbucket_workflow_pr_rules` (arg struct, router method, `content/pr_rules.md`) — more novel (scope fork, prerequisite, version hedge), built once the plumbing is re-validated by step 1.
3. Fold the five minor-gap additions into `admin.md`, `repositories.md`, `build_integration.md`, `pull_requests.md`, `webhooks.md`.
4. Update `master.md` last among content changes, once both new prompt names are final.
5. Update `tests/prompts_workflow.rs`.
6. Update `README.md`.

## Verification

- `cargo fmt --check`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo test --locked` — confirms `EXPECTED_PROMPT_NAMES` matches `prompts/list`'s actual sorted output (don't just trust manual ASCII-sort reasoning — let the test prove it), and that the blanket body-execution test covers both new router methods.
- `bash scripts/coverage.sh` — the 85% production-line gate (currently at 87.05%). Both new prompts are the same thin-dispatch shape as the 9 generic prompts from the original feature (no new branching logic beyond the existing `render_context_header` call pattern already covered), so this should land in the same range with no dedicated new tests required beyond the `EXPECTED_PROMPT_NAMES` update. If it drops unexpectedly, confirm `EXPECTED_PROMPT_NAMES` was actually updated before adding speculative tests.
- Manual smoke check (optional): `cargo run -- start` with an MCP-capable client, fetch `bitbucket_workflow_mesh` and `bitbucket_workflow_pr_rules` by name, confirm the version-hedge and disambiguation language reads unambiguously to a real calling LLM.

## Release (once implementation is complete and `cargo test` passes)

Same tag-driven convention as the original release, and the same `feat` → minor-bump precedent this repo already established (confirmed: `feat(auth)` PAT support went 0.1.3→0.2.0; the original prompts feature itself went 0.5.8→0.6.0 as `feat(prompts)`). This is another `feat`-scoped addition (2 new prompts is new capability), so bump **0.6.0 → 0.7.0**:

1. `git commit` the implementation as `feat(prompts): add mesh and pr-rules guided workflows`.
2. `git commit` `docs/mcp-workflows-expansion-plan.md` separately as `docs: add MCP workflows expansion plan`.
3. Bump `Cargo.toml` to `0.7.0` (let `Cargo.lock` follow via `cargo check`), commit as `chore(release): bump version to 0.7.0`.
4. `git tag v0.7.0`.
5. Confirm with the user, then `git push` the branch and `git push origin v0.7.0`.
