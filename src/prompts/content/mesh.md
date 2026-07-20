# Bitbucket workflow: Mesh administration and migration

Guides registering a Bitbucket Mesh node and migrating one or more
repositories' Git storage onto it. Mesh spans two path families that are
one real domain: `admin/git/mesh` (9 operations — register/list/update/
delete a mesh node, generate a connectivity report, get support zips, get
the control-plane public key) and `migration/mesh` (9 operations —
preview a migration job, start it, search repositories by migration
state, get job status/messages/summary, cancel it). 18 operations in the
default (10.3) store, present in all 6 supported API versions (19 in
8.19, 18 in 9.4) — no version hedge needed for this workflow.

**Don't confuse this with the general instance-wide data migration
covered in `bitbucket_workflow_admin`.** That workflow's migration
material is about exporting/importing an entire Bitbucket instance's data
between deployments. This workflow is specifically about migrating one
repository's Git storage onto a registered Mesh node — a narrower,
per-repository operation that shares only the word "migration" with that
unrelated admin capability.

**This sub-workflow is self-contained and delegable.** If routed here
from `bitbucket_workflow`'s menu, or your environment otherwise supports
running this as its own isolated sub-task, run it that way and report
back only a short summary — what's registered, verified, and migrated —
not the full step-by-step trace.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions. Always
`search` by capability (e.g. "how do I register a mesh node?", "how do I
start a repository migration to mesh?"), then `get` the resolved
operation to confirm its real path and schema before calling it.

## Step 0 — gather required parameters

Check the header above. You need `project_key` and `repo_slug` to migrate
a specific repository — or, if the user's goal is only to register or
verify a node without migrating anything yet, confirm that narrower scope
with the user instead of asking for repository parameters they don't
need.

## Step 1 — fork: does a target mesh node already exist?

Search for how to list registered mesh nodes. If a suitable node is
already registered, reuse its identifier; otherwise register a new one
first — this is a prerequisite for every later step, since no migration
can target a node that doesn't exist yet.

## Step 2 — verify connectivity before trusting the node

A newly registered (or reused) node isn't necessarily reachable. Search
for how to generate a connectivity report for the mesh node and read the
result — don't proceed to migration until it reports the node as
healthy. If the report is verbose and your environment supports an
isolated sub-task, delegate reading it and bring back only the pass/fail
summary.

## Step 3 — preview the migration before starting it

Search for how to preview a repository migration to the target mesh node
(a dry run). Confirm the preview identifies the expected repository (or
repositories) and reports no blocking issues before starting a real job
— this catches configuration mistakes before they touch anything.

## Step 4 — start the migration job, then poll until it completes

Start the migration job, then search for how to check a migration job's
status/messages/summary and poll it — don't declare the repository
migrated just because the start call returned without an error; confirm
the job's status actually reports completion. Don't poll in a tight loop;
space out checks and report progress if it's taking a while.

## Step 5 — fork: cancel if something's wrong

If polling in Step 4 surfaces a blocking error, or the user asks to
abort, search for how to cancel a migration job. Confirm the cancellation
is reflected in the job's status before reporting it stopped.

## Step 6 — summarize

Report what's registered, what connectivity was verified, and which
repositories are migrated, still in progress, or failed — including any
job messages relevant to a failure.
