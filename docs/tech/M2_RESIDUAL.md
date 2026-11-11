# M2 residual (upstream reconcile) — autonomous snapshot

## Verification update — 2026-11-11
Metal-enabled macOS bundle built successfully using `/Applications/Xcode-beta.app` (Xcode 27.3, 27C5020n) and Apple Metal 32023.925. The bundle contains a nonempty `shaders.metallib`; strict deep codesign verification passed. Launch produced an on-screen 1812×1010 window; the app was subsequently terminated. This verifies startup, not live inference or interactive workflows.

Fresh workspace Nextest: **5,123 passed, 80 skipped, 0 failed** across 61 binaries (`--workspace --exclude command-signatures-v2`). Doctests: **7 passed, 3 ignored, 0 failed**. Completer v2: **108 passed, 4 skipped**. Formatting and diff checks passed for that verification. Environment configuration is committed in `cfda418c`; rustls test dependency and doctest fixes in `59f539e2`. Older outstanding workspace/Metal statements below are superseded by this evidence.

Still outstanding: live BYOP local/remote multi-turn GUI/history, IT-005 parent→child pane→inference→result, SSH UJ-4/H4–H7, and fresh remote CI verification. The Factor Agent switch investigation found legacy OpenRouter-only availability gating and a switch bound to effective availability rather than the saved preference; the source fix now accepts configured BYOP models and displays the saved switch preference. All 15 AI settings tests, the static denylist, and offline-local smoke pass. The fixed Metal app bundle was rebuilt and passed strict deep codesign verification; launch and window creation were confirmed. GUI automation exposed no accessible controls or usable screenshot, so interactive toggle clicks and live inference remain unverified.

**Date:** 2026-08-12  
**Judgment (J-06):** Document residual; **do not** run a full upstream rebase this pass.

## Current status (2026-11-11)

Publication is complete: Factor `main` tracks `origin/main` at `74f2796`; fork default `master` was not changed. `origin/master` and `upstream/master` were fetched at `be0d42bc668d9a6b22e51e2ae324e8c35bd142e5`. Factor and upstream have no merge base. Full upstream history is available locally; the shallow sibling-clone limitation is superseded.

W2/W3 allowlisted ports and W4 bulk `run_agents` remain deferred, not completed by publishing `main`. Compare explicit commit/tree baselines; do not assume ancestry or attempt a wholesale merge. First finish live BYOP, IT-005 and SSH dogfood, then select narrow fixes for demonstrated bugs. Earlier disk constraints below are historical: build artifacts and project Cargo downloads now use an external volume.

## Baseline

See [RECONCILE_LOG.md](./RECONCILE_LOG.md) for SHAs and reject list.

## What’s already “good enough” without M2

| Capability | Source | Notes |
|------------|--------|--------|
| Platform strip / denylist | Warper → Factor | FACTOR-001 green |
| Local agent + BYOP | Zap port + Factor wiring | Providers, `/model`, FakeProvider unit loop |
| Local `start_agent` | Factor tool + executor tests | GUI parent→child still dogfood |
| Docker sandbox | Warper flag/dogfood policy | Not amputated |
| SSH/warpify substrate | Warper tree | Matrix + offline smoke scaffolded |

## Still deferred (true M2 work)

| Wave | Item | Risk | Suggested trigger |
|------|------|------|-------------------|
| W1 | Dep/toolchain bump to latest upstream | Medium (disk, lockfile) | After free disk + clean dogfood |
| W2 | Terminal/warpify/ssh cherry-picks | High (merge conflicts) | When SSH dogfood finds a real bug vs upstream |
| W3 | Blocklist local fixes from upstream | Medium | When agent tool bugs appear |
| W4 | `run_agents` bulk orchestration from upstream | High | After `start_agent` UI E2E solid |
| W5 | CLI agent session deltas | Low–med | When 3p CLI UX regresses |
| W6 | Explicit cloud rejects | N/A | Keep denylist |

## How to resume M2 later

```bash
# Compare allowlisted paths only (example)
diff -rq ../warp/app/src/terminal ../factor/app/src/terminal | head
# Port one commit with evidence (warper skill pattern)
# Always: ./script/check_factor_static_denylist all-runtime
```

## Decision needed from you

- **Defer M2 rebase** until after BYOP+orch dogfood (recommended), **or**
- Start W2/W4 porting now despite disk/time cost.
