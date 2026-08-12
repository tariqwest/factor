# Factor

> **factor** /ˈfæktər/ *n.* — as in *warp factor*: how fast you're going when the platform isn't dragging you.

**Factor** is a local-first, agentic terminal. It keeps Warp's best terminal substrate — blocks, sessions, warpify, SSH, shell hooks, third-party CLI agent support, inline suggestions — and a deeply embedded **local** agent with local multi-agent orchestration. It drops accounts, billing, Drive, teams, cloud Oz handoff, and hosted telemetry. Inference is **BYOP** (bring your own providers/keys), never required to route through Warp's servers.

> Star Trek note: "warp factor" is the speed scale; this project is the terminal that goes somewhere without asking for your email first. Alternate names considered: *nacelle*, *deflector*, *impulse* (too slow), *enterprise* (taken). **Factor** sticks for now.

## Relationship to upstream and forks

| Source | What we take |
|--------|----------------|
| [warpdotdev/warp](https://github.com/warpdotdev/warp) | Terminal engine, blocks, sessions, warpify, agent/blocklist substrate, local orchestration primitives |
| [warper](https://github.com/ruslanvakhitov/warper) | Hard strip of cloud/account platform; denylist discipline; OpenRouter as a reference thin provider |
| [zap](https://github.com/zerx-lab/zap) | Robust multi-provider BYOP agent stack |

Factor is **not** a soft fork of any one tree. The planned construction path is:

1. Reconcile **warper** product shape onto **latest upstream warp**
2. Integrate **zap BYOP** as the inference plane
3. Enable **local** agent + orchestration; strip **cloud** Oz/handoff

See [docs/product/PRODUCT.md](docs/product/PRODUCT.md), [docs/tech/TECH.md](docs/tech/TECH.md), and [.agents/plans/](.agents/plans/).

## Status

**M1 complete:** Warper monorepo imported and rebranded as Factor. OpenRouter is interim inference until M3 BYOP. See `docs/tech/IMPORT_MANIFEST.md`.

## License (planned)

Same dual structure as Warp:

- `warpui` / `warpui_core`: MIT
- Remainder: AGPL-3.0

Confirm final notices before first binary release.

## Docs map

| Doc | Purpose |
|-----|---------|
| [PRODUCT.md](docs/product/PRODUCT.md) | User-visible behavior, keep/strip matrix |
| [TECH.md](docs/tech/TECH.md) | Architecture, modules, integration seams |
| [TDD.md](docs/tech/TDD.md) | Testing strategy and gates |
| [DECISIONS.md](docs/adr/DECISIONS.md) | Recorded judgments + open questions |
| [Plans](.agents/plans/) | Execution plans (reconcile, BYOP, bootstrap) |
| [specs/FACTOR-*](specs/) | Spec-driven feature slices |

## Building (future)

```bash
./script/bootstrap
./script/run
./script/presubmit
./script/factor_offline_local_smoke
./script/check_factor_static_denylist all-runtime
```

After bootstrap, use the same entrypoints as Warper/Warp local OSS builds.
