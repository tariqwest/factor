# BYOP integration status (M3+ — OpenAI-compatible first)

> **2026-08-13 retarget:** Primary BYOP is **generic OpenAI-compatible** `base_url` (local `omlx`/`LiteLLM`/Apple FM/`omniroute` + remote `OpenRouter`/Vercel/`Portkey`/`Requesty`). Ollama is **secondary**; native dialects opportunistic. See PRODUCT §5/§9, TECH §3.4, plan 03.

## Progress (B/C/A order)

### B — Settings UI (landed)

- [x] `settings_view/agent_providers_widget.rs` (from Zap)
- [x] `SettingsSection::AgentProviders` under Agents umbrella ("Providers")
- [x] `AISubpage::Providers` + `build_page` wiring
- [x] Full provider/models.dev `AISettingsPageAction` surface + handlers
- [x] `ModelCapabilityKind` for model detail chips
- [x] Specialized model roles UI (Q-06): `specialized_byop_models_widget.rs` on Providers subpage
  - `agents.byop.title_model_id` / `active_ai_model_id` / `next_command_model_id`
  - Free-form model id strings (empty = follow base / agent picker)

### C — Local orchestration tool (landed)

- [x] `agent_providers/tools/start_agent.rs` → `Tool::StartAgentV2` (local mode only)
- [x] Registered in `tools::REGISTRY`
- [x] Gated by `RequestParams.orchestration_enabled`
- [x] Default cargo features include `orchestration` + `orchestration_v2`

### A — Compile-fix (lib tests green for registry + BYOP smoke)

- [x] Soft-skip Metal shader compile when `xcrun metal` missing (`crates/warpui/build.rs`)
- [x] Installed `protoc` via Homebrew
- [x] Registry matrix TDD: `factor_*` tests in `commands.rs` (13 pass)
- [x] `/model` BYOP picker: `ModelsByFeature::merge_byop_models` + `LLMPreferences::refresh_byop_models`
  - Subscribes to `AISettings` + `AgentProviderSecrets` for live refresh
  - `build_byop_llm_infos` → `byop:{provider}:{model}` ids in agent_mode choices
  - Smoke tests in `agent_providers/mod_test.rs`
- [x] Providers settings UI actions: `rebuild_current_page` / `save_agent_provider_edits`
- [x] Compaction settings fields on `AISettings`; `AIConversation.compaction_state`
- [x] Ported `http_client` proxy + `reqwest-eventsource` / `htmd` deps
- [ ] Full `cargo check -p warp --bin factor-oss` GUI binary (Metal tools still optional soft-skip)

## Verify

```bash
cd factor
export PATH="/opt/homebrew/bin:$PATH"  # protoc
cargo test -p warp --lib search::slash_command_menu::static_commands::commands::tests
cargo test -p warp --lib ai::agent_providers::tests
./script/check_factor_static_denylist all-runtime
```

Install full Xcode (or Metal tools) for a runnable macOS GUI binary.

## Known gaps (non-blocking)

- `factor_byop_manual_verify` matrix must be updated to OpenAI-compat local+remote pair (Ollama secondary) — see plan 03 phase 9
- No per-terminal reasoning-effort persistence for BYOP (capability defaults only)
- Drive attachments stubbed; cloud telemetry no-op
- Voice transcribe is local-error-only

## 2026-08-12 autonomous follow-ups

- [x] Active model priority: terminal → last-used → profile → OpenRouter legacy → default
- [x] Persist `byop_last_used_model_id` on picker change; hydrate on new tabs
- [x] `LocalDockerSandbox` Warper approach (dogfood flag + slash when on)
- [x] `start_agent` local-only unit tests
- [x] M4 unit layer for local start_agent parent→child (no full GUI):
  - `tools/start_agent.rs`: REGISTRY, Local execution_mode, `available_tool_names` orchestration gate
  - `execute/start_agent_tests.rs`: CreateAgent Local emit + child run_id → Success
  - `api/fake_provider.rs`: scripted `start_agent` → `StartAgentV2` Local
  - **Still open for IT-005 E2E:** pane_group child pane, enter_agent_view, child inference turn, parent transcript tool result via real UI
- [x] Lightweight `.github/workflows/factor-gates.yml`
- [x] FakeProvider multi-turn tool loop (`api/fake_provider.rs` + `multi_turn_shell_then_final`)
- [x] Q-06 specialized model UI: Settings → Agents → Providers → **Specialized models**
  - Fields: `agents.byop.title_model_id`, `active_ai_model_id`, `next_command_model_id`
  - Widget: `settings_view/specialized_byop_models_widget.rs` (blur/enter saves; empty = base)
  - Wired via `LLMPreferences::get_active_{title,ai,next_command}_model`
- See also [OPEN_FOR_REVIEW.md](../OPEN_FOR_REVIEW.md)
