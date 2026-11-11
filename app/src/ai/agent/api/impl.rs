use crate::ai::agent::redaction;

use super::{ConvertToAPITypeError, RequestParams, ResponseStream};

/// Generate multi-agent / agent-mode model output for the non-BYOP path.
///
/// BYOP models are dispatched earlier in `ResponseStream::new` via
/// `agent_providers::chat_stream::generate_byop_output`. This function only
/// rejects stale non-BYOP selections instead of falling back to another provider.
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

    let _ = cancellation_rx;
    Err(ConvertToAPITypeError::Other(anyhow::anyhow!(
        "Select a configured BYOP provider model in Settings before starting inference."
    )))
}
