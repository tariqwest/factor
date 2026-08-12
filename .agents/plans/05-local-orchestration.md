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
