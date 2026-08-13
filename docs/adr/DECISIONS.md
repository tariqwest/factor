# Factor — Decisions & Open Questions

Record of judgments made without waiting, and items needing human review.

## Decided (best judgment)

### D-01 — Construction base: Warper-first, then upstream reconcile

**Decision:** Use Warper as the product/amputation baseline; reconcile toward latest `warpdotdev/warp` rather than stripping upstream from zero.

**Why:** Warper already deleted cloud surfaces and has denylist discipline. Upstream is fresher but still platform-heavy.

**Fallback:** If merge cost exceeds ~2 engineer-weeks of conflict (estimate), switch to strip-latest-upstream with Warper checklist.

**Review:** Confirm after first reconcile dry-run.

### D-02 — Product name: Factor

**Decision:** Ship as **Factor** (`dev.factor.Factor`).

**Alternatives parked:** Nacelle, Deflector, WarpFactor (too close to trademark risk?), Impulse.

### D-03 — Cloud Oz out; local agent + local orchestration in

**Decision:** Keep Agent Mode, blocklist tools, `start_agent` / local orchestration. Strip cloud mode, handoff, hosted ambient, capacity modals.

### D-04 — BYOP from Zap is the inference plane — OpenAI-compatible first

**Decision:** Port `agent_providers` (+ readiness/compaction as needed). **Primary target is generic OpenAI-compatible** `base_url` — local `omlx`/`LiteLLM`/Apple FM/`omniroute` and remote `OpenRouter`/Vercel AI Gateway/`Portkey`/`Requesty` share one code path. OpenRouter becomes one provider, not the core. **Ollama is secondary** (use OpenAI-compat URL if exposed, else `api_type: Ollama` fallback — not design driver); Anthropic/Gemini/DeepSeek native dialects opportunistic.

**Rationale (2026-08-13 retarget):** Maximizes “any endpoint” reach without multiplying provider-specific tool/prompt branches.

### D-05 — Settings/data isolation from Warp and Warper

**Decision:** Separate app id, settings root, keychain service. No silent migration at launch.

### D-06 — English-first UI in v1

**Decision:** Do not block on Zap Fluent i18n or warp-cn hardcoding. May port Fluent later.

### D-07 — Shell risk confirmations: inherit upstream profiles first

**Decision:** Reuse Warp execution profiles / agent command permissions rather than inventing a new classifier in v1.

### D-10 — Keep Warper OpenRouter path during BYOP bring-up

**Decision:** Temporarily allow OpenRouter adapter as fallback backend behind feature flag `legacy_openrouter_backend` (default off once BYOP green). Delete after one stable release.

### D-11 — License

**Decision:** Preserve upstream MIT (warpui*) + AGPL-3.0 split. Factor modifications under AGPL unless in MIT crates.

### D-12 — Git history strategy for monorepo import

**Decision:** Phase B imports Warper tree as a single “import” commit (or git subtree/vendor) with clear message; full history of Warper not required in Factor for v0. Optionally add `upstream` remotes for merge work later.

**Review:** Prefer `git subtree` or copy + commit for simplicity of Factor as new product.

---

## Needs input (flagged)

### Q-01 — Public GitHub remote

Create `factor` under which org/user? Private vs public AGPL?

**Default assumption:** Local-only until you say; no `git remote add` without approval.

### Q-02 — Autoupdate in v1

Self-hosted GitHub Releases (warp-refined style) vs none?

**Default:** None in v1 (Warper-like). Document as P2.

### Q-03 — Computer use

Keep local computer-use features if they work offline?

**Default:** Disable in default features until explicitly validated; code may remain if denylist clean.

### Q-04 — Trademark / branding legal

Is “Factor” / “warp factor” reference OK for public marketing?

**Default:** Use Factor; avoid Warp logos/marks; “based on open-source Warp” attribution in About.

### Q-05 — SSH host manager / SFTP / gist sync from Zap

In v1?

**Default:** **Out of v1.** Terminal SSH + warpify only.

### Q-06 — Default provider onboarding

Empty state CTA only vs bundled Ollama detection vs OpenRouter signup link?

**Default:** Empty state + docs; optional Ollama auto-detect if local daemon present (P1).

### D-08 — Remote file tools over SSH  ⚠️ HIGH

When warpified on SSH, should `read_files` / `apply_file_diffs` operate on **remote** paths via SSH/SFTP, **local** only, or fail closed with message?

**Default judgment:** Shell tools remote; file tools **fail with clear error** unless remote FS support already exists in session — then use it. Do not silently read local paths when user is on remote.

**Needs validation** against actual warpify/SSH code paths during Phase D.

### D-09 — OrchestrationV2 / event push

Enable only if fully local; else Orchestration v1 (`start_agent`) only for v1.

**Default:** start_agent local only; evaluate V2 during reconcile.

### P-01 — Docker sandbox (Warper approach)

**Status:** **Follow Warper.** Keep implementation; do not delete or hard-disable.

| Channel | `LocalDockerSandbox` |
|---------|----------------------|
| Dogfood | **On** (`DOGFOOD_FLAGS`) |
| Preview / Release | **Off** |
| Slash `/docker-sandbox` | Registered when flag enabled |
| UI tab/session entrypoints | Existing flag gates (Warper parity) |

**Optional later:** lighter isolation substitutes (PRODUCT §11.1).

### P-02 — Maintainer answers (OPEN_FOR_REVIEW MC, 2026-08-12)

| ID | Resolution |
|----|------------|
| A-01…A-07 | **Accept all** autonomous defaults at time of MC |
| Q-01 | **Warper approach** — keep code, dogfood flag |
| Q-02 | Keep OpenRouter path until BYOP onboarding solid; then deprecate |
| Q-03 | Reasoning effort persistence **P1** after `/model` picker trusted |
| Q-04 | Keep `/create-new-project` agent-guided scaffold |
| Q-05 | **Add** `/rename-conversation` slash command |
| Q-06 | **Separate** settings pickers for title / ActiveAI / next-command models |
| Q-07 | Defer isolation backend choice |
| Q-08 | Settings/data under **`~/.factor`** |
| Q-09 | **Skip** requiring `factor-gates` CI for now (local scripts) |

---

## Change log

| Date | Change |
|------|--------|
| 2026-08-12 | Initial decisions for Factor scaffold |
| 2026-08-12 | P-01: park full docker-sandbox deletion + lighter/BYO isolation options |
| 2026-08-12 | P-02: maintainer MC resolutions; P-01 unparked for code deletion |
| 2026-08-12 | P-01: hard-disable + deletion phases (product DROP confirmed) |
| 2026-08-12 | P-01 final: Warper approach (keep docker sandbox, dogfood flag) |
