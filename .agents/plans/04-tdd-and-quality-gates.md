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
| FakeProvider integ | `cargo test -p integration byop_tool_loop` (name TBD) |

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

Still open: full blocklist controller integration (IT-003) driving UI history.

## Denylist TDD example

1. Add pattern `app\.warp\.dev` already exists.  
2. When porting upstream file that reintroduces URL, denylist fails.  
3. Delete URL usage or exclude only if in `docs/`.  

## Ownership

- Factor-owned modules: 80%+ critical paths tested before merge  
- Imported modules: add tests only on change  

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
```

Refine when workspace package names known.
