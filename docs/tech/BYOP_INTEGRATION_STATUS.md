# BYOP integration status (M3)

## Landed

- [x] Copied `app/src/ai/agent_providers/**` from Zap
- [x] Copied `byop_readiness/**`, `byop_compaction/**`
- [x] Vendored `lib/rust-genai` + workspace `genai` patch
- [x] `minijinja` / `genai` workspace deps
- [x] `settings/byop_types.rs` + `agents.warp_agent.providers` setting
- [x] `api_error` with BYOP readiness awareness
- [x] `RequestParams` BYOP fields
- [x] Dual-path routing:
  - BYOP models → `chat_stream::generate_byop_output`
  - else → OpenRouter (`generate_multi_agent_output` / openrouter.rs)
- [x] `blocklist/controller/response_stream.rs` from Zap with OpenRouter fallback

## Not yet complete (expected compile blockers)

- [ ] Full `cargo check -p warp` (disk was ~2.5Gi free at attempt time; toolchain download also heavy)
- [ ] Settings UI for provider CRUD (Zap `settings_view/ai_page` sections) — **settings data model only**
- [ ] Wire `AgentProviderSecrets` into app init (`lib.rs` singleton registration)
- [ ] Controller preflight/readiness hooks from Zap `blocklist/controller.rs` (beyond response_stream)
- [ ] LLM picker synthesis via `build_byop_llm_infos` in active model lists
- [ ] `start_agent` tool in BYOP tool registry
- [ ] Unit tests for `llm_id` / FakeProvider in CI
- [ ] Replace Chinese comments/UI strings remaining in ported modules

## How to finish compile locally

```bash
# free disk first (need multi-GB for target/)
cd factor
./script/bootstrap
cargo check -p warp --bin factor-oss
./script/check_factor_static_denylist all-runtime
```

## Interim UX

Until picker UI lands, BYOP models can be selected only if LLMId is set to `byop:<provider_id>:<model_id>` via settings/debug; OpenRouter path remains default for Warper-style keys.
