# BYOP integration status (M3+)

## Progress (B/C/A order)

### B — Settings UI (landed)

- [x] `settings_view/agent_providers_widget.rs` (from Zap)
- [x] `SettingsSection::AgentProviders` under Agents umbrella ("Providers")
- [x] `AISubpage::Providers` + `build_page` wiring
- [x] Full provider/models.dev `AISettingsPageAction` surface + handlers
- [x] `ModelCapabilityKind` for model detail chips

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

- No per-terminal reasoning-effort persistence for BYOP (capability defaults only)
- Title / active_ai / next_command models fall back to base
- Drive attachments stubbed; cloud telemetry no-op
- Voice transcribe is local-error-only

## 2026-08-12 autonomous follow-ups

- [x] Active model priority: terminal → last-used → profile → OpenRouter legacy → default
- [x] Persist `byop_last_used_model_id` on picker change; hydrate on new tabs
- [x] `LocalDockerSandbox` hard-disabled (test override only)
- [x] `start_agent` local-only unit tests
- [x] Lightweight `.github/workflows/factor-gates.yml`
- [ ] FakeProvider multi-turn tool loop
- See also [OPEN_FOR_REVIEW.md](../OPEN_FOR_REVIEW.md)
