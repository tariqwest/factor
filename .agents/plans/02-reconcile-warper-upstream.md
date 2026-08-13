# 02 — Reconcile Warper with latest upstream Warp

## Goal

Absorb **relevant** upstream improvements (terminal reliability, local agent, warpify/SSH, CLI agents, orchestration **local** primitives) without reintroducing the cloud platform Warper removed.

## Context (as of 2026-08-12 workspace)

| Tree | Approx state |
|------|----------------|
| `../warp` | master HEAD ~2026-08-12, largest (~1.67M LOC, 78 crates) |
| `../warper` | ~2026-07-06, hard strip (~0.92M LOC, 54 crates), last upstream reconciliation ~2026-07-03 |
| Factor after M1 | Warper-derived |

Upstream has continued to add: TUI, cloud object crates, multi-agent client, SoloUserByok, custom routers, orchestration launch modals, etc. **Most cloud additions must not land in Factor.**

## Strategy options

### Option A — Merge upstream into Factor (Warper base)

```text
factor (warper-based)
  remote add upstream https://github.com/warpdotdev/warp.git
  fetch upstream master
  merge / rebase with aggressive ours for deleted cloud paths
```

**Pros:** Preserves Factor/Warper deletions as first-class history.  
**Cons:** Massive conflicts; deleted files reappear as conflict markers.

### Option B — Port by subsystem (recommended default)

1. Diff upstream vs warper by area.
2. Cherry-pick or manually port **allowlisted** patches.
3. After each port: denylist + unit + smoke.

**Pros:** Controlled; matches Warper’s own `upstream-change-analysis` skill philosophy.  
**Cons:** Easy to miss fixes; requires discipline.

### Option C — Strip-latest-upstream (fallback per D-01)

1. Start from upstream master.
2. Apply FACTOR-001 denylist + Warper deletion checklist until green.
3. Re-apply Factor branding + any Factor commits.

**Pros:** Maximum code currency.  
**Cons:** Re-pays strip cost; high risk of missing a phone-home path.

**Default:** **Option B**, with Option C trigger if Option B stalls.

## Allowlist / denylist for ports

### Allowlist (want)

| Area | Examples |
|------|----------|
| Terminal core | PTY, blocks, grid, resize, input |
| Warpify / bootstrap | Shell hooks, SSH warpify |
| CLI agent sessions | Codex/Claude/Gemini plugins |
| Local agent blocklist | Tool executors, controller bugfixes |
| Local orchestration | `start_agent`, local `run_agents` **if local-only** |
| Editor / markdown / rendering | Crash and CJK fixes |
| Settings file format | Non-cloud settings |
| Completer / syntax | Local only |

### Denylist (reject even if upstream “improves” them)

| Area | Examples |
|------|----------|
| Auth / Firebase | login slides, anonymous user |
| Billing / entitlements | RequiresUpgrade gates that force account |
| Drive / cloud objects | new crates `cloud_object_*` |
| Shared sessions / RTC | |
| Cloud mode / Oz handoff | `cloud_mode*`, handoff_*, capacity modals |
| Hosted multi-agent client | `warp_multi_agent_client` as required runtime |
| Telemetry / Sentry / Rudder | |
| Hosted autoupdate | `releases.warp.dev` |
| GraphQL cloud API expansions | unless pure types for local reuse |

## Procedure (Option B)

### 1. Establish remotes & SHAs

```bash
cd factor
git remote add upstream https://github.com/warpdotdev/warp.git  # if not present
git remote add warper-src https://github.com/ruslanvakhitov/warper.git
git fetch upstream master
git fetch warper-src master  # optional
```

Write to `docs/tech/RECONCILE_LOG.md`:

- Factor base SHA  
- upstream master SHA  
- warper SHA  

### 2. Generate area diffs

```bash
# Example: list files changed upstream since warper fork point
# (warper first public base often 0dbd3d5 Initial public release — not perfect merge-base)
git log --oneline warper-src/master..upstream/master | head
```

Use Warper skill pattern (`upstream-change-analysis` if present in imported `.agents/skills`).

Subsystem inventory script (to add):

```bash
script/factor_diff_upstream_areas
# emits: terminal, ai/blocklist, ai/agent, warpify, ssh, features, cargo features
```

### 3. Port in waves

| Wave | Content | Gate |
|------|---------|------|
| W1 | Build/tooling, rustc, deps (careful) | `cargo check` |
| W2 | Terminal + warpify + ssh | integration + denylist |
| W3 | Blocklist / agent local | unit + agent tests |
| W4 | Local orchestration only | start_agent tests |
| W5 | CLI agents / suggestions | smoke |
| W6 | Explicit rejects log | RECONCILE_LOG |

