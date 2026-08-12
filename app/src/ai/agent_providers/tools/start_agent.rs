//! `start_agent` adapter for local multi-agent orchestration (Factor).
//!
//! Maps to `Tool::StartAgentV2` with **local** execution mode by default so BYOP
//! parent agents can spawn local child agents without cloud Oz / remote runners.

use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use warp_multi_agent_api as api;

use super::OpenAiTool;

#[derive(Debug, Deserialize)]
struct Args {
    /// Display name for the child agent (e.g. "research", "Agent 1").
    name: String,
    /// Prompt / task for the child agent.
    prompt: String,
    /// Optional local harness type (e.g. "claude-code", "codex"). Empty = default Factor agent.
    #[serde(default)]
    harness_type: Option<String>,
}

fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Short display name for the child agent."
            },
            "prompt": {
                "type": "string",
                "description": "Full task description / prompt for the child agent to execute."
            },
            "harness_type": {
                "type": "string",
                "description": "Optional local child harness (e.g. claude-code, codex). Omit for the default Factor agent."
            }
        },
        "required": ["name", "prompt"],
        "additionalProperties": false
    })
}

fn from_args(args: &str) -> Result<api::message::tool_call::Tool> {
    let parsed: Args = serde_json::from_str(args)?;
    let harness = parsed
        .harness_type
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .map(|t| api::start_agent_v2::execution_mode::Harness { r#type: t });

    let execution_mode = Some(api::start_agent_v2::ExecutionMode {
        mode: Some(api::start_agent_v2::execution_mode::Mode::Local(
            api::start_agent_v2::execution_mode::Local { harness },
        )),
    });

    Ok(api::message::tool_call::Tool::StartAgentV2(
        api::StartAgentV2 {
            name: parsed.name,
            prompt: parsed.prompt,
            execution_mode,
            lifecycle_subscription: None,
        },
    ))
}

fn result_to_json(result: &api::message::tool_call_result::Result) -> Option<Value> {
    use api::message::tool_call_result::Result as R;
    use api::start_agent_v2_result::Result as SAR;

    let r = match result {
        R::StartAgentV2(r) => r,
        R::StartAgent(r) => {
            // V1 result path
            use api::start_agent_result::Result as SAR1;
            return match &r.result {
                Some(SAR1::Success(s)) => Some(json!({
                    "status": "started",
                    "agent_id": s.agent_id,
                })),
                Some(SAR1::Error(e)) => Some(json!({
                    "status": "error",
                    "error": e.error,
                })),
                None => Some(json!({ "status": "cancelled" })),
            };
        }
        _ => return None,
    };

    Some(match &r.result {
        Some(SAR::Success(s)) => json!({
            "status": "started",
            "agent_id": s.agent_id,
        }),
        Some(SAR::Error(e)) => json!({
            "status": "error",
            "error": e.error,
        }),
        None => json!({ "status": "cancelled" }),
    })
}

pub static START_AGENT: OpenAiTool = OpenAiTool {
    name: "start_agent",
    description: "Start a local child agent for a subtask (local multi-agent orchestration). \
The child runs on-device via Factor's local agent stack — not a cloud runner. \
Use when a subtask can run in parallel or isolation from the parent conversation.",
    parameters,
    from_args,
    result_to_json,
};

#[cfg(test)]
mod tests {
    use super::*;
    use api::start_agent_v2::execution_mode::Mode;

    #[test]
    fn start_agent_always_maps_to_local_execution_mode() {
        let tool = from_args(r#"{"name":"worker","prompt":"do the thing"}"#).unwrap();
        let api::message::tool_call::Tool::StartAgentV2(sa) = tool else {
            panic!("expected StartAgentV2");
        };
        assert_eq!(sa.name, "worker");
        assert_eq!(sa.prompt, "do the thing");
        let mode = sa
            .execution_mode
            .as_ref()
            .and_then(|m| m.mode.as_ref())
            .expect("execution_mode");
        assert!(
            matches!(mode, Mode::Local(_)),
            "Factor start_agent must never emit cloud/remote execution modes"
        );
    }

    #[test]
    fn start_agent_optional_harness_stays_local() {
        let tool =
            from_args(r#"{"name":"cc","prompt":"x","harness_type":"claude-code"}"#).unwrap();
        let api::message::tool_call::Tool::StartAgentV2(sa) = tool else {
            panic!("expected StartAgentV2");
        };
        match sa
            .execution_mode
            .as_ref()
            .and_then(|m| m.mode.as_ref())
            .unwrap()
        {
            Mode::Local(local) => {
                assert_eq!(
                    local.harness.as_ref().map(|h| h.r#type.as_str()),
                    Some("claude-code")
                );
            }
            other => panic!("expected Local mode, got {other:?}"),
        }
    }
}
