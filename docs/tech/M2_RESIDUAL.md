# M2 residual (upstream reconcile) — autonomous snapshot

**Date:** 2026-08-12  
**Judgment (J-06):** Document residual; **do not** run a full upstream rebase this pass.

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
