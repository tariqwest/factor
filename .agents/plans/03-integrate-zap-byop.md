# 03 — Integrate Zap BYOP into Factor

## Goal

Replace Warper’s OpenRouter-only inference core with Zap’s multi-provider BYOP stack so Factor’s local agent loop talks to user-chosen providers **directly**, with keys on-device.

## Source of truth

| Zap path | Purpose |
|----------|---------|
| `app/src/ai/agent_providers/` | Provider registry, chat_stream, prompts, tools, secrets |
| `app/src/ai/byop_readiness/` | Multi-turn tool readiness / repair |
| `app/src/ai/byop_compaction/` | Context compaction sidecar |
| `app/src/settings/` AgentProvider* | Settings schema |
| `app/src/settings_view/ai_page.rs` | UI (port carefully; rebrand) |
| `app/src/ai/agent/api.rs` | RequestParams BYOP fields |
| `app/src/ai/blocklist/controller.rs` | Preflight / flush pending BYOP |

Reference only (optional ideas): warp-refined `local_openai/` for Responses API fidelity — not primary.

## Target seam

Warper/Factor today:

```text
blocklist controller
  → generate_multi_agent_output (api/impl.rs)
      → openrouter::generate_openrouter_output
```

Factor after:

```text
blocklist controller
  → generate_multi_agent_output
      → resolve model id
          → byop:*  → agent_providers::chat_stream
          → legacy openrouter (optional, D-10)
```

## Phased integration

### Phase 0 — Inventory & dependency graph

1. List Zap files under `agent_providers`, `byop_*`, related settings types.  
2. Map crate deps Zap uses that Warper may lack (`genai` crate?, minijinja templates, etc.).  
3. Produce `docs/tech/BYOP_FILE_MANIFEST.md` with copy list.

### Phase 1 — Settings & secrets (TDD first)

**Tests first:**

- Provider struct serde roundtrip in settings.toml  
- Keychain save/load with Factor service name  
- `build_byop_llm_infos` excludes empty base_url  

**Implement:**

- Port `AgentProvider`, `AgentProviderApiType`, `AgentProviderModel` settings  
- Port `AgentProviderSecrets` with `dev.factor.Factor` keychain  
- Settings UI section “Providers” (English)

### Phase 2 — Tool adapters

Port `agent_providers/tools/*` mapping OpenAI-style tools → `warp_multi_agent_api` tool enums.

**Must include for Factor:**

- run_shell_command (+ LRC helpers)  
- read_files, grep, file_glob, apply_file_diffs  
- read_skill, ask_user_question  
- MCP call/read if MCP retained  
- **start_agent** (add if missing in Zap — required for local orchestration)

**Tests:** each adapter pure function test with sample JSON args.

### Phase 3 — chat_stream + prompt_renderer

Port streaming loop and minijinja/system prompts.

**Tests:**

- Fake HTTP or FakeBackend returns tool_call then final text  
- Template selection by api_type (Ollama short template etc.)

### Phase 4 — Controller readiness

Port `byop_readiness` preflight + pending request flush from Zap controller.

**Tests:**

- Pending tool results blocks request; flush after tools complete  
- Repair state sidecar serde on conversation restore  

### Phase 5 — Wire impl.rs

```rust
pub async fn generate_multi_agent_output(params, cancel) {
  redact if needed;
  if let Some(byop) = lookup_byop(params.model) {
    return byop_chat_stream(params, byop, cancel);
  }
  if cfg!(feature = "legacy_openrouter_backend") {
    return openrouter(...);
  }
  Err(NoProviderConfigured)
}
```

### Phase 6 — Delete or quarantine OpenRouter special-case

- OpenRouter remains a **user-configured** OpenAI-compatible or dedicated provider  
- Remove hard-coded `OPENROUTER_CHAT_COMPLETIONS_URL` as sole path  
- Keep env `OPENROUTER_API_KEY` as convenience import into provider list (optional)

### Phase 7 — UX

- Model picker shows BYOP models  
- Empty state: “Add a provider in Settings”  
- Agent mode works without Warp login (already true on Warper; verify after port)

## File-level copy order (suggested)

1. `settings` types for providers  
2. `agent_providers/secrets.rs`, `openai_compatible.rs`, `llm_id.rs`, `mod.rs`  
3. `tools/*`  
4. `prompt_renderer.rs` + `prompts/**`  
5. `reasoning.rs`, `user_context.rs`, `attachment_caps.rs`  
6. `chat_stream.rs` (large — land with tests)  
7. `byop_readiness`, `byop_compaction`  
8. `agent/api.rs` field extensions  
9. `blocklist/controller.rs` integration hunks  
10. `settings_view` UI  

Use `git show zap/main:path > path` style ports; fix compile iteratively.

## Dependencies to expect

From Zap analysis (validate at port time):

- HTTP client already in tree  
- Possibly `genai` for multi-provider chat abstractions  
- Template engine for system prompts  
- Serde types for each api_type  

Run `cargo check` after each module group.

## TDD checklist (exit)

- [ ] Unit: llm_id encode/decode  
- [ ] Unit: tool arg parse for shell/read/grep/start_agent  
- [ ] Unit: secrets roundtrip (mock secure storage if needed)  
- [ ] Integration: FakeProvider two-step tool loop  
- [ ] Manual: OpenAI-compat OR Anthropic real key  
- [ ] Manual: Ollama local no key  
- [ ] Denylist still green (no zap cloud leftovers)  

## Risks & mitigations

| Risk | Mitigation |
|------|------------|
| chat_stream.rs is huge (7k+ LOC) | Port with FakeProvider; don’t rewrite |
| Zap Chinese UI strings | Replace with English in Factor UI |
| Zap still has cloud_object code nearby | Don’t port Drive; only agent_providers |
| Missing start_agent in tools | Implement adapter → existing executor |
| Compaction incomplete | Ship without compaction; add P1 |
| Prompt cache / provider quirks | Track as provider-specific issues |

## Non-goals for this plan

- Zap SSH host manager / SFTP / gist sync  
- Zap Fluent i18n  
- warp-refined Responses-only path (unless BYOP OpenAI-compat insufficient)

## Commits (conventional)

```text
feat(byop): add provider settings and secrets
feat(byop): port tool adapters and prompt renderer
feat(byop): port chat_stream inference backend
feat(byop): wire generate_multi_agent_output to byop
feat(byop): controller readiness and conversation sidecars
test(byop): fake provider multi-turn tool loop
feat(byop): settings UI for providers
chore(byop): demote openrouter to optional legacy backend
```

## Success criteria

1. User can configure ≥2 provider types and complete an agent task with tools.  
2. No Warp server calls required for agent turn.  
3. Local orchestration still compiles; start_agent tool callable when enabled.  
4. FACTOR-003 PRODUCT behaviors pass.
