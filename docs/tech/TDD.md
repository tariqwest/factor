# Factor — TDD & Quality Strategy

**Status:** Draft v0.1  
**Depends on:** PRODUCT.md, TECH.md

## 1. Philosophy

Factor is a large Rust GUI monorepo derivative. We cannot unit-test every pixel. We **can** enforce invariants with a layered test pyramid optimized for **local-first amputation**, **BYOP correctness**, and **terminal agent workflow**.

```text
        /\
       /E2E\          few, manual + scripted smokes
      /------\
     / Integ  \       warp integration harness + agent tool loops
    /----------\
   /  Unit/Denylist \  fast, always-on PR gates
  /------------------\
```

**TDD rule for Factor-owned code:** for any new module under Factor control (BYOP ports, denylist, branding isolation, orchestration wiring), write failing tests **before** or **with** the first implementation commit.

**For imported Warp code:** port existing tests; add regression tests when fixing Factor-specific bugs; do not rewrite the universe.

## 2. Test layers

### L0 — Static denylist (PR-blocking)

**Source:** Warper `script/check_warper_static_denylist` → Factor `script/check_factor_static_denylist`

Scopes:

| Scope | What it checks |
|-------|----------------|
| `startup` | Entry bins + core init: no ServerApi/Auth/Firebase/Sentry dial |
| `startup-network` | No hosted client construction on launch path |
| `all-runtime` | app/src + crates: no warp.dev / firebase / rudder / sentry endpoints |
| `removed-hosted-surface` | No resurrected Drive/sharing/cloud agent symbols |

**TDD:** When adding a strip PR, first add the denylist pattern, watch CI fail, then delete code.

Fixtures: `tests/denylist/expected_clean_paths.txt` (documentation of intentional exceptions, e.g. comments in specs/).

### L1 — Unit tests (PR-blocking)

Target modules (Factor-owned first):

| Module | Examples |
|--------|----------|
| BYOP id encode/decode | `byop:prov:model` roundtrip |
| Provider settings validation | empty base_url excluded from picker |
| Tool schema → API tool mapping | shell/read/grep/start_agent |
| Prompt renderer | template pick by api_type; rules injection |
| Readiness / repair state | pending tool results sidecar serde |
| Channel/AppId | Factor paths never equal Warp/Warper |
| Feature flag matrix | orchestration on ⇒ start_agent tool visible |

Commands (post-import):

```bash
cargo test -p warp --lib byop_ -- --nocapture
cargo test -p warp --lib agent_providers
./script/presubmit  # fmt + clippy + unit subset
```

### L2 — Integration tests (nightly + pre-release)

Use upstream/warper `crates/integration` harness where available.

Scenarios (FACTOR-mapped):

| ID | Scenario |
|----|----------|
| IT-001 | App launches logged-out; terminal accepts input |
| IT-002 | Agent mode toggle; empty provider shows settings CTA |
| IT-003 | Mock provider: multi-turn tool call (shell + read) |
| IT-004 | Failed block → agent context attachment |
| IT-005 | Local start_agent creates child conversation |
| IT-006 | Warpify local shell: preexec creates blocks |
| IT-007 | CLI agent detection for `echo claude` fixtures / fake binary |

**Mocking inference:** inject a `FakeProvider` behind the BYOP trait/seam so CI needs no real API keys.

### L3 — Offline / network smokes (pre-release)

Scripts (stubbed now, real post-import):

| Script | Asserts |
|--------|---------|
| `script/factor_offline_local_smoke` | Launch with network namespace; shell works |
| `script/factor_byop_manual_verify` | Documented manual matrix with real keys |
| `script/factor_ssh_agent_smoke` | Offline: denylist + SSH/warpify pure units; optional live when `FACTOR_SSH_TARGET` set — see `docs/tech/SSH_AGENT_MATRIX.md` |

### L4 — Manual exploratory (release)

Checklist in `tests/e2e/RELEASE_CHECKLIST.md`.

## 3. TDD workflow for a feature slice

Example: FACTOR-003 BYOP

1. Write PRODUCT behaviors for provider CRUD + agent turn  
2. Write unit tests for id codec + settings  
3. Write integration test with FakeProvider for one tool loop  
4. Implement settings + router until green  
5. Add denylist exception only if justified (prefer none)  
6. Manual verify one real provider  

## 4. FakeProvider (landed)

**Location:** `app/src/ai/agent/api/fake_provider.rs` (`#[cfg(any(test, feature = "test-util"))]`)

