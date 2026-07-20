# Bitbucket workflow: Smart Mirroring

Guides registering a mirror server against an upstream Bitbucket Data
Center instance and verifying it's actually syncing — Smart Mirroring is
Bitbucket's largest non-API-prefixed domain (42 operations) and, like
pull requests, has a genuine order dependency: an upstream server must
exist before a mirror server can register against it, and a mirror server
must be accepted before it starts syncing.

**This sub-workflow is self-contained and delegable.** If routed here from
`bitbucket_workflow`'s menu, or your environment otherwise supports
running this as its own isolated sub-task, run it that way and report
back only a short summary — what's registered, accepted, and syncing —
not the full step-by-step trace.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions — confirmed
directly: `getMirrorMode`, `getMirrorSettings`, and `getFarmNodes` resolve
to genuinely different paths depending on version (older versions expose
them at the top level; newer versions nest them under a specific upstream
server). Always `search` by capability, then `get` the resolved operation
to confirm its real path and schema before calling it.

## Step 0 — gather required parameters

Check the header above. You need `upstream_project_key` and
`upstream_repo_slug` (or, for a farm-wide/global mirror setup instead of a
per-repository one, confirm with the user that they mean the whole
instance rather than one repository) before proceeding.

## Step 1 — fork: does an upstream server registration already exist?

Search for how to list configured upstream servers. If the target
instance is already registered, reuse its identifier; otherwise register
it first — this is a prerequisite for every later step.

## Step 2 — two independent registration steps

Once the upstream server exists, registering this instance as a mirror
server and inspecting the upstream's current mirror/sync settings are
independent of each other — call this out as parallelizable and, if the
environment supports it, delegate each to its own sub-task, returning only
a short confirmation rather than the full response body.

## Step 3 — accept the mirror server registration

A newly registered mirror server is not active until explicitly accepted.
Search for how to accept a pending mirror server registration for the
upstream. Don't proceed until the acceptance call is confirmed, not just
attempted.

## Step 4 — verify sync state

Search for how to check farm nodes / sync status for the accepted mirror
server. Confirm it reports an actively syncing state before declaring the
setup complete — a successful acceptance does not by itself guarantee sync
has started.

## Step 5 — summarize

Report what's registered, accepted, and confirmed syncing, and anything
still pending (e.g. sync still catching up).
