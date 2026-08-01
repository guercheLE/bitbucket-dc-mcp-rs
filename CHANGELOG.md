# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project does not currently follow Semantic Versioning strictly (patch
bumps have shipped for `feat` commits early on before a minor-bump convention
was established at 0.6.0); version numbers below simply match this repo's
actual tag history.

## [Unreleased]

No unreleased changes at this time — `HEAD` matches the `v0.7.7` tag.

## [0.7.7] - 2026-07-26

### Fixed

- Multi-arch GHCR image builds now use Docker's build-by-digest-then-merge
  pattern — a matrix job builds `linux/amd64` on `ubuntu-24.04` and
  `linux/arm64` natively on `ubuntu-24.04-arm` (no emulation), then a merge
  job combines both digests into one multi-arch manifest. Replaces the
  previous single-job QEMU cross-build.

## [0.7.6] - 2026-07-26

### Fixed

- The release workflow now actually publishes a multi-arch (`amd64`+`arm64`)
  GHCR image. `docker/build-push-action` was never given a `platforms:`
  value, so only `linux/amd64` was ever built and pushed despite running
  `docker/setup-buildx-action` — pulling on an arm64 host (Apple Silicon, arm64
  CI runners) failed with "no matching manifest".

## [0.7.5] - 2026-07-26

### Fixed

- Fully inlined every non-cyclic `$ref` in the operation catalog's stored
  input/output schemas (`mcp_store.db` and `generated_schemas.json`), which
  previously still referenced a locally-embedded `$defs` block left over from
  an earlier partial fix. Remaining `$ref`s are confirmed genuine reference
  cycles (e.g. `RestComment` nesting itself for PR-comment replies), which
  have no finite fully-inlined representation.

## [0.7.4] - 2026-07-21

### Fixed

- Regenerated every configured API version's stores/schemas (`mcpify
  add-version --force`, mcpify 0.11.5) to pick up a generator fix where a
  component `$ref` inside a `get`-tool response could point at nothing in the
  returned snippet; every operation's schema now embeds a `$defs` map
  alongside any `$ref` so `get` responses are self-contained. Re-populated
  semantic-search embeddings for every refreshed store.

## [0.7.3] - 2026-07-21

### Fixed

- Added `timeout-minutes` caps to the release workflow's build job (45 min)
  and CI workflow's test job (20 min), so a hang falls back to a bounded
  timeout instead of GitHub Actions' 6-hour job ceiling.

## [0.7.2] - 2026-07-21

### Fixed

- The shared embedded-store SQLite connection now recovers from mutex
  poisoning at every lock site instead of propagating it, so a panic while
  holding the lock no longer permanently breaks every subsequent
  `search`/`get`/`call`/CLI invocation sharing that connection. Also wrapped
  the MCP protocol test in a 30s timeout so a future hang fails fast.

## [0.7.1] - 2026-07-20

### Fixed

- Renamed MCP prompt identifiers to kebab-case without the redundant
  "workflow" segment (e.g. `bitbucket_workflow_pull_requests` →
  `bitbucket-pull-requests`, master menu prompt renamed to `bitbucket`), to
  match this ecosystem's slash-command-style naming convention. Every
  cross-reference (router descriptions, content files, README table, tests)
  was updated to match.

## [0.7.0] - 2026-07-20

### Added

- Two new guided workflow prompts: `bitbucket-mesh` (register a Bitbucket
  Mesh node, verify connectivity, migrate repositories onto it) and
  `bitbucket-pr-rules` (standing PR automation rules — default reviewers,
  reviewer groups, default tasks, auto-merge, auto-decline — distinct from
  driving a single PR's lifecycle).
- Smaller catalog gaps folded into existing guided-workflow prompts: instance
  migration, SSO config, hook scripts, Code Insights, deployment tracking, PR
  social features (likes/watch/participants/blocker-comments), and webhook
  delivery statistics.
- Documented the full guided-workflow-prompts feature in `README.md` for the
  first time.

### Documentation

- Added `docs/mcp-workflows-expansion-plan.md`, the implementation plan for
  the Mesh and PR-rules prompts.

## [0.6.0] - 2026-07-20

### Added

- New MCP **prompts** capability alongside the existing `search`/`get`/`call`
  tools: a `bitbucket_workflow` master menu plus 11 guided sub-workflow
  prompts (projects, repositories, pull requests, branches & commits,
  webhooks, access tokens & keys, secret scanning, admin, build/CI
  integration, mirroring, monitoring & diagnostics). Each prompt walks a
  calling LLM through a domain's task step by step, gating progress on
  verified outcomes and phrasing every operation reference as a capability to
  `search` for rather than a fixed `operationId`, since `operationId`s are
  not stable across this server's 6 supported API versions (verified: 19 of
  499 `operationId`s shared between the 10.3 and 8.19 stores resolve to a
  genuinely different path/method depending on version).

### Documentation

- Added `docs/mcp-prompts-workflow-plan.md`, the implementation plan for the
  prompts feature.

## [0.5.8] - 2026-07-19

### Fixed

- Retried the embedded-store rename with a short backoff (5 attempts,
  50ms × attempt) instead of failing outright when a different OS process
  (a concurrently-starting server instance, a `populate_embeddings` run) has
  the destination file open at the exact moment of rename — this could fail
  the rename itself on Windows.
- Skipped the `setup` CLI smoke test on GitHub Actions Windows runners, since
  `inquire`'s Windows backend reads raw console input rather than treating a
  closed/non-console stdin as immediate EOF the way Unix does, causing the
  test to hang indefinitely there.

### Changed

- CI now installs `cargo-dist` from a prebuilt binary instead of compiling it
  from source on every release run, cutting several minutes (roughly double
  on Windows) off release pipeline time.

## [0.5.7] - 2026-07-19

### Documentation

- Added a sponsorship callout to `README.md` and a `FUNDING.yml`.

## [0.5.6] - 2026-07-19

### Fixed

- Stopped the stdio smoke test from hanging indefinitely on GitHub Actions
  Windows runners (previously seen taking ~5 hours before manual
  cancellation). Windows's blocking-thread stdin reader doesn't observe an
  immediately-EOF-closed child stdin the same way Unix does; the test is now
  skipped there via a runtime check (`cfg!(windows) && GITHUB_ACTIONS`)
  rather than a blanket `#[cfg(windows)]`, so it still runs on a real Windows
  developer machine.

