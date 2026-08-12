# Open for your review (decision queue)

**Status:** Resolved 2026-08-12 (maintainer MC + Warper docker override).  
Canonical: [adr/DECISIONS.md](./adr/DECISIONS.md) P-01 / P-02.

## Autonomous defaults (A-01…A-07)

| ID | Decision | Notes |
|----|----------|--------|
| A-01 | `/model` priority: terminal → last-used → profile → OpenRouter legacy → default | Keep |
| A-02 | Persist last-used model id on picker change | Keep |
| A-03 | **Warper approach** for Docker sandbox | Code kept; dogfood flag |
| A-04 | `start_agent` always local execution mode | Keep |
| A-05 | Empty secrets load is not ERROR | Keep |
| A-06 | factor-gates sketch | **Q-09:** skip CI require for now |
| A-07 | Docker code retention | Aligns with Warper |

## Product / tech questions

| ID | Decision | Implication |
|----|----------|-------------|
| **Q-01** | **Warper approach** | Keep docker-sandbox; dogfood `LocalDockerSandbox`; no deletion |
| **Q-02** | Keep OpenRouter until BYOP onboarding solid | Dual path OK; deprecate later |
| **Q-03** | Reasoning effort persistence P1 after picker trusted | Zap-style map after dogfood |
| **Q-04** | Keep `/create-new-project` agent-guided scaffold | Slash KEEP |
| **Q-05** | Add `/rename-conversation` slash | Register + TDD |
| **Q-06** | Separate pickers for title / ActiveAI / next-command | Settings + LLMPreferences |
| **Q-07** | Defer isolation tech pick | Host PTY + permissions |
| **Q-08** | `~/.factor` data/settings | No `~/.warp` share by default |
| **Q-09** | Skip CI workflow for now | Local denylist + cargo tests |

## Suggested execution order

1. ~~Docker~~ → **Warper parity done**
2. ~~`/rename-conversation` slash~~ → **done**
3. ~~Separate model pickers~~ → **settings fields + LLMPreferences + Specialized models UI** (Settings → Agents → Providers; free-form ids; dropdowns later)
4. ~~Reasoning effort persistence~~ → **done** (`byop_last_used_reasoning` + set/get)
5. ~~OpenRouter demotion~~ → **soft-demote labels** (path kept per Q-02)
