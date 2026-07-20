# Bitbucket workflow: pull requests

Guides creating a pull request, gathering reviews, and merging it — the single
largest and most-used Bitbucket Data Center domain (44 operations).

**This sub-workflow is self-contained and delegable.** If you were routed
here from `bitbucket`'s menu, or your environment otherwise
supports running this as its own isolated sub-task (an agent/task tool),
run it that way: everything you need is in this prompt's own text plus the
"Context already provided" parameters above. Report back to whoever
delegated here only a short summary — what was created/merged/still
pending — not the full step-by-step tool trace.

**Every operation reference below is a capability to search for, never a
fixed name.** `operationId`s are not stable across this server's supported
API versions — the same id can resolve to a completely different endpoint
depending on which version is active. Always `search` by capability
("how do I create a pull request?"), then `get` the resolved operation to
confirm its actual path, method, and current schema before calling it.
Never assume a response field name either — read the schema `get` returns.

## Step 0 — gather required parameters

Check the header above first. You need at minimum `project_key`,
`repo_slug`, and `source_branch` before proceeding — ask the user for
whatever's still missing. `target_branch` may default to the repository's
configured default branch if the user doesn't state one; confirm that
default with the user rather than assuming it silently.

## Step 1 — read the repository's merge configuration first

Search for how to fetch a repository's pull-request/merge settings:
required approval count, required builds, the merge-check strategy, and
any unapproved-reviewer rules. Read the actual result — these are
per-repository settings, not global defaults, and they define every gate
condition used in Step 4 and the merge strategy used in Step 5. Do not
proceed on an assumed default.

## Step 2 — fork: does an open PR already exist for this branch pair?

Search for open pull requests on the repository, filtered to the
source/target branch pair.

- **(A) None exists** — continue to Step 3 to create one.
- **(B) One already exists** — skip Step 3 and go straight to Step 4's
  status check against the existing PR, rather than creating a duplicate.

Ask the user only if the search returns multiple ambiguous candidates.

## Step 3 — create the PR, then two independent follow-ups

Create the pull request itself first. Once it exists, adding non-default
reviewers and setting a PR-level default task are independent of each
other and of anything else — call this out as safe to run concurrently,
and as a candidate for delegation: *if your environment provides a way to
run a sub-task in its own context, delegate "add reviewer X" and "add
default task Y" as separate sub-tasks and have each return only a short
confirmation, not the full request/response body. If no such mechanism is
available, just make both calls directly here.*

## Step 4 — the merge gate

Check every condition Step 1 actually identified for this repository —
don't assume a generic default:

- Required approval count met (search for how to list a PR's current
  reviewers/approvals).
- Required builds green, if the repo requires them (search for how to
  check build status for the PR's latest commit).
- No unresolved blocking tasks on the PR.

If any condition isn't satisfied, summarize what's still missing and stop
— don't poll in a tight loop waiting for it to change.

## Step 5 — merge, branching on Step 1's configured strategy

Merge using whichever strategy the repository's settings specify (merge
commit, squash, or fast-forward) — never assume one. After merging,
confirm the PR's state actually changed to merged (not just that the call
returned without an error) before declaring success.

## Step 6 — summarize and offer a follow-up

Report what was accomplished (PR created/merged/still pending and why).
Offer to delete the now-merged source branch if the user wants.

## Also in scope (brief)

Smaller PR-adjacent capabilities that don't need their own numbered
step: comment likes/reactions (path shape differs by version, so search
rather than assuming one), watching a PR, participants (distinct from
required reviewers), blocker comments, and rebasing as an alternative to
Step 5's merge when the repository's strategy allows it. For "what's
waiting on me across every repository", search for the inbox/dashboard
operations rather than repeating Step 2's per-repository PR search.

For configuring *standing* default-reviewer/auto-merge/auto-decline
rules that apply automatically to future PRs — as opposed to acting on
one already-open PR — fetch `bitbucket-pr-rules` instead.

## Composing with other workflows

Step 1's merge-settings lookup and Step 4's build-status check overlap
with `bitbucket-repositories` and `bitbucket-build-integration`
respectively — fetch those prompts by name for more detail rather than
duplicating their content here.