## [0.5.5] - 2026-07-19

### Fixed

- Relaxed a Windows-fragile stdio-transport integration test that asserted
  exact `rmcp` error wording only ever exercised on Linux/macOS before a
  prior fix let the test suite reach it on Windows CI; the platform-
  independent behavior (handshake never completes, process exits non-zero)
  is still checked everywhere, with the exact wording assertion now Unix-only.

## [0.5.4] - 2026-07-19

Internal test-coverage work only (no user-facing changes): closed the
remaining production-code coverage gap toward the 85% CI gate
(1586/1914 → 1659/1914 lines, 82.86% → 86.68%) with real tests for
`AuthManager::normalize_credentials`, `ApiClient::execute`'s full happy path
and circuit-breaker error path, and the `call` tool's known-operation path.

## [0.5.3] - 2026-07-19

### Fixed

- Eliminated a Windows-only race in `resolve_store_path`: concurrent calls
  each re-extracted and renamed the embedded SQLite store into place
  unguarded, and Windows can refuse a rename over a destination another
  thread already has open (unlike POSIX) — this caused unrelated MCP-protocol
  tests to see `search.is_error == true` in CI's Windows release build. The
  resolved path is now cached per API version behind a mutex, so extraction
  happens once per process as intended.

## [0.5.2] - 2026-07-19

### Fixed

- Marked the generated release workflow `allow-dirty` for `cargo-dist`, since
  it's a deliberately simplified hand-written workflow rather than dist's own
  auto-generated multi-job shape, which dist was flagging as "out of date"
  and failing outright.

## [0.5.1] - 2026-07-19

### Fixed

- Stopped a setup-wizard unit test from requiring a real TTY: with more than
  one API version configured, the test routed through `inquire::Select`
  (which needs a real TTY) and panicked in CI; it now constructs the test
  inputs directly instead of prompting.

