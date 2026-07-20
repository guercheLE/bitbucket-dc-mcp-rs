//! The `#[prompt_router]`-decorated `impl McpifyServer` block — one method
//! per guided workflow prompt. Kept in its own file, separate from the
//! `#[tool_router]`-decorated block in `src/core/mcp_server.rs`, mirroring
//! this crate's existing separation between tool business logic
//! (`src/tools/`) and MCP wiring.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{PromptMessage, Role};
use rmcp::{prompt, prompt_router};

use crate::core::mcp_server::McpifyServer;
use crate::prompts::{
    MasterWorkflowArgs, MeshWorkflowArgs, MirroringWorkflowArgs, PrRulesWorkflowArgs,
    PullRequestsWorkflowArgs, render_context_header,
};

// `vis = pub`: the generated `prompt_router()` associated fn must be
// callable from `core::mcp_server`'s `McpifyServer::new`, a different
// module than this one — the macro defaults to module-private otherwise.
#[prompt_router(vis = "pub")]
impl McpifyServer {
    #[prompt(
        name = "bitbucket_workflow",
        description = "Start here. Presents the available Bitbucket Data Center management \
                        workflows, routes to the right guided sub-workflow based on the user's \
                        goal, and — where the environment supports it — delegates that whole \
                        sub-workflow to an isolated sub-task to spare this conversation's \
                        context window."
    )]
    async fn bitbucket_workflow_prompt(
        &self,
        Parameters(args): Parameters<MasterWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("goal", args.goal.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/master.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_pull_requests",
        description = "Guided pull request flow: check the repository's merge/approval \
                        configuration, create the PR (or find an existing one), add \
                        reviewers and tasks, gate on approvals/builds/unresolved tasks, then \
                        merge using the repository's configured strategy."
    )]
    async fn bitbucket_workflow_pull_requests_prompt(
        &self,
        Parameters(args): Parameters<PullRequestsWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("project_key", args.project_key.as_deref()),
            ("repo_slug", args.repo_slug.as_deref()),
            ("source_branch", args.source_branch.as_deref()),
            ("target_branch", args.target_branch.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/pull_requests.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_projects",
        description = "Project lifecycle, settings, permissions, avatar, and restrictions."
    )]
    async fn bitbucket_workflow_projects_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/projects.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_repositories",
        description = "Repository lifecycle, browsing contents (files/raw/readme/license/\
                        archive), forks, related repos, and settings."
    )]
    async fn bitbucket_workflow_repositories_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/repositories.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_branches_commits",
        description = "Commits, branches, branch permissions/restrictions, branch-utils, tags, \
                        and compare/diff between refs."
    )]
    async fn bitbucket_workflow_branches_commits_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/branches_commits.md")
            ),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_webhooks",
        description = "Project- and repository-scoped webhook lifecycle."
    )]
    async fn bitbucket_workflow_webhooks_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/webhooks.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_access_tokens_keys",
        description = "Access tokens (user/project/repo scoped), SSH keys, GPG keys, and \
                        commit signing."
    )]
    async fn bitbucket_workflow_access_tokens_keys_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/access_tokens_keys.md")
            ),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_secret_scanning",
        description = "Project- and repository-scoped secret scanning settings, findings, and \
                        allowlisting."
    )]
    async fn bitbucket_workflow_secret_scanning_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/secret_scanning.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_admin",
        description = "Instance-wide users, groups, permissions, license, and cluster/global \
                        settings."
    )]
    async fn bitbucket_workflow_admin_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/admin.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_build_integration",
        description = "Build status on commits, required-builds merge checks, and Jira \
                        dev-panel linkage."
    )]
    async fn bitbucket_workflow_build_integration_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/build_integration.md")
            ),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_pr_rules",
        description = "Project- or repository-scoped standing PR automation rules: default \
                        reviewers, reviewer groups, default tasks (9.4+), auto-merge, and \
                        auto-decline — policy that shapes future pull requests, distinct \
                        from driving a single PR's lifecycle."
    )]
    async fn bitbucket_workflow_pr_rules_prompt(
        &self,
        Parameters(args): Parameters<PrRulesWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("project_key", args.project_key.as_deref()),
            ("repo_slug", args.repo_slug.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/pr_rules.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_mirroring",
        description = "Guided Smart Mirroring setup: upstream server, mirror server \
                        registration, acceptance, and sync verification."
    )]
    async fn bitbucket_workflow_mirroring_prompt(
        &self,
        Parameters(args): Parameters<MirroringWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("upstream_project_key", args.upstream_project_key.as_deref()),
            ("upstream_repo_slug", args.upstream_repo_slug.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/mirroring.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_mesh",
        description = "Guided Bitbucket Mesh setup: register a mesh node, verify its \
                        connectivity, then preview/start/monitor a repository migration \
                        job onto it. Not to be confused with the general instance-data \
                        migration covered by bitbucket_workflow_admin."
    )]
    async fn bitbucket_workflow_mesh_prompt(
        &self,
        Parameters(args): Parameters<MeshWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("project_key", args.project_key.as_deref()),
            ("repo_slug", args.repo_slug.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/mesh.md")),
        )]
    }

    #[prompt(
        name = "bitbucket_workflow_monitoring_diagnostics",
        description = "Thin pointer to the right read-only signal: indexing status, audit \
                        log, insights reports, application properties."
    )]
    async fn bitbucket_workflow_monitoring_diagnostics_prompt(&self) -> Vec<PromptMessage> {
        let header = render_context_header(&[]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/monitoring_diagnostics.md")
            ),
        )]
    }
}
