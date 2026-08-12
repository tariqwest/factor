# FACTOR-001: Hard network / platform amputation

## Summary

Factor launches and runs as a local terminal with **no** implicit contact with Warp-hosted account, billing, Drive, sharing, cloud Oz, telemetry, or autoupdate infrastructure.

## Goals

- Local terminal works with outbound network blocked.
- Removed surfaces are deleted or compile-out, not soft-hidden behind retries.
- Denylist CI prevents regressions.

## Non-goals

- Offline LLM weights bundled in app.
- Compatibility with Warp accounts.

## Behaviors

1. Fresh install opens a usable terminal without login.
2. Blocking outbound network does not prevent shell use, panes, themes, keybindings, local settings.
3. No contact with `*.warp.dev` agent/account/update endpoints, Firebase, Rudder, Sentry during startup or normal shell use.
4. Account/billing/Drive/teams/sharing/cloud-Oz UI entrypoints are absent.
5. Telemetry and crash uploads are not initialized.
6. Deep links / stale state cannot resurrect removed surfaces.
7. User-configured BYOP providers **may** be contacted only after user configures them and invokes AI.

## Validation

- `check_factor_static_denylist` scopes: startup, all-runtime
- `factor_offline_local_smoke`
- Manual: launch with firewall deny-all
