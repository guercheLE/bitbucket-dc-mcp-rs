# Bitbucket workflow: pull request automation rules

Guides configuring project- or repository-scoped PR automation rules:
default reviewers (9 operations), reviewer groups (11 operations),
default tasks (10 operations), auto-merge conditions (9 operations), and
auto-decline conditions (6 operations) — up to 45 operations in the
default (10.3) store. **This is standing policy that shapes every future
pull request automatically, not the execution of one PR's lifecycle** —
for driving a single PR through review and merge, use
`bitbucket_workflow_pull_requests` instead; fetch this workflow only when
the user's real goal is configuring a rule, not acting on one PR.

**Version hedge — default tasks are absent before 9.4.** Confirm the
active API version (or notice a `search` for "default task" comes back
empty) before assuming default-task automation is available: it exists
from 9.4 onward but not in 8.19 (35 of the operations above apply there,
not 45). If it's unavailable, tell the user rather than searching
indefinitely for an operation that doesn't exist on their version.

**This sub-workflow is self-contained and delegable.** If routed here
from `bitbucket_workflow`'s menu, or your environment otherwise supports
running this as its own isolated sub-task, run it that way and report
back only a short summary — what's configured, and anything skipped —
not the full step-by-step trace.

**Search by capability, never by a fixed operation name.** `operationId`s
are not stable across this server's supported API versions. Always
`search` by capability, then `get` the resolved operation to confirm its
real path and schema before calling it.

## Step 0 — gather required parameters

Check the header above. You need `project_key` at minimum. `repo_slug` is
optional — see Step 1.

## Step 1 — fork: project scope or repository scope?

Nearly every operation here exists at both project scope and repository
scope (confirmed directly: reviewer groups, auto-merge, and auto-decline
each have a project-level path and a separate repository-level path). If
`repo_slug` is supplied, target that repository directly. If it's
omitted, confirm with the user that they actually want project-wide rules
(which cascade to every repository in the project unless a
repository-level rule overrides them) rather than silently defaulting to
one scope or the other — this is a broader-impact choice than most other
workflows make by default.

## Step 2 — prerequisite: reviewer groups before group-based conditions

If the user wants a default-reviewer condition that targets a group of
reviewers rather than named individuals, the group must exist first.
Search for how to list reviewer groups at the chosen scope; if the target
group doesn't exist, create it before configuring any default-reviewer
condition that references it by group.

## Step 3 — configure default-reviewer conditions

Search for how to create a default-reviewer condition at the chosen
scope (branch matcher pair, required approval count, and the
group/users from Step 2 if applicable). Read the created condition back
to confirm it matches what was requested, not just that the call
succeeded.

## Step 4 — default tasks (9.4+ only — see the version hedge above)

If available on the active version, search for how to create a
default-task rule at the chosen scope. Skip this step entirely — and say
so — if the version hedge above rules it out.

## Step 5 — auto-merge and auto-decline, two independent conditions

Once the conditions above exist, configuring auto-merge and auto-decline
rules is independent of each other and of Steps 2-4 — call this out as
parallelizable and, if the environment supports it, delegate each to its
own sub-task, returning only a short confirmation.

## Step 6 — verify

List the rules actually configured at the chosen scope back to the user
and confirm they match what was requested — don't rely on the creation
calls not erroring as proof they saved correctly.

## Step 7 — summarize

Report what's configured (default reviewers, groups, default tasks,
auto-merge, auto-decline) and anything skipped due to the version hedge.

## Composing with other workflows

These rules gate future PRs automatically; they don't affect PRs already
open. For checking or acting on an existing PR's reviewers, tasks, or
merge readiness, fetch `bitbucket_workflow_pull_requests` instead of
duplicating that content here.
