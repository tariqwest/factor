//! Deterministic FakeProvider for Factor BYOP / multi-agent TDD.
//!
//! Inject at the `ResponseStream` boundary (same shape as OpenRouter / BYOP streams)
//! so CI can exercise multi-turn tool loops without real API keys or HTTP.
//!
//! See `docs/tech/TDD.md` §4 and `.agents/plans/04-tdd-and-quality-gates.md`.

#![cfg(any(test, feature = "test-util"))]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use async_stream::stream;
use futures_lite::StreamExt;
use uuid::Uuid;
use warp_multi_agent_api as api;
use warp_multi_agent_api::response_event::{self, stream_finished};

use super::{RequestParams, ResponseStream};
use crate::ai::agent_providers::tools;

/// One scripted assistant response for a single inference call.
#[derive(Debug, Clone)]
pub enum ScriptedAssistantTurn {
    /// Final assistant text only (no tools).
    Text(String),
    /// One or more tool calls (optionally with preamble text).
    ToolCalls {
        text: Option<String>,
        tools: Vec<ScriptedToolCall>,
    },
}

/// Scripted function/tool call in OpenAI-compatible shape (name + JSON args).
#[derive(Debug, Clone)]
pub struct ScriptedToolCall {
    pub id: String,
    pub name: String,
    /// Raw JSON arguments string (as the model would emit).
    pub arguments: String,
}

impl ScriptedToolCall {
    pub fn run_shell_command(id: impl Into<String>, command: impl Into<String>) -> Self {
        let command = command.into();
        Self {
            id: id.into(),
            name: "run_shell_command".to_owned(),
            arguments: serde_json::json!({
                "command": command,
                "is_read_only": true,
                "wait_until_complete": true,
            })
            .to_string(),
        }
    }

    /// Script a BYOP `start_agent` tool call (local orchestration only).
    pub fn start_agent(
        id: impl Into<String>,
        name: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: "start_agent".to_owned(),
            arguments: serde_json::json!({
                "name": name.into(),
                "prompt": prompt.into(),
            })
            .to_string(),
        }
    }
}

/// Snapshot of a request seen by the fake provider (for assertions).
#[derive(Debug, Clone)]
pub struct RecordedRequest {
    pub primary_task_id: String,
    pub input_count: usize,
    pub task_message_count: usize,
    pub has_tool_call_result: bool,
    pub model_id: String,
}

/// Scripted inference backend: queue of assistant turns + request recorder.
#[derive(Debug)]
pub struct FakeProvider {
    turns: Mutex<VecDeque<ScriptedAssistantTurn>>,
    requests: Mutex<Vec<RecordedRequest>>,
}

impl FakeProvider {
    pub fn new(turns: impl IntoIterator<Item = ScriptedAssistantTurn>) -> Arc<Self> {
        Arc::new(Self {
            turns: Mutex::new(turns.into_iter().collect()),
            requests: Mutex::new(Vec::new()),
        })
    }

    pub fn remaining_turns(&self) -> usize {
        self.turns.lock().expect("turns lock").len()
    }

    pub fn recorded_requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().expect("requests lock").clone()
    }

    /// Produce a `ResponseStream` compatible with `generate_multi_agent_output` consumers.
    pub fn stream(self: &Arc<Self>, params: RequestParams) -> ResponseStream {
        let me = Arc::clone(self);
        Box::pin(stream! {
            me.record_request(&params);

            let turn = {
                let mut turns = me.turns.lock().expect("turns lock");
                turns.pop_front()
            };

            let Some(turn) = turn else {
                yield Ok(finished_event(stream_finished::Reason::InternalError(
                    stream_finished::InternalError {
                        message: "FakeProvider script exhausted".to_owned(),
                    },
                )));
                return;
            };

            let conversation_id = params
                .byop_conversation_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "fake-conversation".to_owned());
            let request_id = Uuid::new_v4().to_string();
            let task_id = if params.primary_task_id.is_empty() {
                "fake-task".to_owned()
            } else {
                params.primary_task_id.clone()
            };
            let model_id = params.model.as_str().to_owned();

            yield Ok(stream_init_event(conversation_id, request_id.clone()));

            if should_create_task(&params, &task_id) {
                yield Ok(create_task_event(&task_id));
            }

            yield Ok(add_messages_event(&task_id, &request_id, &model_id, &turn));
            yield Ok(done_event(model_id));
        })
    }

    fn record_request(&self, params: &RequestParams) {
        let task_message_count = params.tasks.iter().map(|t| t.messages.len()).sum();
        let has_tool_call_result = params.tasks.iter().any(|task| {
            task.messages.iter().any(|message| {
                matches!(
                    message.message,
                    Some(api::message::Message::ToolCallResult(_))
                )
            })
        });
        self.requests
            .lock()
            .expect("requests lock")
            .push(RecordedRequest {
                primary_task_id: params.primary_task_id.clone(),
                input_count: params.input.len(),
                task_message_count,
                has_tool_call_result,
                model_id: params.model.as_str().to_owned(),
            });
    }
}

