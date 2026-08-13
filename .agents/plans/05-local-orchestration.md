# 05 — Local orchestration enablement

## Goal

Make Factor’s **local** multi-agent orchestration a supported, default-on capability for power users, without any cloud Oz handoff.

## Definitions

| Term | Meaning |
|------|---------|
| Local orchestration | Parent agent spawns child agent conversations/tasks on-device via `start_agent` / local `run_agents` |
| Cloud Oz | Hosted runners, cloud mode, handoff, capacity, RTC ambient |

## Current state by tree

| Tree | Local start_agent executor | run_agents | Cloud Oz surface |
|------|----------------------------|------------|------------------|
| Warper | Yes | No (or limited) | Mostly stripped |
| Zap | Missing executor | N/A | Partial leftovers |
| Upstream | Yes | Yes | Large |

## Work items

1. **Flags:** Enable `FeatureFlag::Orchestration` in Factor OSS/dogfood flags; do **not** enable cloud-coupled flags.  
2. **Settings:** `orchestration_enabled` default true when local agent available.  
3. **BYOP tools:** Ensure model can call `start_agent` (schema + adapter).  
4. **Executor:** Keep Warper `start_agent.rs`; port upstream fixes if local-only.  
5. **UI:** Keep orchestration block rendering; remove any “Open in Oz cloud” CTAs.  
6. **run_agents:** Evaluate upstream `run_agents.rs` for local-only; port if no server dependency (D-09).  
7. **Tests:** IT-005 child conversation created; unit test tool adapter.

## Explicit rejects

- `OrchestrationLaunchModal` marketing cloud features  
- `cloud_mode_from_local_session`  
- `handoff_local_cloud` / `handoff_cloud_cloud`  
- Cloud ambient task polling  

## Exit criteria

- [ ] Parent agent can spawn local child with BYOP model  
- [ ] Child uses same provider config  
- [ ] No network to warp.dev during spawn  
- [ ] Slash command `/orchestrate` (or equivalent) visible only when enabled  
- [ ] **Assessment additions:** provider 401/429 surfaced in UI (plan 03 phase 8), `agent_session_policy` + `start_agent_tests` green, full GUI E2E in `pane_group` with real `BlocklistController` (IT-005)

## Assessment additions — E2E harness (close J-04)

J-04 notes unit/integration only, no full GUI. To claim DoD #3:

1. Drive `BlocklistController` with `FakeProvider` scripted `start_agent` → assert `SessionContext` child pane + transcript `StartAgentResult::Success` with child `run_id` (see `execute/start_agent_tests.rs` pattern).
2. Gate `/orchestrate` on `FeatureFlag::Orchestration` + `orchestration_enabled` — test both on/off.
3. Measure no `warp.dev` during spawn (denylist `startup-network` scope or `factor_offline_local_smoke` with `FACTOR_SSH_TARGET` unset).  
