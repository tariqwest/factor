//! Local BYOP runner for `factor-oss agent run`.
//!
//! `--runner local` bypasses the third-party CLI harnesses and drives a single
//! BYOP chat completion directly, streaming the response to stdout.

use crate::ai::agent_providers::lookup_byop;
use crate::ai::agent_sdk::local_controller;
use anyhow::Context as _;
use serde_json::json;
use warp_cli::agent::{OutputFormat, RunAgentArgs};
use warp_cli::GlobalOptions;
use warpui::{platform::TerminationMode, AppContext};

use super::{report_fatal_error, resolve_prompt};

/// Run a local BYOP agent from the CLI.
pub fn run(
    ctx: &mut AppContext,
    args: RunAgentArgs,
    global_options: GlobalOptions,
) -> anyhow::Result<()> {
    let runner = ctx.add_singleton_model(|_ctx| LocalAgentRunner);
    runner.update(ctx, move |_, ctx| {
        let output_format = global_options.output_format;
        let prompt = match resolve_user_prompt(&args, ctx) {
            Ok(prompt) => prompt,
            Err(err) => {
                report_fatal_error(err, ctx);
                return;
            }
        };

        let Some(model_id) = args.model.model.clone() else {
            report_fatal_error(
                anyhow::anyhow!("--model is required when using --runner local"),
                ctx,
            );
            return;
        };

        let api_key = global_options.api_key;
        let spawner = ctx.spawner();
        ctx.spawn(
            LocalAgentRunner::chat(spawner, model_id, api_key, prompt, output_format, args.cwd.clone()),
            |_, result, ctx| match result {
                Ok(()) => ctx.terminate_app(TerminationMode::ForceTerminate, None),
                Err(err) => report_fatal_error(err, ctx),
            },
        );
    });
    Ok(())
}

fn resolve_user_prompt(args: &RunAgentArgs, ctx: &AppContext) -> anyhow::Result<String> {
    let prompt = args
        .prompt_arg
        .to_prompt()
        .ok_or_else(|| anyhow::anyhow!("A prompt is required. Use --prompt"))?;
    resolve_prompt(&prompt, ctx).map_err(|e| anyhow::anyhow!(e))
}

struct LocalAgentRunner;

impl warpui::Entity for LocalAgentRunner {
    type Event = ();
}

impl warpui::SingletonEntity for LocalAgentRunner {}

impl LocalAgentRunner {
    async fn chat(
        foreground: warpui::ModelSpawner<Self>,
        raw_model_id: String,
        api_key_override: Option<String>,
        prompt: String,
        output_format: OutputFormat,
        cwd: Option<std::path::PathBuf>,
    ) -> anyhow::Result<()> {
        let (provider, api_key, model_id) = foreground
            .spawn(move |_, ctx| -> anyhow::Result<_> {
                let valid_id = super::common::validate_agent_mode_base_model_id(&raw_model_id, ctx)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let (provider, key, model) =
                    lookup_byop(ctx, &valid_id).ok_or_else(|| {
                        anyhow::anyhow!("Model '{raw_model_id}' is not a BYOP provider")
                    })?;
                Ok((provider, key, model))
            })
            .await??;

        let api_key = api_key_override.unwrap_or(api_key);

        let output_json = matches!(output_format, OutputFormat::Json);
        let cwd_ref = cwd.as_deref();
        let reply = local_controller::run_byop_local_agent(
            &provider,
            api_key,
            model_id,
            prompt,
            cwd_ref,
            output_json,
        )
        .await
        .with_context(|| "local BYOP multi-turn chat failed")?;

        match output_format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&json!({"response": reply }))?);
            }
            _ => println!("{reply}"),
        }

        Ok(())
    }
}