fn should_create_task(params: &RequestParams, task_id: &str) -> bool {
    !task_id.is_empty() && !params.tasks.iter().any(|task| task.id == *task_id)
}

fn stream_init_event(conversation_id: String, request_id: String) -> api::ResponseEvent {
    api::ResponseEvent {
        r#type: Some(response_event::Type::Init(response_event::StreamInit {
            run_id: conversation_id.clone(),
            conversation_id,
            request_id,
        })),
    }
}

fn create_task_event(task_id: &str) -> api::ResponseEvent {
    api::ResponseEvent {
        r#type: Some(response_event::Type::ClientActions(
            response_event::ClientActions {
                actions: vec![api::ClientAction {
                    action: Some(api::client_action::Action::CreateTask(
                        api::client_action::CreateTask {
                            task: Some(api::Task {
                                id: task_id.to_owned(),
                                messages: vec![],
                                dependencies: None,
                                description: String::new(),
                                summary: String::new(),
                                server_data: String::new(),
                            }),
                        },
                    )),
                }],
            },
        )),
    }
}

fn add_messages_event(
    task_id: &str,
    request_id: &str,
    model_id: &str,
    turn: &ScriptedAssistantTurn,
) -> api::ResponseEvent {
    let mut messages = Vec::new();

    messages.push(api::Message {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_owned(),
        request_id: request_id.to_owned(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::ModelUsed(api::message::ModelUsed {
            model_id: model_id.to_owned(),
            model_display_name: model_id.to_owned(),
            is_fallback: false,
        })),
    });

    let (text, tool_calls) = match turn {
        ScriptedAssistantTurn::Text(t) => (Some(t.clone()), Vec::new()),
        ScriptedAssistantTurn::ToolCalls { text, tools } => (text.clone(), tools.clone()),
    };

    if let Some(text) = text {
        messages.push(api::Message {
            id: Uuid::new_v4().to_string(),
            task_id: task_id.to_owned(),
            request_id: request_id.to_owned(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            message: Some(api::message::Message::AgentOutput(api::message::AgentOutput {
                text,
            })),
        });
    }

    for tc in tool_calls {
        let tool = tools::lookup(&tc.name)
            .and_then(|def| (def.from_args)(&tc.arguments).ok())
            .unwrap_or_else(|| {
                // Fallback: still surface a shell tool so the stream stays valid.
                api::message::tool_call::Tool::RunShellCommand(
                    api::message::tool_call::RunShellCommand {
                        command: format!("# unparsed tool {} args={}", tc.name, tc.arguments),
                        is_read_only: true,
                        uses_pager: false,
                        is_risky: false,
                        citations: vec![],
                        wait_until_complete_value: None,
                        risk_category: 0,
                    },
                )
            });

        messages.push(api::Message {
            id: Uuid::new_v4().to_string(),
            task_id: task_id.to_owned(),
            request_id: request_id.to_owned(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            message: Some(api::message::Message::ToolCall(api::message::ToolCall {
                tool_call_id: tc.id,
                tool: Some(tool),
            })),
        });
    }

    api::ResponseEvent {
        r#type: Some(response_event::Type::ClientActions(
            response_event::ClientActions {
                actions: vec![api::ClientAction {
                    action: Some(api::client_action::Action::AddMessagesToTask(
                        api::client_action::AddMessagesToTask {
                            task_id: task_id.to_owned(),
                            messages,
                        },
                    )),
                }],
            },
        )),
    }
}

fn done_event(model_id: String) -> api::ResponseEvent {
    let mut event = finished_event(stream_finished::Reason::Done(stream_finished::Done {}));
    if let Some(response_event::Type::Finished(finished)) = &mut event.r#type {
        finished.token_usage = vec![stream_finished::TokenUsage {
            model_id,
            total_input: 0,
            output: 0,
            ..Default::default()
        }];
    }
    event
}

