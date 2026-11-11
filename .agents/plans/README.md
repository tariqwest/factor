# Factor execution plans

## Verification update — 2026-11-11
Metal-enabled macOS bundle built successfully using `/Applications/Xcode-beta.app` (Xcode 27.3, 27C5020n) and Apple Metal 32023.925. The bundle contains a nonempty `shaders.metallib`; strict deep codesign verification passed. Launch produced an on-screen 1812×1010 window; the app was subsequently terminated. This verifies startup, not live inference or interactive workflows.

Fresh workspace Nextest: **5,123 passed, 80 skipped, 0 failed** across 61 binaries (`--workspace --exclude command-signatures-v2`). Doctests: **7 passed, 3 ignored, 0 failed**. Completer v2: **108 passed, 4 skipped**. Formatting and diff checks passed for that verification. Environment configuration is committed in `cfda418c`; rustls test dependency and doctest fixes in `59f539e2`. Older outstanding workspace/Metal statements below are superseded by this evidence.

Still outstanding: live BYOP local/remote multi-turn GUI/history, IT-005 parent→child pane→inference→result, SSH UJ-4/H4–H7, and fresh remote CI verification. The Factor Agent switch investigation found legacy OpenRouter-only availability gating and a switch bound to effective availability rather than the saved preference; the source fix now accepts configured BYOP models and displays the saved switch preference. All 15 AI settings tests, the static denylist, and offline-local smoke pass. The fixed Metal app bundle was rebuilt and passed strict deep codesign verification; launch and window creation were confirmed. GUI automation exposed no accessible controls or usable screenshot, so interactive toggle clicks and live inference remain unverified.

Ordered plans for building Factor from specs to shippable terminal.

## Status as of 2026-11-11

Implementation is ahead of the original phase checklists. Status below distinguishes landed code from outstanding live verification; unchecked design tasks in individual plans are not a fresh implementation audit.

Latest local evidence: `mise exec -- cargo build --locked` succeeded at `74f2796`, but Metal shader compilation was skipped. This does not prove GUI readiness. August test/smoke results remain historical; no fresh full-suite, CI, provider, or SSH pass is claimed.

Next: produce a runnable Metal-enabled app, refresh quality gates, then validate local/remote BYOP, IT-005 parent/child GUI orchestration, and live SSH UJ-4. See [current overview](00-overview.md).

| # | Plan | Status |
|---|------|--------|
| 00 | [Overview & milestones](00-overview.md) | Active |
| 01 | [Bootstrap & monorepo import](01-bootstrap-and-import.md) | Implemented |
| 02 | [Reconcile Warper ↔ upstream Warp](02-reconcile-warper-upstream.md) | Partial; allowlisted ports deferred |
| 03 | [Integrate Zap BYOP](03-integrate-zap-byop.md) | Largely implemented; live dogfood outstanding |
| 04 | [TDD & quality gates](04-tdd-and-quality-gates.md) | Substantial coverage; fresh gate/CI verification outstanding |
| 05 | [Local orchestration enablement](05-local-orchestration.md) | Backend/unit layer implemented; IT-005 GUI E2E outstanding |
| 06 | [Warpify / SSH agent matrix](06-warpify-ssh-agent.md) | Policy/offline coverage implemented; live UJ-4 outstanding |

Related:

- [docs/product/PRODUCT.md](../../docs/product/PRODUCT.md)
- [docs/tech/TECH.md](../../docs/tech/TECH.md)
- [docs/tech/TDD.md](../../docs/tech/TDD.md)
- [docs/adr/DECISIONS.md](../../docs/adr/DECISIONS.md)
- [specs/FACTOR-*](../../specs/)

## Agent operating rules

1. Read PRODUCT + TECH before coding a slice.
2. Prefer TDD for Factor-owned modules (TDD.md).
3. Never reintroduce denylisted endpoints without an ADR.
4. Record judgment calls in DECISIONS.md when PRODUCT is silent.
5. Conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`.