## [0.5.0] - 2026-07-19

### Changed

- Re-synced against mcpify's current code-generation templates and re-added
  all 6 supported API versions. Notable changes picked up from the generator:
  every store now embeds zstd-compressed by default (a feature this repo had
  previously hand-patched, now upstreamed into mcpify itself); hardened
  auth/validation/schema handling in the core server, API client, and search
  tool that had been hand-patched directly into this repo is now generated
  correctly by default; a simpler hand-rolled release workflow replaced
  cargo-dist's full auto-generated pipeline, plus new coverage/profiling
  scaffolding; README sections (Observability & Resilience, License, install)
  previously hand-authored here and silently dropped by an older thin
  generator template are now restored by the generator itself, with this
  repo's specific facts layered back on top; `.gitignore`'s cargo-default
  entries and the `.mcpify/*` scratch-file guard, lost the same way, are
  restored too.

### Documentation

- Added a "Connect an MCP client" section to `README.md` with real stdio/HTTP
  `mcpServers` JSON configs.

## [0.4.4] - 2026-07-17

### Fixed

- Completed the OAuth2 code-exchange credentials cascade: `AuthManager::
  credentials()` only ever tried `refresh_token()` when stored credentials
  failed validation, never the initial `authenticate()` exchange, so a
  credential blob with neither an access token nor a refresh token fell
  straight to a generic "no active credentials" error instead of re-deriving
  credentials from what was actually stored.
- Body-less `PUT`/`POST`/`DELETE` calls (operations whose arguments are
  entirely query/path params) now send an explicit empty body with
  `Content-Length: 0`; previously no body and no `Content-Length` were sent
  at all, which strict APIs reject with `411 Length Required` before ever
  reaching auth or business logic.

Both fixes were propagated from bugs found downstream in mcpify's shared
generator templates, confirmed present in every mcpify-generated target
including this repo's own generated baseline.

## [0.4.3] - 2026-07-16

### Fixed

- The embedded `mcp_store.db` copy is now extracted to a uniquely-named
  sibling file and renamed into place atomically, instead of being rewritten
  via a direct, non-atomic write on every tool call. Concurrent MCP tool
  calls (separate tokio tasks) could previously read the file mid-truncate
  from another call's in-progress write, and SQLite treats a momentarily-
  empty file as a valid empty database rather than a corrupt one — surfacing
  as spurious "no such table" errors instead of a clear one.

## [0.4.2] - 2026-07-16

### Fixed

- `call` no longer rejects an otherwise-successful API response just because
  it doesn't match the documented output schema — it now logs a warning and
  still returns the live response. Upstream OpenAPI specs are frequently
  wrong about response shape, so treating every mismatch as fatal denied
  callers real data over a documentation bug; input-argument validation is
  unchanged (still rejected, since those are under the caller's control).
- Validation error messages now include the actual JSON Schema violation
  detail instead of just a generic "invalid input" / "unexpected response
  shape" message.

## [0.4.1] - 2026-07-16

### Fixed

- Embedded API-version stores are now zstd-compressed (level 19) instead of
  raw, decompressed to the temp store path on first use — the backfilled
  embeddings across all 6 stores had pushed the packaged crate to 10.2MiB,
  over crates.io's 10,485,760-byte publish limit. Final crate size:
  10,414,100 bytes (~0.68% margin).

## [0.4.0] - 2026-07-15

### Fixed

- Backfilled missing semantic-search embeddings for API version 9.4: `search`
  under that version silently returned `[]` for every query because its
  store's `semantic_endpoints` table had 0 rows despite 546 endpoints being
  present. `populate-embeddings` now defaults to populating and verifying
  every API version's store (row-count parity between endpoints and
  semantic_endpoints), failing loudly on missing operation IDs instead of
  silently under-populating; `search` now warns when a store is incomplete.
  All 6 version stores were backfilled and confirmed at exact row-count
  parity.
- Credentials are now read from `BITBUCKET_DC_MCP_TOKEN`/
  `BITBUCKET_DC_MCP_API_KEY` (or username/password) environment variables
  before falling back to the stored credential.
