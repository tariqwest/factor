use crate::ai::agent::redaction;
use crate::ai::agent_providers::{self, llm_id};
use futures_util::StreamExt;

use super::{openrouter, ConvertToAPITypeError, RequestParams, ResponseStream};

/// Generate multi-agent / agent-mode model output.
///
/// Routing:
/// 1. BYOP model ids (`byop:provider:model`) → Zap-derived agent_providers chat_stream
/// 2. Otherwise → interim OpenRouter backend (Warper path; removable after BYOP default UX lands)
pub async fn generate_multi_agent_output(
    mut params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    if params.should_redact_secrets {
        redaction::redact_inputs(&mut params.input);
    }

    if llm_id::is_byop(&params.model) {
        let output_stream =
            agent_providers::chat_stream::generate_byop_output(params).take_until(cancellation_rx);
        return Ok(Box::pin(output_stream));
    }

    let output_stream = openrouter::generate_openrouter_output(params).take_until(cancellation_rx);
    Ok(Box::pin(output_stream))
}
