//! Multi-turn local BYOP controller for `factor-oss agent run --runner local`.
//!
//! This is intentionally lightweight: it drives `agent_providers::chat_stream::generate_byop_output`
//! directly, executes a small set of local tools (`run_shell_command`, `read_files`,
//! `start_agent`), and loops until the model emits a final text answer.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context as _;
use futures::StreamExt;
use serde_json::json;
use uuid::Uuid;
use warp_multi_agent_api as api;

use crate::ai::agent::api::RequestParams;
use crate::ai::agent::{AIAgentInput, UserQueryMode};
use crate::ai::agent_providers::{attachment_caps::AttachmentCaps, chat_stream};
use crate::ai::blocklist::SessionContext;
use crate::settings::{AgentProvider, ReasoningEffortSetting};

/// Maximum number of model↔tool turns allowed before we stop to avoid runaway loops.
const MAX_TURNS: usize = 16;

/// Maximum depth of nested `start_agent` calls in the local CLI runner.
const MAX_AGENT_DEPTH: usize = 2;

/// Context passed to local tool executors so nested tools (e.g. `start_agent`)
/// can run with the same BYOP provider and settings as the parent.
struct ToolContext {
    provider: AgentProvider,
    api_key: String,
    model_id: String,
    cwd: Option<PathBuf>,
    depth: usize,
    max_depth: usize,
}

/// Execute a multi-turn BYOP agent conversation for the CLI local runner.
pub async fn run_byop_local_agent(
    provider: &AgentProvider,
    api_key: String,
    model_id: String,
    prompt: String,
    cwd: Option<&Path>,
    output_json: bool,
) -> anyhow::Result<String> {
    run_byop_local_agent_inner(
        provider.clone(),
        api_key,
        model_id,
        prompt,
        cwd.map(Path::to_path_buf),
        output_json,
        0,
        MAX_AGENT_DEPTH,
    )
    .await
}

