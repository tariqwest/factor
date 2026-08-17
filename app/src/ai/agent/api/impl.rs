use crate::ai::agent::redaction;
use futures_util::StreamExt;

use super::{openrouter, ConvertToAPITypeError, RequestParams, ResponseStream};

/// Generate multi-agent / agent-mode model output for the non-BYOP path.
///
/// BYOP models are dispatched earlier in `ResponseStream::new` via
/// `agent_providers::chat_stream::generate_byop_output`. This function only
/// serves the interim OpenRouter / cloud-free Factor fallback.
pub async fn generate_multi_agent_output(
    mut params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    if params.should_redact_secrets {
        redaction::redact_inputs(&mut params.input);
    }

    #[cfg(any(test, feature = "test-util"))]
    if let Some(fake) = params.fake_provider.clone() {
        let _ = cancellation_rx;
        return Ok(fake.stream(params));
    }

    let output_stream = openrouter::generate_openrouter_output(params).take_until(cancellation_rx);
    Ok(Box::pin(output_stream))
}
