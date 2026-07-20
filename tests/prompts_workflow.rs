//! Protocol-level `prompts/list`/`prompts/get` integration tests, kept
//! deliberately separate from `src/core/mcp_server.rs`'s own
//! `#[cfg(test)] mod tests` (which stays scoped to `search`/`get`/`call`).
//! Exercises the crate's public API the same way a real MCP client would,
//! mirroring `mcp_protocol_routes_search_get_and_call_requests`'s
//! `tokio::io::duplex` + `ClientHandler` stub pattern.

use std::sync::Arc;

use bitbucket_dc_mcp::auth::auth_manager::AuthManager;
use bitbucket_dc_mcp::core::config_schema::{AuthMethod, Config};
use bitbucket_dc_mcp::core::mcp_server::McpifyServer;
use rmcp::model::{GetPromptRequestParams, ProtocolVersion};
use rmcp::{ServerHandler, ServiceExt};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "basic"
    }))
    .unwrap();
    McpifyServer::new(
        "10.3".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::Basic))),
    )
}

const EXPECTED_PROMPT_NAMES: &[&str] = &[
    "bitbucket",
    "bitbucket-access-tokens-keys",
    "bitbucket-admin",
    "bitbucket-branches-commits",
    "bitbucket-build-integration",
    "bitbucket-mesh",
    "bitbucket-mirroring",
    "bitbucket-monitoring-diagnostics",
    "bitbucket-pr-rules",
    "bitbucket-projects",
    "bitbucket-pull-requests",
    "bitbucket-repositories",
    "bitbucket-secret-scanning",
    "bitbucket-webhooks",
];

#[test]
fn server_info_advertises_the_prompts_capability() {
    let info = server().get_info();
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    assert!(info.capabilities.prompts.is_some());
    assert!(info.instructions.unwrap().contains("bitbucket"));
}

#[tokio::test]
async fn mcp_protocol_lists_every_workflow_prompt_with_optional_arguments() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let prompts = client.list_all_prompts().await.unwrap();
    let mut names: Vec<&str> = prompts.iter().map(|p| p.name.as_ref()).collect();
    names.sort_unstable();
    assert_eq!(names, EXPECTED_PROMPT_NAMES);

    let pull_requests = prompts
        .iter()
        .find(|p| p.name == "bitbucket-pull-requests")
        .expect("bitbucket-pull-requests should be listed");
    let arguments = pull_requests
        .arguments
        .as_ref()
        .expect("pull_requests prompt should advertise arguments");
    let arg_names: Vec<&str> = arguments.iter().map(|a| a.name.as_ref()).collect();
    for expected in ["project_key", "repo_slug", "source_branch", "target_branch"] {
        assert!(
            arg_names.contains(&expected),
            "expected argument `{expected}` in {arg_names:?}"
        );
    }
    for prompt in &prompts {
        if let Some(arguments) = &prompt.arguments {
            for arg in arguments {
                assert_eq!(
                    arg.required,
                    Some(false),
                    "prompt `{}` argument `{}` must never be required — \"ask if missing\" \
                     lives in the prose, not transport-level validation",
                    prompt.name,
                    arg.name
                );
            }
        }
    }

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn master_prompt_links_to_the_pull_requests_sub_workflow() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(GetPromptRequestParams::new("bitbucket"))
        .await
        .unwrap();
    let text = prompt_text(&result);
    assert!(text.contains("bitbucket-pull-requests"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn pull_requests_prompt_echoes_supplied_args_and_lists_missing_ones() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(
            GetPromptRequestParams::new("bitbucket-pull-requests").with_arguments(
                serde_json::json!({
                    "project_key": "PROJ",
                    "repo_slug": "my-repo"
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        )
        .await
        .unwrap();
    let text = prompt_text(&result);
    assert!(text.contains("`project_key`: PROJ"));
    assert!(text.contains("`repo_slug`: my-repo"));
    assert!(text.contains("`source_branch`"));
    assert!(text.contains("`target_branch`"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn every_workflow_prompt_returns_non_empty_guided_text() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    for name in EXPECTED_PROMPT_NAMES {
        let result = client
            .get_prompt(GetPromptRequestParams::new(*name))
            .await
            .unwrap_or_else(|err| panic!("prompts/get for `{name}` failed: {err}"));
        let text = prompt_text(&result);
        assert!(
            text.contains("## Context already provided"),
            "`{name}` should prepend the context header"
        );
        assert!(
            text.len() > 100,
            "`{name}` should return substantial guided content, got {} chars",
            text.len()
        );
    }

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

fn prompt_text(result: &rmcp::model::GetPromptResult) -> String {
    result
        .messages
        .iter()
        .map(|message| match &message.content {
            rmcp::model::ContentBlock::Text(text_content) => text_content.text.clone(),
            other => panic!("expected text content, got {other:?}"),
        })
        .collect::<Vec<_>>()
        .join("\n")
}
