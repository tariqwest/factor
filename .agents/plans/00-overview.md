# 00 — Overview & milestones

## Current status (2026-08-13 assessment — enhancements integrated)

- **M1 done** (import + Factor branding + denylist, `all-runtime` green)
- **M2 partial** (RECONCILE_LOG + M2_RESIDUAL; allowlist ports deferred by design — now time-boxed; drift ~1 month behind `../warp` `a45466be`)
- **M3 largely landed** (BYOP stack, Providers UI, `/model` merge + last-used hydrate, registry TDD, `FakeProvider` API-boundary loop; real Ollama/Anthropic dogfood + controller IT-003 still open)
- **M4 partial** (`start_agent` local tool + `agent_session_policy` units; flags default-on; no full GUI parent→child E2E)
- **M5 advanced offline** — matrix + `factor_m5_smoke`/`factor_ssh_agent_smoke`, session policy + bootstrap asset units, D-08 fail-closed; live UJ-4 + LRC remote bind (H5) + packaging/signing still open
- **Quality:** `factor_*` 5/5, `bootstrap_assets` 4/4, denylist green; **full** `cargo test -p warp --lib` = 7-bucket **0 fail** (1-thread, `ulimit 4096`; was 243→97→7, now 0 via J-13..J-15) + ~10 `mockito` `Operation not permitted` only in network-sandbox (bucketed); outside sandbox 7-bucket green — [NEXT_FAILURES.md](../../docs/tech/NEXT_FAILURES.md), `factor-gates.yml` re-enabled (J-17)
- **Ops:** `target` 21G, disk 93% (16G free), 74 warnings (tab-color / `set_reasoning_effort` dead code), `script/clean` landed, `check_factor_packaging` green
- **Judgments:** [docs/tech/JUDGMENT_LOG.md](../../docs/tech/JUDGMENT_LOG.md)
- **Open for human review:** [docs/OPEN_FOR_REVIEW.md](../../docs/OPEN_FOR_REVIEW.md)

## Vision

**Factor** = Warper-level platform strip + Warp local agent/orchestration + Zap BYOP, branded and isolated.

## Milestones

### M0 — Spec & scaffold (current)

- [x] Product / tech / TDD / decisions docs
- [x] Plans directory
- [x] Repo structure stubs
- [x] Git commits for docs + structure

### M1 — Monorepo import

- [x] Copy Warper (or agreed base) into Factor tree
- Rename branding → Factor
- Green: `cargo check` / offline smoke stub / denylist ported

### M2 — Upstream reconcile

- Add remotes: `upstream` (warpdotdev/warp), optional `warper`, `zap`
- Port terminal + local agent fixes from latest master
- Do **not** reintroduce cloud platform
- Document residual delta

### M3 — BYOP integration (OpenAI-compatible first)

- Port Zap `agent_providers` + settings + secrets — **primary path is generic OpenAI-compatible** (`base_url` → omlx/LiteLLM/Apple FM/`LM Studio`/`Jan`/omniroute local or OpenRouter/Vercel/Portkey/Requesty/Together AI/Fireworks AI/Groq/Cerebras/SiliconFlow remote)
- Seam `generate_multi_agent_output` → BYOP (`byop:{provider}:{model}`)
- Ollama is **secondary** (same `base_url` compat where possible; native `api_type: Ollama` only as fallback)
- FakeProvider tests green; one **OpenAI-compatible** real endpoint manual green (one local + one remote in matrix)

### M4 — Local orchestration + workflow polish

- Orchestration flags on for local paths
- start_agent tool in BYOP schema
- Error→agent and mode fluid UX checks

### M5 — Warpify/SSH agent matrix + packaging

- [x] Matrix doc + offline smoke (`SSH_AGENT_MATRIX.md`, `factor_ssh_agent_smoke`)
- [x] Pure unit slice (session remote flags, D-08 message, SSH util helpers)
- [ ] SSH tool context correctness on live warpified host (UJ-4)
- [ ] LRC remote binding (H5)
- macOS/Linux packages
- Release checklist

## Dependency graph

```text
M0 → M1 → M2 → M3 → M4 → M5
              ↘_____↗
         (M3 can start parallel after M1
          if using warper OpenRouter as interim)
```

**Judgment:** M3 may begin after M1 using Warper tree **before** full M2 if upstream reconcile is slow; must re-test after M2.

## Definition of Done (v1)

See PRODUCT success metrics. Short form:

