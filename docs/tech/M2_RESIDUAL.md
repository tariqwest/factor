# M2 residual (upstream reconcile) — autonomous snapshot

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