- Falls back to the encrypted credential file when the OS keychain cleanly
  reports "not found", not just on a hard error.
- Resolves the home directory via `HOME` then `USERPROFILE` on Windows.
- The setup wizard now prompts for global vs. local config persistence and
  writes YAML the config loader actually reads back.
- The `call` tool's `arguments` field now defaults to `{}` instead of `null`.
- The `api_key` auth-header branch now uses the scheme's real configured
  header name, for parity with other mcpify-generated servers.

### Documentation

- Documented that `BITBUCKET_DC_MCP_URL` must include the `/rest` suffix.

## [0.3.0] - 2026-07-14

### Changed

- Adopted mcpify's current Rust code-generation parity (specifics not
  itemized in the commit; the crate's identity — `bitbucket-dc-mcp`, no
  `-rs` suffix — and version were preserved by hand post-sync).

### Documentation

- Clarified container (Docker) transport usage in the README.

## [0.2.15] - 2026-07-13

### Documentation

- Documented real `call` arguments (replacing placeholder/example text with
  grounded, accurate usage).

## [0.2.14] - 2026-07-10

### Fixed

- Docker builds now `COPY` all 6 version `.db` files instead of only the
  default `mcp_store.db` — `cargo build` inside Docker was failing with
  "No such file or directory" for every non-default version, since
  `include_bytes!` needs every file it embeds to exist at build time.
- Fixed `cargo fmt --check` failures in `store.rs`/`setup_wizard.rs` left
  over from an earlier add-version/remove-version run that never ran
  `cargo fmt` afterward.
