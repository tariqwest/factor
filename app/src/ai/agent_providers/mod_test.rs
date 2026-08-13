//! Smoke tests for BYOP provider configuration and /model picker merge.

use ai::LLMId;
use settings::Setting;
use warp_core::{
    channel::{Channel, ChannelConfig, ChannelState},
    AppId,
};
use warpui::{App, SingletonEntity};

use crate::ai::agent_providers::{llm_id, lookup_byop, AgentProviderSecrets};
use crate::ai::llms::LLMPreferences;
use crate::network::NetworkStatus;
use crate::settings::{AISettings, AgentProvider, AgentProviderApiType, AgentProviderModel};
use crate::test_util::settings::initialize_settings_for_tests;
use crate::workspaces::user_workspaces::UserWorkspaces;

/// Factor BYOP picker only merges when `Channel::Oss` (see `LLMPreferences`).
struct OssChannelGuard;

impl OssChannelGuard {
    fn enter() -> Self {
        ChannelState::set(ChannelState::new(
            Channel::Oss,
            ChannelConfig {
                app_id: AppId::new("dev", "factor", "Factor"),
                logfile_name: "factor.log".into(),
                mcp_static_config: None,
            },
        ));
        Self
    }
}

impl Drop for OssChannelGuard {
    fn drop(&mut self) {
        ChannelState::set(ChannelState::init());
    }
}

fn sample_provider(id: &str) -> AgentProvider {
    AgentProvider {
        id: id.to_owned(),
        name: "Test Ollama".to_owned(),
        kind: Default::default(),
        api_type: AgentProviderApiType::Ollama,
        base_url: "http://localhost:11434".to_owned(),
        models: vec![AgentProviderModel::from_id("llama3.2".to_owned())],
        extra_headers: Vec::new(),
    }
}

fn init_byop_test_app(app: &mut warpui::App) {
    initialize_settings_for_tests(app);
    // AgentProviderSecrets already registered via initialize_settings_for_tests (BYOP)
    app.add_singleton_model(|_| NetworkStatus::new());
    // Factor: no cloud AuthManager / AuthStateProvider — local BYOP only.
    app.add_singleton_model(UserWorkspaces::default_mock);
    app.add_singleton_model(LLMPreferences::new);
}

#[test]
fn smoke_merge_byop_exposes_configured_models_in_agent_picker() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        let provider_id = "provider-smoke-1";
        let expected_id = llm_id::encode(provider_id, "llama3.2");
        app.update(|ctx| {
            AISettings::handle(ctx).update(ctx, |settings, ctx| {
                let _ = settings
                    .agent_providers
                    .set_value(vec![sample_provider(provider_id)], ctx);
            });
        });

        app.read(|ctx| {
            let choices: Vec<_> = LLMPreferences::as_ref(ctx)
                .get_base_llm_choices_for_agent_mode()
                .collect();
            let byop: Vec<_> = choices
                .iter()
                .filter(|c| llm_id::is_byop(&c.id))
                .collect();
            assert_eq!(byop.len(), 1, "expected one BYOP model merged into picker");
            assert!(
                byop[0].disable_reason.is_none(),
                "valid provider should not be disabled"
            );
            assert_eq!(byop[0].id.as_str(), expected_id.as_str());
            // Factor keeps base OSS/OpenRouter choices alongside BYOP.
            assert!(
                choices.len() > 1,
                "BYOP should merge into existing base choices, not replace them"
            );
        });
    });
}

#[test]
fn smoke_merge_byop_has_no_byop_ids_when_unconfigured() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        app.read(|ctx| {
            let byop_count = LLMPreferences::as_ref(ctx)
                .get_base_llm_choices_for_agent_mode()
                .filter(|c| llm_id::is_byop(&c.id) || c.id.as_str() == "byop-placeholder")
                .count();
            assert_eq!(
                byop_count, 0,
                "empty providers list should not inject BYOP picker entries"
            );
        });
    });
}

