# Factor — Technical Spec

**Status:** Draft v0.1 (planning)  
**Depends on:** [PRODUCT.md](../product/PRODUCT.md)  
**Last updated:** 2026-08-12

## 1. Construction strategy

Factor is built in **three phases**. We do **not** copy an entire stale monorepo into `main` as the long-term history without reconciliation.

```text
Phase A  Spec + scaffold (this repo today)
Phase B  Import monorepo base: warper tree → factor branding
Phase C  Reconcile base onto latest upstream warp (or re-apply strip to upstream)
Phase D  Integrate Zap BYOP + enable local orchestration
Phase E  Harden TDD gates, packaging, releases
```

**Judgment (D-01):** Prefer **warper as amputation baseline**, then **port/rebase onto latest upstream**, rather than stripping latest upstream from scratch — Warper already paid the deletion tax. If rebase cost explodes, fall back to **strip-latest-upstream** using Warper denylist + specs as the checklist.

Detailed plans:

- [`.agents/plans/01-bootstrap-and-import.md`](../../.agents/plans/01-bootstrap-and-import.md)
- [`.agents/plans/02-reconcile-warper-upstream.md`](../../.agents/plans/02-reconcile-warper-upstream.md)
- [`.agents/plans/03-integrate-zap-byop.md`](../../.agents/plans/03-integrate-zap-byop.md)
- [`.agents/plans/04-tdd-and-quality-gates.md`](../../.agents/plans/04-tdd-and-quality-gates.md)

## 2. High-level architecture

```text
┌──────────────────────────────────────────────────────────────────┐
│ app (factor binary)                                                │
│  bin/factor.rs / factor_oss.rs                                   │
│  Channel::Oss + AppId(dev.factor.Factor)                          │
├──────────────────────────────────────────────────────────────────┤
│ Terminal substrate                                                │
│  panes · blocks · session · warpify · ssh · bootstrap hooks       │
│  cli_agent_sessions · suggestions · keybindings · settings        │
├──────────────────────────────────────────────────────────────────┤
│ Local agent runtime                                               │
│  blocklist controller · conversation/task store (local SQLite)    │
│  action executors (shell, fs, skills, mcp, start_agent, …)        │
│  orchestration (local parent/child only)                          │
├──────────────────────────────────────────────────────────────────┤
│ Inference plane (BYOP)                                            │
│  agent_providers: chat_stream · prompt_renderer · tools · secrets │
│  **Primary: OpenAI-compatible** (any `base_url` — omlx/LiteLLM/    │
│   Apple FM/omniroute local; OpenRouter/Vercel/Portkey/Requesty    │
│   remote). Native dialects + Ollama are secondary.                │
├──────────────────────────────────────────────────────────────────┤
│ Explicitly ABSENT                                                 │
│  auth product · billing · drive · teams · shared sessions         │
│  cloud_mode · cloud oz handoff · hosted telemetry/update          │
└──────────────────────────────────────────────────────────────────┘
```

## 3. Module map (post-import)

Paths are **target** locations after monorepo import (Warp-style layout).

### 3.1 Entry / channel

| Path | Role |
|------|------|
| `app/src/bin/factor.rs` (or `oss.rs`) | OSS channel bootstrap |
| `crates/warp_core/src/channel/` | Channel config; Factor nulls telemetry/crash/autoupdate or points to self-host |
| `crates/warp_features` | Feature flags; `FACTOR_FLAGS` / `OSS_FLAGS` for local agent + orchestration |

### 3.2 Terminal (keep, brand)

| Path | Role |
|------|------|
| `app/src/terminal/` | View, model, blocks, session |
| `app/src/terminal/warpify/` | Shell integration |
| `app/src/terminal/ssh/` | SSH + warpify remote |
| `app/assets/bundled/bootstrap/` | bash/zsh/fish/pwsh preexec hooks |
| `app/src/terminal/cli_agent_sessions/` | 3p CLI agents |
| `app/src/suggestions/`, `app/src/ai/predict/` | Inline / next-command |

### 3.3 Agent runtime (keep local, wire BYOP)

| Path | Role |
|------|------|
| `app/src/ai/blocklist/` | Controller, history, block UI |
| `app/src/ai/blocklist/action_model/execute/` | Tool execution |
| `app/src/ai/agent/` | Conversation, tasks, API types |
| `app/src/ai/agent/api/` | **Seam:** `generate_multi_agent_output` |
| `app/src/ai/agent_providers/` | **From Zap** — BYOP stack |
| `app/src/ai/byop_readiness/`, `byop_compaction/` | **From Zap** |
| `app/src/ai/blocklist/action_model/execute/start_agent.rs` | Local orchestration |
| `app/src/ai/blocklist/action_model/execute/run_agents.rs` | Port from upstream if local-only |