- Dropped `x86_64-apple-darwin` from the release target matrix (Microsoft's
  ONNX Runtime ended x64 macOS support; `ort` followed suit, with no
  prebuilt binary to link against). Pinned `x86_64-unknown-linux-gnu` to
  `ubuntu-24.04` (the default `ubuntu-22.04` runner's glibc was too old for
  pyke's prebuilt ONNX Runtime). Fixed a Windows CRT mismatch via
  `msvc-crt-static = false`.

## [0.2.13] - 2026-07-10

### Added

- CI now cross-compiles and distributes prebuilt binaries (macOS x86_64/
  arm64, Linux x86_64, Windows) to GitHub Releases with shell/PowerShell
  installers, triggered by pushing a version tag, via `cargo-dist`. Splits
  crates.io publishing and the GHCR image push into their own workflows.
  Linux aarch64 was left out of the target matrix, since this crate bundles
  native SQLite and `cargo-dist` doesn't set up an ARM cross C-toolchain for
  that target.

## [0.2.12] - 2026-07-08

### Documentation

- Fixed a stale `call` usage example that used a fabricated `--some-arg`
  syntax instead of the real `-a`/`--args` JSON flag. Documented the
  `test-connection`/`config`/`version`/`versions` subcommands, the config
  cascade, Docker usage, and the observability/resilience stack, none of
  which had been documented before.

## [0.2.11] - 2026-07-08

### Fixed

- Credentials are now normalized before use in the auth flow.

## [0.2.10] - 2026-07-06

### Fixed

- `reqwest`'s rustls-tls now also trusts the OS root certificate store
  (`rustls-tls-native-roots`) alongside rustls's bundled `webpki-roots` list,
  fixing HTTPS handshake/certificate failures (notably for `fastembed`'s
  Hugging Face model download) behind a corporate TLS-inspecting proxy whose
  injected root CA was trusted by the OS but not by rustls's bundled list
  alone.

## [0.2.8] - 2026-07-06

### Added

- Added the Bitbucket Data Center 9.4 API version as a bundled store via
  `mcpify add-version`. Also fixed a codegen gap where the version-sync step
  had dropped the `VERSION_STORE_BYTES` const (and `pub` on
  `VERSION_STORE_FILES`) from `src/data/store.rs`, which would have failed
  `cargo package`'s build verification.

## [0.2.7] - 2026-07-06

### Fixed

- Logs are now written to stderr instead of stdout. The MCP stdio transport
  speaks JSON-RPC over stdout, and `tracing_subscriber`'s fmt layer defaulted
  to stdout too — every log line was interleaved into that stream, and MCP
  clients that strictly parse stdout as JSON-RPC frames (e.g. VS Code) failed
  schema validation on the first log line and tore down the connection.

## [0.2.6] - 2026-07-06

### Fixed

- Installed the `aws-lc-rs` rustls crypto provider once at process start.
  `opentelemetry-otlp`'s HTTP exporter pulls in a second, transitive
  `reqwest` that links rustls with no crypto backend selected, which panicked
  with "No rustls crypto provider is configured" the first time the OTLP
  span exporter built its client.

## [0.2.5] - 2026-07-06

### Removed

- Pruned the bundled API-version set from 30 to 5 (default 10.3, plus 10.2,
  9.6, 9.5, and 8.19) to fit crates.io's package size limit — the
  embedded-store-bytes fix (0.2.2) made every version's `.db` and compiled
  schema part of the published package, and the full 30-version set
  (125.1MiB uncompressed) exceeded the limit, confirmed by a failed upload
  attempt. Kept the 8.x line represented rather than doing a strict
  "most recent N" cut. Package verified at 8.5MiB compressed. Dropped
  versions can still be fetched from the original OpenAPI source and
  `mcpify add-version`'d back in locally.

## [0.2.4] - 2026-07-06

### Fixed

- Backfilled semantic-search embeddings for every `api_version`'s store by
  running the fixed `populate_embeddings --all` across all 30 versions this
  project supported at the time — previously only the default version (10.3)
  had any embeddings at all, so search returned no ranked results under any
  other configured version.

## [0.2.3] - 2026-07-06

### Fixed

- The embedded-store temp-dir extraction now always overwrites instead of
  skipping when the destination path already existed, so a stale copy from a
  previous install (older embedded bytes) no longer lingers and silently
  serves outdated data after a rebuild. Added a test asserting every
  `VERSION_STORE_FILES` label has a matching `VERSION_STORE_BYTES` entry.
- `populate_embeddings` now accepts an explicit store path, or `--all` to
  backfill every version this project's ledger knows about, instead of being
  hardcoded to `mcp_store.db` only (which left every other version's
  `semantic_endpoints` table permanently empty).

## [0.2.2] - 2026-07-06

### Fixed

- Every `mcp_store*.db` this crate lists is now embedded directly into the
  compiled binary via `include_bytes!`, and `resolve_store_path`'s fallback
  extracts the active version's bytes to the OS temp dir on first use. A
  previous fix (0.2.1) only resolved the store path relative to the
  executable's directory, which still failed once the build checkout was
  moved or deleted; the filesystem-fallback chain (`CARGO_MANIFEST_DIR`,
  exe-dir) is now dead code and was removed, since the embedded copy is
  always available regardless of invocation location.

## [0.2.1] - 2026-07-06

### Fixed

- `resolve_store_path` now falls back to the directory containing the
  running executable when a bundled `mcp_store*.db` isn't found in the
  current working directory, mirroring the install-dir fallback the config
  cascade already used. (Superseded by the embedded-bytes approach in
  0.2.2.)

## [0.2.0] - 2026-07-06

### Added

- Added Personal Access Token (PAT) support: Bitbucket Data Center accepts a
  bearer-token PAT in addition to Basic auth, but the OpenAPI spec this
  server was generated from only declared Basic. Adds a `PatAuthStrategy`
  alongside the existing `BasicAuthStrategy`, a new `AuthMethod::Pat` config
  value, and lets the setup wizard choose between the two.

### Documentation

- Documented the versions, source Swagger v3 URLs, and generated `.db`/
  schema files used to build the vector store.

## [0.1.3] - 2026-07-05

No functional changes (version-bump-only release).

## [0.1.2] - 2026-07-05

### Added

- Initial Rust MCP server implementation for Bitbucket Data Center, generated
  by mcpify: a CLI/harness server exposing `search`/`get`/`call` tools backed
  by per-version embedded semantic databases and validation schemas.
- Published the crate to crates.io via CI.

### Fixed

- Namespaced the helper binary to avoid a naming collision.