fn finished_event(reason: stream_finished::Reason) -> api::ResponseEvent {
    api::ResponseEvent {
        r#type: Some(response_event::Type::Finished(
            response_event::StreamFinished {
                reason: Some(reason),
                ..Default::default()
            },
        )),
    }
}

/// Collect all events from a response stream (test helper).
pub async fn collect_events(mut stream: ResponseStream) -> Vec<api::ResponseEvent> {
    let mut out = Vec::new();
    while let Some(item) = stream.next().await {
        out.push(item.expect("FakeProvider stream should not error"));
    }
    out
}

/// Extract tool calls from streamed `AddMessagesToTask` events.
pub fn extract_tool_calls(events: &[api::ResponseEvent]) -> Vec<api::message::ToolCall> {
    let mut tools = Vec::new();
    for event in events {
        let Some(response_event::Type::ClientActions(actions)) = event.r#type.as_ref() else {
            continue;
        };
        for action in &actions.actions {
            let Some(api::client_action::Action::AddMessagesToTask(add)) = action.action.as_ref()
            else {
                continue;
            };
            for message in &add.messages {
                if let Some(api::message::Message::ToolCall(tc)) = &message.message {
                    tools.push(tc.clone());
                }
            }
        }
    }
    tools
}

/// Extract concatenated agent output text from streamed events.
pub fn extract_agent_text(events: &[api::ResponseEvent]) -> String {
    let mut parts = Vec::new();
    for event in events {
        let Some(response_event::Type::ClientActions(actions)) = event.r#type.as_ref() else {
            continue;
        };
        for action in &actions.actions {
            let Some(api::client_action::Action::AddMessagesToTask(add)) = action.action.as_ref()
            else {
                continue;
            };
            for message in &add.messages {
                if let Some(api::message::Message::AgentOutput(out)) = &message.message {
                    parts.push(out.text.clone());
                }
            }
        }
    }
    parts.join("")
}

