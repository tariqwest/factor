# 04 — TDD & quality gates (execution)

Companion to [docs/tech/TDD.md](../../docs/tech/TDD.md).

## PR gate (required)

| Gate | Command (post-import) | Fails on |
|------|------------------------|----------|
| Format | `cargo fmt --check` | Drift |
| Clippy | `./script` clippy subset | New warnings in Factor-owned code (ratchet) |
| Denylist | `./script/check_factor_static_denylist all-runtime` | Hosted endpoints / surfaces |
| Unit | `cargo test -p warp --lib` filtered sets | Regressions |

## Pre-merge for agent/BYOP PRs

| Gate | Command |
|------|---------|
| BYOP unit | `cargo test byop_ -- --nocapture` |
| FakeProvider integ | `cargo test -p warp --lib fake_provider` (IT-003) |

## Release gate

| Gate | Command |
|------|---------|
| Offline smoke | `./script/factor_offline_local_smoke` |
| Denylist all | same as PR |
| Manual checklist | `tests/e2e/RELEASE_CHECKLIST.md` |

## FakeProvider (landed)

Location: `app/src/ai/agent/api/fake_provider.rs` behind `#[cfg(any(test, feature = "test-util"))]`

Behaviors:

1. Queue of scripted assistant turns (`Text` / `ToolCalls`)
2. Records requests (input count, tool-result presence)
3. Emits warp_multi_agent_api-compatible `ResponseEvent` stream
4. Parses tool args via BYOP `tools::REGISTRY`

Landed test:

```rust
#[test]
fn multi_turn_shell_then_final() {
  // turn1: scripted run_shell_command
  // harness: append ToolCallResult to tasks
  // turn2: final AgentOutput text
  // assert: 2 recorded requests; second has tool result
}
```

IT-003 controller boundary harness landed in `response_stream::tests::it003_fake_provider_multi_turn_response_stream`. Still open: full `BlocklistAIHistoryModel` / UI history action loop.

## Denylist TDD example

1. Add pattern `app\.warp\.dev` already exists.  
2. When porting upstream file that reintroduces URL, denylist fails.  
3. Delete URL usage or exclude only if in `docs/`.  

## Ownership

- Factor-owned modules: 80%+ critical paths tested before merge  
- Imported modules: add tests only on change  

## Full-suite triage (assessment addition — 220 failures)

`cargo test -p warp --lib` currently 2964/220. Before any new feature:

1. `cargo test -p warp --lib 2>&1 | rg 'FAILED|ok$' > /tmp/failures.txt`
2. Bucket: singleton `AIExecutionProfilesModel`, upstream drift, known flaky `workspace::view`.
3. File `docs/tech/NEXT_FAILURES.md` (or `M2_RESIDUAL` appendix) with owner + `ignore` rationale. Do **not** `#[ignore]` without linked issue/ADR.
4. Re-enable `factor-gates.yml` as **non-required** PR check during bucketing; promote to required when bucket = 0 unexpected.

## Supply chain & secrets (assessment addition)

- `cargo deny check` + `cargo audit` in `factor-gates.yml` (advisory mode first).
- Secret-leak scan: forbid `sk-`, `xoxp-`, provider keys in `app/src` except `agent_providers/secrets.rs` + tests; add pattern to `check_factor_static_denylist` `all-runtime` or new `secrets` scope (comments in `docs/` excluded).
- Keep `genai` SHA pinned; update only with provider API change.

## Perf & disk hygiene (assessment addition)

- Budgets per TECH §10: cold start ≤ warper+100ms, agent first-token client overhead <100ms, denylist+unit <5m. Measure in `factor_offline_local_smoke` + `factor_m5_smoke`; appendix in `RECONCILE_LOG`.
- Disk: `target` 21G / 93% already broke Metal. Add `script/clean` (`cargo clean` + `~/.cargo` registry prune) and gate `cargo check -p warp --bin factor-oss` behind `df -h` (J-07/J-08). Document in `JUDGMENT_LOG`.

## PR hygiene (assessment addition)

- Max ~400 lines per PR absent `large-change` label (warper skill pattern). `39 files +1855` in last 5 commits is exception, not norm (J-01).
- Clippy ratchet: Factor-owned code = 0 warnings; vendored `warper`/`zap` code may be `#[allow]` with comment.

## CI sketch (`.github/workflows/ci.yml` post-import)

```yaml
name: ci
on: [pull_request, push]
jobs:
  gates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: ./script/check_factor_static_denylist all-runtime
      - run: cargo fmt --check
      - run: cargo test -p warp --lib agent_providers --offline || cargo test -p warp --lib byop_
      - run: cargo deny check || true  # advisory until triaged
```

Refine when workspace package names known. During triage, run full suite as `cargo test -p warp --lib -- --nocapture 2>&1 | tail` and upload `NEXT_FAILURES.md` as artifact.