async fn run_byop_local_agent_inner(
    provider: AgentProvider,
    api_key: String,
    model_id: String,
    prompt: String,
    cwd: Option<PathBuf>,
    _output_json: bool,
    depth: usize,
    max_depth: usize,
) -> anyhow::Result<String> {
    let root_task_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();
    let root_task = api::Task {
        id: root_task_id.clone(),
        messages: Vec::new(),
        dependencies: None,
        description: String::new(),
        summary: String::new(),
        server_data: String::new(),
    };

    let user_input = AIAgentInput::UserQuery {
        query: prompt,
        context: Arc::from([]),
        static_query_type: None,
        referenced_attachments: HashMap::new(),
        user_query_mode: UserQueryMode::Normal,
        running_command: None,
        intended_agent: None,
    };

    let model = crate::ai::llms::LLMId::from(format!("byop:{}:{}", provider.id, model_id));

    let session_context = match cwd.as_deref() {
        Some(cwd) => SessionContext::new_empty().with_cwd(cwd.to_string_lossy().to_string()),
        None => SessionContext::new_empty(),
    };

    let mut params =
        RequestParams::new_local_run(vec![user_input], vec![root_task], model, session_context);
    // The local CLI runner only executes a small subset of tools; limiting the
    // tool list keeps on-device endpoints (e.g. `fm serve` Apple Foundation
    // Models) within their small context windows.
    params.supported_tools_override = Some(vec![
        api::ToolType::RunShellCommand,
        api::ToolType::ReadFiles,
        api::ToolType::StartAgentV2,
    ]);

    let tool_ctx = ToolContext {
        provider,
        api_key,
        model_id,
        cwd,
        depth,
        max_depth,
    };

    let mut final_text = String::new();
    let mut turn = 0;
    let mut needs_create_task = true;

    while turn < MAX_TURNS {
        turn += 1;

        let input = chat_stream::ByopOutputInput {
            params: params.clone(),
            base_url: tool_ctx.provider.base_url.clone(),
            api_key: tool_ctx.api_key.clone(),
            model_id: tool_ctx.model_id.clone(),
            api_type: tool_ctx.provider.api_type,
            reasoning_effort: ReasoningEffortSetting::default(),
            extra_headers: tool_ctx.provider.extra_headers.clone(),
            task_id: root_task_id.clone(),
            target_task_id: root_task_id.clone(),
            needs_create_task,
            lrc_command_id: None,
            lrc_should_spawn_subagent: false,
            context_window: None,
            cancellation_rx: futures::channel::oneshot::channel().1,
            attachment_caps: AttachmentCaps::default(),
        };

        let mut stream = chat_stream::generate_byop_output(input)
            .await
            .map_err(|e| anyhow::anyhow!("BYOP generate output failed: {e:?}"))?;

        let mut pending_tool_calls: Vec<api::message::ToolCall> = Vec::new();

        while let Some(event) = stream.next().await {
            let event = event.map_err(|e| anyhow::anyhow!("BYOP stream error: {e:?}"))?;
            match event.r#type {
                Some(api::response_event::Type::ClientActions(client_actions)) => {
                    let task = params.tasks.first_mut().context("root task not found")?;
                    for action in client_actions.actions {
                        match action.action {
                            Some(api::client_action::Action::CreateTask(create)) => {
                                if let Some(created) = create.task {
                                    if created.id == root_task_id {
                                        // Root task already exists in our params; nothing to do.
                                    }
                                }
                            }
                            Some(api::client_action::Action::AddMessagesToTask(add)) => {
                                if add.task_id == root_task_id {
                                    for msg in &add.messages {
                                        classify_message(
                                            msg,
                                            &mut pending_tool_calls,
                                            &mut final_text,
                                        );
                                    }
                                    task.messages.extend(add.messages.clone());
                                }
                            }
                            Some(api::client_action::Action::AppendToMessageContent(append)) => {
                                if append.task_id == root_task_id {
                                    if let Some(msg) = append.message.as_ref() {
                                        apply_append_to_message(task, msg, &mut final_text);
                                    }
                                }
                            }
                            Some(api::client_action::Action::UpdateTaskMessage(update)) => {
                                if update.task_id == root_task_id {
                                    if let Some(msg) = update.message.as_ref() {
                                        apply_update_message(task, msg, &mut pending_tool_calls);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Some(api::response_event::Type::Finished(finished)) => {
                    if let Some(reason) = finished.reason {
                        match reason {
                            api::response_event::stream_finished::Reason::Done(_)
                            | api::response_event::stream_finished::Reason::Other(_) => {
                                // Natural completion; let the outer loop decide whether
                                // to execute pending tool calls or finish.
                                break;
                            }
                            api::response_event::stream_finished::Reason::InternalError(err) => {
                                if final_text.is_empty() && pending_tool_calls.is_empty() {
                                    return Err(anyhow::anyhow!(
                                        "BYOP stream error: {}",
                                        err.message
                                    ));
                                } else {
                                    return Ok(final_text);
                                }
                            }
                            _ => {
                                return Err(anyhow::anyhow!(
                                    "BYOP stream finished with reason: {reason:?}"
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if pending_tool_calls.is_empty() {
            return Ok(final_text);
        }

        // Execute pending tools and append results to the task history.
        let mut result_messages = Vec::new();
        for tool_call in pending_tool_calls {
            let result = execute_tool(&tool_call, &tool_ctx).await;
            result_messages.push(api::Message {
                id: Uuid::new_v4().to_string(),
                task_id: root_task_id.clone(),
                request_id: request_id.clone(),
                timestamp: None,
                server_message_data: result,
                citations: vec![],
                message: Some(api::message::Message::ToolCallResult(
                    api::message::ToolCallResult {
                        tool_call_id: tool_call.tool_call_id.clone(),
                        context: None,
                        result: None,
                    },
                )),
            });
        }

        let task = params
            .tasks
            .iter_mut()
            .find(|t| t.id == root_task_id)
            .context("root task not found")?;
        task.messages.extend(result_messages);

        // Subsequent turns have no new user input and no need to create the root task again.
        params.input.clear();
        needs_create_task = false;
    }

    Err(anyhow::anyhow!(
        "Local BYOP agent reached the maximum number of turns ({MAX_TURNS}) without finishing."
    ))
}

fn classify_message(
    msg: &api::Message,
    pending_tool_calls: &mut Vec<api::message::ToolCall>,
    final_text: &mut String,
) {
    match &msg.message {
        Some(api::message::Message::AgentOutput(out)) => {
            final_text.push_str(&out.text);
        }
        Some(api::message::Message::ToolCall(tool_call)) => {
            pending_tool_calls.push(tool_call.clone());
        }
        _ => {}
    }
}

fn apply_append_to_message(task: &mut api::Task, msg: &api::Message, final_text: &mut String) {
    if let Some(existing) = task.messages.iter_mut().find(|m| m.id == msg.id) {
        match (&mut existing.message, &msg.message) {
            (
                Some(api::message::Message::AgentOutput(out)),
                Some(api::message::Message::AgentOutput(append)),
            ) => {
                out.text.push_str(&append.text);
                final_text.push_str(&append.text);
            }
            (
                Some(api::message::Message::AgentReasoning(out)),
                Some(api::message::Message::AgentReasoning(append)),
            ) => {
                out.reasoning.push_str(&append.reasoning);
            }
            _ => {}
        }
    } else {
        // The message does not yet exist in our local task; treat it as an add.
        task.messages.push(msg.clone());
        if let Some(api::message::Message::AgentOutput(out)) = &msg.message {
            final_text.push_str(&out.text);
        }
    }
}

fn apply_update_message(
    task: &mut api::Task,
    msg: &api::Message,
    pending_tool_calls: &mut Vec<api::message::ToolCall>,
) {
    if let Some(existing) = task.messages.iter_mut().find(|m| m.id == msg.id) {
        if let Some(api::message::Message::ToolCall(updated)) = &msg.message {
            if let Some(existing_tool) = existing.message.as_mut() {
                if let api::message::Message::ToolCall(existing_call) = existing_tool {
                    *existing_call = updated.clone();
                    if let Some(pos) = pending_tool_calls
                        .iter()
                        .position(|t| t.tool_call_id == updated.tool_call_id)
                    {
                        pending_tool_calls[pos] = updated.clone();
                    }
                    return;
                }
            }
        }
        // For non-tool updates, just replace the whole message.
        *existing = msg.clone();
    } else {
        task.messages.push(msg.clone());
        if let Some(api::message::Message::ToolCall(tool_call)) = &msg.message {
            pending_tool_calls.push(tool_call.clone());
        }
    }
}

async fn execute_tool(tool_call: &api::message::ToolCall, ctx: &ToolContext) -> String {
    if let Some(tool) = &tool_call.tool {
        match tool {
            api::message::tool_call::Tool::RunShellCommand(shell) => {
                return execute_run_shell_command(shell, ctx).await;
            }
            api::message::tool_call::Tool::ReadFiles(read) => {
                return execute_read_files(read, ctx).await;
            }
            api::message::tool_call::Tool::StartAgentV2(start) => {
                return execute_start_agent(start, ctx).await;
            }
            _ => {}
        }
    }

    let fallback = if tool_call.tool.is_none() {
        "Tool payload was not parsed"
    } else {
        "Tool not supported in local CLI runner"
    };
    serde_json::to_string(&json!({
        "status": "error",
        "message": fallback,
    }))
    .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned())
}

async fn execute_start_agent(start: &api::StartAgentV2, ctx: &ToolContext) -> String {
    // Enforce the local-only execution model for Factor.
    let is_local = start
        .execution_mode
        .as_ref()
        .and_then(|m| m.mode.as_ref())
        .map(|m| matches!(m, api::start_agent_v2::execution_mode::Mode::Local(_)))
        .unwrap_or(true);
    if !is_local {
        return serde_json::to_string(&json!({
            "status": "error",
            "error": "Cloud/remote child agents are disabled in Factor; only local start_agent is supported.",
        }))
        .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned());
    }

    if ctx.depth >= ctx.max_depth {
        return serde_json::to_string(&json!({
            "status": "error",
            "error": format!("Maximum local agent nesting depth ({}) reached.", ctx.max_depth),
        }))
        .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned());
    }

    let agent_id = Uuid::new_v4().to_string();

    // Box the recursive async call so the parent future remains a fixed size.
    let child_fut = Box::pin(run_byop_local_agent_inner(
        ctx.provider.clone(),
        ctx.api_key.clone(),
        ctx.model_id.clone(),
        start.prompt.clone(),
        ctx.cwd.clone(),
        false,
        ctx.depth + 1,
        ctx.max_depth,
    ));
    let child_result = child_fut.await;

    match child_result {
        Ok(result) => serde_json::to_string(&json!({
            "status": "started",
            "agent_id": agent_id,
            "result": result,
        }))
        .unwrap_or_else(|_| {
            json!({
                "status": "started",
                "agent_id": agent_id,
                "result": result,
            })
            .to_string()
        }),
        Err(e) => serde_json::to_string(&json!({
            "status": "error",
            "agent_id": agent_id,
            "error": format!("{e:?}"),
        }))
        .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned()),
    }
}

async fn execute_run_shell_command(
    shell: &api::message::tool_call::RunShellCommand,
    ctx: &ToolContext,
) -> String {
    let mut command = std::process::Command::new("sh");
    command.arg("-c").arg(&shell.command);
    if let Some(cwd) = ctx.cwd.as_deref() {
        command.current_dir(cwd);
    }

    // Run with a generous but safe timeout. Long-running commands are not supported in this
    // lightweight CLI loop; the model should set `wait_until_complete=false` for those.
    let handle = tokio::task::spawn_blocking(move || command.output());
    match tokio::time::timeout(std::time::Duration::from_secs(120), handle).await {
        Ok(Ok(Ok(output))) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{stdout}{stderr}").trim().to_owned();
            serde_json::to_string(&json!({
                "status": "completed",
                "command": shell.command,
                "exit_code": output.status.code().unwrap_or(-1),
                "output": combined,
            }))
        }
        Ok(Ok(Err(e))) => serde_json::to_string(&json!({
            "status": "error",
            "command": shell.command,
            "message": format!("{e:?}"),
        })),
        Ok(Err(_)) => serde_json::to_string(&json!({
            "status": "error",
            "command": shell.command,
            "message": "Command task was cancelled",
        })),
        Err(_) => serde_json::to_string(&json!({
            "status": "error",
            "command": shell.command,
            "message": "Command timed out after 120 seconds",
        })),
    }
    .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned())
}

async fn execute_read_files(
    read: &api::message::tool_call::ReadFiles,
    ctx: &ToolContext,
) -> String {
    let mut files_json = Vec::new();
    let base = ctx.cwd.as_deref().unwrap_or(Path::new(""));

    for file in &read.files {
        let path = base.join(&file.name);
        match tokio::fs::read_to_string(&path).await {
            Ok(content) => {
                files_json.push(json!({
                    "path": file.name,
                    "content": content,
                }));
            }
            Err(e) => {
                files_json.push(json!({
                    "path": file.name,
                    "content": "",
                    "error": format!("{e}"),
                }));
            }
        }
    }

    serde_json::to_string(&json!({
        "status": "ok",
        "files": files_json,
    }))
    .unwrap_or_else(|_| r#"{"status":"error"}"#.to_owned())
}