**Never** do “merge entire master” in one PR without wave gates.

### 4. Conflict resolution rules

1. If conflict in **deleted** cloud module: keep deleted; note upstream change in RECONCILE_LOG as rejected.  
2. If conflict in **shared** terminal file: prefer upstream fix, re-apply Factor branding.  
3. If conflict in **feature flags**: default Factor lean set; enable only local-safe flags.  
4. If unsure: fail closed (keep strip), open ADR.

### 5. Cargo features reconciliation

Diff `app/Cargo.toml` default features:

```bash
# conceptual
comm -13 <(factor defaults) <(upstream defaults)  # upstream-only
```

For each upstream-only feature:

- Classify: local-safe / cloud / unknown  
- Enable local-safe after compile+denylist  
- Document cloud rejects  

### 6. Crate set reconciliation

Upstream may add crates Factor lacks.  
**Do not** add `cloud_object_*`, `firebase`, hosted clients.  
**Do** consider local-only crates that terminal/agent need.

### 7. Continuous denylist

Every wave ends with:

```bash
./script/check_factor_static_denylist all-runtime
./script/factor_offline_local_smoke
```

## Finding the historical fork point

Warper and Warp share early history (`0dbd3d5` initial public release appears in both). Exact merge-base may be messy after Warper rewrites.

Practical approach:

1. Use file-level diff of current trees, not only git merge-base.  
2. `diff -rq warper crates app` vs `warp` for high-signal dirs.  
3. Prefer porting **commits with clear terminal/agent messages** from upstream log since 2026-07-01.

## Success criteria

- [ ] Factor builds against dependency set compatible with modern upstream (or documented pins)
- [ ] No denylist regressions vs M1  
- [ ] At least one known upstream terminal fix verified present  
- [ ] RECONCILE_LOG lists rejected cloud features  
- [ ] Local agent still runs (OpenRouter or BYOP)
- [ ] Full-suite triage doc updated (`NEXT_FAILURES.md` or `M2_RESIDUAL` appendix) — 220-failure bucket resolved or explicitly ignored with owner

## Cadence & PR template (assessment addition)

**Weekly:** refresh `git fetch upstream master` and `script/factor_diff_upstream_areas`; post diff size to `RECONCILE_LOG`. Do not let Factor drift >2 weeks behind `warpdotdev/warp` without a W2/W3 wave.

**Allowlist PR template (use for every W1–W5 wave):**

```md
Wave: W2 terminal / W3 blocklist / ...
Upstream SHAs: <range>
Files: <list>
Tests: `cargo test -p warp --lib` filtered + `check_factor_static_denylist all-runtime`
Rejects: <cloud files intentionally kept deleted, with reason>
Perf: cold-start / smoke notes
```

**Disk/CI hygiene:** before `cargo check -p warp --bin factor-oss`, run `script/clean` if disk >90% (see plan 00 Ops notes, J-07/J-08). Re-enable `factor-gates.yml` as non-required during triage so `all-runtime` still gates PRs even while full suite is being bucketed.

## Effort & abort

- Timebox first full pass: **10 working days**  
- If >50% of allowlisted ports blocked by entangled cloud types → trigger **Option C** design spike (2 days) before continuing  

## Deliverables

1. `docs/tech/RECONCILE_LOG.md`  
2. Series of conventional commits per wave: `feat(terminal): port ... from upstream`  
3. Updated FACTOR-001 denylist if new hostnames appear upstream

## 2026-08-13 autonomous pass — W2/W3 time-box (J-06)

**This pass:** W2/W3 were **time-boxed and deferred** (J-06). Residual is now tracked as P1, not P0 for this landing:

- Warper-fork drift ~5 weeks (2026-07-06 → 2026-08-12 upstream). File-level diff upstream vs warper is large; full port would re-pay amputation tax.
- **Landed in this pass:** denylist + Warper base still valid, bootstrap fix, open_in_warp rename, BYOP generic OpenAI-compat retarget, script/clean disk hygiene, H5 policy.
- **Deferred to next campaign (2 engineer-weeks, Option B waves):** W2 terminal/warpify/ssh cherry-picks and W3 blocklist/agent local merges. Trigger `script/factor_diff_upstream_areas` + weekly RECONCILE_LOG refresh per §Cadence. Abort to Option C per §Effort if >50% entangled.
- **Gate for next W2 wave:** `cargo check` + `check_factor_static_denylist all-runtime` + `cargo test -p warp --lib -- --test-threads=1` (ulimit 4096) green on ~3175 tests; document upstream SHAs per Allowlist PR template.  