### 3.4 Provider seam (critical)

Warper today:

```rust
// warper/app/src/ai/agent/api/impl.rs
pub async fn generate_multi_agent_output(...) {
    openrouter::generate_openrouter_output(params)...
}
```

Factor target:

```text
generate_multi_agent_output(params):
  match resolve_backend(params.model):
    Byop(provider) => agent_providers::chat_stream::run(params, provider)
    // Legacy/test only:
    // OpenRouterThin => openrouter::...  (optional during migration)
```

**Primary BYOP target is generic OpenAI-compatible** — one code path serves local (`omlx`, `LiteLLM`, Apple FM serve, `omniroute`) and remote gateways (`OpenRouter`, Vercel AI Gateway, `Portkey`, `Requesty`) via `base_url` + key + model. Do **not** build a separate Ollama-shaped core; Ollama is secondary (expose via same OpenAI-compat `base_url` where possible, or via `api_type: Ollama` only as fallback).

Model IDs:

- Prefer Zap encoding: `byop:{provider_id}:{model_id}` where `provider_id` is user-chosen; `base_url` decides local vs remote
- Picker synthesizes `LLMInfo` from settings providers (Zap `build_byop_llm_infos`) — filter empty `base_url`, not by brand
- No Warp server model catalog required

### 3.5 Strip targets (must not ship)

| Path / concept | Action |
|----------------|--------|
| Auth UI product surfaces | Delete or `cfg` out; denylist |
| `ServerApiProvider` startup graph for cloud | Remove from init |
| Drive / cloud_object crates & UI | Absent (as warper) |
| `cloud_mode*`, oz handoff, cloud ambient | Absent |
| Firebase / rudder / sentry init | Absent |
| GraphQL cloud billing | Absent |

## 4. Data & identity isolation

| Concern | Factor policy |
|---------|----------------|
| App support dir | ProjectDirs for `dev.factor.Factor` — never Warp or Warper containers |
| Settings | `~/.factor/settings.toml` (macOS also Application Support) |
| SQLite / cache | Under Factor project dirs only |
| Secrets | OS keychain service `dev.factor.Factor`, accounts namespaced per provider |
| Migration | Optional one-shot importer from warper/zap/warp paths — **opt-in UI**, never silent cross-app keychain read at launch (Warper WARPER-002 lesson) |

## 5. Local orchestration design

### 5.1 In scope

- `FeatureFlag::Orchestration` (and local-safe V2 event plumbing if it does not require Warp servers)
- `StartAgentExecutor` creating **local** child conversations
- Parent/child block UI under `block/view_impl/orchestration*`
- Permissions: execution profile `run_agents` / start_agent

### 5.2 Out of scope

- Cloud task IDs, ambient cloud RTC, capacity modals
- `oz_launch_modal`, `handoff_local_cloud`, `cloud_agent_runners`
- Any spawn path that requires logged-in Warp user

### 5.3 BYOP + child agents

Child agents must use the same BYOP router as parent.  
**Risk:** Zap may not implement `start_agent` tool in `agent_providers/tools`.  
**Plan:** Add `start_agent` (and later `run_agents`) tool schemas to BYOP tool registry mapping onto existing executors.

## 6. Warpify + SSH + agent tools

### 6.1 Hooks

“Hooks” = shell bootstrap preexec/precmd (and related DCS hooks in ansi handler), not a separate product module.

| Component | Responsibility |
|-----------|----------------|
| `app/assets/bundled/bootstrap/*` | Injected shell functions |
| `ansi_handler` preexec/precmd | Block lifecycle |
| `warpify/` | Install/enable integration |

### 6.2 Agent tool execution context

When session is remote-warpified:

- `run_shell_command` must target remote PTY
- LRC (long-running command) monitoring must bind remote command ids (Zap LRC work is reference)
- File tools: **clarify** local-vs-remote path semantics — see open question D-08

### 6.3 Tests

Integration fixtures: local warpify + mock/recorded SSH where possible; manual matrix documented for real SSH.

## 7. Feature flags & cargo features

### 7.1 Default cargo features (policy)

Start from **warper’s leaner default set**, then **add**:

- Agent mode family already present
- Explicit `solo_user_byok` equivalent always true via OSS channel
- Local orchestration related features that do not pull cloud

**Never default-enable:** `cloud_mode*`, `viewing_shared_sessions`, `shared_with_me`, `billing_*`, `cloud_agent_runners`, `oz_handoff`, `ambient_agents_rtc` (if cloud-coupled).

### 7.2 Compile-time channel

`Channel::Oss` + Factor `ChannelConfig`:

- `telemetry_config: None`
- `crash_reporting_config: None`
- `autoupdate_config: None` (or Factor GH releases later)
- `server_config`: production URLs **must not be dialed**; prefer absent server client on startup (warper pattern)

## 8. Dependency on upstream crates

Expect workspace members similar to warper (~50–60 crates), not full upstream (78+) until reconciliation decides.

**Do not reintroduce without review:**

- `firebase`, cloud_object_*, `warp_multi_agent_client` (hosted), `warp_server_auth`
- Hosted GraphQL billing surfaces

**May reintroduce carefully from upstream:**

- Local orchestration improvements (`run_agents`)
- Terminal bugfixes, warpify fixes, CLI agent plugins
- Editor/rendering fixes

## 9. Security

1. **Denylist CI** (from warper `check_warper_static_denylist` → `check_factor_static_denylist`) — scopes `startup` + `all-runtime` + new `secrets` (plan 04)
2. No secrets in logs — add unit test that `AgentProviderSecrets` never appears in `tracing` output; forbid `sk-`/`xoxp-` in `app/src` except `secrets.rs`
3. Provider keys only in OS keychain (`dev.factor.Factor`); no silent cross-app read (D-05); add `cargo deny` advisory in `factor-gates.yml` (plan 04)
4. Shell command confirmation policy (PRODUCT §6) — inherit upstream execution profiles; add regression for `start_agent` local-only (plan 05)
5. **Threat model (assessment add):** MCP servers are per-server trust (PRODUCT §6), prompt injection via tool output is untrusted input — treat `read_files`/`grep` results as data, not instructions (add `FakeProvider` injection test under TDD L1). Document in `JUDGMENT_LOG` J-05/J-10.

## 9.1 Data migration (assessment add — D-05 companion)

Optional one-shot importer from `~/.warp`/`~/.warp-oss`/`../warper` paths is **opt-in UI only**. Never at launch. Spec in `specs/FACTOR-002/TECH.md` should list source paths, keychain service mapping, and “imported once” marker under `~/.factor`. No auto-migration.

## 10. Performance budgets (initial)

- Cold start to interactive prompt: no worse than warper on same hardware (measure in smoke)
- Agent first token: dominated by provider; client overhead < 100ms to request start (best-effort)
- Denylist and unit suites: < 5 min CI slice for PR

## 11. Implementation order (code)

See plans. Summary:

1. Import warper → rename Factor  
2. Reconcile upstream (terminal + local agent fixes first)  
3. Drop residual cloud Oz entrypoints  
4. Port Zap BYOP modules + settings UI  
5. Route `generate_multi_agent_output` → BYOP  
6. Enable orchestration + tool schemas  
7. SSH/warpify agent matrix  
8. Packaging  

## 12. Risk register (assessment update 2026-08-13)

| Risk | Mitigation | Plan ref |
|------|------------|----------|
| Warper↔upstream merge hell | Spec-driven port; denylist as invariant; consider strip-upstream fallback; weekly fetch + allowlist PR template | 02 |
| Zap BYOP tightly coupled to Zap UI strings/i18n | Port modules with Factor branding; keep Chinese comments out of user UI or translate; pin `lib/rust-genai` SHA | 03 |
| Orchestration flags require server event push | Feature-detect; use local-only path; disable V2 push if server-bound | 05 |
| Agent file tools on SSH wrong host | Explicit `SessionContext` on every executor; `agent_session_policy` units; D-08 fail-closed (`read_files`/`apply_diff`) | 06, TECH §6.2 |
| License compliance | Preserve MIT/AGPL; document Factor modifications; `cargo deny` advisory | 04 |
| Scope creep (SSH manager, gist sync) | Park in backlog; not v1 (PRODUCT §11.2) | 00 |
| Docker sandbox scope creep into release | Warper parity: dogfood-only flag; not PREVIEW/RELEASE | P-01 |
| **Full test suite red (2964/220)** | Bucket to `NEXT_FAILURES.md`, re-enable `factor-gates.yml` non-required, fix Factor-owned first | 04 |
| **Disk / Metal** | `target` 21G on 93% disk; `script/clean` + `df` gate; Metal soft-skip only for typecheck | 00, J-07/J-08 |
| **Secrets in logs** | Keychain-only, `tracing` test, secret pattern in denylist `secrets` scope | 04, TECH §9 |
| **Perf regression** | Cold start ≤ warper+100ms, `factor_offline_local_smoke` appendix | TECH §10, 04 |

## 13. Open tech questions

See [DECISIONS.md](../adr/DECISIONS.md). Highest impact:

- D-01 base strategy if rebase fails  
- D-08 remote file tool semantics  
- D-09 self-hosted autoupdate in v1?  
- D-10 whether to keep Warper OpenRouter codepath during BYOP bring-up  
