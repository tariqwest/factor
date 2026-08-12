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

### D-04 — BYOP from Zap is the inference plane

**Decision:** Port `agent_providers` (+ readiness/compaction as needed). OpenRouter becomes one provider, not the core.

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

### P-01 — Docker sandbox removal & replacement isolation  🅿️ PARKED

**Status:** Parking lot (not v1). Product surface already dropped (`/docker-sandbox`, no dogfood enablement of `LocalDockerSandbox`).

**Intent when unparked:**

1. Remove remaining docker-sandbox implementation (session type, daemon/image/container plumbing, flags, dead UI), not only hide it.
2. Evaluate lighter local isolation instead of Docker-as-default:
   - micro-VMs
   - Apple native containers
   - Apple Seatbelt profiles
   - Linux bubblewrap / user namespaces
   - **BYO sandbox backend** (pluggable isolation plane, analogous to BYOP for models)

**Default until then:** Host PTY + execution profiles / permissions only; no Factor-managed Docker sandbox.

**Spec home:** [PRODUCT.md §11.1](../product/PRODUCT.md#111-docker-sandbox--full-removal--lighter-isolation-parked).

---

## Change log

| Date | Change |
|------|--------|
| 2026-08-12 | Initial decisions for Factor scaffold |
| 2026-08-12 | P-01: park full docker-sandbox deletion + lighter/BYO isolation options |
