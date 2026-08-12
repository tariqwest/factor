# Open for your review (decision queue)

**Status:** Resolved 2026-08-12 (maintainer MC answers).  
Canonical long-form notes also in [adr/DECISIONS.md](./adr/DECISIONS.md) (P-02 block below mirrored there).

## Autonomous defaults (A-01…A-07) — **ACCEPTED**

| ID | Decision | Notes |
|----|----------|--------|
| A-01 | `/model` priority: terminal → last-used → profile → OpenRouter legacy → default | Keep |
| A-02 | Persist last-used model id on picker change | Keep |
| A-03 | `LocalDockerSandbox` hard-disabled except test overrides | Product DROP; residual under deletion inventory |
| A-04 | `start_agent` always local execution mode | Keep |
| A-05 | Empty secrets load is not ERROR | Keep |
| A-06 | Lightweight `factor-gates.yml` was fine as a sketch | **Superseded by Q-09:** do not run/require CI workflow for now |
| A-07 | Docker residual deleted in phases | View entrypoints gone; shell/PTY residual |

## Product / tech questions — **RESOLVED**

| ID | Decision | Implication |
|----|----------|-------------|
| **Q-01** | **Delete residual docker-sandbox product code** | Hard-disable + phased deletion ([DOCKER_SANDBOX_DELETION.md](./tech/DOCKER_SANDBOX_DELETION.md)) |
| **Q-02** | **Keep OpenRouter until BYOP onboarding solid** | Dual path OK; deprecate OpenRouter UI later, not now |
| **Q-03** | **Yes — reasoning effort persistence P1 after picker trusted** | After dogfood `/model` hydrate; port Zap-style map |
| **Q-04** | **Keep `/create-new-project` agent-guided scaffold** | Stay in slash KEEP matrix |
| **Q-05** | **Add `/rename-conversation` slash command** | Register + wire rename; TDD matrix update |
| **Q-06** | **Separate pickers for title / ActiveAI / next-command** | Settings + LLMPreferences fields; not “always = base” forever |
| **Q-07** | **Defer isolation tech pick** | Host PTY + permissions until later; P-01 isolation candidates stay parked |
| **Q-08** | **`~/.factor` data/settings isolation** | No sharing `~/.warp` by default |
| **Q-09** | **Skip CI workflow for now** | Prefer local scripts (`check_factor_static_denylist`, cargo tests); don’t require `factor-gates` |

## Suggested execution order (from answers)

1. **Docker sandbox deletion** — phases 1–2 done; residual shell/PTY next  
2. **`/rename-conversation` slash** (Q-05) + registry TDD  
3. **Separate model pickers** (Q-06) — settings + preferences (larger)  
4. **Reasoning effort persistence** (Q-03) after picker dogfood  
5. **OpenRouter demotion** only after BYOP onboarding (Q-02)  
6. **No CI require** until you revisit Q-09  

## Still not decided here

- Isolation backend when unparked (Q-07 deferred intentionally)  
- Full controller FakeProvider IT-003  
- GUI Metal / packaging / SSH matrix (plan M5)
