# Bitbucket Data Center guided workflows

Menu of guided, multi-step workflows for common Bitbucket Data Center
management tasks. Each links to its own prompt — fetch it by name via
`prompts/get` once you've matched the user's goal below.

**Delegate the whole matched sub-workflow if you can.** Once you've picked
a sub-workflow, check whether your environment can run an isolated
sub-task (an agent/task tool). If so, delegate the *entire* sub-workflow to
it — pass the sub-workflow's prompt name and whatever parameters are
already known, let the sub-task fetch that prompt itself and carry out
every step (including all of its own `search`/`get`/`call` traffic) in its
own context, and have it report back only a short summary: what was
accomplished/confirmed, and anything still needed from the user. Only run
the sub-workflow's steps directly in this conversation if no delegation
mechanism is available — a single sub-workflow can easily produce far more
intermediate tool traffic than its final summary needs to convey.

## Workflows

| Prompt | Use when the user wants to... |
|---|---|
| `bitbucket-projects` | Manage a project's lifecycle, settings, permissions, or avatar |
| `bitbucket-repositories` | Manage a repository's lifecycle, browse its contents, or manage forks/settings |
| `bitbucket-pull-requests` | Create, review, or merge a pull request |
| `bitbucket-branches-commits` | Work with commits, branches, branch permissions, tags, or compare/diff |
| `bitbucket-webhooks` | Set up or manage project/repo webhooks |
| `bitbucket-access-tokens-keys` | Manage access tokens, SSH keys, GPG keys, or commit signing |
| `bitbucket-secret-scanning` | Configure or review secret scanning |
| `bitbucket-admin` | Manage users, groups, permissions, license, or cluster/global settings |
| `bitbucket-build-integration` | Report build status, manage required-build merge checks, or Jira dev-panel linkage |
| `bitbucket-pr-rules` | Configure standing PR automation rules — default reviewers, reviewer groups, default tasks, auto-merge, auto-decline |
| `bitbucket-mirroring` | Set up or manage Smart Mirroring (upstream servers, mirror servers) |
| `bitbucket-mesh` | Register a Bitbucket Mesh node and migrate repositories onto it |
| `bitbucket-monitoring-diagnostics` | Check a read-only signal: indexing status, audit log, insights, application properties |

## Routing guidance

Match the user's stated goal (if given, above) or their message to exactly
one row above. If it plausibly spans two (e.g. "set up branch protection
and require a green build before merge" touches both
`bitbucket-branches-commits` and
`bitbucket-build-integration`), start with whichever workflow
owns the primary resource the user is trying to change, and mention the
other by name rather than duplicating its content here. If nothing above
fits, fall back to `search`/`get`/`call` directly rather than forcing a
mismatched workflow.

Don't confuse `bitbucket-pull-requests` (driving one PR through
review and merge) with `bitbucket-pr-rules` (configuring the
standing default-reviewer/auto-merge/auto-decline rules that apply
automatically to future PRs) — route on which one the user actually
means.