#[test]
fn smoke_merge_byop_skips_empty_base_url() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        app.update(|ctx| {
            AISettings::handle(ctx).update(ctx, |settings, ctx| {
                let mut broken = sample_provider("broken");
                broken.base_url.clear();
                let _ = settings.agent_providers.set_value(vec![broken], ctx);
            });
        });

        app.read(|ctx| {
            let byop_count = LLMPreferences::as_ref(ctx)
                .get_base_llm_choices_for_agent_mode()
                .filter(|c| llm_id::is_byop(&c.id))
                .count();
            assert_eq!(
                byop_count, 0,
                "provider with empty base_url must not appear in /model picker"
            );
        });
    });
}

#[test]
fn smoke_lookup_byop_resolves_provider_and_model_without_api_key() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        let provider_id = "provider-lookup-1";
        app.update(|ctx| {
            AISettings::handle(ctx).update(ctx, |settings, ctx| {
                let _ = settings
                    .agent_providers
                    .set_value(vec![sample_provider(provider_id)], ctx);
            });
        });

        let encoded = llm_id::encode(provider_id, "llama3.2");
        app.read(|ctx| {
            let (provider, api_key, model_id) =
                lookup_byop(ctx, &encoded).expect("lookup_byop should resolve configured model");
            assert_eq!(provider.id, provider_id);
            assert_eq!(model_id, "llama3.2");
            assert!(api_key.is_empty(), "Ollama path allows empty API key");
        });
    });
}

#[test]
fn smoke_lookup_byop_returns_none_for_unknown_id() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        app.read(|ctx| {
            assert!(lookup_byop(ctx, &LLMId::from("byop:missing:model")).is_none());
            assert!(lookup_byop(ctx, &LLMId::from("not-byop")).is_none());
        });
    });
}

#[test]
fn smoke_last_used_byop_model_hydrates_active_base_model() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        let provider_id = "provider-last-used";
        let expected = llm_id::encode(provider_id, "llama3.2");
        app.update(|ctx| {
            AISettings::handle(ctx).update(ctx, |settings, ctx| {
                let _ = settings
                    .agent_providers
                    .set_value(vec![sample_provider(provider_id)], ctx);
                let _ = settings
                    .byop_last_used_model_id
                    .set_value(Some(expected.as_str().to_owned()), ctx);
            });
        });

        app.read(|ctx| {
            let active = LLMPreferences::as_ref(ctx).get_active_base_model(ctx, None);
            assert_eq!(
                active.id.as_str(),
                expected.as_str(),
                "last-used BYOP id should win over profile/default for agent base model"
            );
        });
    });
}

#[test]
fn smoke_specialized_byop_model_settings_override_title_active_next_command() {
    let _channel = OssChannelGuard::enter();
    App::test((), |mut app| async move {
        init_byop_test_app(&mut app);

        let provider_id = "provider-specialized";
        let mut provider = sample_provider(provider_id);
        provider.models.push(AgentProviderModel::from_id("title-model".to_owned()));
        provider
            .models
            .push(AgentProviderModel::from_id("active-ai-model".to_owned()));
        provider
            .models
            .push(AgentProviderModel::from_id("next-cmd-model".to_owned()));

        let title_id = llm_id::encode(provider_id, "title-model");
        let active_id = llm_id::encode(provider_id, "active-ai-model");
        let next_id = llm_id::encode(provider_id, "next-cmd-model");

        app.update(|ctx| {
            AISettings::handle(ctx).update(ctx, |settings, ctx| {
                let _ = settings.agent_providers.set_value(vec![provider], ctx);
                let _ = settings
                    .byop_title_model_id
                    .set_value(Some(title_id.as_str().to_owned()), ctx);
                let _ = settings
                    .byop_active_ai_model_id
                    .set_value(Some(active_id.as_str().to_owned()), ctx);
                let _ = settings
                    .byop_next_command_model_id
                    .set_value(Some(next_id.as_str().to_owned()), ctx);
            });
        });

        app.read(|ctx| {
            let prefs = LLMPreferences::as_ref(ctx);
            assert_eq!(
                prefs.get_active_title_model(ctx, None).id.as_str(),
                title_id.as_str()
            );
            assert_eq!(
                prefs.get_active_ai_model(ctx, None).id.as_str(),
                active_id.as_str()
            );
            assert_eq!(
                prefs.get_active_next_command_model(ctx, None).id.as_str(),
                next_id.as_str()
            );
        });
    });
}
