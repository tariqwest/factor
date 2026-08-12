# Upstream reconcile log (M2)

## SHAs

| Tree | SHA |
|------|-----|
| Factor base (Warper import) | `1c83af37` (warper) branded |
| Upstream `../warp` | `a45466be` (2026-08-12) |
| Zap BYOP source | `5d874456` |

## Strategy selected

**Option B** (subsystem allowlist ports), with Option C reserved.

## Crate deltas (upstream-only, rejected for cloud/platform)

Rejected by default (do not add without ADR):

- `cloud_object_*`, `firebase`, `graphql`, `warp_multi_agent_client`, `warp_server_auth`
- `warp_tui` (optional later), `mcp` crate (app may still have MCP UI)
- `onboarding` as cloud onboarding — review before port

## Allowlist status

| Wave | Area | Status |
|------|------|--------|
| W1 | Tooling/deps | Deferred (disk-constrained; pin current lockfile) |
| W2 | Terminal/warpify/ssh | **Deferred** — Factor SSH LOC already > upstream (Warper ports kept); diff later |
| W3 | Blocklist/agent local | In progress via M3 BYOP integration |
| W4 | Local orchestration | Factor retains `start_agent`; `run_agents` from upstream **not yet ported** |
| W5 | CLI agents | Deferred |
| W6 | Explicit rejects | This log |

## Notable upstream files not yet ported

- `app/src/ai/blocklist/action_model/execute/run_agents.rs` (~800 LOC)
- Larger `cli_agent_sessions` / `blocklist` deltas
- SoloUserByok / custom_model_routers (partially superseded by BYOP)

## Next reconcile actions

1. After M3 compiles, diff `warpify` and port bugfixes only.
2. Evaluate local-only `run_agents` port (FACTOR-005).
3. Timebox full W2–W5 after first green `cargo check`.