/// Build a successful shell tool-call result message (deterministic fixture).
pub fn shell_tool_result_message(
    task_id: &str,
    tool_call_id: &str,
    command: &str,
    output: &str,
) -> api::Message {
    api::Message {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_owned(),
        request_id: "fake-req".to_owned(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::ToolCallResult(
            api::message::ToolCallResult {
                tool_call_id: tool_call_id.to_owned(),
                context: None,
                result: Some(api::message::tool_call_result::Result::RunShellCommand(
                    #[allow(deprecated)]
                    api::RunShellCommandResult {
                        command: command.to_owned(),
                        output: output.to_owned(),
                        exit_code: 0,
                        result: Some(api::run_shell_command_result::Result::CommandFinished(
                            api::ShellCommandFinished {
                                command_id: format!("cmd-{tool_call_id}"),
                                output: output.to_owned(),
                                exit_code: 0,
                            },
                        )),
                    },
                )),
            },
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::agent::{AIAgentInput, UserQueryMode};
    use futures_lite::future::block_on;

    fn user_query(query: &str) -> AIAgentInput {
        AIAgentInput::UserQuery {
            query: query.to_owned(),
            context: Default::default(),
            static_query_type: None,
            referenced_attachments: Default::default(),
            user_query_mode: UserQueryMode::Normal,
            running_command: None,
            intended_agent: None,
        }
    }

    #[test]
    fn multi_turn_shell_then_final() {
        let fake = FakeProvider::new([
            ScriptedAssistantTurn::ToolCalls {
                text: Some("I'll run a command.".to_owned()),
                tools: vec![ScriptedToolCall::run_shell_command("call_1", "echo hi")],
            },
            ScriptedAssistantTurn::Text("Command finished with output: hi".to_owned()),
        ]);

        let task_id = "task-tool-loop";
        let mut params = RequestParams::new_for_test(vec![user_query("say hi via shell")], vec![]);
        params.primary_task_id = task_id.to_owned();

        // --- Turn 1: model requests run_shell_command ---
        let events1 = block_on(collect_events(fake.stream(params.clone())));
        let tool_calls = extract_tool_calls(&events1);
        assert_eq!(tool_calls.len(), 1, "expected one tool call on turn 1");
        assert_eq!(tool_calls[0].tool_call_id, "call_1");

        let tool = tool_calls[0]
            .tool
            .as_ref()
            .expect("tool payload present");
        let api::message::tool_call::Tool::RunShellCommand(shell) = tool else {
            panic!("expected RunShellCommand, got {tool:?}");
        };
        assert_eq!(shell.command, "echo hi");
        assert!(shell.is_read_only);

        // Registry can re-parse the same args JSON (round-trip through tool surface).
        let def = tools::lookup("run_shell_command").expect("shell tool registered");
        let reparsed = (def.from_args)(r#"{"command":"echo hi","is_read_only":true}"#).unwrap();
        assert!(matches!(
            reparsed,
            api::message::tool_call::Tool::RunShellCommand(_)
        ));

        // Simulate executor success and continue conversation with tool result.
        let tool_call_msg = api::Message {
            id: "m-tool".to_owned(),
            task_id: task_id.to_owned(),
            request_id: "r1".to_owned(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            message: Some(api::message::Message::ToolCall(tool_calls[0].clone())),
        };
        let result_msg =
            shell_tool_result_message(task_id, "call_1", "echo hi", "hi\n");

        params.input.clear();
        params.tasks = vec![api::Task {
            id: task_id.to_owned(),
            messages: vec![tool_call_msg, result_msg],
            dependencies: None,
            description: String::new(),
            summary: String::new(),
            server_data: String::new(),
        }];

        // --- Turn 2: model produces final text after seeing tool result ---
        let events2 = block_on(collect_events(fake.stream(params)));
        let final_text = extract_agent_text(&events2);
        assert!(
            final_text.contains("Command finished"),
            "expected final assistant text, got: {final_text:?}"
        );
        assert!(
            extract_tool_calls(&events2).is_empty(),
            "final turn should not request more tools"
        );

        let recorded = fake.recorded_requests();
        assert_eq!(recorded.len(), 2);
        assert!(!recorded[0].has_tool_call_result);
        assert!(
            recorded[1].has_tool_call_result,
            "second request must include tool result for multi-turn continuation"
        );
        assert_eq!(recorded[1].task_message_count, 2);
        assert_eq!(fake.remaining_turns(), 0);
    }

    #[test]
    fn exhausted_script_finishes_with_internal_error_reason() {
        let fake = FakeProvider::new([]);
        let params = RequestParams::new_for_test(vec![user_query("anything")], vec![]);
        let events = block_on(collect_events(fake.stream(params)));
        assert_eq!(events.len(), 1);
        match events[0].r#type.as_ref() {
            Some(response_event::Type::Finished(finished)) => {
                assert!(
                    matches!(
                        finished.reason,
                        Some(stream_finished::Reason::InternalError(_))
                    ),
                    "exhausted script should surface InternalError, got {:?}",
                    finished.reason
                );
            }
            other => panic!("expected Finished event, got {other:?}"),
        }
    }

    #[test]
    fn text_only_turn_emits_agent_output_and_done() {
        let fake = FakeProvider::new([ScriptedAssistantTurn::Text("hello".to_owned())]);
        let params = RequestParams::new_for_test(vec![user_query("hi")], vec![]);
        let events = block_on(collect_events(fake.stream(params)));
        assert_eq!(extract_agent_text(&events), "hello");
        assert!(events.iter().any(|e| {
            matches!(e.r#type, Some(response_event::Type::Init(_)))
        }));
        assert!(events.iter().any(|e| {
            matches!(e.r#type, Some(response_event::Type::Finished(_)))
        }));
    }

    /// M4: FakeProvider can script `start_agent` → StartAgentV2 Local (no network).
    /// Full parent→child controller loop remains IT-005 / GUI E2E.
    #[test]
    fn scripted_start_agent_maps_to_local_start_agent_v2() {
        use api::start_agent_v2::execution_mode::Mode;

        let fake = FakeProvider::new([ScriptedAssistantTurn::ToolCalls {
            text: Some("Spawning a worker.".to_owned()),
            tools: vec![ScriptedToolCall::start_agent(
                "sa_1",
                "worker",
                "research the bug",
            )],
        }]);

        let params =
            RequestParams::new_for_test(vec![user_query("use a child agent")], vec![]);
        let events = block_on(collect_events(fake.stream(params)));
        let tool_calls = extract_tool_calls(&events);
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].tool_call_id, "sa_1");

        let tool = tool_calls[0].tool.as_ref().expect("tool payload");
        let api::message::tool_call::Tool::StartAgentV2(sa) = tool else {
            panic!("expected StartAgentV2 from registry from_args, got {tool:?}");
        };
        assert_eq!(sa.name, "worker");
        assert_eq!(sa.prompt, "research the bug");
        let mode = sa
            .execution_mode
            .as_ref()
            .and_then(|m| m.mode.as_ref())
            .expect("execution_mode");
        assert!(
            matches!(mode, Mode::Local(_)),
            "scripted start_agent must stay local, got {mode:?}"
        );
    }
}
