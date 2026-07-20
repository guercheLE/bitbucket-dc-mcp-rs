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
| `bitbucket_workflow_projects` | Manage a project's lifecycle, settings, permissions, or avatar |
| `bitbucket_workflow_repositories` | Manage a repository's lifecycle, browse its contents, or manage forks/settings |
| `bitbucket_workflow_pull_requests` | Create, review, or merge a pull request |
| `bitbucket_workflow_branches_commits` | Work with commits, branches, branch permissions, tags, or compare/diff |
| `bitbucket_workflow_webhooks` | Set up or manage project/repo webhooks |
| `bitbucket_workflow_access_tokens_keys` | Manage access tokens, SSH keys, GPG keys, or commit signing |
| `bitbucket_workflow_secret_scanning` | Configure or review secret scanning |
| `bitbucket_workflow_admin` | Manage users, groups, permissions, license, or cluster/global settings |
| `bitbucket_workflow_build_integration` | Report build status, manage required-build merge checks, or Jira dev-panel linkage |
| `bitbucket_workflow_mirroring` | Set up or manage Smart Mirroring (upstream servers, mirror servers) |
| `bitbucket_workflow_monitoring_diagnostics` | Check a read-only signal: indexing status, audit log, insights, application properties |

## Routing guidance

Match the user's stated goal (if given, above) or their message to exactly
one row above. If it plausibly spans two (e.g. "set up branch protection
and require a green build before merge" touches both
`bitbucket_workflow_branches_commits` and
`bitbucket_workflow_build_integration`), start with whichever workflow
owns the primary resource the user is trying to change, and mention the
other by name rather than duplicating its content here. If nothing above
fits, fall back to `search`/`get`/`call` directly rather than forcing a
mismatched workflow.
