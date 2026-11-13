# Factor execution plans

## Current status — 2026-11-13
This update supersedes older conflicting status statements; the historical plans remain useful for scope and acceptance criteria.
- M0–M1: scaffold and macOS application startup are implemented. A Metal-enabled bundle was built and launched; this does not verify end-to-end workflows or release readiness.
- M2: upstream reconciliation remains selective. W2/W3 allowlisted ports and bulk orchestration evaluation remain open; publishing Factor's `main` did not complete those ports. See `docs/tech/M2_RESIDUAL.md`.
- M3: inference requires an explicitly configured BYOP provider and model. OpenRouter is available as a user-configured provider, not a legacy default; legacy settings controls were removed without deleting saved credentials. Live local and remote multi-turn inference, tools, error handling, and GUI history remain to be verified. GUI automation reached Settings but could not inspect its custom-drawn controls; the removal has not been visually verified. See `docs/tech/BYOP_INTEGRATION_STATUS.md`.
- M4: local `start_agent` backend and unit coverage exist; IT-005 must verify parent → child pane/conversation → inference → returned result in the GUI.
- M5: SSH policy, remote file fail-closed behavior, and remote long-running-command tag-in are implemented. Live SSH UJ-4/H4–H7, Linux packaging, and release readiness remain unverified; see `docs/tech/SSH_AGENT_MATRIX.md`.
- Latest recorded workspace Nextest: **5,124 passed, 80 skipped, 0 failed** (`--workspace --exclude command-signatures-v2`); doctests: **11 passed, 3 ignored**. These are historical results, not a fresh run for this update. Fresh remote CI verification remains outstanding.
- At review, local `main` was two commits ahead of `origin/main` (`f04165bb`, `af4b1e8e`); generated untracked files under `crates/warp_files/test_data/test_write/` were left untouched. Branch state is a snapshot, not a continuing guarantee.

## Verification update — 2026-11-11
Metal-enabled macOS bundle built successfully using `/Applications/Xcode-beta.app` (Xcode 27.3, 27C5020n) and Apple Metal 32023.925. The bundle contains a nonempty `shaders.metallib`; strict deep codesign verification passed. Launch produced an on-screen 1812×1010 window; the app was subsequently terminated. This verifies startup, not live inference or interactive workflows.

Fresh workspace Nextest: **5,123 passed, 80 skipped, 0 failed** across 61 binaries (`--workspace --exclude command-signatures-v2`). Doctests: **7 passed, 3 ignored, 0 failed**. Completer v2: **108 passed, 4 skipped**. Formatting and diff checks passed for that verification. Environment configuration is committed in `cfda418c`; rustls test dependency and doctest fixes in `59f539e2`. Older outstanding workspace/Metal statements below are superseded by this evidence.

Still outstanding: live BYOP local/remote multi-turn GUI/history, IT-005 parent→child pane→inference→result, SSH UJ-4/H4–H7, and fresh remote CI verification. The Factor Agent switch investigation found legacy OpenRouter-only availability gating and a switch bound to effective availability rather than the saved preference; the source fix now accepts configured BYOP models and displays the saved switch preference. All 15 AI settings tests, the static denylist, and offline-local smoke pass. The fixed Metal app bundle was rebuilt and passed strict deep codesign verification; launch and window creation were confirmed. GUI automation exposed no accessible controls or usable screenshot, so interactive toggle clicks and live inference remain unverified.

Ordered plans for building Factor from specs to shippable terminal.

## Status as of 2026-11-11

Implementation is ahead of the original phase checklists. Status below distinguishes landed code from outstanding live verification; unchecked design tasks in individual plans are not a fresh implementation audit.

Earlier locked-build evidence at `74f2796` skipped Metal shader compilation; it was superseded by the Metal-enabled bundle and test results recorded above.

Next: validate local/remote BYOP, IT-005 parent/child GUI orchestration, live SSH UJ-4, and remote CI. See [current overview](00-overview.md).

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