1. Offline terminal works  
2. BYOP multi-turn agent works on **generic OpenAI-compatible** (one local `omlx`/`LiteLLM`/Apple FM/`LM Studio`/`Jan` + one remote `OpenRouter`/`Vercel`/`Portkey`/`Requesty`/`Together AI`/`Fireworks AI`/`Groq`/`Cerebras`/`SiliconFlow`; Ollama secondary)  
3. Local orchestration child agent works (parent→child E2E in GUI)  
4. Warpify SSH agent shell works on Linux remote (H4 green, H5 LRC bound)  
5. Denylist clean (`startup` + `all-runtime`)  
6. No Warp login required  
7. **Quality gates green:** full `cargo test -p warp --lib` triaged to 0 unexpected failures, `cargo deny`/`audit` clean, `factor_m5_smoke` + offline smokes pass on CI disk budget

## Next sprint — assessment priorities (2026-08-13)

From assessment; not new scope, just making implicit gaps explicit and ordered:

| # | Priority | What | Owner / gate |
|---|----------|------|--------------|
| 1 | **P0 — Quality triage** | Bucket 220 failing `cargo test -p warp --lib` (singleton `AIExecutionProfilesModel`, upstream drift, denylist). File `docs/tech/NEXT_FAILURES.md` or `M2_RESIDUAL` appendix. Re-enable `factor-gates.yml` as non-required PR check. | TDD plan 04 §PR gate |
| 2 | **P0 — BYOP dogfood + controller** | Run `factor_byop_manual_verify` live: **generic OpenAI-compatible local (`omlx`/`LiteLLM`/Apple FM/`LM Studio`/`Jan`) → remote gateway (`OpenRouter`/`Vercel`/`Portkey`/`Together AI`/`Fireworks AI`/`Groq`/`Cerebras`/`SiliconFlow`)**; Ollama secondary; promote `FakeProvider` to blocklist controller harness (IT-003). | Plan 03 phases 8–9, TDD L2 |
| 3 | **P0 — SSH LRC remote bind** | H5: thread `SessionContext` into `shell_command.rs` LRC tag-in so remote long cmds never spawn second local shell. Add `must_not_nest_ssh` + `lrc_attached` coverage. | Plan 06 |
| 4 | **P1 — M2 time-boxed ports** | W2/W3 cherry-pick sprint (terminal/warpify/blocklist) with allowlist PR template; document rejects in `RECONCILE_LOG`. Abort to Option C spike if >50% entangled. | Plan 02 |
| 5 | **P1 — Packaging & ops** | Define signing/notarization, `~/.factor` opt-in importer, `cargo deny`/secret-leak, `script/clean`, perf budget (cold start < warper +100ms, client overhead <100ms). | TECH §8,12 + plan 04 |

Spec stubs `specs/FACTOR-*` are intentionally thin — PRODUCT is canonical; stubs gain checks only when FACTOR slice ships. Docker sandbox stays **Warper parity** (dogfood flag) per P-01 — no deletion campaign this cycle.

## Known gaps folded into plans  

## Sibling repos (workspace)

| Path | Role |
|------|------|
| `../warp` | Upstream reference |
| `../warper` | Strip baseline |
| `../zap` | BYOP reference |
| `../warp-refined` | Optional local_openai/Responses ideas |
| `../warp-cn` | Not a base; localization ideas only |

## Effort rough order

| Milestone | Rough effort | Remaining (assessment) |
|-----------|--------------|------------------------|
| M1 | 1–3 days | Done |
| M2 | 1–3 weeks (risk high) | 1-week time-boxed W2/W3 sprint next; full Option C only if >50% entangled |
| M3 | 1–2 weeks | 2–4 days: controller IT-003 + live OpenAI-compat dogfood (local `omlx`/`LiteLLM`/Apple FM/`LM Studio`/`Jan` + remote `OpenRouter`/`Portkey`/`Together AI`/`Fireworks AI`/`Groq`/`Cerebras`/`SiliconFlow`; Ollama secondary) + error/rate-limit paths |
| M4 | 3–7 days | 2–3 days: GUI parent→child E2E + `/orchestrate` visibility |
| M5 | 1–2 weeks | 3–5 days: H5 LRC bind + UJ-4 live Linux + packaging/signing |
| Quality triage | — | 1–2 days: 220-failure bucket + `cargo deny` + warnings prune |

## Ops notes (new)

- **Disk:** `target` 21G on 93% disk (16G free) already caused Metal soft-skip. Add `script/clean` (prune `target` + `~/.cargo` cache) and document J-07/J-08 budgets.
- **CI:** `factor-gates.yml.skip` per Q-09 — re-enable as non-required check during triage, promote to required after triage.
- **Warnings:** 74 warnings (dead `tab.rs` color, `set_reasoning_effort`). Prune behind flag or `#[allow(dead_code)]` with comment per plan 04 ratchet.