```text
FakeProvider::new([ScriptedAssistantTurn...]).stream(RequestParams) -> ResponseStream
  - scripted Text / ToolCalls turns (queue)
  - records requests (input count, tool-result presence)
  - emits ResponseEvent Init / CreateTask / AddMessages / Finished
  - tool args parsed via agent_providers::tools::REGISTRY
```

**Tests:** `cargo test -p warp --lib fake_provider`

**Judgment:** Inject at `ResponseStream` boundary (same shape as OpenRouter/BYOP), not HTTP sockets. The IT-003 controller-boundary harness is landed in `response_stream::tests::it003_fake_provider_multi_turn_response_stream`. The full `BlocklistAIHistoryModel` / UI-history action loop remains a later integration step.

## 5. Coverage priorities (not vanity %)

Must be covered before claiming “BYOP done”:

- [ ] **Generic OpenAI-compatible chat + tools** — **must**: local (`omlx`/Apple FM/`LiteLLM`/`LM Studio`/`Jan`/`omniroute`) + remote (`OpenRouter`/Vercel/`Portkey`/`Requesty`/`Together AI`/`Fireworks AI`/`Groq`/`Cerebras`/`SiliconFlow`) `base_url`s via `factor_byop_manual_verify` (plan 03 phase 9) — **Ollama is secondary**, native Anthropic/Gemini opportunistic
- [x] Multi-turn tool result continuation — `api::fake_provider` (`multi_turn_shell_then_final`) + `response_stream` IT-003 controller boundary harness
- [x] LRC tag-in does not spawn wrong shell — `SessionContext` + `lrc_attached` / `must_not_nest_ssh` wired into `shell_command.rs` executor (plan 06 H5)
- [x] start_agent local child (orchestration) — unit: Local `from_args`, REGISTRY, `orchestration_enabled` gating, FakeProvider scripted tool, executor CreateAgent + child run_id Success (no full GUI; see IT-005)  
- [x] Denylist all-runtime
- [x] **Assessment add — error paths:** 401/429/5xx mapped to `AIApiError::ErrorStatus` in `chat_stream.rs`; response body shown in picker, no retry for 401 (plan 03 phase 8)
- [x] **Assessment add — full suite bucket:** `cargo test -p warp --lib -- --test-threads=1` is 0 fail with `ulimit -n 4096`; `NEXT_FAILURES.md` bucketed

> Assessment 2026-08-16: `cargo test -p warp --lib -- --test-threads=1` is 3195/0. IT-003 controller boundary harness is green. Do not claim BYOP done until **generic OpenAI-compat local+remote** + full `BlocklistAIHistoryModel` UI-history loop + NEXT_FAILURES bucket are green (Ollama not gate).

Nice-to-have:

- Gemini / DeepSeek native quirks  
- Compaction sidecar  
- Prompt cache stability  

## 6. CI layout (planned — assessment update)

```yaml
# conceptual — re-enable factor-gates.yml as non-required during triage (plan 04)
pr:
  - denylist (startup + all-runtime + secrets)
  - fmt + clippy (Factor-owned = 0 warnings; vendored allow with comment)
  - unit tests (factor_* + byop_* + agent subset) — full suite triage in NEXT_FAILURES.md, not ignored
  - cargo deny check (advisory) + secret-leak pattern
nightly:
  - integration suite (IT-003 controller harness with FakeProvider)
  - offline smoke (macOS + linux runners, `factor_m5_smoke` + `factor_offline_local_smoke`)
  - perf budget check (cold start ≤ warper+100ms, see TECH §10)
release:
  - full checklist + signed builds + `cargo audit`
```

## 7. Regression policy

Any bug from production/manual use gets:

1. Minimal failing test (unit if possible, else integration, else scripted smoke doc)
2. Fix
3. Link to FACTOR spec id if behavioral

## 8. What we will not TDD early

- Full wgpu pixel diffs  
- Every upstream terminal edge case already tested upstream  
- Real paid provider cost in CI  

## 9. Stub layout in this repo (pre-import)

```text
tests/
  denylist/          # patterns + docs
  unit/              # placeholder READMEs pointing to cargo tests post-import
  integration/       # scenario catalog
  e2e/               # checklists
  fixtures/          # fake provider scripts, sample settings.toml
script/
  check_factor_static_denylist  # stub until import
  factor_offline_local_smoke   # stub
  presubmit                    # stub
```
